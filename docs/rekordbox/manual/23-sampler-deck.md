---
id: sampler-deck
title: "Using the sampler deck"
type: manual
source:
  file: "rekordbox7.2.8_manual_EN.pdf"
  pages: "188-192"
  version: "7.2.8"
topics: [beatgrid, edit, key, mixing, playback, sampler]
modes: [performance]
confidence: pending
last_verified: null
transcribed_by: agent
verified_by: null
---

# Using the sampler deck

rekordbox has a sampler deck which can play 16 (20 including OSC samplers) audio tracks at the same time. In this manual, each player is called "sampler slot" (or "slot").

You can use the sample audio in various ways, from the basic Oneshot mode to the SYNC mode, with a track on the deck.

**Hint**

- For the screen, see "Sampler deck" (page 139).
- You can extract a section of a track on the deck and load it to a sampler slot with the CAPTURE function. For the operation of CAPTURE, see "Extracting a section from a track (CAPTURE)" (page 200).
- You can also use sampler audio files. For details on downloading, see "Download and install sample pack" of "rekordbox Introduction" on the rekordbox website.

## To load sample audio to a sampler slot

1. Click [grid icon] in the global section.
   A sampler deck is displayed.

2. Click [BANK] to switch a bank.
   The sampler deck has four banks, each of which has 16 slots.

3. Drag and drop a track from a track list to a sampler slot.
   The play mode is set to [Oneshot].
   The track is stored after quitting/exiting rekordbox.

### Deleting a track from a sampler slot (Unload)

Position the mouse cursor to the track's title and click [eject icon]. The track loaded to the sampler slot is deleted.

## To play a sampler slot

Click [play icon] ([Play mode (Oneshot)]) or [loop icon] ([Play mode (Loop)]) to light it up and start playing.

- While holding the [Shift] key on the computer keyboard, click [play icon] or [loop icon] to stop playing.
- If you position the mouse cursor to the track's title and click [speaker icon], the audio is muted.
- When [Gate mode] (page 191) is on, the display changes to [gate play icon] or [gate loop icon].

## To play a sampler slot while switching banks

If you click [BANK] to switch banks while playing a sampler slot, the playback does not stop. However, when you play a sampler slot in the same slot position of the previous bank, that sampler slot stops and the sampler slot in the currently displayed bank starts.

The following displays an example of this:

1. Click [play icon] or [loop icon].
   Playback of a sampler slot starts.

2. Click [BANK] to switch banks.
   When a sampler slot is played in another bank, the level meter (gray) is running.
   It displays the status of a sampler slot playing in the background.

3. Click [play icon] or [loop icon].
   When playing the sampler slot in the current bank, the playback of the sampler slot of the previous bank is stopped.

## To use Quantize

Click [Q] (Quantize) on the sampler deck to light it up and turn on Quantize.

For setting the Quantize beat, open the [Preferences] window > [Controller] category > [Others] tab > [Type] > [QUANTIZE], set [LOOP SAMPLER(LOOP)] to [Enable], and then select the value.

**Note**

- The Quantize function is only enabled in sampler slots which are set to [Play mode (Loop)].

## To adjust the volume of the sampler deck

1. Turn [GAIN] in the sampler deck.
   Adjust the volume of the entire sampler deck.
   When mixing, the volume level can be adjusted if the volumes of the sampler deck and other decks differ.

2. Click [mixer icon] in the global section.
   The mixer panel is displayed.
   When both the sampler deck and mixer panel are already displayed, the sampler volume slider and sampler monitor [CUE] are displayed.

   [Screenshot: Sampler deck mixer controls showing GAIN knob, CUE button, BPM display (134.69), and volume slider with +/- buttons]

3. Move the sampler volume slider.
   Adjust the volume of the entire sampler deck.

**Hint**

- Before outputting to MASTER OUT, set the sampler volume slider to the minimum, click the sample monitor [CUE] to turn on, and then check the volume level with headphones.

## To customize the color setting of sampler slots

You can customize colors of playback buttons for tracks loaded to each sampler slot.

1. Open the [Preferences] window > [View] category > [Color] tab > [SAMPLER color] and set [Sync with Frequency].
   The color of playback buttons changes in sync with the frequency color of a track.

**Hint**

- You can select the color by right-clicking [play icon] or [loop icon].

# Editing the sampler slot (Edit mode)

For each sampler slot, you can switch the play mode, adjust volume, etc.
Each setting is saved.

Position the mouse cursor to the track's title in the sampler slot and click the [EDIT] button. The button will light up and enable edit mode.

[Screenshot: Sampler slot edit mode showing waveforms for four tracks ("I Can't Get Enough", "More Bounce", "Two Of A Kind", "Paranoid"), with edit controls including play mode toggle, gate mode, master tempo, sync, gain knob, BPM display (124.01), fine-tune position buttons, BPM halve/double buttons, and BPM value]

1. Switch the play mode.
   - [play icon] [Play mode (Oneshot)]: Play the audio file till the end, and then stop it. During playback, you can click to restart playback from the beginning of the audio file.
   - [loop icon] [Play mode (Loop)]: Start the loop play. During playback, you can click to start the loop playback from the beginning.
   The play mode can be switched by right-clicking the track in the track list and selecting [Track Type].

2. The playback continues only when you click and hold [gate play icon] or [gate loop icon]. When released, the playback pauses. ([Gate mode])

3. The loop play starts without the altering pitch even when the track is played in a tempo different from the original tempo (BPM). (Master Tempo)

4. The loop play starts in sync with the BPM of the sampler deck. (Slot SYNC)

5. If the volume varies widely from the sampler slot, you can adjust it to level the volume. (Slot Gain)

6. Fine-tune playback position. ([Play mode (Oneshot)] only)

7. Halve/double the BPM.

8. Display the BPM value.

# SYNC sampler deck with other decks

You can playback a sampler slot set in the [Play mode (Loop)] in sync with other decks. To use the SYNC play with a sampler deck, you need to set a player deck as the SYNC MASTER in advance. (page 162)

The following displays an example of this:

1. Position the mouse cursor to the track's title in the sampler slot which you want to sync, and click the [EDIT] button to enable edit mode.

2. Click [loop icon] and [SYNC] on the edit screen to light up.

3. Click [BPM SYNC] on the sampler deck to light up.

   [Screenshot: Sampler deck with BPM SYNC enabled showing waveforms, edit controls, SYNC button lit, BPM display (130.21), and BPM SYNC/MASTER buttons at bottom]

4. Click [loop icon] on the left of the title of the sampler slot.
   Playback starts with the BPM of the track on the master deck.
   The BPM value of MASTER for SYNC is displayed on the BPM of the sampler deck.

   [Screenshot: Sampler deck BPM display showing GAIN knob and BPM value 100.40 with +/- buttons]

**Hint**

- To adjust the BPM of the sampler deck, use [+]/[-] in the sampler deck to change the value. Click the BPM display and enter the value to change the BPM. When you click [MASTER] on a sampler deck, the deck is set as the sync master.

## Related Documents

- [manual/18-performance-screen.md](18-performance-screen.md) (beatgrid, mixing, playback, sampler)
- [manual/20-performance-playing.md](20-performance-playing.md) (beatgrid, key, mixing, playback)
- [guides/edit-mode.md](../guides/edit-mode.md) (beatgrid, edit)
- [guides/video-function.md](../guides/video-function.md) (mixing, playback)
- [manual/05-editing-track-info.md](05-editing-track-info.md) (edit, key)
- [manual/14-export-playing.md](14-export-playing.md) (beatgrid, playback)
- [manual/16-export-mixing.md](16-export-mixing.md) (key, mixing)
- [manual/25-slicer.md](25-slicer.md) (beatgrid, sampler)
