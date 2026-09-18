//! Immutable, completely constructed native namespaces and their declaration authority.
//!
//! Durable publication and admission own trust. These providers project already captured
//! inputs; lookup never refreshes a generation, acquires evidence, or changes visibility.

use enrichment_core::{
    native_union::{NativeStruct, Rule},
    telemetry::InventorySummary,
};
use std::{collections::BTreeMap, sync::Arc};

use arrow_schema::SchemaRef;
use async_trait::async_trait;
use datafusion::{
    catalog::{CatalogProvider, SchemaProvider, TableProvider},
    common::{Constraint, Constraints, TableReference},
    dataframe::DataFrame,
    error::{DataFusionError, Result},
    functions_aggregate::expr_fn::count,
    logical_expr::TableType,
    prelude::{SessionContext, col, lit},
};

pub(crate) type Tables = BTreeMap<String, Arc<dyn TableProvider>>;

/// Finite semantic declarations retain their distinct evidence/catalog identities.
pub(crate) trait RelationContract: Copy {
    fn name(self) -> &'static str;
    fn schema(self) -> Result<SchemaRef>;
    fn key(self) -> &'static str;

    fn validated_constraints(self) -> Result<Constraints> {
        let schema = self.schema()?;
        let keys: Vec<_> = schema
            .fields()
            .iter()
            .enumerate()
            .filter(|(_, field)| field.name() == self.key())
            .collect();
        if keys.len() != 1 || !enrichment_core::native_schema::required(keys[0].1) {
            return Err(DataFusionError::Internal(format!(
                "{} requires one non-null key field {}",
                self.name(),
                self.key()
            )));
        }
        Ok(Constraints::new_unverified(vec![Constraint::PrimaryKey(
            vec![keys[0].0],
        )]))
    }

    async fn duplicate_keys(
        self,
        session: &SessionContext,
        table: TableReference,
    ) -> Result<DataFrame> {
        session
            .table(table)
            .await?
            .aggregate(
                vec![col(self.key())],
                vec![count(lit(1)).alias("key_count")],
            )?
            .filter(col("key_count").gt(lit(1i64)))?
            .select(vec![col(self.key())])?
            .limit(0, Some(1))
    }
}

impl RelationContract for crate::admission::Relation {
    fn name(self) -> &'static str {
        self.name()
    }
    fn schema(self) -> Result<SchemaRef> {
        self.schema()
    }
    fn key(self) -> &'static str {
        self.key()
    }
}

impl RelationContract for crate::control::Table {
    fn name(self) -> &'static str {
        self.name()
    }
    fn schema(self) -> Result<SchemaRef> {
        self.schema()
    }
    fn key(self) -> &'static str {
        self.key()
    }
}

/// Binding kind is captured at construction, never inferred from an SQL name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BindingKind {
    AdmittedEvidence,
    AdmittedDomain,
    CandidateEvidence,
    FoldedRecords,
    ValidatedHistory,
    Metadata,
    Declarations,
}
impl BindingKind {
    fn schema(self) -> &'static str {
        match self {
            Self::AdmittedEvidence | Self::CandidateEvidence => "evidence",
            Self::AdmittedDomain => "domain",
            Self::FoldedRecords => "records",
            Self::ValidatedHistory => "history",
            Self::Metadata => "metadata",
            Self::Declarations => "declarations",
        }
    }
    fn trust(self) -> &'static str {
        match self {
            Self::AdmittedEvidence => "admitted_evidence",
            Self::AdmittedDomain => "admitted_domain",
            Self::CandidateEvidence => "candidate",
            Self::FoldedRecords => "folded_records",
            Self::ValidatedHistory => "validated_history",
            Self::Metadata => "derived_metadata",
            Self::Declarations => "generated_declaration",
        }
    }
}

/// Structural namespace. Trust and source ownership reside in its captured table providers.
#[derive(Debug, Clone)]
pub(crate) struct BoundSchema {
    tables: Tables,
    kind: BindingKind,
}

/// Generated, immutable policy inputs are built once per runtime and shared by every session.
/// The schema provider refuses registration/removal through its default mutation contracts.
pub(crate) fn declarations(
    retention: &enrichment_core::operation::retention::RetentionPolicy,
    pool: &Arc<dyn datafusion::execution::memory_pool::MemoryPool>,
) -> Result<Tables> {
    use enrichment_core::{
        evidence::execution::{ExecutionDefinition, ExecutionKind},
        native_union::NativeStruct,
        wire::research::{AspectDefinition, DiscoveryDefinition, DiscoveryKind, InspectionAspect},
    };
    [
        (
            "retention_policy",
            enrichment_core::operation::retention::RetentionPolicy::batch(std::slice::from_ref(
                retention,
            ))?,
        ),
        (
            "execution_payloads",
            ExecutionDefinition::batch(&ExecutionKind::definitions())?,
        ),
        (
            "inspection_aspects",
            AspectDefinition::batch(&InspectionAspect::definitions())?,
        ),
        (
            "discovery_facets",
            DiscoveryDefinition::batch(&DiscoveryKind::definitions())?,
        ),
        (
            "search_families",
            enrichment_core::search::FamilyDefinition::batch(&enrichment_core::search::families())?,
        ),
        (
            "comparison_scopes",
            enrichment_core::compare::ScopeDefinition::batch(
                &enrichment_core::compare::Scope::definitions(),
            )?,
        ),
        (
            "operations",
            enrichment_core::operation::Definition::batch(
                &enrichment_core::request::operation_definitions(),
            )?,
        ),
        (
            "resources",
            enrichment_core::request::resources::ResourceDefinition::batch(
                &enrichment_core::request::resources::definitions(),
            )?,
        ),
    ]
    .into_iter()
    .map(|(name, batch)| {
        Ok((
            name.into(),
            Arc::new(crate::admitted_provider::AdmittedProvider::from_batch(
                batch, pool,
            )?) as Arc<dyn TableProvider>,
        ))
    })
    .collect()
}

impl BoundSchema {
    pub(crate) fn new(kind: BindingKind, tables: Tables) -> Self {
        Self { tables, kind }
    }
}

#[async_trait]
impl SchemaProvider for BoundSchema {
    fn table_names(&self) -> Vec<String> {
        self.tables.keys().cloned().collect()
    }
    fn table_exist(&self, name: &str) -> bool {
        self.tables.contains_key(name)
    }
    async fn table_type(&self, name: &str) -> Result<Option<TableType>> {
        Ok(self.tables.get(name).map(|table| table.table_type()))
    }
    async fn table(&self, name: &str) -> Result<Option<Arc<dyn TableProvider>>> {
        Ok(self.tables.get(name).cloned())
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct BoundCatalog {
    schemas: BTreeMap<String, Arc<dyn SchemaProvider>>,
    inventory: Option<InventorySummary>,
    invalid: bool,
}

impl BoundCatalog {
    pub(crate) fn with_schema(mut self, kind: BindingKind, tables: Tables) -> Self {
        self.invalid |= self
            .schemas
            .insert(
                kind.schema().to_owned(),
                Arc::new(BoundSchema::new(kind, tables)),
            )
            .is_some();
        self
    }
    pub(crate) fn with_metadata(mut self, tables: Tables, summary: InventorySummary) -> Self {
        self.schemas.insert(
            "metadata".into(),
            Arc::new(BoundSchema::new(BindingKind::Metadata, tables)),
        );
        self.inventory = Some(summary);
        self
    }
    pub(crate) fn with_work(mut self) -> Self {
        self.schemas.insert(
            "work".into(),
            Arc::new(datafusion::catalog::MemorySchemaProvider::new()),
        );
        self
    }
}

enrichment_core::native_struct! {
/// Query diagnostics and native metadata derive from these same immutable bindings.
pub(crate) struct RelationInfo {
    catalog_name: String => Rule::NonEmpty,
    schema_name: String => Rule::NonEmpty,
    table_name: String => Rule::NonEmpty,
    table_type: String => Rule::NonEmpty,
    binding_kind: String => Rule::NonEmpty,
    declared_key: Option<String> => Rule::Text,
    validated_key: Option<String> => Rule::Text,
    verified_rows: Option<u64> => Rule::Text,
} }
enrichment_core::native_vocabulary! { pub(crate) enum RuleKind {
    Unique = "unique", Reference = "reference", NullableReference = "nullable_reference",
    ScopedReference = "scoped_reference", Conditional = "conditional",
} }
enrichment_core::native_vocabulary! { pub(crate) enum ReferenceNull {
    Exact = "exact", Unspecified = "unspecified",
} }
enrichment_core::native_struct! { pub(crate) struct ReferenceScope {
    source: Vec<String> => Rule::SequenceBounds { min: 1, max: 64 },
    target: Vec<String> => Rule::SequenceBounds { min: 1, max: 64 },
    null: ReferenceNull => Rule::Text,
} }
enrichment_core::native_struct! { pub(crate) struct RuleInfo {
    relation: String => Rule::NonEmpty,
    rule_id: String => Rule::NonEmpty,
    kind: RuleKind => Rule::Text,
    source_field: Vec<String> => Rule::SequenceBounds { min: 0, max: 64 },
    target_relation: Option<String> => Rule::Text,
    target_field: Option<Vec<String>> => Rule::SequenceBounds { min: 1, max: 64 },
    target_domain: Option<String> => Rule::Text,
    scope: Vec<ReferenceScope> => Rule::SequenceBounds { min: 0, max: 16 },
} }
enrichment_core::native_struct! { pub(crate) struct InventoryInfo {
    relations: u64 => Rule::Text,
    nested_fields: u64 => Rule::Text,
    truncated: bool => Rule::Text,
} }

fn declaration(kind: BindingKind, name: &str) -> Option<&'static str> {
    if matches!(
        kind,
        BindingKind::AdmittedEvidence | BindingKind::CandidateEvidence
    ) {
        crate::admission::Relation::ALL
            .into_iter()
            .find(|relation| relation.name() == name)
            .map(|relation| relation.key())
    } else if kind == BindingKind::FoldedRecords {
        crate::control::Table::ALL
            .into_iter()
            .find(|table| table.name() == name)
            .map(|table| table.key())
    } else {
        None
    }
}

enrichment_core::native_struct! { pub(crate) struct FieldInfo {
    relation: String => Rule::NonEmpty,
    field_path: Vec<String> => Rule::SequenceBounds { min: 1, max: 64 },
    data_type: String => Rule::NonEmpty,
    nullable: bool => Rule::Text,
    required_by_contract: bool => Rule::Text,
    semantic_role: Option<String> => Rule::Text,
    /// Exact declaration metadata, queryable without another field-policy registry.
    metadata: BTreeMap<String, String> => Rule::Map,
} }

impl RuleInfo {
    fn estimated_size(&self) -> usize {
        let path_size = |path: &[String]| path.iter().map(|part| part.len() + 24).sum::<usize>();
        self.relation.len()
            + self.rule_id.len()
            + path_size(&self.source_field)
            + self.target_relation.as_ref().map_or(0, String::len)
            + self.target_field.as_ref().map_or(0, |path| path_size(path))
            + self.target_domain.as_ref().map_or(0, String::len)
            + self
                .scope
                .iter()
                .map(|scope| path_size(&scope.source) + path_size(&scope.target) + 64)
                .sum::<usize>()
            + 256
    }
}

fn reference_info(
    catalog: &str,
    schema: &str,
    relation: &str,
    path: &[String],
    field: &arrow_schema::Field,
) -> Result<Option<RuleInfo>> {
    use enrichment_core::native_union::{ReferenceTarget, ScopeNull};
    let Some(encoded) = field.metadata().get("enrichment.rule") else {
        return Ok(None);
    };
    let rule: Rule = serde_json::from_str(encoded).map_err(|error| {
        datafusion::common::plan_datafusion_err!("invalid catalog field rule: {error}")
    })?;
    let Some((target, scope)) = rule.reference() else {
        return Ok(None);
    };
    let (table, target_field, target_domain) = match target {
        ReferenceTarget::Evidence(domain) => {
            let target = (schema == "evidence")
                .then(|| domain.evidence_target())
                .flatten();
            (
                target.map(|(table, _)| table.to_owned()),
                target.map(|(_, field)| vec![field.into()]),
                Some(domain.prefix().into()),
            )
        }
        ReferenceTarget::Relation { table, field } => (Some(table), Some(field), None),
    };
    Ok(Some(RuleInfo {
        relation: relation.into(),
        rule_id: format!("{relation}:{path:?}"),
        kind: if !scope.is_empty() {
            RuleKind::ScopedReference
        } else if enrichment_core::native_schema::required(field) {
            RuleKind::Reference
        } else {
            RuleKind::NullableReference
        },
        source_field: path.into(),
        target_relation: table.map(|table| {
            TableReference::full(catalog.to_owned(), schema.to_owned(), table).to_string()
        }),
        target_field,
        target_domain,
        scope: scope
            .into_iter()
            .map(|scope| ReferenceScope {
                source: scope.source,
                target: scope.target,
                null: match scope.null {
                    ScopeNull::Exact => ReferenceNull::Exact,
                    ScopeNull::Unspecified => ReferenceNull::Unspecified,
                },
            })
            .collect(),
    }))
}

fn validate_inventory(catalogs: &BTreeMap<String, Arc<dyn CatalogProvider>>) -> Result<()> {
    for (name, catalog) in catalogs {
        let catalog = catalog
            .downcast_ref::<BoundCatalog>()
            .ok_or_else(|| DataFusionError::Internal("source catalog is not immutable".into()))?;
        if catalog.invalid {
            return Err(DataFusionError::Plan("duplicate bound schema".into()));
        }
        for schema in catalog.schemas.values() {
            let schema = schema.downcast_ref::<BoundSchema>().ok_or_else(|| {
                DataFusionError::Internal("source schema is not immutable".into())
            })?;
            let allowed = match name.as_str() {
                "snapshot" | "before" | "after" => matches!(
                    schema.kind,
                    BindingKind::AdmittedEvidence | BindingKind::AdmittedDomain
                ),
                "candidate" => matches!(schema.kind, BindingKind::CandidateEvidence),
                "state" => matches!(
                    schema.kind,
                    BindingKind::FoldedRecords | BindingKind::ValidatedHistory
                ),
                _ => false,
            };
            if !allowed {
                return Err(DataFusionError::Plan(
                    "binding kind is not eligible for this catalog".into(),
                ));
            }
            for table in schema.tables.values() {
                enrichment_core::native_analysis::validate_derived_fields(table.schema().fields())?;
            }
        }
    }
    Ok(())
}

/// Finite authored conditional rules; these remain ordinary SQL plans, not a rule language.
pub(crate) struct SqlRule {
    pub id: &'static str,
    pub relation: &'static str,
    pub sql: &'static str,
}
impl SqlRule {
    pub(crate) async fn violations(&self, session: &SessionContext) -> Result<DataFrame> {
        session.sql(self.sql).await
    }
}

pub(crate) fn metadata(
    catalogs: &BTreeMap<String, Arc<dyn CatalogProvider>>,
    pool: &Arc<dyn datafusion::execution::memory_pool::MemoryPool>,
) -> Result<(Tables, InventorySummary)> {
    use arrow_schema::DataType;
    use datafusion::common::stats::Precision;
    validate_inventory(catalogs)?;
    const MAX_RELATIONS: usize = 128;
    const MAX_FIELDS: usize = 2048;
    const MAX_BYTES: usize = 256 * 1024;
    let mut summary = InventorySummary::default();
    let mut relations = Vec::new();
    let mut fields = Vec::new();
    let mut rules = Vec::new();
    let mut bytes = 0usize;
    'catalogs: for (catalog_name, catalog) in catalogs {
        let catalog = catalog.downcast_ref::<BoundCatalog>().ok_or_else(|| {
            DataFusionError::Internal("source catalog is not an immutable bound inventory".into())
        })?;
        if catalog.invalid {
            return Err(DataFusionError::Plan("duplicate bound schema".into()));
        }
        for (schema_name, schema) in &catalog.schemas {
            let schema = schema.downcast_ref::<BoundSchema>().ok_or_else(|| {
                DataFusionError::Internal("source schema is not immutable".into())
            })?;
            let binding_kind = schema.kind;
            for (name, table) in &schema.tables {
                if relations.len() == MAX_RELATIONS || bytes >= MAX_BYTES {
                    summary.truncated = true;
                    break 'catalogs;
                }
                let qualified =
                    TableReference::full(catalog_name.clone(), schema_name.clone(), name.clone())
                        .to_string();
                let schema = table.schema();
                let key = table.constraints().and_then(|keys| {
                    keys.iter().find_map(|key| match key {
                        Constraint::PrimaryKey(indices) if indices.len() == 1 => {
                            schema.fields().get(indices[0]).map(|f| f.name().clone())
                        }
                        _ => None,
                    })
                });
                let rows = table.statistics().and_then(|stats| match stats.num_rows {
                    Precision::Exact(rows) => Some(rows as u64),
                    _ => None,
                });
                let declaration = declaration(binding_kind, name);
                let rule_start = rules.len();
                if let Some(key) = declaration {
                    rules.push(RuleInfo {
                        relation: qualified.clone(),
                        rule_id: format!("{name}.unique"),
                        kind: RuleKind::Unique,
                        source_field: vec![key.into()],
                        target_relation: None,
                        target_field: None,
                        target_domain: None,
                        scope: Vec::new(),
                    });
                }
                let rule_set = match binding_kind {
                    BindingKind::AdmittedEvidence | BindingKind::CandidateEvidence => {
                        crate::admission::CONDITIONAL_RULES
                    }
                    BindingKind::FoldedRecords => crate::control::CONDITIONAL_RULES,
                    _ => &[],
                };
                for rule in rule_set.iter().filter(|rule| rule.relation == name) {
                    rules.push(RuleInfo {
                        relation: qualified.clone(),
                        rule_id: rule.id.into(),
                        kind: RuleKind::Conditional,
                        source_field: declaration.map(|key| vec![key.into()]).unwrap_or_default(),
                        target_relation: None,
                        target_field: None,
                        target_domain: None,
                        scope: Vec::new(),
                    });
                }
                let rule_bytes = rules[rule_start..]
                    .iter()
                    .map(RuleInfo::estimated_size)
                    .sum::<usize>();
                let cost = qualified.len() * 2 + 256 + rule_bytes;
                if cost > MAX_BYTES.saturating_sub(bytes) {
                    rules.truncate(rule_start);
                    summary.truncated = true;
                    break 'catalogs;
                }
                bytes += cost;
                relations.push(RelationInfo {
                    catalog_name: catalog_name.clone(),
                    schema_name: schema_name.clone(),
                    table_name: name.clone(),
                    table_type: format!("{:?}", table.table_type()),
                    binding_kind: binding_kind.trust().into(),
                    declared_key: declaration.map(str::to_owned),
                    validated_key: key,
                    verified_rows: rows,
                });
                summary.relations.push(qualified.clone());
                if schema.fields().len() > MAX_FIELDS {
                    summary.truncated = true;
                    continue;
                }
                let mut pending: Vec<_> = schema
                    .fields()
                    .iter()
                    .rev()
                    .map(|field| (field.clone(), Vec::<String>::new(), 0usize))
                    .collect();
                while let Some((field, prefix, depth)) = pending.pop() {
                    if fields.len() == MAX_FIELDS || bytes >= MAX_BYTES || depth > 32 {
                        summary.truncated = true;
                        break;
                    }
                    let mut path = prefix;
                    path.push(field.name().clone());
                    // Container labels avoid recursively rendering the entire nested schema.
                    let kind = match field.data_type() {
                        DataType::Struct(_) => "Struct".into(),
                        DataType::List(_)
                        | DataType::LargeList(_)
                        | DataType::ListView(_)
                        | DataType::LargeListView(_)
                        | DataType::FixedSizeList(_, _) => "List".into(),
                        DataType::Map(_, _) => "Map".into(),
                        DataType::Dictionary(_, _) => "Dictionary".into(),
                        DataType::Union(_, _) => "Union".into(),
                        DataType::RunEndEncoded(_, _) => "RunEndEncoded".into(),
                        other => other.to_string(),
                    };
                    let role = field.metadata().get("enrichment.role").cloned();
                    let reference =
                        reference_info(catalog_name, schema_name, &qualified, &path, &field)?;
                    let cost = qualified.len()
                        + path.iter().map(String::len).sum::<usize>()
                        + kind.len()
                        + role.as_ref().map_or(0, String::len)
                        + field
                            .metadata()
                            .iter()
                            .map(|(key, value)| key.len() + value.len() + 48)
                            .sum::<usize>()
                        + reference.as_ref().map_or(0, RuleInfo::estimated_size)
                        + 64;
                    if cost > MAX_BYTES.saturating_sub(bytes) {
                        summary.truncated = true;
                        break;
                    }
                    bytes += cost;
                    rules.extend(reference);
                    fields.push(FieldInfo {
                        relation: qualified.clone(),
                        field_path: path.clone(),
                        data_type: kind,
                        nullable: field.is_nullable(),
                        required_by_contract:
                            enrichment_core::evidence::arrow_model::checks::required(&field),
                        semantic_role: role,
                        metadata: field
                            .metadata()
                            .iter()
                            .map(|(key, value)| (key.clone(), value.clone()))
                            .collect(),
                    });
                    match field.data_type() {
                        DataType::Struct(children) => {
                            if pending.len() + children.len() > MAX_FIELDS {
                                summary.truncated = true;
                                break;
                            }
                            pending.extend(
                                children
                                    .iter()
                                    .rev()
                                    .map(|child| (child.clone(), path.clone(), depth + 1)),
                            );
                        }
                        DataType::List(child)
                        | DataType::LargeList(child)
                        | DataType::ListView(child)
                        | DataType::LargeListView(child)
                        | DataType::FixedSizeList(child, _)
                        | DataType::Map(child, _) => pending.push((child.clone(), path, depth + 1)),
                        DataType::Union(children, _) => {
                            if pending.len() + children.len() > MAX_FIELDS {
                                summary.truncated = true;
                                break;
                            }
                            pending.extend(
                                children
                                    .iter()
                                    .map(|(_, child)| (child.clone(), path.clone(), depth + 1)),
                            );
                        }
                        DataType::RunEndEncoded(ends, values) => {
                            pending.push((values.clone(), path.clone(), depth + 1));
                            pending.push((ends.clone(), path, depth + 1));
                        }
                        DataType::Dictionary(_, value) => {
                            pending.push((
                                Arc::new(arrow_schema::Field::new(
                                    "dictionary_values",
                                    value.as_ref().clone(),
                                    true,
                                )),
                                path,
                                depth + 1,
                            ));
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    summary.nested_fields = fields.len();
    let mut tables = Tables::new();
    for (name, batch) in [
        ("relations", RelationInfo::batch(&relations)?),
        ("fields", FieldInfo::batch(&fields)?),
        ("rules", RuleInfo::batch(&rules)?),
        (
            "inventory",
            InventoryInfo::batch(&[InventoryInfo {
                relations: summary.relations.len() as u64,
                nested_fields: summary.nested_fields as u64,
                truncated: summary.truncated,
            }])?,
        ),
    ] {
        tables.insert(
            name.into(),
            Arc::new(crate::admitted_provider::AdmittedProvider::from_batch(
                batch, pool,
            )?) as Arc<dyn TableProvider>,
        );
    }
    Ok((tables, summary))
}

pub(crate) fn summary(state: &datafusion::execution::SessionState) -> InventorySummary {
    state
        .catalog_list()
        .catalog("operation")
        .and_then(|catalog| {
            catalog
                .downcast_ref::<BoundCatalog>()
                .and_then(|bound| bound.inventory.clone())
        })
        .unwrap_or_else(|| InventorySummary {
            truncated: true,
            ..InventorySummary::default()
        })
}

impl CatalogProvider for BoundCatalog {
    fn schema_names(&self) -> Vec<String> {
        self.schemas.keys().cloned().collect()
    }
    fn schema(&self, name: &str) -> Option<Arc<dyn SchemaProvider>> {
        self.schemas.get(name).cloned()
    }
}

/// Only temporary operation objects may be registered after immutable roots are installed.
/// A finite Arrow protocol input gets its own native relation identity. Anonymous `?table?`
/// sources become ambiguous when independent nested projections meet in a control union.
/// The captured provider exposes no mutation handle and grants no row constraints. Shared
/// buffers carry a fallible pool claim through the final retained input or cache reader.
pub(crate) fn batch(
    session: &datafusion::prelude::SessionContext,
    name: &str,
    batch: arrow::record_batch::RecordBatch,
) -> Result<datafusion::dataframe::DataFrame> {
    captured_batches(session, name, vec![batch])
}

/// Rebind materialized Arrow results without reopening a mutable ingress. All batches must
/// carry the same full field contract. Shared buffers keep payment through physical readers.
pub(crate) fn captured_batches(
    session: &SessionContext,
    name: &str,
    batches: Vec<arrow::record_batch::RecordBatch>,
) -> Result<DataFrame> {
    let provider = Arc::new(crate::admitted_provider::AdmittedProvider::from_batches(
        batches,
        &session.runtime_env().memory_pool,
    )?);
    let plan = datafusion::logical_expr::LogicalPlanBuilder::scan(
        format!("{name}_{}", uuid::Uuid::new_v4().simple()),
        datafusion::datasource::provider_as_source(provider),
        None,
    )?
    .build()?;
    Ok(datafusion::dataframe::DataFrame::new(session.state(), plan))
}

pub(crate) fn work(
    session: &SessionContext,
    name: impl AsRef<str>,
    table: Arc<dyn TableProvider>,
) -> Result<()> {
    let name = name.as_ref();
    if name.is_empty() || !name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_') {
        return Err(DataFusionError::Plan("invalid operation work name".into()));
    }
    let reference = TableReference::full("operation", "work", name);
    for catalog in session
        .catalog_names()
        .into_iter()
        .filter(|catalog| catalog != "operation")
    {
        if let Some(catalog) = session.catalog(&catalog) {
            for schema in catalog.schema_names() {
                if catalog
                    .schema(&schema)
                    .is_some_and(|schema| schema.table_exist(name))
                {
                    return Err(DataFusionError::Plan(format!(
                        "work name {name} shadows a bound source"
                    )));
                }
            }
        }
    }
    if session.table_exist(reference.clone())? {
        return Err(DataFusionError::Plan(format!(
            "operation work table {name} already exists"
        )));
    }
    session.register_table(reference, table)?;
    Ok(())
}

/// A finite input uses the same collision/shadowing policy as every operation work relation.
/// Give its anonymous source a unique native name before installing the immutable view.
pub(crate) fn input(
    session: &SessionContext,
    name: &str,
    value: arrow::record_batch::RecordBatch,
) -> Result<()> {
    work(session, name, batch(session, name, value)?.into_view())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::{QueryLimits, QueryRuntime};
    use arrow::{
        array::{Array, Int32Array, StringArray, StructArray},
        datatypes::{DataType, Field},
        record_batch::RecordBatch,
    };
    use datafusion::{catalog::MemorySchemaProvider, datasource::MemTable};

    #[tokio::test]
    async fn captured_batches_keep_payment_and_schema_through_last_physical_reader() -> Result<()> {
        use datafusion::execution::{
            memory_pool::{GreedyMemoryPool, MemoryPool},
            runtime_env::RuntimeEnvBuilder,
        };
        let pool: Arc<dyn MemoryPool> = Arc::new(GreedyMemoryPool::new(1 << 20));
        let session = SessionContext::new_with_config_rt(
            Default::default(),
            Arc::new(
                RuntimeEnvBuilder::new()
                    .with_memory_pool(pool.clone())
                    .build()?,
            ),
        );
        let original = RecordBatch::try_from_iter([(
            "value",
            Arc::new(StringArray::from(vec!["first", "second", "third"])) as arrow::array::ArrayRef,
        )])?;
        let batches = vec![original.slice(0, 1), original.slice(1, 2)];
        let frame = captured_batches(&session, "captured", batches)?;
        let charge = pool.reserved();
        assert!(charge > 0);
        assert!(captured_batches(&session, "empty", vec![]).is_err());
        let field = Field::new("value", DataType::Utf8, false).with_metadata(
            std::collections::HashMap::from([("different_contract".into(), "true".into())]),
        );
        let changed = RecordBatch::try_new(
            Arc::new(arrow_schema::Schema::new(vec![field])),
            original.columns().to_vec(),
        )?;
        assert!(captured_batches(&session, "mismatch", vec![original.clone(), changed]).is_err());
        assert_eq!(
            pool.reserved(),
            charge,
            "refusal preserves the original payment"
        );
        let plan = frame.create_physical_plan().await?;
        drop(frame);
        drop(original);
        assert_eq!(
            pool.reserved(),
            charge,
            "physical plan retains captured buffers"
        );
        let output = datafusion::physical_plan::collect(plan.clone(), session.task_ctx()).await?;
        drop(plan);
        assert_eq!(output.iter().map(RecordBatch::num_rows).sum::<usize>(), 3);
        assert!(pool.reserved() >= charge);
        drop(output);
        assert_eq!(
            pool.reserved(),
            0,
            "last reader releases buffer reservations"
        );
        Ok(())
    }

    #[tokio::test]
    async fn native_catalog_segments_keep_literal_dots_and_nested_references_distinct() -> Result<()>
    {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let reference_field = |name| {
            enrichment_core::native_union::field::<String>(
                name,
                Rule::foreign_key("target", "key.dot"),
            )
        };
        let leaf = Arc::new(reference_field("value"));
        let nested = Arc::new(StructArray::new(
            vec![leaf].into(),
            vec![Arc::new(StringArray::from(vec!["missing"]))],
            None,
        ));
        let source = RecordBatch::try_new(
            Arc::new(arrow_schema::Schema::new(vec![
                Field::new("owner", DataType::Utf8, false),
                reference_field("nested.value"),
                Field::new("nested", nested.data_type().clone(), false),
            ])),
            vec![
                Arc::new(StringArray::from(vec!["source"])),
                Arc::new(StringArray::from(vec!["retained"])),
                nested,
            ],
        )?;
        let declared = source.schema();
        let target = RecordBatch::try_from_iter(vec![(
            "key.dot",
            Arc::new(StringArray::from(vec!["retained"])) as arrow::array::ArrayRef,
        )])?;
        let session = runtime.session();
        let provider =
            |name, record| Ok::<_, DataFusionError>(batch(&session, name, record)?.into_view());
        let catalog = Arc::new(BoundCatalog::default().with_schema(
            BindingKind::CandidateEvidence,
            Tables::from([
                ("source".into(), provider("source", source)?),
                ("target".into(), provider("target", target)?),
            ]),
        ));
        let session = runtime.bound_session(BTreeMap::from([(
            "candidate".into(),
            catalog as Arc<dyn CatalogProvider>,
        )]))?;
        let references = crate::field_admission::violations(
            &session,
            TableReference::full("candidate", "evidence", "source"),
            declared.as_ref(),
            "owner",
            crate::field_admission::ReferenceNamespace::Records,
        )
        .await?;
        assert_eq!(references.len(), 2);
        let mut outcomes = Vec::new();
        for (_, plan) in references {
            outcomes.push(runtime.execute(plan).await?.rows);
        }
        assert_eq!(outcomes, vec![0, 1]);
        let rules = runtime.records::<RuleInfo>(session.sql("SELECT * FROM operation.metadata.rules WHERE relation='candidate.evidence.source' ORDER BY source_field").await?, 2).await?;
        assert_eq!(rules.len(), 2);
        assert_ne!(rules[0].source_field, rules[1].source_field);
        assert!(
            rules
                .iter()
                .all(|rule| rule.target_field == Some(vec!["key.dot".into()])
                    && rule.target_relation.as_deref() == Some("candidate.evidence.target"))
        );
        let paths = runtime.records::<FieldInfo>(session.sql("SELECT * FROM operation.metadata.fields WHERE relation='candidate.evidence.source' AND (field_path=['nested.value'] OR field_path=['nested','value']) ORDER BY field_path").await?,2).await?;
        assert_eq!(paths.len(), 2);
        assert_ne!(paths[0].field_path, paths[1].field_path);
        let metadata = session.table_provider("operation.metadata.fields").await?;
        assert!(
            metadata
                .downcast_ref::<crate::admitted_provider::AdmittedProvider>()
                .is_some()
        );
        assert!(
            metadata
                .constraints()
                .is_some_and(|constraints| constraints.is_empty())
        );
        runtime.close_diagnostics().await?;
        Ok(())
    }

    #[tokio::test]
    async fn bound_inventory_is_immutable_consistent_and_drives_nested_metadata() {
        let dir = tempfile::tempdir().unwrap();
        let runtime = QueryRuntime::new(dir.path(), QueryLimits::default()).unwrap();
        let leaf = Arc::new(Field::new("value", DataType::Int32, false).with_metadata(
            std::collections::HashMap::from([(
                "enrichment.role".into(),
                "test:semantic-value".into(),
            )]),
        ));
        let nested = StructArray::from(vec![(
            leaf,
            Arc::new(Int32Array::from(vec![1])) as arrow::array::ArrayRef,
        )]);
        let batch = RecordBatch::try_from_iter(vec![(
            "nested",
            Arc::new(nested) as arrow::array::ArrayRef,
        )])
        .unwrap();
        let provider = Arc::new(MemTable::try_new(batch.schema(), vec![vec![batch]]).unwrap())
            as Arc<dyn TableProvider>;
        let catalog = Arc::new(BoundCatalog::default().with_schema(
            crate::native_catalog::BindingKind::AdmittedEvidence,
            Tables::from([("sample".into(), provider.clone())]),
        ));
        let schema = catalog.schema("evidence").unwrap();
        assert_eq!(schema.table_names(), vec!["sample"]);
        assert!(schema.table_exist("sample"));
        assert_eq!(
            schema.table_type("sample").await.unwrap(),
            Some(TableType::Base)
        );
        assert!(schema.table("absent").await.unwrap().is_none());
        assert_eq!(schema.table_type("absent").await.unwrap(), None);
        assert!(
            schema
                .register_table("sample".into(), provider.clone())
                .is_err()
        );
        assert!(schema.deregister_table("sample").is_err());
        assert!(
            catalog
                .register_schema("other", Arc::new(MemorySchemaProvider::new()))
                .is_err()
        );
        let session = runtime
            .bound_session(BTreeMap::from([(
                "snapshot".into(),
                catalog as Arc<dyn CatalogProvider>,
            )]))
            .unwrap();
        assert!(work(&session, "sample", provider).is_err());
        let metadata = runtime.execute(session.sql("SELECT semantic_role FROM operation.metadata.fields WHERE field_path = ['nested','value']").await.unwrap()).await.unwrap();
        assert_eq!(metadata.rows, 1);
        assert_eq!(
            crate::projection::TextColumn::new(metadata.batches[0].column(0).as_ref())
                .unwrap()
                .get(0),
            Some("test:semantic-value")
        );
        let fields = runtime.records::<FieldInfo>(session.sql("SELECT * FROM operation.metadata.fields WHERE field_path = ['nested','value']").await.unwrap(), 1).await.unwrap();
        assert_eq!(
            fields[0]
                .metadata
                .get("enrichment.role")
                .map(String::as_str),
            Some("test:semantic-value")
        );
        let inventory = runtime.execute(session.sql("SELECT table_name FROM information_schema.tables WHERE table_catalog = 'snapshot' AND table_schema = 'evidence'").await.unwrap()).await.unwrap();
        assert_eq!(inventory.rows, 1);
        let retained = session.table("snapshot.evidence.sample").await.unwrap();
        let clone = datafusion::prelude::SessionContext::new_with_state(session.state());
        assert!(clone.deregister_table("snapshot.evidence.sample").is_err());
        assert_eq!(runtime.execute(retained).await.unwrap().rows, 1);
    }

    #[tokio::test]
    async fn invalid_inventory_installs_nothing_and_discovery_reports_its_bound() {
        let dir = tempfile::tempdir().unwrap();
        let runtime = QueryRuntime::new(dir.path(), QueryLimits::default()).unwrap();
        let existing = runtime.session();
        let duplicate = BoundCatalog::default()
            .with_schema(BindingKind::AdmittedEvidence, Tables::new())
            .with_schema(BindingKind::AdmittedEvidence, Tables::new());
        assert!(
            runtime
                .bound_session(BTreeMap::from([(
                    "snapshot".into(),
                    Arc::new(duplicate) as Arc<dyn CatalogProvider>
                )]))
                .is_err()
        );
        let candidate =
            BoundCatalog::default().with_schema(BindingKind::CandidateEvidence, Tables::new());
        assert!(
            runtime
                .bound_session(BTreeMap::from([(
                    "snapshot".into(),
                    Arc::new(candidate) as Arc<dyn CatalogProvider>
                )]))
                .is_err()
        );
        assert!(existing.catalog("snapshot").is_none());
        let schema = Arc::new(arrow_schema::Schema::new(
            (0..3000)
                .map(|index| Field::new(format!("value_{index}"), DataType::Int32, true))
                .collect::<Vec<_>>(),
        ));
        let provider =
            Arc::new(MemTable::try_new(schema, vec![vec![]]).unwrap()) as Arc<dyn TableProvider>;
        let catalog = BoundCatalog::default().with_schema(
            BindingKind::AdmittedEvidence,
            Tables::from([("wide".into(), provider)]),
        );
        let session = runtime
            .bound_session(BTreeMap::from([(
                "snapshot".into(),
                Arc::new(catalog) as Arc<dyn CatalogProvider>,
            )]))
            .unwrap();
        let metadata = summary(&session.state());
        assert!(metadata.truncated);
        assert_eq!(metadata.nested_fields, 0);
        assert!(session.table_exist("snapshot.evidence.wide").unwrap());
        assert_eq!(
            runtime
                .execute(
                    session
                        .sql("SELECT * FROM operation.metadata.fields")
                        .await
                        .unwrap()
                )
                .await
                .unwrap()
                .rows,
            0
        );
    }

    #[derive(Clone, Copy)]
    struct Reordered;
    impl RelationContract for Reordered {
        fn name(self) -> &'static str {
            "reordered"
        }
        fn key(self) -> &'static str {
            "id"
        }
        fn schema(self) -> Result<SchemaRef> {
            Ok(Arc::new(arrow_schema::Schema::new(vec![
                Field::new("value", DataType::Utf8, true),
                Field::new("id", DataType::Utf8, false),
            ])))
        }
    }

    #[tokio::test]
    async fn declared_key_position_and_native_duplicate_plan_follow_reordered_schema() {
        assert_eq!(
            Reordered.validated_constraints().unwrap().iter().next(),
            Some(&Constraint::PrimaryKey(vec![1]))
        );
        let dir = tempfile::tempdir().unwrap();
        let runtime = QueryRuntime::new(dir.path(), QueryLimits::default()).unwrap();
        let batch = RecordBatch::try_new(
            Reordered.schema().unwrap(),
            vec![
                Arc::new(StringArray::from(vec!["a", "b"])),
                Arc::new(StringArray::from(vec!["same", "same"])),
            ],
        )
        .unwrap();
        let table = Arc::new(MemTable::try_new(batch.schema(), vec![vec![batch]]).unwrap())
            as Arc<dyn TableProvider>;
        let catalog = BoundCatalog::default().with_schema(
            crate::native_catalog::BindingKind::CandidateEvidence,
            Tables::from([("reordered".into(), table)]),
        );
        let session = runtime
            .bound_session(BTreeMap::from([(
                "candidate".into(),
                Arc::new(catalog) as Arc<dyn CatalogProvider>,
            )]))
            .unwrap();
        let duplicates = Reordered
            .duplicate_keys(
                &session,
                TableReference::full("candidate", "evidence", "reordered"),
            )
            .await
            .unwrap();
        assert_eq!(runtime.execute(duplicates).await.unwrap().rows, 1);
    }
}
