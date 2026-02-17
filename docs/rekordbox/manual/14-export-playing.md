---
id: export-playing
title: "Playing tracks"
type: manual
source:
  file: rekordbox7.2.8_manual_EN.pdf
  pages: "87-108"
  version: "7.2.8"
topics: [analysis, beatgrid, cue-points, export, history, hot-cue, playback, usb, waveform]
modes: [export]
confidence: pending
last_verified: null
transcribed_by: agent
verified_by: null
---

# Playing tracks

Use the player panel to play tracks.

1. Drag and drop a track to the player panel from the track list of the browser panel.

2. Click ► in the operation panel.
   Playback starts.

## To move the playback position

Click on the enlarged waveform to move the playback position to where you click.

**Hint**

- In the [1 PLAYER]/[DUAL PLAYER] layout, you can also move the playback position by clicking [<]/[>] on the operation panel. The position will be moved by the beat count displayed in the operation panel. If clicking [<]/[>] during the loop play, not only the current playback position but also the LOOP IN and LOOP OUT points are moved. For the loop play, see "Starting loop play" (page 96).

## To listen to tracks easily using the browser panel

Click the waveform in the [Preview] column of the browser panel to start the preview from the position you clicked. Click the stop button on the left side of the waveform to stop the preview.

[Screenshot: Preview waveform in the browser panel with stop button]

When the preview starts in the [Preview] column, any other track playing in the player panel pauses.

**Previewing a track from the beginning of the track**

Click the artwork in the [Artwork] column of the browser panel.

[Screenshot: Artwork column showing play button overlay on album art]

Move the mouse cursor away from the artwork to stop the preview.

During the preview, click on the artwork to skip the preview position ahead by 30-second increments.

[Screenshot: Artwork column showing forward skip button overlay on album art]

When the preview starts in the [Artwork] column, any other track playing in the player panel pauses.

**Displaying Cue Marker in the preview waveform**

Cues and Hot Cue Markers can be displayed on the preview waveform in the track list. You can then check Cue status of the track in the track list without loading the track to the player panel.

To display Cues and Hot Cue Markers on the preview waveform, open the [Preferences] window > [View] category > [Layout] tab and check the [Display Cue Markers on Preview] checkbox of [Browser panel].

Click around Cue Marker on the preview waveform, and then the preview play can be started from the Cue point.

[Screenshot: Preview waveform showing colored Cue Markers at various positions]

**Hint**

- For setting Cues, see "Using cue points" (page 94).

**Triggering play, pause, and CUE in the enlarged waveform**

Using the mouse on the enlarged waveform, triggers the same behavior as clicking ►, **II** and [CUE].

- Left-click: the same behavior as clicking ► and **II**.
- Right-click: the same behavior as clicking [CUE].

To enable/disable this function, open the [Preferences] window > [View] category > [Display Type] tab > [Click on the waveform for PLAY and CUE], and select [Enable] or [Disable].

**Hint**

- For setting Cues, see "Using cue points" (page 94).

**Changing the color of the waveform**

Open the [Preferences] window > [View] category > [Color] tab > [Waveform color], and then select [BLUE], [RGB], or [3Band].

**Note**

- When you select [3Band], the waveform is displayed in blue for DJ equipment that does not support [3 Band].

**Changing the beat count display**

You can change the beat count displayed in the center of the enlarged waveform. Open the [Preferences] window > [View] category > [Display Type] tab > [Beat Count Display], and then select how to display the beat count.

- [Current Position (Bars)]: Display the number of bars from the beginning to the current playback position.
- [Count to the next MEMORY CUE (Bars)]: Display the number of bars from the current playback position to the next Memory Cue. If there is no next Memory Cue, no bars are displayed.
- [Count to the next MEMORY CUE (Beats)]: Display the number of beats from the current playback position to the next Memory Cue. If there is no next Memory Cue, no beats are displayed.

[Screenshot: Enlarged waveform showing "19.4Bars" beat count display in center]

## Checking the beat (beat grid)

1. Drag and drop a track from the track list in the browser panel to a player panel.

2. Click ► in the operation panel.
   Playback starts.
   - The BPM display shows the tempo measured when the track is added.
   - The enlarged waveform display shows the beat grid as a white line which is generated when the track is added (beat grid).
   - On the enlarged waveform display, the beats are indicated on the beat grid every 4 beats from the start to the end of the track. The first beat of bar is displayed as red marks (as a red vertical line on [GRID] panel) on both sides of the beat grid.

**Hint**

- [?] may appear for tracks analyzed by older version of rekordbox. This means information of the track is insufficient or old. Right-click the track, and then select [Add New Analysis Data] to update the information. [?] then disappears.

### Adjust the beat grid

The enlarged waveform displays the beat grid with white lines. The beat grid can be adjusted by the following procedure.

[Screenshot: Enlarged waveform showing beat grid with white lines and "25.4Bars" display]

1. Select [1 PLAYER] on the upper-left of the screen.
   The player panel is changed to [1 PLAYER] layout.

2. Click **II** in the operation panel during playback.
   Playback pauses.

3. Click [GRID] in the function panel.
   The beat grids changed are shown as blue lines instead of white, and can be edited.

4. Adjust the beat position.
   For icons to adjust the grid, see "[GRID] panel" (page 75).

**Note**

- You cannot adjust the beat grid when your computer is connected to DJ equipment supported with PRO DJ LINK.

### Re-analyzing tracks to detect beat positions and BPM

1. Click the settings icon to display the [Preferences] window.

2. Click the [Analysis] category > [Track Analysis] tab, and then set [Track Analysis Mode].
   Select [Normal] or [Dynamic] for analyzing waveform information of tracks.

3. Click the [BPM / Grid] checkbox in [Track Analysis Setting].

4. Close the [Preferences] window.

5. Right-click the track to be analyzed, and then select [Analyze Track].

6. Click [OK].
   The waveform information of the track is analyzed. The detected beat position and the measured BPM are saved on the computer.

**Hint**

- When tracks are re-analyzed, the beat grid set at "Adjust the beat grid" (page 90) is overwritten by the re-analyzed beat grid.

## Setting the Analysis Lock

You can set a track to ignore re-analysis and editing of the beat grid. It prevents grid-adjusted tracks from being mistakenly overwritten.

In the Analysis Lock mode, the following operations are not active.

- Track Analysis (BPM/Grid, key, phrase, and vocal)
- Grid editing operations in the [GRID] panel

When tracks (including tracks with the Analysis Lock mode) are selected to be analyzed in a track list, analysis is skipped on tracks with the Analysis Lock mode.

1. Click the lock icon in the [GRID] panel of the [1 PLAYER] layout.
   The lock icon is changed to locked and the Analysis Lock is turned on.

**Hint**

- You can also set [Analysis Lock] after right-clicking a track in a track list.
- Alternatively, set [Analysis Lock] from the [Track] menu on the upper screen.
- When setting the track with [Analysis Lock], the lock icon is displayed in the status column.

### To turn off Analysis Lock

Click the locked icon to change to unlocked. The Analysis Lock is turned off.

## Using beat grids (quantize)

When setting cues and loops in the player panel, the cue and loop points can easily be set on beat.

1. Click [Q] in the function panel.
   [Q] lights up in red.

2. Set the cue or loop points.
   For instructions on setting, see "Using cue points" (page 94) and "Starting loop play" (page 96).

### To play on a DJ player or DJ mixer using the quantize function

You can use track beat grids detected and adjusted on rekordbox, and enjoy the quantize function for your DJ performance using cues and loops on particular DJ equipment.

If a DJ player is connected by LAN cables, you can use beat grids with the quantize function for special effects.

**Hint**

- For instructions on using the quantize function on DJ equipment, refer to the Instruction Manual for the DJ equipment.

## Selecting a track for analyzing

1. Select a track to be analyzed from a track list.

2. Select [Analyze Track] from the [Track] menu on the upper screen.

**Hint**

- You can select information to be analyzed on the [Preferences] window > [Analysis] category > [Track Analysis] tab > [Track Analysis Setting].
- You can also analyze a track by right-clicking on a track to be analyzed in the track list and selecting [Analyze Track].

**Note**

- When a track has the lock icon on the track list, you cannot select [Analyze Track] (page 92).

## Using cue points

Set a cue point.

1. During playback, click **II** in the operation panel.
   Playback pauses.

2. Click [CUE] in the operation panel.
   The paused point is set as the cue point, and a cue marker is displayed on the lower side of the waveform.

3. Click ► to start playback from the cue point.

**Hint**

- When a new cue point is set, any previous cue point is deleted.
- When a different track is loaded, the previous cue point is canceled.
- You can save cue points as memory cues or Hot Cues.

### To return to the cue point (Back Cue)

During playback, click [CUE] to return the playback point to the cue point immediately. Playback pauses.

To resume playback from the cue point, click ►.

### To check a cue point (Cue Point Sampler)

After returning to the cue point, click and hold [CUE] to start playback from the cue point.

Playback continues while holding [CUE]. When released, playback returns to the cue point, and pauses.

### To set a cue point during playback (Real-Time Cue)

During playback, click [IN] in the function panel at the point to be set as a cue point.

**Hint**

- When the quantize is turned on, the cue point is automatically set to the nearest beat grid by clicking [IN].
- For the function panel, see "Function panel ([1 PLAYER] layout)" (page 74) or "Function panel ([2 PLAYER] layout)" (page 76).

### To save a cue point

After setting a cue point, click [MEMORY] in the function panel to save the cue point. The time (minute/second/millisecond) of the saved cue point is shown in the [MEMORY] panel of the playback track sub-information panel. A cue point marker is displayed on the upper side of the waveform. Up to 10 cue points can be saved for 1 track.

**Hint**

- You can use the saved cue points for your DJ performance on the DJ player.
- For the [MEMORY] panel, see "[MEMORY] panel" (page 70).
- For the function panel, see "Function panel ([1 PLAYER] layout)" (page 74) and "Function panel ([2 PLAYER] layout)" (page 76).

### Calling and playing saved cue points

1. Load the track with the saved cue point to the player panel.
   The saved cue point time (minute/second/millisecond) is shown in the [MEMORY] panel of the playback track sub-information panel. A cue point marker is displayed on the upper side of the waveform.

2. Select a cue point to be used.
   The playback point jumps to the cue point, and pauses.

3. Click ►.
   Playback starts.

**Hint**

- You can select a cue point using the function panel. ([1 PLAYER] layout)
  - ►: Select a cue forward from the current playback position.
  - ◄: Select a cue backward from the current playback position.

## Starting loop play

Set [IN] and [OUT] of the loop point.

1. Click [MA] in the function panel to display [IN] [OUT] [RELOOP].

2. During playback, click [IN] in the function panel at the point to start loop playback. (Loop In Point)
   If using a cue point as the Loop In Point, this operation is not necessary.

3. During playback, click [OUT] in the function panel at the point to end loop playback. (Loop Out Point)
   The loop playback starts from the Loop In Point.

**Hint**

- When a loop point is set, the previous loop point is deleted.
- When a different track is loaded, the previous loop point is canceled. You can save loop points.
- When the quantize is turned on. click [IN] and [OUT] to set a loop point to be matched close to the beat grid on playback.

### To cancel the loop play (Loop Exit)

During the loop play, click [EXIT] in the function panel to cancel the loop play. When reaching the loop out point, playback continues without returning to the loop in point.

### To resume the loop play (Reloop)

After cancelling a loop play, click [RELOOP] in the function panel during the playback. Loop play restarts from the previous loop in point.

### To specify the loop length by the number of beats (Auto Beat Loop)

You can play a loop with a loop length of between 1/64 and 512 beats, according to the BPM of the track.

When you click the Auto Beat Loop button, the current playback position is changed to the loop in point, and a loop is set for the selected number of beats.

**Hint**

- During a loop play, click [AU] to display [<] [number] [>], and click [<] or [>] to halve/double the loop length from the loop in point.
- When the quantize is turned on, click [IN] to set a loop in point to be matched to the beat grid position.
- To cancel a loop play, click the Auto Beat Loop button again.

### To export loops and save them as WAV files ([1 PLAYER] layout)

During a loop play, click the menu icon in the player panel, and then select [Export Loop As Wav].

- Exported loops are saved as WAV files (48 kHz sampling frequency, 16-bit quantization), and added to [Collection]. Specify the file name when saving, which will be shown as a title in [Collection].
- For instructions on using saved loops (WAV files), refer to the Instruction Manual for the DJ equipment.

### To save a loop point

During a loop play or shortly after canceling a loop play click [MEMORY] in the function panel to save a loop point for later. The time (minute/second/millisecond) of saved loop point is then shown in the [MEMORY] panel of the playback track sub-information panel. Up to 10 loop points (including cue points) can be saved for one track.

**Hint**

- You can use the saved loop points for your DJ performance on the DJ player.
- For the [MEMORY] panel, see "[MEMORY] panel" (page 70).

### Using the saved loop points

1. Load the track with the saved loop point to the player panel.
   The loop point time (minute/second/millisecond) is shown in the [MEMORY] panel of the playback track sub-information panel. A loop point (from loop in to out) is shown in yellow on the waveform.

2. Select a loop point to be called.
   The playback point jumps to the loop point, and pauses.

3. Click ►.
   The loop play starts.

**Hint**

- You can select a loop point using the function panel. ([1 PLAYER] layout)
  - ►: Select a loop point forward from the current playback position.
  - ◄: Select a loop point backward from the current playback position.

### Setting a saved loop as the active loop

When a loop point is saved, the loop icon (in orange) is displayed on the loop point.

To set the loop as an active loop, click the loop icon. The loop is set as an active loop, and the icon is changed to the loop icon (in red).

If you play on the player supports the active loop, click the menu icon in the player panel in the [1 PLAYER] layout and set [Active Loop Playback] to [On] to automatically start the loop play at the point of the active loop. You can check how active loop is performed on CDJ/XDJ series, etc. on rekordbox.

**Hint**

- For instructions on active loops, refer to the Instruction Manual for the DJ equipment.

## Using a Hot Cue

When cue and loop points are saved as Hot Cues, they can be played instantaneously. You can choose up to 8 Hot Cues per track.

**Hint**

- Hot Cues can be called out and played on DJ equipment. The number of Hot Cues you can save depends on the DJ equipment.

### To save Hot Cues

During the play or pause mode, in the [HOT CUE] panel of the sub-information panel, click the [A] - [H] (Hot Cue point) at the position you wish to save as a Hot Cue. The playing (or paused) point when the button was clicked is saved as a Hot Cue.

- When a Hot Cue point is clicked during a loop play, the currently playing loop is stored as a Hot Cue in orange.

**Hint**

- When the quantize is turned on, set a Hot Cue. When you click the Hot Cue point, it is automatically set to the nearest beat grid.
- You can save Hot Cues to click the [A] - [H] in the [CUE/LOOP] panel of the function panel. ([1 PLAYER] layout)
- For the [HOT CUE] panel, see "[HOT CUE] panel" (page 71).

### Calling and playing saved hot cue points

1. Load the track with the saved hot cue point to the player panel.
   The saved hot cue point time (minute/second) is shown in the [HOT CUE] panel of the sub-information panel. A hot cue point marker is displayed on the upper side of the waveform.

2. Select a hot cue point.
   Playback starts from the selected hot cue point.

**Hint**

- You can select a hot cue point using [A] to [H] in the [CUE/LOOP] panel in the function panel. ([1 PLAYER] layout)
- If the [Preferences] window > [Advanced] tab > [Others] tab > [Hot Cue], and [During Pause, GATE playback is applied.] is selected, switch to the [GATE] mode. In the [GATE] mode, the playback continues only when you click and hold a hot cue button during pause. Also, when you release the button, the playback pauses, and the playback point is returned to a hot cue point.

### To organize Hot Cues by using Hot Cue Bank List

Using Hot Cue Bank Lists, you can call cue/loop points of different tracks on your DJ performance.

The cue/loop point information for different tracks can be stored in Hot Cue Banks. The combination of the 8 Hot Cue Banks is called Hot Cue Bank List.

If loading multiple Hot Cue Bank Lists onto a DJ player, you can make a variety of DJ performances using Hot Cue combinations.

To display [Hot Cue Bank List] in the tree view, open the [Preferences] window > [View] category > [Layout] tab, and click the [Hot Cue Bank List] checkbox of [Media Browser].

[Screenshot: Hot Cue Bank List view showing "Hot Cue bank A" with tracks listed under slots A through H, displaying Track Title, Artist, and Time columns. Three tracks are loaded: "Inseparable" by Phil Chambers (00:54:837), "In And Out Of My Life" by Steve Richmond (00:11:240), and "Tryin' To Get Over You" by Tevin Maxwell (02:08:098)]

**Note**

- The number of Hot Cue Banks depends on the DJ equipment.
- Hot Cue Bank Lists cannot be loaded onto some DJ equipment.
- If the player panel is in the [2 PLAYER] layout, Hot Cue Bank and [X] are not displayed.

**Create a new Hot Cue Bank List**

1. Click [+] on the right side of the [Hot Cue Bank Lists] in the tree view of the browser panel.
   [Untitled Hot Cue Bank List] is added under the [Hot Cue Bank Lists] folder.

2. Type the list name using your computer, and then press the [Enter] key.

**Hint**

- To delete a Hot Cue Bank List, right-click on the Hot Cue Bank List, and then select [Delete Hot Cue Bank List]. You can also click on the Hot Cue Bank List, and then press the [Delete] key on your computer keyboard.

**Save cue and loop points as Hot Cue Banks**

During the play or pause mode, click the [A] - [H] in Hot Cue Bank List at the position you wish to save. The cue point is stored in a Hot Cue Bank. The saved point is displayed in green.

**Hint**

- When a Hot Cue Bank is clicked during a loop play, the currently playing loop point is stored and displayed in orange in a Hot Cue Bank.

**Check saved Hot Cue Banks**

In the Hot Cue Bank List, click a Hot Cue Bank of a saved cue/loop point to start its playback.

**Hint**

- To delete a cue/loop point saved in the Hot Cue Bank List, click its [X] on the right side of the Hot Cue Bank.

**Call a saved Hot Cue and save it in Hot Cue Bank**

1. Open the [Preferences] window > [Advanced] category > [Others] tab, and click the [During Pause, GATE playback is applied.] checkbox of [HOT CUE].

2. Load a track containing Hot Cue points onto the player panel.
   The cue or loop points saved in the track are displayed in the [HOT CUE] panel.

3. In the [HOT CUE] panel, click a point you wish to save in Hot Cue Bank.
   The playback position is moved to the selected point, and the play pauses.

4. Click the appropriate [A] - [H] in the Hot Cue Bank List.
   The saved point is displayed in the Hot Cue Bank.

**Organize Hot Cue Bank Lists by using folders**

1. Right-click the [Hot Cue Bank Lists] folder, and then select [Create New Folder].
   [Untitled Folder] is created under the [Hot Cue Bank Lists] folder.

2. Type the folder name using your computer keyboard, and then press the [Enter] key.

3. Drag Hot Cue Bank Lists to the created folder.
   Hot Cue Bank Lists are moved under the created folder.

**Hint**

- To delete a folder, right-click on the folder, and then select [Delete Folder]. You can also press the [Delete] key on your computer keyboard. All Hot Cue Bank lists and folders contained in the folder are deleted.

**Add an artwork image to a Hot Cue Bank List**

Valid artwork image file formats are JPEG (.jpg or .jpeg) and PNG (.png).

1. Right-click a Hot Cue Bank List, and then select [Add Artwork].

2. Select an image file, and then click [Open].
   The artwork image is added to a Hot Cue Bank List.

**Hint**

- To display the added artwork image, right-click the Hot Cue Bank List.

## Checking the play histories

Use [Histories] to check played tracks and track orders.

1. Double-click [Histories] in the tree view.
   Play histories are displayed in the year and month folders.

2. Select a play history.

## Using USB storage devices

You can load tracks and playlists from [Collection] to USB storage devices (flash memory or hard disk). Connect the USB storage device to a DJ player for your DJ performance.

[Screenshot: Devices panel in tree view showing USB MEMORY with Device Library containing All Tracks, Playlists, Hot Cue Bank Lists, and Device Library Plus with All Tracks, Playlists, Hot Cue Bank Lists]

**Hint**

- For music files (file formats) that can be loaded and played on DJ player, refer to the Instruction Manual for each DJ player.
- An SD memory card can be used for some DJ players. For details, refer to "Supported media (file system)" (page 255).
- To import music files, connect a USB storage device which stores a playlist exported from another computer. Right-click a playlist or folder in [Devices], and then select [Import Playlist] or [Import Folder].

[Screenshot: Context menu on Devices showing Add Artwork, Import Playlist, Delete Playlist, Export a playlist to a file, and Add To Shortcut options]

**Connect the USB storage device to your computer**

Insert the USB storage device into your computer, and then click the device icon. USB storage device settings are displayed in [Devices].

You can make settings for USB storage devices to use on a DJ player. Refer also to the Instruction Manual for the DJ player.

| | | |
|---|---|---|
| [General] | [Device Name] | Set the name to display. |
| | [Background Color] | Set the background color to display. |
| | [Waveform Color] | Set the waveform color to display. |
| | [Type of the Overview Waveform] | Select the display type of the waveform to display. |
| [Category] | | Specify a category and the order to display. |
| [Sort] | | Specify a sort order to display. |
| [Column] | | Specify a user setting category to display on the right side of the title, when browsing music files on the display window of DJ equipment. |
| [Color] | | Enter or edit a comment on one of the eight colors used for classifying the music files categories. |
| [My Settings] | | Set to whether all settings of all items are applied automatically. |

**Hint**

- You can change the setting of [My Settings] on the [Preferences] window > [DJ System] category > [My Settings] tab. When clicking [Apply on all connected devices], the changed settings are applied to USB storage devices.

**Note**

- File systems such as NTFS and HFS are not supported by rekordbox, thus they are displayed with a warning icon. Use FAT32 for Windows and a FAT32 or HFS+ formatted device for Mac.
- If you set [Color] to [3Band], the waveform is displayed in blue for DJ equipment that does not support [3 Band].

**Remove the USB storage device from your computer**

Click the eject icon on the right side of the device icon to remove before disconnecting the USB storage device from your computer.

**Note**

- The USB storage device must not be disconnected while the data is transferring. If you want to disconnect the device while data is being transferred, click [X] on the right of the progress bar to cancel the data transfer.

### To transfer tracks to a USB storage device

Drag tracks from [Collection] to the device icon in [Devices]. The transfer progress bar is displayed at the bottom of the screen, and both tracks and track information are transferred to the USB storage device.

**Hint**

- Tracks can also be transferred by dragging them from [iTunes] or [Explorer] to the device icon in [Devices].
- To delete a track from the USB storage device, select the track in [Devices], and then click the [Delete] key on the computer keyboard. (Tracks in the process of being transferred cannot be deleted.)
- Tracks can be transferred to 2 devices at the same time.
- For some DJ equipment, when tracks are transferred to the USB storage device, an exclusive library ([Collection] or [Playlist]) is created on the USB storage device when it is removed.

### To transfer playlists to a USB storage device

1. Click ► on the left of the device icon to transfer the playlist.

2. Click ► on the left of the [Playlists] folder.

3. Drag a playlist to [Devices].
   The playlist and the tracks contained in it are transferred to the USB storage device.

**Hint**

- A playlist can be transferred by dragging it from [iTunes] to [Devices].
- To delete the playlist from the USB storage device, select the playlist in [Devices], and then click the [Delete] key on the computer keyboard.
- Depending on the DJ equipment, there may be restrictions on the names of playlists and the folder structure on the USB storage device. For details, refer to the Instruction Manual of the DJ equipment.

### To transfer Hot Cue Bank Lists to USB storage devices

You can use combinations of Hot Cues of different tracks on DJ equipment by using a Hot Cue Bank List created prior to your DJ performance.
The available number of Hot Cue Banks depends on the DJ equipment.

1. Click ► on the left of the device icon to transfer the Hot Cue Bank List.

2. Click ► on the left of the [Hot Cue Bank Lists] folder.

3. Drag the Hot Cue Bank List to [Devices].
   The Hot Cue Bank List and the tracks contained in it are transferred to the USB storage device.

**Hint**

- To delete the Hot Cue Bank List from the USB storage device, select the Hot Cue Bank List in [Devices], and then click the [Delete] key on the computer keyboard.

### To use USB storage devices connected to a DJ player for DJ performance (USB EXPORT)

You can use USB storage devices (flash memory or hard disk) for your DJ performance. Without using a computer, you can transfer tracks, playlists, and track information (such as cue points) to the DJ player. See also "Using USB storage devices for DJ performance (USB EXPORT)" (page 83).

- An SD memory card can also be used for some DJ equipment.
- Hot Cue Bank Lists cannot be loaded onto some DJ equipment.
- For instructions on connecting USB storage devices to DJ equipment and using the USB storage devices for your DJ performance, refer to the Instruction Manual for the DJ player.

**DJ performance using the quantize function on a DJ player or DJ mixer**

The beat grid of tracks detected and adjusted beat grids using rekordbox can be used through the quantize function of cueing and looping on the DJ player. When the DJ player and DJ mixer are connected by a LAN cable, you can use detected and adjusted beat grids through the quantize function for special effects.

- For instructions on using the quantize function on the DJ player or DJ mixer, refer to the Instruction Manual for the DJ equipment.

**Using the beat sync function between DJ players for DJ performance**

If you detected and adjusted beat grids of tracks using rekordbox, you can synchronize tempos (BPM) and beats of DJ players connected via PRO DJ LINK (page 83). You can also synchronize them of the left and right decks of an all-in-one DJ system.

- For instructions on using the beat sync function on the DJ player, refer to the Instruction Manual of the DJ player.

**Using Hot Cues on a DJ player**

Hot Cue ([A] - [H]) information of tracks can be called and used on a DJ player.

- For instructions on using the Hot Cue information on a DJ player, refer to the Instruction Manual of the DJ player.
- If [Auto Load Hot Cue] is set on tracks, load these tracks onto the DJ player. Hot Cues saved in tracks are then automatically loaded onto Hot Cues on the DJ player.
- The number of Hot Cues depends on the DJ player.

**Save play histories on rekordbox from DJ equipment**

Connect the USB storage device used for your DJ performance to the computer. A play history of tracks in the device is automatically saved on rekordbox. The history is displayed in the [Histories] folder with its saved date.

- You can choose to delete or retain the history from the USB storage device.

**Note**

- When the [Preferences] window > [DJ System] category > [Device] tab > [History] > [Import the play history automatically] is not set, play histories are not imported automatically.
- To choose whether to delete or retain the play history from the device when importing it, open the [Preferences] window > [DJ System] category > [Device] tab > [History], and select [Delete from the device after importing the play history].
- When importing manually, select [Devices] in [Media Browser], and open the [Histories] folder in the USB storage device. Right-click the play history to be saved, and then select [Import History].
- Tracks in the play histories can only be exported from [Collection] of your rekordbox. Tracks exported from another rekordbox or tracks deleted from your rekordbox cannot be saved in the play histories. Such tracks are excluded.
- Some DJ players support the use of SD memory cards in addition to USB storage devices.

**Load track information updated on a DJ player**

The following information is updated.

- Cue Points and Loop Points
- Hot Cues
- Beat Grids
- Other information (colors, ratings, and comments)

1. Connect the USB storage device used for your DJ performance to the computer.

2. Right-click the device icon of the USB storage device you wish to load track information, and then select [Update Collection].
   The [Update Collection with USB Device] window is opened when the track information starts updating.

**Hint**

- If you click [Cancel] while updating, the [Update Collection with USB Device] window is closed.
- Depending on the DJ player, updated information may differ.
