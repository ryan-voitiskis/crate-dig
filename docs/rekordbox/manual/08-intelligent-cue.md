---
id: intelligent-cue
title: "Using the INTELLIGENT CUE CREATION function"
type: manual
source:
  file: "rekordbox7.2.8_manual_EN.pdf"
  pages: "41-44"
  version: "7.2.8"
topics: [analysis, cue-points, hot-cue, memory-cue, playlists, preferences]
modes: [common]
confidence: pending
last_verified: null
transcribed_by: agent
verified_by: null
---

# Using the INTELLIGENT CUE CREATION function

You can perform CUE Analysis, which automatically sets CUE points (Hot Cue points or memory cue points) in a track during track analysis.

**Note**

- Depending on your subscription plan, this function may not be available. For details on subscription plan, refer to the rekordbox website.

## Preparing for the INTELLIGENT CUE CREATION function

You can select Hot Cue or memory cues as the type of CUE you want to set, and you can set up to 16 Hot Cues and 10 memory cues.
There are two CUE Analysis modes: [Auto] and [Manual].

In the [Auto] mode, CUE Analysis is performed using two types of CUE trend information.

- Overall CUE trend information: Learning data created based on general CUE point setting information
- Personal CUE trend information: Data analyzed from CUE points of tracks added to [CUE Analysis playlist]

Overall CUE trend information is automatically imported after installation, so no setting is required.
Personal CUE trend information is generated when you register tracks with CUE points set in [CUE Analysis playlist] and click the [Start] button in [CUE Analysis playlist].

**Hint**

- When personal CUE trend information does not exist, CUE Analysis is performed using only Overall CUE trend information.

In the [Manual] mode, CUE points are set according to the settings in [Preferences].
You can set a CUE point at the 1.1Bars position.
You can set CUE points repeatedly for each set number of bars starting from the 1.1 Bars position.

To specify the information to be analyzed, open the [Preferences] window > [Analysis] category > [Track Analysis] tab > [Track Analysis Setting], and select [BPM/Grid], [KEY], [Phrase] and [Vocal].
For Analysis setting items, see page 230.

**Note**

- CUE Analysis may not be available if the phrase of the track cannot be analyzed or the track is too short.
- If the number of phrases or bars in the track is small, the number of Hot Cues/memory cues set by CUE Analysis may be smaller than the number set in [Upper limit] of Hot Cues/memory cues.
- If [Prohibit overwriting CUEs] is off, the number of memory cues may be smaller than before CUE Analysis.

## Using a playlist for CUE Analysis

In the [Auto] mode, you can reflect your own CUE setting trends in the analysis results.
CUE trend information can be created using the following method.

[Screenshot: Tree View showing Playlists with CUE Analysis Playlist and Start button, plus Untitled Playlist below it]

1. Add a track with CUE points to the [CUE Analysis playlist].

2. Click the [Start] button to analyze the CUE trend information.

**Hint**

- When you update the tracks in the [CUE Analysis playlist], click the [Start] button again.
- The more consistent the trends in the CUE points of a track, the more likely CUE points that is close to your own will be able to be set.
- The more tracks you register in [CUE Analysis playlist], the more your personal CUE trend information will be reflected. If you register tracks in different genres, personal CUE trend information will be reflected even more. Please register 30 tracks or more.
- You can turn on/off the display of [CUE Analysis playlist] in the [Preferences] window.

**Note**

- The [Start] button is disabled while the track is being analyzed or if the track is not registered in [CUE Analysis playlist].
- If multiple [CUE Analysis playlist] are created, delete them manually.

## Using the Comment function

When performing CUE Analysis, you can add comments to Hot cue points or memory cue points.

1. Open the [Preferences] window > [Analysis] category > [CUE Analysis] tab, and select [Add a comment to CUE].
   After analyzing the track, comments will be added to the analyzed CUE points.

Comments of the [Auto] mode

[Screenshot: Hot Cue pad view showing A CUE(Auto), B CUE(Auto), C CUE(Auto), D CUE(Auto) with HOT CUE dropdown]

[Screenshot: Hot Cue list view showing A 00:13 CUE(Auto), B 00:27 CUE(Auto), C 00:46 CUE(Auto), D 01:13 CUE(Auto) with MEMORY, HOT CUE, INFO tabs]

Comments of the [Manual] mode

[Screenshot: Hot Cue pad view showing A 1.1Bars, B 1.1Bars+8, C 1.1Bars+16, D 1.1Bars+24 with HOT CUE dropdown]

[Screenshot: Hot Cue list view showing A 00:15 1.1Bars, B 00:29 1.1Bars+8, C 00:44 1.1Bars+16, D 00:59 1.1Bars+24 with MEMORY, HOT CUE, INFO tabs]

## Related Documents

- [guides/edit-mode.md](../guides/edit-mode.md) (cue-points, hot-cue, memory-cue)
- [manual/14-export-playing.md](14-export-playing.md) (analysis, cue-points, hot-cue)
- [manual/18-performance-screen.md](18-performance-screen.md) (cue-points, hot-cue, memory-cue)
- [manual/20-performance-playing.md](20-performance-playing.md) (analysis, cue-points, hot-cue)
- [faq/library-and-collection.md](../faq/library-and-collection.md) (analysis, playlists)
- [guides/pad-editor.md](../guides/pad-editor.md) (cue-points, hot-cue)
- [manual/29-mix-point-link.md](29-mix-point-link.md) (hot-cue, memory-cue)
- [manual/31-preferences.md](31-preferences.md) (analysis, preferences)
