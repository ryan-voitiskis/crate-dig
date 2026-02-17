---
id: device-library-backup
title: "Device Library Backup Guide"
type: guide
source:
  file: "rekordbox6.7.4_device_library_backup_guide_EN.pdf"
  pages: "1-13"
  version: "6.7.4"
topics: [backup, devices, export, library, usb]
modes: [common, export]
confidence: pending
last_verified: null
transcribed_by: agent
verified_by: null
---

# Device library backup Operation Guide

[Screenshot: Pioneer DJ logo and rekordbox logo on title page]

# About this manual

This manual explains the Device library backup function of rekordbox. Read "rekordbox Introduction" and "Instruction Manual".
rekordbox.com/en/download/#manual

- In this manual, the name of buttons and menus displayed on rekordbox are indicated with brackets (e.g. [Restore to device], [Device library management] window).
- Please note that depending on the operating system version, web browser settings, etc., operation may differ from the procedures described in this manual.
- Please note that the language on the rekordbox screen described in this manual may differ from the language on your screen.
- Please note that the specifications, design, etc. of rekordbox may be modified without notice and may differ from the descriptions in this manual.

## Contents

| Section | Page |
|---|---|
| About this manual | 2 |
| **Introduction** | **4** |
| About Device library backup | 4 |
| **Device library backup** | **6** |
| Type of Device library backup | 6 |
| Automatically backing up Device library | 7 |
| Manually backing up the Device library | 8 |
| Setting the backup destination of the Device library | 9 |
| **Device library restoration** | **10** |
| Restoring the Device library | 10 |
| **Device library deletion** | **11** |
| Deleting the backed up Device library | 11 |
| **Others** | **12** |
| Troubleshooting | 12 |
| System requirements | 13 |
| Copyright warning | 13 |
| Trademarks and licenses | 13 |

# Introduction

## About Device library backup

When you export music files to a USB storage device, the BPM, key, waveforms, etc. analyzed music files, and management information such as Cues and loops are also exported at the same time. This management information is called "Device library."

If your USB storage device is damaged or lost, the Device library may also be lost. By using the Device library backup, you can back up the Device library in your PC/Mac or to the cloud storage (Dropbox). You can restore the Device library from the backup even if the USB storage device is damaged or lost.

[Screenshot: Diagram showing backup and restoration flow between rekordbox on PC/Mac, USB storage device, and Cloud storage. Arrows labeled "Backup" go from USB device to rekordbox and from rekordbox to cloud storage. Arrows labeled "Restoration" go from rekordbox to USB device and from cloud storage to rekordbox. A cylinder icon labeled "Device library" appears next to the USB storage device.]

- Before backing up to cloud storage, turn on Cloud Library Sync in the [MY PAGE] window and log in to cloud storage.
- Device library backup supports all modes of rekordbox (EXPORT mode, PERFORMANCE mode, LIGHTING mode, and EDIT mode).

### rekordbox version

Device library backup is included in rekordbox ver. 6.5.3 or later. If you are using an earlier version, please install the latest version.

### Subscription (Upgrading to Professional Plan)

Device library backup is available with the subscription Professional Plan. If you subscribed to Free Plan, Core Plan, or Creative Plan, please upgrade to Professional Plan.
rekordbox.com/en/plan/

To backup the Device library to the cloud storage, create a Dropbox account.
rekordbox.com/en/cloud-unlimited-setup-guide/

### Supported OS

Device library backup is supported by macOS and Windows.
For the supported version of your operating system, refer to the operating environment page on the rekordbox website.
rekordbox.com/en/download/#system

# Device library backup

The management information of the music files (Device library) exported to the USB storage device is backed up in your PC/Mac or to the cloud storage.

**Note**

- Only the latest Device library is backed up for each USB storage device, and old backups are deleted.
- Music files are not backed up. When restoring the Device library, same music files in the PC/Mac or cloud storage can be copied and restored. If you delete music files or move files to the other PC/Mac, you will not be able to restore files.
- When backing up the Device library to the cloud storage, it is saved for each PC/Mac performed the backup.
- Backup may take some time. Do not remove the USB storage device until the backup is complete.
- A backup file larger than 4GB cannot be created on a FAT32 formatted drive. To create a backup file of 4GB or more, use the USB storage device formatted with HFS+, etc.
- Even if you cancel the subscription (Professional Plan), the backed up Device library will not be deleted.
- If there is not enough space on the USB storage device, the Device library will not be able to back up. Delete unnecessary files to make space, and then try again.

## Type of Device library backup

There are two types of Device library backup such as automatic backup and manual backup. Automatic backup is performed automatically when you remove the USB storage device from your PC/Mac. Manual backup can be performed at your preferred timing.

Automatic backup can be enabled of disabled for each USB storage device. Manual backup can be performed on all USB storage devices.

## Automatically backing up Device library

When the USB storage device is removed from your PC/Mac, the Device library will be automatically backed up.

You can set to enable or disable for each USB storage device. On the window appeared when you connect a USB storage device which you have never connected to, select [Yes] or [No].

### Changing settings for automatic backup

You can change the settings for automatic backup on [Devices] of the tree view.

1. Select [Devices] > the device name from the tree view, and then right-click it.

[Screenshot: Tree view showing Devices section with "My DJ" device expanded, containing "All Tracks", "Playlists", and "Hot Cue Bank Lists" items]

2. Select [Device library backup] > [Automatically backup Device library upon removal] from the menu screen, and then select [On] or [Off].

## Manually backing up the Device library

You can back up the Device library at any time you like.

1. Select [Devices] > the device name from the tree view, and then right-click it.

[Screenshot: Tree view showing Devices section with "My DJ" device expanded, containing "All Tracks", "Playlists", and "Hot Cue Bank Lists" items]

2. Select [Device library backup] > [Backup now] from the menu screen. Backup starts. Follow the screens.

## Setting the backup destination of the Device library

By operating the [Preferences] window > [Advanced] tab > [Library Sync] tab, you can set the backup destination to your PC/Mac or cloud storage on [Device library].

[Screenshot: Preferences window showing the Advanced tab with Library Sync sub-tab selected. The window displays sections for Cloud Library Sync (with "Show MY PAGE" button, radio buttons for "Copy music files" and "Move music files", a checkbox for "Confirm operation for every upload", and a Location of Files field with Browse button), Mobile Library Sync (with Location of Files field and Browse button), and Device library (with a checkbox for "Change backup destination to cloud storage" highlighted with a red border)]

When you select [Change backup destination to cloud storage], the Device library will back up to the cloud storage.

When you do not select it, the Device library will back up to your PC/Mac.

**Note**

- It cannot be set in LIGHTING mode. Use EXPORT mode, PERFORMANCE mode, or EDIT mode.
- To back up the Device library to the cloud storage, click [Show MY PAGE] to display the [MY PAGE] window, turn on Cloud Library Sync, and then log in to the cloud storage. For more information about Cloud Library Sync and cloud storage service, refer to "Cloud Library Sync Operation Guide."

# Device library restoration

## Restoring the Device library

1. Connect the USB storage device, which you want to restore, to your PC/Mac.

2. Select [Devices] > the device name from the tree view, and then right-click it.

[Screenshot: Tree view showing Devices section with "My DJ" device expanded, containing "All Tracks", "Playlists", and "Hot Cue Bank Lists" items]

3. Select [Device library restoration] from the menu screen. The [Device library restoration] window appears.

4. Select a Device library to be restored, and then click [Restore to device].

[Screenshot: Device library restoration window showing a table titled "Restorable device library" with "4 files." The table has columns for Device name, Computer name, Date and time of backup, Backup destination, File system, and Device background. Four rows show: My DJ 1 / My PC / xxxx/xx/xx xx:xx:xx / This computer / exFAT / Default; My DJ 2 / My PC / xxxx/xx/xx xx:xx:xx / Dropbox / exFAT / Default; My DJ 3 (selected, highlighted in red) / My PC / xxxx/xx/xx xx:xx:xx / Google Drive / exFAT / Default; My DJ 4 / My PC / xxxx/xx/xx xx:xx:xx / Google Drive / exFAT / Default. Below the table: "Restore destination file system: exFAT" and a "Restore to device" button.]

5. Select [OK]. Restoration starts. Follow the screens.

**Note**

- After restoring, the old Device library in the USB storage device will be overwritten.
- Restoration may take some time. Do not remove the USB storage device until the restoration is complete.
- To restore, use a USB storage device formatted with same file system as when the backup was performed. If using the different formatted device, you cannot select the Device library.

# Device library deletion

## Deleting the backed up Device library

1. Click [Library] from the [File] menu on the upper screen, and then click [Backed up device library management]. The [Device library management] window appears.

2. Select the Device library to be deleted, click [Delete].

[Screenshot: Device library management window showing a table titled "Restorable device library" with "4 files." The table has columns for a checkbox, Device name, Computer name, Date and time of backup, Backup destination, File system, and Device background. Four rows show: unchecked My DJ 1 / My PC / xxxx/xx/xx xx:xx:xx / This computer / exFAT / Default; checked My DJ 2 (highlighted in blue) / My PC / xxxx/xx/xx xx:xx:xx / Dropbox / exFAT / Default; checked My DJ 3 (highlighted in blue) / My PC / xxxx/xx/xx xx:xx:xx / Google Drive / exFAT / Default; unchecked My DJ 4 / My PC / xxxx/xx/xx xx:xx:xx / Google Drive / exFAT / Default. Below the table are "Select all", "Clear all", and "Delete" buttons, with the Delete button highlighted with a red border.]

- You can select multiple Device library to delete.
- If you click [Select all], all Device library will be selected.
- If you click [Clear all], selected Device library will be unselected.

3. Click [OK]. Deletion starts. Follow the screens.

# Others

## Troubleshooting

Before making inquiries about operations or technical issues, refer to troubleshooting below, or check the [FAQ] on the rekordbox website.
rekordbox.com/en/support/faq/device-library-backup-6/

### There is not enough free space on the cloud storage.

- Change the cloud storage service plan of your personal subscription to increase the capacity of the cloud storage, or move some music files to the local folder on your PC/Mac with rekordbox for Mac/Windows that was used to upload the music files to the cloud storage.

### There is not enough free space on the HDD or SSD of your PC/Mac.

- Connect an external HDD to your PC/Mac and move the Dropbox folder to the external HDD.
- If you cannot obtain an external HDD, launch the Dropbox desktop application, access the Preferences menu, open [Sync] > [Selective Sync], and then cancel synchronization of a folder in the rekordbox folder of Dropbox.

### Data backed up on the cloud storage cannot be shown on the other PC/Mac.

- Check if the synchronization is complete on the Dropbox desktop application of the PC/Mac used for backup.

## System requirements

The latest information (supported OS, required operating environment, etc.) is available on the rekordbox website.
rekordbox.com/en/download/#system

## Copyright warning

To use of rekordbox is restricted with regard to the playing and copying of copy-protected music.

- The program may not operate properly if copy-protect encryption data is embedded in recorded media.
- Playback, analysis, and other operations may stop if copy-protect encryption data is detected as embedded in recorded media.

The material you record is for your own listening enjoyment, and cannot be used for other purposes without permission from the copyright owner.

- Music recorded from CDs and other media may be protected by the copyright laws of individual countries as well as by international convention. You are solely responsible for the lawful use of the recordings you make.
- When handling music that has been downloaded over the internet, the person who has downloaded the music is fully responsible for using the music in compliance with his/her contract with the website from which the music was downloaded.

## Trademarks and licenses

- rekordbox(TM) is a trademark or registered trademark of AlphaTheta Corporation.
- Dropbox is a trademark or registered trademark of Dropbox, Inc.
- Mac and macOS are registered trademarks of Apple Inc. in the U.S. and other countries.
- "Google", the "Google Logo", and "Google Drive(TM)" are trademarks or registered trademarks of Google LLC.

Other product, technology and company names, etc. mentioned herein are trademarks or registered trademarks of their respective owners.

(C) 2021 AlphaTheta Corporation.

## Related Documents

- [faq/usb-and-devices.md](../faq/usb-and-devices.md) (backup, devices, export, usb)
- [guides/usb-export.md](usb-export.md) (devices, export, library, usb)
- [faq/hardware-compatibility.md](../faq/hardware-compatibility.md) (export, usb)
- [faq/library-and-collection.md](../faq/library-and-collection.md) (export, library)
- [features/cloud-setup-guide.md](../features/cloud-setup-guide.md) (backup, library)
- [features/overview.md](../features/overview.md) (export, library)
- [guides/cloud-direct-play.md](cloud-direct-play.md) (devices, usb)
- [guides/cloud-library-sync.md](cloud-library-sync.md) (backup, library)
