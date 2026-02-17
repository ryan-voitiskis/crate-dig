---
id: lighting-mode
title: "Lighting Mode Operation Guide"
type: guide
source:
  file: "rekordbox7.0.7_lighting_operation_guide_EN.pdf"
  pages: "1-71"
  version: "7.0.7"
topics: [effects, interface, lighting, midi, phrase, preferences]
modes: [lighting]
confidence: pending
last_verified: null
transcribed_by: agent
verified_by: null
---

# LIGHTING mode Operation Guide

## Contents

| Section | Title | Page |
|---------|-------|------|
| 1 | Before Start | 3 |
| 1.1 | Before getting started | 3 |
| 1.2 | System requirements | 3 |
| 1.3 | Overview of LIGHTING mode | 3 |
| 2 | Terms | 6 |
| 3 | Steps to easily control lighting | 9 |
| 3.1 | Connecting equipment | 10 |
| 3.2 | Installing the latest version of rekordbox | 10 |
| 3.3 | Downloading data used in LIGHTING mode | 11 |
| 3.4 | Assigning your fixtures (LIGHTING mode) | 11 |
| 3.4.1 | Moving Head initial settings | 14 |
| 3.5 | Phrase analysis | 16 |
| 3.6 | Playing phrase-analyzed tracks (PERFORMANCE mode) | 16 |
| 4 | Using rekordbox with a controller | 16 |
| 4.1 | Controlling lighting effects | 16 |
| 4.2 | Controlling decks | 19 |
| 5 | Customizing lighting control | 20 |
| 5.1 | Changing relation between scenes and phrases per track (PERFORMANCE mode) | 20 |
| 5.2 | Changing relation between phrases and scenes (LIGHTING mode) | 20 |
| 5.3 | Changing relation between scenes and phrases per track (LIGHTING mode) | 21 |
| 5.4 | Creating a new scene (LIGHTING mode) | 22 |
| 5.5 | Editing scenes (LIGHTING mode) | 24 |
| 5.6 | Deleting scenes (LIGHTING mode) | 26 |
| 5.7 | Resetting scenes (LIGHTING mode) | 27 |
| 5.8 | Copying and creating new scenes (LIGHTING mode) | 28 |
| 5.9 | Editing scenes per track (LIGHTING mode) | 28 |
| 5.10 | Details of editing in LIGHTING mode | 30 |
| 5.10.1 | Setting anchor | 30 |
| 5.10.2 | Setting color | 32 |
| 5.10.3 | Setting color transition | 36 |
| 5.10.4 | Setting strobe | 38 |
| 5.10.5 | Setting moving head pattern | 40 |
| 5.10.6 | Setting rotation | 43 |
| 6 | Part names | 48 |
| 6.1 | Global section | 48 |
| 6.2 | Fixture Library screen | 48 |
| 6.3 | Macro Mapping screen | 49 |
| 6.4 | Scene editor screen | 51 |
| 6.4.1 | Keyboard shortcuts (Scene editor) | 53 |
| 6.5 | Macro editor screen | 54 |
| 6.5.1 | Keyboard shortcuts (Macro editor) | 57 |
| 6.6 | Exporting and importing lighting settings | 58 |
| 7 | Preferences | 59 |
| 7.1 | PERFORMANCE mode | 59 |
| 7.2 | LIGHTING mode | 61 |
| 8 | Controlling lighting effects in PERFORMANCE mode | 62 |
| 9 | Using Ambient mode | 64 |
| 10 | Using DMX Direct Control | 65 |
| 11 | Using PRO DJ LINK Lighting | 68 |
| 12 | Online support site | 70 |

# 1 Before Start

## 1.1 Before getting started

This Operation Guide explains rekordbox LIGHTING mode and related features. For instructions on rekordbox in general, go to rekordbox.com [Support] > [manual] and see the rekordbox Instruction Manual.

## 1.2 System requirements

Regarding the supported OS and required system (minimum operating environment), please refer to rekordbox.com [Support] > [System requirements].

Also, we have not tested all lighting fixtures, so there is no guarantee that all fixtures can be controlled from rekordbox.

## 1.3 Overview of LIGHTING mode

The Lighting mode is a mode to assign fixtures and edit scenes. Select Lighting mode from the global section.

[Screenshot: Global section showing rekordbox icon and LIGHTING mode selector button]

The contents of the Lighting mode are as follows.

- Fixture Library screen:

You can assign and select settings for your fixtures.

[Screenshot: FIXTURE LIBRARY screen showing venue tabs, DMX address grid with fixture assignments, fixture list with Par Lights and Moving Heads assigned to color-coded categories]

- Macro Mapping screen:

You can change relations between scenes and phrases.

[Screenshot: MACRO MAPPING screen showing mood/bank selection at top (HIGH, MID, LOW, COOL, NATURAL, HOT, SUBTLE, WARM, VIVID, CLUB 1, CLUB 2), scene assignment area with phrase thumbnails (INTRO 1, INTRO 2, UP 1, UP 2, UP 3, CHORUS 1, CHORUS 2, DOWN, OUTRO 1, OUTRO 2), and scene display area below with filter dropdowns and scene thumbnails]

- Macro Editor screen:

You can quickly edit scenes to match the flow of the track.

[Screenshot: MACRO EDITOR screen showing fixture channel editors with luminance curves, track waveform with phrase markers, browser/scene/grid tabs at bottom, venue and BPM controls]

Please see the overview of LIGHTING mode at rekordbox.com.

\* Select PERFORMANCE mode to perform scenes in sync with tracks. For details, please see *3. Steps to easily control lighting*.

[Screenshot: PERFORMANCE mode deck showing "About Time (Original Mix)" by Martin Ikin & Low Step, with phrase bar, hot cue pads, BPM display at 124.00, and key display F#m]

# 2 Terms

The major terms used in this operation guide are explained as below.

DMX/DMX512:

> DMX512 is a communication protocol used for controlling stage lighting equipment (fixtures).

DMX interface:

> This is the hardware for converting the lighting control signals sent from the PC/Mac into the DMX 512 signals and transmitting it to the connected fixtures.

Address:

> It is a number assigned to control multiple fixtures individually on the DMX.

Universe:

> It is a unit of DMX addresses. 1 universe means 512 addresses.

Fixture Library:

> The fixture library is the list of fixture profiles including manufacturers, models, DMX channels, categories, etc. provided by AtlaBase Ltd.

Venue:

> The venue allows you to separately save DMX channel assignment information (1 universe) of multiple fixtures according to your needs.

Lighting package:

> The lighting package is a set of 16 fixtures which has a predetermined positional relationship as shown below.

[Screenshot: Diagram showing lighting package layout with 16 numbered fixtures arranged on a stage setup - Par Lights 1-4 on truss, Bar Lights 1-6 on various positions, Moving Heads 1-4 on truss corners, Strobe, and Mirrorball Spot]

| Number | Fixture | Number | Fixture |
|--------|---------|--------|---------|
| 1 | Par Light 1 | 9 | Bar Light 5 |
| 2 | Par Light 2 | 10 | Bar Light 6 |
| 3 | Par Light 3 | 11 | Moving Head 1 |
| 4 | Par Light 4 | 12 | Moving Head 2 |
| 5 | Bar Light 1 | 13 | Moving Head 3 |
| 6 | Bar Light 2 | 14 | Moving Head 4 |
| 7 | Bar Light 3 | 15 | Strobe |
| 8 | Bar Light 4 | 16 | Mirrorball Spot |

Lighting packages consist of the following categories of fixtures.

- Par Light
- Par Light (Simple)
- Bar Light
- Bar Light (Simple)
- Moving Head
- Moving Head (Simple)
- Strobe
- Mirrorball Spot

\* Scenes pre-set in rekordbox are created with this lighting package as the target. By making the same configuration, you can play your scene close to the original scene.

Scene:

> Scene means lighting effects composed of multiple different types of fixtures. Up to 16 kinds of lighting effects information can be saved per one scene.

Macro:

> The macro means relation of multiple scenes.

Bank:

> The bank is a variation of scenes consisting of COOL/NATURAL/HOT/SUBTLE/WARM/VIVID/CLUB 1/CLUB 2.

Mood:

> The mood is a classification of music based on audio information including tempo, rhythm, kick drum and sound density. It is classified as HIGH /MID/LOW.

Phrase analysis:

> Phrase analysis is to analyze the structure of tracks and define each composition as a phrase. Phrases are categorized as below: Intro/Up/Down/Chorus/Bridge/Verse/Outro.

Phrase:

> Phrases are composition of music such as: Intro/Up/Down/Chorus/Bridge/Verse/Outro.

[Screenshot: Diagram showing the relationship between Track, Phrase, Mood, Macro, Bank, Scene, and Fixture Package. Track contains Phrases (Intro, Up, Chorus, Down, Outro) with Mood levels (High, Mid, Low). Each phrase maps through Macro to Scene banks (COOL, NATURAL, HOT, SUBTLE, WARM, VIVID, CLUB1, CLUB2) which control the Fixture Package (Par Light 1-4, Bar Light 1-6, Moving Head 1-4, Strobe, Mirrorball Spot)]

# 3 Steps to easily control lighting

Below is the procedure for executing the scene in sync with tracks.

3.1 Connecting equipment (page 10)

3.2 Installing rekordbox 5.2.0 or later (page 10)

3.3 Downloading data used in LIGHTING mode (page 11)

3.4 Assigning your fixtures (LIGHTING mode) (page 11)

3.5 Phrase analysis (page 16)

3.6 Playing phrase-analyzed tracks (PERFORMANCE mode) (page 16)

## 3.1 Connecting equipment

Connect devices according to the connection diagram shown below

[Screenshot: Connection diagram showing [1] PC/Mac with rekordbox connected via [2] USB cable to [3] Pioneer DJ DMX interface (RB-DMX1), connected via [4] DMX cable to [5] lighting fixtures]

- [1] PC/Mac where rekordbox is installed
- [2] USB cable
- [3] rekordbox supported DMX interface

\*See rekordbox.com for the DMX interface compatible with rekordbox.

\*Please note that you can ONLY use the DMX interface compatible with rekordbox.

- [4] DMX cable

\*Please note that the type of DMX cable varies depending on your DMX interface. (XLR 3-pin / XLR 5-pin).

- [5] Your fixtures

> You may be unable to properly operate the device depending on USB connection status. This could be due to band width of USB port and USB hub on your PC/Mac. Changing USB hub or port could solve the problem.

## 3.2 Installing the latest version of rekordbox

This manual explains the Lighting mode for rekordbox ver.6.1.0 or later. Please update rekordbox to the latest version if you haven't already.

## 3.3 Downloading data used in LIGHTING mode

To use LIGHTING mode, you need to download fixture library and scene data.

When you select LIGHTING mode in the global section, the following dialog appears. Follow the instructions to download the necessary data.

[Screenshot: Dialog box reading "No data for Fixture Library/Scene. Do you want to download it? (File size: 23.8 MB)" with Ok and Cancel buttons]

\* Your PC/Mac must be connected to the internet to download data.

\* Open PERFORMANCE mode > [Preferences] > [Extensions] category > [Lighting] tab and check [Enable Lighting function].

## 3.4 Assigning your fixtures (LIGHTING mode)

1. Select LIGHTING Mode in the global section and click [Fixture Library] to show LIGHTING mode FIXTURE LIBRARY screen.

[Screenshot: FIXTURE LIBRARY screen with numbered areas: [2] lighting device assign area showing DMX address grid, [3] category selection area on right with Moving Head assignments, [9] fixture display area at bottom showing fixture list]

1. Select a fixture from the list displayed in the fixture display area [9] and drag & drop it to a DMX address in the lighting device assign area [2].

\*For the DMX address, please set the same address as the setting value of each fixture.

The selected fixture is registered in rekordbox, and the DMX address is fixed at the same time.

\* To change the address, click with the mouse and drag it to an address you would like to fix.

\* To delete assignment, click it by a mouse and click X.

2. In the category selection area [3], select a category from the drop-down menu.

[Screenshot: Category selection dropdown showing fixture assignments (GigBAR 2 PAR1 #1 1-5, GigBAR 2 PAR2 #1 6-10, etc.) with category dropdown expanded showing: Par Light 1, Par Light 2, Par Light 3, Par Light 4, Bar Light 1-6, Moving Head 1-4, Strobe, Mirrorball Spot, Effect 1, Effect 2, Laser, Par Light 1 (Simple), Par Light 2 (Simple), Bar Light 1 (Simple), Bar Light 2 (Simple), Moving Head 1 (Simple), Moving Head 2 (Simple), No Assign]

The category of the assigned fixture is fixed.

\*If you assign no category, select [No Assign].

\*Simple category

You can use Simply category for Par Light, Bar Light and Moving Head.

When you select the Simple category, the pre-set scene is applied which is optimized when there are 2 lighting devices assigned to each category.

### 3.4.1 Moving Head initial settings

Click the menu button to select Moving Head initial settings from the menu.

[Screenshot: Menu showing Moving Head settings options: "Moving Head initial position setting", "Moving Head pan/tilt limit setting", "Moving Head tilt reverse setting" for assigned Moving Head fixtures]

\*When the Moving Head is connected, you can check the initial position.

Click [Moving Head initial position setting] in the menu to open the dialog box for Moving Head initial position setting.

[Screenshot: Dialog box "Set an initial position for the Moving Head." with (0,0) at top-left, PAN: 127, TILT: 127 input fields, crosshair position indicator, (255,255) at bottom-right, OK and Cancel buttons]

Click [Moving Head pan/tilt limit setting] in the menu to open the dialog box for Moving Head pan/tilt limit setting.

[Screenshot: Dialog box "Set a range for the Moving Head." with (0,0) at top-left, MIN section (PAN: 64, TILT: 64), MAX section (PAN: 234, TILT: 234), range indicator on grid, (255,255) at bottom-right, OK and Cancel buttons]

Click [Moving Head tilt reverse setting] to open the dialog box to set a direction for the Moving Head.

[Screenshot: Dialog box "Set a direction for the Moving Head." with "reverse TILT" checkbox, OK and Cancel buttons]

## 3.5 Phrase analysis

For details on how to analyze phrases of tracks, refer to rekordbox.com > [Support] > [manual] > [rekordbox Instruction Manual].

## 3.6 Playing phrase-analyzed tracks (PERFORMANCE mode)

[Screenshot: PERFORMANCE mode deck showing "About Time (Original Mix)" by Martin Ikin & Low Step with phrase bar, hot cue pads, and BPM display at 124.00]

Load and play a phrase-analyzed track on a deck in PERFORMANCE mode.

# 4 Using rekordbox with a controller

## 4.1 Controlling lighting effects

You can assign the followings to performance pads to control them using your DJ controller. Press a pad to enable turn on the feature. Press again to reset.

[Screenshot: Pioneer DJ DDJ-XP1 controller with illuminated performance pads]

- AUTO MOOD: Resets the current mood to automatically select it.
- HIGH: Switches the current mood to HIGH.
- MID: Switches the current mood to MID.
- LOW: Switches the current mood to LOW.
- AUTO BANK: Resets the current bank to automatically select it.
- COOL: Switches the current bank to COOL.
- NATURAL: Switches the current bank to NATURAL.
- HOT: Switches the current bank to HOT.
- SUBTLE: Switches the current bank to SUBTLE.
- WARM: Switches the current bank to WARM.
- VIVID: Switches the current bank to VIVID.
- CLUB 1: Switches the current bank to CLUB 1.
- CLUB 2: Switches the current bank to CLUB 2.
- AUTO COLOR: Resets the current color to automatically select it.
- RED: Switches the current color to RED.
- GREEN: Switches the current color to GREEN.
- BLUE: Switches the current color to BLUE.
- MAGENTA: Switches the current color to MAGENTA.
- YELLOW: Switches the current color to YELLOW.
- CYAN: Switches the current color to CYAN.
- WHITE: Switches the current color to WHITE.
- USERCOLOR: Switches the current color to USER Color that can be selectable by the user.
- BLACK OUT: Switches Turn off all the lighting.
- AUTO STROBE: Resets the strobe effect to automatically select it.
- STROBE (FAST): Blinks all the lighting in high speed.
- STROBE (MIDDLE): Blinks all the lighting in mid speed.
- STROBE (SLOW): Blinks all the lighting in low speed.
- STROBE OFF: Turns off only Strobe effect. (\*Others unchanged.)
- INTERLUDE 1: Starts Ambient mode with [INTERLUDE 1].
- INTERLUDE 2: Starts Ambient mode with [INTERLUDE 2].
- INTERLUDE 3: Starts Ambient mode with [INTERLUDE 3].
- INTERLUDE 4: Starts Ambient mode with [INTERLUDE 4].
- INTERLUDE 5: Starts Ambient mode with [INTERLUDE 5].
- INTERLUDE 6: Starts Ambient mode with [INTERLUDE 6].
- AMBIENT MODE OFF: Stops Ambient mode.
- DMX DIRECT CONTROL 1: Turns on/off DMX Direct Control button 1.
- DMX DIRECT CONTROL 2: Turns on/off DMX Direct Control button 2.
- DMX DIRECT CONTROL 3: Turns on/off DMX Direct Control button 3.
- DMX DIRECT CONTROL 4: Turns on/off DMX Direct Control button 4.
- DMX DIRECT CONTROL 5: Turns on/off DMX Direct Control button 5.
- DMX DIRECT CONTROL 6: Turns on/off DMX Direct Control button 6.
- DMX DIRECT CONTROL 7: Turns on/off DMX Direct Control button 7.
- DMX DIRECT CONTROL 8: Turns on/off DMX Direct Control button 8.
- DMX DIRECT CONTROL 9: Turns on/off DMX Direct Control button 9.
- DECK SEL/DECK1: Selects DECK1 to apply lighting effects.
- DECK SEL/DECK2: Selects DECK2 to apply lighting effects.
- DECK SEL/DECK3: Selects DECK3 to apply lighting effects.
- DECK SEL/DECK4: Selects DECK4 to apply lighting effects.
- DECK SEL/DECK5: Selects DECK5 to apply lighting effects.
- DECK SEL/DECK6: Selects DECK6 to apply lighting effects.
- DECK SEL/AUTO: Automatically selects the deck to apply lighting effects.
- MASTER DIMMER: Adjusts the brightness of fixtures.

\*Available only in PERFORMANCE mode or on the PRO DJ LINK Lighting screen.

\*DECK SEL/DECK5, DECK SEL/DECK6 are only available on the PRO DJ LINK Lighting screen.

\*MASTER DIMMER is only available via MIDI.

\*For details on the USER Color setting, see "7 Preferences".

## 4.2 Controlling decks

On the Macro editor screen, you can control decks using a DJ controller or MIDI controller. For DJ units supporting this function, visit rekordbox.com > [Support] > [Compatible DJ units].

\*For compatible DJ equipment, see > rekordbox.com > [Support] > [Compatible DJ units].

\*For MIDI LEARN settings, see rekordbox.com > [Support] > [Manual] > [MIDI LEARN Operation Guide].

\*For Pad Editor operation, SEE rekordbox.com> [Support] > [Manual] > [Pad Editor Operation Guide].

> You may be unable to properly operate the device depending on USB connection status. This could be due to band width of USB port and USB hub on your PC/Mac. Changing USB hub or port could solve the problem.

# 5 Customizing lighting control

## 5.1 Changing relation between scenes and phrases per track (PERFORMANCE mode)

1. Select PERFORMANCE mode in the global section.

2. Load a phrase-analyzed track on Deck1 or Deck2.

Thumbnail of phrases and scenes are displayed on the deck.

3. Right-click a thumbnail. The pop-up appears as below.

[Screenshot: PERFORMANCE mode deck showing "About Time (Original Mix)" with phrase thumbnails and a pop-up scene selection overlay with multiple scene options]

4. Click a scene you would like to select in the pop-up. The relation has been changed.

\* This change will not be saved. To save the change, see 5.3 Changing relation between scenes and phrases per track (LIGHTING mode).

## 5.2 Changing relation between phrases and scenes (LIGHTING mode)

1. Click [Macro Mapping] to display Macro Mapping screen and select the mood and bank of the phrase you want to change relation with the scene in mood/bank selection area [1].

Scenes displayed in scene assignment area [2] will be changed.

[Screenshot: MACRO MAPPING screen with numbered areas: [1] mood/bank selection area at top, [2] scene assignment area with phrase thumbnails, [3] undo/redo buttons, [4] reset button, [5] scene filter dropdowns, [6] scene display area with scene thumbnails]

2. From scene display area [6], drop & drop a thumbnail of a scene to scene assignment area [2] to change the relation.

## 5.3 Changing relation between scenes and phrases per track (LIGHTING mode)

1. Click [MACRO EDITOR] to display Macro Editor screen and select the browser tub to show the browser.

[Screenshot: MACRO EDITOR screen showing fixture editors at top, track waveform with phrase markers, and browser panel at bottom with Collection tree and track list]

2. From the browser, select a track you want to change the relation of the scene and load it to the track display area.

The selected track will be loaded to the track display area.

3. Select the scene tab.

The scene tab will be displayed.

[Screenshot: MACRO EDITOR screen with save button highlighted, fixture editors showing luminance curves, track display area showing "Around Summer" by Eric Shea, and scene tab at bottom displaying scene thumbnails with USER'S SCENE checkbox]

4. Drag and drop the thumbnail of a scene onto the scene you want to change the relation. The relation of the selected scene has been changed.

5. Click the save button in the upper part of the macro editor screen to save the relation of scenes per track.

## 5.4 Creating a new scene (LIGHTING mode)

1. Click [Macro Mapping] to display macro mapping screen.

Click [+] button on the bottom of the scene thumbnail in the scene display area.

[Screenshot: MACRO MAPPING screen showing scene display area with scene thumbnails and a [+] button at the end for adding new scenes]

From the dialog below, you can select the number of bars and file name for a new scene.

[Screenshot: Dialog box with "Bars" section showing options 4, 8, 16, 32, 64, a "Title Name" text field, OK button, and Cancel button]

2. Select the number of the bars and file name and click OK to show a screen to create a new scene.

[Screenshot: Scene editor screen showing empty fixture channels (Par Light 1-4, Bar Light 1) with grid lines, playback controls at bottom with BPM display showing 120.00]

\*For details on the editing work, please see 5.9 Details of editing in LIGHTING mode and 6.4 Scene editor screen.

## 5.5 Editing scenes (LIGHTING mode)

1. Click [Macro Mapping] to display macro mapping screen and hover the mouse to a scene in the scene display area and click the edit button.

[Screenshot: MACRO MAPPING screen showing scene thumbnails with an edit button highlighted on one scene labeled "HIGH INTRO1 BL... 8"]

The scene edit screen appears.

[Screenshot: Scene editor screen showing Par Light 1-4 and Bar Light 1 channels with luminance curves (white lines) and color bars (green, red), playback controls at bottom with BPM 120.00]

\* For details on the editing work, please see 5.9 Details of editing in LIGHTING mode and 6.4 Scene editor screen.

## 5.6 Deleting scenes (LIGHTING mode)

1. Click [Macro Mapping] to display macro mapping screen and right-click a thumbnail of a scene you want to delete in the scene display area.

\*You can only delete scenes you created.

Below drop-down menu opens.

[Screenshot: MACRO MAPPING screen showing scene display area with right-click context menu showing DELETE, COPY, and ColorChecker options]

2. Select [DELETE] from the menu.

The below dialog appears.

[Screenshot: Dialog box reading "Are you sure you want to delete the selected Scene?" with OK and Cancel buttons]

3. Click [OK].

The selected scene has been deleted.

## 5.7 Resetting scenes (LIGHTING mode)

1. Click [Macro Mapping] to show macro mapping screen and right-click a scene you want to reset in the scene display area.

\*The scenes you created cannot be reset.

Below menu opens.

[Screenshot: MACRO MAPPING screen showing scene display area with right-click context menu showing RESET and COPY options on a scene labeled "HIGH INTRO1 HOT 1"]

2. Select [RESET] from the menu.

The below dialog opens.

[Screenshot: Dialog box reading "Are you sure you want to reset the selected Scene?" with OK and Cancel buttons]

3. Click [OK].

Selected scene has been reset.

## 5.8 Copying and creating new scenes (LIGHTING mode)

1. Click [Macro Mapping] to display the Macro Mapping screen. Right-click the thumbnail of the Scene you want to copy in the scene display area with your mouse to display the context menu.

[Screenshot: MACRO MAPPING screen showing scene display area with right-click context menu showing DELETE, COPY, and ColorChecker options]

2. Select [Copy] in the context menu. The Copy scene dialog box appears.

[Screenshot: Copy scene dialog box with text input field, OK and Cancel buttons]

3. Enter a new scene name and click [OK]. Copy the selected scene and creates a new one.

## 5.9 Editing scenes per track (LIGHTING mode)

1. Click [MACRO EDITOR] to display macro editor screen and select the browser tab to display the browser.

[Screenshot: MACRO EDITOR screen showing track waveform area, browser panel with Collection tree and track list]

2. Select the track you want to edit the scene from the browser and load it in the track display area.

The track will be loaded to the track display area. When you close the tab, the scene appears in the edit area.

\* For details on the editing work, please see 5.9 Details of editing in LIGHTING mode and 6.4 Scene editor screen.

## 5.10 Details of editing in LIGHTING mode

### 5.10.1 Setting anchor

1. Click a point on the white line showing luminance.

[Screenshot: Par Light 1 editor showing empty luminance line with COLOR, STROBE, PATTERN rows below]

The anchor is set on the white line.

[Screenshot: Par Light 1 editor showing a single anchor point on the luminance line highlighted with yellow square]

\*For the anchor, the lowest position means all turned off and the highest position means all lit.

\* You can click the anchor to move up, down, left and right.

- Copy an anchor

1. Select the area of the anchor you want to copy with your mouse and right-click to show the drop-down menu.

[Screenshot: Par Light 1 editor showing selected area with dashed outline on luminance line, right-click context menu showing Color, Color Transition, Strobe, Delete, Copy]

2. Select [Copy].

The anchor has been copied.

- Paste an anchor

3. Click at the position you want to paste and right click on the same position.

[Screenshot: Par Light 1 editor showing luminance line with anchors and cursor position with right-click context menu]

The drop-down menu opens.

[Screenshot: Par Light 1 editor showing Paste option in context menu]

4. Select [Paste].

The anchor has been pasted.

[Screenshot: Par Light 1 editor showing pasted anchors highlighted with yellow rectangle]

- Move an anchor

1. Keep clicking the anchor and move it.

[Screenshot: Par Light 1 editor showing an anchor being dragged with dashed arrow indicating movement direction]

The anchor has been moved.

\*The anchor cannot be moved to the left of the leftmost anchor

It also cannot be moved to the right of the rightmost anchor.

In addition, you cannot move multiple anchors at the same time.

- Delete an anchor

1. Select the anchor you want to delete with the mouse and right click to show the drop-down menu.

[Screenshot: Par Light 1 editor showing right-click context menu on anchor with Color, Color Transition, Strobe, Delete, Copy options with Delete highlighted]

2. Select [Delete].

The anchor has been deleted.

[Screenshot: Par Light 1 editor showing luminance line with anchor removed, dashed outline showing deleted area]

### 5.10.2 Setting color

1. Select the area you want to set the color with the mouse and right click or click the color button to show the drop-down menu.

[Screenshot: Par Light 1 editor showing selected area on COLOR row with dashed outline, right-click context menu showing Color (highlighted), Color Transition, Delete, Copy]

2. Select [Color].

The dialog for color setting appears.

[Screenshot: Color picker dialog with color spectrum, hue slider, hex values (ED, 07, 5B), Ok and Cancel buttons]

3. Select a color in the dialog and click OK.

The color has been set at the selected area.

[Screenshot: Par Light 1 editor showing pink/magenta color bar set on the COLOR row]

- Copy color

1. Select the area you want to copy with the mouse and right-click to display the drop-down menu.

[Screenshot: Par Light 1 editor showing selected color area on COLOR row with right-click context menu showing Color, Color Transition, Delete, Copy (highlighted)]

2. Select [Copy].

The selected area has been copied.

- Paste color

1. Click at the position you want to paste and right click on the same position.

[Screenshot: Par Light 1 editor showing COLOR row with existing pink color and cursor at paste position]

The drop-down menu appears.

[Screenshot: Par Light 1 editor showing Paste option in context menu]

2. Click [Paste].

The color has been pasted.

[Screenshot: Par Light 1 editor showing original pink color and pasted pink color highlighted with yellow rectangle]

- Adjust color length

1. Move the mouse to the right or left edge of the color you want to adjust its length.

The resize icon is displayed at the edge of the right side or left side of the color.

[Screenshot: Par Light 1 editor showing pink color bar on COLOR row with resize arrows at the edge]

2. Click and hold the resize icon to move left and right to change the length.

[Screenshot: Par Light 1 editor showing extended pink color bar on COLOR row]

\* You cannot overlap right or left color area.

- Delete color

1. Right-click the color you want to delete to show the drop-down menu.

[Screenshot: Par Light 1 editor showing right-click context menu on color with Delete (highlighted) and Edit options]

2. Click [Delete].

The selected color has been deleted.

[Screenshot: Par Light 1 editor showing empty COLOR row after deletion]

- Edit color

1. Right-click the color you want to edit to show the drop-down menu.

[Screenshot: Par Light 1 editor showing right-click context menu on color with Delete and Edit (highlighted) options]

2. Click [Edit].

The dialog for color setting appears.

[Screenshot: Color picker dialog with color spectrum, hue slider, hex values (09, 07, ED), Ok and Cancel buttons]

3. Select a color in the dialog above and click [OK] to edit.

[Screenshot: Par Light 1 editor showing blue color bar on COLOR row after editing]

### 5.10.3 Setting color transition

1. Select the beat in the area you want to set color transition with the mouse and right click or click the color transition button to show the drop-down menu.

[Screenshot: Par Light 1 editor showing selected area on COLOR row with dashed outline, right-click context menu showing Color, Color Transition (highlighted), Delete, Copy]

2. Click [Color Transition].

The dialog for color transition setting appears.

[Screenshot: Color Transition dialog with color spectrum, hue slider, hex values (FF, 00, 00), start point color indicator at bottom-left, Ok and Cancel buttons]

3. Set the color for the start point in the dialog above.

The color for the start point of the selected area has been set.

4. Set the color for the end point in the same way.

[Screenshot: Color Transition dialog with color spectrum showing cyan/teal selection, hex values (00, FF, FF), end point color indicator at bottom-right, gradient preview bar showing red-to-cyan transition, Ok and Cancel buttons]

5. Click OK.

The color transition has been set at the selected area.

[Screenshot: Par Light 1 editor showing red-to-cyan color transition gradient on COLOR row]

\*Copying, adjusting length, and deleting are the same as described in color settings. Please refer to *5.10.2 Setting color*.

- Edit color transition

1. Right-click the color transition you want to edit to show the drop-down menu.

[Screenshot: Par Light 1 editor showing right-click context menu on color transition with Delete and Edit (highlighted) options]

2. Select [Edit].

The dialog for color transition settings appears.

[Screenshot: Color Transition dialog with color spectrum showing yellow-green selection, hex values (D2, FF, 00), start point color indicator, Ok and Cancel buttons]

3. Set the color for the start point in the dialog above.

The color for the start point of the selected area has been set.

4. Set the color for the end point in the same way.

[Screenshot: Color Transition dialog with color spectrum, hex values (FF, 00, 56), end point color indicator, gradient preview bar, Ok and Cancel buttons]

The color for the end point of the selected area has been set.

5. Click OK.

The selected color transition has been edited.

[Screenshot: Par Light 1 editor showing edited yellow-green-to-pink color transition on COLOR row]

### 5.10.4 Setting strobe

1. Select the area you want to set the strobe with the mouse and right-click or click the strobe button to show the drop-down menu.

[Screenshot: Par Light 1 editor showing selected area on STROBE row with dashed outline, right-click context menu showing Strobe (highlighted), Delete, Copy]

2. Click [Strobe] to show the dialog for strobe settings.

[Screenshot: Strobe dialog showing graph with start point (bottom-left, ~10%) and end point (top-right, ~90%) connected by a line, Ok and Cancel buttons]

3. In the dialog above, set the strobe amounts for the start point (left side) and end point (right side), and click OK.

The strobes are set to the selected area.

[Screenshot: Par Light 1 editor showing strobe pattern (dashed line) on STROBE row]

\* Copying, adjusting length, and deleting are the same as described in color settings. Please refer to *5.10.2 Setting color*.

- Edit strobes

1. Click the strobe you want to edit with the mouse and right-click to show the drop-down menu.

[Screenshot: Par Light 1 editor showing right-click context menu on strobe with Delete and Edit (highlighted) options]

2. Click [Edit].

[Screenshot: Strobe dialog showing graph with start point (~40%) and end point (~60%) connected by a line, Ok and Cancel buttons]

The dialog for strobe settings appears.

3. In the dialog above, set the strobe amounts for the start point (left side) and end point (right side), and click OK.

The selected strobe has been edited.

[Screenshot: Par Light 1 editor showing edited strobe pattern on STROBE row]

### 5.10.5 Setting moving head pattern

1. Select the area you want to set the pattern of the moving head with the mouse and right click or click the pattern button to show the drop-down menu.

[Screenshot: Moving Head 1 editor showing selected area on PATTERN row with dashed outline, right-click context menu showing Pattern (highlighted), Delete, Copy]

2. Click [Pattern].

The dialog for moving head pattern settings appears.

[Screenshot: Moving Head pattern setting dialog with Fixture list (Moving Head 1), pattern preview showing circle path, Parameters section (Width: 127, Height: 127, X Offset: 127, Y Offset: 127, Rotation: 0, Start Offset: 0, Cycle (msec): 20000, X Frequency: 2, Y Frequency: 3, X Phase: 90, Y Phase: 0), Direction (Forward/Backward), Type (Loop/Ping pong/Single shot), Pattern list (Circle, Eight, Line, Line2, Diamond, Square, SquareChoppy, Leaf, Lissajous), Play button, OK and Cancel buttons]

- Pattern: Select the moving head pattern from below 9 patterns.

[Screenshot: Circle pattern preview] : Circle

[Screenshot: Eight pattern preview] : Eight

[Screenshot: Line pattern preview] : Line

[Screenshot: Line2 pattern preview] : Line2

[Screenshot: Diamond pattern preview] : Diamond

[Screenshot: Square pattern preview] : Square

[Screenshot: SquareChoppy pattern preview] : SquareChoppy

[Screenshot: Leaf pattern preview] : Leaf

[Screenshot: Lissajous pattern preview] : Lissajous

Parameters

- Width: Adjusts the width of the selected pattern.
- Hight: Adjusts the height of the selected pattern.
- X Offset: Adjusts the panning position of the selected pattern.
- Y Offset: Adjusts the tilting position of the selected pattern.
- Rotation: Adjusts the rotational position of the selected pattern.
- Start Offset: Adjusts the start position of the selected pattern.
- Cycle (msec): Adjusts the cycle of the selected pattern.
- X Frequency: Adjusts the frequency in a panning direction of the Lissajous.
- Y Frequency: Adjusts the frequency in a tilting direction of the Lissajous.
- X Phase: Adjusts the phase in a panning direction of Lissajous.
- Y Phase: Adjusts the phase in a tilting direction of Lissajous.
- Forward: Moves the selected pattern forward.
- Backward: Moves the selected pattern backward.
- Loop: Repeats the selected pattern in the same way.
- Ping pong: Repeats the selected pattern while reversing direction every time.
- Single shot: Moves the selected pattern only one time.

- Play button: Starts moving heads to check the selected pattern with the actual move.

3. Select a pattern and parameters in the dialog above and click OK.

The moving head pattern has been set at the selected area.

[Screenshot: Moving Head 1 editor showing pattern indicator (dashed line between two circles) on PATTERN row]

\* Copying, adjusting length, and deleting are the same as described in color settings. Please refer to *5.10.2 Setting color*.

- Edit moving head pattern

1. Right-click the moving head pattern you want to edit to show the drop-down menu.

[Screenshot: Moving Head 1 editor showing right-click context menu on pattern with Delete and Edit (highlighted) options]

2. Click [edit].

The dialog for moving head pattern settings appears.

[Screenshot: Moving Head pattern setting dialog with same layout as before, showing Circle pattern selected]

3. In the dialog above, select a pattern and parameters and click OK.

The selected moving head pattern has been edited.

[Screenshot: Moving Head 1 editor showing edited pattern indicator on PATTERN row]

### 5.10.6 Setting rotation

1. Select the area you want to set the rotation with the mouse and right-click or click the rotation button to show the drop-down menu.

[Screenshot: Effect 1 editor showing selected area on ROTATE row with dashed outline, right-click context menu showing Rotate (highlighted), Delete, Copy]

2. Click [Rotate] to show the dialog for the rotation settings.

[Screenshot: Rotate dialog showing graph with start point (bottom-left, ~10%) and end point (top-right, ~90%) connected by a line, Ok and Cancel buttons]

3. In the dialog above, select the start point (left side) and end point (right side) to set how much you want to rotate, and click OK.

The rotation is set to the selected area.

[Screenshot: Effect 1 editor showing rotation indicator (dashed line) on ROTATE row]

\* You can copy, adjust length, and delete the rotation settings in the same way as described in the color settings. Please refer to *5.10.2 Setting color*.

- Edit rotation

4. Click the rotate you want to edit with the mouse and right-click to show the dropdown menu.

[Screenshot: Effect 1 editor showing right-click context menu on rotation with Delete and Edit (highlighted) options]

5. Click [Edit].

[Screenshot: Rotate dialog showing graph with start point (~10%) and end point (~90%) connected by an ascending line, Ok and Cancel buttons]

The dialog for rotate settings is shown.

6. In the dialog above, select the start point (left side) and end point (right side) to set how much you want to rotate, and click OK.

The selected rotate has been edited.

[Screenshot: Effect 1 editor showing edited rotation indicator on ROTATE row]

### 5.10.7 Setting Gobo

1. Select the area you want to set Gobo with the mouse, and right-click or click the icon G button. The context menu will appear.

[Screenshot: Moving Head 1 editor showing GOBO row with selected area, Moving Head 2 below, right-click context menu showing Gobo (highlighted), Delete, Copy]

2. Select [Gobo]. A dialog box for Gobo settings will appear.

[Screenshot: Gobo dialog box with "Gobo No." spinner set to 1, Ok, Cancel, and Apply buttons]

3. In the above dialog, set the Gobo number, then click OK.

The Gobo will be set in the selected area.

[Screenshot: Moving Head 1 editor showing Gobo indicator (line between two circles) on GOBO row]

\* You can copy, adjust length, and delete the gobo settings in the same way as described in the color settings. Please refer to *5.10.2 Setting color*.

- Editing Gobo

4. Select the Gobo you want to edit using the mouse, then right-click on it.

The context menu will appear.

[Screenshot: Moving Head 1 editor showing GOBO row with gobo set, Moving Head 2 with right-click context menu showing Delete and Edit (highlighted)]

5. Select [Edit].

[Screenshot: Gobo dialog box with "Gobo No." spinner set to 1, Ok, Cancel, and Apply buttons]

A dialog box for Gobo settings will appear.

6. In the above dialog, set the Gobo number, and click OK.

The contents of the selected Gobo will change.

[Screenshot: Moving Head 1 editor showing updated Gobo indicator on GOBO row]

# 6 Part names

## 6.1 Global section

You can select EXPORT/PERFORMANCE/LIGHTING mode.

## 6.2 Fixture Library screen

This screen is for fixture assignment and settings.

Click [Fixture Library] to show the screen below.

[Screenshot: FIXTURE LIBRARY screen with numbered areas: [1] venue area with tabs, [2] fixture assignment area showing DMX address grid, [3] category selection area with Par Light and Moving Head assignments, [4] fixture library version number display area, [5] fixture manufacturer display area, [6] search window, [7] request button to add fixtures, [8] button to check fixture library update, [9] fixture display area showing fixture details]

[1] Venue area

> You can rename or copy venues in the menu that is displayed when you right-click on the tab. By copying venues, you can take over Moving Head initial settings and DMX Direct Control settings.

> You can delete venues by clicking the [x] button on the tab.

> For details on Moving Head initial settings, see "3.4.1 Moving Head initial settings".

> For details on DMX Direct Control, see "10 Using DMX Direct Control".

> \* You can assign different fixtures per venue.

[2] Fixture assignment area

> You can assign your fixture to an address on DMX.

[3] Category selection area

> You can select the category of fixtures assigned to rekordbox.

[4] Fixture library version number display area

> The version number of the fixture library is displayed with a 4-digit number.

[5] Fixture manufacturer display area

> Fixture manufacturers are displayed in an alphabetical order.

[6] Search Window

> You can search fixtures in the fixture library by manufacturer or fixture name.

[7] Request button to add fixtures to the fixture library

> You can request addition of your fixtures to the fixture library if your fixture in not in the library.

[8] Button to check fixture library update

> You can ask the server if the fixture library is updated.

[9] Fixture display area

> Fixture names are displayed in an alphabetical order.

## 6.3 Macro Mapping screen

You can change relations between scenes and phrases.

Click [Macro Mapping] to display the screen below.

[Screenshot: MACRO MAPPING screen with numbered areas: [1] mood/bank selection at top, [2] scene assignment area with phrase thumbnails, [3] undo/redo buttons, [4] reset button, [5] scene filter dropdowns, [6] scene display area with thumbnails]

[1] Mood/Bank selection area

> You can select mood and bank of scenes displayed in scene assignment area [2].

> Mood: High/Mid/Low

> Bank: COOL/NATURAL/HOT/SUBTLE/WARM/VIVID/CLUB 1/CLUB 2

[2] Scene assignment area

> Scenes related to each phrase are shown in thumbnail format.

> You can change relation between each phrase and scene by dragging & dropping a scene shown in scene display area [6].

[3] Undo/Redo button

> Undo: You can cancel the last action

> Redo: You can do an action again after you've undone the action (reverse the undo).

[4] Reset button

> You can reset all the relations between phrases and scenes.

[5] Scene filter

> You can filter scenes displayed in scene display area [6] by following filters.

> Mood: ALL/HIGH/MID/LOW

> Bank: ALL/COOL/NATURAL/HOT/SUBTLE/WARM/VIVID/CLUB 1/CLUB 2/AMBIENT

> When you check [USER'S SCENE], only scenes created by the user will be displayed.

> \* To check [USER'S SCENE], select [ALL] in mood and bank drop-down menu.

[6] Scene display area

> Scenes are shown in thumbnail format.

## 6.4 Scene editor screen

You can create and edit scenes. Please refer to below sections to show this screen.

5.4 Creating a new scene (LIGHTING mode)

5.5 Edit of scenes (LIGHTING mode)

[Screenshot: Scene editor screen with numbered areas: [1] RETURN button, [2] SAVE button, [3] Undo/Redo button, [4] Collective quantize button, [5]-[10] setting buttons, [11] Anchor quantize button, [12] Fixture information display area, [13] Edit area with luminance curves and color bars, [14]-[15] playback controls, [16] BPM input, [17] zoom controls]

[1] RETURN button

> You can finish editing work and go back to the macro mapping screen.

[2] SAVE button

> You can save the edited scene.

[3] Undo/Redo button

> Undo: You can cancel the last action

> Redo: You can do an action again after you've undone the action (reverse the undo).

[4] Collective quantize button

> You can move existing anchors to the nearest grid position.

> \*It applies only to the anchor selected with the mouse.

[5] Color setting button

> The dialog for color settings is shown.

[6] Color transition setting button

> The dialog for color transition settings is shown.

[7] Strobe setting button

> The dialog for strobe settings is shown.

[8] Moving head setting button

> The dialog for moving head settings is shown.

[9] Rotation setting button

> The dialog for the rotation settings is shown.

[10] Gobo setting button

> A dialog box for Gobo settings is shown.

[11] Anchor quantize button

> When you click this button, anchors are set only on grid position.

[12] Fixture information display area

> The names and addresses of fixtures are displayed. You can display/hide per fixture by clicking the dropdown arrow.

[13] Edit area

> You can edit scenes.

[14] Button to return to the top of the scenes

> The position returns to the top of the scenes.

[15] Play/pause button

> Play scenes with BPM set by [15] BPM input box.

[16] BPM input box

> You can enter BPM value to play scenes

> \* This value is valid only for this scene editor.

[17] Zoom in/out or reset button

> Click the zoom in/out buttons to zoom in/out the edit area.

> Click RST to reset the zoom.

### 6.4.1 Keyboard shortcuts (Scene editor)

- Collapse/Expand all fixtures

[End] key

- Play/Pause

[Space] or [Z] key

- Return to the beginning of the scene

[Home] , [H] , or Shift+[A] key

- 4 beats forward

[->] , [F] , or Ctrl+[T] key

- 4 beats backward

[<-] , [B] , or Ctrl+[R] key

## 6.5 Macro editor screen

You can intuitively edit scenes matching the flow of the music.

Click [MACRO EDITOR] to display the screen.

[Screenshot: MACRO EDITOR screen with numbered areas: [1] venue selection, [2] save button, [3] undo/redo, [4] collective quantize, [5]-[10] setting buttons, [11] anchor quantize, [12] fixture information, [13] edit area with luminance curves, [14] play/pause, [15] return to beginning, [16] zoom, [17] track display area with waveform and phrase markers, [18] bank selection, [19] browser tab, [20] scene tab, [21] grid tab]

[1] Venue selection area

> You can select a venue you want to edit its macro.

[2] Save button

> You can save edited macro per venue.

[3] Undo/Redo button

> Undo: You can cancel the last action

> Redo: You can do an action again after you've undone the action (reverse the undo).

[4] Collective quantize button

> You can move existing anchors to the nearest grid position.

> \*It applies only to the anchor selected with the mouse.

[5] Color setting button

> The dialog for color settings is shown.

[6] Color transition setting button

> The dialog for color transition settings is shown.

[7] Strobe setting button

> The dialog for strobe settings is shown.

[8] Moving head setting button

> The dialog for moving head settings is shown.

[9] Rotation setting button

> The dialog for the rotation settings is shown.

[10] Gobo setting button

> A dialog box for Gobo settings is shown.

[11] Anchor quantize button

> When you click this button, anchors are set only on grid position.

[12] Fixture information display area

> The names and addresses of fixtures are displayed. You can display/hide per fixture by clicking the dropdown arrow.

[13] Edit area

> You can edit macro.

[14] Play/pause button

> Play macro with a track.

[15] Button to return to the beginning of the track

> The position returns to the beginning of the track.

[16] Zoom in/out or reset button

> Click the zoom in/out buttons to zoom in/out the edit area.

> Click RST to reset the zoom.

[17] Track display area

> You can open the browser by clicking browser tab [18] and load a track.

[18] Bank selection drop-down menu

> You can select macro's banc from the drop-down menu per a track: COOL/NATURAL/HOT/SUBTLE/WARM/VIVID/CLUB 1/CLUB 2.

[19] Browser tab

> You can open the browser and load a track to track display area [16].

[Screenshot: Browser tab showing Collection tree on left with playlists, track list on right with columns for artwork, track title, artist, album, BPM, time, key, date added]

[20] Scene tab

> You can change relation of scene per phrase displayed in track display area [16].

[Screenshot: Scene tab showing mood/bank filter dropdowns, USER'S SCENE checkbox, and scene thumbnails in grid]

[21] Grid tab

> You can edit grid and phrases of tracks displayed in track display area [16].

[Screenshot: Grid tab showing GRID EDIT section with bars, BPM controls, and PHRASE EDIT section with CUT and CLEAR buttons]

On the Macro editor screen, you can control decks using a DJ controller or MIDI controller. For DJ units supporting this function, visit rekordbox.com > [Support] > [Compatible DJ units].

For details on grid edit, go to rekordbox.com> [Support] > [Manual] and see rekordbox Instruction Manual.

For details on phrase edit, go to rekordbox.com> [Support] > [Manual] and see PHRASE EDIT Operation Guide.

### 6.5.1 Keyboard shortcuts (Macro editor)

- Collapse/Expand all fixtures

[End] key

- Play/Pause

[Space] or [Z] key

- Return to the beginning of the track

[Home] , [H] , or Shift+[A] key

- 4 beats forward

[->] , [F] , or Ctrl+[T] key

- 4 beats backward

[<-] , [B] , or Ctrl+[R] key

## 6.6 Exporting and importing lighting settings

In LIGHTING mode, you can export and import your lighting settings. Use this function for backup or to copy to another computer.

[Screenshot: rekordbox menu bar showing File > Library submenu with "Export lighting settings" [1] and "Import lighting settings" [2] options highlighted, alongside other options like Backup Library, Restore Library]

[1] Export lighting settings

> Exports the settings file for lighting.

[2] Import lighting settings

> Imports the settings file for lighting.

# 7 Preferences

## 7.1 PERFORMANCE mode

In PERFORMANCE mode, open [Preferences] > [Extensions] category > [Lighting] tab and you can select the following settings.

- Enable Lighting Function:

> If you uncheck this, you cannot use LIGHTING mode. Macro will not work as well.

- Lighting Thumbnail display setting:

> If you uncheck [Display lighting thumbnail on decks], lighting thumbnail on deck1 or deck 2 will not be displayed.

- Setting of Venue to play Macro:

> Select a venue to play macro.

- Delay Compensation for Lighting:

> You can set delay compensation value from -500 msec to +500 msec to sync audio and lighting.

- Setting of playing Macro:

> If you uncheck [Play Macro even no music on the floor], macro will not be played when music is not on-air.

> If [Use deck 3 and deck 4] is unchecked, the deck 3 and deck 4 are not selected for automatic lighting effect.

- Ambient mode setting:

> [Tempo] allows you to change the speed of the ambient mode effect.

> If you check [Start automatic lighting effects when all tracks have stopped playing], Ambient mode starts automatically when all tracks have stopped playing.

> For details on Ambient Mode, see "9 Using Ambient mode".

- Output DMX-IF Setting

> Select the DMX interface you use.

- USER Color setting

> You can set the USER Color in Performance mode.

- DMX Direct Control settings

> You can set DMX Direct Control 4 - 9.

## 7.2 LIGHTING mode

In LIGHTING mode, open [Preferences] > [Extensions] category and you can select the following settings.

- Setting of Value to play Macro:

> Select a venue to play macro.

- Delay Compensation for Lighting:

> You can set delay compensation value from -500 msec to +500 msec to sync audio and lighting.

- Ambient mode setting:

> [Tempo] allows you to change the speed of the ambient mode effect.

> If you check [Start automatic lighting effects when all tracks have stopped playing] checkbox, Ambient mode starts automatically when all tracks have stopped playing.

> For details on Ambient Mode, see "9 Using Ambient mode".

- Output DMX-IF Setting

> Select the DMX interface you use.

- PRO DJ LINK Lighting:

> If [Use PRO DJ LINK Lighting] is turned on, the [PRO DJ LINK LIGHTING] tab is added to LIGHTING mode, allowing to connect to the PRO DJ LINK network via LAN.

> For details on using PRO DJ LINK Lighting, see "11 Using PRO DJ LINK Lighting".

- Reset all Scenes data :

> When you click this, all scenes data will be reset.

> \*Scenes newly created by the user or saved under a different name are not reset.

# 8 Controlling lighting effects in PERFORMANCE mode

In PERFORMANCE mode, you can open the Lighting panel to control lighting effects.

[Screenshot: PERFORMANCE mode screen with Lighting panel open at bottom, showing numbered controls: [1] AUTO/MANUAL button, [2] DECK select buttons (1-4), [3] Mood (HIGH/MID/LOW), [4] Bank (COOL/NATURAL/HOT/SUBTLE/WARM/VIVID/CLUB1/CLUB2), [5] Color, [6] Strobe, [7] BLACK OUT, [8] DIMMER slider, [9] Ambient mode, [10] DMX Direct Control]

[1] AUTO/MANUAL button

> If you're using DJ equipment that supports external mixer mode(\*), [MANUAL] is forcibly selected and [AUTO] cannot be selected.

> When [MANUAL] is selected, you can use [2] DECK select button.

> When [AUTO] is selected, you can't use [2] DECK select button.

> You can check the mixer mode at [Preferences] > [Audio] category > [Input/Output] tab > [Mixer Mode].

> (\*) Except for Pioneer DJ products that support Lighting mode. For the list of Pioneer DJ products that support Lighting mode, see the FAQ on rekordbox.com.

> https://rekordbox.com/en/support/faq/lighting-6/#faq-q600149

[2] DECK select button

> [DECK1]: Click [1] to select DECK1 for lighting performance.

> [DECK2]: Click [2] to select DECK2 for lighting performance.

> [DECK3]: Click [3] to select DECK3 for lighting performance.

> [DECK4]: Click [4] to select DECK4 for lighting performance.

[3] Mood

> Select [HIGH], [MID] or [LOW] for the mood of the lighting that is being performed.

[4] Bank

> Select [COOL], [NATURAL], [HOT], [SUBTLE], [WARM], [VIVID], [CLUB1] or [CLUB2] for the bank of the lighting that is being performed.

[5] Color

> Select [RED], [GREEN], [BLUE], [MAGENTA], [YELLOW], [CYAN], [WHITE] or [USER] for the color of the lighting that is being performed.

[6] Strobe

> Select [FAST], [MIDDLE], [SLOW] or [OFF] for strobe effects for all lighting effects.

[7] BLACK OUT

> When it is selected, all lights being performed are turned off.

[8] DIMMER

> Use this to adjust brightness on the Lighting panel.

[9] Ambient mode

> Use this mode to select a scene in Ambient mode and turn it on/off.

> For details on Ambient Mode, see "9 Using Ambient mode".

[10] DMX Direct Control

> Use this to set DMX Direct Control and turn it on/off.

> For details on DMX Direct Control, see "10 Using DMX Direct Control".

You can control items in the Lighting panel via MIDI LEARN, PAD EDITOR or keyboard shortcut feature.

\*To learn more about MIDI LEARN, please go to rekordbox.com > [Support] > [Manual] and refer to MIDI LEARN Operation Guide.

\* To learn more about PAD EDITOR, please go to rekordbox.com > [Support] > [Manual] and refer to Pad Editor Operation Guide.

\* To learn more about keyboard shortcut feature, please go to rekordbox.com > [Support] > [Manual] and refer to Default keyboard shortcut references.

# 9 Using Ambient mode

In Ambient mode, lighting effects start independently from playback. Even if there is no track loaded on the deck, you can start and end the lighting effects at any timing.

[Screenshot: Ambient mode dropdown showing INTERLUDE 1 through INTERLUDE 6 options, with AUTO and MANUAL buttons]

You can set at Preferences to start automatic lighting effects when all tracks have stopped playing.

[Screenshot: Ambient Mode Setting panel showing Tempo slider set to 120 BPM, Auto Start checkbox "Start automatic lighting effects when all tracks have stopped playing"]

You can edit scene data for Ambient mode. Select [AMBIENT] on the Macro Mapping screen in LIGHTING mode.

[Screenshot: Macro Mapping scene filter showing ALL dropdown and AMBIENT dropdown selected, with ambient scene thumbnails including INTERLUDE 1 through 6]

# 10 Using DMX Direct Control

This feature enables controlling fixtures that are not supported by rekordbox such as FOG machines and mirror balls. PAN/TILT override on the moving head during automatic lighting effects.

Turn on the setting button to open DMX Direct Control setting dialog box. You can select values used when the button is turned on/off for the DMX channel. If there is no value, the value of automatic lighting is applied.

[Screenshot: DMX Direct Control Setting 2 dialog showing Title "Gigbar2 UV", checkboxes for "Turn button ON when opening" and "Sync and turn OFF with the BLACKOUT button", table with 32 rows showing No, Channel, Min-Value On-Value, Max-Value Off-Value, Knob checkbox, and Note columns. First 4 rows configured for Body1 PAR1, Body1 PAR2, Body2 PAR1, Body2 PAR2]

In the "Note" text box, you can enter any text you want.

If you check the "Turn button ON when opening" checkbox, the button will turn on automatically when you start rekordbox.

If you check the "Sync and turn OFF with the BLACKOUT button" checkbox, the button will turn off automatically when the "BLACKOUT" button is turned on.

When you turn on the "Knob" checkbox for a channel, you can control the output DMX value for the channel using the MIDI knob or slider within the Min-Value and Max-Value range.

[Screenshot: DMX Direct Control Setting 2 dialog with Min-Value and Max-Value columns highlighted, showing channels configured with values 0-255 for Body PAR controls with Knob checkboxes enabled]

Select the MIDI knob/slider you use via MIDI LEARN.

[Screenshot: MIDI setting dialog showing Connected device section, function list with DMXDirectControl Knob 2 and DMXDirectControl 2 selected, and category menu showing Blackout, MasterDimmer, DeckSelect, Mood, Bank, Color, Strobe, AmbientMode, DMXDirectControl, DMXDirectControlKnob with sub-items DMXDirectControl Knob 1 through 9]

The DMX Direct Control settings are saved for each venue. Copying the venues also copies the DMX Direct Control settings.

You'll need a DJ controller or MIDI controller to use DMX Direct Control 4 ~ 9 in PERFORMANCE mode.

\* For DMX Direct Control 4 ~ 9 settings, see 7. Preferences.

\* For MIDI LEARN settings, see rekordbox.com > [Support] > [Manual] > [MIDI LEARN Operation Guide].

\* For Pad Editor operation, SEE rekordbox.com> [Support] > [Manual] > [Pad Editor Operation Guide].

# 11 Using PRO DJ LINK Lighting

When you select [PRO DJ LINK LIGHTING] tab in Lighting mode, you can create lighting effects that matches the music played by the DJ player connected to the PRO DJ LINK network via LAN.

When playing a track analyzed by phrase analysis in rekordbox on a DJ player that supports PRO DJ LINK LIGHTING, scenes that match the phrases are automatically selected.

For details on phrase edit, see the rekordbox PHRASE EDIT operation guide, available at the URL below: rekordbox. com > [Support] > [Manual]

[Screenshot: PRO DJ LINK LIGHTING screen with numbered areas: [1] PLAYER information showing 4 player slots with CUE button, PHRASE and ON AIR indicators, [2] Player selection for lighting effects with AUTO and numbered buttons, [3] Mood/Bank selection with AUTO/HIGH/MID/LOW and bank buttons, [4] Phrase selection showing INTRO through OUTRO and VERSE options, [5] Default scene dropdown, [6] DMX DIRECT CONTROL with Stage Spot/Fogging/UV Lighting buttons, [7] AMBIENT MODE with AUTO and INTERLUDE 1-6 buttons, [8] COLOR selection with AUTO and color buttons (RED/GREEN/BLUE/WHITE/MAGENTA/YELLOW/CYAN/USER), [9] STROBE with AUTO/FAST/MIDDLE/SLOW/OFF, [10] BRIGHTNESS with BLACK OUT button and MIN-MAX slider]

[1] PLAYER information

> This section shows information about DJ players connected to the PRO DJ LINK network. Playback status, phrase information, and on-air status are shown.

[2] Player selection for lighting effects

> Displays player No. for lighting effects. When the [AUTO] button is turned on, the player for lighting effects will be selected automatically.

> Click the player No. to change. The [AUTO] button will be turned off.

[3] Mood/Bank selection

> Displays the Mood/Bank during the lighting effects. When the [AUTO] button is on, the Mood/Bank in the lighting effects is displayed.

> Click the Mood/Bank to change. The [AUTO] button will be turned off.

[4] Phrase

> Displays the name of the phrase currently selected for lighting effects.

> Click the phrase name to change the lighting effects.

[5] Default scene

> Select the default lighting effect pattern used when no phrase information is received from the player.

[6] DMX Direct Control

> Select DMX Direct Control and turn it on/off.

[7] Ambient mode

> Click one of the scenes to immediately start Ambient mode. Click [AUTO] to stop Ambient mode.

> In [Preferences], you can set Ambient mode to start automatically when all players stop playing.

[8] Color

> Select the color of the lighting effects currently using from [RED], [GREEN], [BLUE], [MAGENTA], [YELLOW], [CYAN], [WHITE] or [USER]. Turn on [AUTO] to return to the lighting effects selected for mood/bank/phrase.

> You can set any color to [USER] by selecting [USER] button.

[9] Strobe

> Select [FAST], [MIDDLE], [SLOW] or [OFF] for strobe effects for all lighting effects. Turn on [AUTO] to return to the lighting effects selected for mood/bank/phrase.

[10] Brightness

> Move the slider to adjust the brightness of fixtures. Turn on [BLACK OUT] to turn off all fixtures.

# 12 Online support site

Before making inquiries on rekordbox operating procedures or technical issues, read rekordbox Instruction Manual and check the FAQs provided on rekordbox.com.

---

- rekordbox is a trademark or registered trademark of AlphaTheta Corporation.
- Windows is a trademark or registered trademark of Microsoft Corporation in the U.S. and other countries.
- Mac and macOS are trademarks of Apple Inc., registered in the U.S. and other countries and regions.
- Intel is a registered trademark of Intel Corporation in the U.S. and other countries.
- Other product, technology and company names, etc. mentioned herein are trademarks or registered trademarks of their respective owners.

(C) AlphaTheta Corporation

## Related Documents

- [manual/32-menu-list.md](../manual/32-menu-list.md) (effects, interface, lighting, preferences)
- [features/overview.md](../features/overview.md) (effects, interface, lighting)
- [guides/video-function.md](video-function.md) (effects, interface, preferences)
- [guides/edit-mode.md](edit-mode.md) (interface, phrase)
- [guides/introduction.md](introduction.md) (interface, lighting)
- [manual/18-performance-screen.md](../manual/18-performance-screen.md) (effects, interface)
- [manual/31-preferences.md](../manual/31-preferences.md) (interface, preferences)
- [faq/lighting-and-video.md](../faq/lighting-and-video.md) (lighting)
