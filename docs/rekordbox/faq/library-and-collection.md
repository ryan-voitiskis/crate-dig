---
id: faq-library-and-collection
title: "FAQ: Library & Collection"
type: faq
source:
  file: "rekordbox7-faq.md"
  url: "https://rekordbox.com/en/support/faq/rekordbox7/"
  version: "7.x"
topics: [analysis, collaborative-playlists, collection, export, import, library, metadata, playlists, track-suggestion]
modes: [common]
confidence: verified
last_verified: "2026-02-17"
transcribed_by: agent
verified_by: agent
---

### Can I automatically export XML format collections?

By performing settings with the following procedures, you can automatically export collections when closing rekordbox.

1. Open [Preferences] > [Advanced] category > [Others] tab.
2. Enable [XML Auto Export] in [Tribe XR].
3. Specify a saving destination on [Location of the xml file on dropbox].

---

### Can the rekordbox library be used on Tribe XR?

Yes you can. It can be used by exporting collection in XML format.
Export collection in XML format with the following procedures.

1. Select [File] menu > [Export Collection in xml format].
2. Copy the exported XML file on to Dropbox.
3. Connect to Dropbox from Tribe XR and load the exported XML.

---

### The BPM and keys of tracks on Apple Music are not shown.

To show the BPM and keys of tracks on Apple Music, you need to import the tracks to rekordbox and perform track analysis.

---

### Play histories in a USB storage device connected in PERFORMANCE mode are not imported. The context menu for [import History] is grayed out.

Importing play histories is limited to EXPORT mode.
Switch to EXPORT mode to use this function.

---

### Even if [Import the play history automatically] is set to ON, there are some play histories that are not imported.

Even if [Import the play history automatically] is set to ON, the same name and contents as the previously imported play history will not be imported automatically.

You can manually import the play histories in the following ways.
Select the [Display Devices] icon in the Media Browser, select the play history you wish to import from the [Histories] folder in the USB storage device (or SD memory card), right-click and select [import History] from the context menu.

---

### After importing the play histories in a USB storage device, the play histories are deleted from the device.

The default setting is to delete the play histories from the device when importing the play histories in the USB storage device (or SD memory card).

By changing the following settings, you can keep the play histories on your device.
Turn OFF [Prefernces] > [Devices] category > [Delete from the device after importing the play history].

---

### After updating to ver. 7, do I have to set cue points and beatgrid information for my tracks again?

No, you don't have to set them again.

When you run ver. 7 for the first time, your current library will be converted for use with ver. 7.

All information in your library, such as cues, beatgrid, and playlists, will be copied over. (Alternatively you can select [File] menu > [Library] > [Import your library from ver.5 (or earlier)].)

---

### [Collection Radar] and [Streaming Radar] are not displayed in [Track Suggestion].

Enable [Preferences] > [Analysis] category > [Track Analysis] tab > [Radar Analysis]
to use [Collection Radar] and [Streaming Radar].

---

### What is Track Suggestion?

Track Suggestion is a recommendation function that displays recommended tracks as a list based on the loading track and tracks specified in the list.

You can select up to 5 recommendations, including [Era], [Mood], [Association], [Collection Radar], and [Streaming Radar] with Track Suggestion.

While [Era], [Mood], and [Association] make recommendations based on track meta data, [Collection Radar] and [Streaming Radar] recommend tracks with similar musical characteristics based on unique algorithms developed using AI learning.

- Era
Prioritizes the display of tracks with similar year, BPM, etc. from [Collections].
- Mood
Prioritizes the display of tracks with similar genre, BPM, etc. from [Collections].
- Association
Prioritizes the display of tracks with similar label, composer, etc. information from [Collections].
- Collection Radar
Prioritizes the display of tracks with similar musical characteristics from [Collections].
- Streaming Radar
Prioritizes the display of tracks with similar musical characteristics from the streaming service.

Track Suggestion can be displayed using the sub-panel button located in Tree View or on the right side of the browse screen.

---

### When analyzing tracks, [Vocal] is grayed out and I can't select it.

This might occur in the following cases.

- Free Plan or Core Plan users

You'll need Creative or Professional Plan to use Vocal Position Detection and display.
For details on the features available in each plan, see [here](https://rekordbox.com/en/plan/).

- Your computer doesn't meet the system requirements for rekordbox

The system requirements for rekordbox is [here](https://rekordbox.com/en/download/#system).

---

### After I updated rekordbox to ver. 7, [?☆] is displayed on a track list.

On rekordbox ver. 7, [Collection Radar/Streaming Radar] is available under [Track Suggestion] as a new recommendation function.
The [?☆] icon is displayed for tracks that do not have the analysis data required for recommendation.

Tracks with the [?☆] icon displayed cannot be recommended as a base track with [Collection Radar/Streaming Radar].
Even if tracks with analysis data are being used as base tracks, tracks without analysis data are not recommended.

To fix the problem, right-click the track and select [Add New Analysis Data] from the context menu.

---

### How do I drag and drop tracks in [Collections] to add them to the playlist?

Display the sub browser window and drag and drop tracks you want to add to the playlist from [Collections] to add them.

*Vertical display and horizontal display is available as display methods for the sub browser window.

Selecting the "vertical" displays the browser window and sub browser window lined up next to each other vertically.

![](https://cdn.rekordbox.com/files/20240510084224/Sub-Brws-Low-EN-300x115.png)

Selecting the "horizontal" displays the browser window and sub browser window lined up next to each other horizontally.

![](https://cdn.rekordbox.com/files/20240510084219/Sub-Brws-Side-EN-300x115.png)

---

### After I enter my email address to the AlphaTheta account entry window and press "OK", [Unknown error.] is displayed.

Possible reasons are as follows.

- The email address was entered incorrectly.
- The member you are inviting to share the playlist is not subscribed to the Professional plan.
- The maximum number of shares (100 shares) has been reached or will be exceeded for the playlist.

---

### The streaming service tracks I added to the collaborative playlist are not being displayed on the shared members' collaborative playlists.

The shared members need to log in to the streaming service for the streaming service tracks to be displayed.

The shared members must be subscribed as well as logged in to the same streaming service.

---

### My collaborative playlist reverted back into a normal playlist on its own.

Possible reasons are as follows.

- The owner has deleted the collaborative playlist.
- Changes have not been made to the collaborative playlist for 30 days.
- Your Professional plan was canceled or deactivated.
- Your Professional plan subscription has expired.

---

### The playlist tracks shared by the owner go missing on the collaborative playlist.

Collaborative playlists cannot be used to share track data (music files).
Please use a different method to receive the tracks from the owner and follow the steps below to locate the missing tracks.

1. Receive the tracks and save the music files in any folder (it is recommended to create a new folder).
2. Select [Preferences] > [Advanced]category > [Database]tab and add a checkmark to the [Specified user folders] checkbox of  the [Auto Relocate Search Folders].
Add the folder from Step 1.
Remove the checkmark from all the items except for [Specified user folders].
3. Open the [File] menu and select [Display All Missing Files].
Check that the collaborative playlist tracks are displayed in the [Missing File Manager] window.
*If tracks other than the collaborative playlist tracks are displayed, add the folder those tracks are saved in using the points in Step 2.
4. In the [Missing File Manager] window, click the [Auto Relocate] button.

---

### Can I use a collaborative playlist to share track data (music files)?

Track data cannot be shared with a collaborative playlist. To share tracks that are saved locally on a Windows or Mac computer, the tracks need to be shared using a separate procedure.

When sharing with other members a playlist created based on the tracks on a streaming service, all those members need to be subscribed to that streaming service.

---

### Please tell me more about the information that is shared through a collaborative playlist.

- Information about the track such as the track title/artist name and information such as CUE/Grid will be shared when joining the Collaborative Playlist for the first time.
- Edits such as adding/deleting track and changing the order of tracks are synchronized at any time and shared among members.
- Edits such as the track title/artist name and CUE/Grid can be shared among members by updating a Collaborative Playlist.

---

### What do I have to do to use Collaborative Playlist?

Collaborative Playlist can be used by Professional plan users.

Follow the steps below.

1. Select a playlist you want to share.
Select [PLaylists] menu > [Collaborative Playlist] > [Share playlist].
2. Enter the AlphaTheta account (email address) of the person you want to join as a shared member.
*A notification and invitation email are sent to the shared member's [NOTIFICATION]tab.
3. Have the shared member accept the invitation.
*Click the [Join the collaborative playlist] link to open the web acceptance window. Add a check to the checkbox for the playlist you wish to join and click the Accept button.

---

### What is the Collaborative Playlist?

Collaborative Playlist is a function that allows a user to share a playlist with other users and, in turn, also share with those users the information displayed in the tracklist, including the title, artist, and preview waveform of each track.

Up to 5 users including the owner can share a Collaborative Playlist.

The members who have accepted the owner's invitation to share the Collaborative Playlist are referred to as the shared members.

If you have the same track data (music files) as the owner, or if you are subscribed to the same streaming service as the owner and share the streaming service tracks, more detailed information including the CUE, GRID, and Phrases can also be shared.

---

### Can I convert my rekordbox ver. 7 library to a ver. 5 library?

It's not possible to convert a rekordbox ver. 7 library to a ver. 5 library.

---

### I started rekordbox ver. 7 with an empty library. How can I convert my library from ver. 5 (or earlier) and import it after starting ver. 7?

Go to [File] > [Library] and select [Import your library from ver. 5 (or earlier)].

Follow the on-screen instructions to convert the library.

---

### I'm updating from rekordbox ver. 5 to ver. 7. Tell me about the new library in rekordbox ver. 6.

You'll need to convert the library you used with rekordbox ver. 5 (or an earlier version), to use it with ver. 6.

If rekordbox ver. 5 (or earlier) is installed on your computer, you'll see a button to begin the library conversion when you start rekordbox ver. 7 for the first time.

Follow the on-screen instructions to start converting your library.

Please note that the library conversion may take some time and you won't be able to use rekordbox ver. 6 until the conversion has finished.

You can use rekordbox ver. 6 without importing your library from ver. 5 (or earlier). In this case you would use rekordbox with an empty library.

You can also start rekordbox ver. 6 with an empty library, convert your old library from ver. 5 (or earlier), and then import the library to ver. 6 at a later. date Find out how to do this [here](https://rekordbox.com/en/support/faq/library-6/#faq-q60002).

---

### CUE (auto) is running on track analyze, but HOT CUE and MEMORY CUE are not set on the desired points.

HOT CUE and MEMORY CUE are automatically set using Phrase analysis information on CUE Analysis (auto).
To improve CUE Analysis precision, perform the following procedures and generate your CUE point information (personal CUE trend information).
This may improve the positioning of HOT CUE and MEMORY CUE to the desired points.

1. Add tracks with CUE point set to them to [CUE Analysis Playlist]
*Approximately over 30 tracks recommended
*CUE points set as phrase separators are required
2. Click the [Start] button on the right side of [CUE Analysis Playlist]
*Analyzes the CUE point and generates personal CUE Tendency information.
The more similar track CUE point tendencies are, the more closer you can set the CUE points to your CUE points.
When adding or deleting tracks from your playlist, click the [Start] button and update your personal CUE Tendency information.
3. Run CUE Analysis (auto) again
*If [Prohibit overwriting CUEs] is off, the latest analysis results will be reflected when running CUE Analysis again.

---

### HOT CUE and MEMORY CUE are not set according to the amount determined in CUE Analysis.

HOT CUE and MEMORY CUE may not be set on CUE Analysis if track phrases cannot be analyzed or the track is too short.

If the amount of track phrases and bars is too low in number, the amount of HOT CUE and MEMORY CUE set in CUE Analysis may become lower compared to the amount set in HOT CUE and MEMORY CUE [Upper limit].

The amount of MEMORY CUE may be lower than the amount before CUE Analysis if [Prohibit overwriting CUEs] is off.

---

### The caution "You have reached the number of times to limit API call. Please wait a while and try again." is displayed when Streaming Radar is selected.

This message is displayed since there is currently concentrated access on the server where recommended track information is stored.

Wait a while and try again.

---

### Tracks are not displayed even if Collection Radar or Streaming Radar is selected.

To use [Collection Radar] and [Streaming Radar], recommendation analysis data is required.
Recommendation analysis data is generated when importing, loading, or reanalyzing tracks.

An [?☆]icon is displayed on imported tracks that do not have recommendation analysis data.
If setting a track without analysis data as the base track, tracks will not be recommended.
Additionally, even if a track with analysis data is set as the base track, tracks without analysis data are not recommended.

Right-click the track and select [Add New Analysis Data] from the context menu.

If the number of tracks is extremely low or if the recommendation conditions are set narrowly, tracks may not be displayed.
Increase the number of tracks with recommendation analysis data or review recommendation conditions.

---

### Is reanalysis of BeatGrid, etc. required to use Collection Radar and Streaming Radar?

Reanalysis of BeatGrid is not required.

Right-click the track and select [Add New Analysis Data] from the context menu.

If running [Add New Analysis Data] for all tracks in the library, perform operations following the procedures below.

1. Click Collection
2. Click the track list and place the cursor on the track list
3. Press Ctrl+A and select all tracks
4. Right-click and select [Add New Analysis Data] from the context menu

If tracks that already have recommendation analysis data exist when all tracks are selected, the addition of analysis data for that track is skipped automatically.

---

### Is library conversion required when updating from ver. 6 to ver. 7?

No, it is not. rekordbox ver. 7 is available without library conversion when updating from ver. 6 to ver. 7.

---

### Do I need to reanalyze tracks to use Collection Radar/Streaming Radar?

You don't have to reanalyze tracks.

Right-click the track and select [Add New Analysis Data] from a context menu.

Follow the steps below to select [Add New Analysis Data] for the all the tracks of the library.

1. Click Collection
2. Click a track list to set the cursor on the track list.
3. Press [Ctrl+A] to select all tracks.
4. Right-click and select [Add New Analysis Data] from the context menu.

If the selected track contains a track that already has the analysis data for recommendation, the analysis will automatically skip the track.

---

### What is Tribe XR?

Tribe XR is a platform that enables users to learn DJ skills on authentic DJ equipment in VR.

---

### What is the Cloud Analysis?

Cloud Analysis is a service that shortens track analysis time by downloading analyzed information (BPM, BEAT GRID, waveforms, and vocal position) of tracks from the server provided by AlphaTheta.

---

### I changed the file extension of music files that are registered in the Collection. Will this change be applied to the Collection?

No, for the change to be applied to the Collection, the music files need to be reimported to the Collection after the file extension is changed.
*Because the waveform, time, CUE point, etc. data change when the file extension is modified, data from before this modification cannot be used as is.

Relocate and Auto Relocate also cannot be used when the music files are listed as Missing Files.

---

### Which file formats can I render an audio file in?

You can render audio files in the following formats.  (Version 6.5.1 and later)

Mac: FLAC, AIFF, WAV, M4A(128, 192, 256kbps)

Windows: FLAC, AIFF, WAV, M4A(128, 192kbps), MP3(128, 192, 256, 320kbps)

Note: Audio files rendered in M4A format may not be supported by some CDJ and XDJ players. If that is the case, please select a different format.

---

### Can I create a new audio file by editing multiple tracks?

You can only edit single tracks in EDIT mode.

---

### What is a project?

A project is a record of your track edit.

When you save a project, you can close rekordbox then continue editing the track the next time you launch the application.

---

### What is Inflyte?

Inflyte is a music promotion platform.

Promoters and record labels use it to share tracks with DJs so they can play them in their sets.

---

### When I imported recorded files to iTunes, the Tag information I entered when recording was not reflected to iTunes.

The artist name, album title and track title information you entered when recording is written in the recorded file and in your rekordbox library but not in your iTunes library.

---

### Does my rekordbox ver. 6 library sync with the one from ver. 5 (or earlier)?

Imported tracks and information such as cue points in one version won't be synced with the other version.

---

### What is [Merge Duplicate Files]?

[Merge Duplicate Files] is a feature that consolidates tracks with the same Title and Artist in the rekordbox library into a single representative file.

rekordbox automatically detects duplicated files and lists them in the [Duplicated File Manager] screen. To merge, select the tracks from the list and press the [Merge] button on the [Select Representative File] screen.

After merging, one of the selected files will remain as the representative file, while the other files are deleted from rekordbox's [Library]. All information from the merged files will be replaced by the information of the representative file.

Caution: Since duplicated files other than the representative file will be deleted from the Library after merging, it is recommended to review the contents in advance and back them up if necessary.

Note: The merged files will no longer be managed by rekordbox, but the actual music files on your computer will not be deleted.

---

### Are there any types of files I can't edit?

You can't edit the following types of files:

- Tracks from streaming services
- Video files
- Copyright-protected music
- Files with the extension ".rbsc" extracted from the DRUMS Stem

Note: Audio files rendered in M4A format may not be supported by some CDJ and XDJ players. If that is the case, please select a different format.

---

## Related Documents

- [manual/03-adding-tracks.md](../manual/03-adding-tracks.md) (analysis, collection, import, library)
- [manual/07-playlists.md](../manual/07-playlists.md) (collection, export, import, playlists)
- [manual/09-collaborative-playlists.md](../manual/09-collaborative-playlists.md) (collaborative-playlists, export, import, playlists)
- [reference/developer-integration.md](../reference/developer-integration.md) (export, import, library, metadata)
- [reference/xml-import-export.md](../reference/xml-import-export.md) (export, import, library, metadata)
- [features/overview.md](../features/overview.md) (collection, export, library)
- [guides/cloud-library-sync.md](../guides/cloud-library-sync.md) (collection, library, playlists)
- [guides/introduction.md](../guides/introduction.md) (collection, export, library)
