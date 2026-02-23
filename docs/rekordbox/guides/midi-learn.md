---
id: midi-learn
title: "MIDI Learn Operation Guide"
type: guide
source:
  file: "rekordbox7.0.5_midi_learn_operation_guide_EN.pdf"
  pages: "1-8"
  version: "7.0.5"
topics: [connection, equipment, midi, performance]
modes: [performance]
confidence: verified
last_verified: "2026-02-17"
transcribed_by: agent
verified_by: agent
---

# MIDI LEARN Operation Guide

# 1 OVERVIEW

This guide explains how to use MIDI LEARN function in rekordbox (Performance mode). For instructions on rekordbox in general, see the Operating Instructions of rekordbox.

rekordbox Operation Instructions rekordbox.com/en/download/#manual

# 2 MIDI LEARN

MIDI LEARN enables free assignment of rekordbox (Performance mode) features to MIDI controllable equipment.

You can assign various functions of rekordbox, such as Play/Pause and Hot Cue, to the controls (e.g. buttons and knobs) of your MIDI controllable equipment as you wish.

If the equipment supports rekordbox, you can change the preset MIDI mapping.

This feature is supposed to be used with AlphaTheata products. We do not provide support for configuring third party products.

# 3 MIDI LEARN operation procedures

## 3.1 Connection of MIDI controllable equipment

**1. Connect your computer with DJ equipment you wish to assign MIDI.**

**2. Launch rekordbox and select [PERFORMANCE] mode.**

[Screenshot: rekordbox mode selection dropdown showing EXPORT, PERFORMANCE (selected), LIGHTING, and EDIT options, with the Performance mode screen visible in the background]

Note: MIDI LEARN function works on MIDI controllable equipment connected to your computer with a USB cable. (If it is not connected to your computer, you are not able to assign MIDI.)

## 3.2 MIDI setting window

**Click the MIDI button in upper right of the screen, next to the Preference icon.**

[Screenshot: rekordbox Performance mode screen with the MIDI button highlighted in the upper right corner, next to the PAD button and info icon]

-> MIDI setting window appears.

**MIDI setting window**

[Screenshot: MIDI setting window showing a connected DDJ-FLX4 device with a list of MIDI setting items including functions like Loop In, Loop Out, Master, PlayPause, Sync, TapBpm, TempoRange, and Tempo with their corresponding Deck, MIDI IN, Type, MIDI OUT, and Comment columns. Bottom bar shows buttons labeled LEARN [5], ADD [6], DUPLICATE [7], DELETE [8], IMPORT [9], EXPORT [10]. Device selector [1] at top left, DEFAULT button [2] at top right, tabs [3] for DECK/PAD/FX/SAMPLER/MIXER/BROWSE/OTHER/VIDEO/LIGHTING/MIXPOINTLINK, and the MIDI setting item list area [4]]

**[1] Menu to select a connected device**

Click the right arrow to display MIDI controllable equipment currently connected to your computer in the drop-down menu.

When you select one, MIDI setting of the selected equipment will be displayed in the MIDI setting item list and you can edit ([LEARN], [ADD], [DUPLICATE] and [DELETE]) , import saved setting ([IMPORT]), save ([EXPORT]) and reset ([DEFAULT]) the MIDI setting.

**[2] DEFAULT button**

Click the [DEFAULT] button to reset the MIDI setting of the selected equipment back to default.

For equipment supporting rekordbox, the MIDI setting will be reset back to the factory default. If the equipment does not support rekordbox, the MIDI setting will be unset.

**[3] TAB to switch MIDI setting item list**

Click a tab to display a list of MIDI setting items for the function of the tab name. Functions that can be added using the [ADD] button will be confined to the category of the function of the tab name.

**[4] MIDI setting item list**

You can display and edit MIDI setting of the equipment selected at the menu of the **Menu to select a connected device** ([1]).

Items shown on the columns:

**[Function]**: Name of the function assigned to the equipment.

**[Deck]**: Deck number.

Click to select the Deck number. 4-digit number in hex.

**[MIDI IN]**: MIDI code received from the equipment. It is described as 4-digit number in hex. Each code is a trigger for a function to operate. If you operate the equipment while the [LEARN] button is turned on, rekordbox will receive the MIDI code from the control (e.g. a button) of the equipment. You can also click the cell to edit the MIDI code directly. (Please enter 4-digit number in hex.)

**[Type]**: Type of the controls (e.g. buttons, faders)

When you assign functions with the [LEARN] button on, assign functions by operating the controls (e.g. buttons and faders) set in the Type cell.

Button, Button(for Pad), Pad:

> Assign a function to a button or pad on the equipment.

Knob/Slider (0h-7Fh), Knob/Slider (0h-3FFFh):

> Assign the function to a knob or fader on the equipment.
>
> Select 128 (0h-7Fh) or 16384 (0h-3FFFh) according to the resolution capability.

Rotary:

> Assign a function to a rotary encoder.

Indicator:

> Illumination information is sent to the equipment.
>
> For this type, you cannot assign functions by operating the equipment.
>
> Click the [MIDI OUT] and enter a MIDI code.

Value:

> This type is used to set MIDI codes for Needle Search and for velocity of Velocity Sampler.
>
> a) Needle Search:
>
>> Set MIDI code by sliding a finger along the ribbon controller of the equipment.
>
> b) Velocity Sampler:
>
>> MIDI for Velocity sampler can only be set to a pad that sends 2 MIDI data (Note On and Control Change (CC)) when they are pressed. The velocity is set by the CC. Enter Status byte (hex) and Data 1 (hex), as the velocity sampler MIDI cannot be assigned by pressing pads.
>>
>> Velocity Sampler [MIDI IN] data input: Bnxx
>>
>> n: MIDI CH 1-16 (0h-Fh)
>>
>> xx: Data 1 (00h-FFh)

**[MIDI OUT]**: MIDI code sent to the equipment.

If the function includes indicator (e.g. light on/off), the same code as the [MIDI IN] will be sent automatically to the [MIDI OUT]. Click the cell to directly edit the MIDI code.

**[Comment]**: Click to leave any comments.

You can sort the MIDI setting items by clicking the name of each column.

**[5] LEARN button**

This is to assign MIDI code. Click this icon to toggle ON/OFF. When you operate a control (e.g. button) of the equipment while the LEARN button is ON, the MIDI code sent from the equipment will be assigned to the selected function.

**[6] ADD button**

Click this button to display assignable functions in a menu. Displayed functions are limited to the category of the function of the tab name. If you select a function on the menu, it will be added to the last line of the MIDI setting item list.

**[7] DUPLICATE button**

Click this button to duplicate the selected function.

**[8] DELETE button**

Click this button to delete the selected function.

**[9] IMPORT button**

If you click this button, a dialog box will appear to select a MIDI setting file. If you select a MIDI file you exported (saved), the MIDI file will be imported to overwrite the current MIDI setting.

**[10] EXPORT button**

Click to show a dialog box to export (save) MIDI setting.

## 3.3 Selection of equipment (when multiple MIDI controllable units are connected)

If only one unit is connected, the unit will be automatically selected.

When 2 or more units are connected, select the unit you wish to set MIDI as follows:

**1 Click the arrow in the menu to select a connected device.**

-> MIDI controllable devices currently USB-connected to your computer will be shown in the drop-down menu.

**2 Select the unit you wish to assign functions.**

-> Current MIDI setting of the selected unit will be shown in the MIDI setting item list. (If the MIDI is not set, a blink will be displayed.)

## 3.4 Adding functions

**1 Click the [ADD] button.**

-> Functions in the category of the selected tab function will be displayed in a menu.

**2 Select a function.**

-> The selected function will be added to the last line of the MIDI setting item list.

- Use [DUPLICATE] when you add the same function to Deck1, 2, 3 or 4.
- Also use [DELETE] to delete unnecessary functions.

## 3.5 Assignment of functions to MIDI controllable equipment

**1 Click the function you wish to assign.**

-> The line of the selected function will be highlighted.

**2 Click the [LEARN] button.**

-> The [LEARN] button will be enabled with the light on.

**3 Operate the equipment.**

Operate the control (e.g. button, fader) of the equipment set by the [Type] display. (See p.4. _[4] MIDI setting item list_ for details of [Type].)

-> When the MIDI code is sent from the equipment, the code will be shown on the [MIDI IN].

Note: One MIDI code cannot be assigned to multiple functions.

## 3.6 Export of MIDI setting (save MIDI setting file with a different name)

When you use the [LEARN] and other edit buttons in the MIDI setting window, the setting will be automatically saved when the MIDI setting window is closed. If you wish to save the file currently you are editing, you can save the MIDI setting file with a different name by using [EXPORT] button.

**1 Click the [EXPORT] button.**

-> A dialog box will appear.

**2 Specify the location to save and click the [Save] button.**

-> The MIDI setting file will be saved.

## 3.7 Import of MIDI setting

Using the [IMPORT] button, you can import the exported (saved) MIDI setting file and overwrite the current MIDI setting of the selected equipment.

**1 Click the [IMPORT] button.**

-> A message will appear to confirm whether you will export the MIDI setting file you are currently editing (save the file with a different name).

**2 Click [Yes] to save it. Click [No] if you do not want to save it.**

-> A dialog box will appear.

**3 Select the MIDI setting file you wish to import (save) and click [Open].**

-> The MIDI setting item list will be overwritten by the imported MIDI setting file.

---

- rekordbox(TM) is a trademark or registered trademark of AlphaTheta Corporation.
- Other product, technology and company names, etc. mentioned herein are trademarks or registered trademarks of their respective owners.

(C) AlphaTheta Corporation.

## Related Documents

- [guides/dvs-setup.md](dvs-setup.md) (connection, equipment, performance)
- [guides/performance-mode-connection.md](performance-mode-connection.md) (connection, equipment, performance)
- [guides/pro-dj-link-setup.md](pro-dj-link-setup.md) (connection, equipment)
- [faq/hardware-compatibility.md](../faq/hardware-compatibility.md) (equipment)
- [faq/usb-and-devices.md](../faq/usb-and-devices.md) (equipment)
- [features/overview.md](../features/overview.md) (performance)
- [guides/introduction.md](introduction.md) (performance)
- [guides/lighting-mode.md](lighting-mode.md) (midi)
