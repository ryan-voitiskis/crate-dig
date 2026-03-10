use security_framework::passwords;

const SERVICE: &str = "com.reklawdbox.broker-session";

/// macOS `errSecItemNotFound` (-25300).
const ERR_SEC_ITEM_NOT_FOUND: i32 = -25300;

/// Store a session token in the macOS Keychain.
pub fn set_session_token(broker_url: &str, token: &str) -> Result<(), String> {
    passwords::set_generic_password(SERVICE, broker_url, token.as_bytes())
        .map_err(|e| format!("Keychain write failed: {e}"))
}

/// Retrieve a session token from the macOS Keychain.
/// Returns `Ok(None)` if no entry exists.
pub fn get_session_token(broker_url: &str) -> Result<Option<String>, String> {
    match passwords::get_generic_password(SERVICE, broker_url) {
        Ok(bytes) => String::from_utf8(bytes)
            .map(Some)
            .map_err(|e| format!("Keychain value is not UTF-8: {e}")),
        Err(e) if e.code() == ERR_SEC_ITEM_NOT_FOUND => Ok(None),
        Err(e) => Err(format!("Keychain read failed: {e}")),
    }
}

/// Delete a session token from the macOS Keychain.
/// Silently succeeds if no entry exists.
pub fn delete_session_token(broker_url: &str) -> Result<(), String> {
    match passwords::delete_generic_password(SERVICE, broker_url) {
        Ok(()) => Ok(()),
        Err(e) if e.code() == ERR_SEC_ITEM_NOT_FOUND => Ok(()),
        Err(e) => Err(format!("Keychain delete failed: {e}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_URL: &str = "https://test-broker.example.com/keychain-test";

    #[test]
    fn round_trip() {
        // Clean up from any prior failed run.
        let _ = delete_session_token(TEST_URL);

        set_session_token(TEST_URL, "test-secret-123").unwrap();
        let got = get_session_token(TEST_URL).unwrap();
        assert_eq!(got.as_deref(), Some("test-secret-123"));

        // Overwrite.
        set_session_token(TEST_URL, "test-secret-456").unwrap();
        let got = get_session_token(TEST_URL).unwrap();
        assert_eq!(got.as_deref(), Some("test-secret-456"));

        // Delete.
        delete_session_token(TEST_URL).unwrap();
        let got = get_session_token(TEST_URL).unwrap();
        assert!(got.is_none());

        // Delete again (idempotent).
        delete_session_token(TEST_URL).unwrap();
    }

    #[test]
    fn not_found_returns_none() {
        let got =
            get_session_token("https://nonexistent.example.com/no-such-keychain-entry").unwrap();
        assert!(got.is_none());
    }
}
