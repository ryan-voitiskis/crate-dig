---
id: adding-tracks
title: "Adding a track"
type: manual
source:
  file: "rekordbox7.2.8_manual_EN.pdf"
  pages: "13-20"
  version: "7.2.8"
topics: [analysis, collection, file-formats, history, import, library, xml]
modes: [common]
confidence: verified
last_verified: "2026-02-17"
transcribed_by: agent
verified_by: agent
---

# Adding a track

To use music files saved on the computer, add them to [Collection]. Also, you can use a library saved in the rekordbox xml format or a library in another music applications.
Tracks can also be added from streaming, cloud, and mobile devices.
In the PERFORMANCE mode, video files can be added as well. For details, refer to "rekordbox video Operation Guide" on the rekordbox website.

1. Click [Collection] in the [Media Browser].
   The list of files in [Collection] is displayed as a track list.
   When [All Audio] and [All Videos] are in the tree view, click [All Audio] to display only music files or click [All Videos] to display only video files. Make sure the [Enable Video Function] checkbox is checked to display [All Audio] and [All Videos]. Open the [Preferences] window > [Extensions] category > [Video] tab, and then turn off [Enable Video Function].

2. Open the Finder or Windows Explorer, and then drag and drop files or folders to the track list.
   Tracks are added to [Collection], and their tag information is loaded and displayed.

**Hint**

- When tracks in [Collection] are displayed in the track list, right-click the column and make sure that [Attribute] is selected. With [Attribute] selected, you can click [Attribute] to display only tracks which meet the conditions selected in respective category.
- You can search for tracks by using [Date Added], [Genre], [Artist], and [Album]. For details, see "Using Collection Filter" (page 27).

**Note**

- When a WAVE file is added to [Collection], its tag information may not be displayed. Displayable tag information of music files are ID3 tag (v1, v1.1, v2.2.0, v2.3.0, and v2.4.0) of MP3 and AIFF files, the meta tag of M4A files, RIFF INFO of WAVE files, and Vorbis Comment of FLAC files.
- Depending on your subscription plan and DJ equipment connected to your computer, some functions may not be available. For details on subscription plan, refer to the rekordbox website.

## Analysis of music files

For settings on analysis, see page 230.

- There are three modes of this analysis, [Normal], [Dynamic] and [Auto]. If you select [Normal], specify the BPM range to be analyzed. The [Auto] mode is available only when [Use high precision BeatGrid analysis] is on.
- When the [Preferences] window > [Analysis] category > [Track Analysis] tab > [Auto Analysis] is set to [Enable], analysis of the music file starts after adding the file to [Collection]. The progress status is displayed on the left of the music file name. If [Auto Analysis] is set to [Disable], select a track to be analyzed, and then select [Analyze Track] by clicking [Track] menu or right-clicking the track (page 93). In addition, you can analyze a track which has already been analyzed. When analysis is completed, the waveform information is displayed on [Preview] of the browser panel (page 79) (page 143).
- To specify the information to be analyzed, open the [Preferences] window > [Analysis] category > [Track Analysis] tab > [Track Analysis Setting], and select [BPM / Grid], [KEY], [Phrase] and [Vocal].
- Cues are automatically set at the first beat of each track, and saved. Open the [Preferences] window > [Analysis] category > [CUE Analysis] tab, and check the [Set to 1.1Bars] checkbox of [CUE Setting]. Cues are set automatically when importing and re-analyzing.
- Setting the [Use high precision BeatGrid analysis] to on will result in slower analysis times, but more accurate analysis results compared to when it is set to off.

**Hint**

- [?] may appear for tracks analyzed by older version of rekordbox. This means information of the track is insufficient or old. Right-click the track, and then select [Add New Analysis Data] to update the information. [?] then disappears.

## Adding files or folders

1. Select [Import] from the [File] menu on the upper screen.

2. Select [Import Track] to select a music file, or [Import Folder] to select a folder.

3. Select a music file or folder.
   A track is added to [Collection], then its tag information is loaded and displayed.
   If Auto Analysis is turned on, analysis starts.

## Using the iTunes library

If iTunes is installed on your computer and the iTunes library XML is shared with other applications, the iTunes library can be used with rekordbox.

[Screenshot: The iTunes tree view in Media Browser showing iTunes expanded with All Audio, All Videos, and Playlists sub-items.]

[Refresh]: Update the iTunes library for its latest status.

**Note**

- For macOS Catalina and later, iTunes service is not available. This section is for customers who use iTunes on an operating system other than macOS Catalina and later, including Windows. For instructions on importing a library from the Apple Music app, see page 16.

### If the iTunes library is not displayed

Set up the following.

1. Check the [Share iTunes Library XML with other applications] checkbox of [Advanced Preferences] in iTunes.

2. Restart iTunes and rekordbox.

3. On rekordbox, click [Preferences], open the [Preferences] window > [View] category > [Layout] tab, and check the [iTunes] checkbox of [Media Browser].

4. On rekordbox, click [Preferences], open the [Preferences] window > [Advanced] category > [Database] tab, and set up [iTunes Library File] in [iTunes].

### To add a track from the iTunes library

Music files can be added to rekordbox from the iTunes library.

1. Select [iTunes] in [Media Browser], double-click [iTunes] in the tree view, and then click [All Audio].
   iTunes music files are displayed in the track list.

2. Open sub browser, drag and drop an iTunes music file from the track list to [Collection] of the sub browser.

**Hint**

- In the [iTunes] track list, [rekordbox] is shown on the track added to [Collection]. Also, waveforms, artworks, BPM, and keys analyzed by rekordbox are shown. For Auto Analysis, see "Analysis of music files" (page 14).
- To import a playlist from the iTunes library, see "Importing a playlist from iTunes" (page 45).

### To update information of the iTunes library

Right-click the track, and then select [Get info from iTunes]. The iTunes library information is updated.

**Hint**

- When the iTunes library information is updated, all information which has been changed in the [Information] window is replaced (page 45).

### Using the Apple Music app

On the Apple Music app, you can create a library XML to be imported into rekordbox.

1. In Music, click [File] > [Library] > [Export Library...].

2. Enter the information of the library XML you are creating, and then click [Save].

3. Click [Preferences], open the [Preferences] window > [Advanced] category > [Database] tab > [iTunes], and then click the [Browse] button in [iTunes Library File].

4. Select the library XML you created.
   Tracks and playlists from Music will be displayed in [iTunes] in the tree view.

## Using Inflyte

Tracks downloaded from Inflyte can be added as rekordbox tracks. Also, when Automatic Sync is on, all tracks will be imported to Dropbox automatically.
[Refresh]: Update the track list of Inflyte for its latest status.

**If [Inflyte] is not displayed in [Media Browser].**
Configure the following setting.

- Click [Preferences], open the [Preferences] window > [View] category > [Layout] tab, and check the [Inflyte] checkbox of [Media Browser].

### To add tracks from Inflyte

1. Select [Inflyte] in [Media Browser], click [Log In] of [Inflyte] in the tree view.

[Screenshot: The Inflyte login panel in the Media Browser tree view showing the Inflyte label with a Log In button.]

2. Enter your Inflyte login information, and then click [SIGN IN].
   Your web browser will launch to download the track list of [Promo Locker].
   The following icons will be displayed in the [Download] column in the track list of [Promo Locker].
   - [Heart]: tracks set as Favorite in Inflyte
   - [Synced]: tracks downloaded to [Downloaded] using Automatic Sync
   - [wav/aif/mp3]: tracks you can download

**Hint**

- To set the location to store tracks, open the [Preferences] window > [Advanced] category > [Others] tab > [Inflyte].

### To log out

Right-click [Inflyte] in the tree view, and then select [Log out].

## Using a folder or tracks on the computer or USB storage devices

You can upload a folder or tracks from the computer to [Collection].

1. Click [Explorer] in [Media Browser].
   Folders on the computer are displayed in the tree view.

2. Select a folder in [Explorer].
   Tracks in the folder are displayed in a track list.

**Note**

- If [Explorer] is not displayed in Step 1, click [Preferences], open the [Preferences] window > [View] category > [Layout] tab, and check the [Explorer] checkbox of [Media Browser].

### To add a file from the computer

1. Select a folder in [Explorer].
   Tracks in the folder are displayed in a track list.

2. Open sub browser, and then drag and drop a track from the track list to [Collection] of the sub browser.

**Hint**

- In the track list of [Explorer], the track added to [Collection] is displayed with [rekordbox]. The waveform, artwork, and analysis information such as BPM, key by rekordbox are also displayed. For Auto Analysis, see "Analysis of music files" (page 14).
- Folders in the computer or USB storage devices can be used as a playlist. For details, see "Using a folder in the computer or USB storage devices" (page 45).

## Using rekordbox xml

After importing files in the rekordbox xml format, they are displayed in the rekordbox xml library.
[Refresh]: Update the rekordbox xml library for its latest status.

**If the rekordbox xml library is not displayed**
Set up the following.

- Click [Preferences], open the [Preferences] window > [View] category > [Layout] tab, and check the [rekordbox xml] checkbox of [Media Browser].
- Click [Preferences], open the [Preferences] window > [Advanced] category > [Database] tab, and set up [Imported Library] in [rekordbox xml].

### To add a track from the rekordbox xml library

Music files can be added to rekordbox from the rekordbox xml library.

1. Select [rekordbox xml] in [Media Browser], double-click [rekordbox xml] in the tree view, and then click [All Tracks].
   rekordbox xml music files are displayed in the track list.

2. Drag and drop the music file in the track list to [Collection].

**Hint**

- To import a playlist from the rekordbox xml library, see "Importing from rekordbox xml" (page 45).

## Using [Histories]

Select [Histories] in [Media Browser] to display a list of tracks which can be played for 1 minute or more in the PERFORMANCE mode or LINK EXPORT as a history.

- PERFORMANCE mode: [HISTORY yyyy-mm-dd]
- LINK EXPORT: [LINK HISTORY yyyy-mm-dd]

Right-click a history to copy to the playlist, or to store files.

## Removing a track from [Collection]

When a track is removed from [Collection], the music file is not deleted from the computer.

1. Select a track to be removed in [Collection].

2. Press the [Delete] key on the computer keyboard.

3. Click [OK].
   The music file is removed from [Collection].

**Hint**

- A track can be also removed by right-clicking the track and selecting [Remove from Collection].
- You can choose to just remove a track in Inflyte from [Collection], or completely delete the file from the computer.

## Related Documents

- [faq/library-and-collection.md](../faq/library-and-collection.md) (analysis, collection, import, library)
- [reference/developer-integration.md](../reference/developer-integration.md) (import, library, xml)
- [reference/xml-import-export.md](../reference/xml-import-export.md) (import, library, xml)
- [features/overview.md](../features/overview.md) (collection, library)
- [guides/cloud-library-sync.md](../guides/cloud-library-sync.md) (collection, library)
- [guides/introduction.md](../guides/introduction.md) (collection, library)
- [guides/streaming-services.md](../guides/streaming-services.md) (collection, library)
- [guides/xml-format-spec.md](../guides/xml-format-spec.md) (import, xml)
