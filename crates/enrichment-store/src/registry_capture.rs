//! Exact-version registry facts use the same native definition and retention authority as jobs.
use crate::{control::ControlStore, immutable_definitions::Definitions, runtime::QueryRuntime};
use arrow::{datatypes::DataType, record_batch::RecordBatch};
use datafusion::{
    common::Result,
    dataframe::DataFrame,
    functions::core::expr_fn::coalesce,
    functions_aggregate::expr_fn::array_agg,
    prelude::{col, lit},
};
use enrichment_core::{
    evidence::{
        Artifact,
        arrow_model::expressions::{literal, record, variant},
    },
    native_union::{Cell, NativeStruct},
    operation::{
        retention::Dependency,
        sources::{RegistryCapture, RegistryFacts},
    },
};

#[derive(Clone)]
pub struct RegistryStore {
    definitions: Definitions<enrichment_core::identity::RegistryCaptureId>,
    control: ControlStore,
    runtime: QueryRuntime,
}

impl RegistryStore {
    pub fn new(control: ControlStore, runtime: QueryRuntime) -> Self {
        Self {
            definitions: Definitions::<enrichment_core::identity::RegistryCaptureId>::new(
                control.clone(),
                runtime.clone(),
            ),
            control,
            runtime,
        }
    }

    pub async fn rust_index(
        &self,
        artifact: &Artifact,
        accept: Option<&str>,
        batches: Vec<RecordBatch>,
    ) -> Result<DataFrame> {
        let input = rust_index_plan(&self.runtime, artifact, accept, batches)?;
        self.retain(artifact, input, "rust_index").await
    }

    pub async fn python_files(
        &self,
        artifact: &Artifact,
        accept: Option<&str>,
        batches: Vec<RecordBatch>,
    ) -> Result<DataFrame> {
        let frame = python_files_plan(&self.runtime, artifact, accept, batches)?;
        self.retain(artifact, frame, "python_files").await
    }

    pub async fn python_versions(
        &self,
        artifact: &Artifact,
        accept: Option<&str>,
        versions: &[String],
    ) -> Result<DataFrame> {
        let frame = python_versions_plan(&self.runtime, artifact, accept, versions)?;
        self.retain(artifact, frame, "python_versions").await
    }

    async fn retain(
        &self,
        artifact: &Artifact,
        input: DataFrame,
        variant: &str,
    ) -> Result<DataFrame> {
        self.control
            .retain_artifacts(&self.runtime, std::slice::from_ref(artifact))
            .await?;
        let (id, binding) = self
            .definitions
            .retain_plan(
                input,
                vec![Dependency::Artifact {
                    artifact_id: artifact.artifact_id.clone(),
                }],
            )
            .await?;
        // Reopen the exact captured version through the protected provider contract. Neither
        // current HTTP cache state nor a fresh registry head chooses the facts being queried.
        let selected = self.definitions.read(&id, &binding).await?;
        let session = self.runtime.session();
        crate::native_catalog::work(&session, "registry_capture", selected.into_view())?;
        // This finite tag is selected by the methods above, never a caller-supplied identifier.
        if variant == "python_versions" {
            session
                .sql(
                    "SELECT unnest(facts.python_versions.entries) AS version FROM registry_capture",
                )
                .await
        } else {
            session.sql(&format!("SELECT entry.* FROM (SELECT unnest(facts.{variant}.entries) AS entry FROM registry_capture)")).await
        }
    }
}

fn records<T: NativeStruct + Cell>(
    runtime: &QueryRuntime,
    batches: Vec<RecordBatch>,
) -> Result<DataFrame> {
    let input =
        crate::native_catalog::captured_batches(&runtime.session(), "registry_capture", batches)?;
    enrichment_core::native_schema::check_input(
        input.schema().as_arrow(),
        &arrow::datatypes::Schema::new(T::fields()),
    )?;
    let fields = T::fields();
    let values = fields
        .iter()
        .map(|field| (field.name().as_str(), col(field.name())))
        .collect::<Vec<_>>();
    let row = record(&DataType::Struct(fields.clone()), &values)?;
    input
        .aggregate(vec![], vec![array_agg(row).alias("entries")])?
        .select(vec![
            coalesce(vec![col("entries"), literal(&Vec::<T>::new())?]).alias("entries"),
        ])
}

fn capture(
    entries: DataFrame,
    artifact: &Artifact,
    accept: Option<&str>,
    tag: &str,
    decoder: &str,
) -> Result<DataFrame> {
    let decoder = enrichment_core::canonical::sha256_hex(
        format!("{decoder}\n{}", include_str!("../../../Cargo.lock")).as_bytes(),
    );
    let facts = variant(
        &RegistryFacts::data_type(),
        tag,
        &[("entries", col("entries"))],
    )?;
    let values = [
        ("artifact", literal(artifact)?),
        ("accept", literal(&accept.map(str::to_owned))?),
        ("decoder", lit(decoder)),
        ("facts", facts),
    ];
    // Projection follows the declaration, so adding a capture field cannot silently omit it.
    let row = record(&RegistryCapture::data_type(), &values)?;
    use datafusion::functions::core::expr_ext::FieldAccessor;
    entries.select(
        RegistryCapture::fields()
            .iter()
            .map(|field| row.clone().field(field.name()).alias(field.name()))
            .collect::<Vec<_>>(),
    )
}

/// Pure native plan builder used by retention and isolated source-contract tests.
pub fn rust_index_plan(
    runtime: &QueryRuntime,
    artifact: &Artifact,
    accept: Option<&str>,
    batches: Vec<RecordBatch>,
) -> Result<DataFrame> {
    capture(
        records::<enrichment_core::registry::facts::Fact>(runtime, batches)?,
        artifact,
        accept,
        "rust_index",
        concat!(
            include_str!("../../enrichment-core/src/registry/mod.rs"),
            include_str!("../../enrichment-core/src/registry/facts.rs"),
        ),
    )
}

fn python_files_plan(
    runtime: &QueryRuntime,
    artifact: &Artifact,
    accept: Option<&str>,
    batches: Vec<RecordBatch>,
) -> Result<DataFrame> {
    capture(
        records::<enrichment_core::producer::python::facts::Fact>(runtime, batches)?,
        artifact,
        accept,
        "python_files",
        concat!(
            include_str!("../../enrichment-core/src/producer/python.rs"),
            include_str!("../../enrichment-core/src/producer/python/facts.rs"),
        ),
    )
}

fn python_versions_plan(
    runtime: &QueryRuntime,
    artifact: &Artifact,
    accept: Option<&str>,
    versions: &[String],
) -> Result<DataFrame> {
    let input = crate::native_catalog::batch(
        &runtime.session(),
        "registry_capture",
        RecordBatch::try_from_iter([(
            "version",
            std::sync::Arc::new(arrow::array::StringArray::from(versions.to_vec()))
                as arrow::array::ArrayRef,
        )])?,
    )?
    .aggregate(vec![], vec![array_agg(col("version")).alias("entries")])?
    .select(vec![
        coalesce(vec![col("entries"), literal(&Vec::<String>::new())?]).alias("entries"),
    ])?;
    capture(
        input,
        artifact,
        accept,
        "python_versions",
        include_str!("../../enrichment-core/src/producer/python/registry.rs"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::QueryLimits;
    use enrichment_core::{evidence::ArtifactKind, native_time::AcquisitionTime};
    use std::collections::BTreeMap;

    #[tokio::test]
    async fn registry_capture_plans_preserve_source_scope_empty_facts_and_native_identity() {
        let directory = tempfile::tempdir().unwrap();
        let runtime = QueryRuntime::new(directory.path(), QueryLimits::default()).unwrap();
        let bytes = br#"{"name":"sample","vers":"1.0.0","cksum":"digest"}"#;
        let artifact = Artifact::describe(
            bytes,
            ArtifactKind::RegistryIndexEntry,
            "text/plain",
            "https://example.org/sample",
            AcquisitionTime::now().unwrap(),
        );
        let frame = rust_index_plan(
            &runtime,
            &artifact,
            Some("text/plain"),
            enrichment_core::registry::facts::decode(std::str::from_utf8(bytes).unwrap(), 1)
                .unwrap(),
        )
        .unwrap();
        let captured: RegistryCapture = runtime
            .records(frame.clone(), 1)
            .await
            .unwrap()
            .pop()
            .unwrap();
        match &captured.facts {
            RegistryFacts::RustIndex { entries } => {
                assert_eq!(entries.len(), 1);
                assert_eq!(entries[0].source_line, 1);
                assert_eq!(entries[0].release.vers, "1.0.0");
            }
            _ => panic!("wrong registry fact variant"),
        }
        enrichment_core::native_struct! { struct Identity { id: enrichment_core::identity::RegistryCaptureId => enrichment_core::native_union::Rule::Text } }
        let selected: Identity = runtime
            .records(
                frame
                    .select(vec![
                        enrichment_core::native_key::Key::RegistryCapture
                            .identity()
                            .unwrap()
                            .alias("id"),
                    ])
                    .unwrap(),
                1,
            )
            .await
            .unwrap()
            .pop()
            .unwrap();
        assert_eq!(
            selected.id,
            enrichment_core::identity::RegistryCaptureId::try_from_record(&captured).unwrap()
        );
        let mut changed = captured.clone();
        changed.accept = None;
        assert_ne!(
            selected.id,
            enrichment_core::identity::RegistryCaptureId::try_from_record(&changed).unwrap()
        );
        changed = captured.clone();
        changed.decoder.push('x');
        assert_ne!(
            selected.id,
            enrichment_core::identity::RegistryCaptureId::try_from_record(&changed).unwrap()
        );
        let mut relocated = artifact.clone();
        relocated.source_uri.push_str("/mirror");
        changed = captured;
        changed.artifact = relocated;
        assert_ne!(
            selected.id,
            enrichment_core::identity::RegistryCaptureId::try_from_record(&changed).unwrap()
        );
        let empty = rust_index_plan(
            &runtime,
            &artifact,
            None,
            enrichment_core::registry::facts::decode("", 1).unwrap(),
        )
        .unwrap();
        let captured: RegistryCapture = runtime.records(empty, 1).await.unwrap().pop().unwrap();
        assert!(
            matches!(captured.facts, RegistryFacts::RustIndex { entries } if entries.is_empty())
        );
        let empty = python_files_plan(
            &runtime,
            &artifact,
            None,
            enrichment_core::producer::python::facts::decode(&BTreeMap::new(), 1).unwrap(),
        )
        .unwrap();
        let captured: RegistryCapture = runtime.records(empty, 1).await.unwrap().pop().unwrap();
        assert!(
            matches!(captured.facts, RegistryFacts::PythonFiles { entries } if entries.is_empty())
        );
        let frame =
            python_versions_plan(&runtime, &artifact, None, &["1.0".into(), "2.0".into()]).unwrap();
        let captured: RegistryCapture = runtime.records(frame, 1).await.unwrap().pop().unwrap();
        assert!(
            matches!(captured.facts, RegistryFacts::PythonVersions { entries } if entries == ["1.0", "2.0"])
        );
        runtime.close_diagnostics().await.unwrap();
    }
}
