# DSP Features for Techno Subgenre Discrimination

Status: research complete, reviewed, ready for implementation
Date: 2026-03-09
Context: Dub Techno, Deep Techno, and Techno cannot be distinguished by any
current audio features (spectral centroid, grid stability, dissonance,
rhythm regularity, dynamic complexity — all overlap completely). This doc
proposes new DSP features that target the actual sonic differences.

Note: The genre classification SOP has been split into separate genre audit
and genre classification SOPs. These features would feed into the
classification SOP's decision tree once validated.

## What Makes These Genres Sound Different

| Characteristic | Dub Techno | Deep Techno | Techno |
|---------------|-----------|-------------|--------|
| Reverb | Heavy, long tails (1-4s) | Moderate, controlled | Minimal, short |
| Delay | Tempo-synced, high feedback | Sparse or none | Sparse, dry |
| Transients | Soft, smeared by reverb | Moderate | Sharp, crisp |
| Harmonic content | Sustained chord pads | Bass drones, minimal | Percussive, atonal |
| Stereo field | Wide (reverb/delay spread) | Narrow-moderate | Narrow, centered |
| Temporal texture | Smooth, slowly evolving | Hypnotic, repetitive | Driving, rapid changes |

Current features measure *spectral shape* and *rhythm timing* — but these
genres share similar spectral shapes and are all 4/4 grid-locked. What differs
is the *production processing chain*: reverb, delay, filtering, and how
transients decay. We need features that measure those things.

---

## Prerequisites

### ~~Cache invalidation~~ ✅ Done

~~Adding new features to analysis output creates a cache compatibility problem.~~

**Implemented:**
1. Added `#[serde(default)]` + `Default` to `StratumResult` (audio.rs:33) —
   new `Option<T>` fields gracefully default to `None` on old cache entries.
2. Added `STRATUM_SCHEMA_VERSION` and `ESSENTIA_SCHEMA_VERSION` constants
   (audio.rs:59-62). Both `is_cache_fresh` (CLI) and `check_analysis_cache`
   (MCP) now compare `analysis_version` against these constants. All cache
   write sites store the schema version constant. Bump to evict stale entries.
3. New feature fields should be `Option<T>` with `None` meaning "not yet
   computed" — allows gradual cache population.

### `compute_stft()` visibility

The plan originally assumed `stratum_dsp::features::chroma::extractor::compute_stft()`
is public. This is likely wrong — stratum-dsp's public API only exports
`analyze_audio`, result types, and `compute_confidence`. The STFT function is
probably not reachable at the crate boundary.

**Verify before Phase 2.** If not public, fork stratum-dsp from day one rather
than trying to recompute the STFT externally.

### Track trimming

All new features should be computed on **trimmed audio**: remove the first and
last 15% of the track to exclude intros/outros. Intros are typically sparse
and unrepresentative of the genre's sonic character. Apply trimming uniformly
to all features.

---

## Recommended Features (Priority Order)

### 1. Modulation Spectral Centroid (HIGH priority)

**What it measures:** How much reverb smooths out temporal envelopes.

Reverb acts as a low-pass filter on amplitude modulations — it fills in the
gaps between transients, suppressing fast amplitude changes while preserving
slow ones.

**Algorithm:**
1. Trim first/last 15% of track
2. Compute STFT magnitude frames (2048-pt, hop 512 — reuse existing)
3. Group STFT bins into 8 frequency bands
4. For each band: the magnitude summed per frame across time IS the amplitude
   envelope, sampled at ~86 Hz (sr/hop). No Hilbert transform or rectification
   needed.
5. FFT each band envelope (full track length, zero-pad to next power of 2)
   → modulation spectrum per band
6. Compute **modulation spectral centroid** per band: center of mass of the
   modulation spectrum (weighted mean of modulation frequencies by energy)
7. Average across bands → single scalar: `mod_centroid`

**Why modulation spectral centroid instead of a ratio:**
- No arbitrary frequency boundary needed (the original 10 Hz / 4 Hz had a
  dead zone missing musically important content)
- More robust to BPM variation (a fixed cutoff penalizes higher BPMs where
  eighth-note modulations naturally fall higher)
- Single number that captures overall "speed" of amplitude fluctuations

**Modulation frequency reference:**
- 0.5-2 Hz: slow LFO, filter sweeps (all genres)
- 2-4 Hz: half-note pulsing at ~130 BPM, dub techno's slow rhythmic pulse
- 4-8 Hz: eighth/sixteenth notes — this is where reverb smoothing is visible
- 8-16 Hz: fast hi-hat patterns, transient crispness
- >16 Hz: roughness, approaching pitch domain

**Expected discrimination:**
- Dub Techno: low centroid (reverb suppresses fast modulations, energy
  concentrated at low modulation frequencies)
- Techno: high centroid (crisp transients create fast modulations)
- Deep Techno: medium

**Implementation:**
- Rust with `rustfft` — STFT magnitude frames → band grouping → FFT of
  envelope → centroid calculation
- If `compute_stft()` is not public, fork stratum-dsp and add as a new
  analysis pass reusing existing `magnitude_spec_frames`
- Estimated added time: < 1s per track

**Robustness:** Good. Based on Modulation Transfer Function theory. Robust to
mastering compression (measures relative modulation depth). One caveat:
sidechain compression (common in techno) creates strong modulation at kick
rate (~2 Hz) which could pull the centroid lower for dry techno. The centroid
approach partially mitigates this because sidechaining preserves fast
transient modulations — it just adds low-frequency energy without removing
high-frequency energy.

**Edge cases / guards:**
- Clamp output to [0, 43] Hz (Nyquist of envelope at ~86 Hz frame rate)
- If all envelope energy is near DC (silence-padded track), output `None`

**Prior art:** Pampalk (2006) "fluctuation patterns" for music similarity;
Barchiesi & Sandler (2011) reverb estimation from mixed music.

---

### 2. HPSS Harmonic/Percussive Proportion (HIGH priority)

**What it measures:** Balance of sustained harmonic content (pads, chords) vs
percussive transients (kicks, hats, claps).

**Algorithm:**
1. Trim first/last 15% of track
2. Compute STFT magnitude spectrogram
3. Median filter along time axis (kernel: 31 frames = ~360ms) → harmonic
4. Median filter along frequency axis (kernel: 17 bins = ~366 Hz) → percussive
5. Create soft masks, compute energy of each component
6. Output: **H / (H + P)** — bounded [0, 1]

**Why H/(H+P) instead of H/P:**
- Bounded [0, 1] — well-behaved as classifier input
- 0.5 = balanced, near 1.0 = pad-dominated, near 0.0 = percussion-dominated
- H/P is unbounded (approaches infinity for pad-only sections)

**Why asymmetric kernels (31 frames × 17 bins):**
- Per Driedger et al. (2014), asymmetric kernels often work better
- 31 frames at hop=512, sr=44100 = 360ms — shorter than one beat at target
  tempos (448-500ms at 120-134 BPM), correct for harmonic median filter
- 17 bins at 21.5 Hz/bin = 366 Hz — narrower frequency kernel preserves
  more harmonic detail
- Fixed kernels are fine for the narrow BPM range (120-134). BPM-adaptive
  kernels add complexity for marginal gain.

**Expected discrimination:**
- Dub Techno: high proportion (pad-heavy, reverb extends harmonic content)
- Techno: low proportion (percussion-dominant)
- Deep Techno: medium (bass drones are harmonic, but less pad-heavy than dub)

**Implementation:**
- Rust: STFT (have it), 2D median filter over magnitude spectrogram
- The spectrogram for a 5-min track at hop=512 is ~13K frames × 1025 bins
- Consider `ndarray` crate for 2D array ops, or hand-roll sliding median
- Estimated added time: 1-3s per track (median filtering is the bottleneck)

**Note on existing HPSS:** stratum-dsp has HPSS in `features::onset::hpss`
but `hpss_decompose` takes a single `margin` parameter for both axes. The
plan uses asymmetric kernels (31 × 17), so the existing function signature
needs modification or reimplementation.

**Robustness:** Strong. Well-established algorithm (Fitzgerald 2010, Driedger
2014). Not sensitive to mastering choices — the harmonic/percussive
distinction is structural.

**Edge cases / guards:**
- If H + P ≈ 0 (silence), output `None`

---

### 3. Post-Transient Spectral Decay Rate (MEDIUM-HIGH priority)

**What it measures:** How quickly sound decays after each transient — directly
measures reverb tail length.

**Algorithm:**
1. Trim first/last 15% of track
2. Detect transients (onset detection — already available)
3. For each transient, extract energy envelope in **two bands**:
   - Band A: 1-4 kHz (chord/snare reverb)
   - Band B: 4-8 kHz (hi-hat/cymbal reverb — strong dub techno signal)
4. Window: from onset to **next onset minus 20ms guard** (variable length,
   not fixed 500ms). Discard windows shorter than **150ms**.
5. Fit exponential decay in log domain:
   `log(energy) = -t/tau + c` → linear regression → slope gives tau
6. Output per band:
   - `decay_tau_median` — median tau across all usable transients
   - `decay_tau_iqr` — interquartile range (spread)
   - `decay_fit_r2_median` — median R² of the fit (decay linearity —
     non-exponential decay from modulated reverbs may itself be
     discriminative)
   - `decay_usable_count` — number of transients with windows ≥ 150ms
     (quality indicator; flag if < 20)

**Why variable windows instead of fixed 500ms:**
At 133 BPM techno, the kick fires every ~450ms. With hi-hats and other
percussion, actual inter-onset gaps in the 1-4 kHz band can be well under
100ms. A fixed 500ms window would be truncated for most events. Variable
windows naturally select transients followed by gaps — exactly where reverb
tails are audible.

**Why two bands:**
- 1-4 kHz captures mid-range reverb (chord pads, snares)
- 4-8 kHz captures hi-hat/cymbal reverb, which is a strong dub techno
  indicator. Also less affected by kick drum energy.

**Expected discrimination:**
- Dub Techno: long decay (tau > 500ms), possibly lower R² (modulated reverbs)
- Techno: short decay (tau < 200ms), higher R² (clean exponential)
- Deep Techno: medium

**Implementation:**
- Rust: STFT band selection, onset times (available), envelope extraction,
  least-squares fit on log-envelope (simple linear regression)
- Estimated added time: < 1s per track

**Robustness:** Moderate. Heavy mastering compression shortens apparent decay.
Dense arrangements limit usable windows. Mitigations:
- Median across many transients (robust to outliers)
- Two bands provide independent measurements
- `decay_usable_count` signals when data is sparse
- R² flags non-standard decay profiles

**Edge cases / guards:**
- If `decay_usable_count < 20`, set confidence flag, still report median
- If no usable transients (extremely dense track), output `None`
- Clamp tau to [0, 5000] ms (5s max — beyond this is not reverb)

---

### 4. Temporal Statistics of Existing Features (LOW priority, near-zero cost)

**What it measures:** How features *change over time*, not just their average.

Currently all Essentia features are reduced to a single mean. Even if means
overlap between genres, the temporal dynamics may differ.

**Proposed additions to Essentia script:**

Spectral centroid (requires switching to per-frame computation — current
script uses `SpectralCentroidTime` on full waveform, not per-frame):
- `spectral_centroid_cv` — coefficient of variation (std/mean), normalized
  for brightness differences between tracks

Spectral flux (new feature, computed per-frame):
- `spectral_flux_mean` — average rate of spectral change
- `spectral_flux_iqr` — interquartile range (outlier-robust variability)
- **Normalize spectral flux by frame energy** before computing stats
  (prevents loudness from dominating)

MFCCs (extend existing frame loop):
- `mfcc_std` (13 values) — timbral variation over time. Reverb-heavy music
  should have smoother MFCC trajectories.

**Implementation:** ~15-20 lines of Python in the Essentia script frame loop.
Spectral centroid must switch from `SpectralCentroidTime` (full waveform) to
per-frame `SpectralCentroid` (takes spectrum input) — this is a small but
non-obvious change.

**Robustness:** Well-established features. Coefficient of variation is
preferred over raw std dev because it normalizes for the mean (brighter tracks
don't automatically get higher centroid std). IQR is preferred over std for
spectral flux because it's robust to outlier frames (breakdown sections).

---

## Where to Implement Each Feature

| Feature | Where | Rationale |
|---------|-------|-----------|
| Modulation spectral centroid | **stratum-dsp fork** | Needs STFT frames. Fork is likely required anyway (compute_stft probably not public). |
| HPSS proportion | **stratum-dsp fork** | Existing HPSS code needs API change for asymmetric kernels. Natural home. |
| Post-transient decay | **stratum-dsp fork** | Needs onset times (already computed) + STFT bands. Natural fit. |
| Temporal feature stats | **Essentia script** | Trivial addition to existing Python frame loop. |

### stratum-dsp modification path

stratum-dsp is a published crate pinned at `=1.0.0`. To add features:
1. Fork `https://github.com/HLLMR/stratum-dsp`
2. Add new fields to `AnalysisResult` struct (all `Option<f32>`)
3. Add new analysis passes in `lib.rs`, reusing existing `magnitude_spec_frames`
4. Use `[patch.crates-io]` in reklawdbox's `Cargo.toml` to point at the fork
5. Add fields to reklawdbox's `StratumResult` (with `#[serde(default)]`) and
   cache schema

### Architecture of stratum-dsp (relevant details)

- Entry: `analyze_audio(samples: &[f32], sample_rate: u32, config: AnalysisConfig)`
- STFT computed once (2048-pt, hop 512) and shared between onset/tempogram
- Key detection gets separate 8192-pt STFT
- All FFT via `rustfft` 6.x
- HPSS already exists in `features::onset::hpss` (used for onset detection
  only, single `margin` param for both axes)
- Onset times available as `Vec<f64>` (seconds)
- No trait-based extension — monolithic pipeline in `lib.rs`
- `rayon` available for parallelism

---

## Implementation Plan

### Phase 0: Prerequisites — ✅ Done

1. ~~Add `#[serde(default)]` to `StratumResult`~~ ✅
2. ~~Add `analysis_version` to cache freshness check~~ ✅
3. Fork stratum-dsp, set up `[patch.crates-io]`
4. Verify STFT and HPSS internals are accessible from new analysis passes

### Phase 1: Quick wins (Essentia, no Rust changes)

Add to the Essentia Python script:
- `spectral_centroid_cv` (coefficient of variation, per-frame computation)
- `spectral_flux_mean`, `spectral_flux_iqr` (energy-normalized)
- `mfcc_std` (13 values)

Cost: ~15-20 lines of Python. Test against the 15 reference tracks
immediately (force re-analysis with `skip_cached: false`).

### Phase 2: Modulation spectral centroid

Implement in stratum-dsp fork:
1. After existing STFT, group magnitude frames into 8 bands
2. For each band: sum magnitudes per frame → envelope time series
3. Apply 15% trim to envelope
4. FFT each trimmed envelope (zero-pad to next power of 2)
5. Compute spectral centroid of modulation spectrum per band, average
6. Add `mod_centroid: Option<f32>` to `AnalysisResult`

Test against 15 reference tracks.

### Phase 3: HPSS harmonic/percussive proportion

Implement in stratum-dsp fork:
1. Modify `hpss_decompose` to accept separate time/frequency kernel sizes
   (or add a new function alongside the existing one)
2. After existing STFT, run HPSS with 31×17 kernels
3. Compute H/(H+P) on trimmed spectrogram (15% from each end)
4. Add `harmonic_proportion: Option<f32>` to `AnalysisResult`

Test against 15 reference tracks.

### Phase 4: Post-transient decay rate

Implement in stratum-dsp fork:
1. After onset detection + STFT, extract band energy envelopes (1-4 kHz,
   4-8 kHz) from trimmed audio
2. For each onset, find next onset, compute window length
3. Discard windows < 150ms
4. Fit exponential decay (linear regression in log domain)
5. Report median tau, IQR, median R², usable count per band
6. Add decay fields to `AnalysisResult`

Test against 15 reference tracks.

### Phase 5: Stereo width — SKIPPED

Decision: skip stereo features. Stereo doubles decoded sample data and
requires a separate analysis pass (stratum-dsp takes mono `&[f32]`). The
reverb/delay signal stereo width captures is already measured from different
angles by modulation depth (temporal smoothing) and decay rate (tail length).
If those three mono features can't separate the genres, stereo width won't
save it. Revisit only if Phases 1-4 fail completely.

---

## Validation Protocol

For each new feature, test against the 15 curated reference tracks:

**Dub Techno (5):**
- Burger/Ink — Elvism (120 BPM, track ID 77922683)
- Exos — Indigo (120 BPM, track ID 254473062)
- Sascha Rydell — Laisser Faire (125 BPM, track ID 38915066)
- Traumer — Listen (126 BPM, track ID 12113269)
- Dialogue — Talkstart (130 BPM, track ID 265126276)

**Techno (5):**
- Robert Hood — Transform (127 BPM, track ID 255046660)
- S.A.M. — Right To Disobey (133 BPM, track ID 63094899)
- SDB — SB (133 BPM, track ID 188419898)
- Stefano Moretti — Turning Eyes (133 BPM, track ID 181255132)
- Voiski — Galaxy At Large (134 BPM, track ID 223245168)

**Deep Techno (5):**
- nthng — I Just Am (128 BPM, track ID 149244874)
- traumprinz — i gave my life (126 BPM, track ID 86319903)
- prince of denmark — tool 517 (126 BPM, track ID 185094504)
- Polar Inertia — Sonic Outlaws (122 BPM, track ID 240424121)
- Donato Dozzy — K5 (124 BPM, track ID 237150731)

**Success criteria for a feature:**
- Dub Techno mean is at least 1 standard deviation away from Techno mean
- Deep Techno falls between or clusters with one group (not overlapping both)
- No genre has > 1 track falling into the "wrong" cluster
- False positive rate < 15% on a broader sample (30+ tracks per genre)

**BPM decorrelation check:**
The reference set spans 120-134 BPM. Dub techno clusters at 120-130, techno
at 127-134. Features that correlate with BPM (faster = denser transients =
different modulation profile) could be BPM proxies rather than genre features.
For each feature, compute Pearson correlation with BPM across all 15 tracks.
If |r| > 0.7, the feature may be a BPM proxy — investigate further before
relying on it.

---

## What Won't Work (Ruled Out)

These were investigated and found non-discriminative across the reference tracks:

| Feature | Why it fails |
|---------|-------------|
| `spectral_centroid_mean` | Means: Dub 958, Techno 906, Deep 1111 — overlap completely |
| `grid_stability` | Means: Dub 0.34, Deep 0.39, Techno 0.43 — overlap completely |
| `dissonance_mean` | Flat 0.39-0.47 across all three genres |
| `rhythm_regularity` | No pattern — high variance within each genre |
| `dynamic_complexity` | No pattern |
| `danceability` | No pattern |
| Deep learning embeddings | Trained on datasets that don't distinguish electronic subgenres |
| More MFCCs / delta-MFCCs | MFCCs capture spectral shape; these genres share similar shapes |
| Onset timing features | All three are 4/4 grid-locked; timing is identical |
| Stereo width | Requires pipeline refactor; same reverb signal captured by mono features |

---

## Downstream Documentation Updates

Once these features are implemented and validated, the following docs/SOPs need
updating to stay in sync:

### Set Building SOP
The new DSP features provide strong signals for transition scoring and set
sequencing:
- **Modulation spectral centroid** — quantifies reverb density, useful for
  matching "atmosphere" between adjacent tracks. Smooth transitions need
  similar mod_centroid values; energy shifts can use deliberate jumps.
- **HPSS harmonic proportion** — indicates pad vs percussion balance. Moving
  from high to low harmonic_proportion maps to a "building energy" arc;
  the reverse signals a cooldown.
- **Post-transient decay rate** — reverb tail compatibility. Mixing a long-tail
  track into a dry track creates audible clashes. Transition scores should
  penalize large tau mismatches in the 4-8 kHz band.

The set building SOP should incorporate these as transition scoring inputs
alongside existing BPM, key, and energy compatibility.

### Other docs to check for drift
- **Genre classification SOP** — update decision tree (Step D/E) to use new
  features once validation confirms thresholds
- **Genre taxonomy** — if validation reveals new distinguishable subgenres,
  the taxonomy may need expansion
- **Agent SOPs (reklawdbox.com)** — the published agent SOPs reference specific
  feature names and thresholds; update after each phase ships
- **analyze_track_audio / analyze_audio_batch tool descriptions** — document
  new output fields so agents know what's available
- **Cache schema docs** — document new fields, version semantics, and
  migration behavior
