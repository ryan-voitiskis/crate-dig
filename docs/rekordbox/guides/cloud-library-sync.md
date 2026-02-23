---
id: cloud-library-sync
title: "Cloud Library Sync Operation Guide"
type: guide
source:
  file: "rekordbox7.2.8_cloud_library_sync_operation_guide_EN.pdf"
  pages: "1-36"
  version: "7.2.8"
topics: [backup, cloud, collection, library, mobile, onelibrary, playlists]
modes: [common]
confidence: verified
last_verified: "2026-02-17"
transcribed_by: agent
verified_by: agent
---

# Cloud Library Sync Operation Guide

# About this manual

This manual explains the Cloud Library Sync function of rekordbox. Read "rekordbox Introduction" and "Instruction Manual".
rekordbox.com/en/download/#manual

- In this manual, the name of buttons and menus displayed on rekordbox are indicated with brackets (e.g. [BPM], [Collection] window).
- Please note that depending on the operating system version, web browser settings, etc., operation may differ from the procedures described in this manual.
- Please note that the language on the rekordbox screen described in this manual may differ from the language on your screen.
- Please note that the specifications, design, etc. of rekordbox may be modified without notice and may differ from the descriptions in this manual.

# Introduction

## About Cloud Library Sync

Cloud Library Sync can synchronize libraries across computers and mobile devices which use the same AlphaTheta account. By uploading music files (or video files) to a cloud storage service, all computers and mobile devices that you own can download and play the uploaded music files (or video files).
(This manual mainly refers to music files in mp3, wav or other formats, as files available for Cloud Library Sync; however, video files in mp4, mov or other formats can also be uploaded and downloaded.)
For Cloud Library Sync, refer to the rekordbox website.

### rekordbox version

To use Cloud Library Sync, install the latest version of rekordbox.

### Creating a cloud storage service account

Cloud Library Sync supports Dropbox and Google Drive. (As of March 2025)
You need a Dropbox or Google account to use Cloud Library Sync. If you do not have an account, please accept the Dropbox or Google Drive Terms of Service and create a Dropbox or Google Drive account.
Please select and subscribe to the plan for Dropbox or Google Drive according to the size of the music files you upload.
https://www.dropbox.com
https://www.google.com

### Cloud storage service capacity

Cloud Library Sync can upload music files to the cloud storage service, but uploads cannot exceed the capacity of your service plan. Select a service plan with a capacity larger than the total amount of all files to be uploaded.

- By subscribing to Professional Plan and joining Professional DJ team, you can have a Dropbox account with the capacity up to 5 TB at no additional charge.
- By subscribing to Cloud Option and joining Cloud Option DJ team, you can have a Dropbox account with the capacity up to 1 TB.

### Subscription

The number of music files and available Dropbox storage size for Cloud Library Sync differs to each plan.

<!-- dprint-ignore -->
| Plan | The number of music files to synchronize | Available Dropbox storage size |
|---|---|---|
| Free Plan *1 | 20 music files in [Trial playlist - Cloud Library Sync] and 1000 music files in [Cloud Export] *2 | - |
| Core Plan *1 | 20 music files in [Trial playlist - Cloud Library Sync] and 1000 music files in [Cloud Export] *2 | - |
| Creative Plan *1 | All music files in the library | Contracted subscription storage size |
| Professional Plan | All music files in the library | 5 TB |
| Cloud Option | All music files in the library | 1 TB |

\*1: Cloud Option can be added.
\*2: When using Free Plan, you need to either connect a Hardware Unlock device or owner registration.

For Hardware Unlock devices, refer to the following URL:
rekordbox.com/en/support/hardware-unlock/

For owner registration equipment, refer to the following URL:
rekordbox.com/en/support/faq/v7/#faq-q700014

### Connection speed

The time required for library synchronization and music file upload/download varies greatly depending on your Internet connection speed. With rekordbox's Cloud Library Sync, an operating environment with a connection speed of 20 Mbps or higher is recommended for both upload and download.

### Supported OS

Cloud Library Sync is supported by the following OS. For the OS version, refer to the System Requirements or Specifications on each website.

- Computer: macOS, Windows
  rekordbox.com/en/download/#system
- Mobile device: iOS, Android
  rekordbox.com/en/feature/mobile/

### Individual use

With Cloud Library Sync, individuals can use the same library on multiple computers and mobile devices, but multiple people cannot share the same library.

## Activation for computers/mobile devices

Free Plan Core Plan

You can use [Trial playlist - Cloud Library Sync] without activation.
To use [Cloud Export] with Free Plan, connecting a Hardware Unlock device or owner registration is required.
To use [Cloud Export] with Core Plan, activation is required.
Activate all computers and mobile devices that you use.
Log in to rekordbox on all computers/mobile devices that you use Cloud Library Sync on with the same AlphaTheta account.

[Screenshot: rekordbox for Mac/Windows showing Current plan as Free Plan with Subscribe to a plan button and Plan History section]

[Screenshot: rekordbox for iOS/Android showing Account Information screen with Email address, Subscription Plan as Free Plan, Cloud Library Sync toggle, and Cloud Storage Services option]

Creative Plan Professional Plan Cloud Option

Activate all computers/mobile devices that you use Cloud Library Sync on.
When using rekordbox on multiple computers and mobile devices, Cloud Library Sync can synchronize your library if you use the same AlphaTheta account on all devices. For details on the number of computers/mobile devices which you can activate and synchronize the libraries to, refer to the Plan page on rekordbox.com.
rekordbox.com/en/plan/

[Screenshot: rekordbox for Mac/Windows showing Current plan as Professional Plan with Activate this computer toggle enabled, Expiration Date Jan 10, 2025, and Plan History showing Jan 10, 2024 - Professional]

[Screenshot: rekordbox for iOS/Android showing Account Information screen with Subscription Plan as Creative, Valid until 11.28, 2020, Activate toggle, and note that if activated Cloud Library Sync will become available]

1. AlphaTheta account: Your currently logged in AlphaTheta account
2. Subscription plan: Your currently contracted subscription plan
3. Activate: Turn the activation on/off.
   When you turn on the activation with rekordbox for iOS/Android, the screen in "Synchronizing your library" (page 25) is displayed.

**Note**

- For details on activation, refer to "rekordbox Introduction" on the rekordbox website.

# rekordbox for Mac/Windows

Use Cloud Library Sync with rekordbox on Mac/Windows.

## Installing the cloud storage service desktop application

When using rekordbox on Mac/Windows, Cloud Library Sync uses the cloud storage service desktop application to upload/download music files.
Install the Dropbox desktop application.

**Note**

- In order to use Cloud Library Sync, please accept the Dropbox Terms of Service and create a Dropbox account to use Dropbox.
  https://www.dropbox.com/en/privacy#terms
- When you use Google Drive, please accept the Google Terms of Service and create a Google account to use Google Drive.
  https://policies.google.com/privacy?hl=en
- By subscribing to Professional Plan and joining Professional DJ team, you can have a Dropbox account with the capacity up to 5 TB at no additional charge.
  By subscribing to Cloud Option and joining Cloud Option DJ team, you can have a Dropbox account with the capacity up to 1 TB.
  For details, refer to the Setup Guide.
  rekordbox.com/en/cloud-setup-guide/
- By subscribing to Creative Plan or Professional Plan, you do not need to install the Dropbox desktop application when [Synchronize selected music files only (Recommended)] is set for Synchronization method. The same applies when using Google Drive. For details, see "Setting Synchronization method for music files" (page 15).

1. Install the Dropbox desktop application.
   https://www.dropbox.com/install

**Note**

- If you log in to Dropbox after installing the Dropbox desktop application, use the same Dropbox account when you perform step 6 of "Synchronizing your library" (page 10). Also, if you are already using the Dropbox desktop application, use the same Dropbox account when you perform step 6 of "Synchronizing your library" (page 10).

## Synchronizing your library

By synchronizing your library using Cloud Library Sync, you can upload/download/move music files between Mac/Windows and the cloud storage.

1. Click the settings icon in the upper-right corner of the screen to open the [Preferences] window.

2. Click the [Library Sync] tab of the [CLOUD] category.

3. Turn on [Sync library to another device] of [Cloud Library Sync].

[Screenshot: Preferences window showing Library Sync tab under CLOUD category, with Cloud Library Sync section showing Sync library to another device toggle, Sync of music files section, Cloud storage service section with Dropbox and Google Drive login options showing Personal account and Logout buttons, and Default Cloud Storage radio buttons for Dropbox and Google Drive with Reset to defaults button]

\*1 The Dropbox account you used to log in to Dropbox in step 6 will be displayed.
When you subscribe to Professional Plan, you can check if you are using Professional DJ team with the capacity up to 5 TB.
When Cloud Option is added, you can check if you are using Cloud Option DJ team with the capacity up to 1 TB.

\*2 The Google account you used to log in to Google Drive in step 6 will be displayed.

4. Select [NEXT] when the window explaining the required account is displayed.

5. Select [OK] when the confirmation window to merge libraries is displayed.

6. Select the cloud storage from [Dropbox] or [Google Drive] to synchronize.
   The Cloud Library Sync guide screen is displayed.

7. Select [Login] when the window asking you to log in is displayed.
   The login screen to the cloud storage service is displayed on the web browser.

8. Log in to the cloud storage service.
   When you succeed in logging in, the Dropbox or Google account you used to log in will be displayed in [Cloud storage service] of the [Library Sync] tab. (See step 3.)

**Hint**

- The account you used to log in to the cloud storage service (Dropbox/Google) is synchronized across all computers and mobile devices which use the same AlphaTheta account. For your secondary and subsequent computers, log in to the cloud storage service desktop application using the same account you used to log in to the cloud storage service (Dropbox/Google) on rekordbox.
- The cloud storage service logged in will be set as the [Default Cloud Storage] of the [Library Sync] tab. You can download music files from any cloud storage service by logging in to the other cloud storage service. However, you can only upload to the cloud storage service you have set as [Default Cloud Storage].
- You can change the cloud settings from the [Library Sync] tab of the [CLOUD] category or the [DJ equipment linkage] tab.
- Click the [Setup Wizard] button of the [CLOUD] category to display the [Cloud Setup Wizard] window. By proceeding through the windows, you can set up cloud functions that match your DJ style and equipment environment. The settings configured in the [Cloud Setup Wizard] can be changed later in the [Preferences].

[Screenshot: Cloud Setup Wizard window showing equipment selection step with icons for DJ players, all-in-one systems, and controllers, with Cancel and NEXT buttons]

## Adding a music file to Trial playlist to upload

Free Plan Core Plan

When you add tracks to [Trial playlist - Cloud Library Sync], they will be automatically uploaded to the cloud storage.
You can add up to 20 tracks.

[Screenshot: Tree View showing Playlists section with Trial playlist - Cloud Library Sync, CUE Analysis Playlist with Start button, and My Playlist]

Music files are automatically downloaded to [Trial playlist - Cloud Library Sync] on the other logged-in computers/mobile devices and synchronized.

**Note**

- The following information cannot be synchronized.
  - Histories
  - Hot Cue Bank Lists
  - Tag List
  - Matching
  - Sampler List
  - Related Tracks

## Adding a music file to Cloud Export to upload

When you add music files to [Cloud Export], they will be automatically uploaded to the cloud storage.
In Free Plan and Core Plan, the number of music files that can be added is limited to 1000.
There is no such limitation for Creative Plan, Professional Plan, or Cloud Option.

[Screenshot: Cloud Export tree view showing folder structure with STANDARD folder containing House and Drum'n'bass subfolders, Imported Playlists folder with add button, and BEST HITS folder containing 90s, 00s, and oldies subfolders]

Music files are automatically downloaded to [Cloud Export] on the other logged-in computers/mobile devices and synchronized.

You can also add playlists to [Cloud Export] using the following [Cloud Export] screen.
Check the playlists/folders on the playlist synchronization screen, and click the [SYNC] button to add the selected playlists to [Cloud Export].

[Screenshot: Cloud Export sync screen showing Local Folder on the left with Playlists tree including STANDARD (House, Drum'n'bass), Imported Playlists (BEST HITS with 90s, 00s, oldies), and CloudExport on the right with matching folder structure, with a SYNC button in the center and Close button at bottom]

**Hint**

- Click the arrow button on the right side of the [Cloud Export] tree to display the [Cloud Export] screen.
- Playlists added to [Cloud Export] do not automatically synchronize with the original playlists. If you change the contents of the original playlists, click the [SYNC] button again to update the contents of [Cloud Export].

**Note**

- The following information cannot be synchronized.
  - Histories
  - Hot Cue Bank Lists
  - Tag List
  - Matching
  - Sampler List
  - Related Tracks

## Setting Synchronization method for music files

Creative Plan Professional Plan Cloud Option

You can set a method to synchronize music files with Dropbox.

When you select [Synchronize selected music files only (Recommended)], you can use your computer's storage efficiently.
It is suitable for storing music files on external storage or using only necessary playlists on your laptop computer.

When you select [Synchronize all music files], you can keep the same contents with your computers at home/studio and Dropbox.
It is suitable for storing all music files on your each computer's storage when it has enough free space.

**Note**

- You can select only [Synchronize selected music files only (Recommended)] when you use Google Drive.
- When you select [Synchronize all music files], install the Dropbox desktop application. See "Installing the cloud storage service desktop application" (page 9).
- For details about Synchronization method, click [?] or check the FAQ on the rekordbox website below.
  rekordbox.com/en/support/faq/library-sync-7/#faq-q700165

## Uploading/downloading/moving/deleting music files

Creative Plan Professional Plan Cloud Option

### Upload and download status of music files

The track list of the [Collection] shows the cloud storage upload status of each track, including music files uploaded and not uploaded to the cloud storage from this computer, music files not uploaded from other computers or your mobile devices, music files that can be downloaded from the cloud storage, etc.

[Screenshot: Track list showing three tracks with cloud status icons - Get Down (Original Mix) by Eli Brown, Trees (Original Mix) by Felon, and Feel Alright (Original Mix) by Franky Rizardo]

The icons displayed on the track list indicate the following statuses:

- Cloud icon (dark): A track is stored on the cloud storage and your computer. The track has already been uploaded to the cloud.
- Cloud icon (upload arrow): A track is not stored on the cloud storage. The track can be uploaded from this computer to the cloud.
- Cloud icon (crossed out): A track is not stored on the cloud storage. The track is stored on other computers or mobile devices.
- Cloud icon (download arrow): A track is stored on the cloud storage. The track can be downloaded from the cloud to this computer.

### Uploading music files

You can upload a music file from Mac/Windows to the cloud storage.

1. Select tracks from the track list to upload.

2. Right-click the track, and then select [Cloud Library Sync] > [Upload tracks].

[Screenshot: Right-click context menu showing various options including Add To Playlist, Add To Tag List, Preview, Reload Tags, Get Info from iTunes, Track Type, Cloud Library Sync submenu expanded showing Upload tracks, Move tracks to local storage, Download tracks, Delete tracks from local storage options, Export Tracks, Auto Load Hot Cue, Reset DJ Play Counts, Add New Analyses Data, Remove from Playlist, Remove from Collection]

3. When the [Upload tracks] dialog is displayed, select [Move] or [Copy].
   When you select [Move], the music files are moved to the Dropbox folder and uploaded to the cloud storage.
   When you select [Copy], the music files are copied to the Dropbox folder and uploaded to the cloud storage.

**Hint**

- By Opening the [Preferences] window > [CLOUD] category > [Library Sync] tab, you can select to move or copy in [Operation for uploading tracks]. See "[Cloud Library Sync] in the [Preferences] window" (page 24).
- When you select a playlist, you can upload all music files included in the playlist.
- When you select a playlist, and then select [Move tracks to local storage], all music files included in the playlist will be moved to the local folder on your computer.
- If you import a music file from the Dropbox folder on Mac/Windows to the [Collection], it will be displayed as "A track stored on the cloud storage and your computer" (cloud icon dark) in the track list of the [Collection]. See "Upload and download status of music files" (page 16).
- When you import a music file from the Google Drive folder on Mac/Windows to the [Collection], it will be displayed as "A track that is not stored on the cloud storage and can be uploaded from this computer to the cloud" (cloud icon upload arrow) in the track list of the [Collection]. See "Upload and download status of music files" (page 16).
  To synchronize this music file to other devices, upload it to Google Drive. Google Drive will be stored with both the already stored file and file uploaded from rekordbox. To synchronize it to other devices without storing both files, move the music file to the local folder on Mac/Windows, and then import it to the [Collection].

**Note**

- When you select [Move] in step 3, you may not be able to use the uploaded file on other applications.
- When [Synchronize selected music files only (Recommended)] is selected for [Synchronization method] (page 15), [Copy] is automatically set in step 3.

### Automatically uploading music files included in the playlist

When you turn on [Auto Upload] for the playlist, music files will be automatically uploaded to the cloud storage simply by adding them to the playlist.

1. Select a playlist.

2. Right-click the playlist, and then select [Auto Upload] > [On].
   Music files included in the playlist and stored on the computer will be automatically uploaded to the cloud storage.

[Screenshot: Right-click context menu on playlist showing Cloud Library Sync submenu, Auto Upload submenu with On/Off options, Export Playlist, Batch Auto Upload setting, Create New Playlist, Create New Intelligent Playlist, Create New Folder, Playlist display setting, Add Artwork, Delete Playlist]

**Hint**

- You can set whether or not to upload for each playlist in [Batch Auto Upload setting].

### Automatically uploading music files imported to the [Collection]

Professional Plan Cloud Option

When you enable [Collection Auto Upload], all music files imported to the [Collection] will be automatically uploaded to the cloud storage.

[Screenshot: Preferences window showing Library Sync tab under CLOUD category with Synchronization method section (Synchronize selected music files only / Synchronize all music files radio buttons), Collection Auto Upload section with Disable toggle, Operation for uploading tracks section with Copy music files / Move music files / Confirm operation for every upload radio buttons, and Cloud Library Sync section with Location of Files path and Browse button]

### Downloading music files

#### Selecting [Synchronization method] > [Synchronize selected music files only (Recommended)]

You can download only files selected from music files that are uploaded to the cloud storage, but not stored on your computer (music files with cloud download icon in the track list of [Collection]).

1. Select uploaded tracks form the track list.

2. Right-click the track, and then select [Cloud Library Sync] > [Download tracks].
   The music files are downloaded from the cloud storage to your computer.

**Hint**

- When you select a playlist, and then select [Download tracks], all music files included in the playlist will be downloaded to your computer.

#### Selecting [Synchronization method] > [Synchronize all music files]

Music files uploaded from your computer are automatically downloaded to other computers by the cloud storage service desktop application. Manual operation is not necessary to download files.

### Moving music files to the local folder on your computer

You can move uploaded music files to the local folder on your computer to increase the free space on the cloud storage.

1. Select uploaded tracks from the track list.

2. Right-click the track, and then select [Cloud Library Sync] > [Move tracks to local storage].
   The music files are moved from the Dropbox folder/Google Drive folder to the local folder on your computer.

**Hint**

- When you select a playlist, and then select [Move tracks to local storage], all music files included in the playlist will be moved to the local folder on your computer.

### Deleting music files from local folder on your computer

You can delete music files from your computer to increase the free space on your computer's storage.

1. Select uploaded tracks form the track list.

2. Right-click the track, and then select [Cloud Library Sync] > [Delete tracks from local storage].
   The music files are deleted from the local folder on your computer.

## Library synchronization

### Checking the library synchronization status

Edits (playlist edits, HOT CUE/Memory CUE edits, etc.) made with rekordbox on other computers and your mobile devices are automatically synchronized to this computer via the Internet.
An icon is displayed to indicate the Cloud Library Sync synchronization status in the taskbar on Windows or in the menu bar on Mac.

[Screenshot: Windows taskbar showing system tray icons with date 2022/12/26 and time 15:56]

- Sync icon (normal): Library is up to date.
- Sync icon (arrows): Library is being synchronized.
- Sync icon (paused): Library synchronization is paused.
- Sync icon (disconnected): Your computer is not connected to the Internet.

### Pausing/restarting library synchronization

If your library is being synchronized with a lot of data using Cloud Library Sync, rekordbox may respond slow depending on your computer and network environment.
In this case, pausing synchronization may solve the issue.

#### Pausing synchronization

Right-click the rekordboxAgent icon that appears in the taskbar on Windows or in the menu bar on Mac and select [Cloud Library Sync] > [Pause sync].

[Screenshot: Windows taskbar context menu showing Display rekordbox notifications, Notification settings, Notification sound settings, Cloud Library Sync submenu with Sync now and Pause sync options, and Quit rekordbox]

**Note**

- Changes made in rekordbox library after you pause synchronization will not be synchronized to rekordbox on other devices until you restart synchronization.

#### Restarting synchronization

Right-click the rekordboxAgent icon that appears in the taskbar on Windows or in the menu bar on Mac and select [Cloud Library Sync] > [Sync now].

**Note**

- Even if you pause synchronization, synchronization will restart when you reboot your computer.

## [Cloud Library Sync] in the [Preferences] window

By opening the [Preferences] window > [CLOUD] category > [Library Sync] tab, you can operate the following settings.

[Screenshot: Preferences window showing Library Sync tab under CLOUD category with Synchronization method section (Synchronize selected music files only / Synchronize all music files), Collection Auto Upload with Disable toggle, Operation for uploading tracks with three radio options (Copy music files, Move music files, Confirm operation for every upload), and Cloud Library Sync section with Location of Files path and Browse button]

1. Set to copy or move music files when uploading them to the cloud storage.
2. Set to open the [Upload tracks] dialog for selecting [Move] or [Copy] when uploading a music file to the cloud storage.
3. You can specify the file save destination in the following cases.
   - The case where uploaded music file cannot be moved to its original location (e.g. when the upload source drive no longer exists)
   - The case where you select a music file from the cloud storage and download it (when you perform [Download tracks] or [Move tracks to local storage] by the [Synchronize selected music files only (Recommended)] method)

# rekordbox for iOS/Android

Use Cloud Library Sync with rekordbox for iOS/Android on your mobile device.
When you use Cloud Library Sync with rekordbox for iOS/Android, Cloud Library Sync uses the Dropbox cloud storage service in the same way as when you use rekordbox on Mac/Windows. You do not need to install a cloud storage service application such as the Dropbox desktop application other than rekordbox.

## Synchronizing your library

By synchronizing your library using Cloud Library Sync, you can upload/download/delete a music file on both your mobile device and the cloud storage.

**Hint**

- When you activate on rekordbox for iOS/Android ("Activation for computers/mobile devices" (page 7)), the screen in step 1 is displayed.

**Note**

- Accept the Dropbox Terms of Service and create a Dropbox account to use Dropbox.
  https://www.dropbox.com/en/privacy#terms

1. Turn on [Cloud Library Sync] on the [Account Information] screen of rekordbox to synchronize your library.

[Screenshot: rekordbox for iOS/Android Account Information screen showing Email address, Subscription Plan as Creative (Valid until 11.28, 2020), Activate toggle on, Cloud Library Sync toggle on, description text about automatic syncing, Cloud Storage Services option, and Log out button]

2. Log in to Dropbox when the Dropbox login screen is displayed.

## Adding a music file to Trial playlist to upload

Free Plan Core Plan

When you add tracks to [Trial playlist - Cloud Library Sync], they will be automatically uploaded to the cloud storage.
You can add up to 20 tracks.

[Screenshot: rekordbox for iOS/Android Playlists screen showing Trial playlist - Cloud Libra... with cloud sync icon and count of 10]

### Downloading a music file

You can download a music file in the track list of [Collection] with cloud download icon from the cloud storage to your mobile device.

1. Select a track from the track list to download and swipe it to the left.
   [...] appears to the right of the swiped track.

[Screenshot: rekordbox for iOS/Android Trial playlist screen showing track list with Search bar, tracks including Mystify (Original Mix) by GotSome, Green plants by Mata Jones Twofalls, 6AM (Original Mix) by Juliet Fox, Friends (Original Mix) by Capa, Wanna (Original Mix) by Catchment, Extra Trippy (Original Mix) by Danny Howard, Get Down (Original Mix) by Eli Brown, Trees (Original Mix) by Felon, Feel Alright (Original Mix) by Franky Rizardo, Nexo (Original Mix) by Gai Barone, with cloud upload/download icons and a now playing bar showing Mystify (Original Mix) by GotSome]

2. Tap [...].
   The menu is displayed.

3. Select [Download from Cloud] in the menu.
   The music file will be downloaded from the cloud storage.

## Uploading/downloading/deleting a music file

Creative Plan Professional Plan Cloud Option

### Upload and download status of music files

The track list of the [Collection] shows the cloud storage upload status of each track, including music files uploaded and not uploaded to the cloud storage from this mobile device, music files not uploaded from your computers or other mobile devices, etc.
The icons displayed on the track list indicate the following statuses:

- Cloud icon (upload arrow): A track that can be uploaded from the mobile device
- Cloud icon (download arrow): A track that can be downloaded to the mobile device

**Hint**

- A white track title indicates that the music file is stored on the mobile device.
- A gray track title indicates that the music file is not stored on the mobile device.

### Uploading a music file

You can upload a music file in the track list of [Collection] with cloud upload icon from your mobile device to the cloud storage.

1. Select a track from the track list to upload and swipe it to the left.
   [...] appears to the right of the swiped track.

[Screenshot: rekordbox for iOS/Android Collection screen showing track list with Search bar, tracks including Sirus Hood, Serenade (Ki Creighton & Makanan...) by Sirus Hood, Serenade (RSMASTER) by Sirus Hood, On The Corner (Original Mix) by Solardo, In All The Fire (Original Mix) by Trilicker with swiped state showing menu, Say It To Me (Original Mix) by Weiss & Christian Nielsen, Wolf by Mata Jones Twofalls, Workin' feat. Leela D (Original...) by Worthy & option4, with cloud upload icons and now playing bar]

2. Tap [...].
   The menu is displayed.

3. Select [Upload to Cloud] in the menu.
   The music file will be uploaded to the cloud storage.

### Automatically uploading music files included in the playlist

When you turn on [Enable Auto Upload] for the playlist, music files will be automatically uploaded to the cloud storage simply by adding them to the playlist.

1. Swipe the playlist to the left.
   [...] appears to the right of the swiped playlist.

[Screenshot: rekordbox for iOS/Android Playlist screen showing Untitled Playlist swiped to the left revealing menu button, with now playing bar showing Friends (Original Mix) by Capa]

2. Tap [...].
   The menu is displayed.

3. Select [Enable Auto Upload] in the menu.
   Music files included in the playlist and stored on the mobile device will be automatically uploaded to the cloud storage.

### Automatically uploading music files imported to the [Collection]

Professional Plan Cloud Option

When you turn on [Collection Auto Upload], all music files imported to the [Collection] will be automatically uploaded to the cloud storage.

[Screenshot: rekordbox for iOS/Android Account Information screen showing Email address, Subscription Plan as Professional Plan (Valid until 05.28, 2022), Activate toggle on, Cloud Library Sync toggle on, description text about automatic syncing, Cloud Storage Services option, Collection Auto Upload toggle on with description text, and Uploads files only when connected to a Wi-Fi network toggle on, Log out button]

### Uploading music files only when connected to a Wi-Fi network

Professional Plan Cloud Option

For the setting of [Collection Auto Upload], you can select whether to upload a music file only when connected to a Wi-Fi network or to upload it during mobile data communication.

[Screenshot: rekordbox for iOS/Android Account Information screen showing same layout as previous with Uploads files only when connected to a Wi-Fi network toggle highlighted]

### Downloading a music file

You can download a music file in the track list of [Collection] with cloud download icon from the cloud storage to your mobile device.

1. Select a track from the track list to download and swipe it to the left.
   [...] appears to the right of the swiped track.

[Screenshot: rekordbox for iOS/Android Collection screen showing track list with tracks including Friends (Original Mix) by Capa, Wanna (Original Mix) by Catchment, Extra Trippy (Original Mix) by Danny Howard, Get Down (Original Mix) by Eli Brown with swiped state showing menu, Trees (Original Mix) by Felon, Feel Alright (Original Mix) by Franky Rizardo, Nexo (Original Mix) by Gai Barone, Mystify (Original Mix) by GotSome, with cloud upload/download icons and now playing bar showing Wolf by Mata Jones Twofalls]

2. Tap [...].
   The menu is displayed.

3. Select [Download from Cloud] in the menu.
   The music file will be downloaded from the cloud storage.

### Deleting a music file from your mobile device

You can delete a music file downloaded from the cloud storage to increase the free space on your mobile device storage.

1. Select a track from the track list to delete and swipe it to the left.
   [...] appears to the right of the swiped track.

[Screenshot: rekordbox for iOS/Android Collection screen showing track list with tracks including Trees (Original Mix) by Felon, Feel Alright (Original Mix) by Franky Rizardo, Nexo (Original Mix) by Gai Barone, Mystify (Original Mix) by GotSome with swiped state showing menu, Green plants by Mata Jones Twofalls, 6AM (Original Mix) by Juliet Fox, Can't Stop Won't Stop (Original Mix) by Karuva, with now playing bar showing Workin' feat. Leela D by Worthy & option4]

2. Tap [...].
   The menu is displayed.

3. Select [Delete Song files] in the menu.
   The music file will be deleted from your mobile device, and the status of the deleted music file will change to not downloaded.

### Library synchronization

Edits (playlist edits, HOT CUE/Memory CUE edits, etc.) made with rekordbox on your computers and other mobile devices are automatically synchronized to this mobile device via the Internet.

# Others

## Troubleshooting

Before making inquiries about operations or technical issues, refer to troubleshooting below, or check the [FAQ] on the rekordbox website.
rekordbox.com/en/support/faq/

### There is not enough free space on the cloud storage.

- Change the cloud storage service plan of your personal subscription to increase the capacity of the cloud storage, or move some music files to the local folder on your computer with rekordbox for Mac/Windows that was used to upload the music files to the cloud storage.
- By subscribing to Professional Plan and joining Professional DJ team, you can have a Dropbox account with the capacity up to 5 TB at no additional charge.
- By subscribing to Cloud Option and joining Cloud Option DJ team, you can have a Dropbox account with the capacity up to 1 TB.

### There is not enough free space on the HDD or SSD of your computer.

- Connect an external HDD to your computer and move the Dropbox folder to the external HDD.
  For information on how to move the Dropbox folder to the external HDD, see the Dropbox website below.
  https://help.dropbox.com/installs-integrations/desktop/move-dropbox-folder
- If you cannot obtain an external HDD, launch the Dropbox desktop application, access the Preferences menu, open [Sync] > [Selective Sync], and then cancel synchronization of a specific folder in the rekordbox folder of Dropbox.
  For information on how to use Selective Sync of Dropbox, see the Dropbox website below.
  https://help.dropbox.com/installs-integrations/sync-uploads/selective-sync-overview

## Trademarks and licenses

- rekordbox is a trademark or registered trademark of AlphaTheta Corporation.
- Dropbox is a trademark or registered trademark of Dropbox, Inc.
- Windows is a trademark or registered trademark of Microsoft Corporation in the U.S. and other countries.
- Mac and macOS are trademarks of Apple Inc., registered in the U.S. and other countries and regions.
- iOS is a trademark containing Cisco trademark signs in the U.S. and other countries.
- "Google", the "Google Logo", and "Google Drive" are trademarks or registered trademarks of Google LLC.
- Android is a trademark or registered trademark of Google LLC.

Other product, technology and company names, etc. mentioned herein are trademarks or registered trademarks of their respective owners.

(C) 2024 AlphaTheta Corporation.

## Related Documents

- [features/cloud-setup-guide.md](../features/cloud-setup-guide.md) (backup, cloud, library, onelibrary)
- [faq/library-and-collection.md](../faq/library-and-collection.md) (collection, library, playlists)
- [features/overview.md](../features/overview.md) (cloud, collection, library)
- [manual/04-management.md](../manual/04-management.md) (backup, collection, library)
- [faq/cloud-and-sync.md](../faq/cloud-and-sync.md) (cloud, onelibrary)
- [faq/usb-and-devices.md](../faq/usb-and-devices.md) (backup, mobile)
- [features/whats-new-v7.md](../features/whats-new-v7.md) (cloud, onelibrary)
- [guides/device-library-backup.md](device-library-backup.md) (backup, library)
