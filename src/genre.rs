/// The starter genre taxonomy. Not a closed list — arbitrary genres are accepted.
/// This list serves as a reference for consistency and auto-complete suggestions.
pub const GENRES: &[&str] = &[
    "Acid",
    "Ambient",
    "Ambient Techno",
    "Bassline",
    "Breakbeat",
    "Deep House",
    "Deep Techno",
    "Disco",
    "Downtempo",
    "Drum & Bass",
    "Dub",
    "Dub Techno",
    "Dubstep",
    "Electro",
    "Experimental",
    "Garage",
    "Grime",
    "Hard Techno",
    "Hip Hop",
    "House",
    "IDM",
    "Jungle",
    "Minimal",
    "Speed Garage",
    "Techno",
    "Trance",
];

pub fn get_taxonomy() -> Vec<String> {
    GENRES.iter().map(|s| s.to_string()).collect()
}

pub fn is_known_genre(genre: &str) -> bool {
    GENRES.iter().any(|g| g.eq_ignore_ascii_case(genre))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn taxonomy_not_empty() {
        assert!(!get_taxonomy().is_empty());
    }

    #[test]
    fn known_genre_case_insensitive() {
        assert!(is_known_genre("deep house"));
        assert!(is_known_genre("Deep House"));
        assert!(is_known_genre("TECHNO"));
        assert!(!is_known_genre("Polka"));
    }
}
