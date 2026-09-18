//! One native distribution inventory and metadata policy for acquisition and execution.
use crate::{native_catalog, runtime::QueryRuntime};
use datafusion::{
    common::{DataFusionError, Result, ScalarValue},
    execution::context::SessionContext,
};
use enrichment_core::{
    native_union::{NativeStruct, Rule},
    producer::python::Distribution,
    request::ResolveRequest,
};
use std::{path::PathBuf, sync::Arc};

pub struct Captured {
    pub distribution: Distribution,
    texts: Vec<Text>,
    _text_memory: datafusion::execution::memory_pool::MemoryReservation,
}

impl Captured {
    pub fn texts(&self) -> &[Text] {
        &self.texts
    }
}

/// An import-root set identifies a single library target only when it has one distinct member.
/// Multi-package distributions retain their full root set without inventing a first root.
pub async fn release_binding(
    runtime: &QueryRuntime,
    release: &enrichment_core::identity::Release,
    roots: &[String],
) -> Result<enrichment_core::identity::Release> {
    use datafusion::{functions::core::expr_ext::FieldAccessor, prelude::col};
    use enrichment_core::identity::Release;
    enrichment_core::native_struct! { struct Binding {
        release: Release => Rule::Text,
        roots: Vec<String> => Rule::Set,
    } }
    let session = runtime.session();
    native_catalog::input(
        &session,
        "distribution_binding",
        Binding::batch(&[Binding {
            release: release.clone(),
            roots: roots.to_vec(),
        }])?,
    )?;
    let frame = session.sql("SELECT release,CASE WHEN cardinality(array_distinct(roots))=1 THEN roots[1] END AS root_module FROM distribution_binding").await?;
    let frame = frame.select(
        Release::fields()
            .iter()
            .map(|field| {
                if matches!(field.name().as_str(), "root_module" | "lib_name") {
                    col("root_module").alias(field.name())
                } else {
                    col("release").field(field.name()).alias(field.name())
                }
            })
            .collect::<Vec<_>>(),
    )?;
    runtime.records(frame, 1).await?.pop().ok_or_else(|| {
        datafusion::common::exec_datafusion_err!("distribution release binding missing")
    })
}

/// The supplied physical owner must cover this already-admitted, private root. Blocking
/// readers retain it after cancellation; the returned text buffers retain their reservation.
pub async fn capture<O: Send + Sync + 'static>(
    runtime: &QueryRuntime,
    root: PathBuf,
    owner: Arc<O>,
    filename: &str,
    sha256: &str,
    source_root: &str,
) -> Result<Captured> {
    let held = owner.clone();
    let directory = root.clone();
    let paths = runtime
        .blocking(move || {
            let _held = held;
            enrichment_core::producer::python::archive::files(&directory)
                .map_err(DataFusionError::Execution)
        })
        .await??;
    let selected = select(runtime, &paths, filename).await?;
    let pool = runtime.session().runtime_env().memory_pool.clone();
    let (texts, memory) = runtime
        .blocking(move || -> Result<_> {
            let _owner = owner;
            let memory = datafusion::execution::memory_pool::MemoryConsumer::new(
                "distribution-metadata-text",
            )
            .register(&pool);
            let mut texts = Vec::with_capacity(selected.len());
            for selected in selected {
                let bytes = crate::owned_bytes::OwnedBytes::read_file(
                    &root.join(&selected.path),
                    1_048_576,
                    &pool,
                    "distribution-metadata-read",
                )?;
                memory.try_grow(bytes.len())?;
                let text = std::str::from_utf8(&bytes)
                    .map_err(|error| DataFusionError::Execution(error.to_string()))?
                    .to_owned();
                texts.push(Text {
                    path: selected.path,
                    role: selected.role,
                    text,
                });
            }
            Ok((texts, memory))
        })
        .await??;
    let distribution = inventory(runtime, &paths, filename, sha256, source_root, &texts).await?;
    Ok(Captured {
        distribution,
        texts,
        _text_memory: memory,
    })
}

enrichment_core::native_vocabulary! { pub enum Role { Metadata="metadata", Wheel="wheel", EntryPoints="entry_points" } }
enrichment_core::native_struct! { pub struct File { path:String => Rule::MemberPath } }
enrichment_core::native_struct! { pub struct Selected { path:String => Rule::MemberPath, role:Role => Rule::Text } }
enrichment_core::native_struct! { pub struct Text { path:String => Rule::MemberPath, role:Role => Rule::Text, text:String => Rule::Text } }
enrichment_core::native_struct! { struct Claim { name:String => Rule::NonEmpty, version:String => Rule::NonEmpty, python_version:Option<String> => Rule::Text } }
enrichment_core::native_struct! { struct Warning { text:String => Rule::Text } }

fn files(session: &SessionContext, paths: &[String]) -> Result<()> {
    if paths.len() > 20_000 || paths.iter().map(String::len).sum::<usize>() > 16_777_216 {
        return Err(DataFusionError::ResourcesExhausted(
            "distribution file inventory bound".into(),
        ));
    }
    native_catalog::input(
        session,
        "distribution_files",
        File::batch(
            &paths
                .iter()
                .map(|path| File { path: path.clone() })
                .collect::<Vec<_>>(),
        )?,
    )
}

pub async fn select(
    runtime: &QueryRuntime,
    paths: &[String],
    filename: &str,
) -> Result<Vec<Selected>> {
    let session = runtime.session();
    files(&session, paths)?;
    runtime.require_empty(session.sql("SELECT 'distribution_filename' AS witness WHERE octet_length($1)>1024 OR regexp_like($1,'[/\\\\:%]') OR $1='' ").await?.with_param_values(vec![ScalarValue::from(filename)])?,"distribution_filename","python_preparation").await?;
    let frame=session.sql(r#"
      WITH metadata AS (SELECT path,'metadata' AS role FROM distribution_files WHERE ends_with(path,'.dist-info/METADATA') OR (NOT ends_with($1,'.whl') AND path='PKG-INFO'))
      SELECT * FROM metadata
      UNION ALL SELECT path,'entry_points' FROM distribution_files WHERE ends_with(path,'.dist-info/entry_points.txt')
      UNION ALL SELECT w.path,'wheel' FROM distribution_files w JOIN metadata m ON w.path=concat(regexp_replace(m.path,'METADATA$',''),'WHEEL') WHERE ends_with($1,'.whl')
    "#).await?.with_param_values(vec![ScalarValue::from(filename)])?;
    native_catalog::work(&session, "distribution_selected", frame.into_view())?;
    runtime.require_empty(session.sql("SELECT role AS witness FROM distribution_selected GROUP BY role HAVING count(*)>1 UNION ALL SELECT path FROM distribution_files GROUP BY path HAVING count(*)<>1").await?,"distribution_metadata_authority","python_preparation").await?;
    runtime
        .records(
            session
                .sql("SELECT * FROM distribution_selected ORDER BY role,path")
                .await?,
            3,
        )
        .await
}

/// No filesystem lookup or source selection occurs while projecting the complete distribution.
pub async fn inventory(
    runtime: &QueryRuntime,
    paths: &[String],
    filename: &str,
    sha256: &str,
    source_root: &str,
    texts: &[Text],
) -> Result<Distribution> {
    let session = runtime.session();
    files(&session, paths)?;
    bind_texts(&session, texts)?;
    let baseline = Distribution {
        filename: filename.into(),
        sha256: sha256.into(),
        source_root: source_root.into(),
        ..Default::default()
    };
    native_catalog::input(
        &session,
        "distribution_base",
        Distribution::batch(&[baseline])?,
    )?;
    let selected = select(runtime, paths, filename).await?;
    native_catalog::input(
        &session,
        "distribution_selected",
        Selected::batch(&selected)?,
    )?;
    runtime.require_empty(session.sql("SELECT t.path AS witness FROM distribution_text t LEFT ANTI JOIN distribution_selected s ON t.path=s.path AND t.role=s.role UNION ALL SELECT s.path FROM distribution_selected s LEFT ANTI JOIN distribution_text t ON t.path=s.path AND t.role=s.role").await?,"distribution_metadata_capture","python_preparation").await?;
    let header=session.sql("SELECT coalesce((SELECT python_metadata_headers_v1(text) FROM distribution_text WHERE role='metadata'),metadata) AS metadata FROM distribution_base").await?;
    native_catalog::work(&session, "distribution_headers", header.into_view())?;
    runtime.require_empty(session.sql(r#"
      WITH fields AS (SELECT unnest(native_map_entries(metadata)) AS entry FROM distribution_headers)
      SELECT entry.key AS witness FROM fields WHERE NOT regexp_like(entry.key,'^[a-z0-9-]+$')
        OR (entry.key IN ('name','version','requires-python') AND (cardinality(entry.value)<>1 OR trim(entry.value[1])=''))
    "#).await?,"distribution_scalar_headers","python_preparation").await?;
    let imports=session.sql(r#"
      WITH mapped AS (SELECT path,CASE WHEN contains(path,'.data/purelib/') THEN split_part(path,'.data/purelib/',2)
        WHEN contains(path,'.data/platlib/') THEN split_part(path,'.data/platlib/',2)
        WHEN contains(path,'.data/') OR contains(path,'.dist-info/') OR contains(path,'.egg-info/') THEN NULL
        WHEN NOT ends_with(filename,'.whl') AND starts_with(path,'src/') THEN substr(path,5) ELSE path END AS member
        FROM distribution_files CROSS JOIN distribution_base),
      syntax AS (SELECT *,CASE WHEN ends_with(member,'.pyi') THEN 'stub' WHEN ends_with(member,'.py') THEN 'source' END AS origin,
        string_to_array(regexp_replace(regexp_replace(member,'\.pyi?$',''),'-stubs(/|$)','$1','g'),'/') AS parts FROM mapped),
      modules AS (SELECT *,CASE WHEN array_element(parts,-1)='__init__' THEN array_pop_back(parts) ELSE parts END AS module_parts FROM syntax)
      SELECT path,member,origin,module_parts,array_to_string(module_parts,'.') AS module FROM modules
    "#).await?;
    native_catalog::work(&session, "distribution_imports", imports.into_view())?;
    let source=session.sql(r#"SELECT path AS file,module,origin FROM distribution_imports WHERE origin IS NOT NULL AND cardinality(module_parts)>0 AND regexp_like(module,'^[\p{Alphabetic}\p{Number}_]+(\.[\p{Alphabetic}\p{Number}_]+)*$')"#).await?;
    native_catalog::work(&session, "distribution_source", source.into_view())?;
    let frame=session.sql(r#"
      SELECT d.filename,d.sha256,map_extract(h.metadata,'name')[1][1] AS name,map_extract(h.metadata,'version')[1][1] AS version,
        coalesce((SELECT array_agg(DISTINCT root ORDER BY root) FROM (
          SELECT split_part(module,'.',1) AS root FROM distribution_source
          UNION SELECT split_part(split_part(member,'/',1),'.',1) FROM distribution_imports WHERE ends_with(member,'.so') OR ends_with(member,'.pyd'))),d.import_roots) AS import_roots,
        coalesce((SELECT array_agg(named_struct('file',file,'module',module,'origin',origin) ORDER BY file) FROM distribution_source),d.files) AS files,
        coalesce((SELECT array_agg(path ORDER BY path) FROM distribution_imports WHERE ends_with(member,'.so') OR ends_with(member,'.pyd')),d.native_files) AS native_files,
        coalesce((SELECT array_agg(path ORDER BY path) FROM distribution_files WHERE ends_with(path,'py.typed')),d.typed_markers) AS typed_markers,
        h.metadata,(SELECT text FROM distribution_text WHERE role='entry_points') AS entry_points,d.worker_artifact_id,d.source_root,d.inventory
        FROM distribution_base d CROSS JOIN distribution_headers h
    "#).await?;
    let frame = crate::native_delta::project(
        frame,
        &arrow::datatypes::Schema::new(Distribution::fields()),
    )?;
    runtime
        .records::<Distribution>(frame, 1)
        .await?
        .pop()
        .ok_or_else(|| DataFusionError::Execution("distribution inventory missing".into()))
}

fn bind_texts(session: &SessionContext, texts: &[Text]) -> Result<()> {
    if texts.len() > 3 || texts.iter().any(|t| t.text.len() > 1_048_576) {
        return Err(DataFusionError::ResourcesExhausted(
            "distribution metadata capture bound".into(),
        ));
    }
    native_catalog::input(session, "distribution_text", Text::batch(texts)?)
}

pub async fn admit(
    runtime: &QueryRuntime,
    distribution: &Distribution,
    texts: &[Text],
    request: &ResolveRequest,
    version: &str,
) -> Result<Vec<String>> {
    let session = runtime.session();
    bind_texts(&session, texts)?;
    native_catalog::input(
        &session,
        "distribution",
        Distribution::batch(std::slice::from_ref(distribution))?,
    )?;
    native_catalog::input(
        &session,
        "distribution_claim",
        Claim::batch(&[Claim {
            name: request.name.clone(),
            version: version.into(),
            python_version: request.python_version.clone(),
        }])?,
    )?;
    let frame=session.sql(r#"
      SELECT d.*,c.name AS expected_name,c.version AS expected_version,c.python_version,
        map_extract(d.metadata,'name')[1] AS names,map_extract(d.metadata,'version')[1] AS versions,
        map_extract(d.metadata,'requires-python')[1] AS requires_python,
        string_to_array(regexp_replace(d.filename,'\.whl$',''),'-') AS filename_parts,
        (SELECT regexp_replace(path,'/METADATA$','') FROM distribution_text WHERE role='metadata') AS metadata_dir,
        (SELECT python_metadata_headers_v1(text) FROM distribution_text WHERE role='wheel') AS wheel,
        (SELECT python_metadata_headers_v1(text) FROM distribution_text WHERE role='metadata') AS observed_metadata
        FROM distribution d CROSS JOIN distribution_claim c
    "#).await?;
    native_catalog::work(&session, "distribution_values", frame.into_view())?;
    runtime.require_empty(session.sql(r#"
      SELECT 'distribution_identity' AS witness FROM distribution_values WHERE
        (metadata IS DISTINCT FROM observed_metadata) OR (cardinality(names) IS DISTINCT FROM 1) OR (cardinality(versions) IS DISTINCT FROM 1)
        OR (regexp_replace(lower(names[1]),'[-_.]+','-','g') IS DISTINCT FROM expected_name)
        OR pep440_value_v1(versions[1]) IS NULL OR pep440_value_v1(expected_version) IS NULL
        OR (pep440_value_v1(versions[1])['precedence'] IS DISTINCT FROM pep440_value_v1(expected_version)['precedence'])
        OR (requires_python IS NOT NULL AND (cardinality(requires_python)<>1 OR trim(requires_python[1])=''
          OR pep440_matches_v1(requires_python[1],coalesce(python_version,'0')) IS NULL
          OR (python_version IS NOT NULL AND NOT pep440_matches_v1(requires_python[1],python_version))))
    "#).await?,"distribution_identity_and_interpreter","python_preparation").await?;
    let wheel=session.sql(r#"
      SELECT *,regexp_replace(metadata_dir,'\.dist-info$','') AS directory_stem,
        map_extract(wheel,'wheel-version')[1] AS wheel_versions,map_extract(wheel,'root-is-purelib')[1] AS purelibs,
        map_extract(wheel,'tag')[1] AS tags FROM distribution_values WHERE ends_with(filename,'.whl')
    "#).await?;
    native_catalog::work(&session, "wheel_values", wheel.into_view())?;
    runtime.require_empty(session.sql(r#"
      SELECT 'wheel_contract' AS witness FROM wheel_values WHERE
        metadata_dir IS NULL OR contains(metadata_dir,'/') OR wheel IS NULL OR cardinality(filename_parts) NOT IN (5,6)
        OR NOT regexp_like(directory_stem,'^.+-[^-]+$')
        OR regexp_replace(lower(filename_parts[1]),'[-_.]+','-','g')<>expected_name
        OR regexp_replace(lower(regexp_replace(directory_stem,'-[^-]+$','')),'[-_.]+','-','g')<>expected_name
        OR pep440_value_v1(filename_parts[2]) IS NULL OR pep440_value_v1(split_part(directory_stem,'-',-1)) IS NULL
        OR (pep440_value_v1(filename_parts[2])['precedence'] IS DISTINCT FROM pep440_value_v1(expected_version)['precedence'])
        OR (pep440_value_v1(split_part(directory_stem,'-',-1))['precedence'] IS DISTINCT FROM pep440_value_v1(expected_version)['precedence'])
        OR (cardinality(wheel_versions) IS DISTINCT FROM 1) OR NOT coalesce(regexp_like(wheel_versions[1],'^1\.[0-9]+$'),false)
        OR try_cast(split_part(wheel_versions[1],'.',2) AS BIGINT) IS NULL
        OR (cardinality(purelibs) IS DISTINCT FROM 1) OR purelibs[1] NOT IN ('true','false') OR tags IS NULL OR cardinality(tags)=0
    "#).await?,"wheel_contract","python_preparation").await?;
    let expected=session.sql(r#"
      WITH py AS (SELECT filename_parts,unnest(string_to_array(array_element(filename_parts,-3),'.')) AS py FROM wheel_values),
      abi AS (SELECT *,unnest(string_to_array(array_element(filename_parts,-2),'.')) AS abi FROM py),
      platform AS (SELECT *,unnest(string_to_array(array_element(filename_parts,-1),'.')) AS platform FROM abi)
      SELECT DISTINCT concat(py,'-',abi,'-',platform) AS tag FROM platform
    "#).await?;
    let observed = session
        .sql("SELECT DISTINCT unnest(tags) AS tag FROM wheel_values")
        .await?;
    let difference = expected
        .clone()
        .except_distinct(observed.clone())?
        .union(observed.except_distinct(expected)?)?;
    runtime
        .require_empty(difference, "wheel_filename_tags", "python_preparation")
        .await?;
    let warnings=session.sql(r#"
      SELECT 'Requires-Python has not been checked against a declared interpreter' AS text FROM distribution_values WHERE requires_python IS NOT NULL AND python_version IS NULL
      UNION ALL SELECT concat('Wheel-Version ',wheel_versions[1],' is newer than supported 1.0; unknown fields ignored') FROM wheel_values WHERE try_cast(split_part(wheel_versions[1],'.',2) AS BIGINT)>0
    "#).await?;
    Ok(runtime
        .records::<Warning>(warnings, 2)
        .await?
        .into_iter()
        .map(|row| row.text)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn inputs() -> (Vec<String>, Vec<Text>) {
        let texts=vec![Text{path:"pkg-1.0.dist-info/METADATA".into(),role:Role::Metadata,text:"Name: pkg\nVersion: 1.0.0\nRequires-Python: >=3.10\nRequires-Dist: dep>=1\n".into()},
            Text{path:"pkg-1.0.dist-info/WHEEL".into(),role:Role::Wheel,text:"Wheel-Version: 1.0\nRoot-Is-Purelib: true\nTag: py2-none-any\nTag: py3-none-any\n".into()}];
        let mut files = vec![
            "ns/part.py",
            "foo-stubs/__init__.pyi",
            "x.data/purelib/mapped.py",
            "x.data/scripts/ignored.py",
            "foo-stubs/py.typed",
            "src/__init__.py",
            "src/api.py",
            "native.so",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect::<Vec<_>>();
        files.extend(texts.iter().map(|text| text.path.clone()));
        (files, texts)
    }
    #[tokio::test]
    async fn native_distribution_preserves_layout_and_admits_metadata_identity() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let (files, texts) = inputs();
        let filename = "pkg-1.0-py2.py3-none-any.whl";
        let distribution =
            inventory(&runtime, &files, filename, &"a".repeat(64), "", &texts).await?;
        assert_eq!(
            distribution.import_roots,
            ["foo", "mapped", "native", "ns", "src"]
        );
        assert_eq!(distribution.files.len(), 5);
        assert_eq!(distribution.typed_markers, ["foo-stubs/py.typed"]);
        assert_eq!(distribution.native_files, ["native.so"]);
        assert!(distribution.files.iter().any(|f| f.module == "src.api"));
        assert!(distribution.files.iter().any(|f| f.module == "foo"
            && f.origin == enrichment_core::producer::python::ObservationOrigin::Stub));
        let mut request = ResolveRequest {
            name: "pkg".into(),
            python_version: Some("3.14".into()),
            ..Default::default()
        };
        assert!(
            admit(&runtime, &distribution, &texts, &request, "1.0")
                .await?
                .is_empty()
        );
        request.python_version = Some("3.9".into());
        assert!(
            admit(&runtime, &distribution, &texts, &request, "1.0")
                .await
                .is_err()
        );
        request.python_version = None;
        assert_eq!(
            admit(&runtime, &distribution, &texts, &request, "1.0")
                .await?
                .len(),
            1
        );
        request.python_version = Some("3.14".into());
        for change in [
            "Wheel-Version: 2.0\nRoot-Is-Purelib: true\nTag: py2-none-any\nTag: py3-none-any\n",
            "Wheel-Version: 1.0\nRoot-Is-Purelib: true\nTag: py3-none-any\n",
            "Wheel-Version: 1.0\nRoot-Is-Purelib: false\nRoot-Is-Purelib: false\nTag: py2-none-any\nTag: py3-none-any\n",
        ] {
            let mut changed = texts.clone();
            changed[1].text = change.into();
            assert!(
                admit(&runtime, &distribution, &changed, &request, "1.0")
                    .await
                    .is_err()
            );
        }
        let mut changed = texts.clone();
        changed[1].text = changed[1].text.replace("1.0", "1.1");
        assert_eq!(
            admit(&runtime, &distribution, &changed, &request, "1.0")
                .await?
                .len(),
            1
        );
        let mut duplicate = files.clone();
        duplicate.push("other.dist-info/METADATA".into());
        assert!(select(&runtime, &duplicate, filename).await.is_err());
        changed = texts.clone();
        changed[0].text.push_str("Name: pkg\n");
        assert!(
            inventory(&runtime, &files, filename, &"a".repeat(64), "", &changed)
                .await
                .is_err()
        );
        let source = inventory(
            &runtime,
            &["src/__init__.py".into(), "src/api.py".into()],
            "pkg.tar.gz",
            &"a".repeat(64),
            "",
            &[],
        )
        .await?;
        assert_eq!(source.import_roots, ["api"]);
        assert_eq!(source.files.len(), 1);
        for name in ["../pkg.whl", "pkg\\name.whl", "pkg%20.whl"] {
            assert!(select(&runtime, &files, name).await.is_err());
        }
        runtime.close_diagnostics().await
    }
}
