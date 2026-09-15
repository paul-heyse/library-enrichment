//! Bounded operation projections over one admitted, catalog-pinned snapshot.
use crate::{
    catalog_generation::PinnedCatalog,
    projection,
    repository::{EvidenceRepository, OpenedSnapshot},
    runtime::QueryRuntime,
};
use datafusion::{
    dataframe::DataFrame,
    error::DataFusionError,
    prelude::{SessionContext, col, lit},
};
use enrichment_core::{
    evidence::{
        EvidenceFragment, FragmentKind, Symbol,
        metadata::ReleaseMetadata,
        path::PublicPath,
        relational::{CoverageFact, InputArtifact, RelationshipObservation},
        snapshot::EvidenceManifest,
    },
    identity::SnapshotId,
    producer::ProducerRun,
    wire::data::NamespaceFacet,
};
use std::{collections::BTreeMap, path::PathBuf, sync::Arc};

pub struct SnapshotReader {
    ctx: SessionContext,
    opened: OpenedSnapshot,
    runtime: QueryRuntime,
}

#[derive(Debug, thiserror::Error)]
pub enum QueryError {
    #[error("snapshot {0} is not published")]
    NotPublished(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("query failed: {0}")]
    DataFusion(#[from] DataFusionError),
    #[error(transparent)]
    Arrow(#[from] arrow::error::ArrowError),
}

impl QueryError {
    #[must_use]
    pub fn is_budget(&self) -> bool {
        matches!(
            self,
            Self::DataFusion(DataFusionError::ResourcesExhausted(_))
        )
    }
}

pub struct Overview {
    pub definitions_by_kind: BTreeMap<String, u64>,
    pub namespaces: Vec<NamespaceFacet>,
    pub truncated_namespaces: u64,
    pub features: BTreeMap<String, String>,
    pub documentation_headings: Vec<String>,
    pub release_note_headings: Vec<String>,
    pub examples: Vec<String>,
    pub reexports: u64,
    pub unresolved_reexports: u64,
}

/// Filters lower to native predicates before the bounded alternatives page is decoded.
#[derive(Default)]
pub struct ExecutionSelection<'a> {
    pub symbol_id: Option<&'a str>,
    pub document_id: Option<&'a str>,
    pub kind: Option<&'a str>,
    pub methods: &'a [enrichment_core::evidence::execution::SemanticMethod],
    pub position: Option<enrichment_core::evidence::execution::Utf8Position>,
    pub runtime: Option<&'a enrichment_core::request::RuntimeSelection>,
    pub result_artifact_ids: &'a [String],
    pub image: Option<&'a str>,
    pub containment: Option<&'a str>,
    pub producer: Option<(&'a str, &'a str)>,
}

/// Closed fragment selection. Exact inspection refuses incomplete alternatives; overview
/// citations intentionally select a bounded sample, separate from complete evidence facts.
pub struct FragmentSelection<'a> {
    pub path: Option<&'a str>,
    pub symbol_id: Option<&'a str>,
    pub kinds: &'a [FragmentKind],
    pub limit: usize,
    pub require_complete: bool,
}

impl SnapshotReader {
    /// # Errors
    /// Membership, exact file integrity and admission must all succeed before domain views exist.
    pub async fn open(
        repository: &EvidenceRepository,
        catalog: Arc<PinnedCatalog>,
        id: &SnapshotId,
    ) -> Result<Self, QueryError> {
        if catalog.snapshot(&repository.runtime, id).await?.is_none() {
            return Err(QueryError::NotPublished(id.to_string()));
        }
        let opened = repository.open_snapshot(catalog, id).await?;
        let ctx = opened.research_session(&repository.runtime).await?;
        Ok(Self {
            ctx,
            opened,
            runtime: repository.runtime.clone(),
        })
    }
    #[must_use]
    pub fn manifest(&self) -> &EvidenceManifest {
        &self.opened.manifest
    }
    #[must_use]
    pub fn dir(&self) -> &PathBuf {
        &self.opened.directory
    }
    #[must_use]
    pub fn session(&self) -> &SessionContext {
        &self.ctx
    }
    #[must_use]
    pub fn runtime(&self) -> &QueryRuntime {
        &self.runtime
    }
    #[must_use]
    pub fn pinned(&self) -> &OpenedSnapshot {
        &self.opened
    }

    /// Dependency identities of static declarations, excluding mutable registry selection
    /// and execution-only inputs. This bounded identity projection never hydrates API text.
    pub async fn static_inputs(&self) -> Result<Vec<(String, String)>, QueryError> {
        let frame = self.ctx.sql("WITH bindings AS (SELECT source.producer_binding_id AS id FROM api_observations UNION SELECT source.producer_binding_id AS id FROM fragments WHERE source.evidence_class IN ('declared', 'statically_extracted')) SELECT DISTINCT i.sha256, i.source_uri FROM input_artifacts i LEFT SEMI JOIN bindings b ON i.producer_binding_id = b.id WHERE i.kind NOT IN ('registry_index_entry', 'registry_version_metadata') ORDER BY i.sha256, i.source_uri LIMIT 8193").await?;
        let out = self.runtime.execute(frame).await?;
        if out.rows > 8192 {
            return Err(DataFusionError::ResourcesExhausted(
                "static dependency identity projection exceeds 8192 inputs".into(),
            )
            .into());
        }
        let digests = projection::render::strings(&out.batches, "sha256")?;
        let sources = projection::render::strings(&out.batches, "source_uri")?;
        Ok(digests.into_iter().zip(sources).collect())
    }

    /// Native retained execution selection, with a sentinel rather than silent alternatives loss.
    /// # Errors
    /// Excessive alternatives or admission/query failures remain explicit.
    pub async fn execution_observations(
        &self,
        symbol_id: Option<&str>,
        document_id: Option<&str>,
    ) -> Result<Vec<enrichment_core::evidence::execution::ExecutionObservation>, QueryError> {
        self.execution_selection(ExecutionSelection {
            symbol_id,
            document_id,
            ..Default::default()
        })
        .await
    }

    pub async fn execution_selection(
        &self,
        selection: ExecutionSelection<'_>,
    ) -> Result<Vec<enrichment_core::evidence::execution::ExecutionObservation>, QueryError> {
        use datafusion::functions::core::expr_ext::FieldAccessor;
        let mut plan = self.ctx.table("execution_observations").await?;
        if let Some(image) = selection.image {
            plan = plan.filter(col("image_id").eq(lit(image)))?;
        }
        if let Some(containment) = selection.containment {
            plan = plan.filter(col("containment_identity").eq(lit(containment)))?;
        }
        if let Some((name, version)) = selection.producer {
            plan = plan.filter(
                col("source")
                    .field("extractor")
                    .eq(lit(name))
                    .and(col("source").field("extractor_version").eq(lit(version))),
            )?;
        }
        if let Some(symbol) = selection.symbol_id {
            plan = plan.filter(
                col("subject")
                    .field("symbol_id")
                    .eq(lit(symbol))
                    .or(col("payload")
                        .field("semantic_query")
                        .field("anchor_symbol_id")
                        .eq(lit(symbol))),
            )?;
        }
        if let Some(document) = selection.document_id {
            plan = plan.filter(col("subject").field("artifact_id").eq(lit(document)))?;
        }
        if let Some(kind) = selection.kind {
            plan = plan.filter(col("payload").field("kind").eq(lit(kind)))?;
        }
        let semantic = col("payload").field("semantic_query");
        if !selection.methods.is_empty() {
            plan = plan.filter(semantic.clone().field("method").in_list(
                selection.methods.iter().map(|m| lit(m.as_str())).collect(),
                false,
            ))?;
        }
        if let Some(position) = selection.position {
            let point = semantic.clone().field("position");
            plan = plan.filter(
                semantic.field("method").eq(lit("diagnostics")).or(point
                    .clone()
                    .field("line")
                    .eq(lit(position.line))
                    .and(point.field("byte").eq(lit(position.byte)))),
            )?;
        }
        if let Some(runtime) = selection.runtime {
            let object = col("payload").field("runtime_object");
            // Validated import components cannot contain '.', making this rendering injective.
            let path = datafusion::functions_nested::string::array_to_string_udf()
                .call(vec![object.clone().field("selection"), lit(".")]);
            plan = plan.filter(
                object
                    .field("module")
                    .eq(lit(&runtime.module))
                    .and(path.eq(lit(runtime.attributes.join(".")))),
            )?;
        }
        if !selection.result_artifact_ids.is_empty() {
            plan = plan.filter(col("source").field("artifact_id").in_list(
                selection.result_artifact_ids.iter().map(lit).collect(),
                false,
            ))?;
        }
        let output = self
            .runtime
            .execute(
                plan.sort(vec![col("observation_id").sort(true, false)])?
                    .limit(0, Some(65))?,
            )
            .await?;
        if output.rows > 64 {
            return Err(DataFusionError::ResourcesExhausted(
                "more than 64 retained execution alternatives; refine query".into(),
            )
            .into());
        }
        let mut observations = Vec::new();
        for batch in output.batches {
            observations.extend(projection::execution::decode(&batch)?);
        }
        Ok(observations)
    }
    /// # Errors
    /// Invalid public paths are errors, never a broadened namespace.
    pub fn area(&self, display: &str) -> Result<PublicPath, QueryError> {
        PublicPath::parse(self.manifest().metadata.ecosystem, display)
            .map_err(|e| DataFusionError::Plan(e).into())
    }

    async fn render_symbols(
        &self,
        frame: DataFrame,
        docs: bool,
    ) -> Result<Vec<Symbol>, QueryError> {
        // A small exact inspection can have multiple declaration/trait alternatives. The
        // sentinel is checked before rendering rather than silently dropping alternatives.
        let output = self
            .runtime
            .execute(
                frame
                    .sort(vec![
                        col("symbol_id").sort(true, false),
                        col("origin").sort(true, true),
                        col("observation_id").sort(true, true),
                    ])?
                    .limit(0, Some(1025))?,
            )
            .await?;
        if output.rows > 1024 {
            return Err(DataFusionError::ResourcesExhausted(
                "inspection has more than 1024 observation rows; refine the path".into(),
            )
            .into());
        }
        Ok(projection::render::symbols(&output.batches, docs)?)
    }

    /// # Errors
    /// Exact lookup is bounded and retains same-path definition/observation alternatives.
    pub async fn symbols_at(
        &self,
        path: &str,
        definition_id: Option<&str>,
        docs: bool,
    ) -> Result<Vec<Symbol>, QueryError> {
        let mut predicate = col("path").eq(lit(path));
        if let Some(id) = definition_id {
            predicate = predicate.and(col("definition_id").eq(lit(id)));
        }
        self.render_symbols(
            self.ctx
                .table(if docs {
                    "api_surface"
                } else {
                    "inspection_surface"
                })
                .await?
                .filter(predicate)?,
            docs,
        )
        .await
    }

    /// # Errors
    /// Unqualified lookup uses literal typed suffix components in the snapshot's ecosystem.
    pub async fn symbols_ending_with(
        &self,
        suffix: &str,
        definition_id: Option<&str>,
        docs: bool,
    ) -> Result<Vec<Symbol>, QueryError> {
        use datafusion::{
            common::ScalarValue,
            functions_nested::expr_fn::{array_length, array_slice},
        };
        let path = self.area(suffix)?;
        let parts = path
            .components()
            .iter()
            .map(|p| ScalarValue::Utf8(Some(p.clone())))
            .collect::<Vec<_>>();
        let length = array_length(col("components"));
        let mut predicate = array_slice(
            col("components"),
            length.clone() - lit(parts.len() as i64) + lit(1i64),
            length,
            None,
        )
        .eq(lit(ScalarValue::List(ScalarValue::new_list(
            &parts,
            &arrow::datatypes::DataType::Utf8,
            false,
        ))));
        if let Some(id) = definition_id {
            predicate = predicate.and(col("definition_id").eq(lit(id)));
        }
        self.render_symbols(
            self.ctx
                .table(if docs {
                    "api_surface"
                } else {
                    "inspection_surface"
                })
                .await?
                .filter(predicate)?,
            docs,
        )
        .await
    }

    /// # Errors
    /// Alias discovery filters the definition before materializing a bounded path list.
    pub async fn aliases_for(
        &self,
        definition: &str,
        except_path: &str,
    ) -> Result<Vec<String>, QueryError> {
        let output = self
            .runtime
            .execute(
                self.ctx
                    .table("symbols")
                    .await?
                    .filter(
                        col("definition_id")
                            .eq(lit(definition))
                            .and(col("path").not_eq(lit(except_path))),
                    )?
                    .select(vec![col("path")])?
                    .distinct()?
                    .sort(vec![col("path").sort(true, false)])?,
            )
            .await?;
        Ok(projection::render::strings(&output.batches, "path")?)
    }

    /// # Errors
    /// Every independently qualified API observation of the selected binding is retained.
    pub async fn observations_for(
        &self,
        symbol_id: &str,
        docs: bool,
    ) -> Result<Vec<enrichment_core::wire::data::ApiObservationProjection>, QueryError> {
        let (base, bound) = if docs {
            ("api_observations", "bound_observations")
        } else {
            ("inspection_observations", "inspection_bound")
        };
        let sql = format!(
            "SELECT o.* FROM {base} o LEFT SEMI JOIN {bound} b ON o.observation_id = b.observation_id AND b.binding_id = $1"
        );
        let frame = self
            .ctx
            .sql(&sql)
            .await?
            .with_param_values(vec![datafusion::common::ScalarValue::from(symbol_id)])?;
        let output = self
            .runtime
            .execute(
                frame
                    .sort(vec![col("observation_id").sort(true, false)])?
                    .limit(0, Some(1025))?,
            )
            .await?;
        if output.rows > 1024 {
            return Err(DataFusionError::ResourcesExhausted(
                "inspection has more than 1024 qualified observations".into(),
            )
            .into());
        }
        Ok(projection::render::api_observations(&output.batches, docs)?)
    }

    async fn render_fragments(
        &self,
        frame: DataFrame,
    ) -> Result<Vec<EvidenceFragment>, QueryError> {
        let output = self
            .runtime
            .execute(frame.sort(vec![col("fragment_id").sort(true, false)])?)
            .await?;
        Ok(projection::render::fragments(&output.batches)?)
    }
    /// # Errors
    /// Resolve symbol/definition subjects through binding membership before materialization.
    pub async fn fragments(
        &self,
        selection: FragmentSelection<'_>,
    ) -> Result<Vec<EvidenceFragment>, QueryError> {
        if selection.limit == 0 || selection.limit > 1024 || selection.kinds.is_empty() {
            return Err(DataFusionError::Plan(
                "fragment kinds and a limit in 1..=1024 are required".into(),
            )
            .into());
        }
        let mut frame = self.ctx.table("fragment_surface").await?;
        if let Some(symbol) = selection.symbol_id {
            frame = self.ctx.sql("SELECT f.* FROM fragment_surface f LEFT SEMI JOIN fragment_paths p ON f.fragment_id = p.fragment_id AND p.symbol_id = $1").await?.with_param_values(vec![datafusion::common::ScalarValue::from(symbol)])?;
        } else if let Some(path) = selection.path {
            frame = self.ctx.sql("SELECT f.* FROM fragment_surface f LEFT SEMI JOIN (SELECT DISTINCT p.fragment_id FROM fragment_paths p JOIN symbols s ON p.symbol_id = s.symbol_id WHERE s.path = $1) m ON f.fragment_id = m.fragment_id").await?.with_param_values(vec![datafusion::common::ScalarValue::from(path)])?;
        }
        frame = frame
            .filter(
                col("kind").in_list(
                    selection
                        .kinds
                        .iter()
                        .map(|kind| lit(kind.as_str()))
                        .collect(),
                    false,
                ),
            )?
            .sort(vec![col("fragment_id").sort(true, false)])?
            .limit(
                0,
                Some(selection.limit + usize::from(selection.require_complete)),
            )?;
        let output = self.runtime.execute(frame).await?;
        if output.rows > selection.limit {
            return Err(DataFusionError::ResourcesExhausted(
                "fragment alternatives exceed the requested complete selection".into(),
            )
            .into());
        }
        Ok(projection::render::fragments(&output.batches)?)
    }
    /// # Errors
    /// Example eligibility remains literal and the plan limits the final citations.
    pub async fn examples_for(&self, name: &str) -> Result<Vec<EvidenceFragment>, QueryError> {
        let spec = enrichment_core::search::spec::SearchSpec::new(name);
        let frame = self
            .ctx
            .table("fragment_surface")
            .await?
            .filter(
                col("kind")
                    .eq(lit(FragmentKind::Example.as_str()))
                    .and(crate::scoring::fragment_eligibility(&spec.fragment_clauses)),
            )?
            .sort(vec![col("fragment_id").sort(true, false)])?
            .limit(0, Some(3))?;
        self.render_fragments(frame).await
    }
    /// # Errors
    /// Explicit symbol and definition references remain distinct, including external targets.
    pub async fn relationships_for(
        &self,
        symbol_id: &str,
        limit: usize,
    ) -> Result<Vec<RelationshipObservation>, QueryError> {
        let frame = self
            .ctx
            .sql(
                r"SELECT r.* FROM relationships r LEFT SEMI JOIN symbols s ON (
            r.subject.symbol_id = s.symbol_id OR r.subject.definition_id = s.definition_id OR
            r.target.symbol_id = s.symbol_id OR r.target.definition_id = s.definition_id
            ) AND s.symbol_id = $1",
            )
            .await?
            .with_param_values(vec![datafusion::common::ScalarValue::from(symbol_id)])?;
        let limit = limit.clamp(1, 1024);
        let output = self
            .runtime
            .execute(
                frame
                    .sort(vec![col("relationship_id").sort(true, false)])?
                    .limit(0, Some(limit + 1))?,
            )
            .await?;
        if output.rows > limit {
            return Err(DataFusionError::ResourcesExhausted(
                "relationship alternatives exceed inspection bound".into(),
            )
            .into());
        }
        Ok(output
            .batches
            .iter()
            .map(projection::relationships_from_batch)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect())
    }

    /// # Errors
    /// Facets are computed by native aggregates/windows with bounded final batches.
    pub async fn overview(
        &self,
        area: Option<&str>,
        per_namespace: usize,
    ) -> Result<Overview, QueryError> {
        let area = area.map(|a| self.area(a)).transpose()?;
        let page = crate::browse::overview(
            &self.ctx,
            &self.runtime,
            area.as_ref(),
            per_namespace.min(128),
            (10_000 / per_namespace.max(1)).min(256),
        )
        .await?;
        let feature_rows = self
            .runtime
            .execute(
                self.ctx
                    .table("fragment_surface")
                    .await?
                    .filter(col("kind").eq(lit(FragmentKind::FeatureDefinition.as_str())))?
                    .select(vec![col("label"), col("text")])?
                    .distinct()?
                    .sort(vec![
                        col("label").sort(true, false),
                        col("text").sort(true, false),
                    ])?
                    .limit(0, Some(129))?,
            )
            .await?;
        if feature_rows.rows > 128 {
            return Err(DataFusionError::ResourcesExhausted(
                "feature summary exceeds 128 qualified entries".into(),
            )
            .into());
        }
        let subjects = projection::render::strings(&feature_rows.batches, "label")?;
        let texts = projection::render::strings(&feature_rows.batches, "text")?;
        let mut features = BTreeMap::new();
        for (subject, text) in subjects.into_iter().zip(texts) {
            if features.insert(subject, text).is_some() {
                return Err(DataFusionError::ResourcesExhausted(
                    "feature summary has conflicting qualified definitions; retrieve the feature evidence explicitly".into(),
                ).into());
            }
        }
        let mut headings = BTreeMap::new();
        for kind in [
            FragmentKind::ReadmeSection,
            FragmentKind::ChangelogSection,
            FragmentKind::Example,
        ] {
            let output = self
                .runtime
                .execute(
                    self.ctx
                        .table("fragment_surface")
                        .await?
                        .filter(col("kind").eq(lit(kind.as_str())))?
                        .select(vec![col("label")])?
                        .distinct()?
                        .sort(vec![col("label").sort(true, false)])?
                        .limit(0, Some(257))?,
                )
                .await?;
            if output.rows > 256 {
                return Err(DataFusionError::ResourcesExhausted(
                    "overview headings exceed 256 entries".into(),
                )
                .into());
            }
            headings.insert(
                kind.as_str(),
                projection::render::strings(&output.batches, "label")?,
            );
        }
        Ok(Overview {
            definitions_by_kind: page.definitions_by_kind,
            namespaces: page.namespaces,
            truncated_namespaces: page.truncated_namespaces,
            features,
            documentation_headings: headings
                .remove(FragmentKind::ReadmeSection.as_str())
                .unwrap_or_default(),
            release_note_headings: headings
                .remove(FragmentKind::ChangelogSection.as_str())
                .unwrap_or_default(),
            examples: headings
                .remove(FragmentKind::Example.as_str())
                .unwrap_or_default(),
            reexports: self.manifest().counts.reexports,
            unresolved_reexports: self.manifest().counts.unresolved_reexports,
        })
    }

    /// # Errors
    /// Input references are selected from the exact admitted snapshot.
    pub async fn inputs(&self) -> Result<Vec<InputArtifact>, QueryError> {
        let output = self
            .runtime
            .execute(self.ctx.table("input_artifacts").await?)
            .await?;
        Ok(output
            .batches
            .iter()
            .map(projection::input_artifacts_from_batch)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect())
    }
    /// # Errors
    /// Artifact selection is performed before decoding input descriptors.
    pub async fn inputs_of_kind(
        &self,
        kind: enrichment_core::evidence::ArtifactKind,
    ) -> Result<Vec<InputArtifact>, QueryError> {
        let output = self
            .runtime
            .execute(
                self.ctx
                    .table("input_artifacts")
                    .await?
                    .filter(col("kind").eq(lit(kind.as_str())))?,
            )
            .await?;
        Ok(output
            .batches
            .iter()
            .map(projection::input_artifacts_from_batch)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect())
    }
    /// # Errors
    /// Role lookup never falls back to a different acquisition.
    pub async fn inputs_for_role(&self, role: &str) -> Result<Vec<InputArtifact>, QueryError> {
        let output = self
            .runtime
            .execute(
                self.ctx
                    .table("input_artifacts")
                    .await?
                    .filter(col("role").eq(lit(role)))?,
            )
            .await?;
        Ok(output
            .batches
            .iter()
            .map(projection::input_artifacts_from_batch)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect())
    }
    /// # Errors
    /// Returned attempts belong to this pinned snapshot's catalog generation.
    pub async fn producer_runs(&self) -> Result<Vec<ProducerRun>, QueryError> {
        let session = self.opened.catalog.session(&self.runtime).await?;
        let output = self
            .runtime
            .execute(
                session
                    .table("attempts")
                    .await?
                    .filter(col("snapshot_id").eq(lit(self.manifest().snapshot_id.as_str())))?,
            )
            .await?;
        Ok(output
            .batches
            .iter()
            .map(projection::catalog::attempts_from_batch)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .map(|a| a.run)
            .collect())
    }
    /// # Errors
    /// Exact attempt lookup refuses ambiguous identities rather than selecting a receipt.
    pub async fn attempt(
        &self,
        id: &str,
    ) -> Result<enrichment_core::evidence::catalog::SnapshotAttempt, QueryError> {
        let session = self.opened.catalog.session(&self.runtime).await?;
        let output = self
            .runtime
            .execute(
                session
                    .table("attempts")
                    .await?
                    .filter(
                        col("snapshot_id")
                            .eq(lit(self.manifest().snapshot_id.as_str()))
                            .and(col("attempt_id").eq(lit(id))),
                    )?
                    .limit(0, Some(2))?,
            )
            .await?;
        if output.rows != 1 {
            return Err(DataFusionError::Execution(
                "expected one qualified producer attempt".into(),
            )
            .into());
        }
        let mut rows = Vec::new();
        for batch in output.batches {
            rows.extend(projection::catalog::attempts_from_batch(&batch)?);
        }
        rows.pop()
            .ok_or_else(|| DataFusionError::Execution("producer attempt disappeared".into()).into())
    }
    /// # Errors
    /// Acquisition-specific URIs, clocks and validators come from actual catalog attempts.
    pub async fn artifacts(&self) -> Result<Vec<enrichment_core::evidence::Artifact>, QueryError> {
        self.catalog_artifacts(
            "WITH raw AS (
            SELECT unnest(acquisitions) AS artifact, started_at, attempt_id FROM attempts
            WHERE snapshot_id = $1
        ), ranked AS (
            SELECT artifact, row_number() OVER (
                PARTITION BY artifact.artifact_id, artifact.source_uri
                ORDER BY started_at, attempt_id
            ) AS position FROM raw
        ) SELECT artifact FROM ranked WHERE position = 1
          ORDER BY artifact.artifact_id, artifact.source_uri",
        )
        .await
    }
    /// Complete presentation artifacts are operational catalog references, not producer inputs.
    /// # Errors
    /// Invalid catalog rows and bounded selection errors are explicit.
    pub async fn job_deliveries(
        &self,
    ) -> Result<Vec<enrichment_core::evidence::Artifact>, QueryError> {
        self.catalog_artifacts(
            "SELECT artifact FROM (
            SELECT delivery AS artifact, row_number() OVER (
                PARTITION BY delivery.artifact_id ORDER BY job_id
            ) AS position FROM job_publications WHERE snapshot_id = $1
        ) WHERE position = 1 ORDER BY artifact.artifact_id",
        )
        .await
    }
    /// Bounded operational outputs associated with actual attempts, outside snapshot identity.
    /// # Errors
    /// The catalog must supply an exact acquisition descriptor for every non-null log.
    pub async fn attempt_logs(
        &self,
    ) -> Result<Vec<enrichment_core::evidence::Artifact>, QueryError> {
        self.catalog_artifacts(
            "WITH raw AS (
            SELECT unnest(acquisitions) AS artifact, log, started_at, attempt_id
            FROM attempts WHERE snapshot_id = $1 AND log IS NOT NULL
        ), ranked AS (
            SELECT artifact, row_number() OVER (
                PARTITION BY artifact.artifact_id ORDER BY started_at, attempt_id
            ) AS position FROM raw WHERE artifact.artifact_id = log
        ) SELECT artifact FROM ranked WHERE position = 1 ORDER BY artifact.artifact_id",
        )
        .await
    }
    async fn catalog_artifacts(
        &self,
        sql: &str,
    ) -> Result<Vec<enrichment_core::evidence::Artifact>, QueryError> {
        let session = self.opened.catalog.session(&self.runtime).await?;
        let plan = session.sql(sql).await?.with_param_values(vec![
            datafusion::common::ScalarValue::from(self.manifest().snapshot_id.as_str()),
        ])?;
        let output = self.runtime.execute(plan).await?;
        let mut artifacts = Vec::new();
        for batch in output.batches {
            artifacts.extend(projection::catalog::selected_artifacts(&batch)?);
        }
        Ok(artifacts)
    }

    /// # Errors
    /// Coverage remains explicit for successful empty and missing scopes.
    pub async fn coverage(&self) -> Result<Vec<CoverageFact>, QueryError> {
        let output = self
            .runtime
            .execute(self.ctx.table("coverage").await?)
            .await?;
        Ok(output
            .batches
            .iter()
            .map(projection::coverage_from_batch)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect())
    }
    /// # Errors
    /// Offline resolution never re-runs archive extraction to recreate these facts.
    pub async fn release_metadata(&self) -> Result<Vec<ReleaseMetadata>, QueryError> {
        let output = self
            .runtime
            .execute(self.ctx.table("release_metadata").await?)
            .await?;
        Ok(output
            .batches
            .iter()
            .map(projection::metadata::decode)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect())
    }
}
