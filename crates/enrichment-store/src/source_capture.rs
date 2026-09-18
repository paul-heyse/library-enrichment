//! One native file-capture policy for package, revision and Python source consumers.
use crate::{native_catalog, runtime::QueryRuntime};
use datafusion::common::{DataFusionError, Result};
use enrichment_core::{
    evidence::{ArtifactKind, FragmentKind},
    native_union::{NativeStruct, Rule},
};

enrichment_core::native_vocabulary! { pub enum Family {
    RustPackage = "rust_package", RustRevision = "rust_revision", Python = "python",
} }
enrichment_core::native_struct! { struct File { path: String => Rule::MemberPath } }
enrichment_core::native_struct! { struct Scope { family: Family => Rule::Text } }
enrichment_core::native_struct! { struct Document {
    path: String => Rule::MemberPath,
    kind: FragmentKind => Rule::Text,
} }
enrichment_core::native_struct! { pub struct Capture {
    path: String => Rule::MemberPath,
    artifact_kind: ArtifactKind => Rule::Text,
    media_type: String => Rule::NonEmpty,
    role: Option<String> => Rule::NonEmpty,
    fragment_kind: Option<FragmentKind> => Rule::Text,
} }

pub async fn select(
    runtime: &QueryRuntime,
    files: Vec<String>,
    family: Family,
) -> Result<Vec<Capture>> {
    if files.len() > enrichment_core::policy::ArchivePolicy::default().max_entries {
        return Err(DataFusionError::ResourcesExhausted(
            "source inventory exceeds entry bound".into(),
        ));
    }
    let session = runtime.session();
    native_catalog::input(
        &session,
        "source_files",
        File::batch(
            &files
                .into_iter()
                .map(|path| File { path })
                .collect::<Vec<_>>(),
        )?,
    )?;
    native_catalog::input(
        &session,
        "source_family",
        Scope::batch(&[Scope { family }])?,
    )?;
    native_catalog::input(
        &session,
        "source_documents",
        Document::batch(
            &enrichment_core::producer::source::DOCUMENT_FILES
                .iter()
                .map(|(path, kind)| Document {
                    path: (*path).into(),
                    kind: *kind,
                })
                .collect::<Vec<_>>(),
        )?,
    )?;
    runtime.require_empty(session.sql("SELECT path AS witness FROM source_files GROUP BY path HAVING count(*)<>1 UNION ALL SELECT 'source_path_bytes' AS witness FROM source_files HAVING sum(octet_length(path))>16777216").await?, "source_capture_inventory", "source_capture").await?;
    let frame = session.sql(r#"
      WITH classified AS (
        SELECT f.path,s.family,lower(f.path) AS lower_path,
          coalesce(d.kind,CASE WHEN starts_with(f.path,'examples/') AND ends_with(f.path,'.rs') AND array_length(string_to_array(f.path,'/'))=2 THEN 'example' END) AS rust_kind,
          CASE WHEN contains(lower(f.path),'readme') THEN 'readme'
            WHEN contains(lower(f.path),'changelog') OR contains(lower(f.path),'changes') THEN 'changelog'
            ELSE 'source_file' END AS python_kind
        FROM source_files f CROSS JOIN source_family s LEFT JOIN source_documents d ON f.path=d.path
      ), selected AS (
        SELECT *,CASE WHEN family='python' THEN python_kind
          WHEN family='rust_revision' THEN 'source_file'
          WHEN rust_kind='readme_section' THEN 'readme'
          WHEN rust_kind='changelog_section' THEN 'changelog'
          ELSE 'source_file' END AS artifact_kind FROM classified
        WHERE family='rust_revision' OR (family='rust_package' AND rust_kind IS NOT NULL)
          OR (family='python' AND (ends_with(path,'.py') OR ends_with(path,'.pyi') OR ends_with(path,'.md')
            OR ends_with(path,'.rst') OR ends_with(path,'.txt') OR ends_with(path,'METADATA')
            OR ends_with(path,'PKG-INFO') OR ends_with(path,'pyproject.toml')))
      ) SELECT path,artifact_kind,
        CASE WHEN family<>'rust_package' THEN 'text/plain' WHEN ends_with(path,'.md') THEN 'text/markdown' ELSE 'text/x-rust' END AS media_type,
        CASE WHEN family='python' THEN concat('python-source:',path) WHEN family='rust_revision' THEN concat('revision-source:',path) END AS role,
        CASE WHEN family<>'python' THEN rust_kind WHEN artifact_kind='changelog' THEN 'changelog_section'
          WHEN starts_with(lower_path,'examples/') THEN 'example'
          WHEN artifact_kind='readme' THEN 'readme_section' END AS fragment_kind
        FROM selected ORDER BY path
    "#).await?;
    runtime
        .records(
            frame,
            enrichment_core::policy::ArchivePolicy::default().max_entries,
        )
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn native_capture_keeps_independent_roles_and_declared_source_scope() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let files = [
            "Cargo.toml",
            "src/lib.rs",
            "README.md",
            "CHANGES.md",
            "examples/basic.rs",
            "examples/nested/other.rs",
            "pkg/api.py",
            "pkg/api.pyi",
            "examples/README.md",
            "pkg.dist-info/METADATA",
            "native.so",
        ];
        let input = || files.into_iter().map(str::to_owned).collect::<Vec<_>>();
        let rust = select(&runtime, input(), Family::RustPackage).await?;
        assert_eq!(
            rust.iter().map(|r| r.path.as_str()).collect::<Vec<_>>(),
            ["CHANGES.md", "README.md", "examples/basic.rs"]
        );
        assert!(rust.iter().all(|row| row.role.is_none()));
        assert_eq!(rust[0].fragment_kind, Some(FragmentKind::ChangelogSection));
        assert_eq!(rust[2].media_type, "text/x-rust");
        let revision = select(&runtime, input(), Family::RustRevision).await?;
        assert_eq!(revision.len(), files.len());
        assert!(
            revision
                .iter()
                .all(|row| row.artifact_kind == ArtifactKind::SourceFile
                    && row.role.as_deref() == Some(&format!("revision-source:{}", row.path)))
        );
        let python = select(&runtime, input(), Family::Python).await?;
        assert_eq!(python.len(), 6);
        let example = python
            .iter()
            .find(|row| row.path == "examples/README.md")
            .unwrap();
        assert_eq!(
            (example.artifact_kind, example.fragment_kind),
            (ArtifactKind::Readme, Some(FragmentKind::Example))
        );
        assert!(
            python
                .iter()
                .any(|row| row.path == "pkg/api.pyi" && row.fragment_kind.is_none())
        );
        assert!(
            select(
                &runtime,
                vec!["same.py".into(), "same.py".into()],
                Family::Python
            )
            .await
            .is_err()
        );
        assert!(
            select(&runtime, vec![], Family::RustPackage)
                .await?
                .is_empty()
        );
        runtime.close_diagnostics().await?;
        Ok(())
    }
}
