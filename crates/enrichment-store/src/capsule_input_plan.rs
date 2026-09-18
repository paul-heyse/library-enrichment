//! Select retained consumer inputs from the exact prepared inventory, without rescanning policy.
use crate::{native_catalog, runtime::QueryRuntime};
use datafusion::common::Result;
use enrichment_core::{
    native_union::{NativeStruct, Rule},
    operation::ownership::PreparedCapsule,
};

enrichment_core::native_struct! { pub struct Capture {
    path:String => Rule::MemberPath,
    bytes:u64 => Rule::Text,
    sha256:String => Rule::Sha256,
    source_uri:String => Rule::NonEmpty,
    media_type:String => Rule::NonEmpty,
} }

pub async fn select(runtime: &QueryRuntime, prepared: &PreparedCapsule) -> Result<Vec<Capture>> {
    select_inventory(
        runtime,
        &prepared.inventory,
        prepared.inputs.release.key.ecosystem,
    )
    .await
}

async fn select_inventory(
    runtime: &QueryRuntime,
    inventory: &enrichment_core::capsule_protocol::inventory::Inventory,
    ecosystem: enrichment_core::identity::Ecosystem,
) -> Result<Vec<Capture>> {
    enrichment_core::native_struct! { struct Input {
        inventory:enrichment_core::capsule_protocol::inventory::Inventory => Rule::Map,
        ecosystem:enrichment_core::identity::Ecosystem => Rule::Text,
    } }
    let session = runtime.session();
    native_catalog::input(
        &session,
        "capsule_input",
        Input::batch(&[Input {
            inventory: inventory.clone(),
            ecosystem,
        }])?,
    )?;
    let selected=session.sql(r#"
      WITH entries AS (SELECT ecosystem,unnest(native_map_entries(inventory)) AS entry FROM capsule_input),
      files AS (SELECT ecosystem,entry.key AS path,entry.value.file.bytes AS bytes,entry.value.file.sha256 AS sha256 FROM entries WHERE entry.value.kind='file')
      SELECT *,path='Cargo.toml' AS manifest FROM files WHERE
        (ecosystem='rust' AND (path='Cargo.toml' OR (starts_with(path,'cargo-home/registry/cache/') AND ends_with(path,'.crate'))))
        OR (ecosystem='python' AND starts_with(path,'wheelhouse/') AND ends_with(path,'.whl'))
    "#).await?;
    native_catalog::work(&session, "capsule_selected", selected.into_view())?;
    runtime.require_empty(session.sql(r#"
      SELECT path AS witness FROM capsule_selected WHERE bytes>CASE WHEN manifest THEN 1048576 ELSE 268435456 END
      UNION ALL SELECT 'capsule_archive_bounds' FROM capsule_selected WHERE NOT manifest HAVING count(*)>4096 OR sum(bytes)>536870912
      UNION ALL SELECT 'capsule_manifest_missing' FROM capsule_input WHERE ecosystem='rust' AND NOT EXISTS(SELECT 1 FROM capsule_selected WHERE manifest)
    "#).await?,"capsule_input_closure","consumer_acquisition").await?;
    let frame=session.sql("SELECT path,bytes,sha256,CASE WHEN manifest THEN 'consumer://manifest/cargo' ELSE concat('consumer-dependency://archive/',sha256) END AS source_uri,CASE WHEN manifest THEN 'application/toml' ELSE 'application/octet-stream' END AS media_type FROM capsule_selected ORDER BY path").await?;
    runtime.records(frame, 4097).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use enrichment_core::{capsule_protocol::inventory::Entry, identity::Ecosystem};
    #[tokio::test]
    async fn native_capsule_inputs_use_prepared_scope_and_preserve_equal_content_paths()
    -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let file = Entry::File {
            mode: 0o400,
            bytes: 10,
            sha256: "a".repeat(64),
        };
        let inventory = [
            "Cargo.toml",
            "wheelhouse/a.whl",
            "wheelhouse/b.whl",
            "wheelhouse/not-a-wheel.crate",
            "cargo-home/registry/cache/index/a.crate",
            "cargo-home/registry/cache/index/a.whl",
            "python/installed.py",
        ]
        .into_iter()
        .map(|path| (path.into(), file.clone()))
        .collect();
        let python = select_inventory(&runtime, &inventory, Ecosystem::Python).await?;
        assert_eq!(
            python.iter().map(|c| c.path.as_str()).collect::<Vec<_>>(),
            ["wheelhouse/a.whl", "wheelhouse/b.whl"]
        );
        assert_eq!(python[0].sha256, python[1].sha256);
        let rust = select_inventory(&runtime, &inventory, Ecosystem::Rust).await?;
        assert_eq!(
            rust.iter().map(|c| c.path.as_str()).collect::<Vec<_>>(),
            ["Cargo.toml", "cargo-home/registry/cache/index/a.crate"]
        );
        let mut changed = inventory.clone();
        changed.remove("Cargo.toml");
        assert!(
            select_inventory(&runtime, &changed, Ecosystem::Rust)
                .await
                .is_err()
        );
        let large = Entry::File {
            mode: 0o400,
            bytes: 268435457,
            sha256: "a".repeat(64),
        };
        changed.insert("wheelhouse/a.whl".into(), large);
        assert!(
            select_inventory(&runtime, &changed, Ecosystem::Python)
                .await
                .is_err()
        );
        runtime.close_diagnostics().await
    }
}
