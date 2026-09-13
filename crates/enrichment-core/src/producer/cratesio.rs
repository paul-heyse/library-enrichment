//! The crates.io registry producer: index and API URLs, and the `.crate` tarball (§4.2).
//!
//! Pure URL construction and naming. Network I/O lives in the daemon's `fetch` module; the
//! shapes those responses take are parsed by [`crate::registry`].

use url::Url;

use crate::registry;

/// Producer name for registry metadata.
pub const PRODUCER: &str = "crates-io-registry";
/// Producer name for the crate source tarball.
pub const TARBALL_PRODUCER: &str = "crates-io-tarball";
/// Producer version, bumped when the shape of what this module records changes.
pub const VERSION: &str = "1";

/// Why a URL could not be built.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("cannot build a registry URL from base `{base}`: {message}")]
pub struct UrlError {
    /// The configured base URL.
    pub base: String,
    /// What went wrong.
    pub message: String,
}

fn join(base: &str, path: &str) -> Result<Url, UrlError> {
    let mut base_url = Url::parse(base).map_err(|e| UrlError {
        base: base.to_owned(),
        message: e.to_string(),
    })?;
    // Make the base a directory so `join` appends instead of replacing the last segment.
    if !base_url.path().ends_with('/') {
        let path = format!("{}/", base_url.path());
        base_url.set_path(&path);
    }
    base_url.join(path).map_err(|e| UrlError {
        base: base.to_owned(),
        message: e.to_string(),
    })
}

/// The sparse-index URL for a crate name.
///
/// # Errors
///
/// Fails only if the configured base is not a URL.
pub fn index_url(index_base: &str, name: &str) -> Result<Url, UrlError> {
    join(index_base, &registry::index_path(name))
}

/// The API URL for one version's record.
///
/// # Errors
///
/// Fails only if the configured base is not a URL.
pub fn version_url(api_base: &str, name: &str, version: &str) -> Result<Url, UrlError> {
    join(api_base, &format!("crates/{name}/{version}"))
}

/// The API download URL, which crates.io answers with a redirect to `static.crates.io`.
///
/// # Errors
///
/// Fails only if the configured base is not a URL.
pub fn download_url(api_base: &str, name: &str, version: &str) -> Result<Url, UrlError> {
    join(api_base, &format!("crates/{name}/{version}/download"))
}

/// The conventional tarball file name.
#[must_use]
pub fn tarball_file_name(name: &str, version: &str) -> String {
    format!("{name}-{version}.crate")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urls_follow_the_registry_conventions() {
        assert_eq!(
            index_url("https://index.crates.io", "serde")
                .expect("url")
                .as_str(),
            "https://index.crates.io/se/rd/serde"
        );
        assert_eq!(
            version_url("https://crates.io/api/v1", "serde", "1.0.219")
                .expect("url")
                .as_str(),
            "https://crates.io/api/v1/crates/serde/1.0.219"
        );
        assert_eq!(
            download_url("https://crates.io/api/v1/", "serde", "1.0.219")
                .expect("url")
                .as_str(),
            "https://crates.io/api/v1/crates/serde/1.0.219/download"
        );
        assert_eq!(tarball_file_name("serde", "1.0.219"), "serde-1.0.219.crate");
    }

    #[test]
    fn a_loopback_fixture_base_with_a_path_prefix_works() {
        assert_eq!(
            index_url("http://127.0.0.1:8123/index", "enr-fixture")
                .expect("url")
                .as_str(),
            "http://127.0.0.1:8123/index/en/r-/enr-fixture"
        );
        assert!(index_url("not a url", "x").is_err());
    }
}
