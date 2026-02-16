use rusqlite::{Connection, OpenFlags, params};

use crate::types::{GenreCount, KeyCount, LibraryStats, Playlist, Track, rating_to_stars};

/// The universal Rekordbox 6/7 SQLCipher key (publicly known, same for all installations).
const DB_KEY: &str = "402fd482c38817c35ffa8ffb8c7d93143b749e7d315df7a81732a1ff43608497";

/// Open a read-only connection to the Rekordbox master.db.
pub fn open(path: &str) -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
    conn.pragma_update(None, "key", &format!("x'{DB_KEY}'"))?;
    // Verify the key works by running a simple query
    conn.query_row("SELECT count(*) FROM sqlite_master", [], |_| Ok(()))?;
    Ok(conn)
}

/// Open an unencrypted in-memory DB (for tests).
#[cfg(test)]
pub fn open_test() -> Connection {
    Connection::open_in_memory().unwrap()
}

/// Base SELECT for track queries — joins all lookup tables.
const TRACK_SELECT: &str = "
SELECT
    c.ID,
    COALESCE(c.Title, '') AS Title,
    COALESCE(a.Name, '') AS ArtistName,
    COALESCE(al.Name, '') AS AlbumName,
    COALESCE(g.Name, '') AS GenreName,
    COALESCE(c.BPM, 0) AS BPM,
    COALESCE(k.ScaleName, '') AS KeyName,
    COALESCE(c.Rating, 0) AS Rating,
    COALESCE(c.Commnt, '') AS Comments,
    COALESCE(col.Commnt, '') AS ColorName,
    COALESCE(col.ColorCode, 0) AS ColorCode,
    COALESCE(l.Name, '') AS LabelName,
    COALESCE(ra.Name, '') AS RemixerName,
    COALESCE(c.ReleaseYear, 0) AS ReleaseYear,
    COALESCE(c.Length, 0) AS Length,
    COALESCE(c.FolderPath, '') AS FolderPath,
    COALESCE(c.DJPlayCount, '0') AS DJPlayCount,
    COALESCE(c.BitRate, 0) AS BitRate,
    COALESCE(c.SampleRate, 0) AS SampleRate,
    COALESCE(c.FileType, 0) AS FileType,
    COALESCE(c.created_at, '') AS DateAdded
FROM djmdContent c
LEFT JOIN djmdArtist a ON c.ArtistID = a.ID
LEFT JOIN djmdAlbum al ON c.AlbumID = al.ID
LEFT JOIN djmdGenre g ON c.GenreID = g.ID
LEFT JOIN djmdKey k ON c.KeyID = k.ID
LEFT JOIN djmdLabel l ON c.LabelID = l.ID
LEFT JOIN djmdColor col ON c.ColorID = col.ID
LEFT JOIN djmdArtist ra ON c.RemixerID = ra.ID
";

fn row_to_track(row: &rusqlite::Row) -> Result<Track, rusqlite::Error> {
    let bpm_raw: i32 = row.get("BPM")?;
    let rating_raw: i32 = row.get("Rating")?;
    let play_count_str: String = row.get("DJPlayCount")?;

    Ok(Track {
        id: row.get("ID")?,
        title: row.get("Title")?,
        artist: row.get("ArtistName")?,
        album: row.get("AlbumName")?,
        genre: row.get("GenreName")?,
        bpm: bpm_raw as f64 / 100.0,
        key: row.get("KeyName")?,
        rating: rating_to_stars(rating_raw as u16),
        comments: row.get("Comments")?,
        color: row.get("ColorName")?,
        color_code: row.get("ColorCode")?,
        label: row.get("LabelName")?,
        remixer: row.get("RemixerName")?,
        year: row.get("ReleaseYear")?,
        length: row.get("Length")?,
        file_path: row.get("FolderPath")?,
        play_count: play_count_str.parse().unwrap_or(0),
        bit_rate: row.get("BitRate")?,
        sample_rate: row.get("SampleRate")?,
        file_type: row.get("FileType")?,
        date_added: row.get("DateAdded")?,
    })
}

/// Search parameters for filtering tracks.
pub struct SearchParams {
    pub query: Option<String>,
    pub artist: Option<String>,
    pub genre: Option<String>,
    pub rating_min: Option<u8>,
    pub bpm_min: Option<f64>,
    pub bpm_max: Option<f64>,
    pub key: Option<String>,
    pub playlist: Option<String>,
    pub has_genre: Option<bool>,
    pub limit: Option<u32>,
}

/// Search tracks with dynamic filtering.
pub fn search_tracks(conn: &Connection, params: &SearchParams) -> Result<Vec<Track>, rusqlite::Error> {
    let mut sql = format!("{TRACK_SELECT} WHERE c.rb_local_deleted = 0");
    let mut bind_values: Vec<Box<dyn rusqlite::types::ToSql>> = vec![];

    if let Some(ref q) = params.query {
        let idx = bind_values.len() + 1;
        sql.push_str(&format!(" AND (c.Title LIKE ?{idx} OR a.Name LIKE ?{idx})"));
        bind_values.push(Box::new(format!("%{q}%")));
    }

    if let Some(ref artist) = params.artist {
        let idx = bind_values.len() + 1;
        sql.push_str(&format!(" AND a.Name LIKE ?{idx}"));
        bind_values.push(Box::new(format!("%{artist}%")));
    }

    if let Some(ref genre) = params.genre {
        let idx = bind_values.len() + 1;
        sql.push_str(&format!(" AND g.Name LIKE ?{idx}"));
        bind_values.push(Box::new(format!("%{genre}%")));
    }

    if let Some(rating_min) = params.rating_min {
        let idx = bind_values.len() + 1;
        sql.push_str(&format!(" AND c.Rating >= ?{idx}"));
        let min_rating = crate::types::stars_to_rating(rating_min) as i32;
        bind_values.push(Box::new(min_rating));
    }

    if let Some(bpm_min) = params.bpm_min {
        let idx = bind_values.len() + 1;
        sql.push_str(&format!(" AND c.BPM >= ?{idx}"));
        bind_values.push(Box::new((bpm_min * 100.0) as i32));
    }

    if let Some(bpm_max) = params.bpm_max {
        let idx = bind_values.len() + 1;
        sql.push_str(&format!(" AND c.BPM <= ?{idx}"));
        bind_values.push(Box::new((bpm_max * 100.0) as i32));
    }

    if let Some(ref key) = params.key {
        let idx = bind_values.len() + 1;
        sql.push_str(&format!(" AND k.ScaleName = ?{idx}"));
        bind_values.push(Box::new(key.clone()));
    }

    if let Some(has_genre) = params.has_genre {
        if has_genre {
            sql.push_str(" AND g.Name IS NOT NULL AND g.Name != ''");
        } else {
            sql.push_str(" AND (g.Name IS NULL OR g.Name = '')");
        }
    }

    // Playlist filter: join through djmdSongPlaylist
    if let Some(ref playlist_id) = params.playlist {
        let idx = bind_values.len() + 1;
        sql.push_str(&format!(
            " AND c.ID IN (SELECT sp.ContentID FROM djmdSongPlaylist sp WHERE sp.PlaylistID = ?{idx})"
        ));
        bind_values.push(Box::new(playlist_id.clone()));
    }

    let limit = params.limit.unwrap_or(50).min(200);
    sql.push_str(&format!(" ORDER BY c.Title LIMIT {limit}"));

    let mut stmt = conn.prepare(&sql)?;
    let refs: Vec<&dyn rusqlite::types::ToSql> = bind_values.iter().map(|b| b.as_ref()).collect();
    let rows = stmt.query_map(refs.as_slice(), row_to_track)?;

    rows.collect()
}

/// Get a single track by ID.
pub fn get_track(conn: &Connection, track_id: &str) -> Result<Option<Track>, rusqlite::Error> {
    let sql = format!("{TRACK_SELECT} WHERE c.ID = ?1 AND c.rb_local_deleted = 0");
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query_map(params![track_id], row_to_track)?;
    match rows.next() {
        Some(Ok(track)) => Ok(Some(track)),
        Some(Err(e)) => Err(e),
        None => Ok(None),
    }
}

/// Get all playlists.
pub fn get_playlists(conn: &Connection) -> Result<Vec<Playlist>, rusqlite::Error> {
    let sql = "
        SELECT
            p.ID,
            COALESCE(p.Name, '') AS Name,
            COALESCE(p.ParentID, '') AS ParentID,
            COALESCE(p.Attribute, 0) AS Attribute,
            (SELECT COUNT(*) FROM djmdSongPlaylist sp WHERE sp.PlaylistID = p.ID) AS TrackCount
        FROM djmdPlaylist p
        WHERE p.rb_local_deleted = 0
        ORDER BY p.Seq
    ";
    let mut stmt = conn.prepare(sql)?;
    let rows = stmt.query_map([], |row| {
        let attr: i32 = row.get("Attribute")?;
        Ok(Playlist {
            id: row.get("ID")?,
            name: row.get("Name")?,
            track_count: row.get("TrackCount")?,
            parent_id: row.get("ParentID")?,
            is_folder: attr == 1,
            is_smart: attr == 4,
        })
    })?;
    rows.collect()
}

/// Get tracks in a playlist.
pub fn get_playlist_tracks(
    conn: &Connection,
    playlist_id: &str,
    limit: Option<u32>,
) -> Result<Vec<Track>, rusqlite::Error> {
    let limit = limit.unwrap_or(200).min(200);
    let sql = format!(
        "{TRACK_SELECT}
         INNER JOIN djmdSongPlaylist sp ON sp.ContentID = c.ID
         WHERE sp.PlaylistID = ?1 AND c.rb_local_deleted = 0
         ORDER BY sp.TrackNo
         LIMIT {limit}"
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params![playlist_id], row_to_track)?;
    rows.collect()
}

/// Get library statistics.
pub fn get_library_stats(conn: &Connection) -> Result<LibraryStats, rusqlite::Error> {
    let total_tracks: i32 = conn.query_row(
        "SELECT COUNT(*) FROM djmdContent WHERE rb_local_deleted = 0",
        [],
        |row| row.get(0),
    )?;

    let avg_bpm: f64 = conn
        .query_row(
            "SELECT COALESCE(AVG(BPM), 0) FROM djmdContent WHERE rb_local_deleted = 0 AND BPM > 0",
            [],
            |row| row.get(0),
        )
        .map(|v: f64| v / 100.0)?;

    let rated_count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM djmdContent WHERE rb_local_deleted = 0 AND Rating > 0",
        [],
        |row| row.get(0),
    )?;

    let playlist_count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM djmdPlaylist WHERE rb_local_deleted = 0 AND Attribute != 1",
        [],
        |row| row.get(0),
    )?;

    // Genre distribution
    let mut stmt = conn.prepare(
        "SELECT COALESCE(g.Name, '(none)') AS GenreName, COUNT(*) AS cnt
         FROM djmdContent c
         LEFT JOIN djmdGenre g ON c.GenreID = g.ID
         WHERE c.rb_local_deleted = 0
         GROUP BY g.Name
         ORDER BY cnt DESC",
    )?;
    let genres: Vec<GenreCount> = stmt
        .query_map([], |row| {
            Ok(GenreCount {
                name: row.get(0)?,
                count: row.get(1)?,
            })
        })?
        .collect::<Result<_, _>>()?;

    // Key distribution
    let mut stmt = conn.prepare(
        "SELECT COALESCE(k.ScaleName, '(none)') AS KeyName, COUNT(*) AS cnt
         FROM djmdContent c
         LEFT JOIN djmdKey k ON c.KeyID = k.ID
         WHERE c.rb_local_deleted = 0
         GROUP BY k.ScaleName
         ORDER BY cnt DESC",
    )?;
    let key_distribution: Vec<KeyCount> = stmt
        .query_map([], |row| {
            Ok(KeyCount {
                name: row.get(0)?,
                count: row.get(1)?,
            })
        })?
        .collect::<Result<_, _>>()?;

    Ok(LibraryStats {
        total_tracks,
        genres,
        playlist_count,
        rated_count,
        unrated_count: total_tracks - rated_count,
        avg_bpm,
        key_distribution,
    })
}

/// Get multiple tracks by their IDs.
pub fn get_tracks_by_ids(conn: &Connection, ids: &[String]) -> Result<Vec<Track>, rusqlite::Error> {
    if ids.is_empty() {
        return Ok(vec![]);
    }
    let placeholders: Vec<String> = (1..=ids.len()).map(|i| format!("?{i}")).collect();
    let sql = format!(
        "{TRACK_SELECT} WHERE c.ID IN ({}) AND c.rb_local_deleted = 0",
        placeholders.join(", ")
    );
    let mut stmt = conn.prepare(&sql)?;
    let refs: Vec<&dyn rusqlite::types::ToSql> = ids.iter().map(|s| s as &dyn rusqlite::types::ToSql).collect();
    let rows = stmt.query_map(refs.as_slice(), row_to_track)?;
    rows.collect()
}

/// Detect the default Rekordbox DB path.
pub fn default_db_path() -> Option<String> {
    let home = std::env::var("HOME").ok()?;
    let path = format!("{home}/Library/Pioneer/rekordbox/master.db");
    if std::path::Path::new(&path).exists() {
        Some(path)
    } else {
        None
    }
}

/// Resolve the DB path from env var or auto-detect.
pub fn resolve_db_path() -> Option<String> {
    if let Ok(path) = std::env::var("REKORDBOX_DB_PATH") {
        if std::path::Path::new(&path).exists() {
            return Some(path);
        }
    }
    default_db_path()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Create a test DB with the Rekordbox schema (unencrypted).
    pub fn create_test_db() -> Connection {
        let conn = open_test();
        conn.execute_batch(
            "
            CREATE TABLE djmdArtist (
                ID VARCHAR(255) PRIMARY KEY,
                Name VARCHAR(255),
                rb_local_deleted INTEGER DEFAULT 0
            );
            CREATE TABLE djmdAlbum (
                ID VARCHAR(255) PRIMARY KEY,
                Name VARCHAR(255),
                rb_local_deleted INTEGER DEFAULT 0
            );
            CREATE TABLE djmdGenre (
                ID VARCHAR(255) PRIMARY KEY,
                Name VARCHAR(255),
                rb_local_deleted INTEGER DEFAULT 0
            );
            CREATE TABLE djmdKey (
                ID VARCHAR(255) PRIMARY KEY,
                ScaleName VARCHAR(255),
                rb_local_deleted INTEGER DEFAULT 0
            );
            CREATE TABLE djmdLabel (
                ID VARCHAR(255) PRIMARY KEY,
                Name VARCHAR(255),
                rb_local_deleted INTEGER DEFAULT 0
            );
            CREATE TABLE djmdColor (
                ID VARCHAR(255) PRIMARY KEY,
                ColorCode INTEGER,
                Commnt VARCHAR(255),
                rb_local_deleted INTEGER DEFAULT 0
            );
            CREATE TABLE djmdContent (
                ID VARCHAR(255) PRIMARY KEY,
                Title VARCHAR(255),
                ArtistID VARCHAR(255),
                AlbumID VARCHAR(255),
                GenreID VARCHAR(255),
                KeyID VARCHAR(255),
                ColorID VARCHAR(255),
                LabelID VARCHAR(255),
                RemixerID VARCHAR(255),
                BPM INTEGER DEFAULT 0,
                Rating INTEGER DEFAULT 0,
                Commnt TEXT DEFAULT '',
                ReleaseYear INTEGER DEFAULT 0,
                Length INTEGER DEFAULT 0,
                FolderPath VARCHAR(255) DEFAULT '',
                DJPlayCount VARCHAR(255) DEFAULT '0',
                BitRate INTEGER DEFAULT 0,
                SampleRate INTEGER DEFAULT 0,
                FileType INTEGER DEFAULT 0,
                created_at TEXT DEFAULT '',
                rb_local_deleted INTEGER DEFAULT 0
            );
            CREATE TABLE djmdPlaylist (
                ID VARCHAR(255) PRIMARY KEY,
                Seq INTEGER,
                Name VARCHAR(255),
                Attribute INTEGER DEFAULT 0,
                ParentID VARCHAR(255) DEFAULT '',
                rb_local_deleted INTEGER DEFAULT 0
            );
            CREATE TABLE djmdSongPlaylist (
                ID VARCHAR(255) PRIMARY KEY,
                PlaylistID VARCHAR(255),
                ContentID VARCHAR(255),
                TrackNo INTEGER
            );

            -- Lookup data
            INSERT INTO djmdArtist (ID, Name) VALUES ('a1', 'Burial');
            INSERT INTO djmdArtist (ID, Name) VALUES ('a2', 'Actress');
            INSERT INTO djmdArtist (ID, Name) VALUES ('a3', 'Ricardo Villalobos');
            INSERT INTO djmdAlbum (ID, Name) VALUES ('al1', 'Untrue');
            INSERT INTO djmdAlbum (ID, Name) VALUES ('al2', 'R.I.P.');
            INSERT INTO djmdGenre (ID, Name) VALUES ('g1', 'Dubstep');
            INSERT INTO djmdGenre (ID, Name) VALUES ('g2', 'Techno');
            INSERT INTO djmdGenre (ID, Name) VALUES ('g3', 'Minimal');
            INSERT INTO djmdKey (ID, ScaleName) VALUES ('k1', 'Am');
            INSERT INTO djmdKey (ID, ScaleName) VALUES ('k2', 'Cm');
            INSERT INTO djmdKey (ID, ScaleName) VALUES ('k3', 'Fm');
            INSERT INTO djmdLabel (ID, Name) VALUES ('l1', 'Hyperdub');
            INSERT INTO djmdLabel (ID, Name) VALUES ('l2', 'Ninja Tune');
            INSERT INTO djmdColor (ID, ColorCode, Commnt) VALUES ('c1', 16711935, 'Rose');
            INSERT INTO djmdColor (ID, ColorCode, Commnt) VALUES ('c2', 65280, 'Green');

            -- Tracks
            INSERT INTO djmdContent (ID, Title, ArtistID, AlbumID, GenreID, KeyID, LabelID, ColorID, BPM, Rating, Commnt, ReleaseYear, Length, FolderPath, DJPlayCount, BitRate, SampleRate, FileType, created_at)
            VALUES ('t1', 'Archangel', 'a1', 'al1', 'g1', 'k1', 'l1', 'c1', 13950, 204, 'iconic garage vocal', 2007, 240, '/Users/vz/Music/Burial/Untrue/01 Archangel.flac', '12', 1411, 44100, 5, '2023-01-15');
            INSERT INTO djmdContent (ID, Title, ArtistID, AlbumID, GenreID, KeyID, LabelID, BPM, Rating, ReleaseYear, Length, FolderPath, DJPlayCount, BitRate, SampleRate, FileType, created_at)
            VALUES ('t2', 'Endorphin', 'a1', 'al1', 'g1', 'k2', 'l1', 14000, 153, 2007, 300, '/Users/vz/Music/Burial/Untrue/02 Endorphin.flac', '5', 1411, 44100, 5, '2023-01-15');
            INSERT INTO djmdContent (ID, Title, ArtistID, AlbumID, GenreID, KeyID, BPM, Rating, ReleaseYear, Length, FolderPath, BitRate, SampleRate, FileType, created_at)
            VALUES ('t3', 'R.I.P.', 'a2', 'al2', 'g2', 'k3', 12800, 102, 2012, 360, '/Users/vz/Music/Actress/R.I.P./01 R.I.P..flac', 1411, 44100, 5, '2023-02-20');
            INSERT INTO djmdContent (ID, Title, ArtistID, GenreID, BPM, Length, FolderPath, BitRate, SampleRate, FileType, created_at)
            VALUES ('t4', 'Dexter', 'a3', 'g3', 12500, 480, '/Users/vz/Music/Villalobos/Dexter.wav', 1411, 44100, 11, '2023-03-10');
            INSERT INTO djmdContent (ID, Title, ArtistID, BPM, Length, FolderPath, BitRate, SampleRate, FileType, created_at)
            VALUES ('t5', 'Unknown Track', 'a1', 0, 200, '/Users/vz/Music/unknown.mp3', 320, 44100, 1, '2023-04-01');

            -- Playlists
            INSERT INTO djmdPlaylist (ID, Seq, Name, Attribute, ParentID) VALUES ('p1', 1, 'Deep Cuts', 0, 'root');
            INSERT INTO djmdPlaylist (ID, Seq, Name, Attribute, ParentID) VALUES ('p2', 2, 'Folders', 1, 'root');
            INSERT INTO djmdSongPlaylist (ID, PlaylistID, ContentID, TrackNo) VALUES ('sp1', 'p1', 't1', 1);
            INSERT INTO djmdSongPlaylist (ID, PlaylistID, ContentID, TrackNo) VALUES ('sp2', 'p1', 't3', 2);
            ",
        )
        .unwrap();
        conn
    }

    #[test]
    fn test_search_all() {
        let conn = create_test_db();
        let params = SearchParams {
            query: None,
            artist: None,
            genre: None,
            rating_min: None,
            bpm_min: None,
            bpm_max: None,
            key: None,
            playlist: None,
            has_genre: None,
            limit: None,
        };
        let tracks = search_tracks(&conn, &params).unwrap();
        assert_eq!(tracks.len(), 5);
    }

    #[test]
    fn test_search_by_genre() {
        let conn = create_test_db();
        let params = SearchParams {
            query: None,
            artist: None,
            genre: Some("Dubstep".to_string()),
            rating_min: None,
            bpm_min: None,
            bpm_max: None,
            key: None,
            playlist: None,
            has_genre: None,
            limit: None,
        };
        let tracks = search_tracks(&conn, &params).unwrap();
        assert_eq!(tracks.len(), 2); // Archangel + Endorphin
        assert!(tracks.iter().all(|t| t.genre == "Dubstep"));
    }

    #[test]
    fn test_search_by_bpm_range() {
        let conn = create_test_db();
        let params = SearchParams {
            query: None,
            artist: None,
            genre: None,
            rating_min: None,
            bpm_min: Some(130.0),
            bpm_max: Some(145.0),
            key: None,
            playlist: None,
            has_genre: None,
            limit: None,
        };
        let tracks = search_tracks(&conn, &params).unwrap();
        assert_eq!(tracks.len(), 2); // 139.5 and 140.0
        assert!(tracks.iter().all(|t| t.bpm >= 130.0 && t.bpm <= 145.0));
    }

    #[test]
    fn test_search_has_no_genre() {
        let conn = create_test_db();
        let params = SearchParams {
            query: None,
            artist: None,
            genre: None,
            rating_min: None,
            bpm_min: None,
            bpm_max: None,
            key: None,
            playlist: None,
            has_genre: Some(false),
            limit: None,
        };
        let tracks = search_tracks(&conn, &params).unwrap();
        assert_eq!(tracks.len(), 1); // Unknown Track has no genre
        assert_eq!(tracks[0].title, "Unknown Track");
    }

    #[test]
    fn test_search_by_rating() {
        let conn = create_test_db();
        let params = SearchParams {
            query: None,
            artist: None,
            genre: None,
            rating_min: Some(3),
            bpm_min: None,
            bpm_max: None,
            key: None,
            playlist: None,
            has_genre: None,
            limit: None,
        };
        let tracks = search_tracks(&conn, &params).unwrap();
        assert_eq!(tracks.len(), 2); // Archangel (4 stars) + Endorphin (3 stars)
    }

    #[test]
    fn test_search_by_key() {
        let conn = create_test_db();
        let params = SearchParams {
            query: None,
            artist: None,
            genre: None,
            rating_min: None,
            bpm_min: None,
            bpm_max: None,
            key: Some("Am".to_string()),
            playlist: None,
            has_genre: None,
            limit: None,
        };
        let tracks = search_tracks(&conn, &params).unwrap();
        assert_eq!(tracks.len(), 1);
        assert_eq!(tracks[0].title, "Archangel");
    }

    #[test]
    fn test_search_by_playlist() {
        let conn = create_test_db();
        let params = SearchParams {
            query: None,
            artist: None,
            genre: None,
            rating_min: None,
            bpm_min: None,
            bpm_max: None,
            key: None,
            playlist: Some("p1".to_string()),
            has_genre: None,
            limit: None,
        };
        let tracks = search_tracks(&conn, &params).unwrap();
        assert_eq!(tracks.len(), 2); // Archangel + R.I.P.
    }

    #[test]
    fn test_get_track() {
        let conn = create_test_db();
        let track = get_track(&conn, "t1").unwrap().unwrap();
        assert_eq!(track.title, "Archangel");
        assert_eq!(track.artist, "Burial");
        assert_eq!(track.genre, "Dubstep");
        assert_eq!(track.bpm, 139.5);
        assert_eq!(track.rating, 4);
        assert_eq!(track.comments, "iconic garage vocal");
        assert_eq!(track.label, "Hyperdub");
        assert_eq!(track.year, 2007);
    }

    #[test]
    fn test_get_track_not_found() {
        let conn = create_test_db();
        let track = get_track(&conn, "nonexistent").unwrap();
        assert!(track.is_none());
    }

    #[test]
    fn test_get_playlists() {
        let conn = create_test_db();
        let playlists = get_playlists(&conn).unwrap();
        assert_eq!(playlists.len(), 2);
        let deep_cuts = playlists.iter().find(|p| p.name == "Deep Cuts").unwrap();
        assert_eq!(deep_cuts.track_count, 2);
        assert!(!deep_cuts.is_folder);
        let folders = playlists.iter().find(|p| p.name == "Folders").unwrap();
        assert!(folders.is_folder);
    }

    #[test]
    fn test_get_playlist_tracks() {
        let conn = create_test_db();
        let tracks = get_playlist_tracks(&conn, "p1", None).unwrap();
        assert_eq!(tracks.len(), 2);
        assert_eq!(tracks[0].title, "Archangel");
        assert_eq!(tracks[1].title, "R.I.P.");
    }

    #[test]
    fn test_library_stats() {
        let conn = create_test_db();
        let stats = get_library_stats(&conn).unwrap();
        assert_eq!(stats.total_tracks, 5);
        assert_eq!(stats.rated_count, 3);
        assert_eq!(stats.unrated_count, 2);
        assert_eq!(stats.playlist_count, 1); // only non-folder playlists
        assert!(stats.avg_bpm > 0.0);
        assert!(!stats.genres.is_empty());
        assert!(!stats.key_distribution.is_empty());
    }

    #[test]
    fn test_get_tracks_by_ids() {
        let conn = create_test_db();
        let tracks = get_tracks_by_ids(&conn, &["t1".to_string(), "t3".to_string()]).unwrap();
        assert_eq!(tracks.len(), 2);
    }
}
