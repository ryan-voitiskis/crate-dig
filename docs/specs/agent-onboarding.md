# Spec: Agent Onboarding & SOP Discovery

Status: Implemented (community section deferred)

## Problem

A user installs reklawdbox via Homebrew, configures it as an MCP server, and
starts a Claude Code session. The agent sees 29 tools with short descriptions
and a one-line server instruction. It has no idea that detailed workflow guides
exist on reklawdbox.com, no sense of which tools to use for which task, and no
way to guide the user through a multi-step procedure like genre classification
or set building.

Today the SOPs live in `docs/sops/` in the repo. Repo contributors can read
them. Installed users cannot — and even if they could, dumping 25KB of SOP
into context for every session is wasteful.

## Goals

1. A new user's agent can discover what reklawdbox does and how to use it
   within the first interaction — without reading the repo.
2. SOPs and workflow guides are accessible on demand, not preloaded.
3. Token budget is respected: bootstrap is minimal (~200 tokens), full SOPs
   are fetched only when needed.
4. reklawdbox.com becomes the canonical home for workflow guides, SOPs, and
   agent-optimized prompts — useful for both humans and agents.

## Design

### Layer 1: MCP Server Self-Description (~200 tokens)

The `ServerInfo.instructions` field is the agent's first contact. Expand it
from the current one-liner to a compact capability map:

```
Rekordbox library management server. Read-only DB access, staged XML export.

Capabilities:
- Library search and track inspection
- Audio analysis (BPM, key, energy) via stratum-dsp + Essentia
- Metadata enrichment via Discogs and Beatport
- Genre classification with evidence-based recommendations
- Transition scoring and DJ set sequencing
- Collection auditing (naming, tagging, missing metadata)

Quick start: call read_library to see the collection.
Guided workflows: call help for workflow guides with links to reklawdbox.com.
```

This tells the agent *what's possible* and *where to go next* without loading
any SOP content.

### Layer 2: `help` Tool (~500 tokens per call)

A new MCP tool that returns a structured capability map with workflow
descriptions and URLs. The agent calls it when the user asks "what can you do?"
or when it needs to discover the right workflow.

```
help(topic?: string)
```

- No arguments → returns the full menu: list of workflows with one-line
  descriptions and reklawdbox.com URLs.
- `topic` argument → returns a focused summary for that workflow plus the URL
  to the full guide.

Example response (no topic):

```json
{
  "workflows": [
    {
      "name": "Genre Classification",
      "summary": "Classify genres using Discogs, Beatport, and audio evidence.",
      "url": "https://reklawdbox.com/workflows/genre-classification/",
      "key_tools": ["enrich_tracks", "suggest_normalizations", "update_tracks"]
    },
    {
      "name": "Set Building",
      "summary": "Build transition-scored DJ set sequences.",
      "url": "https://reklawdbox.com/workflows/set-building/",
      "key_tools": ["query_transition_candidates", "score_transition", "build_set"]
    },
    {
      "name": "Collection Audit",
      "summary": "Detect and fix naming, tagging, and convention violations.",
      "url": "https://reklawdbox.com/workflows/collection-audit/",
      "key_tools": ["audit_state", "update_tracks", "write_file_tags"]
    }
  ],
  "getting_started": "https://reklawdbox.com/getting-started/",
  "reference": "https://reklawdbox.com/reference/tools/",
  "tip": "Run cache_coverage to check enrichment and analysis readiness before starting a workflow."
}
```

The agent can then use WebFetch to pull the full workflow guide from
reklawdbox.com when the user wants to follow a specific procedure.

### Layer 3: reklawdbox.com as Agent-Optimized Content Hub

The site already exists (Astro/Starlight, deployed via GitHub Pages). Current
content:

- Getting Started (setup, MCP config, Essentia, Discogs)
- Concepts (architecture, safety model, harmonic mixing)
- Workflows (genre classification, set building, collection audit)
- Reference (29-tool catalog, env vars, transition scoring, XML export)
- Troubleshooting

#### What to add

**Agent-optimized SOP pages.** The current workflow pages are human-readable
guides (~200 lines each). Add companion pages or sections that are designed for
agent consumption:

- Concise, imperative, step-by-step
- Tool call sequences with parameter examples
- Decision trees as structured lists, not prose
- Explicit stop points ("ask user before proceeding")
- Token-conscious: aim for <2000 tokens per SOP

These could live at URLs like:
- `reklawdbox.com/agent/genre-classification/`
- `reklawdbox.com/agent/set-building/`
- `reklawdbox.com/agent/collection-audit/`

Or as a dedicated section within each existing workflow page (e.g. an
"Agent Instructions" tab or collapsible block).

**Community SOPs and prompts.** A section for user-contributed workflows,
prompt templates, and specialized procedures. Could start as a `community/`
content directory in the site, accepting contributions via PR.

### Layer 4: Token-Conscious Fetch Pattern

The intended interaction flow:

1. Agent reads `ServerInfo.instructions` (~200 tokens) — knows capabilities
2. User asks to do something → agent calls `help` (~500 tokens) — gets
   workflow map with URLs
3. Agent identifies the right workflow → uses WebFetch to pull the
   agent-optimized SOP from reklawdbox.com (~2000 tokens)
4. Agent follows the SOP step by step, calling MCP tools as directed

Total context cost for a guided workflow: ~2700 tokens upfront, versus
~6000+ tokens if SOPs were embedded in CLAUDE.md or bundled in the binary.

## Relationship to Existing Content

| Content | Location | Audience | Purpose |
| --- | --- | --- | --- |
| Internal SOPs | `docs/sops/` | Contributors | Detailed procedures, decision rationale |
| Site workflows | `site/src/content/docs/workflows/` | Humans | Readable guides with context |
| Agent SOPs | `site/src/content/docs/agent/` (new) | Agents | Token-optimized step-by-step instructions |
| `help` tool | MCP server | Agents | Discovery and routing |
| `ServerInfo` | MCP protocol | Agents | Bootstrap orientation |

The internal SOPs in `docs/sops/` remain the source of truth. Site workflow
pages are the human-friendly version. Agent SOP pages are distilled from both,
optimized for token efficiency and tool-call sequencing.

## Implementation Sequence

1. **Expand `ServerInfo.instructions`** — minimal change in `src/tools/mod.rs`.
2. **Add `help` tool** — new tool in the MCP server, returns static JSON.
3. **Author agent-optimized SOP pages** on reklawdbox.com — distill from
   existing `docs/sops/` and site workflow pages.
4. **Add community section** to site — `community/` content directory.

Steps 1-2 are small code changes. Step 3 is content work. Step 4 is
structural.

## Resolved Questions

- **`help` format:** Structured JSON. Easier for agents to parse; the `tip`
  field directs to WebFetch for prose SOPs.
- **SOPs as separate pages or embedded:** Separate `/agent/` directory. Keeps
  human workflow pages clean; agent pages are plain markdown for clean
  WebFetch/llms.txt output.
- **`format` parameter:** Not needed. JSON-only is sufficient; all current MCP
  hosts handle it well.

## Open Questions

- **Versioning:** How to version agent SOPs relative to the server? If a tool's
  parameters change, the SOP on the site needs updating. Deferred until a
  breakage actually occurs.
