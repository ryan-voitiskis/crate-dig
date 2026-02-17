---
id: usb-export
title: "USB Export Guide"
type: guide
source:
  file: "USB_export_guide_en_251007.pdf"
  pages: "1-5"
  version: "7.x"
topics: [devices, export, library, playlists, usb]
modes: [export]
confidence: verified
last_verified: "2026-02-17"
transcribed_by: agent
verified_by: agent
---

# USB Export Guide

[Infographic: Title page showing a DJ character wearing headphones with a question mark, thinking. In the center is a blue speech bubble with "NEW" and the OneLibrary logo. A USB flash drive is shown in the middle. Dotted lines with link icons connect the elements: the DJ character is linked to a green DJ controller setup (representing Device Library compatible equipment), the OneLibrary logo is linked to a blue DJ controller setup (representing OneLibrary compatible equipment), and the USB drive is linked to a green database icon (representing Device Library). A laptop with the rekordbox icon is shown in the bottom right corner.]

## What is a device library?

### Device Library

When tracks are exported from rekordbox, a database is created on the USB storage device containing the playlists, cue information, and more. This database is called a Device Library.

[Infographic: Green database icon labeled "Device Library". Below it, a flow diagram shows: rekordbox for Mac/Windows icon >> USB flash drive >> green DJ controller setup, labeled "2009 - 2022".]

### OneLibrary*

To accommodate new features in future products, AlphaTheta has created a new export database to replace the traditional device library. This new database is called OneLibrary.

[Infographic: Blue OneLibrary logo icon labeled "OneLibrary". Below it, a flow diagram shows: rekordbox for Mac/Windows icon >> USB flash drive >> blue DJ controller setup, labeled "Since 2023".]

*Formerly: Device Library Plus

## Conversion from the traditional Device Library to OneLibrary

You can easily convert your existing Device Library to OneLibrary format. Install the latest version of rekordbox for Mac/Windows and connect your USB storage device. Follow the instructions displayed in the window to complete the conversion. Once converted, you can continue to use the USB storage device in the same manner as before.

[Infographic: A laptop with the rekordbox icon connected via dotted line to a USB flash drive. Below, an arrow diagram shows the green Device Library database icon converting (arrow pointing right) to the blue OneLibrary logo icon.]

- You can convert from Device Library to OneLibrary at any time from the context menu under the USB storage device in the Tree View.
- A OneLibrary cannot be converted to Device Library.
- Caution: If a OneLibrary exists on the USB storage device, converting the Device Library will overwrite the existing OneLibrary.

## OneLibrary compatible DJ equipment

The compatible libraries vary depending on the model of DJ equipment.
See here to find out which libraries are compatible for which DJ equipment model.

[Infographic: A DJ character with a question mark, thinking about which library type to use. Two columns are shown: left column has the green Device Library database icon with a USB flash drive below it pointing to a green DJ controller setup labeled "For Device Library Only"; right column has the blue OneLibrary logo icon with a USB flash drive below it pointing to a blue DJ controller setup labeled "For OneLibrary Only".]

## Two types of libraries on a USB storage device

Two libraries will coexist within the USB storage device:
OneLibrary (a new library) and Device Library (a traditional library).

> The type of library used depends on the DJ equipment model.

[Infographic: A USB flash drive in the center. Above and to the right, the blue OneLibrary logo icon is connected via a dotted line with a link icon to a blue DJ controller setup labeled "For OneLibrary Only". Below and to the left, a green DJ controller setup labeled "For Device Library Only" is connected via a dotted line with a link icon to the green Device Library database icon.]

## There may be differences between library types

The two types of libraries are managed separately.
OneLibrary compatible DJ equipment: The playlists and histories created with DJ equipment are saved in the OneLibrary.
Device Library compatible DJ equipment: The playlists and histories created with DJ equipment are saved in the traditional Device Library.

[Infographic: Top section shows the blue OneLibrary logo icon receiving (indicated by << arrows) Playlist A and History A icons from a blue DJ controller setup labeled "For OneLibrary Only". A USB flash drive is shown in the center. Bottom section shows a green DJ controller setup labeled "For Device Library Only" sending (indicated by >> arrows) Playlist B and History B icons to the green Device Library database icon.]

The contents of a USB storage device may appear differently depending on the DJ equipment used.
The histories or playlists created on OneLibrary compatible DJ equipment will not be displayed on Device Library compatible DJ equipment, and vice-versa.

[Infographic: Left side shows a DJ character looking confused with a speech bubble saying "Where?" next to a History B icon, indicating they cannot find History B. Right side shows Playlist A and History A icons with an arrow pointing down to a blue DJ controller setup labeled "For OneLibrary Only", indicating that only OneLibrary content is visible on OneLibrary-compatible equipment.]

## Steps to make two types of library the same

### STEP 01

#### Update Collection

Transfer the fine-tuned Cue points and Beat Grids edited with DJ equipment to reflect them in the rekordbox library.

[Infographic: The blue OneLibrary logo icon sends (>> arrows) cue point and beat grid data (shown as colored markers A, B, C, D and a waveform) to the rekordbox for Mac/Windows icon. Similarly, the green Device Library database icon sends the same type of cue point and beat grid data to rekordbox. A USB flash drive is shown to the left.]

In the Tree View under Devices, right-click on the USB storage device and select [Update Collection].

[Screenshot: Context menu in rekordbox Tree View showing "Devices" expanded with "USB Device" selected. The right-click menu shows three options: "Remove USB Device", "Update Collection" (highlighted), and "Import My Settings". Under USB Device, "Device Library" and "OneLibrary" sub-items are visible.]

### STEP 02

#### Import Playlist

Transfer playlists created with DJ equipment from a USB storage device to rekordbox.

[Infographic: The blue OneLibrary logo icon sends (>> arrows) Playlist A and History A icons toward the rekordbox for Mac/Windows icon. The green Device Library database icon sends (>> arrows) Playlist B and History B icons toward rekordbox. A USB flash drive is shown to the left.]

In the Tree View under Devices, select Playlists, right-click and select [Import Playlist].

[Screenshot: Context menu in rekordbox Tree View showing "Devices" expanded with "USB Device" > "Device Library" > "All Tracks" and "Playlists" visible. Under Playlists, "Playlist 01" and "Playlist 02" are shown. The right-click menu shows three options: "Add Artwork", "Import Playlist" (highlighted), and "Delete Playlist".]

rekordbox automatically imports history by default.

### STEP 03

#### Export Playlist

Exporting a playlist to a USB storage device will create the playlist on both OneLibrary and Device Library.

[Infographic: From the rekordbox for Mac/Windows icon, playlists and histories are exported to both library types on the USB flash drive. The blue OneLibrary logo icon receives Playlist A, History A, Playlist B, and History B. The green Device Library database icon also receives Playlist A, History A, Playlist B, and History B. Chevron arrows (<<) indicate the direction of data flow into each library.]

In the Tree View, select the desired [playlist] from the Playlists, right-click and select [Export Playlist].

[Screenshot: Context menu in rekordbox Tree View showing "All Tracks" and "Playlists" with "Playlist 02" selected. The right-click menu shows options including "Auto Upload" (with submenu arrow), "Export Playlist" (highlighted, with submenu arrow showing "USB Device"), and "Batch Auto Upload setting" (with submenu arrow).]

## Related Documents

- [guides/device-library-backup.md](device-library-backup.md) (devices, export, library, usb)
- [faq/library-and-collection.md](../faq/library-and-collection.md) (export, library, playlists)
- [faq/usb-and-devices.md](../faq/usb-and-devices.md) (devices, export, usb)
- [manual/09-collaborative-playlists.md](../manual/09-collaborative-playlists.md) (export, playlists, usb)
- [faq/hardware-compatibility.md](../faq/hardware-compatibility.md) (export, usb)
- [features/overview.md](../features/overview.md) (export, library)
- [guides/cloud-direct-play.md](cloud-direct-play.md) (devices, usb)
- [guides/cloud-library-sync.md](cloud-library-sync.md) (library, playlists)
