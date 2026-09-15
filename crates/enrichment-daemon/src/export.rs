//! Operator entry points for complete typed evidence bundles.
//! All storage selection, closure validation and atomic export live in Rust enrichment-store.
use enrichment_store::StatePaths;
pub use enrichment_store::bundle::{BUNDLE, Exported, MANIFEST};
use std::{io, path::Path};

/// Export an admitted snapshot with its complete referenced evidence.
/// # Errors
/// Invalid state, missing closure members and publication failures remain explicit.
pub fn export(paths: &StatePaths, context_id: &str, out: &Path) -> io::Result<Exported> {
    tokio::runtime::Runtime::new()?
        .block_on(enrichment_store::bundle::export(paths, context_id, out))
}

/// Independently verify a bundle, including its native relational contracts.
/// # Errors
/// Invalid paths, malformed manifests and failed admission are errors.
pub fn verify(root: &Path) -> io::Result<Vec<String>> {
    tokio::runtime::Runtime::new()?.block_on(enrichment_store::bundle::verify(root))
}
