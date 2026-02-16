use rand::Rng;
use reqwest::Client;
use serde::{Deserialize, Serialize};

const CONSUMER_KEY: &str = "qcBoWbIaUkAteWishTTb";
const SIGNATURE: &str =
    "vSsDVPUiDhwwyhYXgnJxpyzMPxsnxHIv&gzFloPqEjCYmotqOMxqeAptrsQMRilhAHXZeoKFY";
const TOKEN: &str = "tIuAcrOEPeUAuVZBLIxjscIYMZnMjNSxBOZTMUvz";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscogsResult {
    pub title: String,
    pub year: String,
    pub label: String,
    pub genres: Vec<String>,
    pub styles: Vec<String>,
    pub fuzzy_match: bool,
}

#[derive(Deserialize)]
struct SearchResponse {
    results: Option<Vec<SearchResult>>,
}

#[derive(Deserialize)]
struct SearchResult {
    title: Option<String>,
    year: Option<String>,
    label: Option<Vec<String>>,
    genre: Option<Vec<String>>,
    style: Option<Vec<String>>,
}

/// Normalize an artist name for matching: lowercase, strip non-alphanumeric.
fn normalize(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == ' ')
        .collect::<String>()
        .trim()
        .to_string()
}

/// Generate a random hex nonce.
fn nonce() -> String {
    let mut rng = rand::rng();
    let bytes: [u8; 16] = rng.random();
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Look up a track on Discogs. Returns None if no match found.
pub async fn lookup(
    client: &Client,
    artist: &str,
    title: &str,
) -> Result<Option<DiscogsResult>, String> {
    lookup_inner(client, artist, title, false).await
}

async fn lookup_inner(
    client: &Client,
    artist: &str,
    title: &str,
    is_retry: bool,
) -> Result<Option<DiscogsResult>, String> {
    // Rate limit
    tokio::time::sleep(std::time::Duration::from_millis(1100)).await;

    let query = format!("{artist} {title}");
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let url = format!(
        "https://api.discogs.com/database/search?q={query}&type=release&per_page=5\
         &oauth_consumer_key={CONSUMER_KEY}\
         &oauth_nonce={nonce}\
         &oauth_signature={sig}\
         &oauth_signature_method=PLAINTEXT\
         &oauth_timestamp={timestamp}\
         &oauth_token={TOKEN}\
         &oauth_version=1.0",
        query = urlencoding(&query),
        nonce = nonce(),
        sig = urlencoding(SIGNATURE),
    );

    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("request failed: {e}"))?;

    if resp.status() == 429 {
        if is_retry {
            return Err("rate limited after retry".into());
        }
        eprintln!("[crate-dig] Discogs rate limited, waiting 30s...");
        tokio::time::sleep(std::time::Duration::from_secs(30)).await;
        return Box::pin(lookup_inner(client, artist, title, true)).await;
    }

    if !resp.status().is_success() {
        return Err(format!("Discogs HTTP {}", resp.status()));
    }

    let data: SearchResponse = resp
        .json()
        .await
        .map_err(|e| format!("JSON parse error: {e}"))?;

    let results = match data.results {
        Some(r) if !r.is_empty() => r,
        _ => return Ok(None),
    };

    // Find best match by artist name in result title
    let norm_artist = normalize(artist);

    for r in &results {
        let result_title = r.title.as_deref().unwrap_or("").to_lowercase();
        if result_title.contains(&norm_artist) || norm_artist.len() < 3 {
            return Ok(Some(to_result(r, false)));
        }
    }

    // Fallback to first result
    Ok(Some(to_result(&results[0], true)))
}

fn to_result(r: &SearchResult, fuzzy: bool) -> DiscogsResult {
    DiscogsResult {
        title: r.title.clone().unwrap_or_default(),
        year: r.year.clone().unwrap_or_default(),
        label: r
            .label
            .as_ref()
            .and_then(|v| v.first().cloned())
            .unwrap_or_default(),
        genres: r.genre.clone().unwrap_or_default(),
        styles: r.style.clone().unwrap_or_default(),
        fuzzy_match: fuzzy,
    }
}

/// Percent-encode a string for URL query parameters.
fn urlencoding(s: &str) -> String {
    use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, utf8_percent_encode};
    const SET: &AsciiSet = &NON_ALPHANUMERIC
        .remove(b'-')
        .remove(b'_')
        .remove(b'.')
        .remove(b'~');
    utf8_percent_encode(s, SET).to_string()
}
