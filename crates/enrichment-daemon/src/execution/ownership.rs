//! Physical path observations for the native execution-root control family.
use enrichment_core::config::Execution;
use enrichment_store::{
    StatePaths,
    physical_ownership::{ExecutionRoot, OwnershipStore},
};
use std::{io, path::Path};

pub fn validate_root(cache: &Path, root: &Path, broker: &Path) -> io::Result<()> {
    if !broker.is_absolute()
        || !root.is_absolute()
        || root.parent().is_none()
        || root == cache
        || cache.starts_with(root)
        || (root.starts_with(cache) && !root.starts_with(cache.join("podman")))
    {
        return Err(io::Error::other(
            "execution root overlaps cache payloads or has an invalid broker path",
        ));
    }
    if root.canonicalize()? != root {
        return Err(io::Error::other(
            "execution root is no longer the recorded physical path",
        ));
    }
    Ok(())
}

pub async fn validate_for_state(
    paths: &StatePaths,
    execution: &Execution,
    store: &OwnershipStore,
) -> io::Result<()> {
    let mut roots = store.roots().await.map_err(io::Error::other)?;
    for root in &roots {
        validate_root(
            &paths.cache_root,
            Path::new(&root.root),
            Path::new(&root.broker),
        )?;
    }
    roots.push(ExecutionRoot {
        root: super::admission::execution_root(execution, &paths.cache_root)
            .to_str()
            .ok_or_else(|| io::Error::other("invalid root path"))?
            .into(),
        cache: store.cache().into(),
        broker: execution
            .broker()
            .to_str()
            .ok_or_else(|| io::Error::other("invalid broker path"))?
            .into(),
    });
    for root in roots {
        let root = Path::new(&root.root);
        if root.starts_with(&paths.data_root)
            || paths.data_root.starts_with(root)
            || root == paths.cache_root
            || paths.cache_root.starts_with(root)
            || (root.starts_with(&paths.cache_root)
                && !root.starts_with(paths.cache_root.join("podman")))
        {
            return Err(io::Error::other(
                "execution storage overlaps evidence or deletable cache state",
            ));
        }
    }
    Ok(())
}
