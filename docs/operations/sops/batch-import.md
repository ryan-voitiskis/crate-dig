# Batch Import Preparation — Agent SOP

Process newly acquired music (downloaded albums, loose tracks, zips) into an organized structure ready for Rekordbox import. Follow step-by-step.

**Goal:** After this SOP, any processed track can be imported into Rekordbox with Artist, Title, Album, Year, Track Number, and Label displaying correctly. Genre is handled separately via the genre classification SOP.

## Rekordbox Import Readiness

<!-- dprint-ignore -->
| Field | FLAC | WAV | MP3 |
|-------|------|-----|-----|
| Artist | Vorbis Comment | RIFF INFO (tag 3) | ID3v2 (tag 2) |
| Title | Vorbis Comment | RIFF INFO (tag 3) | ID3v2 (tag 2) |
| Album | Vorbis Comment | RIFF INFO (tag 3) | ID3v2 (tag 2) |
| Year | Vorbis Comment | RIFF INFO (tag 3) | ID3v2 (tag 2) |
| Track | Vorbis Comment | RIFF INFO (tag 3) | ID3v2 (tag 2) |
| Cover art | Embedded (auto) | **Not imported** | Embedded (auto) |

**WAV critical:** Rekordbox reads **only** RIFF INFO (tag 3) from WAV files. ID3v2 (tag 2) is ignored. Both must be written — tag 2 for general compatibility, tag 3 for Rekordbox.

**WAV cover art:** Rekordbox cannot import cover art from WAV files. Embed in tag 2 for other apps, but note WAV tracks need manual cover art in Rekordbox after import.

## Constraints

- **Tags are source of truth.** Write tags before renaming — filenames are derived from tags.
- **Never set genre.** Leave genre tags empty; handled via genre classification SOP.
- **Stop on ambiguity.** Never guess artist names, album years, or label names — ask the user.
- **Process album by album.** Complete one fully before starting the next.
- **WAV dual-tag rule.** Always write both tag 2 and tag 3 for WAV files.
- **Verify before moving.** Confirm tags and filenames before moving to final location.

## Prerequisites

<!-- dprint-ignore -->
| Tool | Purpose | Install |
|------|---------|---------|
| `kid3-cli` | Tag writing, file renaming, cover art embedding | `brew install kid3` |
| `exiftool` | Tag reading as JSON | `brew install exiftool` |
| `reklawdbox` MCP | Discogs/Beatport metadata lookups | This project |
| `unzip` | Extract zip archives | Pre-installed on macOS |

`lookup_discogs` and `lookup_beatport` are MCP tool calls, not shell commands.

**Shell note:** Claude Code does not persist shell state between tool calls. All shell snippets below use literal paths — substitute the actual path for each invocation.

## Convention Reference

### Target structure

**Single artist albums:**

```
destination/
└── Artist Name/
    └── Album Name (Year)/
        ├── 01 Artist Name - Track Title.flac
        └── cover.jpg
```

**Various Artists compilations:**

```
destination/
└── Various Artists/
    └── Label Name/
        └── Album Name (Year)/
            ├── 01 Artist A - Track Title.flac
            └── cover.jpg
```

**Loose tracks (play directories only):**

```
destination/
├── Artist Name - Track Title.wav
└── cover_Artist Name - Track Title.jpg
```

### File naming

- **Album tracks:** `NN Artist Name - Track Title.ext` (zero-padded, `-` separator)
- **Loose tracks:** `Artist Name - Track Title.ext` (no track number)

### Required tags

<!-- dprint-ignore -->
| Tag | Album tracks | Loose tracks |
|-----|-------------|-------------|
| Artist | Required | Required |
| Title | Required | Required |
| Track | Required | Not needed |
| Album | Required | Optional |
| Date/Year | Required | Optional |
| Publisher | Recommended (required for VA) | Optional |
| AlbumArtist | Required for VA | Not needed |

### Album directory naming

- Format: `Album Name (Year)/`
- No artist name in album directory
- Remove: `[FLAC]`, `[WAV]`, `24-96`, usernames, catalog numbers
- Preserve: `(Deluxe Edition)`, `Vol. 1`, `(Remastered)`
- Replace `/` with `-`, `:` with `-`

### Album type classification

<!-- dprint-ignore -->
| Pattern | Type | Directory structure |
|---------|------|-------------------|
| All tracks same artist | Single Artist | `Artist/Album (Year)/` |
| Different artists per track | VA Compilation | `Various Artists/Label/Album (Year)/` |
| Multiple named artists (A & B) | Collaboration | `Various Artists/Label/Album (Year)/` |

---

## Phase 1: Assessment

### Step 1: List the batch directory

```sh
ls -la "/path/to/batch/"
```

Categorize: directories (albums/EPs), audio files at root (loose tracks), zip files (need extraction).

### Step 2: Handle zip files

Extract all root-level zips. Already-extracted zips (matching directory exists with files) are archived without re-extraction. Failed extractions go to `_failed_zips/`.

```sh
cd "/path/to/batch"
mkdir -p "_processed_zips" "_failed_zips"
find . -maxdepth 1 -type f -name "*.zip" -print0 | while IFS= read -r -d '' zip; do
    zip="${zip#./}"
    dir="${zip%.zip}"

    if [ -d "$dir" ] && find "$dir" -type f -print -quit | grep -q .; then
        mv "$zip" "_processed_zips/$zip"
        echo "Archived already-extracted: $zip"
        continue
    fi

    tmp_dir="$(mktemp -d "${TMPDIR:-/tmp}/batch-import-unzip.XXXXXX")"
    if unzip -o "$zip" -d "$tmp_dir"; then
        mkdir -p "$dir"
        find "$tmp_dir" -mindepth 1 -maxdepth 1 -exec mv -n {} "$dir"/ \;
        if find "$dir" -type f -print -quit | grep -q .; then
            mv "$zip" "_processed_zips/$zip"
            echo "Extracted: $zip"
        else
            mv "$zip" "_failed_zips/$zip"
            echo "No files extracted: $zip"
        fi
    else
        mv "$zip" "_failed_zips/$zip"
        echo "Extraction failed: $zip"
    fi
    rm -rf "$tmp_dir"
done
```

### Step 3: Report to user

Summarize what was found: album directories, loose tracks, zip results.

---

## Phase 2: Process Albums

For each album subdirectory, follow these steps in order.

### Step 1: Survey current state

```sh
ls -la "/path/to/batch/Album Directory/"

cd "/path/to/batch/Album Directory"
exiftool -j -ext flac -ext wav -ext mp3 \
         -Artist -Title -Album -Year -TrackNumber -Publisher .
```

Note: current filename pattern, which tags are present, whether cover art exists.

### Step 2: Parse directory name

Common incoming patterns:

- `Artist Name - Album Name`
- `Artist Name - Album Name (Year)`
- `Artist Name - Album Name [FLAC 24-96]`

Extract: artist, album name, year (if present).

### Step 3: Determine album type

- **Same artist on all tracks** → Single Artist
- **Different artists per track** → VA Compilation
- **"Various Artists" in dir name** → VA Compilation

For VA: label name is **required**. Check Publisher tag, directory name, or look up.

### Step 4: Look up metadata

```
lookup_discogs(artist="Artist Name", title="First Track Title", album="Album Name")
```

If no Discogs result, try Beatport:

```
lookup_beatport(artist="Artist Name", title="First Track Title")
```

Use results for: release year, label name, artist/album spelling verification.

**Stop and ask** on: multiple matches with different years, no results and year/label unknown, ambiguous artist.

Never use lookup results for genre.

### Step 5: Write tags

**Album-wide tags** (FLAC/MP3):

```sh
cd "/path/to/album"
kid3-cli -c "select all" -c "tag 2" \
         -c "set album 'Album Name'" \
         -c "set date YEAR" \
         -c "set publisher 'Label Name'" \
         -c "set album artist 'Artist Name'" \
         -c "save" .
```

**Album-wide tags** (WAV — both tag 2 and tag 3):

```sh
kid3-cli -c "select all" \
         -c "tag 2" \
         -c "set album 'Album Name'" \
         -c "set date YEAR" \
         -c "set publisher 'Label Name'" \
         -c "set album artist 'Artist Name'" \
         -c "tag 3" \
         -c "set album 'Album Name'" \
         -c "set date YEAR" \
         -c "set publisher 'Label Name'" \
         -c "set album artist 'Artist Name'" \
         -c "save" .
```

**Per-track tags** — parse from filenames. Common incoming patterns:

<!-- dprint-ignore -->
| Pattern | Parse as |
|---------|----------|
| `Artist - Album - NN Title.wav` | Track N: Artist - Title |
| `NN Artist - Title.wav` | Track N: Artist - Title |
| `NN Title.wav` | Track N: [AlbumArtist] - Title |
| `NN. Title.wav` | Track N: [AlbumArtist] - Title |

For each track (FLAC/MP3):

```sh
kid3-cli -c "select 'original-filename.flac'" -c "tag 2" \
         -c "set artist 'Track Artist'" \
         -c "set title 'Track Title'" \
         -c "set track number N" \
         -c "save" .
```

For WAV, write the same fields to both tag 2 and tag 3.

### Step 6: Verify tags

```sh
exiftool -j -ext flac -ext wav -ext mp3 \
         -Artist -Title -Album -Year -TrackNumber .
```

Confirm every file has: Artist, Title, Track, Album, Year.

### Step 7: Rename files from tags

```sh
kid3-cli -c "select all" \
         -c "fromtag '%{track} %{artist} - %{title}' 2" \
         -c "save" .
```

Expected result: `01 Artist Name - Track Title.ext`

If rename produces unexpected results, stop and check tags — rename depends entirely on tag correctness.

### Step 8: Embed cover art

```sh
find . -maxdepth 1 -type f \( -iname "*.jpg" -o -iname "*.jpeg" -o -iname "*.png" \) -print
```

**Single obvious cover** (`cover.jpg`, `front.jpg`, `folder.jpg`):

```sh
kid3-cli -c "select all" \
         -c "set picture:'cover.jpg' 'Cover (front)' ''" \
         -c "save" .
```

**Multiple images:** Ask user which is the cover.

**No images:** Try `lookup_discogs(...)` — if result includes `cover_image`, download and embed:

```sh
curl -s -o "cover.jpg" "COVER_IMAGE_URL"
kid3-cli -c "select all" \
         -c "set picture:'cover.jpg' 'Cover (front)' ''" \
         -c "save" .
```

If no cover from Discogs either, note for user to source manually.

### Step 9: Create target directory and move files

Determine clean directory name (strip tech specs, add year, clean special chars).

```sh
# Single Artist
mkdir -p "/path/to/dest/Artist Name/Album Name (Year)"

# Or VA
mkdir -p "/path/to/dest/Various Artists/Label Name/Album Name (Year)"

# Move audio + cover art
find "/path/to/batch/Old Dir" -maxdepth 1 -type f \
     \( -iname "*.wav" -o -iname "*.flac" -o -iname "*.mp3" \) \
     -exec mv -n {} "/path/to/dest/Artist Name/Album Name (Year)/" \;
find "/path/to/batch/Old Dir" -maxdepth 1 -type f -iname "cover.*" \
     -exec mv -n {} "/path/to/dest/Artist Name/Album Name (Year)/" \;

# Remove old empty directory
rmdir "/path/to/batch/Old Dir"
```

### Step 10: Verify final state

```sh
ls -la "/path/to/dest/Artist Name/Album Name (Year)/"
kid3-cli -c "get" "/path/to/dest/Artist Name/Album Name (Year)/01 Artist - Track.ext"
```

Confirm: files in correct location, `NN Artist - Title.ext` format, all tags present, VA has label subdirectory, cover art embedded.

---

## Phase 3: Process Loose Tracks

For audio files at the root of the batch directory.

### Step 1: Clean filenames

Remove `(Original Mix)` suffix. Keep other parenthetical info (`(Remix)`, `(Edit)`, etc.):

```sh
cd "/path/to/batch"
find . -maxdepth 1 -type f -name "* (Original Mix).*" -print0 | while IFS= read -r -d '' f; do
    new_name="${f/ (Original Mix)/}"
    mv "$f" "$new_name"
done
```

Expected format: `Artist Name - Track Title.ext`. If unparseable, ask user.

### Step 2: Read/write tags

For each loose track, read existing tags:

```sh
kid3-cli -c "get" "/path/to/batch/Artist - Title.wav"
```

If tags are missing, look up with `lookup_discogs(...)` / `lookup_beatport(...)`.

Write minimum tags (FLAC/MP3):

```sh
kid3-cli -c "select 'Artist - Title.flac'" -c "tag 2" \
         -c "set artist 'Artist Name'" \
         -c "set title 'Track Title'" \
         -c "set publisher 'Label Name'" \
         -c "save" "/path/to/batch"
```

For WAV, write same fields to both tag 2 and tag 3.

### Step 3: Embed cover art (if available)

Check for `cover_Artist Name - Track Title.jpg` files. If found, embed. If not, try Discogs `cover_image`. Note any tracks without cover art in the report.

---

## Phase 4: Multi-Disc Albums

If an album has disc subdirectories (`CD1/`, `CD2/`, `Disc 1/`, etc.):

- Track numbers restart at 01 per disc
- Cover art at album root (not in disc folders)
- Set Disc Number tag on each track:
  ```sh
  cd "/path/to/album/CD1"
  kid3-cli -c "select all" -c "tag 2" -c "set disc number 1" -c "save" .
  # For WAV, also: -c "tag 3" -c "set disc number 1"
  ```
- Album-wide tags (album, year, publisher) go on all tracks across all discs

---

## Phase 5: Final Verification

Summarize: albums processed (single artist vs VA), loose tracks processed, any unresolved items, WAV tracks needing manual cover art in Rekordbox, and next steps (import, cover art, collection audit SOP, genre classification SOP).

---

## Decision Reference

### Proceed automatically when

- Artist clearly identified in tags, filename, or directory name
- Year present in tags, directory name, or single clear Discogs match
- Label present in tags or Discogs (for VA)
- Single artist with consistent tags across album

### Stop and ask when

- Multiple matches with different years
- No results and year/label unknown
- VA album but label unknown
- Ambiguous: collaboration vs VA vs single artist
- Multiple images — which is album cover?
- Conflicting metadata between tags and filenames
- Unparseable filenames

---

## Common Incoming Filename Patterns

### Album tracks

<!-- dprint-ignore -->
| Pattern | Parse as |
|---------|----------|
| `Artist - Album - NN Title.wav` | Track N: Artist - Title |
| `Artist - Album - NN. Title.wav` | Track N: Artist - Title |
| `NN Artist - Title.wav` | Track N: Artist - Title |
| `NN. Artist - Title.wav` | Track N: Artist - Title |
| `NN Title.wav` | Track N: [AlbumArtist] - Title |
| `NN. Title.wav` | Track N: [AlbumArtist] - Title |
| `Artist - Album - NN AX. Title.wav` | Track N: Artist - Title (vinyl) |

### Loose tracks

<!-- dprint-ignore -->
| Pattern | Status |
|---------|--------|
| `Artist - Title.wav` | Correct |
| `Artist - Title (Remix Info).wav` | Correct |
| `Artist, Artist B - Title.wav` | Correct |
| `Artist - Title (Original Mix).wav` | Remove "(Original Mix)" |
| `Title.wav` | Missing artist — ask user |
