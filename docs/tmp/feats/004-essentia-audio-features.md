# 004: Essentia Audio Feature Extraction

Status: ready
Date: 2026-02-19
Depends on: 001 (internal store), 002 Tier 1 (stratum-dsp — completed)

## Context

stratum-dsp provides BPM and key with confidence scores. This is useful for cross-referencing Rekordbox values but does not help distinguish between genres that share tempo and key ranges. Essentia fills this gap with danceability, energy, loudness, rhythmic descriptors, and spectral features — the signals Claude needs for informed genre reasoning.

The original spec (002) listed Essentia as "Tier 2, prove the need first." With ~2,448 tracks to genre-tag and stratum-dsp + enrichment now working, the need is clear: many tracks share similar BPM/key but differ in feel. Rhythmic descriptors (four-on-the-floor ratio, beat complexity) are the single most genre-relevant audio feature for electronic music.

## Essentia overview

[Essentia](https://essentia.upf.edu/) is a C++ library with Python bindings from the Music Technology Group at UPF Barcelona. Mature, well-documented, widely used in MIR research.

- PyPI package: `essentia` (current: `2.1b6.dev1389`)
- ARM64 macOS wheels: available for macOS 15.0+ on Python 3.9–3.13
- Size: ~20MB wheel

### Python version constraint

This machine runs Python 3.14.3 (Homebrew). Essentia wheels are only published for Python 3.9–3.13. The implementation must use a dedicated venv with a compatible Python version via pyenv.

```bash
# Setup (one-time, operator performs this)
pyenv install 3.13.4
pyenv shell 3.13.4
python3 -m venv ~/.local/share/crate-dig/essentia-venv
~/.local/share/crate-dig/essentia-venv/bin/pip install essentia
```

### essentia-tensorflow

Not available on ARM64 (open issue: MTG/essentia#1486). Mood/arousal/valence models require it. Excluded from this feature — revisit if the issue is resolved.

## Features to extract

All algorithms below are from Essentia's standard (non-TF) library.

| Feature | Algorithm | Output | Genre relevance |
|---|---|---|---|
| Danceability | `Danceability` | 0–3 float | Distinguishes floor tracks from ambient/experimental |
| Energy (integrated loudness) | `LoudnessEBUR128` | LUFS float | Separates genres by production intensity |
| Loudness range | `LoudnessEBUR128` | LU float | Compressed (techno) vs dynamic (jazz, ambient) |
| Dynamic complexity | `DynamicComplexity` | float | Breakdown-heavy vs steady tracks |
| Average loudness | `Loudness` | float | Simple loudness proxy |
| Rhythm: four-on-the-floor ratio | `BeatsLoudness` band ratios | 0–1 float | **The key signal** — house/techno vs breakbeat/DnB/garage |
| Rhythm: onset rate / complexity | `OnsetRate` | float | Busy vs sparse rhythmic patterns |
| Spectral centroid | `SpectralCentroidTime` | Hz float | Bright (trance) vs dark (dub techno) |

### Python extraction script

Embedded in Rust via `include_str!` or inline constant. Takes a file path as argv[1], prints JSON to stdout.

```python
import sys, json, essentia, essentia.standard as es

audio = es.MonoLoader(filename=sys.argv[1], sampleRate=44100)()

features = {}

# Danceability
features["danceability"] = float(es.Danceability()(audio)[0])

# Loudness (EBU R128)
ebu = es.LoudnessEBUR128()(audio)
features["loudness_integrated"] = float(ebu[0])
features["loudness_range"] = float(ebu[2])

# Dynamic complexity
features["dynamic_complexity"] = float(es.DynamicComplexity()(audio)[0])

# Average loudness
features["average_loudness"] = float(es.Loudness()(audio))

# Rhythm
rhythm = es.RhythmExtractor2013(method="multifeature")(audio)
features["bpm_essentia"] = float(rhythm[0])
features["onset_rate"] = float(es.OnsetRate()(audio)[0])

# Beats loudness for four-on-the-floor detection
beats = rhythm[1]  # beat tick positions
if len(beats) > 4:
    bl = es.BeatsLoudness(beats=beats)(audio)
    band_ratios = bl[1]  # loudness per frequency band per beat
    import numpy as np
    # Ratio of energy on downbeats (every 4th) vs all beats in low band
    n = len(band_ratios)
    downbeat_energy = np.mean([band_ratios[i][0] for i in range(0, n, 4)])
    all_energy = np.mean([band_ratios[i][0] for i in range(n)])
    features["rhythm_regularity"] = float(downbeat_energy / max(all_energy, 1e-6))
else:
    features["rhythm_regularity"] = None

# Spectral centroid
features["spectral_centroid_mean"] = float(
    es.CentralMoments()(es.Spectrum()(audio))[0]
)

features["analyzer_version"] = essentia.__version__

json.dump(features, sys.stdout)
```

Note: the exact algorithm choices and parameters are implementation decisions. The script above is a starting point — validate against known reference tracks and adjust. The `BeatsLoudness` four-on-the-floor detection in particular may need refinement (the ratio calculation is a simplified heuristic).

## Architecture

### Rust integration

```
analyze_track_audio (existing tool)
  ├── stratum-dsp: spawn_blocking (existing, ~200ms)
  └── essentia: tokio::process::Command (new, ~5-15s)
        ├── stdout → JSON features
        └── stderr → captured, logged on failure
```

Key design points:

1. **Subprocess, not FFI.** Python via `Command::new(python_path)`. No PyO3, no shared memory. Clean process boundary.
2. **Venv discovery.** Check paths in order:
   - `CRATE_DIG_ESSENTIA_PYTHON` env var (explicit path to python binary)
   - `~/.local/share/crate-dig/essentia-venv/bin/python` (default venv)
   - Skip Essentia silently if neither exists
3. **Per-track timeout.** 300 seconds default. Kill subprocess on timeout.
4. **Stderr isolation.** Essentia's Python bindings print warnings to stderr. Capture separately — never leak to MCP stdio transport.
5. **Cache key.** Same as stratum-dsp: `(file_path, "essentia", file_size, file_mtime, analysis_version)`. Version from `essentia.__version__`.

### Changes to existing tools

**`analyze_track_audio`:**
- After stratum-dsp analysis, check if Essentia is available
- If available, run Essentia subprocess and cache result
- Return combined output with both `stratum_dsp` and `essentia` sections
- If Essentia unavailable, return `essentia: null` (existing behavior)

**`analyze_audio_batch`:**
- Run stratum-dsp for all tracks first (fast, blocking tasks)
- Then run Essentia sequentially per track (slow, subprocess)
- Consider parallel Essentia workers (default 2-4) via `tokio::spawn` — each subprocess is independent
- Memory: ~80-120 MB per Python process, so 4 workers ≈ 480 MB peak

**`resolve_track_data` / `resolve_tracks_data`:**
- Already have `essentia: null` stub and `data_completeness.essentia: false`
- Wire up: read `audio_analysis_cache` with `analyzer = "essentia"`, parse `features_json`
- Set `data_completeness.essentia_installed` based on venv probe (cache the probe result in `OnceLock<bool>`)

### New internal state

Add to `ServerState`:
```rust
essentia_python: OnceLock<Option<String>>,  // None = not available, Some(path) = python binary
```

Probe once on first Essentia tool call. Cache the result.

## Output schema changes

`analyze_track_audio` response gains an `essentia` section:

```json
{
  "stratum_dsp": { ... },
  "essentia": {
    "danceability": 0.82,
    "loudness_integrated": -8.2,
    "loudness_range": 7.1,
    "dynamic_complexity": 0.44,
    "average_loudness": 0.67,
    "bpm_essentia": 128.1,
    "onset_rate": 3.2,
    "rhythm_regularity": 0.91,
    "spectral_centroid_mean": 2120.4,
    "analyzer_version": "2.1b6.dev1389"
  },
  "essentia_available": true
}
```

`resolve_track_data` `audio_analysis` section:
```json
{
  "audio_analysis": {
    "stratum_dsp": { ... },
    "essentia": { ... },
    "bpm_agreement": true,
    "key_agreement": true
  }
}
```

`data_completeness`:
```json
{
  "essentia": true,
  "essentia_installed": true
}
```

## Implementation plan

### Step 1: Venv probe and state

- Add `essentia_python: OnceLock<Option<String>>` to `ServerState`
- Implement `probe_essentia()` that checks env var, then default venv path
- Validate by running `python -c "import essentia; print(essentia.__version__)"` and checking exit code

### Step 2: Python script and subprocess runner

- Embed extraction script as a Rust constant (`const ESSENTIA_SCRIPT: &str = ...`)
- Implement `run_essentia(python_path: &str, audio_path: &str) -> Result<serde_json::Value, String>`
- Capture stdout (JSON), stderr (warnings/errors), enforce timeout
- Parse and validate output structure

### Step 3: Wire into analyze tools

- `analyze_track_audio`: run Essentia after stratum-dsp if available, cache result
- `analyze_audio_batch`: run Essentia pass after stratum-dsp pass, with configurable parallelism

### Step 4: Wire into resolve tools

- Read Essentia cache in `resolve_track_data` and `resolve_tracks_data`
- Update `data_completeness.essentia` and `data_completeness.essentia_installed`
- `essentia_installed` is based on venv probe, `essentia` is based on cache existence

### Step 5: Tests

- Unit test: mock subprocess output parsing
- Unit test: venv probe with env var override
- Integration test (ignored): real Essentia analysis of a known track
- Integration test (ignored): cache round-trip
- Verify `essentia_available: false` when venv is absent (default test environment)

## Acceptance criteria

1. Essentia subprocess runs against real audio files and returns valid JSON features.
2. Results cached in `audio_analysis_cache` with `analyzer = "essentia"`.
3. `analyze_track_audio` returns combined stratum-dsp + Essentia output.
4. `analyze_audio_batch` runs both analyzers with structured error handling.
5. `resolve_track_data` includes Essentia features when cached.
6. `data_completeness.essentia_installed` correctly reflects venv availability.
7. Missing Essentia venv does not prevent stratum-dsp analysis or any other tools.
8. Per-track subprocess timeout kills hung processes.
9. Stderr from Python never leaks to MCP stdio.

## Risks

1. **Essentia is a dev pre-release** (`2.1b6.dev*`) — API may shift between releases. Pin version in venv.
2. **Python 3.13 max** — if pyenv isn't set up, Essentia won't work with the system Python 3.14.
3. **Algorithm tuning** — the feature extraction script is a starting point. `BeatsLoudness` ratio for four-on-the-floor detection needs validation against known tracks (house/techno should score high, breakbeat/DnB should score low).
4. **Batch duration** — 2,448 tracks at ~10s each = ~7 hours single-threaded. 4 workers ≈ 2 hours. Plan for interruptibility.
5. **Memory** — 4 concurrent Python processes × 120MB = ~480MB. Acceptable on 24GB machine.
