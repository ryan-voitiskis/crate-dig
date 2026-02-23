# Set Builder Harmonic V2 — Implementation Plan

**Date:** 2026-02-22
**Companion to:** `2026-02-22-set-builder-harmonic-query-plan.md`

This document refines the original query plan with implementation-grounded decisions, resolves the open questions, and defines concrete code changes.

---

## Resolved Design Decisions

### D1: Key scoring — Camelot-first, not MIREX-first

The original plan proposed adopting MIREX-style relation labels. The existing implementation already uses Camelot wheel distances directly (`score_key_axis` in `tools.rs:3624`), which is the right foundation. Instead of replacing it with MIREX weights and back-mapping to Camelot, we keep Camelot as the source of truth and attach music-theory labels for explainability.

**Current scores vs. proposed recalibration:**

| Relation                           | Current | Proposed | Rationale                                                       |
| ---------------------------------- | ------- | -------- | --------------------------------------------------------------- |
| Same key (8A → 8A)                 | 1.0     | 1.0      | No change                                                       |
| Adjacent ±1 same letter (8A → 9A)  | 0.9     | 0.9      | Already correct — this is the Camelot bread-and-butter          |
| Mood shift same number (8A → 8B)   | 0.8     | 0.8      | No change — well-calibrated                                     |
| Adjacent ±2 same letter (8A → 10A) | 0.5     | 0.45     | Slightly lower — ±2 is audible and risky                        |
| Diagonal ±1 cross letter (8A → 7B) | 0.4     | 0.55     | **Raise** — energy boost/drop diagonals are a real DJ technique |
| Everything else                    | 0.1     | 0.1      | No change — true clashes                                        |

The diagonal raise (0.4 → 0.55) is the most impactful change. 8A → 7B and 8A → 9B are recognized Camelot energy moves that the current scoring undervalues relative to ±2 same-letter moves.

**New label vocabulary for explainability:**

- `Perfect` — same key
- `Camelot adjacent` — ±1 same letter (fifth relation)
- `Mood shift` — same number, A↔B (relative major/minor)
- `Energy diagonal` — ±1 cross letter
- `Extended` — ±2 same letter
- `Clash` — everything else

### D2: BPM penalty — piecewise with comfort zone

**Current:** step thresholds at 2/4/6/8 BPM delta.
**Problem:** absolute BPM delta doesn't account for the percentage context (4 BPM at 128 is 3.1%, 4 BPM at 170 is 2.4%).

**Proposed:** score based on percentage delta with a sigmoid-like piecewise curve.

```
pct = abs(from_bpm - to_bpm) / from_bpm * 100

pct ≤ 1.5%  → 1.0   "Seamless"
pct ≤ 3.0%  → 0.85  "Comfortable"
pct ≤ 5.0%  → 0.6   "Noticeable"
pct ≤ 8.0%  → 0.3   "Creative transition needed"
pct > 8.0%  → 0.1   "Jarring"
```

At 128 BPM, the thresholds become: ≤1.9 / ≤3.8 / ≤6.4 / ≤10.2 BPM. This models the ±6% CDJ comfort zone better than fixed BPM deltas.

### D3: Master Tempo effective key transposition

**New parameter:** `master_tempo: Option<bool>` on `BuildSetParams` and `ScoreTransitionParams`. Default: `Some(true)` (Master Tempo on, key stays fixed regardless of tempo change).

**When `master_tempo = false`:**

1. Determine the tempo ratio: `r = target_bpm / track_bpm`
2. Compute semitone shift: `shift = 12.0 * log2(r)`
3. Round to nearest semitone (CDJ pitch is continuous but key relations are discrete)
4. Transpose the track's Camelot key by that many semitones
5. Use the transposed key for harmonic scoring

The `target_bpm` during set building is the previous track's BPM (since DJs tempo-match the incoming track to the outgoing one).

**Implementation:** new function `transpose_camelot_key(key: CamelotKey, semitones: i32) -> CamelotKey` that rotates the Camelot number. Each semitone is +7 Camelot positions mod 12 (following the circle of fifths mapping).

Wait — that's not right. Camelot numbers don't follow chromatic semitones directly. The Camelot wheel is ordered by fifths, so +1 semitone in pitch = a specific Camelot rotation. Let me be precise:

**Camelot A (minor) chromatic order:** 5A(Cm), 12A(C#m), 7A(Dm), 2A(Ebm), 9A(Em), 4A(Fm), 11A(F#m), 6A(Gm), 1A(G#m), 8A(Am), 3A(Bbm), 10A(Bm)

So +1 semitone maps: 5A→12A (+7 Camelot), 12A→7A (+7 again), etc. **+1 semitone = +7 Camelot positions mod 12.** This holds for both A and B lanes.

```rust
fn transpose_camelot_key(key: CamelotKey, semitones: i32) -> CamelotKey {
    // +1 semitone = +7 Camelot positions (circle of fifths)
    let camelot_shift = (semitones * 7).rem_euclid(12);
    let new_number = ((key.number as i32 - 1 + camelot_shift).rem_euclid(12) + 1) as u8;
    CamelotKey { number: new_number, letter: key.letter }
}
```

### D4: Modulation budget — section-based, riding the energy curve

Modulation budget is expressed as a **per-phase policy** rather than a global count or score budget.

```rust
#[derive(Debug, Clone, Copy, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HarmonicStyle {
    Conservative,  // Only same-key, adjacent, mood shift allowed
    Balanced,      // Default — all Camelot-compatible moves, penalize clashes
    Adventurous,   // Allow clashes in build/peak, still penalize in warmup/release
}
```

Phase × Style interaction matrix (minimum key score to avoid penalty):

| Phase   | Conservative                  | Balanced          | Adventurous         |
| ------- | ----------------------------- | ----------------- | ------------------- |
| Warmup  | 0.8 (same/adjacent/mood only) | 0.45 (no clashes) | 0.45 (no clashes)   |
| Build   | 0.8                           | 0.45              | 0.1 (anything goes) |
| Peak    | 0.8                           | 0.45              | 0.1                 |
| Release | 0.8                           | 0.45              | 0.45                |

When the key score falls below the phase minimum, apply a multiplicative penalty to the composite: `composite *= 0.5`. This acts as a soft gate — clashes are possible in adventurous mode during peak phases, but they're never "free" in warmup or release.

### D5: BPM trajectory coherence

Add a new scoring term: **BPM trajectory penalty** that considers the accumulated BPM drift, not just pairwise delta.

During `build_candidate_plan`, maintain a running `bpm_trajectory` (the sequence of BPMs so far). For each candidate, compute:

```
trajectory_drift = abs(candidate_bpm - start_bpm)
max_allowed_drift = total_bpm_range * (position / total_tracks)
```

where `total_bpm_range` is a configurable cap (default: 15 BPM for a 12-track set). If the drift exceeds the budget for this position, apply a penalty. This prevents the greedy algorithm from drifting monotonically in one BPM direction and ending far from where it started.

This is a **lightweight path-level constraint** that doesn't require beam search — it augments the existing greedy loop.

### D6: Genre stickiness

Add a **genre run bonus** to the genre axis: if the previous N tracks share a genre family, continuing that streak gets a small bonus (+0.1), while switching family gets a proportional cooldown.

```
genre_run_length = count of consecutive tracks in same family
bonus = if same_family && genre_run_length < 5 { 0.1 } else { 0.0 }
penalty = if different_family && genre_run_length < 2 { -0.1 } else { 0.0 }
```

This prevents jittery genre alternation without hard-blocking transitions. Cap at run length 5 to avoid monotony.

**Implementation:** `build_candidate_plan` already has the ordered sequence available. Pass the genre family run length into `score_transition_profiles` as additional context.

---

## Phase 1: Scoring Engine Upgrades

**Files touched:** `src/tools.rs`

### 1.1 Recalibrate `score_key_axis`

- Raise diagonal ±1 cross-letter from 0.4 → 0.55
- Lower ±2 same-letter from 0.5 → 0.45
- Add music-theory labels alongside Camelot labels in `AxisScore.label`
- Add `key_relation: String` field to `AxisScore` (or return it separately in the transition JSON) for structured explainability

### 1.2 Percentage-based `score_bpm_axis`

- Change from absolute BPM delta to percentage delta
- New thresholds: 1.5% / 3% / 5% / 8%
- Keep labels descriptive with both percentage and absolute values

### 1.3 Master Tempo transposition in key scoring

- Add `transpose_camelot_key(key, semitones)` function
- Add `master_tempo: Option<bool>` to `ScoreTransitionParams` and `BuildSetParams`
- When `master_tempo == Some(false)`, compute effective key before calling `score_key_axis`
- In `score_transition` tool: include `effective_key` and `pitch_shift_semitones` in output JSON
- In `build_candidate_plan`: pass `master_tempo` through to scoring

### 1.4 Harmonic style + modulation policy

- Add `HarmonicStyle` enum and `harmonic_style: Option<HarmonicStyle>` to both param structs
- Apply phase × style penalty matrix after composite scoring
- Default: `Balanced`

### 1.5 Genre stickiness

- Add genre run length tracking to `build_candidate_plan`
- Pass into scoring as context for genre axis bonus/penalty

### 1.6 BPM trajectory coherence

- Track cumulative BPM drift in `build_candidate_plan`
- Apply trajectory penalty when drift exceeds position-proportional budget

---

## Phase 2: Ordering Improvements

**Files touched:** `src/tools.rs`

### 2.1 Beam search (width 3–5)

Replace greedy selection in `build_candidate_plan` with beam search:

- Maintain top-K partial sequences (K=3 by default, configurable)
- At each step, expand all beams by scoring top-M candidates per beam (M=5)
- Prune back to K beams by **cumulative path score**, not just last-transition score
- Cumulative score = mean composite of all transitions in the path, with a harmonic drift penalty for total Camelot distance traveled

### 2.2 Cumulative harmonic coherence

Score paths on total Camelot distance traveled, not just pairwise compatibility. A path that goes 8A→9A→10A→11A→12A scores well pairwise (all ±1) but drifts 4 Camelot positions. Apply a gentle penalty when total Camelot drift exceeds 3 positions from the start key:

```
drift_penalty = max(0, total_camelot_distance - 3) * 0.02
path_score = mean_composite - drift_penalty
```

### 2.3 Controlled randomness via path sampling

Instead of always returning the top-K beams:

1. Complete beam search deterministically
2. Score all surviving paths
3. Sample from paths with score within 5% of the best (softmax-style with temperature)
4. Use `candidates` parameter to control how many samples to return

This preserves deterministic scoring while producing varied output across candidates.

---

## Phase 3: Tool Interface Changes

**Files touched:** `src/tools.rs`, `src/types.rs`

### 3.1 Updated `BuildSetParams`

```rust
pub struct BuildSetParams {
    pub track_ids: Vec<String>,
    pub target_tracks: u32,
    pub priority: Option<SetPriority>,
    pub energy_curve: Option<EnergyCurveInput>,
    pub start_track_id: Option<String>,
    pub candidates: Option<u8>,
    // New fields:
    pub master_tempo: Option<bool>,           // Default: true
    pub harmonic_style: Option<HarmonicStyle>, // Default: Balanced
    pub bpm_drift_limit: Option<f64>,          // Max BPM drift from start. Default: 15.0
}
```

### 3.2 Updated `ScoreTransitionParams`

```rust
pub struct ScoreTransitionParams {
    pub from_track_id: String,
    pub to_track_id: String,
    pub energy_phase: Option<EnergyPhase>,
    pub priority: Option<SetPriority>,
    // New fields:
    pub master_tempo: Option<bool>,
    pub harmonic_style: Option<HarmonicStyle>,
}
```

### 3.3 Enriched transition output

Each transition in the `build_set` response gains:

```json
{
  "from_index": 0,
  "to_index": 1,
  "scores": { "...existing..." },
  "effective_key": "9A",
  "pitch_shift_semitones": 0.3,
  "bpm_adjustment_pct": 2.1,
  "key_relation": "Camelot adjacent (+1)",
  "cumulative_camelot_drift": 1
}
```

### 3.4 No new `query_transition_candidates` tool

Per the analysis, keep the transition query logic internal to `build_set`. Exposing it as a separate MCP tool would create chatty N-call workflows. The enriched per-transition output in `build_set` provides the explainability that a separate tool would have offered.

---

## Phase 4: SOP + Agent Behavior

**Files touched:** `docs/operations/sops/set-builder.md`

### 4.1 Pre-build configuration step

The SOP instructs the agent to confirm before calling `build_set`:

1. **Master Tempo** — "Are you using Master Tempo? (on = key stays fixed, off = pitch affects key)"
2. **Harmonic style** — "How adventurous with key changes? conservative / balanced / adventurous"
3. **BPM intent** — "Stay near a target BPM range, or allow drift?"
4. **Energy curve** — existing (warmup-build-peak-release, flat, custom)
5. **Set length** — existing
6. **Progression narrative** — free-form in plain English, agent interprets into parameters

The agent summarizes interpreted config back to the user before calling `build_set`.

### 4.2 Camelot + theory dual labeling in agent output

When presenting set candidates, the agent shows both representations:

```
Track 3 → Track 4: 8A → 9A (Am → Em, Camelot adjacent)
  Key: 0.9 | BPM: 0.85 (+2.3%) | Energy: rising (build phase)
```

---

## Phase 5: Calibration

### 5.1 Regression test

Use a known 12-track deep techno set as the regression baseline. Run `build_set` with current scoring, save output. Re-run after each phase, compare:

- Mean composite score
- Harmonic coherence (% of transitions scoring ≥0.8 on key axis)
- BPM smoothness (max single-step percentage change)
- Total Camelot drift
- Energy curve adherence (% of transitions matching phase direction)

### 5.2 A/B evaluation harness

Build a lightweight Rust test that:

1. Takes a pool of 20+ tracks with known audio analysis
2. Runs `build_set` under old and new scoring
3. Outputs a side-by-side comparison table
4. Flags regressions (transitions that scored higher before but lower now)

This can be a `#[test]` in `tools.rs` using in-memory profiles (no DB needed).

---

## Implementation Sequencing

| Step | Phase                                     | Effort | Dependency |
| ---- | ----------------------------------------- | ------ | ---------- |
| 1    | 1.1 Key recalibration                     | Small  | None       |
| 2    | 1.2 BPM percentage scoring                | Small  | None       |
| 3    | 1.3 Master Tempo transposition            | Medium | 1.1        |
| 4    | 1.4 Harmonic style + modulation policy    | Medium | 1.1        |
| 5    | 1.5 Genre stickiness                      | Small  | None       |
| 6    | 1.6 BPM trajectory coherence              | Small  | 1.2        |
| 7    | 3.1–3.3 Param + output changes            | Small  | 1.3, 1.4   |
| 8    | 5.1 Regression baseline (capture current) | Small  | None       |
| 9    | 2.1 Beam search                           | Large  | 1.*        |
| 10   | 2.2 Cumulative harmonic coherence         | Medium | 2.1        |
| 11   | 2.3 Path sampling                         | Small  | 2.1        |
| 12   | 4.1–4.2 SOP update                        | Small  | 3.*        |
| 13   | 5.2 A/B evaluation harness                | Medium | 2.*        |

Steps 1, 2, 5, 8 can run in parallel. Steps 3 and 4 can run in parallel after 1. Step 9 is the largest single piece of work.

---

## Decisions deferred from original plan

1. **Phrase/cue-point modeling** — still deferred.
2. **Global optimization (MIP)** — beam search is the practical middle ground. Revisit only if beam width 5 proves insufficient.
3. **Preset-only UX** — explicitly rejected in favor of conversational control.
4. **Separate `query_transition_candidates` tool** — rejected, kept internal to `build_set`.

## Open questions resolved

| #  | Question                            | Resolution                                                                        |
| -- | ----------------------------------- | --------------------------------------------------------------------------------- |
| Q1 | BPM penalty curve shape             | Percentage-based piecewise with comfort zone at ≤3% (see D2)                      |
| Q2 | Modulation budget expression        | Section-based, coupled to energy phases via HarmonicStyle (see D4)                |
| Q3 | Top-N vs. full frontier             | Top-K beam paths, sampled for variety. No full frontier exposed.                  |
| Q4 | Mandatory explanation fields        | `key_relation`, `effective_key`, `bpm_adjustment_pct`, `cumulative_camelot_drift` |
| Q5 | Conservative hard-block vs. penalty | Soft gate via multiplicative penalty (×0.5), not hard block (see D4)              |
