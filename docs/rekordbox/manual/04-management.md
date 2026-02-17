---
id: management
title: "Management"
type: manual
source:
  file: "rekordbox7.2.8_manual_EN.pdf"
  pages: "21-24"
  version: "7.2.8"
topics: [backup, collection, library]
modes: [common]
confidence: pending
last_verified: null
transcribed_by: agent
verified_by: null
---

# Management

## Saving data using the backup function

Regularly backing-up is recommended to prevent data from being lost due to computer failure or other disasters.

Data in rekordbox such as [Collection], playlists, analysis data and music files can be backed up. The backup function can also be used for migrating data from the current computer to a new one. Data backed up for Mac can be restored in Windows and vice versa.

**Note**

- If a lot of music files exist in [Collection], it may take a long time to back them up, depending on the computer's performance.

1. Click [Library] from the [File] menu on the upper screen, and then click [Backup Library].

2. Click [OK].
   [Do you want to back up music files as well?] is displayed.

3. Select [Yes] or [No].
   Backing up your music files is recommended when you migrate data from your current computer to a new one. Music files are backed up in the [rekordbox_bak] folder, to be restored in the same path as specified in the backed up files.

4. Specify a path to store the backup data, and then click [Save].
   The backup starts.
   Once the backup is completed, [Collection was backed up.] is shown.

5. Click [OK].

## Using the Restore function

Your data is replaced with the backed up data.

**Note**

- If the backup data consists a lot of music files, it may take a long time to restore them, depending on the computer's performance.

1. Click [Library] from the [File] menu on the upper screen, and then click [Restore Library].

2. Click [OK].

3. Select the backup data (.zip or .edb), and then click [Open].
   Replacement of the present data with the backup data starts.
   Once the process is completed, [Collection was restored.] is shown.

4. Click [OK].

**Hint**

- You can replace the present data with the backup data in .edb files from old rekordbox versions.
- To restore data with music files, the backup music files (rekordbox_bak folder) must be placed at the same path as the backup data.

## Updating the file paths of missing music files

rekordbox manages information on where music files are stored (file paths). If file and folder names are changed or if files or folders are moved or deleted, it may no longer be possible to play the music file. ([!] is displayed to the left of missing music files.) rekordbox can find missing files and relocate their directories automatically.

### Relocate automatically

1. Open the [Preferences] window > [Advanced] category > [Database] tab > [Auto Relocate Search Folders], and select file types ([Music] and/or [Video]).

2. Click the [User specified folders] checkbox, click [Add], and then select the directory folder and drive containing the files.
   Click [Add] again to select multiple folders.
   To search in desktop, select [Desktop].
   To delete the set folders, display the folder you want to delete, and then click [Del].

3. Click [File] on the upper screen, and select [Display All Missing Files].
   A list of all missing files is displayed.

4. Click [Auto Relocate].
   The files are deleted from the list after all relocating is completed.
   If file cannot be detected because it has been deleted from your computer, it remains on the list. In that case, click [Delete] to remove it from [Collection].

**Hint**

- Right-click on the track or video marked with [!] in [Collection], and select [Auto Relocate] to automatically relocate it. Also, you can select multiple tracks and videos to automatically relocate at the same time. You can also select [Auto Relocate] from the [Track] menu on the upper-left of the screen.

### Relocate manually

1. Click [File] on the upper screen, and select [Display All Missing Files].
   This displays a list of all missing files.

2. Select a track to modify its file path, and click [Relocate].

3. Go to the folder containing the music file, select the file name, and click [Open].
   The file path is modified.

**Hint**

- You can also modify the file path by right-clicking a track or video marked with [!] in [Collection] and selecting [Relocate].
- You can select multiple tracks at once when using [Relocate]. Doing this will automatically relocate all selected tracks to the file path of the first track to be relocated.

**Note**

- Be careful not to select the wrong file when selecting the new file path for [Relocate].

## Related Documents

- [guides/cloud-library-sync.md](../guides/cloud-library-sync.md) (backup, collection, library)
- [faq/library-and-collection.md](../faq/library-and-collection.md) (collection, library)
- [features/cloud-setup-guide.md](../features/cloud-setup-guide.md) (backup, library)
- [features/overview.md](../features/overview.md) (collection, library)
- [guides/device-library-backup.md](../guides/device-library-backup.md) (backup, library)
- [guides/introduction.md](../guides/introduction.md) (collection, library)
- [guides/streaming-services.md](../guides/streaming-services.md) (collection, library)
- [manual/02-collection-window.md](02-collection-window.md) (collection, library)
