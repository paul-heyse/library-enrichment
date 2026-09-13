//! The shared service state one daemon process owns: configuration, resolved state roots, the
//! policy-bound HTTP client, the blob store and the catalog.
//!
//! Built once in the binary after the roots are resolved, then shared by every connection.
//! Tests build one over a temporary directory; nothing here reads `LIBENR_*` itself.

use std::io;

use enrichment_core::clock;
use enrichment_core::config::Config;
use enrichment_core::policy::FetchPolicy;
use enrichment_store::{BlobStore, Catalog, StatePaths};

use crate::fetch::{FetchError, Fetcher};

/// Everything a request handler needs.
#[derive(Debug)]
pub struct Service {
    /// The configuration in force.
    pub config: Config,
    /// Resolved cache and data roots.
    pub paths: StatePaths,
    /// The bounded HTTP client.
    pub fetcher: Fetcher,
    /// Content-addressed artifacts.
    pub blobs: BlobStore,
    /// Releases, contexts and current-snapshot pointers.
    pub catalog: Catalog,
    /// When this service started, RFC 3339.
    pub started_at: String,
}

/// Why the service could not be assembled.
#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    /// A state directory could not be created.
    #[error("cannot open service state under {path}: {source}")]
    Store {
        /// The root at fault.
        path: String,
        /// The I/O failure.
        source: io::Error,
    },
    /// The HTTP client could not be built.
    #[error(transparent)]
    Fetch(#[from] FetchError),
}

impl Service {
    /// Assemble the service over explicit roots.
    ///
    /// # Errors
    ///
    /// Fails if a state directory cannot be created or the HTTP client cannot be built.
    pub fn open(config: Config, paths: StatePaths) -> Result<Self, ServiceError> {
        let store_err = |path: &std::path::Path| {
            let path = path.display().to_string();
            move |source| ServiceError::Store { path, source }
        };
        std::fs::create_dir_all(&paths.cache_root).map_err(store_err(&paths.cache_root))?;
        std::fs::create_dir_all(&paths.data_root).map_err(store_err(&paths.data_root))?;
        let blobs = BlobStore::open(&paths.data_root).map_err(store_err(&paths.data_root))?;
        let catalog = Catalog::open(&paths.data_root).map_err(store_err(&paths.data_root))?;
        let fetcher = Fetcher::new(FetchPolicy::from_config(&config))?;
        Ok(Self {
            config,
            paths,
            fetcher,
            blobs,
            catalog,
            started_at: clock::now_rfc3339(),
        })
    }

    /// Whether the data root is present and writable, which is what `cache_ready` reports.
    #[must_use]
    pub fn cache_ready(&self) -> bool {
        let probe = self.paths.data_root.join(".write-probe");
        let ok = std::fs::write(&probe, b"").is_ok();
        let _ = std::fs::remove_file(&probe);
        ok
    }
}
