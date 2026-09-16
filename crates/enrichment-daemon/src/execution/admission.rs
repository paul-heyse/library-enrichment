//! Mechanical capture of the operator receipt and physical execution identity.
//! Qualification and route eligibility are evaluated by the native execution policy.
use enrichment_core::{config::Execution, execution::facts::Receipt};
use enrichment_store::execution_policy::Capture;
use std::{
    io::{self, Read},
    path::{Path, PathBuf},
};

pub fn execution_root(config: &Execution, cache_root: &Path) -> PathBuf {
    config
        .storage_root
        .clone()
        .unwrap_or_else(|| cache_root.join("podman"))
}

pub fn receipt_path(execution_root: &Path) -> PathBuf {
    execution_root.join("admitted-images.json")
}

pub fn capture(config: &Execution, cache_root: &Path, cleanup_error: Option<String>) -> Capture {
    let root = execution_root(config, cache_root);
    let path = receipt_path(&root);
    let read = || -> io::Result<Receipt> {
        let mut bytes = Vec::new();
        std::fs::File::open(&path)?
            .take(1_048_577)
            .read_to_end(&mut bytes)?;
        if bytes.len() > 1_048_576 {
            return Err(io::Error::other(
                "qualification receipt exceeds its 1 MiB input bound",
            ));
        }
        serde_json::from_slice(&bytes).map_err(io::Error::other)
    };
    let (receipt, receipt_error) = match read() {
        Ok(receipt) => (Some(receipt), None),
        Err(error) => (
            None,
            Some(format!(
                "No readable qualification receipt at {}: {error}; run just execution-qualify --apply.",
                path.display()
            )),
        ),
    };
    let (containment_identity, containment_error) =
        match super::description::containment_identity(config) {
            Ok(identity) => (Some(identity), None),
            Err(error) => (
                None,
                Some(format!(
                    "Execution identity unavailable: {error}; build the helper and requalify."
                )),
            ),
        };
    Capture {
        execution_root: root.to_string_lossy().into_owned(),
        containment_identity,
        containment_error,
        receipt,
        receipt_error,
        cleanup_error,
    }
}
