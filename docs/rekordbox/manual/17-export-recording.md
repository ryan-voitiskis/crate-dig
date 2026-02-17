---
id: export-recording
title: "Recording"
type: manual
source:
  file: "rekordbox7.2.8_manual_EN.pdf"
  pages: "118-120"
  version: "7.2.8"
topics: [analysis, devices, export, file-formats, recording]
modes: [export]
confidence: verified
last_verified: "2026-02-17"
transcribed_by: agent
verified_by: agent
---

# Recording

You can record sound input through hardware connected via USB. For example, when connected to a turntable via USB, you can record source music from an analog record. Also, you can record your DJ performance by connecting your computer to an audio interface, such as a DJ mixer.
A recording is created as a WAV file.
In the EXPORT mode, only audio input from external equipment can be recorded. To record from Master Out of rekordbox, use the recording function in the PERFORMANCE mode. (page 173)

1. Click the recording icon on the top of the screen.
   The recording panel is displayed.

2. Select a recording source.
   The recording source shown in the menu depends on the connected hardware.
   - When PLX-500 is connected, select [USB REC].
   - When DJM-900NXS2 is connected, select [Master Out], [Channel 1], [Channel 2], [Channel 3], or [Channel 4].
   - When DJM-900NXS is connected, select [Channel 1], [Channel 2], [Channel 3], or [Channel 4].
   - Depending on the recording sources and the connected hardware, settings of the connected hardware may be necessary. For details, refer to the operating instructions of the connected hardware.

3. Turn the recording level knob to adjust the recording level.
   Start playback of some tracks on the connected hardware, check the recording level indicator, and adjust the recording level accordingly.
   If the level is too low, recorded files will contain noise and the volume will be low. If the level is too high, clipping will occur and the sound will be distorted.
   The best recording quality can be obtained by increasing the maximum recording level to the point where not all the indicators hit the red zone.

4. Click the record button.
   The record button flashes and rekordbox is in the recording standby mode.

5. Play a track to be recorded on the connected hardware.
   Recording starts when audio input is received. The record button changes to the recording indicator, and the elapsed time and remaining time for recording are shown.

6. Click the recording indicator.
   Recording stops.
   When no sound continues more than 20 seconds, recording stops automatically.

7. Enter information about the recorded file.

8. Click [OK].
   The recorded file is saved on [Recordings] in the tree view of the browser panel.

**Hint**

- To avoid automatic start/stop recording, open the [Preferences] window > [Advanced] category > [Recordings] tab, and set [Recording starts when:]/[Recording ends when:] to [Pressing the REC button].

### To separate the recorded file automatically

rekordbox has a feature to automatically create another file when it detects a certain length of silence. If you enable this function, you can create audio files per track when recording an analog record (if it has multiple tracks and breaks between them).

The length of the silence can be set from 1 to 10 seconds.

Open the [Preferences] window > [Advanced] category > [Recordings] tab, and adjust the settings of automatic separation as follows.

- If the file does not separate;
  [Silence Level Threshold]: Move the slider to right to increase the threshold.
  [When silence continues for]: Decrease the length of silence.
- If the file separates unnecessarily;
  [Silence Level Threshold]: Move the slider to left to decrease the threshold.
  [When silence continues for]: Increase the length of silence.

See "Other settings for recordings" (page 120) in "EXPORT mode".

**Hint**

- When the automatic separation feature is enabled, a screen with [Will you save the separate recording files?] is displayed after recording. If you click [No], the recording is saved as one file (not separated).

**Note**

- This feature separates a file by detecting silence. Therefore, if there is no silence between tracks, the recorded file is not separated. Conversely, if a track has a silent or a low volume-level part, the file may be separated there. If this feature does not work correctly even after these setting values are adjusted, open the [Preferences] window > [Advanced] tab > [Recordings] tab, and do not select the [Automatically separate files when silence continues].

### To display recorded files

1. Click ► on the left of [Recordings] in the tree view of the browser panel.
   Folders containing recorded files are shown in the tree view.

[Screenshot: Recordings folder expanded in tree view showing Contents folder with subfolders organized by artist, plus UnknownArtist, iTunes, and PioneerDJ folders]

2. Select a file in the track list, and then drag and drop it to the player.
   To start playback, click ► in the operation panel.

**Hint**

- If you have input tag information, the file is shown in [Album] in [Artist] under [Recordings]. If you have not input tag information, click [Recordings] to display the recorded files in the track list.
- To delete unwanted recorded files, right-click the file and select [Remove Recorded File].

**Note**

- Folders in [Recordings] cannot be edited in the tree view. To edit the name of a folder, etc., use the rename facility in Finder or Windows Explorer.

### Other settings for recordings

See "[Recordings] (EXPORT mode)" (page 240).

## Related Documents

- [manual/19-performance-preparing.md](19-performance-preparing.md) (devices, export, recording)
- [faq/cloud-and-sync.md](../faq/cloud-and-sync.md) (analysis, export)
- [faq/hardware-compatibility.md](../faq/hardware-compatibility.md) (export, file-formats)
- [faq/library-and-collection.md](../faq/library-and-collection.md) (analysis, export)
- [faq/usb-and-devices.md](../faq/usb-and-devices.md) (devices, export)
- [guides/device-library-backup.md](../guides/device-library-backup.md) (devices, export)
- [guides/usb-export.md](../guides/usb-export.md) (devices, export)
- [manual/03-adding-tracks.md](03-adding-tracks.md) (analysis, file-formats)
