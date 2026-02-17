---
id: dvs-setup
title: "DVS Setup Guide"
type: guide
source:
  file: rekordbox7.0.5_dvs_setup_guide_EN.pdf
  pages: "1-19"
  version: "7.0.5"
topics: [compatibility, connection, dvs, equipment, mixing, performance]
modes: [performance]
confidence: pending
last_verified: null
transcribed_by: agent
verified_by: null
---

# DVS Setup Guide

# Table of Contents

1. [Before Start](#1-before-start) ..... 1
   1. [Prior to use](#11-prior-to-use) ..... 1
   2. [Overview of DVS](#12-overview-of-dvs) ..... 1
   3. [Required equipment](#13-required-equipment) ..... 2
   4. [System requirements](#14-system-requirements) ..... 2
2. [Preparing application](#2-preparing-application) ..... 3
3. [Audio Setup](#3-audio-setup) ..... 6
   1. [Connecting to a DJ mixer or controller](#31-connecting-to-a-dj-mixer-or-controller) ..... 6
   2. [Connecting to a stand-alone audio interface](#32-connecting-to-a-stand-alone-audio-interface) ..... 7
4. [DVS setup](#4-dvs-setup) ..... 8
   1. [Enable DVS function](#41-enable-dvs-function) ..... 9
   2. [Routing Setting](#42-routing-setting) ..... 12
   3. [Calibrating the control signal](#43-calibrating-the-control-signal) ..... 14
   4. [Waveform display](#44-waveform-display) ..... 17
   5. [Control Vinyl](#45-control-vinyl) ..... 17

# 1 Before Start

## 1.1 Prior to use

This Setup guide explains DVS. For instructions on rekordbox in general, see the Operating Instructions of rekordbox.

rekordbox Operating Instructions: rekordbox.com/en/download/#manual

## 1.2 Overview of DVS

You can control rekordbox virtual decks using dedicated control vinyl, turntables and DJ units supporting DVS\*.
\*DVS...Digital Vinyl System

[Screenshot: DVS system diagram showing a laptop computer connected via USB cable to a DJ mixer in the center, with two turntables connected to the mixer via audio cables on left and right sides]

You can control rekordbox virtual decks using turntables. Playback direction, tempo, scratching and playback position are reflected to the track loaded on the deck.

## 1.3 Required equipment

The following equipment is necessary for DVS.

- **Computer**
  See *1.4 System Requirements* for system requirements.

- **Turntables (or CDJ/XDJ)**
  Recommended turntable:
  - PLX-CRSS12
  - PLX-1000
  - PLX-500

- **Control Vinyl (Control signal WAV file for CDJ/XDJ)**
  AlphaTheta Control Vinyl dedicated for rekordbox:
  - RB-VS1-K

**Please note:** We do not support any control vinyl, CD nor control signal WAV data of other manufacturers. If you use them, they won't work properly, for example, the marker on the vinyl and the cue point and the playhead positions may not match.

For CDJ/XDJ users, download Control Signal WAV file dedicated for rekordbox dvs.
Please go to rekordbox.com FAQ page: rekordbox.com/en/support/faq/dvs-7/
Click [How can I use the DVS feature on multi-players (CDJ, XDJ)?] > rekordbox_Control_Signal.zip, download rekordbox Control Signal WAV file, and unzip it and burn it on CD-R, save it in a USB storage device, etc.

- **DJ units supporting DVS**
  For the latest information, please see: rekordbox.com/en/support/link

## 1.4 System requirements

The latest information (supported OS, required operating environment, etc.) is available on the rekordbox website.
rekordbox.com/en/download/#system

# 2 Preparing application

Select either of the following procedures to use DVS.

\*If you are starting rekordbox for the first time, you will need to setup the AlphaTheta account.
Please refer to the rekordbox Introduction for details of AlphaTheta account.

**1 Using the DVS function with the Professional, Creative, or Core plan**

When you subscribe to one of the subscription plans, you can use DVS feature by enabling DVS setting.

The plan you are currently subscribed to will be displayed in the top right of the screen.
Please refer to the Plan page (rekordbox.com/ja/plan/) or
FAQ (rekordbox.com/en/support/faq/plans-7/) for details of subscription plans.

**Enable DVS Function**

- Open [Preferences] > [DVS] category and set [Enable DVS Function] to on.

[Screenshot: Preferences window showing DVS category selected in left sidebar, with Enable DVS Function toggle at top turned on, Control Signal Setting section below showing DECK1 and DECK2 each with calibration display, AUTO/MANUAL radio buttons, L/R P/A Balance Start button, Noise Level Start button, and Size slider with Auto button]

**2 Using the DVS function with the Free plan**

Even if you are not subscribed to a subscription plan, you can use the DVS feature by flowing steps A or B.

**A. Use DJ units supporting DVS that supports hardware unlock**

**STEP 1. Connect to Use DJ units supporting DVS that supports hardware unlock**

- Connect to the DJ units supporting DVS as listed on the Hardware Unlock page of rekordbox.com. (rekordbox.com/en/support/hardware-unlock/)

**STEP 2. Enable DVS Function**

- Open [Preferences] > [DVS] category and set [Enable DVS Function] to on.

[Screenshot: Preferences window showing DVS category selected in left sidebar, with Enable DVS Function toggle at top turned on, Control Signal Setting section below showing DECK1 and DECK2 each with calibration display, AUTO/MANUAL radio buttons, L/R P/A Balance Start button, Noise Level Start button, and Size slider with Auto button]

**B. 30-day free TRIAL**

Install rekordbox and start TRIAL. You can use all features of rekordbox including DVS for 30 days. (TRIAL will be expired once the 30 days have passed.)

**STEP 1. Start Trial**

- Launch rekordbox and tick the [OK] button on 30-day free trial screen.

[Screenshot: 30-day free trial dialog showing "Feel free to experience the functions available only on the paid plan while you can." with a list of core functions available during the trial under PERFORMANCE category: Control with DJ equipment, Recording, Starting mix, MERGE FX, STEMS, GROOVE CIRCUIT (NEW), MIX POINT LINK, DVS, Video, DJM EFFECTS, RMX EFFECTS. Bottom shows OK button highlighted in red]

**STEP 2. Enable DVS Function**

- Open [Preferences] > [DVS] category and set [Enable DVS Function] to on.

[Screenshot: Preferences window showing DVS category selected in left sidebar, with Enable DVS Function toggle at top turned on, Control Signal Setting section below showing DECK1 and DECK2 each with calibration display, AUTO/MANUAL radio buttons, L/R P/A Balance Start button, Noise Level Start button, and Size slider with Auto button]

# 3 Audio Setup

## 3.1 Connecting to a DJ mixer or controller

[Screenshot: DVS connection diagram showing a laptop connected via USB cable to a DJ mixer/controller in the center, with two turntables connected to the mixer via audio cables on left and right sides]

- Connect your computer to a DJ mixer (controller) using a USB cable.
- Connect turntables (or CDJs/XDJs) to the DJ mixer (controller) using RCA audio cables.
  For turntable users:
  The turntable must be connected to a channel with [PHONO] or [PHONO/LINE]. Be sure to connect the ground wire of the turntable to the [SIGNAL GND] terminal of the DJ mixer (controller).
  For CDJ/XDJ USERS:
  The CDJ/XDJ must be connected to a channel with [CD/LINE], [LINE] or [PHONO/LINE].
- Input selector of the mixer (controller) channel connected to the turntables (or CDJs/XDJs) must be set to [USB-A], [USB-B], [USB], [PC], [computer icon], [DECK 1-4], etc. to the position of the computer to which rekordbox is connected.
- Use AlphaTheta Control Vinyl (RB-VS1-K) for DVS.

For CDJ/XDJ users, download rekordbox Control Signal.wav\* file and burn it on CD-R or save it in USB memory, etc. and insert the media device to the CDJ/XDJ (\*see details to *1.3 Required equipment*).

## 3.2 Connecting to a stand-alone audio interface

[Screenshot: DVS connection diagram showing a laptop connected via USB cable to a stand-alone audio interface in the center, with two turntables connected via audio cables on left and right sides, and the audio interface connected to a DJ mixer via audio cable below]

- Connect your computer to a stand-alone audio interface using a USB cable.
- Connect turntables (or CDJs/XDJs) to the stand-alone audio interface using RCA audio cables.
  For turntable users:
  Select [PHONO] for the input selector switch of the audio interface.
  Connect the turntables to the input terminals of the audio interface.
  Be sure to connect the ground wire of the turntable to the [SIGNAL GND] terminal of the audio interface.
  For CDJ/XDJ users:
  Select [LINE] for the input selector switch of the audio interface.
  Connect the CDJs/XDJs to input terminals of the audio interface.
- Connect output terminals of the audio interface to CD/LINE terminals of a DJ mixer using RCA audio cables.
- Use AlphaTheta Control Vinyl (RB-VS1-K) for DVS.

For CDJ/XDJ users, download rekordbox Control Signal.wav file\* and burn it on CD-R or save it in USB memory, etc. and insert the media device to the CDJ/XDJ (\*see details to *1.3 Required equipment*).

# 4 DVS setup

Open [Preferences] > [DVS] category in rekordbox PERFORMANCE mode.
[ 1 ] Enable DVS Function
[ 2 ] Routing setting
[ 3 ] Control signal setting
[ 4 ] Waveform display
[ 5 ] Control Vinyl

[Screenshot: Full DVS preferences panel showing five numbered sections: (1) Enable DVS Function toggle at top, (3) Control Signal Setting with DECK1-DECK4 calibration panels each showing signal rings display, AUTO/MANUAL calibration options, L/R P/A Balance and Noise Level Start buttons, and Size slider, (4) Waveform display section with "When the tempo is changed" toggle, (5) Control Vinyl section with Needle operation toggle, and (2) Routing Setting section at bottom showing Control Hardware radio buttons (Turntables, CDJs, Customized Setting) and Audio Routing diagram with DECK1 and DECK2 vinyl icons connected to MIXER CHANNEL numbers 1-4]

## 4.1 Enable DVS function

[Screenshot: Enable DVS Function toggle button shown in on position]

When [Enable DVS function] is set to ON, each DECK will switch to DVS mode.

[Screenshot: rekordbox deck in DVS mode showing "Not Loaded" state with KEY SYNC and BPM SYNC/MASTER buttons, MUTE/DRUMS/VOCAL/INST buttons, hot cue pads, REL dropdown selector, SLIP and Q buttons, BPM display showing 00.00, AU/MA buttons, and INT play button at bottom, MT button]

DVS modes: ABSOLUTE, RELATIVE and INTERNAL
(Stand-alone audio interface: ABSOLUTE, RELATIVE, INTERNAL and THROUGH)

Select one of the DVS modes by clicking the DECK number.

Click [REL] and select the mode from the drop-down menu.

[ABS]: ABSOLUTE mode
- Location of the needle on the vinyl is linked with the playback position of rekordbox DECK.
- Direction, tempo, scratch and other performances on the vinyl are linked to the DECK.
- Digital features such as Hot Cue and Needle Search do not work in this mode.

Note: See *Restricted features in the ABSOLUTE and RELATIVE mode* in page 11 for details.

[REL]: RELATIVE mode
- Location of the needle on the vinyl is not linked with the playback position of rekordbox deck.
- Direction, tempo, scratch and other performances on the vinyl are linked to the deck.

Note: See *Restricted features in the ABSOLUTE and RELATIVE mode* in page 17 for details.

[INT]: INTERNAL mode
- Location of the needle, direction, tempo, scratch and other performances on the vinyl are not linked to the deck.
- You can use all features on the deck.

[THR]: THROUGH mode (This appears only when using a stand-alone audio interface)
- Audio from a turntable is sent through an audio interface to an output terminal as is.
- The deck will pause.

[INT] is displayed except for the INTERNAL mode.
Click this icon to switch to the INTERNAL mode.
Use this feature in the ABSOLUTE mode when you temporally switch to the INTERNAL mode, e.g. when the needle is approaching to the end of the control vinyl.

**Restricted features in the ABSOLUTE and RELATIVE mode**

| Features on the deck | ABSOLUTE mode | RELATIVE mode |
|---|---|---|
| PLAY/PAUSE | N/A | N/A |
| CUE | N/A | N/A |
| AUTO BEAT LOOP | N/A | checkmark |
| LOOP IN / OUT | N/A | checkmark |
| QUANTIZE | N/A | checkmark |
| SLIP | N/A | checkmark |
| TEMPO SLIDER | N/A \*1 | N/A \*1 |
| TEMPO RANGE | N/A \*1 | N/A \*1 |
| BPM SYNC | N/A | checkmark \*2 |
| NEEDLE SEARCH | N/A | checkmark |
| PAD / GRID Panel | N/A | checkmark |

\*1...Tempo Control is available by using Tempo Slider or Tempo Range on the turntable (or CDJ/XDJ).

\*2...BPM Sync on the deck in the Relative mode works as follows:
- Click [BPM SYNC] you wish to synchronize.
  -> The deck is synchronized to the master deck (not grid).
- When you change the master deck tempo using the tempo slider,
  -> The BPM of the deck with [BPM SYNC] turned on is the BPM of the master deck. If you change the tempo of a deck (other than the master deck), only the BPM of the deck is changed.
- To cancel Sync, click [BPM SYNC] again (toggle feature).
  -> The BPM remains the same even though you clicked [BPM SYNC].
  To return the BPM to the speed you changed using the tempo slider on the turntable (CDJ/XDJ), click [RESET]\* (\*shown as [R] in the 2 deck/4 deck horizontal mode).
  To display [RESET], hover your mouse over a virtual deck.

[Screenshot: Virtual deck display showing REL mode dropdown, SLIP and Q buttons, BPM reading of 125.0 with 0.8% tempo change, and RESET button highlighted in red below the BPM display, with INT play button and MT button]

## 4.2 Routing Setting

This audio routing setting procedure is available on DJ units supporting DVS(\*1).
(\*1)...DJ units supporting DVS (rekordbox.com/en/support/link)

To use the DVS function, open [Preferences] > [Audio] category > [Input/Output] tab and complete the following settings.
1. [Input Channel]
2. Output setting at [MIXER OUTPUT] in the [Setting Utility] of the hardware. (Depending on the DJ unit used, this may not be necessary.)
3. [Output Channel]

The audio routing setting makes these settings simple and easy.
Once you completed the procedure, all the settings (1, 2 and 3) will also be completed.
Please note that this audio routing setting is only for the configuration using 2 turntables (or 2 CDJs/XDJs) for the DVS. For other configuration, complete each setting at [Preferences] > [Audio] category > [Input/Output] tab one by one.

- **Control hardware**

[Screenshot: Control Hardware radio button group showing three options: Turntables (selected), CDJs, and Customized Setting]

Click one of three options above to select the DJ player.
- To use 2 turntables, click [Turntables].
  -> The input to your computer is set to [Control tone PHONO].
- To use 2 CDJs/XDJs, click [CDJs].
  -> The input to your computer is set to [Control tone CD/LINE].
- For configuration except for the above 2 cases, click [Custom].
  When you select [Custom], complete each setting at [Preferences] > [Audio] category > [Input/Output] tab one by one.

- **Audio Routing**

[Screenshot: Audio Routing diagram showing DECK1 and DECK2 represented as vinyl record icons at top, connected by curved lines to MIXER CHANNEL numbers 1, 2, 3, and 4 at the bottom, with draggable connection points]

This illustrates an audio routing setting of input and output channels in rekordbox. You can change the setting by yourself.
Click the dot and reconnect between deck and the channel number of connected DJ equipment supporting DVS to make it the same as the real configuration.
-> The connection will be reflected to [Input Channel] and [Output Channel] in [Preferences] > [Audio] category > [Input/Output] tab.

## 4.3 Calibrating the control signal

This section describes how to calibrate the control signal from the turntable to rekordbox.

[Screenshot: DECK1 control signal calibration panel showing signal rings display with value 0.280545, CALIBRATION section with AUTO selected and MANUAL option, L/R P/A Balance Start button, Noise Level Start button, and Size slider with Auto button]

The control signal is described as dual rings. (The number shown in the upper left describes intensity of the signal.)

**Important**: Be sure to calibrate each deck. If not, scratching and other plays on the turntables may not work correctly.

When using the PLX-CRSS12, play the rekordbox control vinyl in ANALOG mode or select [APPLICATION] > rekordbox in DIGITAL VINYL mode and press the [START/STOP] button.

[Screenshot: PLX-CRSS12 display showing APPLICATION button selected with rekordbox label, DECK info showing KEY, BPM 129.7, and a mode selection showing NEEDLE MODE with DIGITAL VINYL and ANALOG options, with ANALOG mode indicated]

DIGITAL VINYL mode:
Using DVS without analog records or tone arms
ANALOG mode:
Play analog records (control vinyl)

Select [AUTO] or [MANUAL] for calibration.
First, select the [AUTO] mode to calibrate the signal automatically. Then start the turntable and scratch it. If playback on the deck is not stable when scratching, switch the mode to [MANUAL] and calibrate the signal by hand.

- **[AUTO]**

[Screenshot: DECK1 control signal calibration panel in AUTO mode showing signal rings display with value 0.280545, CALIBRATION with AUTO selected, L/R P/A Balance Start button, Noise Level Start button, and Size slider with Auto button]

STEP 1. Adjusting the control signal.
Start the turntable and click [Start] for L/R, P/A Balance.
When the calibration is completed, [Complete] will be displayed in the upper part of the [Start] button.
If the rings of the control signal are close to circular, the signal is adjusted.

STEP 2. Adjusting the threshold for the reading of the control signal based on measurement of noise due to vibration from the turntable.
Stop the turntable. Switch the deck mode to [INT] and play the deck. With sound coming from the speaker, click [Start] at Noise Level.
When the calibration is completed, [Complete] will be displayed in the upper part of the [Start] button.

- **[MANUAL]**

[Screenshot: DECK1 control signal calibration panel in MANUAL mode showing signal rings display with value 0.280527, CALIBRATION with MANUAL selected, L/R Balance slider, P/A Balance slider, and Noise Level slider, with Size slider and Auto button at bottom]

STEP 1. Adjusting the control signal (L/R Balance, P/A Balance).
Start the turntable. Move sliders for L/R, P/A Balance to make the rings of the control signal as close to circular as possible.

STEP 2. Adjusting the threshold for the reading of the control signal.
Adjust the threshold by moving Noise Level slider. As you move the slider to the right, the application ignores more noise occurred due to the vibration from the turntable. As you move the slider to the left, you can hear scratching more precisely as more signals can be received.

You can change size of the control signal display by Size slider.
Note: You can enlarge the size of the rings.

## 4.4 Waveform display

[Screenshot: Waveform display settings panel showing "When the tempo is changed" label with a toggle switch and description "The beat position intervals of the enlarged waveform are changed."]

When set to on, the beat position intervals of the enlarged waveform are changed when you change the tempo of the track. (The scrolling speed does not change.)

When set to off, the scrolling speed of the enlarged waveform is changed when you change the tempo of the track. (The beat position intervals do not change.)
This is available only when [Enable DVS function] is on.
(When [Enable DVS function] is off, the beat position intervals of the enlarged waveform are changed.)

## 4.5 Control Vinyl

[Screenshot: Control Vinyl settings panel showing "Needle operation" label with a toggle switch and description "When the needle is placed on the lead-in part of the vinyl, the playback position jumps to the beginning of the track."]

This is available only in RELATIVE mode.
When set to on, the playback position jumps to the beginning of the track when the needle is placed on the lead-in part of the control vinyl.
(When using the control signal WAV file, place the playback position of the player to the beginning of the WAV file to use this feature.)

The DVS setup is now complete.

---

- rekordbox is a registered or registered trademark of AlphaTheta Corporation.
- Other product, technology and company names, etc. mentioned herein are trademarks or registered trademarks of their respective owners.

(c) AlphaTheta Corporation.
