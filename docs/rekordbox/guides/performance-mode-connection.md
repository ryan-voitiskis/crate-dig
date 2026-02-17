---
id: performance-mode-connection
title: "Connection Guide for Performance Mode"
type: guide
source:
  file: "rekordbox5.3.0_connection_guide_for_performance_mode_EN.pdf"
  pages: "1-17"
  version: "5.3.0"
topics: [compatibility, connection, devices, dvs, equipment, performance]
modes: [performance]
confidence: pending
last_verified: null
transcribed_by: agent
verified_by: null
---

[Screenshot: Pioneer DJ logo and rekordbox logo on title page]

# Connection Guide for PERFORMANCE mode

(CDJ/XDJ/DJM/Audio Interface)

[Screenshot: rekordbox logo]

# 1 Connection Guide

This guide explains procedures to set up the connection for CDJ/XDJs to control rekordbox PERFORMANCE mode and adjust audio output setting.
It also includes how to set up connection with a DJ mixer (DJM)/Audio Interface as an audio device to use it for audio output.

## 1.1 Compatible products

### 1.1.1 Compatible CDJ/XDJs

Compatible CDJ/XDJs are as follows.

DJ PLAYER

> CDJ-TOUR1, CDJ-2000NXS2, CDJ-2000NXS, CDJ-2000, CDJ-900NXS, CDJ-900, CDJ-850, CDJ-350, CDJ-400, XDJ-1000MK2, XDJ-1000, XDJ-700

ALL-IN-ONE DJ SYSTEM

> XDJ-RX2, XDJ-RX, XDJ-R1, XDJ-AERO

For the latest information of compatible units, see (rekordbox.com/en/support/link.php).

### 1.1.2 DJM that can be used as an audio device

Following DJMs can be used as an audio device when connected using a USB cable.
Please note: DJMs are not able to control rekordbox.

DJ MIXER

> DJM-TOUR1, DJM-900NXS2, DJM-2000NXS, DJM-2000, DJM-900NXS, DJM-900SRT, DJM-850, DJM-750MK2, DJM-750, DJM-450, DJM-250MK2, DJM-4000

For the latest information of compatible units, see (rekordbox.com/en/support/link.php).

### 1.1.3 Compatible stand-alone Audio Interface

Compatible stand-alone audio interface are as follows:

Stand-alone Audio Interface

> INTERFACE 2

## 1.2 Preparations before connecting CDJ/XDJ/DJM/Audio Interface

This section describes required procedures before connecting CDJ/XDJ/DJM/Audio Interface to a computer.

### For Windows

Install the driver software for the CDJ/XDJ/DJM/Audio Interface on your computer prior to the connection.

We recommend you to upgrade the firmware to the latest version. If not, they may not work correctly.

### For Mac OS

When installing Mac OS standard driver software, installation of a driver software for each CDJ/XDJ is not required.

When you connect a DJM/Audio Interface and a Mac using a USB cable, you need to install the driver software for the DJM/Audio Interface. When you use multiple DJ players (CDJs/XDJs) as audio devices, you need to download CDJ/XDJ Aggregator\* to automatically create an "aggregate device" required for audio output to multiple CDJ/XDJs.

When you use an all-in-one DJ system, the Aggregator is not required.

\* To download the CDJ/XDJ Aggregator, visit Pioneer DJ support page to access **"Software & Firmware Updates" for each product at:**

pioneerdj.com/en/support/software/

The CDJ/XDJ Aggregator is applicable for all products in CDJ/XDJ series. When you downloaded one, you need not to do it again for other CDJ/XDJs.

We recommend you to upgrade the firmware for your CDJ/XDJ/DJM/Audio Interface to the latest version. If not, they may not work correctly.

### 1.2.1 Firmware updates

> Download the firmware and manual for your DJ equipment **from "Software & Firmware Updates" at** Pioneer DJ website (pioneerdj.com/en/support/software/).
>
> Follow the instructions described in the manual to update the firmware.

### 1.2.2 Driver installation

> The driver must be installed before connecting your computer to CDJ/XDJ/DJM/Audio Interface.
>
> When they are already connected, disconnect them.
>
> > Note: The driver is common for DJ players (CDJ/XDJ).
>
> Download the driver and manual for your DJ equipment **from "Software & Firmware Updates" at** Pioneer DJ website (pioneerdj.com/en/support/software/).
>
> Follow the instructions described in the manual to install the driver.

### 1.2.3 HID SETTING for CDJ-2000/900

On CDJ-2000/900, HID SETTING should be selected as [ADVANCED].

Press and hold the [MENU] button on the CDJ to open [UTILITY]. Select [ADVANCED] for the HID SETTING.

Note: This process is NOT required for other CDJs/XDJs.

### 1.2.4 (For Mac only) Creation of an aggregate device using CDJ/XDJ Aggregator

(If you use your DJ equipment other than DJ player (CDJ/XDJ) as an audio device, this process is NOT necessary.)

(If you use the same DJ equipment when you launched the CDJ/XDJ Aggregator, this process is NOT necessary.)

When you launch the CDJ/XDJ Aggregator after connecting a Mac to CDJ/XDJ, the following window will appear and the aggregate device will be created. Then click [OK] and close the window.

When using the CDJ-350/850, be sure to press the [PC] button on the CDJ to enter PC connection mode before launching the CDJ/XDJ Aggregator.

[Screenshot: CDJ/XDJ Aggregator window showing "Created the aggregate device, 'Pioneer CDJ/XDJ,' which contains the following devices (remember to set up this aggregate device for audio output using your DJ software)." with PIONEER CDJ-850 listed twice, and an OK button]

> Depending on USB connection status, the aggregate device may not be created correctly or tracks may not be played properly from the created aggregate device. This might be due to band width of a USB bus or USB hub on the Mac connected to CDJ/XDJ. The same thing may happen even if you do not use the CDJ/XDJ Aggregator and create an aggregate device by Audio MIDI setting (Mac OS). Please rename the aggregator as "Pioneer CDJ/XDJ".
>
> Changing a USB hub or port may solve the problem.

## 1.3 To use DJ players as audio devices

[Screenshot: Connection diagram showing a laptop connected to two CDJ players via USB cables, with both CDJ players connected to a DJM mixer via audio cables]

1. Connect your computer and all DJ players using USB cables.
   Connect DJ players and a DJM using audio cables.

   > Note: We recommend to connect DJ players and your computer directly with USB cables. If you use a USB hub, they may not work correctly depending on the USB hub.

2. Launch rekordbox and select PERFORMANCE mode.

3. Press the [Link] button on each DJ player (for CDJ-350/850, the [PC] button).

   [CONNECTED] appears on the screen of the DJ players. When you use DJ players supporting PRO DJ LINK, contents of USB devices or SD cards connected to other devices may appear on the screen when pressing the LINK button. In this case, press the [BACK] button of the DJ players.

4. Turn the browser rotary encoder of a DJ player to select a rekordbox DECK number. Press the browser rotary encoder to assign the DECK to the DJ player.

5. Repeat the step 4 for all the DJ players to complete the DECK assignment.

This completes the control settings. You can control rekordbox from the DJ players.
Next, follow the steps below for audio output setting to output from rekordbox to the DJ players.

6. Click the gear icon in the upper right of the rekordbox screen to open the [Preferences] window.

7. Click the audio icon to open the [Audio] setting window.

   [Screenshot: Audio icon]

8. At the [Audio] section, select [Pioneer CDJXDJ ASIO] for Windows.
   For Mac, select [Pioneer CDJ/XDJ].
   If it does not appear in the drop-down menu, install the driver again or run CDJ/XDJ Aggregator again.

   [Screenshot: Audio setting showing "Pioneer CDJXDJ ASIO" selected with a Driver button]

9. Select [External] for [Mixer Mode].

   [Screenshot: Mixer Mode setting showing Internal and External radio buttons, with External selected]

10. Check settings for [Output channels]. Monitor the sound from the DJ players and adjust the settings so that the sound from the assigned deck can be heard from each DJ player.

This completes the settings for DJ players.

## 1.4 To use a DJ mixer as an audio device

[Screenshot: Connection diagram showing a laptop connected to two CDJ players via USB cables and to a DJM mixer via USB cable, with CDJ players connected to the DJM via audio cables]

This section describes how to set up connection with a DJ mixer as an audio device using DJM-900NXS as an example. Install driver software before connection as described in 1.2.2 Driver installation.

1. Connect your PC/Mac with a DJM using a USB cable.

   > Note: We recommend to connect CDJ/XDJs, your computer and a DJM directly with USB cables. If you use a USB hub, they may not work correctly depending on the USB hub.

2. Launch rekordbox and select PERFORMANCE mode.

3. Click the gear icon in the upper right of the rekordbox screen to open the [Preferences] window.

4. Click the audio icon to open the [Audio] window.

   [Screenshot: Audio icon]

> Note: When using DJM-900NXS2 and CDJ-2000NXS2
>
> By connecting the DJM-900NXS2 to your computer using a USB cable, you can control rekordbox dj decks from up to 4 units of the CDJ-2000NXS2 via PRO DJ LINK.
>
> (Note: Please update the DJM-900NXS2 firmware to the latest version.)
>
> [Screenshot: Connection diagram showing a laptop connected to a DJM-900NXS2 via USB cable, with the DJM-900NXS2 connected to a switching hub via LAN cable, and four CDJ-2000NXS2 units connected to the switching hub via LAN cables. Units labeled left to right: CDJ-2000NXS2, CDJ-2000NXS2, DJM-900NXS2, CDJ-2000NXS2, CDJ-2000NXS2]
>
> The settings for the CDJ-2000NXS2
>
> Press the LINK button on the CDJ-2000NXS2. When [CONTROL MODE(DJM-USB \*)] appears on the LCD screen, turn and press the browser rotary encoder to select it. When [USB MIDI] "PUSH TO CONNECT." appears, push the rotary encoder. Now you can control the rekordbox dj deck corresponding to each player number of the CDJ-2000NXS2.

5. At the [Audio] section, select the name of the DJM you connected.
   (The figure below is an example on Windows. On Mac, it is displayed as [PIONEER DJM-900nexus]).
   If the DJM name does not appear in the drop-down menu, install the driver again.

   [Screenshot: Audio setting showing "PIONEER DJM-900nexus ASIO" selected with a Driver button]

6. Select [External] for [Mixer Mode].

   [Screenshot: Mixer Mode setting showing Internal and External radio buttons, with External selected]

7. Check the settings in [Output channels]. Play a track on a deck on rekordbox, monitor the sound from the DJM. If the setting is not the one you wish, change it.

   To check the audio from the DJM, select [USB] using the input selector switch for respective channel.

   > Example: The figure below is the input selector switch on the DJM-900NXS.

   [Screenshot: Input selector switch on DJM-900NXS showing CD/LINE, PHONO, DIGITAL, and USB positions, with USB highlighted]

   (The figure below is an example on Windows.)

   [Screenshot: Output channels setting showing Output Deck1 with L: DJM-900nexus OUT 3 and R: DJM-900nexus OUT 4, and Output Deck2 with L: DJM-900nexus OUT 5 and R: DJM-900nexus OUT 6]

The completes audio setting for DJM.

## 1.5 To use a stand-alone audio interface

[Screenshot: Connection diagram showing a laptop connected to two CDJ players via USB cables and to a stand-alone audio interface (INTERFACE 2) via USB cable, with the audio interface connected to a DJM mixer via audio cables]

This section describes how to set up connection with a stand-alone audio interface as an audio device. Install driver software before connection as described in 1.2.2 Driver installation.

1. Connect your PC/Mac with CDJs/XDJs using USB cables.

2. Connect your PC/Mac with a stand-alone audio interface using a USB cable.

   > Note: We recommend to connect CDJ/XDJs, your computer and a stand-alone audio interface directly with USB cables. If you use a USB hub, they may not work correctly depending on the USB hub.

3. Connect output terminals of the stand-alone audio interface and CD/LINE terminals of a DJ mixer using audio cables.

4. Launch rekordbox and select PERFORMANCE mode.

5. Click the gear icon in the upper right of the rekordbox screen to open the [Preferences] window.

6. Click the audio icon to open the [Audio] window.

   [Screenshot: Audio icon]

7. At the [Audio] section, select the name of the stand-alone audio interface you connected.
   (The figure below is an example on Windows. On Mac, it is displayed as [INTERFACE 2]).
   If it does not appear in the drop-down menu, install the driver again.

   [Screenshot: Audio setting showing "INTERFACE2 ASIO" selected with a Driver button]

8. Select [External] for [Mixer Mode].

   [Screenshot: Mixer Mode setting showing Internal and External radio buttons, with External selected]

9. Setting Utility: Go to [ROUTING] tab -> [Output Routing] and select [USB] (not [THRU]).

   [Screenshot: Setting Utility button highlighted in the Input channels area of the Preferences window]

   [Screenshot: INTERFACE2 Setting Utility window showing the ROUTING tab selected, with tabs for ROUTING, ASIO, and About. The window shows a laptop connected to an INTERFACE 2 device with "Connected" status. Output Routing section shows DECK1: USB and DECK2: USB drop-downs. Instructions state "When you wish to output USB AUDIO from the INTERFACE2, select [USB] for [Output Routing]." and "Select the [LINE/PHONO] switch on the INTERFACE2 depending on the equipment connected to the [INPUT] on the INTERFACE2."]

10. Check the settings in [Output channels]. Play a track on a deck on rekordbox, monitor the sound from the DJM. If the setting is not the one you wish, change it.

This completes the settings for a stand-alone audio interface.

## 1.6 To use an all-in-one DJ system as an audio device

[Screenshot: Connection diagram showing a laptop connected to an XDJ-RX all-in-one DJ system via a single USB cable]

1. Connect your PC/Mac and an all-in-one DJ system using a USB cable.

2. Launch rekordbox and select PERFORMANCE mode.

3. Open UTILITY on the all in one DJ system and set MIXER MODE.

   > XDJ-RX: Press and hold the [MENU] button to open the UTILITY setting. Select [MIDI CONTROL] for [MIXER MODE].
   >
   > XDJ-R1/XDJ-AERO: Press and hold the [INFO] button to open the UTILITY setting. Select [PC] for [MIXER MODE].

4. Press the MIDI button of the all-in-one DJ system. (XDJ-R1/XDJ-AERO has a MIDI button per player.)

5. Follow the instructions on the all-in-one system screen and press the LOAD button. (XDJ-R1/XDJ-AERO: Press the browser rotary encoder)

This completes the control settings for an all-in-one DJ system. You can control rekordbox using the all-in-one DJ system. Next, follow the steps for audio output setting to output from rekordbox to all-in-one DJ system.

6. Click the gear icon in the upper right of the rekordbox screen to open the [Preferences] window.

7. Click the audio icon to open the [Audio] setting window.

   [Screenshot: Audio icon]

8. At the [Audio] section, select [PIONEER \*\*\*\* ASIO] for Windows. For Mac, select [PIONEER \*\*\*\*].
   (\*\*\*\* is the unit name.) If it does not appear in the drop-down menu, install the driver again or run CDJ/XDJ Aggregator again.

   [Screenshot: Audio setting showing "PIONEER XDJ-RX ASIO" selected with a Driver button]

9. Check settings for [Output channels]. Monitor the sound from the all-in-one DJ system.

The completes settings for the all-in-one DJ system.

## 1.7 Troubleshooting

| Troubles | Check this | Solution |
|---|---|---|
| No sound output. | Open [Preferences] > [Audio]. Check the [Audio] setting. | When you use CDJ/XDJs as audio devices, select the following: **For Windows** "Pioneer CDJ/XDJ ASIO" **For Mac** "Pioneer CDJ/XDJ" When you use a DJM as an audio device, select the following: **For Windows** "Pioneer DJM (unit name) ASIO" **For Mac** "DJM (unit name)" When you use a stand-alone audio interface, select the following: **For Windows** "INTERFACE 2 ASIO" **For Mac** "INTERFACE 2" |
| | Open [Preferences] > [Audio]. Check the [Output channels]. | When you use CDJ/XDJs as audio devices, set the CDJ/XDJ channels to Output Deck L/R. When you click each drop-down menu of the Output Deck, [Front Left/Front Right/Front Left/Front Right] will be displayed. The first set of "Front Left/Right" is for one CDJ/XDJ and the second set of "Front Left/Right" is for another CDJ/XDJ. Set the L/R of each Output Deck you wish to direct audio output to CDJ/XDJ. |

[Screenshot: Output channels setting showing Output Deck1 with L: Front Left selected, and a drop-down menu showing Front Left (checked), Front Right, Front Left, Front Right options]

| Troubles | Check this | Solution |
|---|---|---|
| | | When you use a DJM as an audio device, set the DJM channels to Output Deck L/R. Click each drop-down menu of the Output Deck and select the channel you wish to use. |
| | Check the setting of input selector switch of the DJM. | When you use CDJ/XDJs as audio devices, audio output will be directed from CDJ/XDJs to the DJM. In this case, set the input selector switches to the destination of audio cables, such as [CDJ/LINE]. When you use a DJM as an audio device, set the input selector switches to [USB]. |
| Audio output from rekordbox dj virtual Deck is sometimes directed adversely to CDJ/XDJs. | Open [Preferences] > [Audio]. Check the [Output channels]. | When you click each drop-down menu of the Output Deck, [Front Left/Front Right/Front Left/Front Right] will be displayed. The first set of "Front Left/Right" is for one CDJ/XDJ and the second set of "Front Left/Right" is for the other CDJ/XDJ. |

[Screenshot: Output channels setting showing Output Deck1 with L: Front Left selected, and a drop-down menu showing Front Left (checked), Front Right, Front Left, Front Right options, with the second set of Front Left/Front Right highlighted in red]

| Troubles | Check this | Solution |
|---|---|---|
| | | If the Output Deck is adversely selected, click the drop-down menu and change the setting. |
| Audio output from rekordbox dj virtual Deck is directed to the same CDJ/XDJ. | Open [Preferences] > [Audio]. Check the [Output channels]. | When you click each drop-down menu of the Output Deck, [Front Left/Front Right/Front Left/Front Right] will be displayed. The first set of "Front Left/Right" is for one CDJ/XDJ and the second set of "Front Left/Right" is for the other CDJ/XDJ. |

[Screenshot: Output channels setting showing Output Deck1 with L: Front Left selected, and a drop-down menu showing Front Left (checked), Front Right, Front Left, Front Right options, with the second set of Front Left/Front Right highlighted in red]

| Troubles | Check this | Solution |
|---|---|---|
| | | Select the L/R of each Output Deck you wish to direct audio output to the CDJ/XDJ from the drop-down menu. |
| I cannot control rekordbox dj from CDJ/XDJ. | Did you use a USB cable to connect the CDJ/XDJ to your computer? | When you wish to control rekordbox dj, connect your computer and CDJ/XDJs using USB cables. When they are not recognized even if you connect them with USB cables, reboot the CDJ/XDJs. Open [Preferences] > [Audio] > [Control device information] to check if the connected device is displayed or not. |
| | Aren't you using a USB hub? | When you connect multiple units (e.g. CDJ, DJM, etc.) to one USB hub, the units may not work correctly depending on the performance of the USB hub or the computer. Please connect the units directly to the computer. |

---

- Pioneer DJ is a trademark of the PIONEER CORPORATION and is used under license.
- rekordbox(TM) is a registered trademark of Pioneer DJ Corporation.
- Microsoft and Windows(R) are registered trademarks or trademarks of Microsoft Corporation in the U.S. and other countries.
- Mac, OS X and macOS are registered trademarks of Apple Inc. in the U.S. and other countries.
- ASIO is a trademark of Steinberg Media Technologies GmbH.
- Other product, technology and company names, etc. mentioned herein are the trade names or registered trademarks of their respective owners.

(C) 2018 Pioneer DJ Corporation. All rights reserved.

## Related Documents

- [guides/dvs-setup.md](dvs-setup.md) (compatibility, connection, dvs, equipment)
- [faq/hardware-compatibility.md](../faq/hardware-compatibility.md) (compatibility, dvs, equipment)
- [guides/midi-learn.md](midi-learn.md) (connection, equipment, performance)
- [guides/pro-dj-link-setup.md](pro-dj-link-setup.md) (connection, devices, equipment)
- [manual/31-preferences.md](../manual/31-preferences.md) (devices, dvs, equipment)
- [faq/usb-and-devices.md](../faq/usb-and-devices.md) (devices, equipment)
- [manual/10-mobile-devices.md](../manual/10-mobile-devices.md) (connection, devices)
- [manual/15-export-lan.md](../manual/15-export-lan.md) (connection, devices)
