//! Native semantic admission for an exact immutable Delta provider inventory.

use std::collections::BTreeMap;
use std::fs::File;
use std::sync::Arc;

use crate::native_catalog::{BoundCatalog, RelationContract, Tables};
use arrow_schema::SchemaRef;
use datafusion::catalog::TableProvider;
use datafusion::common::TableReference;
use datafusion::error::{DataFusionError, Result};
use datafusion::functions::core::expr_ext::FieldAccessor;
use datafusion::prelude::SessionContext;
use datafusion::prelude::{col, lit};
use enrichment_core::identity::Ecosystem;
use serde::{Deserialize, Serialize};

use crate::{projection, runtime::QueryRuntime};

/// This registry owns physical schemas and domain decoding; arbitrary tables cannot bypass it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Relation {
    Definitions,
    Symbols,
    ApiObservations,
    ExecutionObservations,
    Relationships,
    Fragments,
    ProducerRuns,
    InputArtifacts,
    Coverage,
    ReleaseMetadata,
}

impl Relation {
    pub const ALL: [Self; 10] = [
        Self::Definitions,
        Self::Symbols,
        Self::ApiObservations,
        Self::ExecutionObservations,
        Self::Relationships,
        Self::Fragments,
        Self::ProducerRuns,
        Self::InputArtifacts,
        Self::Coverage,
        Self::ReleaseMetadata,
    ];
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Definitions => "definitions",
            Self::Symbols => "symbols",
            Self::ApiObservations => "api_observations",
            Self::ExecutionObservations => "execution_observations",
            Self::Relationships => "relationships",
            Self::Fragments => "fragments",
            Self::ProducerRuns => "producer_runs",
            Self::InputArtifacts => "input_artifacts",
            Self::Coverage => "coverage",
            Self::ReleaseMetadata => "release_metadata",
        }
    }

    /// The sole physical schema, including nested semantic metadata.
    ///
    /// # Errors
    /// Encoding an empty typed relation fails only on an internal schema defect.
    pub fn schema(self) -> Result<SchemaRef> {
        Ok(match self {
            Self::Definitions => projection::definitions(&[])?,
            Self::Symbols => projection::bindings(&[])?,
            Self::ApiObservations => projection::observations(&[])?,
            Self::ExecutionObservations => projection::execution::encode(&[])?,
            Self::Relationships => projection::relationships(&[])?,
            Self::Fragments => projection::fragments(&[])?,
            Self::ProducerRuns => projection::producer_runs(&[])?,
            Self::InputArtifacts => projection::input_artifacts(&[])?,
            Self::Coverage => projection::coverage(&[])?,
            Self::ReleaseMetadata => projection::metadata::encode(&[])?,
        }
        .schema())
    }

    /// Qualified admitted source reference; candidate validation has a distinct namespace.
    #[must_use]
    pub fn reference(self) -> TableReference {
        TableReference::full("snapshot", "evidence", self.name())
    }
    pub(crate) fn key(self) -> &'static str {
        match self {
            Self::Definitions => "definition_id",
            Self::Symbols => "symbol_id",
            Self::ApiObservations | Self::ExecutionObservations => "observation_id",
            Self::Relationships => "relationship_id",
            Self::Fragments => "fragment_id",
            Self::ProducerRuns => "attempt_id",
            Self::InputArtifacts => "input_id",
            Self::Coverage => "coverage_id",
            Self::ReleaseMetadata => "metadata_id",
        }
    }
}

/// Semantic scope participates in admission identity; a validated table cannot be rebound to
/// an unrelated release/environment merely because its physical file is unchanged.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceScope {
    pub ecosystem: Ecosystem,
    pub symbol_package: String,
    pub release_id: String,
    pub environment_id: String,
}

/// Native evidence admission limits; producer decoding owns its separate physical bounds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdmissionLimits {
    pub record_bytes: usize,
    pub table_rows: usize,
    pub deadline: std::time::Duration,
}

impl Default for AdmissionLimits {
    fn default() -> Self {
        Self {
            record_bytes: 1024 * 1024,
            table_rows: 1_000_000,
            deadline: std::time::Duration::from_secs(30),
        }
    }
}

/// An admitted relation set pins the exact native provider inventory for its lifetime.
pub struct AdmittedRelations {
    providers: BTreeMap<Relation, Arc<dyn TableProvider>>,
    views: tokio::sync::OnceCell<BTreeMap<String, Arc<dyn TableProvider>>>,
}

impl AdmittedRelations {
    /// Bind admitted sources atomically through the immutable catalog.
    /// # Errors
    /// Contract construction errors remain explicit; base providers never acquire request leases.
    pub fn session(
        &self,
        runtime: &QueryRuntime,
        lease: Option<Arc<File>>,
    ) -> Result<SessionContext> {
        let catalog = evidence_catalog(
            runtime,
            &self.providers,
            lease.as_ref(),
            crate::native_catalog::BindingKind::AdmittedEvidence,
        )?;
        runtime.bound_session(BTreeMap::from([(
            "snapshot".into(),
            Arc::new(catalog) as Arc<dyn datafusion::catalog::CatalogProvider>,
        )]))
    }

    /// Bind transparent native domain views and bases through the same immutable inventory.
    /// # Errors
    /// Missing or changed sources and view construction failures remain explicit.
    pub async fn research_session(
        &self,
        runtime: &QueryRuntime,
        lease: Option<Arc<File>>,
    ) -> Result<SessionContext> {
        self.research_session_with(runtime, lease, &Tables::new())
            .await
    }

    pub(crate) async fn research_session_with(
        &self,
        runtime: &QueryRuntime,
        lease: Option<Arc<File>>,
        materialized: &Tables,
    ) -> Result<SessionContext> {
        let bases = evidence_catalog(
            runtime,
            &self.providers,
            lease.as_ref(),
            crate::native_catalog::BindingKind::AdmittedEvidence,
        )?;
        let views = self
            .views
            .get_or_try_init(|| async {
                crate::views::build(
                    runtime,
                    evidence_catalog(
                        runtime,
                        &self.providers,
                        None,
                        crate::native_catalog::BindingKind::AdmittedEvidence,
                    )?,
                )
                .await
            })
            .await?;
        let mut bound = Tables::new();
        let staging = runtime.session();
        for (name, view) in views {
            let view = materialized.get(name).unwrap_or(view);
            let provider = match &lease {
                Some(lease) => crate::leases::leased_view(view, &staging, lease)?,
                None => Arc::clone(view),
            };
            bound.insert(name.clone(), provider);
        }
        runtime.bound_session(BTreeMap::from([(
            "snapshot".into(),
            Arc::new(bases.with_schema(crate::native_catalog::BindingKind::AdmittedDomain, bound))
                as Arc<dyn datafusion::catalog::CatalogProvider>,
        )]))
    }
}

fn evidence_catalog(
    runtime: &QueryRuntime,
    providers: &BTreeMap<Relation, Arc<dyn TableProvider>>,
    lease: Option<&Arc<File>>,
    kind: crate::native_catalog::BindingKind,
) -> Result<BoundCatalog> {
    let mut tables = Tables::new();
    for (relation, provider) in providers {
        let mut entries = vec![(
            relation.name(),
            Arc::clone(provider) as Arc<dyn TableProvider>,
        )];
        if *relation == Relation::ApiObservations {
            let schema = crate::projection::inspection_schema(&provider.schema())?;
            let projected = runtime.session().read_table(Arc::clone(provider))?.select(
                schema
                    .fields()
                    .iter()
                    .map(|field| col(field.name()))
                    .collect::<Vec<_>>(),
            )?;
            entries.push(("inspection_observations", projected.into_view()));
        }
        for (name, input) in entries {
            let input = match lease {
                Some(lease) => {
                    Arc::new(crate::leases::LeasedProvider::new(input, Arc::clone(lease)))
                        as Arc<dyn TableProvider>
                }
                None => input,
            };
            tables.insert(name.to_owned(), input);
        }
    }
    Ok(BoundCatalog::default().with_schema(kind, tables))
}

/// Enforces declared native contracts before exposing optimizer constraints.
pub struct NativeAdmission {
    runtime: QueryRuntime,
    limits: AdmissionLimits,
}

pub(crate) const CONDITIONAL_RULES: &[crate::native_catalog::SqlRule] = &[
    crate::native_catalog::SqlRule {
        id: "metadata worker artifact outside producer input closure",
        relation: "release_metadata",
        sql: "SELECT m.metadata_id FROM candidate.evidence.release_metadata m LEFT ANTI JOIN candidate.evidence.input_artifacts i ON m.python_distribution.worker_artifact_id = i.artifact_id AND m.source.producer_binding_id = i.producer_binding_id WHERE m.kind = 'python_distribution' AND m.python_distribution.worker_artifact_id IS NOT NULL LIMIT 1",
    },
    crate::native_catalog::SqlRule {
        id: "execution lacks actual log or correct producer policy",
        relation: "execution_observations",
        sql: "SELECT o.observation_id FROM candidate.evidence.execution_observations o JOIN candidate.evidence.producer_runs p ON o.source.producer_binding_id = p.producer_binding_id WHERE p.log IS NULL OR ((o.payload.kind = 'runtime_object' OR o.payload.usage_probe.mode = 'runtime') AND p.profile != 'runtime') OR ((o.payload.kind = 'semantic_query' OR o.payload.usage_probe.mode IN ('compile','typecheck')) AND p.profile != 'build') LIMIT 1",
    },
    crate::native_catalog::SqlRule {
        id: "execution document outside its producer inputs",
        relation: "execution_observations",
        sql: "SELECT o.observation_id FROM candidate.evidence.execution_observations o LEFT ANTI JOIN candidate.evidence.input_artifacts i ON o.subject.artifact_id = i.artifact_id AND o.source.producer_binding_id = i.producer_binding_id WHERE o.subject.kind = 'document' LIMIT 1",
    },
    crate::native_catalog::SqlRule {
        id: "dangling semantic anchor",
        relation: "execution_observations",
        sql: "SELECT o.observation_id FROM candidate.evidence.execution_observations o LEFT ANTI JOIN candidate.evidence.symbols s ON o.payload.semantic_query.anchor_symbol_id = s.symbol_id WHERE o.payload.kind = 'semantic_query' AND o.payload.semantic_query.anchor_symbol_id IS NOT NULL LIMIT 1",
    },
    crate::native_catalog::SqlRule {
        id: "semantic location outside artifact closure",
        relation: "execution_observations",
        sql: "SELECT t.artifact_id FROM (SELECT unnest(payload.semantic_query.locations) AS t FROM candidate.evidence.execution_observations WHERE payload.kind = 'semantic_query') q LEFT ANTI JOIN candidate.evidence.input_artifacts i ON q.t.artifact_id = i.artifact_id WHERE q.t.kind = 'artifact' LIMIT 1",
    },
];

impl NativeAdmission {
    /// # Errors
    /// Zero resource bounds are configuration errors.
    pub fn new(runtime: QueryRuntime, limits: AdmissionLimits) -> Result<Self> {
        if limits.record_bytes == 0 || limits.table_rows == 0 || limits.deadline.is_zero() {
            return Err(invalid("admission limits must be positive"));
        }
        Ok(Self { runtime, limits })
    }

    /// Admit the exact native inventory before declaring uniqueness and references.
    pub async fn admit_native(
        &self,
        scope: &EvidenceScope,
        providers: BTreeMap<Relation, Arc<dyn TableProvider>>,
    ) -> Result<Arc<AdmittedRelations>> {
        tokio::time::timeout(
            self.limits.deadline,
            self.validate_native(scope, &providers),
        )
        .await
        .map_err(|_| {
            DataFusionError::ResourcesExhausted("native admission deadline exceeded".into())
        })??;
        Ok(Arc::new(AdmittedRelations {
            providers: with_constraints(providers)?,
            views: tokio::sync::OnceCell::new(),
        }))
    }

    async fn validate_native(
        &self,
        scope: &EvidenceScope,
        providers: &BTreeMap<Relation, Arc<dyn TableProvider>>,
    ) -> Result<()> {
        if providers.len() != Relation::ALL.len() {
            return Err(invalid("native evidence inventory is incomplete"));
        }
        for relation in Relation::ALL {
            let provider = providers
                .get(&relation)
                .ok_or_else(|| invalid("missing native relation"))?;
            if provider.schema() != relation.schema()? {
                return Err(invalid(format!(
                    "native semantic contract differs: {}",
                    relation.name()
                )));
            }
        }
        let candidate = evidence_catalog(
            &self.runtime,
            providers,
            None,
            crate::native_catalog::BindingKind::CandidateEvidence,
        )?;
        let session = self.runtime.bound_session(BTreeMap::from([(
            "candidate".into(),
            Arc::new(candidate) as Arc<dyn datafusion::catalog::CatalogProvider>,
        )]))?;
        for relation in providers.keys() {
            let reference = TableReference::full("candidate", "evidence", relation.name());
            let input = session.table(reference.clone()).await?;
            let count = input
                .clone()
                .limit(0, Some(self.limits.table_rows.saturating_add(1)))?
                .aggregate(
                    vec![],
                    vec![datafusion::functions_aggregate::expr_fn::count(lit(1)).alias("rows")],
                )?
                .filter(col("rows").gt(lit(self.limits.table_rows as u64)))?
                .select(vec![lit(relation.name()).alias("witness_id")])?;
            self.runtime
                .require_empty(count, "native relation row budget", relation.name())
                .await?;
            let schema = relation.schema()?;
            let bytes = enrichment_core::native_identity::canonical_bytes(
                "admission-row/1",
                schema.fields().clone(),
            )
            .call(schema.fields().iter().map(|f| col(f.name())).collect());
            let oversized = input
                .filter(
                    enrichment_core::native_identity::byte_length(bytes)
                        .gt(lit(self.limits.record_bytes as u64)),
                )?
                .select(vec![col(relation.key())])?
                .limit(0, Some(1))?;
            self.runtime
                .require_empty(oversized, "native record byte budget", relation.name())
                .await?;
            for (rule, violations) in enrichment_core::evidence::arrow_model::checks::violations(
                session.table(reference.clone()).await?,
                relation.schema()?.as_ref(),
                relation.key(),
            )? {
                self.runtime
                    .require_empty(violations, &rule, relation.name())
                    .await?;
            }
            let duplicates = relation
                .duplicate_keys(
                    &session,
                    TableReference::full("candidate", "evidence", relation.name()),
                )
                .await?;
            if self.runtime.execute(duplicates).await?.rows != 0 {
                return Err(invalid(format!(
                    "duplicate relation key: {}.unique",
                    relation.name()
                )));
            }
        }
        for relation in Relation::ALL {
            for rule in relation.references() {
                self.runtime
                    .require_empty(
                        rule.violations(
                            &session,
                            TableReference::full("candidate", "evidence", relation.name()),
                            relation.key(),
                        )
                        .await?,
                        rule.id,
                        "admission",
                    )
                    .await?;
            }
        }
        self.validate_identities(&session, scope).await?;
        for relation in [Relation::ApiObservations, Relation::ExecutionObservations] {
            let outside_environment = session
                .table(TableReference::full(
                    "candidate",
                    "evidence",
                    relation.name(),
                ))
                .await?
                .filter(col("environment_id").not_eq(lit(&scope.environment_id)))?
                .select(vec![col(relation.key())])?
                .limit(0, Some(1))?;
            self.runtime
                .require_empty(
                    outside_environment,
                    "observation environment disagrees with snapshot",
                    "relation_admission",
                )
                .await?;
        }
        let metadata = session
            .table("candidate.evidence.release_metadata")
            .await?
            .filter(
                col("release_id")
                    .not_eq(lit(&scope.release_id))
                    .or(col("kind").not_eq(lit(match scope.ecosystem {
                        Ecosystem::Rust => "rust_docs",
                        Ecosystem::Python => "python_distribution",
                    }))),
            )?
            .select(vec![col("metadata_id")])?
            .limit(0, Some(1))?;
        self.runtime
            .require_empty(
                metadata,
                "release metadata disagrees with snapshot scope",
                "relation_admission",
            )
            .await?;

        for relation in [
            Relation::Relationships,
            Relation::Fragments,
            Relation::Coverage,
            Relation::ExecutionObservations,
        ] {
            let outside_release = session
                .table(TableReference::full(
                    "candidate",
                    "evidence",
                    relation.name(),
                ))
                .await?
                .filter(
                    col("subject").field("kind").eq(lit("library")).and(
                        col("subject")
                            .field("release_id")
                            .not_eq(lit(&scope.release_id)),
                    ),
                )?
                .select(vec![col(relation.key())])?
                .limit(0, Some(1))?;
            self.runtime
                .require_empty(
                    outside_release,
                    "library subject disagrees with snapshot release",
                    "relation_admission",
                )
                .await?;
        }
        for (tag, table, key) in [
            ("symbol", "symbols", "symbol_id"),
            ("definition", "definitions", "definition_id"),
        ] {
            let sql = format!(
                "SELECT o.relationship_id FROM candidate.evidence.relationships o LEFT ANTI JOIN candidate.evidence.{table} t ON o.target.{key} = t.{key} WHERE o.target.kind = '{tag}' LIMIT 1"
            );
            self.require_empty(&session, &sql, "dangling local relationship target")
                .await?;
        }
        for relation in [
            Relation::ApiObservations,
            Relation::ExecutionObservations,
            Relation::Relationships,
            Relation::Fragments,
            Relation::ReleaseMetadata,
        ] {
            let sql = format!(
                "SELECT o.{} FROM candidate.evidence.{} o LEFT ANTI JOIN candidate.evidence.input_artifacts i ON o.source.producer_binding_id = i.producer_binding_id AND o.source.artifact_id = i.artifact_id AND (o.source.source_uri IS NULL OR o.source.source_uri = i.source_uri) LIMIT 1",
                relation.key(),
                relation.name()
            );
            self.require_empty(
                &session,
                &sql,
                "fact provenance outside producer input closure",
            )
            .await?;
        }
        for relation in [
            Relation::Fragments,
            Relation::Coverage,
            Relation::ExecutionObservations,
        ] {
            let sql = format!(
                "SELECT o.{} FROM candidate.evidence.{} o LEFT ANTI JOIN candidate.evidence.input_artifacts i ON o.subject.artifact_id = i.artifact_id WHERE o.subject.kind IN ('document', 'example') LIMIT 1",
                relation.key(),
                relation.name()
            );
            self.require_empty(
                &session,
                &sql,
                "document/example subject outside artifact closure",
            )
            .await?;
        }
        let wrong_operation = match scope.ecosystem {
            Ecosystem::Rust => {
                "SELECT observation_id FROM candidate.evidence.execution_observations WHERE payload.kind = 'runtime_object' OR payload.usage_probe.mode = 'typecheck' OR (payload.kind = 'semantic_query' AND source.evidence_class != 'compiler_derived') LIMIT 1"
            }
            Ecosystem::Python => {
                "SELECT observation_id FROM candidate.evidence.execution_observations WHERE payload.usage_probe.mode = 'compile' OR (payload.kind = 'semantic_query' AND source.evidence_class != 'typechecker_observed') LIMIT 1"
            }
        };
        self.require_empty(
            &session,
            wrong_operation,
            "execution operation disagrees with ecosystem",
        )
        .await?;

        for rule in CONDITIONAL_RULES {
            self.runtime
                .require_empty(
                    rule.violations(&session).await?,
                    rule.id,
                    "relation_admission",
                )
                .await?;
        }
        Ok(())
    }

    async fn validate_identities(
        &self,
        session: &SessionContext,
        scope: &EvidenceScope,
    ) -> Result<()> {
        use datafusion::logical_expr::{BinaryExpr, Expr, Operator};
        use enrichment_core::native_key::Key;
        let differs = |left: Expr, right: Expr| {
            Expr::BinaryExpr(BinaryExpr::new(
                Box::new(left),
                Operator::IsDistinctFrom,
                Box::new(right),
            ))
        };
        for (relation, key) in [
            (Relation::ApiObservations, Key::ApiObservation),
            (Relation::ExecutionObservations, Key::ExecutionObservation),
            (Relation::ReleaseMetadata, Key::ReleaseMetadata),
            (Relation::Fragments, Key::TextFragment),
            (Relation::Relationships, Key::Relationship),
            (Relation::InputArtifacts, Key::InputArtifact),
            (Relation::Coverage, Key::Coverage),
        ] {
            let violations = session
                .table(TableReference::full(
                    "candidate",
                    "evidence",
                    relation.name(),
                ))
                .await?
                .filter(differs(col(relation.key()), key.expression()))?
                .select(vec![col(relation.key())])?;
            self.runtime
                .require_empty(violations, "native evidence identity", relation.name())
                .await?;
        }
        let producers = session
            .table("candidate.evidence.producer_runs")
            .await?
            .filter(differs(
                col("producer_binding_id"),
                Key::ProducerBinding.expression(),
            ))?
            .select(vec![col("attempt_id")])?;
        self.runtime
            .require_empty(producers, "native producer identity", "producer_runs")
            .await?;
        let definition_key = Key::Definition.bind(vec![
            col("defined_in_package"),
            col("definition_path"),
            col("kind"),
            col("qualifier"),
        ])?;
        let definitions = session
            .table("candidate.evidence.definitions")
            .await?
            .filter(
                differs(col("definition_id"), definition_key)
                    .or(col("defined_in_package").eq(lit("")))
                    .or(col("definition_path").eq(lit(""))),
            )?
            .select(vec![col("definition_id")])?;
        self.runtime
            .require_empty(definitions, "native definition identity", "definitions")
            .await?;
        let ecosystem = match scope.ecosystem {
            Ecosystem::Rust => "rust",
            Ecosystem::Python => "python",
        };
        let separator = match scope.ecosystem {
            Ecosystem::Rust => "::",
            Ecosystem::Python => ".",
        };
        let joined = session.sql(&format!("SELECT s.*, d.kind AS definition_kind, d.qualifier AS definition_qualifier, \
            array_to_string(s.components, '{separator}') AS expected_path, array_element(s.components, -1) AS expected_name, \
            array_length(s.components) AS component_count, array_slice(s.components, 1, CAST(array_length(s.components) AS BIGINT) - 1) AS parent_components \
            FROM candidate.evidence.symbols s JOIN candidate.evidence.definitions d ON s.definition_id = d.definition_id")).await?;
        let symbol = Key::PublicBinding.bind(vec![
            lit(&scope.symbol_package),
            col("ecosystem"),
            col("components"),
            col("definition_kind"),
            col("qualifier"),
        ])?;
        let path = Key::PublicPath.bind(vec![col("ecosystem"), col("components")])?;
        let parent = datafusion::logical_expr::expr_fn::when(
            col("component_count").gt(lit(1i64)),
            Key::PublicPath.bind(vec![col("ecosystem"), col("parent_components")])?,
        )
        .otherwise(lit(datafusion::common::ScalarValue::Utf8(None)))?;
        let violations = joined
            .filter(
                differs(col("symbol_id"), symbol)
                    .or(differs(col("path_id"), path))
                    .or(differs(col("parent_path_id"), parent))
                    .or(differs(col("qualifier"), col("definition_qualifier")))
                    .or(differs(col("name"), col("expected_name")))
                    .or(differs(col("path"), col("expected_path")))
                    .or(differs(col("ecosystem"), lit(ecosystem)))
                    .or(col("component_count").lt(lit(1i64)))
                    .or(col("component_count").gt(lit(
                        enrichment_core::evidence::path::PublicPath::MAX_DEPTH as u64,
                    ))),
            )?
            .select(vec![col("symbol_id")])?;
        self.runtime
            .require_empty(violations, "native public binding identity", "symbols")
            .await?;
        // UNNEST exposes component-level value checks without reconstructing paths in Rust.
        let components = session
            .sql(
                "SELECT symbol_id, unnest(components) AS component FROM candidate.evidence.symbols",
            )
            .await?;
        let bad_components = components
            .filter(
                col("component")
                    .is_null()
                    .or(col("component").eq(lit("")))
                    .or(datafusion::functions::regex::expr_fn::regexp_like(
                        col("component"),
                        lit(r"[\p{Cc}]"),
                        None,
                    )),
            )?
            .select(vec![col("symbol_id")])?;
        self.runtime
            .require_empty(bad_components, "native path component", "symbols")
            .await?;
        let bytes = session.sql("WITH components AS (SELECT symbol_id, unnest(components) AS component FROM candidate.evidence.symbols) \
            SELECT symbol_id FROM components GROUP BY symbol_id HAVING sum(octet_length(component)) > 4096").await?;
        self.runtime
            .require_empty(bytes, "native path byte bound", "symbols")
            .await?;
        Ok(())
    }

    async fn require_empty(
        &self,
        session: &SessionContext,
        sql: &str,
        violation: &str,
    ) -> Result<()> {
        self.runtime
            .require_empty(session.sql(sql).await?, violation, "relation_admission")
            .await
    }
}

fn with_constraints(
    providers: BTreeMap<Relation, Arc<dyn TableProvider>>,
) -> Result<BTreeMap<Relation, Arc<dyn TableProvider>>> {
    providers
        .into_iter()
        .map(|(relation, provider)| {
            Ok((
                relation,
                Arc::new(crate::admitted_provider::AdmittedProvider::new(
                    provider,
                    relation.validated_constraints()?,
                )) as Arc<dyn TableProvider>,
            ))
        })
        .collect()
}

fn invalid(message: impl Into<String>) -> DataFusionError {
    DataFusionError::Execution(message.into())
}
