//! Bounded operation projections over one admitted, catalog-pinned snapshot.
use crate::{
    control::ControlSnapshot,
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
        FragmentKind, SymbolHeader, TextFragment,
        path::PublicPath,
        relational::{InputArtifact, RelationshipObservation},
        snapshot::EvidenceManifest,
    },
    identity::SnapshotId,
    producer::ProducerRun,
    wire::data::NamespaceFacet,
};
use std::{collections::BTreeMap, sync::Arc};

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
            self.diagnostic().cause,
            enrichment_core::wire::DiagnosticCause::Capacity
                | enrichment_core::wire::DiagnosticCause::Deadline
        )
    }
}

pub struct Overview {
    pub definitions_by_kind: BTreeMap<String, u64>,
    pub namespaces: Vec<NamespaceFacet>,
    pub truncated_namespaces: u64,
    pub reexports: u64,
    pub unresolved_reexports: u64,
}

/// Native lookup policy outputs; the transport only renders the selected branch.
pub struct InspectionBindings {
    pub definition_count: u64,
    pub selected: Option<SymbolHeader>,
    pub candidates: Vec<enrichment_core::wire::data::InspectionCandidate>,
}

enrichment_core::native_struct! {
    struct DefinitionCount {
        count: u64 => enrichment_core::native_union::Rule::Text,
    }
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

/// A sentinel proves continuation without claiming an unexecuted exact count.
pub struct NativePage<T> {
    pub items: Vec<T>,
    pub has_more: bool,
    pub next_key: Option<String>,
}

impl<T> NativePage<T> {
    fn selected(
        items: Vec<T>,
        boundary: crate::page_plan::Boundary,
        key: impl Fn(&T) -> &str,
    ) -> Result<Self, QueryError> {
        if items.len() as u64 != boundary.returned {
            return Err(DataFusionError::Internal(
                "native page decoder changed row cardinality".into(),
            )
            .into());
        }
        let next_key = boundary
            .has_more
            .then(|| items.last().map(|item| key(item).to_owned()))
            .flatten();
        Ok(Self {
            items,
            has_more: boundary.has_more,
            next_key,
        })
    }
}

struct PageOutput {
    batches: Vec<arrow::record_batch::RecordBatch>,
    boundary: crate::page_plan::Boundary,
}

impl SnapshotReader {
    async fn page_selection(
        &self,
        mut frame: DataFrame,
        key: &str,
        family: crate::preparation::QueryFamily,
        limit: usize,
        after: Option<&str>,
    ) -> Result<crate::page_plan::Selected, QueryError> {
        family.require(frame.schema().as_arrow())?;
        if let Some(after) = after {
            frame = frame.filter(col(key).gt(lit(after)))?;
        }
        let selected = crate::page_plan::select(
            &self.runtime,
            frame.sort(vec![col(key).sort(true, false)])?,
            crate::page_plan::Policy {
                page_size: limit as u64,
                offset: 0,
                total: None,
                detail: false,
            },
        )
        .await?;
        Ok(selected)
    }

    async fn page_rows(
        &self,
        frame: DataFrame,
        key: &str,
        family: crate::preparation::QueryFamily,
        limit: usize,
        after: Option<&str>,
    ) -> Result<PageOutput, QueryError> {
        let selected = self
            .page_selection(frame, key, family, limit, after)
            .await?;
        let output = self
            .runtime
            .execute_family(selected.frame, Some(family))
            .await?;
        Ok(PageOutput {
            batches: output.batches,
            boundary: selected.boundary,
        })
    }

    /// Independently page qualified API observations before decoding selected leaves.
    pub async fn observation_page(
        &self,
        symbol: &str,
        docs: bool,
        limit: usize,
        after: Option<&str>,
    ) -> Result<NativePage<enrichment_core::wire::data::ApiObservationProjection>, QueryError> {
        let (base, bound) = if docs {
            ("api_observations", "bound_observations")
        } else {
            ("inspection_observations", "inspection_bound")
        };
        let frame = self.ctx.sql(&format!("SELECT o.* FROM snapshot.evidence.{base} o LEFT SEMI JOIN snapshot.domain.{bound} b ON o.observation_id = b.observation_id AND b.binding_id = $1"))
            .await?.with_param_values(vec![datafusion::common::ScalarValue::from(symbol)])?;
        let out = self
            .page_rows(
                frame,
                "observation_id",
                crate::preparation::QueryFamily::Observations { docs },
                limit,
                after,
            )
            .await?;
        NativePage::selected(
            projection::render::api_observations(&out.batches, docs)?,
            out.boundary,
            |item| item.observation_id.as_str(),
        )
    }

    /// Consensus over independently qualified ancillary observations.
    pub async fn ancillary_facts(
        &self,
        symbol: &str,
    ) -> Result<enrichment_core::evidence::model::AncillaryFacts, QueryError> {
        let plan = self
            .ctx
            .sql(
                "SELECT payload.cfg_hints AS cfg_hints, source.locator AS locator
            FROM snapshot.evidence.inspection_observations o
            LEFT SEMI JOIN snapshot.domain.inspection_bound b
                ON o.observation_id=b.observation_id AND b.binding_id=$1",
            )
            .await?
            .with_param_values(vec![datafusion::common::ScalarValue::from(symbol)])?;
        Ok(crate::research_inspection::ancillary(&self.runtime, plan).await?)
    }

    /// Relationship identities preserve direction, self edges and external endpoints.
    pub async fn relationship_page(
        &self,
        symbol: &str,
        limit: usize,
        after: Option<&str>,
    ) -> Result<NativePage<RelationshipObservation>, QueryError> {
        let frame = self
            .ctx
            .sql(
                "SELECT r.* FROM snapshot.evidence.relationships r LEFT SEMI JOIN snapshot.evidence.symbols s ON (
            r.subject.symbol.symbol_id = s.symbol_id OR r.subject.definition.definition_id = s.definition_id OR
            r.target.symbol.symbol_id = s.symbol_id OR r.target.definition.definition_id = s.definition_id)
            AND s.symbol_id = $1",
            )
            .await?
            .with_param_values(vec![datafusion::common::ScalarValue::from(symbol)])?;
        let out = self
            .page_rows(
                frame,
                "relationship_id",
                crate::preparation::QueryFamily::Relation(
                    crate::admission::Relation::Relationships,
                ),
                limit,
                after,
            )
            .await?;
        let items = out
            .batches
            .iter()
            .map(projection::relationships_from_batch)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect();
        NativePage::selected(items, out.boundary, |item| item.relationship_id.as_str())
    }

    /// SymbolHeader documentation and examples are reachable without complete-set hydration.
    pub async fn navigation_page(
        &self,
        symbol: &str,
        members: bool,
        limit: usize,
        after: Option<&str>,
    ) -> Result<NativePage<SymbolHeader>, QueryError> {
        let sql = if members {
            "SELECT h.* FROM snapshot.domain.symbol_headers h LEFT SEMI JOIN (
                SELECT r.subject FROM snapshot.evidence.relationships r JOIN snapshot.evidence.symbols owner
                ON r.target.symbol.symbol_id = owner.symbol_id OR r.target.definition.definition_id = owner.definition_id
                WHERE owner.symbol_id = $1 AND r.relation = 'member_of'
            ) m ON m.subject.symbol.symbol_id = h.symbol_id OR m.subject.definition.definition_id = h.definition_id"
        } else {
            "SELECT h.* FROM snapshot.domain.symbol_headers h LEFT SEMI JOIN (
                SELECT m.symbol_id FROM snapshot.domain.namespace_members m JOIN snapshot.evidence.symbols owner
                ON m.namespace_components = owner.components AND m.ecosystem = owner.ecosystem
                WHERE owner.symbol_id = $1 AND m.is_direct
            ) c ON c.symbol_id = h.symbol_id"
        };
        let frame = self
            .ctx
            .sql(sql)
            .await?
            .with_param_values(vec![datafusion::common::ScalarValue::from(symbol)])?;
        let out = self
            .page_rows(
                frame,
                "symbol_id",
                crate::preparation::QueryFamily::SymbolHeaders,
                limit,
                after,
            )
            .await?;
        NativePage::selected(
            projection::render::symbol_headers(&out.batches)?,
            out.boundary,
            |s| s.symbol_id.as_str(),
        )
    }

    /// SymbolHeader documentation and examples are reachable without complete-set hydration.
    pub async fn fragment_page(
        &self,
        symbol: &str,
        kinds: &[FragmentKind],
        limit: usize,
        after: Option<&str>,
        max_characters: Option<usize>,
    ) -> Result<NativePage<crate::research_fragments::SelectedFragment>, QueryError> {
        self.fragment_projection(Some(symbol), kinds, limit, after, max_characters)
            .await
    }

    pub async fn discovery_page(
        &self,
        kind: FragmentKind,
        limit: usize,
        after: Option<&str>,
        max_characters: Option<usize>,
    ) -> Result<NativePage<crate::research_fragments::SelectedFragment>, QueryError> {
        self.fragment_projection(None, &[kind], limit, after, max_characters)
            .await
    }

    async fn fragment_projection(
        &self,
        symbol: Option<&str>,
        kinds: &[FragmentKind],
        limit: usize,
        after: Option<&str>,
        max_characters: Option<usize>,
    ) -> Result<NativePage<crate::research_fragments::SelectedFragment>, QueryError> {
        let relation = if symbol.is_some() {
            "snapshot.domain.fragment_surface f LEFT SEMI JOIN snapshot.domain.fragment_paths p ON f.fragment_id = p.fragment_id AND p.symbol_id = $2"
        } else {
            "snapshot.domain.fragment_surface f"
        };
        let sql = format!(
            "SELECT f.fragment_id, f.kind, f.subject_ref, f.source, f.label,
            f.path_id, f.components, f.ecosystem, f.definition_id, f.symbol_id,
            substring(f.text FROM 1 FOR $1) AS text, character_length(f.text) <= $1 AS text_complete
            FROM {relation}"
        );
        let mut params = vec![datafusion::common::ScalarValue::from(
            max_characters
                .map(i64::try_from)
                .transpose()
                .map_err(|error| DataFusionError::Plan(error.to_string()))?
                .unwrap_or(i64::MAX),
        )];
        if let Some(symbol) = symbol {
            params.push(datafusion::common::ScalarValue::from(symbol));
        }
        let frame = self
            .ctx
            .sql(&sql)
            .await?
            .with_param_values(params)?
            .filter(col("kind").in_list(kinds.iter().map(|k| lit(k.as_str())).collect(), false))?;
        let selected = self
            .page_selection(
                frame,
                "fragment_id",
                crate::preparation::QueryFamily::FragmentProjection { bounded: true },
                limit,
                after,
            )
            .await?;
        let items = self
            .runtime
            .records(
                crate::research_fragments::from_surface(selected.frame)?,
                limit,
            )
            .await?;
        NativePage::selected(
            items,
            selected.boundary,
            |item: &crate::research_fragments::SelectedFragment| item.fragment.fragment_id.as_str(),
        )
    }

    /// # Errors
    /// Membership, exact file integrity and admission must all succeed before domain views exist.
    pub async fn open(
        repository: &EvidenceRepository,
        catalog: Arc<ControlSnapshot>,
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
    async fn static_input_plan(&self) -> Result<DataFrame, QueryError> {
        Ok(self.ctx.sql("WITH bindings AS (SELECT source.producer_binding_id AS id FROM snapshot.evidence.api_observations UNION SELECT source.producer_binding_id AS id FROM snapshot.evidence.fragments WHERE source.evidence_class IN ('declared', 'statically_extracted')) SELECT DISTINCT i.sha256, i.source_uri FROM snapshot.evidence.input_artifacts i LEFT SEMI JOIN bindings b ON i.producer_binding_id = b.id WHERE i.kind NOT IN ('registry_index_entry', 'registry_version_metadata') LIMIT 8193").await?)
    }

    /// Keep paired digest/locator values relational; no parallel arrays, zip or Rust set policy.
    pub async fn same_static_inputs(&self, other: &Self) -> Result<bool, QueryError> {
        let left = self.static_input_plan().await?;
        let right = other.static_input_plan().await?;
        crate::environment_plan::same_static_inputs(&self.runtime, left, right)
            .await
            .map_err(Into::into)
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
        let plan = self.execution_frame(selection).await?;
        let output = self
            .runtime
            .execute_family(
                plan.sort(vec![col("observation_id").sort(true, false)])?,
                Some(crate::preparation::QueryFamily::Relation(
                    crate::admission::Relation::ExecutionObservations,
                )),
            )
            .await?;
        let mut observations = Vec::new();
        for batch in output.batches {
            observations.extend(projection::execution::decode(&batch)?);
        }
        Ok(observations)
    }

    /// A retained execution facet has its own stable continuation, independent of other facets.
    pub async fn execution_page(
        &self,
        selection: ExecutionSelection<'_>,
        limit: usize,
        after: Option<&str>,
    ) -> Result<NativePage<enrichment_core::evidence::execution::ExecutionObservation>, QueryError>
    {
        let plan = self.execution_frame(selection).await?;
        let output = self
            .page_rows(
                plan,
                "observation_id",
                crate::preparation::QueryFamily::Relation(
                    crate::admission::Relation::ExecutionObservations,
                ),
                limit,
                after,
            )
            .await?;
        let mut observations = Vec::new();
        for batch in output.batches {
            observations.extend(projection::execution::decode(&batch)?);
        }
        NativePage::selected(observations, output.boundary, |item| &item.observation_id)
    }

    async fn execution_frame(
        &self,
        selection: ExecutionSelection<'_>,
    ) -> Result<DataFrame, QueryError> {
        use datafusion::functions::core::expr_ext::FieldAccessor;
        let mut plan = self
            .ctx
            .table("snapshot.evidence.execution_observations")
            .await?;
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
            plan = plan.filter(
                col("subject")
                    .field("document")
                    .field("artifact_id")
                    .eq(lit(document)),
            )?;
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
        Ok(plan)
    }
    /// # Errors
    /// Invalid public paths are errors, never a broadened namespace.
    pub fn area(&self, display: &str) -> Result<PublicPath, QueryError> {
        PublicPath::parse(self.manifest().metadata.ecosystem, display)
            .map_err(|e| DataFusionError::Plan(e).into())
    }

    async fn render_symbols(&self, frame: DataFrame) -> Result<Vec<SymbolHeader>, QueryError> {
        // A small exact inspection can have multiple declaration/trait alternatives. The
        // sentinel is checked before rendering rather than silently dropping alternatives.
        let output = self
            .runtime
            .execute_family(
                frame
                    .sort(vec![col("symbol_id").sort(true, false)])?
                    .limit(0, Some(1025))?,
                Some(crate::preparation::QueryFamily::SymbolHeaders),
            )
            .await?;
        if output.rows > 1024 {
            return Err(DataFusionError::ResourcesExhausted(
                "inspection has more than 1024 candidate bindings; refine the path".into(),
            )
            .into());
        }
        Ok(projection::render::symbol_headers(&output.batches)?)
    }

    /// # Errors
    /// Exact lookup is bounded and retains same-path definition/observation alternatives.
    pub async fn symbols_at(
        &self,
        path: &str,
        definition_id: Option<&str>,
    ) -> Result<Vec<SymbolHeader>, QueryError> {
        let mut predicate = col("path").eq(lit(path));
        if let Some(id) = definition_id {
            predicate = predicate.and(col("definition_id").eq(lit(id)));
        }
        self.render_symbols(
            self.ctx
                .table("snapshot.domain.symbol_headers")
                .await?
                .filter(predicate)?,
        )
        .await
    }

    /// Native inspection selection over the exact captured snapshot.
    pub async fn inspection_bindings(
        &self,
        path: &str,
        definition_id: Option<&str>,
    ) -> Result<InspectionBindings, QueryError> {
        inspection_bindings(
            &self.runtime,
            self.ctx.table("snapshot.domain.symbol_headers").await?,
            path,
            definition_id,
        )
        .await
    }

    /// # Errors
    /// Unqualified lookup uses literal typed suffix components in the snapshot's ecosystem.
    pub async fn symbols_ending_with(
        &self,
        suffix: &str,
        definition_id: Option<&str>,
    ) -> Result<Vec<SymbolHeader>, QueryError> {
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
        // Native list indices are signed; combining UInt64 length with signed
        // offsets otherwise promotes the endpoint to Decimal128 during coercion.
        let length = datafusion::logical_expr::expr_fn::cast(
            array_length(col("components")),
            arrow::datatypes::DataType::Int64,
        );
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
                .table("snapshot.domain.symbol_headers")
                .await?
                .filter(predicate)?,
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
            .execute_family(
                self.ctx
                    .table("snapshot.evidence.symbols")
                    .await?
                    .filter(
                        col("definition_id")
                            .eq(lit(definition))
                            .and(col("path").not_eq(lit(except_path))),
                    )?
                    .select(vec![col("path")])?
                    .distinct()?
                    .sort(vec![col("path").sort(true, false)])?,
                Some(crate::preparation::QueryFamily::Paths),
            )
            .await?;
        Ok(projection::render::strings(&output.batches, "path")?)
    }

    async fn render_fragments(&self, frame: DataFrame) -> Result<Vec<TextFragment>, QueryError> {
        let output = self
            .runtime
            .execute_family(
                frame.sort(vec![col("fragment_id").sort(true, false)])?,
                Some(crate::preparation::QueryFamily::FragmentProjection { bounded: false }),
            )
            .await?;
        Ok(projection::render::fragments(&output.batches)?)
    }
    /// # Errors
    /// Resolve symbol/definition subjects through binding membership before materialization.
    pub async fn fragments(
        &self,
        selection: FragmentSelection<'_>,
    ) -> Result<Vec<TextFragment>, QueryError> {
        if selection.limit == 0 || selection.limit > 1024 || selection.kinds.is_empty() {
            return Err(DataFusionError::Plan(
                "fragment kinds and a limit in 1..=1024 are required".into(),
            )
            .into());
        }
        let mut frame = self.ctx.table("snapshot.domain.fragment_surface").await?;
        if let Some(symbol) = selection.symbol_id {
            frame = self.ctx.sql("SELECT f.* FROM snapshot.domain.fragment_surface f LEFT SEMI JOIN snapshot.domain.fragment_paths p ON f.fragment_id = p.fragment_id AND p.symbol_id = $1").await?.with_param_values(vec![datafusion::common::ScalarValue::from(symbol)])?;
        } else if let Some(path) = selection.path {
            frame = self.ctx.sql("SELECT f.* FROM snapshot.domain.fragment_surface f LEFT SEMI JOIN (SELECT DISTINCT p.fragment_id FROM snapshot.domain.fragment_paths p JOIN snapshot.evidence.symbols s ON p.symbol_id = s.symbol_id WHERE s.path = $1) m ON f.fragment_id = m.fragment_id").await?.with_param_values(vec![datafusion::common::ScalarValue::from(path)])?;
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
        let output = self
            .runtime
            .execute_family(
                frame,
                Some(crate::preparation::QueryFamily::FragmentProjection { bounded: false }),
            )
            .await?;
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
    pub async fn examples_for(&self, name: &str) -> Result<Vec<TextFragment>, QueryError> {
        let spec = enrichment_core::search::spec::SearchSpec::new(name);
        let frame = self
            .ctx
            .table("snapshot.domain.fragment_surface")
            .await?
            .filter(
                col("kind")
                    .eq(lit(FragmentKind::Example.as_str()))
                    .and(crate::scoring::fragment_eligibility(&spec)?),
            )?
            .sort(vec![col("fragment_id").sort(true, false)])?
            .limit(0, Some(3))?;
        self.render_fragments(frame).await
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
        Ok(Overview {
            definitions_by_kind: page.definitions_by_kind,
            namespaces: page.namespaces,
            truncated_namespaces: page.truncated_namespaces,
            reexports: self.manifest().counts.reexports,
            unresolved_reexports: self.manifest().counts.unresolved_reexports,
        })
    }

    /// # Errors
    /// Input references are selected from the exact admitted snapshot.
    pub async fn inputs(&self) -> Result<Vec<InputArtifact>, QueryError> {
        let output = self
            .runtime
            .execute_family(
                self.ctx.table("snapshot.evidence.input_artifacts").await?,
                Some(crate::preparation::QueryFamily::Relation(
                    crate::admission::Relation::InputArtifacts,
                )),
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
    /// Artifact selection is performed before decoding input descriptors.
    pub async fn inputs_of_kind(
        &self,
        kind: enrichment_core::evidence::ArtifactKind,
    ) -> Result<Vec<InputArtifact>, QueryError> {
        let output = self
            .runtime
            .execute_family(
                self.ctx
                    .table("snapshot.evidence.input_artifacts")
                    .await?
                    .filter(col("kind").eq(lit(kind.as_str())))?,
                Some(crate::preparation::QueryFamily::Relation(
                    crate::admission::Relation::InputArtifacts,
                )),
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
    /// Native source coordinates, bounded window and exact acquisition consensus.
    pub async fn source_read(
        &self,
        ecosystem: enrichment_core::identity::Ecosystem,
        locator: Option<&enrichment_core::evidence::relational::Locator>,
        max_lines: usize,
    ) -> Result<Option<crate::research_source::Read>, QueryError> {
        Ok(crate::research_source::select(
            &self.runtime,
            self.ctx.table("snapshot.evidence.input_artifacts").await?,
            ecosystem,
            locator,
            max_lines,
        )
        .await?)
    }
    /// # Errors
    /// Returned attempts belong to this pinned snapshot's catalog generation.
    pub async fn producer_runs(&self) -> Result<Vec<ProducerRun>, QueryError> {
        let session = self.opened.catalog.session(&self.runtime).await?;
        let output = self
            .runtime
            .execute_family(
                session
                    .table("state.records.attempts")
                    .await?
                    .filter(col("snapshot_id").eq(self.manifest().snapshot_id.literal()))?,
                Some(crate::preparation::QueryFamily::Catalog(
                    crate::control::Table::Attempts,
                )),
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
        id: &enrichment_core::identity::AttemptId,
    ) -> Result<enrichment_core::evidence::catalog::SnapshotAttempt, QueryError> {
        let session = self.opened.catalog.session(&self.runtime).await?;
        let output = self
            .runtime
            .execute_family(
                session
                    .table("state.records.attempts")
                    .await?
                    .filter(
                        col("snapshot_id")
                            .eq(self.manifest().snapshot_id.literal())
                            .and(col("attempt_id").eq(lit(id))),
                    )?
                    .limit(0, Some(2))?,
                Some(crate::preparation::QueryFamily::Catalog(
                    crate::control::Table::Attempts,
                )),
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
    /// The primary archive is selected from this snapshot's release and acquisition records.
    /// No global first-retrieval sidecar can substitute a different release's locator.
    pub async fn source_artifact(
        &self,
    ) -> Result<Option<enrichment_core::evidence::Artifact>, QueryError> {
        Ok(self
            .catalog_artifacts(
                "WITH source AS (
                SELECT r.key.artifact_digest AS artifact_digest FROM state.records.snapshots s
                JOIN state.records.contexts c ON s.context_id=c.context_id
                JOIN state.records.releases r ON c.release_id=r.release_id WHERE s.snapshot_id=$1
            ), receipts AS (
                SELECT unnest(acquisitions) AS artifact,started_at,attempt_id
                FROM state.records.attempts WHERE snapshot_id=$1
            ) SELECT artifact FROM receipts JOIN source ON artifact.sha256=artifact_digest
              ORDER BY started_at,attempt_id,artifact.source_uri LIMIT 1",
            )
            .await?
            .pop())
    }

    pub async fn artifacts(&self) -> Result<Vec<enrichment_core::evidence::Artifact>, QueryError> {
        self.catalog_artifacts(
            "WITH raw AS (
            SELECT unnest(acquisitions) AS artifact, started_at, attempt_id FROM state.records.attempts
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
            ) AS position FROM (
                SELECT job_id, delivery FROM state.records.job_publications WHERE snapshot_id = $1
                UNION ALL
                SELECT job_id, delivery FROM state.records.comparison_publications WHERE after_snapshot_id = $1
            )
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
            FROM state.records.attempts WHERE snapshot_id = $1 AND log IS NOT NULL
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
        let plan =
            session
                .sql(sql)
                .await?
                .with_param_values(datafusion::common::ParamValues::List(vec![
                    self.manifest().snapshot_id.parameter(),
                ]))?;
        let output = self
            .runtime
            .execute_family(plan, Some(crate::preparation::QueryFamily::CatalogArtifact))
            .await?;
        let mut artifacts = Vec::new();
        for batch in output.batches {
            artifacts.extend(projection::catalog::selected_artifacts(&batch)?);
        }
        Ok(artifacts)
    }
}

/// Select inspection bindings over the captured header relation.
/// This function can be exercised with an in-memory relation without opening durable state.
pub async fn inspection_bindings(
    runtime: &QueryRuntime,
    headers: DataFrame,
    path: &str,
    definition_id: Option<&str>,
) -> Result<InspectionBindings, QueryError> {
    let session = runtime.session();
    crate::native_catalog::work(&session, "inspection_headers", headers.into_view())?;
    use datafusion::common::ScalarValue;
    use enrichment_core::native_union::NativeStruct;
    let matches = session
        .sql(
            r#"
            WITH matching AS (
                SELECT *, path=$1 AS exact_path FROM inspection_headers
                WHERE (path=$1 OR (NOT contains($1,'::') AND NOT contains($1,'.')
                    AND array_element(components,-1)=$1))
                  AND ($2 IS NULL OR definition_id=$2)
            )
            SELECT * FROM matching WHERE exact_path OR NOT EXISTS
                (SELECT 1 FROM matching WHERE exact_path)
        "#,
        )
        .await?
        .with_param_values(vec![
            ScalarValue::Utf8(Some(path.into())),
            ScalarValue::Utf8(definition_id.map(str::to_owned)),
        ])?;
    crate::native_catalog::work(&session, "inspection_matches", matches.into_view())?;
    let count = session.sql("SELECT arrow_cast(count(DISTINCT definition_id),'UInt64') AS count FROM inspection_matches").await?;
    crate::native_catalog::work(
        &session,
        "inspection_definition_count",
        count.clone().into_view(),
    )?;
    let definition_count = runtime
        .records::<DefinitionCount>(count, 1)
        .await?
        .pop()
        .ok_or_else(|| DataFusionError::Internal("native inspection count returned no row".into()))?
        .count;
    let selected = session.sql("SELECT m.* FROM inspection_matches m CROSS JOIN inspection_definition_count c WHERE c.count=1 ORDER BY is_reexport,path,symbol_id LIMIT 1").await?;
    let selected = selected.select(
        SymbolHeader::fields()
            .iter()
            .map(|field| col(field.name()))
            .collect::<Vec<_>>(),
    )?;
    let selected = runtime.records::<SymbolHeader>(selected, 1).await?.pop();
    let candidates = session.sql("SELECT DISTINCT definition_id,path,kind,qualifier FROM inspection_matches CROSS JOIN inspection_definition_count c WHERE c.count>1 ORDER BY definition_id,path,kind,qualifier").await?;
    let candidates = runtime.records(candidates, 1024).await?;
    Ok(InspectionBindings {
        definition_count,
        selected,
        candidates,
    })
}
