---
id: pro-dj-link-setup
title: "PRO DJ LINK Setup Guide"
type: guide
source:
  file: "PRODJLINK_SetupGuide_ver2_en.pdf"
  pages: "1-17"
  version: "2.0"
topics: [connection, devices, equipment, link, pro-dj-link]
modes: [common, export]
confidence: verified
last_verified: "2026-02-17"
transcribed_by: agent
verified_by: agent
---

# PRO DJ LINK Setup Guide

# Contents

## Before you start

- Purpose of this guide — 2
- Overview of PRO DJ LINK — 2
- Type of equipment supporting PRO DJ LINK — 3
- Precautions for Using PRO DJ LINK — 3

## Connection Method

- Preparation for Connection — 4
- Using the Internet (CloudDirectPlay, StreamingDirectPlay) — 5
  - 1. Connecting DJ Equipment — 5
    - Wired LAN connection — 5
    - Wireless LAN (Wi-Fi) connection — 6
  - 2. Computer (Mac/Windows)/Mobile Device (iOS/Android) Connection — 7
    - Wired LAN connection — 7
    - USB Connection — 8
    - Wireless LAN (Wi-Fi) connection — 9
- PRO DJ LINK Only — 10
  - 1. Connecting DJ Equipment — 10
  - 2. Computer (Mac/Windows)/Mobile Device (iOS/Android) Connection — 11
    - Wired LAN connection — 11
    - USB Connection — 12
    - Wireless LAN (Wi-Fi) connection — 13
- DJ Equipment Setup and Connection Check — 14

## Others

- About trademarks and registered trademarks — 16

# Before you start

## Purpose of this guide

This guide describes the procedures for making the connections required to use the PRO DJ LINK functions.

## Overview of PRO DJ LINK

To enable various PRO DJ LINK functions developed by AlphaTheta, connect PRO DJ LINK-compatible DJ players and mixers, along with mobile devices (iOS/Android) or computers (Mac/Windows) that have rekordbox installed, using wireless LAN or LAN cable (CAT5e or higher).
The main functions enabled by PRO DJ LINK are as follows:

### Synchronization and Linkage Between DJ Equipment

- The BEAT SYNC feature allows automatic tempo and beat alignment across up to 6 DJ players <sup>1</sup>.
- The ON AIR DISPLAY feature allows you to instantly check the channel status (on-air status, channel number) of the DJ mixer.
- The QUANTIZE feature enables you to use loops and Hot Cues on the DJ player, and Beat FX on the DJ mixer, in sync with the beat of the track you are playing.

### Music Library Sharing

- The SD and USB Export feature allows you to share and play audio files on USB Storage Devices (flash drive, hard disk, etc.) and SD cards on up to 6 DJ players <sup>1</sup>.
- The LINK EXPORT feature allows you to share and play audio files stored on your computer (Mac/Windows) or mobile device running rekordbox on up to 6 DJ players <sup>1</sup>.

### Track Selection and Mixing Support

- With the KEY SYNC feature <sup>2</sup>, you can adjust the key of a track playing on a unit to match with the key of a track loaded on the master player.
- The TRAFFIC LIGHT feature makes it easy to find tracks that would make for a harmonic blend with the track playing on the master player.

### Synchronization with lighting

- The PRO DJ LINK LIGHTING feature enables rekordbox for Mac/Windows to receive phrase information from tracks played on a DJ player, when running in LIGHTING mode.
  It can then control lighting equipment via the RB-DMX1 or compatible third-party DMX interfaces.
  This allows synchronized lighting effects even in small clubs and venues without a dedicated lighting operator.

1 Up to 6 PRO DJ LINK-compatible DJ players can be connected. However, when using DJ players released before 2019 that support PRO DJ LINK, the maximum number of connectable units is 4. Music files on CD/DVD cannot be shared.

2 The KEY SYNC feature is available only when all the connected players are Key Sync-compatible.

## Type of equipment supporting PRO DJ LINK

**DJ players supporting PRO DJ LINK**
AlphaTheta DJ players equipped with LAN ports.

**DJ mixers supporting PRO DJ LINK**
AlphaTheta DJ mixers equipped with LAN ports.

For the latest information, visit the following website:
rekordbox.com/en/support/link/

## Precautions for Using PRO DJ LINK

- In large venues with multiple PRO DJ LINK systems, use a separate router for each system.
- When connecting DJ equipment to network devices such as a switching hub, do not create a loop connection (a configuration in which multiple ports within the same network are connected in a circular manner).
- Note that when using a single player, it is not possible to connect DJ equipment without using a LAN cable.

# Connection Method

## Preparation for Connection

### Setting the PLAYER No.

Usually, [PLAYER No.] is assigned to each DJ player automatically. However, if the [PLAYER No.] on the DJ player does not match the channel number on the DJ mixer, assign [PLAYER No.] manually using the following procedure.

1. Disconnect the LAN cable from the device, and remove any computers (Mac/Windows), mobile devices (iOS/Android), or storage media (USB/SD) connected to the DJ player. If you are logged into CloudDirectPlay, log out and remove all media.

\* You cannot change the [PLAYER No.] setting while a mobile device (iOS/Android) or storage media (USB/SD) is inserted, a LINK is established, or you are logged into CloudDirectPlay.

2. Turn on the power of the DJ players and DJ mixer.

3. Open [UTILITY] screen of the DJ player and set the [PLAYER No.] to match the mixer channel number.

- See the instruction manual for each DJ player to display the [UTILITY] screen.
- Set other DJ players in the same way.
- [PLAYER No.] is set to [AUTO] upon shipment from the factory.

[Screenshot: UTILITY screen of a DJ player showing DJ Settings tab with Player No. set to 2, Duplication set to All, MIDI Channel set to 1, and radio button options for Auto, 1-6. The PRO DJ LINK tab is visible in the left sidebar. The bottom of the screen shows PLAYER 2, REMAIN time 00:00, "Not Loaded." status, MT indicator, and 0.00% BPM.]

**Example: Settings screen when Player No. is set to 2**

## Using the Internet (CloudDirectPlay, StreamingDirectPlay)

### 1. Connecting DJ Equipment

#### Wired LAN connection

[Screenshot: Network diagram showing a wired LAN connection setup for DJ equipment with internet access. A legend shows a blue line representing a LAN cable. From top to bottom: Internet (globe icon) connects to a Router, which connects to a Switching hub. The Switching hub connects via LAN cables to five pieces of DJ equipment: two DJ players on the left, a DJ mixer in the center, and two DJ players on the right.]

- Connect the DJ equipment to a switching hub or a router with a built-in switching hub using LAN cables as shown in the diagram.

\* Connect the router and switching hub first. After confirming that the power of the router and switching hub (or the router with a built-in switching hub) is fully turned on, connect the DJ equipment. If the power is turned off or the LAN cable is disconnected after the connection, turn off the DJ equipment once and reconnect following the steps above.

\* When connecting a combination of Wi-Fi-compatible and non-compatible DJ equipment, use this connection method. For details on supported models, refer to the instruction manual of each device.

- If you are not connecting a computer (Mac/Windows) or a mobile device (iOS/Android)---for example, when using a USB storage device or SD card--- then proceed to the 'DJ Equipment Setup and Connection Check' section (page 14).

#### Wireless LAN (Wi-Fi) connection

[Screenshot: Network diagram showing a wireless LAN connection setup for DJ equipment with internet access. A legend shows a blue line representing a LAN cable. From top to bottom: Internet (globe icon) connects to a Wireless LAN router (with Wi-Fi signal icon), which connects to a Switching hub. The Switching hub connects to five pieces of DJ equipment: two DJ players on the left, a DJ mixer in the center, and two DJ players on the right. All DJ players show Wi-Fi signal icons indicating wireless connectivity.]

- Connect the DJ equipment to a switching hub or a router with a built-in switching hub using LAN cables as shown in the diagram.
- If you are not connecting a computer (Mac/Windows) or a mobile device (iOS/Android)---for example, when using a USB storage device or SD card--- then proceed to the 'DJ Equipment Setup and Connection Check' section (page 14).

\* When connecting Wi-Fi-compatible DJ players to each other, this connection method can be used. For details on supported models, refer to the instruction manual of each device.

### 2. Computer (Mac/Windows)/Mobile Device (iOS/Android) Connection

#### Wired LAN connection

[Screenshot: Network diagram showing a wired LAN connection setup for a computer/mobile device with DJ equipment and internet access. A legend shows a blue line representing a LAN cable. A Computer and Mobile Device are shown at the top left. From top: Internet (globe icon) connects to a Wireless LAN router or router (two router options shown with "or" between them). Below that is a Switching hub or router with built-in switching hub. The Computer/Mobile Device, along with five pieces of DJ equipment (two DJ players, a DJ mixer, and two more DJ players), all connect to the switching hub via LAN cables.]

- Connect the computer (Mac/Windows) or mobile device (iOS) to a switching hub or a router with a built-in switching hub using LAN cables as shown in the diagram.

\* When connecting a computer (Mac/Windows) or mobile device (iOS) to a switching hub or a router with a built-in switching hub, prepare the appropriate adapter or conversion device according to your device's specifications.

\* rekordbox for Android does not support wired connection.

\* Before switching to a wired connection, turn off Wi-Fi on your mobile device.

\* When connecting Stagehand <sup>1</sup> via PRO DJ LINK, Use the same connection method as for mobile devices.

1 Stagehand is an application that allows you to wirelessly connect to AlphaTheta DJ players and mixers ("devices") so you can check their status, change their setting, and remote control them.

#### USB Connection

[Screenshot: Network diagram showing a USB connection setup for a computer/mobile device with DJ equipment and internet access. A legend shows a blue line representing a LAN cable and a black line representing a USB cable. A Computer and Mobile Device are shown at the top left. From top: Internet (globe icon) connects to a Wireless LAN router or router. Below is a Switching hub or router with built-in switching hub. Five pieces of DJ equipment (two DJ players, a DJ mixer, two DJ players) connect to the switching hub via LAN cables. The Computer/Mobile Device connects to one of the DJ players via a USB cable.]

- Connect the computer (Mac/Windows) or mobile device (iOS) to the DJ player using the USB port located on the rear panel of the DJ player, as shown in the diagram.

\* When connecting a computer (Mac/Windows) or mobile device (iOS) to the DJ player, prepare an appropriate adapter or conversion device that matches the connector type of your device.

\* rekordbox for Android does not support wired connection.

\* Before switching to a wired connection, turn off Wi-Fi on your mobile device.

\* When connecting Stagehand <sup>1</sup> via PRO DJ LINK, Use the same connection method as for mobile devices.

1 Stagehand is an application that allows you to wirelessly connect to AlphaTheta DJ players and mixers ("devices") so you can check their status, change their setting, and remote control them.

#### Wireless LAN (Wi-Fi) connection

[Screenshot: Network diagram showing a wireless LAN connection setup for a computer/mobile device with DJ equipment and internet access. A legend shows a blue line representing a LAN cable and a dotted line representing a Wireless Connection. A Computer and Mobile Device are shown at the top left, connected wirelessly (dotted line) to a Wireless router with built-in switching hub. From top: Internet (globe icon) connects to the wireless router. Five pieces of DJ equipment (two DJ players, a DJ mixer, two DJ players) connect to the wireless router via LAN cables.]

- Connect the computer (Mac/Windows) or mobile device (iOS/Android) to a wireless router with a built-in switching hub using LAN cables, as shown in the diagram.

\* When connecting Stagehand <sup>1</sup> via PRO DJ LINK, Use the same connection method as for mobile devices.

1 Stagehand is an application that allows you to wirelessly connect to AlphaTheta DJ players and mixers ("devices") so you can check their status, change their setting, and remote control them.

## PRO DJ LINK Only

### 1. Connecting DJ Equipment

[Screenshot: Network diagram showing a PRO DJ LINK Only connection setup for DJ equipment without internet. A legend shows a blue line representing a LAN cable. A Switching hub or router with built-in switching hub is at the top. Five pieces of DJ equipment (two DJ players, a DJ mixer, two DJ players) connect to the switching hub via LAN cables.]

- Connect the DJ equipment to a switching hub or a router with a built-in switching hub using LAN cables as shown in the diagram.
- If you are not connecting a computer (Mac/Windows) or a mobile device (iOS/Android)---for example, when using a USB storage device or SD card--- then proceed to the 'DJ Equipment Setup and Connection Check' section (page 14).

\* If you plan to use the Internet (CloudDirectPlay, StreamingDirectPlay) later, turn off all DJ equipment, then reconnect them following the steps in "Using the Internet (CloudDirectPlay, StreamingDirectPlay)" on page 5.

### 2. Computer (Mac/Windows)/Mobile Device (iOS/Android) Connection

#### Wired LAN connection

[Screenshot: Network diagram showing a wired LAN connection setup for a computer/mobile device with DJ equipment, without internet. A legend shows a blue line representing a LAN cable. A Computer and Mobile Device are shown at the top left. A Switching hub or router with built-in switching hub is at the top center. The Computer/Mobile Device, along with five pieces of DJ equipment (two DJ players, a DJ mixer, two DJ players), all connect to the switching hub via LAN cables.]

- Connect the computer (Mac/Windows) or mobile device (iOS) to a switching hub or a router with a built-in switching hub using LAN cables as shown in the diagram.

\* When connecting a computer (Mac/Windows) or mobile device (iOS) to a switching hub or a router with a built-in switching hub, prepare the appropriate adapter or conversion device according to your device's specifications.

\* rekordbox for Android does not support wired connection.

\* Before switching to a wired connection, turn off Wi-Fi on your mobile device.

\* If you plan to use the internet (CloudDirectPlay, StreamingDirectPlay) later, turn off the power of all DJ equipment once and reconnect them following the steps in "Using the Internet (CloudDirectPlay, StreamingDirectPlay)" on page 5.

\* When connecting Stagehand <sup>1</sup> via PRO DJ LINK, Use the same connection method as for mobile devices.

1 Stagehand is an application that allows you to wirelessly connect to AlphaTheta DJ players and mixers ("devices") so you can check their status, change their setting, and remote control them.

#### USB Connection

[Screenshot: Network diagram showing a USB connection setup for a computer/mobile device with DJ equipment, without internet. A legend shows a blue line representing a LAN cable and a black line representing a USB cable. A Computer and Mobile Device are shown at the top left. A Switching hub or router with built-in switching hub is at the top center. Five pieces of DJ equipment connect to the switching hub via LAN cables. The Computer/Mobile Device connects to one of the DJ players via a USB cable.]

- Connect the computer (Mac/Windows) or mobile device (iOS) to the DJ player using the USB port located on the rear panel of the DJ player, as shown in the diagram.

\* When connecting a computer (Mac/Windows) or mobile device (iOS) to the DJ player, prepare an appropriate adapter or conversion device that matches the connector type of your device.

\* rekordbox for Android does not support wired connection.

\* Before switching to a wired connection, turn off Wi-Fi on your mobile device.

\* If you plan to use the internet (CloudDirectPlay, StreamingDirectPlay) later, turn off the power of all DJ equipment once and reconnect them following the steps in "Using the Internet (CloudDirectPlay, StreamingDirectPlay)" on page 5.

\* When connecting Stagehand <sup>1</sup> via PRO DJ LINK, Use the same connection method as for mobile devices.

1 Stagehand is an application that allows you to wirelessly connect to AlphaTheta DJ players and mixers ("devices") so you can check their status, change their setting, and remote control them.

#### Wireless LAN (Wi-Fi) connection

[Screenshot: Network diagram showing a wireless LAN connection setup for a computer/mobile device with DJ equipment, without internet. A legend shows a blue line representing a LAN cable and a dotted line representing a Wireless Connection. A Computer and Mobile Device are shown at the top left, connected wirelessly (dotted line) to a Wireless router with built-in switching hub. Five pieces of DJ equipment (two DJ players, a DJ mixer, two DJ players) connect to the wireless router via LAN cables.]

- Connect the computer (Mac/Windows) or mobile device (iOS/Android) to a wireless router with a built-in switching hub using LAN cables, as shown in the diagram.

\* If you plan to use the internet (CloudDirectPlay, StreamingDirectPlay) later, turn off the power of all DJ equipment once and reconnect them following the steps in "Using the Internet (CloudDirectPlay, StreamingDirectPlay)" on page 5.

\* When connecting Stagehand <sup>1</sup> via PRO DJ LINK, Use the same connection method as for mobile devices.

1 Stagehand is an application that allows you to wirelessly connect to AlphaTheta DJ players and mixers ("devices") so you can check their status, change their setting, and remote control them.

## DJ Equipment Setup and Connection Check

### 1. Connecting audio cables

Connect the audio cable so that the [PLAYER No.] on the DJ player matches the channel number of the audio input terminal on the DJ mixer.

[Screenshot: Connection diagram showing audio cables (orange lines) connecting DJ equipment. Two DJ players on the left and two DJ players on the right are connected via audio cables to a DJ mixer in the center. The audio cables run from the rear of each DJ player to the corresponding channel inputs on the DJ mixer.]

### 2. Checking the connections

1. Make sure that the player number displayed in the lower-left corner of the DJ player's screen is lit in white.

[Screenshot: Two DJ player screen close-ups showing the PLAYER number indicator in the lower-left corner. The left example shows "PLAYER 2" with the number lit in white, labeled "Connected to the LINK". The right example shows "PLAYER 2" with the number not lit (grayed out), labeled "Can't connect to the LINK". Both show REMAIN time 00:00.]

\* LINK is not established when the player number is not lit in white (grayed-out or blinking). In this case, redo the connection procedure.

2. If a computer (Mac/Windows) or mobile device (iOS/Android) is connected, confirm the connection as follows:
   For computers, click the LINK button displayed on the rekordbox screen.
   For mobile devices, tap the "Connect to CDJ/XDJ/DJM" button in the rekordbox settings screen.

[Screenshot: Two images side by side. Left image labeled "Computer" shows the rekordbox application interface with the LINK button highlighted with a red circle in the lower-left corner. Right image labeled "Mobile Device" shows the rekordbox mobile settings screen with menu items: "Connect to rekordbox (Mac/Win)", "Import songs", "Audio", "General", "Connect to CDJ/XDJ/DJM" (highlighted with a red rectangle), "CloudDirectPlay settings", "Streaming services", "Support".]

3. Confirm that the DJ player's number matches the mixer channel where the level indicator lights up. If you are using storage media (USB/SD), insert it into the USB/SD port located on the top panel of the DJ player and perform the same check.

[Screenshot: Two images side by side. Left image shows a DJ player screen displaying a track called "Allergens" at 04:31, with waveform display, PLAYER 2 indicator circled in red in the lower-left corner, showing REMAIN time 02:12.514, 0.00%, and 87.0 BPM in Fm key. Right image shows a DJ mixer channel strip with level meters for channels 1 and 2, with the channel 2 TRIM knob area circled in red, showing EQ knobs (HI, MID, LOW) and level indicators.]

\* If the player number and channel number do not match, the connection may not be correct. Then, redo the connection procedure again.

# Others

## About trademarks and registered trademarks

- PRO DJ LINK(TM) is a trademark or registered trademark of AlphaTheta Corporation.
- rekordbox(TM) is a trademark or registered trademark of AlphaTheta Corporation.
- Stagehand(TM) is a trademark or registered trademark of AlphaTheta Corporation.
- Windows is a trademark or registered trademark of Microsoft Corporation in the U.S. and other countries.
- Mac is a trademark of Apple Inc., registered in the U.S. and other countries and regions.
- iOS is a trademark or registered trademark of Cisco in the U.S. and other countries and regions.
- Android is a trademark or registered trademark of Google LLC.
- Wi-Fi is a registered trademark of Wi-Fi Alliance(R).
- Other product, technology and company names, etc. mentioned herein are trademarks or registered trademarks of their respective owners.

---

Copyright 2025 AlphaTheta Corporation. All right reserved.

## Related Documents

- [faq/usb-and-devices.md](../faq/usb-and-devices.md) (devices, equipment, pro-dj-link)
- [guides/performance-mode-connection.md](performance-mode-connection.md) (connection, devices, equipment)
- [manual/13-export-pro-dj-link.md](../manual/13-export-pro-dj-link.md) (connection, link, pro-dj-link)
- [manual/15-export-lan.md](../manual/15-export-lan.md) (connection, devices, pro-dj-link)
- [manual/19-performance-preparing.md](../manual/19-performance-preparing.md) (devices, equipment, link)
- [guides/cloud-direct-play.md](cloud-direct-play.md) (devices, pro-dj-link)
- [guides/dvs-setup.md](dvs-setup.md) (connection, equipment)
- [guides/midi-learn.md](midi-learn.md) (connection, equipment)
