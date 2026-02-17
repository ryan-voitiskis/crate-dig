---
id: performance-playing
title: "Playing tracks"
type: manual
source:
  file: rekordbox7.2.8_manual_EN.pdf
  pages: "150-172"
  version: "7.2.8"
topics: [analysis, beatgrid, cue-points, hot-cue, key, mixing, playback]
modes: [performance]
confidence: pending
last_verified: null
transcribed_by: agent
verified_by: null
---

# PERFORMANCE mode

# Playing tracks

Use the player panel to play tracks.

**Hint**

- For the JOG panel, see "JOG panel" (page 125).
- For the browser panel, see "Browser panel" (page 143).

1. Drag and drop a track to a deck of the player panel from the track list of the browser panel.

2. Click ▶ in the JOG panel of the deck.
   Playback starts.

## To adjust the playback speed (tempo control)

The tempo increases when you click [+] and decreases when you click [-] on the platter during playback.

**Hint**

- The changed rate (%) is indicated on the platter.
- Double-click the rate (%) to reset the tempo. (tempo reset)

**Note**

- When [-100%] is set, playback stops.

## To change the range of the playback speed

Click [Tempo Range] in the platter to change the playback speed by 0.02% at [±6] / 0.04% at [±10] and [±16] / 0.5% at [WIDE].

## To adjust playback speed without changing the pitch (Master Tempo)

Click [MT] on the platter which is being played. The [MT] lights up in red. The pitch of the track does not change even when you change the playback speed with [+]/[-] on the platter.

**Note**

- The sound quality may change because the sound is digitally reprocessed.

## To listen to tracks easily using the browser panel

See "To listen to tracks easily using the browser panel" (page 87) in "EXPORT mode".

---

# Checking the beat (beat grid)

1. Drag and drop a track from the track list of the browser panel to the player panel.

2. Click ▶ in the JOG panel.
   Playback starts.
   - The BPM display shows the tempo measured when the track is added.
   - The enlarged waveform display shows the beat grid as a white line which is generated when the track is added (beat grid).
   - On the enlarged waveform display, the beats are indicated on the beat grid as a red line every four beats, from the start to the end of the track.

**Hint**

- [?] may appear for tracks analyzed by older versions of rekordbox. This means information of the track is insufficient or old. Right-click the track, and then select [Add New Analysis Data] to update the information. [?] then disappears.

## To adjust the beat grid

The enlarged waveform displays the beat grid with white lines. The beat grid can be adjusted by the following procedure.

[Screenshot: Enlarged waveform display showing beat grid with white vertical lines and colored waveform]

1. Click ∎∎ in the JOG panel during playback.
   Playback pauses.

2. Drag the enlarged waveform display to the left or right.
   Adjust the beat to match the vertical white line at the center of the enlarged waveform display.

3. Click the grid edit icon to display the [GRID EDIT] panel.
   The beat grids changed are shown as blue lines instead of white, and can be edited.

4. Click the snap icon.
   The entire beat grid is slid based on the white vertical line which is placed on the center of the enlarged waveform. The center beat is changed to the beginning of a bar, and is displayed as a red line.
   To slide the beat grid beyond a specific point, click the partial snap icon. Beat grid points, passing through the white center vertical line, are locked, and only beat grid points coming to the white center vertical line, are slid.

**Hint**

- For icons displayed in the [GRID EDIT] panel, see "[GRID/PHRASE EDIT] panel" (page 135).

### Re-analyzing tracks to detect beat positions and BPM

See "Re-analyzing tracks to detect beat positions and BPM" (page 91) of "EXPORT mode".

---

# Setting the Analysis Lock

You can set a track to ignore re-analysis and editing of the beat grid. It prevents grid-adjusted tracks from being mistakenly overwritten.
In the Analysis Lock mode, the following operations are not active.

- Track Analysis (BPM/Grid, key, phrase, and vocal)
- Grid editing operations in the [GRID EDIT] panel

When tracks (including tracks with the Analysis Lock mode) are selected to be analyzed in a track list, analysis is skipped on tracks with the Analysis Lock mode.

1. Click the grid edit icon to display the [GRID EDIT] panel (page 135) if the performance pad is displayed on the deck where the track to be analysis locked is loaded.

2. Click the unlock icon.
   The icon is changed to the lock icon and the Analysis Lock is turned on.

**Hint**

- Right-click the track in the track list, and then set [Analysis lock] to [On]. When [Analysis lock] is set, the lock icon is shown on the status column.
- You can also set [Analysis Lock] from the [Track] menu on the upper screen.

## To turn off Analysis Lock

Click the lock icon to change to the unlock icon. The Analysis Lock is turned off.

---

# Using beat grids (quantize)

When setting cues and loops in the player panel, the cue and loop points can easily be set on beat. The quantize can be set independently on decks.

1. Click [Q] in the JOG panel for each deck.
   [Q] lights up in red.

2. Set the cue or loop points.
   For instructions on setting, see "Using cue points" (page 155) and "Starting loop play" (page 157).

**Hint**

- When the beat setting of the quantize is set to a fraction (1/16, 1/8, 1/4, 1/2), [BEAT SYNC] of tracks is disabled.
- When the fraction loop (1/32 to 1/2) is set, [BEAT SYNC] of tracks is disabled.
- To set the quantize for the sampler deck, click [Q] on the sampler deck. [Q] lights up in red.
- When turning on [BEAT SYNC] (page 162) and the quantize, you can keep the rhythm synchronized with the beat position even when you play Hot Cues and loop.
- When turning on the quantize, you can set the quantize on all decks to be turned on automatically. Open the [Preferences] window > [Controller] category > [Others] tab > [Setting], and select [All Decks].

---

# Selecting a track for analyzing

See "Selecting a track for analyzing" (page 93) of "EXPORT mode".

---

# Using cue points

A cue point can be set for each player deck.

1. During playback, click ∎∎ in the JOG panel.
   Playback pauses.

2. Click [CUE] in the JOG panel.
   The paused position is set as the cue point, and the cue mark (in orange) is shown on the enlarged waveform display.

3. Click ▶.
   Playback starts from the cue point.

**Hint**

- When a new cue point is set, any previous cue point is deleted.
- When a different track is loaded, the previous cue point is canceled.
- You can save cue points as memory cues or Hot Cues.

**Note**

- When the quantize is turned on, the cue point may be set at a slightly different position from the point you manually set. For details, see "Using beat grids (quantize)" (page 154).
- The headphones cue is a different function from cue point function. For details, see "Using the microphone feature" (page 169).

## To return to the cue point (Back Cue)

During playback, click [CUE] to return the playback position to the cue point immediately. Playback pauses.
To resume playback from the cue point, click ▶.

## To check a cue point (Cue Point Sampler)

After returning to the cue point, click and hold (holding the left click button of the mouse) [CUE] to start playback from the cue point.
Playback continues while holding [CUE]. When released, playback returns to the cue point, and pauses.

## To set a cue point during playback (Real-Time Cue)

During playback, click [IN] in the JOG panel at the point to be set as a cue point.

**Hint**

- When the quantize is turned on, click [IN] to set the cue point to the nearest beat grid. (page 154)
- For the JOG panel, see "JOG panel" (page 125).

## To save a cue point

After setting a cue point, select [MEMORY CUE] on the performance pad, and click [MEMORY] in the [MEMORY CUE] panel to save the cue point. 10 cue points can be saved for a track.

**Hint**

- You can use the saved cue points for your DJ performance on the DJ player.
- For the [MEMORY CUE] panel, see "[MEMORY CUE] mode" (page 134).

### Using saved cue points

1. Load the track with the saved cue point to the player panel.

2. Select [MEMORY CUE] on the performance pad.
   The cue point time (minute/second/millisecond) is shown in the [MEMORY CUE] panel.

3. Select a cue point to be used.
   The playback position jumps to the cue point, and pauses.

4. Click ▶.
   Playback starts.

---

# Starting loop play

You can start a loop in two ways. For the manual loop, set its start position and end position. For the auto beat loop, set its start position and the length of the loop in number of beats.

**Note**

- When a supported DJ controller is connected, you cannot operate the loop play on rekordbox. In this case, operate the loop play on the DJ controller.

## To set the auto beat loop

1. Click [AU] in the JOG panel to display as following.

   [Screenshot: Auto beat loop controls showing beat count "4", left/right arrows, and AU/MA buttons]

2. Click [<]/[>] to select the length of loop (beat).

3. During playback, click the auto beat loop (beat count) in the JOG panel at the point to start the auto loop. [AUTO LOOP] lights up.
   The loop play starts.

4. To cancel the loop play, click the auto beat loop (beat count) again.

## To set the manual loop

1. Click [MA] in the JOG panel to display as following.

   [Screenshot: Manual loop controls showing IN, OUT buttons, RELOOP button, and AU/MA buttons]

2. During playback, click [IN] at the point to start loop playback (Loop In Point).
   The cue point is also set.

3. During playback, click [OUT] at the point to end loop playback (Loop Out Point).
   [IN] and [OUT] light up, and the loop playback starts from the Loop In Point.
   [RELOOP] changes to [EXIT].

4. To cancel the loop play, click [EXIT].
   During playback, click [RELOOP] to call the last setting loop to start the loop play.

**Hint**

- When a loop point is set, the previous loop point is deleted.
- When a different track is loaded, the previous loop point is canceled. You can save loop points.
- When the quantize is turned on, set a loop point to be matched close to the beat grid on playback.

## To save the loop

During the loop play, select [MEMORY CUE] on the performance pad, and click [MEMORY] in the [MEMORY CUE] panel to save the loop point. 10 loop points can be saved for a track.

**Hint**

- You can use the saved loop points for your DJ performance on the DJ player.
- For the [MEMORY CUE] panel, see "[MEMORY CUE] mode" (page 134).

### Using saved loop points

1. Load the track with the saved loop point to the player panel.

2. Select [MEMORY CUE] on the performance pad.
   The cue point time (minute/second/millisecond) is shown in the [MEMORY CUE] panel.

3. Select a loop point to be used.
   The playback position jumps to the loop point, and pauses.

4. Click ▶.
   Playback starts.

### Setting a saved loop as the active loop

When a loop point is saved, the active loop icon (in orange) is displayed on the right side of loop point of [MEMORY CUE] panel.
To set the loop as an active loop, click the active loop icon. The loop is set as an active loop, and the icon is changed to the active loop icon (in red).

**Hint**

- For instructions on active loops, refer to the Instruction Manual for the DJ equipment.

---

# Using a Hot Cue

When using a Hot Cue, you can start playback from the cue and loop points quickly. You can choose up to 16 Hot Cues per track.

**Hint**

- For the JOG panel, see "JOG panel" (page 125).
- For the [HOT CUE] panel, see "[HOT CUE] mode" (page 127).

## To save a Hot Cue

1. Select [HOT CUE] on the performance pad.
   The [HOT CUE] panel is displayed.

2. During playback, click [A] - [P] performance pads at the point to be set as a Hot Cue.
   The point is saved as a Hot Cue.

**Hint**

- When a Hot Cue point is clicked during a loop play, the currently playing loop point is stored as a Hot Cue. (HOT CUE LOOP)
- When the quantize is turned on, set a Hot Cue. When you click the Hot Cue point, it is set to the nearest beat grid. (page 154)

**Note**

- You cannot save a Hot Cue at a performance pad which has already had one saved.

## To play a Hot Cue

1. Select [HOT CUE] on the performance pad.
   The [HOT CUE] panel is displayed.

2. Click the performance pad with a saved Hot Cue.

**Hint**

- During the Hot Cue playback, you can cancel the loop play and adjust the length of the loop.

## To delete a Hot Cue

1. Select [HOT CUE] on the performance pad.
   The [HOT CUE] panel is displayed.

2. Click [X] on the performance pad with the saved Hot Cue to be deleted.

## To convert memory cues to Hot Cues

Right-click a track and select [Convert Memory Cues to Hot Cues] to orderly convert saved cue points (memory cues) to unset Hot Cues.

- When Hot Cues are fully set, conversion is not available.
- When there are more saved cue points (memory cues) than unset Hot Cues, the same number of memory cues as unset Hot Cues will be converted.

**Hint**

- A memory loop is converted to a hot loop.
- You can also use [Convert Memory Cues to Hot Cues] in the EXPORT mode.

---

# Checking the play histories

See "Checking the play histories" (page 102) in "EXPORT mode".

---

# Using the slip mode

Even when you change the playback position, such as by scratching, playback continues in the background.
When you finish your performance in the slip mode, the track resumes to play in the foreground. You can perform without losing track rhythm and breaking the flow of tracks.

- The slip mode can be set for each deck.
- While playback continues in the background, the red ring of the platter rotates. The playback position of the background playback is displayed on the waveform as a line.

The slip mode is corresponded with following features.

- Hot Cue: By holding a pad or holding a performance pad on the DJ controller, playback starts from the registered Hot Cue position. (slip Hot Cue)
- Scratch (only if the JOG operation is available on the connected DJ equipment): While you touch the top of the JOG on the DJ controller. (slip scratch)
- Pause: When you click ∎∎ after starting playback.
- Auto loop / manual loop: During a loop play. (slip auto loop / slip manual loop)
- Reverse playback: After turning on REVERSE on the DJ controller, playback is in the opposite direction for 8 beats and the background playback continues. (slip reverse)

1. Click [SLIP] in the JOG panel.
   [SLIP] lights in red, and the slip mode is turned on.

2. Operate a feature.
   When operating a feature, playback continues in the background.

3. Cancel operation of the feature.
   Playback continues from the point which is being played in the background.

4. Click [SLIP] again, and then the slip mode is turned off.

---

# Using the beat sync function

The BPM and beat positions of the track playing can automatically be in sync with a track playing on the sync master deck.
Before using the beat sync function, you need to analyze music files to obtain beat grid information. (page 151)

1. Click [MASTER] in the track information panel of the deck which you wish to use as sync master.

2. Start playback of a track on the deck to be synchronized.

3. Click [BEAT SYNC] in the track information panel of the deck to be synchronized.
   The BPM and beat positions of the track playing is in sync with a track playing on the sync master.

4. To cancel the beat sync function, click [BEAT SYNC] again.

**Hint**

- For the track information panel, see "Track information panel" (page 124).
- If you click [MASTER] on the other deck, the sync master is switched to the deck.
- To sync with sample audio source on the sampler deck, see "Using the sampler deck" (page 188) and "SYNC sampler deck with other decks" (page 192).
- When changing or unloading a track on the deck of the sync master the sync master is switched to the other deck.

---

# Synchronizing at doubled or halved BPM

When [BEAT SYNC] is on, the BPM can be changed to equal, double, or half of the sync master BPM.

1. Turn on the Beat Sync function.

2. Click [x 1], [x 2] or [x 1/2] in the track information panel of the deck which you wish to change.

**Hint**

- The BPM switches in the order of [x 1] → [x 2] → [x 1/2]. If the tempo exceeds +100% when doubled, [x 2] will be skipped.
- The display for this function in the track information panel can be turned on/off in the [Preferences] window (page 223).
- The display for this function in the track information panel is hidden on the deck of the sync master.

---

# Using the Instant Doubles function

When you call up the Instant Doubles function on a DJ controller, a track on a deck is loaded to another deck on which the LOAD button is pressed. At this time, the playback status (the playback position, loop play, etc.) and the deck setting (pads, slip mode, etc.) are copied, and playback starts.
How to call up the Instant Doubles function depends on the DJ controller (such as pressing the LOAD button twice). Refer to the Instruction Manual of each DJ controller.

**Hint**

- You can also load a track by dragging and dropping a title part of a track loaded on a deck to another deck.

---

# Using the beat jump function

You can instantly jump backward or forward from the current playback position by a predetermined number of beats.

**Hint**

- For the screen, see "[BEAT JUMP] mode" (page 130).

1. Select [BEAT JUMP] on the performance pad.
   The [BEAT JUMP] panel is displayed.

2. Switch the display with a scroll bar.

3. Click a performance pad.
   The playback position jumps in the direction designated by the selected number of beats.
   Select from [FINE] (= 5 ms), [1/8] beat, [1/4] beat, [1/2] beat, [1] beat, [2] beats, [4] beats, [8] beats, [16] beats, [32] beats, [64] beats, and [128] beats.

---

# Using the key shift function

rekordbox has a key shift function; key sync to automatically sync the key among tracks on other decks, and semitone up/down to manually shift the key of a track.
Also, you can use two Pad modes with the key shift function; [KEY SHIFT] mode and [KEYBOARD] mode.
The key sync function enables you to shift the key of a track, and sync the key among tracks loaded to a deck. This allows you to mix tracks with different keys without any discordance.
The [KEYBOARD] mode on the Pad mode enables you to shift the key of a track, and start a playback from the selected Hot Cue point. This allows you to improvise with any sound from tracks, like using a musical instrument keyboard. In the [KEY SHIFT] mode you can specify a value for the key change.

## To use [KEY SYNC]

You can shift the key of a track, and sync the key among tracks loaded to decks. This allows you to mix tracks with different keys without any discordance.

**Hint**

- Before using [KEY SYNC], it needs to have already been analyzed. (page 14).

1. Click [KEY SYNC] in the track information panel of the deck of which the key is used for synchronizing.
   The key is automatically shifted to sync tracks on other decks.

2. Click [KEY SYNC] again to cancel the key sync.
   After cancelling the key sync, the synced key remains.
   To reset the key, click [KEY RESET].

## To use [KEY SHIFT]

Using the [KEY SHIFT] mode on the Pad mode, you can specify a value for the key change. In addition, you can operate the key sync, semitone up/down, and key reset.

1. Select [KEY SHIFT] on the performance pad.
   Values for the key change ([-12] to [+12]), [KEY SYNC], [SEMITONE UP]/[SEMITONE DOWN], and [KEY RESET] are displayed in the list or on the pad.

2. Click the pad.
   The key on a deck you selected is shifted.

## To use [KEYBOARD]

Using the [KEYBOARD] mode on the Pad mode, you can shift the key of a track, and start a playback from the selected Hot Cue point. This allows you to improvise with any sound from tracks, like using a musical instrument keyboard.

1. Select [KEYBOARD] on the performance pad.

2. Click the settings icon on the performance pad.
   The setting status of Hot Cues is displayed in the list or on the pad.

3. Select the Hot Cue of which you want to shift the key.
   If no Hot Cue is set, click the pad or list to set it.

4. Click [-12] to [+12] on the pad.
   The Hot Cue with the key shifted by the selected value starts playing back.

5. Click [KEY RESET] to reset the key.

**Hint**

- If you click the settings icon, Hot Cues are displayed in the list, and you can select a Hot Cue of which you want to shift the key.

---

# Using Automix playlists

Select a playlist to be used as an Automix playlist.

1. Click the Automix icon.
   The [Automix] window is displayed.

2. Drag & drop a playlist from the tree view into the [Automix] window.
   Tracks in the playlist are added into the [Automix] window.

   [Screenshot: Automix window showing a playlist of 5 tracks with columns for Artwork, Track Title, Artist, and Album, with numbered controls at the top right: 1 - clear/save, 2 - save as new playlist, 3 - settings]

   1. Clear the display of tracks on the current Automix playlist from the [Automix] window. You can also storage as a new playlist.
   2. Storage the Automix playlist displayed on the [Automix] window as a new playlist.
   3. Set [Repeat playback], [Random playback], or the method to load to a deck ([Automix settings]).

3. Click the Automix start icon.
   The track of the Automix playlist is loaded onto the deck 1 or deck 2 automatically, and Automix starts.

4. To finish Automix, click the Automix start icon again.
   Automix stops, and the current playing track continues until the end of the track.
   When unloading the loaded track, Automix also stops.

---

# Using the mixer function

Click the mixer icon in the global section to display the mixer panel.

**Hint**

- For the mixer panel, see "Mixer panel" (page 141).
- When you connect your computer to a DJ controller, the mixer panel is automatically hidden.

## To adjust the audio tone level

Turn [HIGH], [MID], or [LOW] control in the mixer panel during playback.

## To switch the function of [EQ/ISO (HI, MID, LOW)] controls

Open the [Preferences] window > [Controller] category > [Mixer] tab > [EQ], and set [EQ/ISOLATOR].

- [EQ]: Use as the equalizer mode.
- [ISOLATOR]: Use as the isolator mode.

When [EQ] is selected, set [EQ Type].

- [DJM-900NXS]: Set to the same EQ characteristics of DJM-900NXS.
- [DJM-900NXS2]: Set to the same EQ characteristics of DJM-900NXS2.

**Hint**

- When you click words of [HIGH]/[MID]/[LOW] to light up, the band is turned off. While they light up, each controller is not activated.

---

# Monitoring audio

Use headphones to monitor audio.

**Hint**

- For the mixer panel and headphones panel, see "Mixer panel" (page 141).

1. Click the mixer icon in the global section.

2. Turn [MIX] in the headphones panel to the middle position.
   The audio volume balance of [MASTER] (master volume) and [CUE] (headphones volume) are the same on playback.

3. Turn [LEVEL] in the headphones panel all the way to the left.
   The audio is not output.

4. Click [CUE] in the mixer panel which corresponds with the deck for monitoring.
   [CUE] lights up and monitoring is available.

5. Adjust the volume with [LEVEL] in the headphones panel.

---

# Using the microphone feature

When compatible DJ equipment is connected, open the microphone panel to use the microphone feature.
For compatible DJ equipment, refer to FAQ on the rekordbox website (rekordbox.com).

## Microphone panel

Click the mixer icon in the global section to display the microphone panel. Depending on the DJ equipment connected, the display may differ.

[Screenshot: Microphone panel showing controls numbered 1-9: MIC ON toggle, volume level meter, LOW/MID/HI equalizer knobs, ECHO effect button, effect selector dropdown, effect level knob, TALK OVER button, LIGHT feedback reducer button, and feedback reducer type selector]

1. Turn on/off the microphone.
2. Display the microphone volume level.
3. Adjust the microphone equalizer.
4. Turn on/off the microphone effect.
5. Select a microphone effect.
6. Adjust the microphone effect level.
7. Turn on/off TALKOVER. (page 170)
8. Turn on/off FEEDBACK REDUCER. (page 171)
9. Select the FEEDBACK REDUCER type. (page 171)

## To turn on MIC TALKOVER

When you change the microphone sound volume above the set value, the sound from other channels attenuates automatically.

1. Open the [Preferences] window > [Controller] category > [Mixer] tab > [Microphone Input], and set [Talkover Mode] to [Advanced] or [Normal].
   - [Advanced]: Only the mid-range sound of channels, other than the MIC channel, is attenuated according to the set value of the talkover level.

     [Screenshot: Frequency response graph showing Advanced talkover mode - mid-range frequencies are attenuated while low and high frequencies remain, with microphone and music note icons]

   - [Normal]: The sound of channels, other than the MIC channel, is attenuated according to the set value of the talkover level.

     [Screenshot: Frequency response graph showing Normal talkover mode - all frequencies are attenuated equally, with microphone and music note icons]

2. Click [TALKOVER] in the microphone panel.
   [TALKOVER] lights up and MIC TALKOVER is turned on.

## To reduce microphone Feedback (FEEDBACK REDUCER)

When microphone feedback occurs, the frequency is detected, and a filter is inserted to cut only the detected frequency band. This reduces the howling sound of the feedback with minimum effect on sound quality.

1. Click FEEDBACK REDUCER to turn it on.
   When FEEDBACK REDUCER is turned on, it lights up.

2. Click the type selector to select a type.
   - [LIGHT -FEEDBACK REDUCER]: The cutting frequency band is narrow. Select this type when you want to prioritize sound quality, such as for singing or rap performances.
   - [HEAVY -FEEDBACK REDUCER]: The cutting frequency band is wide. Select this type when you want to prioritize feedback reduction, such as for speeches or MCing.

---

# Changing the screen layout

At the global section or from [View] menu on the upper screen, you can customize the screen according to your DJ performance style.

- You can change the number of decks, the horizontal / vertical display of enlarged waveform, and size of browser.
- You can display the effect, sampler deck, mixer, recording, video, lyric, or LIGHTING panel.

### Using 2 decks on DJ performance

- Global section: Select [2Deck Horizontal] or [2Deck Vertical].
- [View] menu: Select [2Deck Horizontal] or [2Deck Vertical].

### Using 4 decks on DJ performance

- Global section: Select [4Deck Horizontal] or [4Deck Vertical].
- [View] menu: Select [4Deck Horizontal] or [4Deck Vertical].

**Hint**

- When selecting [2Deck Horizontal] or [4Deck Horizontal], the enlarged waveform is displayed in the full width of the layout screen. This makes it easy for mixing.
- When selecting [2Deck Vertical] or [4Deck Vertical], the enlarged waveform is displayed in the full height of the layout screen. This makes it easy for scratching.
- Select [Browse] to enlarge the browser panel to make the selection easier.
- For the global section, see "Global section" (page 123).
