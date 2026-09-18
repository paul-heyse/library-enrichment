//! Native source commands; the driver observes files and reads only the selected member.
use crate::{native_catalog, runtime::QueryRuntime};
use datafusion::common::Result;
use enrichment_core::{
    evidence::relational::{InputArtifact, Locator},
    identity::Ecosystem,
    native_union::{NativeStruct, Rule, Unit},
};

enrichment_core::native_struct! { struct Request {
    ecosystem: Ecosystem => Rule::Text,
    locator: Option<Locator> => Rule::Text,
    max_lines: usize => Rule::Text,
} }
enrichment_core::native_struct! { pub struct Location {
    path: String => Rule::NonEmpty,
    start_line: u32 => Rule::Coordinate(Unit::LineOneBased),
    max_lines: usize => Rule::UnsignedRange { min: 1, max: enrichment_core::producer::source::MAX_WINDOW_LINES as u64 },
    archive: bool => Rule::Text,
    role: Option<String> => Rule::NonEmpty,
} }
pub struct Read {
    pub artifact: InputArtifact,
    pub location: Location,
}
enrichment_core::native_struct! { pub struct Candidate {
    path: String => Rule::MemberPath,
    ordinal: u64 => Rule::Coordinate(Unit::Ordinal),
} }
enrichment_core::native_struct! { pub struct Observation {
    candidate: Candidate => Rule::Text,
    present: bool => Rule::Text,
} }

async fn location(
    runtime: &QueryRuntime,
    ecosystem: Ecosystem,
    locator: Option<&Locator>,
    max_lines: usize,
) -> Result<Option<Location>> {
    let session = runtime.session();
    native_catalog::input(
        &session,
        "source_request",
        Request::batch(&[Request {
            ecosystem,
            locator: locator.cloned(),
            max_lines,
        }])?,
    )?;
    let frame = session.sql(r#"
      WITH coordinates AS (
        SELECT ecosystem='rust' AS archive,
          CASE WHEN ecosystem='rust' AND locator.kind='rustdoc_item' THEN locator.rustdoc_item.reported_file
            WHEN ecosystem='python' AND locator.kind='python_declaration' THEN locator.python_declaration.file END AS path,
          CASE WHEN ecosystem='rust' AND locator.kind='rustdoc_item' THEN locator.rustdoc_item.reported_line
            WHEN ecosystem='python' AND locator.kind='python_declaration' THEN locator.python_declaration.line END AS start_line,
          least(greatest(max_lines,CAST(1 AS BIGINT UNSIGNED)),$1) AS max_lines
        FROM source_request
      ) SELECT path,start_line,max_lines,archive,CASE WHEN NOT archive THEN concat('python-source:',path) END AS role
        FROM coordinates WHERE path IS NOT NULL AND start_line IS NOT NULL AND start_line>0
    "#).await?.with_param_values(vec![datafusion::common::ScalarValue::UInt64(Some(enrichment_core::producer::source::MAX_WINDOW_LINES as u64))])?;
    Ok(runtime.records(frame, 1).await?.pop())
}

pub(crate) async fn select(
    runtime: &QueryRuntime,
    inputs: datafusion::dataframe::DataFrame,
    ecosystem: Ecosystem,
    locator: Option<&Locator>,
    max_lines: usize,
) -> Result<Option<Read>> {
    let Some(location) = location(runtime, ecosystem, locator, max_lines).await? else {
        return Ok(None);
    };
    let session = runtime.session();
    native_catalog::input(
        &session,
        "source_location",
        Location::batch(std::slice::from_ref(&location))?,
    )?;
    native_catalog::work(&session, "source_inputs", inputs.into_view())?;
    let inputs = session.sql("SELECT i.* FROM source_inputs i CROSS JOIN source_location l WHERE (l.archive AND i.kind='crate_tarball') OR (NOT l.archive AND i.role=l.role)").await?;
    Ok(crate::research_inspection::source_input(runtime, inputs)
        .await?
        .map(|artifact| Read { artifact, location }))
}

/// Bounded path suffixes reflect producer-reported archive prefixes. Unsafe components
/// never reach the filesystem; an actual regular-file observation is required below.
pub async fn candidates(runtime: &QueryRuntime, location: &Location) -> Result<Vec<Candidate>> {
    let session = runtime.session();
    native_catalog::input(
        &session,
        "source_location",
        Location::batch(std::slice::from_ref(location))?,
    )?;
    runtime.require_empty(session.sql("SELECT 'source_path_bound' AS witness FROM source_location WHERE octet_length(path)>16384 OR array_length(string_to_array(path,'/'))>256").await?, "source_path_bound", "source_window").await?;
    let frame = session.sql(r#"
      WITH segments AS (SELECT string_to_array(path,'/') AS parts FROM source_location),
        suffixes AS (SELECT parts,unnest(generate_series(1,CAST(array_length(parts) AS BIGINT))) AS ordinal FROM segments),
        members AS (SELECT array_slice(parts,ordinal,CAST(array_length(parts) AS BIGINT)) AS parts,ordinal FROM suffixes)
      SELECT array_to_string(parts,'/') AS path,CAST(ordinal AS BIGINT UNSIGNED) AS ordinal FROM members
      WHERE NOT array_has_any(parts,['','..','.']) ORDER BY ordinal
    "#).await?;
    runtime.records(frame, 256).await
}

pub async fn member(
    runtime: &QueryRuntime,
    observations: &[Observation],
) -> Result<Option<Candidate>> {
    let session = runtime.session();
    native_catalog::input(
        &session,
        "source_members",
        Observation::batch(observations)?,
    )?;
    let frame = session.sql("SELECT candidate.path AS path,candidate.ordinal AS ordinal FROM source_members WHERE present ORDER BY candidate.ordinal,candidate.path LIMIT 1").await?;
    Ok(runtime.records(frame, 1).await?.pop())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn plan19_source_location_and_member_selection_use_native_bounds_and_order() -> Result<()>
    {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let locator = Locator::RustdocItem {
            item: 1,
            reported_file: Some("/build/pkg/src/lib.rs".into()),
            reported_line: Some(7),
        };
        assert!(
            location(&runtime, Ecosystem::Python, Some(&locator), 4)
                .await?
                .is_none()
        );
        assert!(
            location(&runtime, Ecosystem::Rust, None, 4)
                .await?
                .is_none()
        );
        let mut selected = location(&runtime, Ecosystem::Rust, Some(&locator), 8192)
            .await?
            .unwrap();
        assert_eq!((selected.start_line, selected.max_lines), (7, 1024));
        assert!(selected.archive);
        assert!(selected.role.is_none());
        let paths = candidates(&runtime, &selected).await?;
        assert_eq!(
            paths.iter().map(|p| p.path.as_str()).collect::<Vec<_>>(),
            [
                "build/pkg/src/lib.rs",
                "pkg/src/lib.rs",
                "src/lib.rs",
                "lib.rs"
            ]
        );
        let observations = paths
            .iter()
            .rev()
            .map(|candidate| Observation {
                candidate: candidate.clone(),
                present: candidate.path == "lib.rs" || candidate.path == "src/lib.rs",
            })
            .collect::<Vec<_>>();
        assert_eq!(
            member(&runtime, &observations).await?.unwrap().path,
            "src/lib.rs"
        );
        assert!(member(&runtime, &[]).await?.is_none());
        selected.path = "../src/lib.rs".into();
        assert_eq!(candidates(&runtime, &selected).await?[0].path, "src/lib.rs");
        selected.path = "x/".repeat(257);
        assert!(candidates(&runtime, &selected).await.is_err());
        let locator = Locator::PythonDeclaration {
            file: "pkg/module.py".into(),
            declaration: "f".into(),
            line: Some(2),
            origin: enrichment_core::evidence::relational::ApiOrigin::Source,
            overload: None,
        };
        let selected = location(&runtime, Ecosystem::Python, Some(&locator), 0)
            .await?
            .unwrap();
        assert_eq!(
            selected.role.as_deref(),
            Some("python-source:pkg/module.py")
        );
        assert_eq!(selected.max_lines, 1);
        runtime.close_diagnostics().await?;
        Ok(())
    }
}
