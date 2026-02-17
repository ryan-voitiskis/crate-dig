---
id: developer-integration
title: "Developer Integration"
type: reference
source:
  url: "https://rekordbox.com/en/support/developer/"
  version: "7.x"
topics: [export, import, library, metadata, xml]
modes: [common]
confidence: pending
last_verified: null
transcribed_by: agent
verified_by: null
---

# rekordbox for Developers

## XML playlist

You can now display your rekordbox playlists in the [Bridge] pane by importing playlist information from an XML file.

1. Create an XML file and save it in your desired location.

2. Open the previously created XML file in a text editor. The first row should display as:

   ```xml
   <?xml version="1.0" encoding="UTF-8" ?>
   ```

   In order to save playlists and their information to rekordbox, all rows from the second row and beyond must follow a format which rekordbox supports. Please refer to [a list of XML formats which rekordbox supports (PDF)](https://cdn.rekordbox.com/files/20200410160904/xml_format_list.pdf).

3. Start up rekordbox and select the generated XML file you want to import under [File] > [Preferences] > [Bridge] > [Imported Library].

Multiple playlists or playlist folders with the same name can't exist in the same level of a directory.

## Support

### Ask the forum

[View forum](https://community.pioneerdj.com/hc/en-us/community/topics)

### Inquiries

[Make an Inquiry](https://forums.pioneerdj.com/hc/en-us/requests/new?ticket_form_id=72145)

## Related Documents

- [reference/xml-import-export.md](xml-import-export.md) (export, import, library, metadata)
- [faq/library-and-collection.md](../faq/library-and-collection.md) (export, import, library, metadata)
- [guides/xml-format-spec.md](../guides/xml-format-spec.md) (export, import, metadata, xml)
- [manual/03-adding-tracks.md](../manual/03-adding-tracks.md) (import, library, xml)
- [manual/09-collaborative-playlists.md](../manual/09-collaborative-playlists.md) (export, import, xml)
- [features/overview.md](../features/overview.md) (export, library)
- [guides/device-library-backup.md](../guides/device-library-backup.md) (export, library)
- [guides/introduction.md](../guides/introduction.md) (export, library)
