---
id: faq-usb-and-devices
title: "FAQ: USB & Devices"
type: faq
source:
  file: "rekordbox7-faq.md"
  url: "https://rekordbox.com/en/support/faq/rekordbox7/"
  version: "7.x"
topics: [backup, devices, equipment, export, mobile, pro-dj-link, usb]
modes: [common]
confidence: pending
last_verified: null
transcribed_by: agent
verified_by: null
---

### My USB storage device or SD card was already authenticated with another AlphaTheta account in rekordbox CloudDirectPlay so I can't authenticate it. What should I do?

If your USB storage device or SD card is authenticated with another AlphaTheta account, you'll need to cancel the authentication using one of the following methods.

1. If you have multiple accounts: Switch the account to the account that authenticated the device, then cancel the authentication.
2. If the account belongs to someone else: Ask the person to cancel the authentication.
3. Back up the contents of the USB storage device or SD card, then format it.

---

### Can MIX POINT LINK function be controlled by DJ equipment?

Yes, it can be controlled by DJ equipment on which MIX POINT LINK function is mounted.

In addition, you can use each function by assigning it to the hardware on MIDI LEARN.

However, it is necessary to subscribe to a supported Plan.

---

### Exported SD memory cards or USB storage devices could not be recognized by DJ equipment, and tracks are not displayed.

Your device may be formatted in a file system that is not supported by the DJ equipment.

If the file system is exFAT, only [these DJ equipment](https://rekordbox.com/en/support/faq/v7/#faq-q700010) are available.

If you intend to use other DJ equipment, go [here](https://rekordbox.com/en/support/faq/v7/#faq-q600157), and format to a file system other than exFAT for use.

---

### When I uninstall or upgrade rekordbox, will my backups of Device library still exist?

Yes. Even if you uninstall or upgrade rekordbox, your backups of Device library won't be deleted.

---

### If I cancel the Professional plan, will my backups of the Device library be deleted?

No. Even if you cancel the Professional plan, your backups of the Device library won't be deleted.

You can use the backup data in the future if you subscribe to the Professional plan again.

---

### What is the Device library backup?

With this feature, you can make a backup of your Device library in advance so you can restore it via rekordbox if the USB storage device you exported files to is corrupted or lost.

---

### What do I need to use the Device library backup?

To use the Device library backup, you'll need to satisfy all of the following:

- rekordbox for Mac/Windows ver. 6.5.3 or later
- Professional plan subscription
- Turn on Cloud Library Sync
- Subscribe and login* to Dropbox

*Not required if you don't want to save backups of the Device library to the cloud.
Supported services are Dropbox and Google Drive.

---

### Can I restore older backups of my Device library?

No, only the last exported state of the backup is saved for each USB storage device

Older backups will be deleted automatically.

---

### I can't see the data of my Device library that I backed up to the cloud via another PC/Mac. Why not?

If the backup process doesn't finish properly, the data won't be visible on another PC/Mac.

In this case, make sure Dropbox is completely synced.

---

### I can't restore my Device library. Why not?

If the file system of the device you lost and that of the new device you want to save the restored files to is not the same, you can't restore them.

Check the file system of the device you want to save the restore data to.

---

### I can't restore my music files. Why not?

The Device library backup doesn't backup music files themselves.

To restore the data, the same music files on your PC/Mac or in the cloud are copied to the new USB storage device.

If you delete a file or move it to another PC/Mac, that file won't be restored.

---

### Which DJ equipment supports exFAT?

The following DJ equipment supports exFAT.

- CDJ-3000X
- CDJ-3000
- OPUS-QUAD
- OMNIS-DUO
- XDJ-XZ
- XDJ-RX3

As of September 2025

---

### Which file system should I format to in order to use SD memory cards and USB storage devices?

File systems supported by rekordbox are the following.

![](https://cdn.rekordbox.com/files/20220323175522/filesystem_EN22.png)

*Go [here](https://rekordbox.com/en/support/faq/v7/#faq-q700010) for DJ equipment that support exFAT.

Refer to the Pioneer DJ support page for SD memory cards and USB storage devices that are available on DJ equipment.

---

## Related Documents

- [guides/device-library-backup.md](../guides/device-library-backup.md) (backup, devices, export, usb)
- [faq/hardware-compatibility.md](hardware-compatibility.md) (equipment, export, usb)
- [guides/cloud-direct-play.md](../guides/cloud-direct-play.md) (devices, pro-dj-link, usb)
- [guides/pro-dj-link-setup.md](../guides/pro-dj-link-setup.md) (devices, equipment, pro-dj-link)
- [guides/usb-export.md](../guides/usb-export.md) (devices, export, usb)
- [manual/10-mobile-devices.md](../manual/10-mobile-devices.md) (devices, export, mobile)
- [manual/13-export-pro-dj-link.md](../manual/13-export-pro-dj-link.md) (export, pro-dj-link, usb)
- [manual/15-export-lan.md](../manual/15-export-lan.md) (devices, export, pro-dj-link)
