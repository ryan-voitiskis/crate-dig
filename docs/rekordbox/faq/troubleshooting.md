---
id: faq-troubleshooting
title: "FAQ: Troubleshooting"
type: faq
source:
  file: "rekordbox7-faq.md"
  url: "https://rekordbox.com/en/support/faq/rekordbox7/"
  version: "7.x"
topics: [analysis, compatibility, link]
modes: [common]
confidence: pending
last_verified: null
transcribed_by: agent
verified_by: null
---

### I haven't received the AlphaTheta account confirmation e-mail.

If you didn't receive the AlphaTheta account confirmation e-mail, there's a possibility that you entered a wrong e-mail address or the confirmation e-mail ended up in the "spam folder", etc.

* When you are using Gmail, right-click on the confirmation e-mail in your "spam folder" and click "not spam".

---

### Let me know the procedure for operating MIX POINT LINK function with a mouse.

Example:

Procedure for mixing from track A of Deck 1 to track B of Deck 2.

1. Set Hot Cues or Memory Cues to the position of MIX POINT preliminarily. To use SYNC function, set BEAT SYNC to ON.
2. Click MIX POINT LINK panel button of global section to display MIX POINT LINK panel.
3. Load track A to Deck 1 to replay it. Load track B to Deck 2.
4. Click the Enlarged Waveform of Deck 1 in MIX OUT selection mode to set the Deck 1 to MIX OUT deck.
5. Click the marker of Hot Cues/Memory Cues that is displayed on Enlarged Waveform of Deck 1 to set it to MIX POINT (MIX OUT).
6. Click the Enlarged Waveform of Deck 2 in MIX IN selection mode to set the Deck 2 to MIX IN deck.
7. Click the marker of Hot Cues/Memory Cues that is displayed on Enlarged Waveform of Deck 2 to set it to MIX POINT (MIX IN).
8. Press MIX POINT LINK button and set MIX POINT LINK.

If the replay bar of Deck 1 reaches the beginning of Deck 2, the automatic playing starts.

*The MIX POINT set in steps 5 and 7 can also be set by pressing the [MIX POINT SELECT] button on the MIX POINT LINK panel.
*When the Silent is turned on, the tracks in the deck are muted from the start of a track to the Silent end point. To turn on the Silent, select deck 2 in the MIX IN selection mode after step 7 or 8, then turn on the [SILENT on/off] button.

---

### Let me know how to operate MIX POINT LINK using MIDI LEARN function as hardware.

Example:

Here explains about the operation method referencing the procedure for assigning the button of MIX POINT LINK to the hardware on MIDI LEARN and mixing track A of Deck 1 to track B of Deck 2.

1. Set Hot Cues or Memory Cues to the position of MIX POINT preliminarily. To use SYNC function, set BEAT SYNC to ON.
In MIX POINT LINK tab of MIDI LEARN, assign [MixPointLink CueSelectBack/Next] button and [MixPointLink Set] button to the hardware on MIDI LEARN.
2. Load track A to Deck 1 to replay it. Load track B to Deck 2.
3. Press [MemoryCueCall] button of Deck 2 or the button / pad for which Hot Cues are set and call the starting MIX POINT from the cue point of track B or Hot Cues.
4. Press [MixPointLink CueSelectBack/Next] button of Deck 2 and select the ending MIX POINT from track A that is currently replayed.
5. Press [MixPointLink Set] button and set MIX POINT LINK.
If the replay bar of Deck 1 reaches the beginning of Deck 2, the automatic playing starts.

---

### Is it possible to zoom in/out the waveform on MIX POINT LINK panel?

Yes, it is possible to zoom in/out the waveform.

Put a mouse cursor on the waveform on MIX POINT LINK panel and operate the mouse wheel.

On Mac, click and hold the waveform on the track pad and swipe it up and down to zoom in/out the waveform.

---

### When connecting some MIDI devices including a DJ controller, MIX POINT LINK function was locked with a key icon and became disabled.

For MIX POINT LINK function, you can try mouse operation only when a MIDI/HID device is not connected.

If you want to use a MIDI/HID device or Keyboard shortcuts, you will need to subscribe to a supported Plan.

---

### When a track is loaded to the deck, the following message is displayed and the track cannot be loaded: "Analysis Failed. There is not enough memory available."

Free up the memory space on the PC/Mac and reload the track again.

Example of how to free up the memory space

Close applications that are not in use.
Close the web browser.
Close Windows Explorer and Mac Finder.

You can check the memory used on your PC/Mac from Task Manager for Windows or Activity Monitor for Mac.

Make sure that your PC/Mac meets the necessary [System requirements](https://rekordbox.com/en/download/#system) as well.

---

### I found a rekordboxAgent process in Task Manager/Activity Monitor. What kind of process is this?

rekordboxAgent is an application required for the operation of rekordbox ver. 6.

Please refrain from force quitting this application in Task Manager/Activity Monitor, as this can cause issues such as the incorrect operation of rekordbox ver. 6.

rekordboxAgent functions when rekordbox ver. 6 is running, and the rekordbox icon is shown on the taskbar/menu bar.

![](https://cdn.rekordbox.com/files/20221128180621/capture-20221128-180149.png)

Even when rekordbox is not running, this application

- sends you notifications on rekordbox news,
- keeps your Library up-to-date through Cloud Library Sync,

so please use this application as it is.

---

### Use third party tools not authorized by AlphaTheta

There have been reports of events such as the corruption of rekordbox libraries due to the use of library management tools not authorized by AlphaTheta Corporation.

We are not responsible for damage to libraries caused by the use of unauthorized tools.

The use of such tools is not advised.

---

### The "Copying music files" dialog appears every time when starting rekordbox. Is there any way to prevent this dialog from appearing?

If there are 50 or more missing files, the "Copying music files" dialog will appear when starting rekordbox.

You can prevent this with the following method.

- If the location of the music file has been changed, relocate the music file.

Select [Display All Missing Files] from the [File] menu.

In the [Missing File Manager] window, click the [Auto Relocate] or [Relocate] button.

For details, refer to this [FAQ](https://rekordbox.com/en/support/faq/v6/#faq-q600162).

- If a music file has already been deleted, the missing file will be removed from the collection.

Select [Display All Missing Files] from the [File] menu.

In the [Missing File Manager] window, click the [Delete] button.

---

### Rendering an audio file in EDIT mode takes a long time.

The rendering time will be longer for tracks that have been analyzed dynamically, or if the beatgrid has been partially adjusted using the GRID panel. This is because the tracks BeatGrids are no longer static.

You have the option to analyze tracks using Normal analysis, ensuring static BeatGrids and faster and the rendering times.

Please be aware that this can compromise the beatgrids accuracy on tracks that actually require Dynamic analyzing.

---

### Can I control Ableton Link's BPM via hardware?

When hardware such as a DJ controller is connected, you can't change the BPM of Ableton Link using the TEMPO slider.

To control the BPM via hardware, you have to control the BPM on the Ableton Link subscreen (*1).

To do this, MIDI mapping is needed for the encoder or the button you want to use.

*1 While Ableton Link is turned on, you can right-click the [LINK] button in the global section and choose to show or hide the Ableton Link subscreen.

---

### How do I change the BPM of Ableton Link?

You can change the BPM in the following ways:

- Open the Ableton Link subscreen (*1) and use the +/- buttons or the TAP button, enter a BPM value, or drag the BPM display up or down with your mouse.
- Use the +/- buttons displayed on the platter or double-click the BPM display and enter a BPM value or drag the BPM display up or down with your mouse.

*1 While Ableton Link is turned on, you can right-click the [LINK] button in the global section and choose to show or hide the Ableton Link subscreen.

---

### How can I sync a track loaded on a deck to Ableton Link's BPM?

Turn on the [LINK] button in the global section to enable Ableton Link.

Then turn on the [LINK] button on the deck you want to sync with the BPM of Ableton Link. (If Ableton Link is enabled, the decks' [BEAT SYNC] buttons will change to [LINK].)

Please note you can't sync to the BPM of Ableton Link in the following cases:

- When the BPM exceeds +/-100% of the tempo range.
- When REL or ABS mode is set for DVS.

---

### What is Ableton Link?

Ableton Link is a technology that aligns the timing of different devices connected to a local network.

Simply connect to the same network and you can jam with other people using applications and hardware that support Ableton Link.

Anyone can join or exit the jam session freely. And you can simultaneously play or stop multiple applications and/or hardware devices connected via Ableton Link.

---

### The volume sometimes drops suddenly when I'm playing iTunes music files on rekordbox.

We have received reports that there are sudden changes to the volume when music files (AAC) purchased from the iTunes Store are played on rekordbox.

This can be improved by converting the music files to an uncompressed file format (AIFF, WAV).

For how to convert the files, please see the iTunes User Guide.
[Convert music file formats in iTunes on PC](https://support.apple.com/guide/itunes/convert-music-file-formats-itns2972/12.12/windows/10)

---

### I forgot which email address I used for my AlphaTheta account, which is also used for logging into rekordbox (Mac/Windows). Where can I check which address I used?

Check the email address of the AlphaTheta account with which you're logged into rekordbox (Mac/Windows) from the [INFO] window or [Preferences].

The email address with which you're logged into rekordbox is displayed on the upper right area of the [INFO] window.

![](https://cdn.rekordbox.com/files/20240426215145/email_info_en.png)

The email address with which you're logged into rekordbox is displayed on the upper right area of the [Preferences] > [Plan] category or [CLOUD] category.

![](https://cdn.rekordbox.com/files/20240426215146/email_prefe_en.png)

---

### When I start rekordbox from my Mac's Dock, ver. 6 starts instead of ver. 7. What should I do?

On a Mac, the install folder for ver. 7 has been renamed from [rekordbox 6] to [rekordbox 7].

Use Finder to select [Applications], click the [rekordbox 7] folder, then double-click **rekordbox.app** to start ver. 7.

Make sure the icon for ver. 7 is the one on the Dock if you want to start it from there.

---

### I have multiple AlphaTheta accounts. Can I switch between them?

You can switch between accounts on [INFO] and [Preferences] windows.

- [INFO]

![](https://cdn.rekordbox.com/files/20241010160616/INFO_ACCOUNT_EN-300x122.png)

- [Preferences]

![](https://cdn.rekordbox.com/files/20241010160621/PREF_ACCOUNT_EN-300x122.png)

---

### I've installed rekordbox ver. 7. Can I use rekordbox ver. 6 on the same computer? /Can I use rekordbox ver. 6 and ver. 7 on the same computer?

You can use rekordbox ver. 6 on the same computer even after installing ver. 7.

However, you can't run ver. 6 and ver. 7 at the same time.

---

### I want to use previous rekordbox ver. 7.

You can download it from the following links.

**ver. 7.2.9**

- [Windows(64bit)](https://cdn.rekordbox.com/files/20260109112838/Install_rekordbox_x64_7_2_9.zip)
- [Mac](https://cdn.rekordbox.com/files/20260109112617/Install_rekordbox_7_2_9.pkg_.zip)

**ver. 7.2.8**

- [Windows(64bit)](https://cdn.rekordbox.com/files/20251203152238/Install_rekordbox_x64_7_2_8.zip)
- [Mac](https://cdn.rekordbox.com/files/20251203151846/Install_rekordbox_7_2_8.pkg_.zip)

**ver. 7.2.7**

- [Windows(64bit)](https://cdn.rekordbox.com/files/20251111151238/Install_rekordbox_x64_7_2_7.zip)
- [Mac](https://cdn.rekordbox.com/files/20251111150953/Install_rekordbox_7_2_7.pkg_.zip)

**ver. 7.2.6**

- [Windows(64bit)](https://cdn.rekordbox.com/files/20251023202513/Install_rekordbox_x64_7_2_6.zip)
- [Mac](https://cdn.rekordbox.com/files/20251023213714/Install_rekordbox_7_2_6.pkg_.zip)

**ver. 7.2.4**

- [Windows(64bit)](https://cdn.rekordbox.com/files/20251007153136/Install_rekordbox_x64_7_2_4.zip)
- [Mac](https://cdn.rekordbox.com/files/20251007153540/Install_rekordbox_7_2_4.pkg_.zip)

**ver. 7.2.3**

- [Windows(64bit)](https://cdn.rekordbox.com/files/20250918055307/Install_rekordbox_x64_7_2_3.zip)
- [Mac](https://cdn.rekordbox.com/files/20250918070915/Install_rekordbox_7_2_3.pkg_.zip)

**ver. 7.2.2**

- [Windows(64bit)](https://cdn.rekordbox.com/files/20250904154448/Install_rekordbox_x64_7_2_2.zip)
- [Mac](https://cdn.rekordbox.com/files/20250904154238/Install_rekordbox_7_2_2.pkg_.zip)

**ver. 7.2.0**

- [Windows(64bit)](https://cdn.rekordbox.com/files/20250821143814/Install_rekordbox_x64_7_2_0.zip)
- [Mac](https://cdn.rekordbox.com/files/20250821195007/Install_rekordbox_7_2_0.pkg_.zip)

**ver. 7.1.5**

- [Windows(64bit)](https://cdn.rekordbox.com/files/20250805161510/Install_rekordbox_x64_7_1_5.zip)
- [Mac](https://cdn.rekordbox.com/files/20250805161036/Install_rekordbox_7_1_5.pkg_.zip)

**ver. 7.1.4**

- [Windows(64bit)](https://cdn.rekordbox.com/files/20250724145935/Install_rekordbox_x64_7_1_4.zip)
- [Mac](https://cdn.rekordbox.com/files/20250724150226/Install_rekordbox_7_1_4.pkg_.zip)

**ver. 7.1.3**

- [Windows(64bit)](https://cdn.rekordbox.com/files/20250610145528/Install_rekordbox_x64_7_1_3.zip)
- [Mac](https://cdn.rekordbox.com/files/20250610145401/Install_rekordbox_7_1_3.pkg_.zip)

**ver. 7.1.2**

- [Windows(64bit)](https://cdn.rekordbox.com/files/20250514152025/Install_rekordbox_x64_7_1_2.zip)
- [Mac](https://cdn.rekordbox.com/files/20250514151842/Install_rekordbox_7_1_2.pkg_.zip)

**ver. 7.1.1**

- [Windows(64bit)](https://cdn.rekordbox.com/files/20250410141214/Install_rekordbox_x64_7_1_1.zip)
- [Mac](https://cdn.rekordbox.com/files/20250410141405/Install_rekordbox_7_1_1.pkg_.zip)

**ver. 7.1.0**

- [Windows(64bit)](https://cdn.rekordbox.com/files/20250319122655/Install_rekordbox_x64_7_1_0.zip)
- [Mac](https://cdn.rekordbox.com/files/20250319122828/Install_rekordbox_7_1_0.pkg_.zip)

**ver. 7.0.9**

- [Windows(64bit)](https://cdn.rekordbox.com/files/20250203165003/Install_rekordbox_x64_7_0_9.zip)
- [Mac](https://cdn.rekordbox.com/files/20250203174138/Install_rekordbox_7_0_9.pkg_.zip)

**ver. 7.0.8**

- [Windows(64bit)](https://cdn.rekordbox.com/files/20241224183105/Install_rekordbox_x64_7_0_8.zip)
- [Mac](https://cdn.rekordbox.com/files/20241224191039/Install_rekordbox_7_0_8.pkg_.zip)

**ver. 7.0.5**

- [Windows(64bit)](https://cdn.rekordbox.com/files/20241024213216/Install_rekordbox_x64_7_0_5.zip)
- [Mac](https://cdn.rekordbox.com/files/20241024221054/Install_rekordbox_7_0_5.pkg_.zip)

**ver. 7.0.4**

- [Windows(64bit)](https://cdn.rekordbox.com/files/20241004022151/Install_rekordbox_x64_7_0_4.zip)
- [Mac](https://cdn.rekordbox.com/files/20241004014350/Install_rekordbox_7_0_4.pkg_.zip)

**ver. 7.0.3**

- [Windows(64bit)](https://cdn.rekordbox.com/files/20240826214835/Install_rekordbox_x64_7_0_3.zip)
- [Mac](https://cdn.rekordbox.com/files/20240820101659/Install_rekordbox_7_0_3.pkg_.zip)

**ver. 7.0.2**

- [Windows(64bit)](https://cdn.rekordbox.com/files/20240624221214/Install_rekordbox_x64_7_0_2.zip)
- [Mac](https://cdn.rekordbox.com/files/20240620173348/Install_rekordbox_7_0_2.pkg_.zip)

**ver. 7.0.1**

- [Windows(64bit)](https://cdn.rekordbox.com/files/20240521191055/Install_rekordbox_x64_7_0_1.zip)
- [Mac](https://cdn.rekordbox.com/files/20240521172903/Install_rekordbox_7_0_1.pkg_.zip)

---

## Related Documents

- [faq/cloud-and-sync.md](cloud-and-sync.md) (analysis)
- [faq/hardware-compatibility.md](hardware-compatibility.md) (compatibility)
- [faq/library-and-collection.md](library-and-collection.md) (analysis)
- [guides/dvs-setup.md](../guides/dvs-setup.md) (compatibility)
- [guides/performance-mode-connection.md](../guides/performance-mode-connection.md) (compatibility)
- [guides/phrase-edit.md](../guides/phrase-edit.md) (analysis)
- [guides/pro-dj-link-setup.md](../guides/pro-dj-link-setup.md) (link)
- [guides/streaming-services.md](../guides/streaming-services.md) (compatibility)
