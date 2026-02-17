---
id: xml-format-spec
title: "XML Format List"
type: guide
source:
  file: "xml_format_list.pdf"
  pages: "1"
  version: "7.x"
topics: [export, import, metadata, xml]
modes: [common]
confidence: verified
last_verified: "2026-02-17"
transcribed_by: agent
verified_by: agent
---

# XML file format for playlists sharing (includes all playlists information)

| Name of element/attribute | Description | Expected data type and format |
|---|---|---|
| DJ_PLAYLISTS | | |
| &nbsp;&nbsp;Version | Version of the XML format for share the playlists | utf-8 string | The latest version is 1,0,0. |
| &nbsp;&nbsp;PRODUCT | | |
| &nbsp;&nbsp;&nbsp;&nbsp;Name | Name of product | utf-8 string | This name will be displayed in each application software. |
| &nbsp;&nbsp;&nbsp;&nbsp;Version | Version of application | utf-8 string |
| &nbsp;&nbsp;&nbsp;&nbsp;Company | Name of company | utf-8 string |
| &nbsp;&nbsp;COLLECTION | | | The informations of the tracks who are not included in any playlist are unnecessary. |
| &nbsp;&nbsp;&nbsp;&nbsp;Entries | Number of TRACK in COLLECTION | signed 32bit integer |
| &nbsp;&nbsp;&nbsp;&nbsp;TRACK | | | "Location" is essential for each track ; |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;TrackID | Identification of track | signed int |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Name | Name of track | utf-8 string |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Artist | Name of artist | utf-8 string |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Composer | Name of composer (or producer) | utf-8 string |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Album | Name of Album | utf-8 string |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Grouping | Name of goupe | utf-8 string |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Genre | Name of genre | utf-8 string |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Kind | Type of audio file | utf-8 string |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Size | Size of audio file | signed 64bit integer | Unit : Octet |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;TotalTime | Duration of track | 64bit float value | Unit : Second (without decimal numbers) |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;DiscNumber | Order number of the disc of the album | signed 32bit integer |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;TrackNumber | Order number of the track in the album | signed 32bit integer |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Year | Year of release | signed 32bit integer |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;AverageBpm | Value of average BPM | 64bit float value | Unit : Second (with decimal numbers) |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;DateModified | Date of last modification | utf-8 string (formatted) | Format : yyyy-mm-dd ; ex. : 2010-08-21 |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;DateAdded | Date of addition | utf-8 string | Format : yyyy-mm-dd ; ex. : 2010-08-21 |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;BitRate | Encoding bit rate | signed 32bit integer | Unit : Kbps |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;SampleRate | Frequency of sampling | 64bit float value | Unit : Hertz |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Comments | Comments | utf-8 string |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;PlayCount | Play count of the track | signed 32bit integer |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;LastPlayed | Date of last playing | utf-8 string (formatted) | Format : yyyy-mm-dd ; ex. : 2010-08-21 |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Rating | Rating of the track | signed 32bit integer | 0 star = "0", 1 star = "51", 2 stars = "102", 3 stars = "153", 4 stars = "204", 5 stars = "255" |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Location | Location of the file | utf-8 string (URI formatted) | includes the file name |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Remixer | Name of remixer | utf-8 string |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Tonality | Tonality (Kind of musical key) | utf-8 string |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Label | Name of record label | utf-8 string |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Mix | Name of mix | utf-8 string |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Colour | Colour for track grouping | utf-8 string (formatted) | RGB format (3 bytes) ; rekordbox : Rose(0xFF007F), Red(0xFF0000), Orange(0xFFA500), Lemon(0xFFFF00), Green(0x00FF00), Turquoise(0x25FDE9), Blue(0x0000FF), Violet(0x660099) |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;TEMPO | | | For BeatGrid; More than two "TEMPO" can exist for each track |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Inizio | Start position of BeatGrid | 64bit float value | Unit : Second (with decimal numbers) |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Bpm | Value of BPM | 64bit float value | Unit : Second (with decimal numbers) |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Metro | Kind of musical meter | utf-8 string (formatted) | ex. 3/4, 4/4, 7/8... |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Battito | Beat number in the bar | signed 32bit integer | If the value of "Metro" is 4/4, the value should be 1, 2, 3 or 4. |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;POSITION_MARK | | | More than two "POSITION MARK" can exist for each track |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Name | Name of position mark | utf-8 string |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Type | Type of position mark | signed 32bit integer | Cue = "0", Fade-In = "1", Fade-Out = "2", Load = "3", Loop = "4" |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Start | Start position of position mark | 64bit float value | Unit : Second (with decimal numbers) |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;End | End position of position mark | 64bit float value | Unit : Second (with decimal numbers) |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Num | Number for identification of the position mark | signed 32bit integer | rekordbox : Hot Cue A, B, C : "0", "1", "2"; Memory Cue : "-1" |
| &nbsp;&nbsp;PLAYLISTS | | |
| &nbsp;&nbsp;&nbsp;&nbsp;NODE | | | Root Folder |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Type | Type of NODE | signed 32bit integer | "0" (FOLDER) | These elements are essential for "PLAYLISTS". |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Name | Name of NODE | utf-8 string | ("ROOT") |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Count | Number of NODE in the NODE | signed 32bit integer |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;NODE | | |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Type | Type of NODE | signed 32bit integer | "0" (FOLDER) or "1" (PLAYLIST) |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Name | Name of NODE | utf-8 string |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;(if "Type" == "1") | | |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Entries | Number of TRACK in PLAYLIST | signed 32bit integer |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;KeyType | Kind of identification | signed 32bit integer | "0" (Track ID) or "1"(Location) |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;TRACK | | |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Key | Identification of track | signed 32bit integer or utf-8 string | "Track ID" or "Location" in "COLLECTION" |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;(if "Type" == "0") | | |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;Count | | signed 32bit integer |
| &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;... | | |

**NB**

- all 'string' fields are encoded using UTF8 encoding, with XML entities encoded (&, <, >, ' and ")
- numeric fields are encoded using integer or floating point values, which should be 'locale' independent (no space between digits, dot or comma decimal separator for floating point (e.g. "321453.16312") )
- 'LOCATION' field is encoded as an URI; expected media to be located in 'file://localhost/'

## Related Documents

- [reference/developer-integration.md](../reference/developer-integration.md) (export, import, metadata, xml)
- [reference/xml-import-export.md](../reference/xml-import-export.md) (export, import, metadata, xml)
- [faq/library-and-collection.md](../faq/library-and-collection.md) (export, import, metadata)
- [manual/09-collaborative-playlists.md](../manual/09-collaborative-playlists.md) (export, import, xml)
- [manual/03-adding-tracks.md](../manual/03-adding-tracks.md) (import, xml)
- [manual/07-playlists.md](../manual/07-playlists.md) (export, import)
- [faq/cloud-and-sync.md](../faq/cloud-and-sync.md) (export)
- [faq/hardware-compatibility.md](../faq/hardware-compatibility.md) (export)
