//! Immutable, completely constructed native namespaces and their declaration authority.
//!
//! Durable publication and admission own trust. These providers project already captured
//! inputs; lookup never refreshes a generation, acquires evidence, or changes visibility.

use enrichment_core::telemetry::InventorySummary;
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
    fn references(self) -> &'static [ReferenceRule];

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
    fn references(self) -> &'static [ReferenceRule] {
        match self {
            Self::Symbols => &[ReferenceRule {
                id: "symbol_definition",
                field: "definition_id",
                target: "definitions",
                target_field: "definition_id",
                nullable: false,
                when: None,
            }],
            Self::Coverage => COVERAGE_REFERENCES,
            Self::InputArtifacts => &[ReferenceRule {
                id: "semantic_producer_binding",
                field: "producer_binding_id",
                target: "producer_runs",
                target_field: "producer_binding_id",
                nullable: false,
                when: None,
            }],
            _ => &[],
        }
    }
}

const COVERAGE_REFERENCES: &[ReferenceRule] = &[ReferenceRule {
    id: "semantic_producer_binding",
    field: "producer_binding_id",
    target: "producer_runs",
    target_field: "producer_binding_id",
    nullable: false,
    when: None,
}];

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
    fn references(self) -> &'static [ReferenceRule] {
        match self {
            Self::Contexts => &[
                ReferenceRule {
                    id: "context_release",
                    field: "release_id",
                    target: "releases",
                    target_field: "release_id",
                    nullable: false,
                    when: None,
                },
                ReferenceRule {
                    id: "context_environment",
                    field: "environment_id",
                    target: "environments",
                    target_field: "environment_id",
                    nullable: false,
                    when: None,
                },
                ReferenceRule {
                    id: "context_parent",
                    field: "parent_context_id",
                    target: "contexts",
                    target_field: "context_id",
                    nullable: true,
                    when: None,
                },
            ],
            Self::Snapshots => &[ReferenceRule {
                id: "snapshot_context",
                field: "context_id",
                target: "contexts",
                target_field: "context_id",
                nullable: false,
                when: None,
            }],
            Self::Attempts => &[ReferenceRule {
                id: "attempt_snapshot",
                field: "snapshot_id",
                target: "snapshots",
                target_field: "snapshot_id",
                nullable: false,
                when: None,
            }],
            Self::JobTransitions | Self::Claims | Self::Interests => &[ReferenceRule {
                id: "job_command",
                field: "job_id",
                target: "commands",
                target_field: "job_id",
                nullable: false,
                when: None,
            }],
            _ => &[],
        }
    }
}

/// Repeated mechanical closure checks; conditional/composite semantics remain explicit plans.
#[derive(Clone, Copy)]
pub(crate) struct ReferenceRule {
    pub(crate) id: &'static str,
    pub(crate) field: &'static str,
    pub(crate) target: &'static str,
    pub(crate) target_field: &'static str,
    pub(crate) nullable: bool,
    pub(crate) when: Option<(&'static str, &'static str)>,
}

impl ReferenceRule {
    pub(crate) async fn violations(
        &self,
        session: &SessionContext,
        source: TableReference,
        key: &str,
    ) -> Result<DataFrame> {
        let target = TableReference::full(
            source
                .catalog()
                .ok_or_else(|| DataFusionError::Internal("rule requires a catalog".into()))?,
            source
                .schema()
                .ok_or_else(|| DataFusionError::Internal("rule requires a schema".into()))?,
            self.target,
        );
        let mut input = session.table(source).await?;
        let value = |path: &str| {
            use datafusion::functions::core::expr_ext::FieldAccessor;
            let mut parts = path.split('.');
            let root = col(parts.next().expect("declared reference path"));
            parts.fold(root, |expr, part| expr.field(part))
        };
        if let Some((field, expected)) = self.when {
            input = input.filter(value(field).eq(lit(expected)))?;
        }
        if self.nullable {
            input = input.filter(value(self.field).is_not_null())?;
        }
        input = input.with_column("reference_value", value(self.field))?;
        input
            .join(
                session.table(target).await?,
                datafusion::logical_expr::JoinType::LeftAnti,
                &["reference_value"],
                &[self.target_field],
                None,
            )?
            .select(vec![col(key)])?
            .limit(0, Some(1))
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
}
impl BindingKind {
    fn schema(self) -> &'static str {
        match self {
            Self::AdmittedEvidence | Self::CandidateEvidence => "evidence",
            Self::AdmittedDomain => "domain",
            Self::FoldedRecords => "records",
            Self::ValidatedHistory => "history",
            Self::Metadata => "metadata",
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
        }
    }
}

/// Structural namespace. Trust and source ownership reside in its captured table providers.
#[derive(Debug, Clone)]
pub(crate) struct BoundSchema {
    tables: Tables,
    kind: BindingKind,
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

/// Query diagnostics and native metadata derive from these same immutable bindings.
#[derive(Clone)]
pub(crate) struct RelationInfo {
    pub catalog: String,
    pub schema: String,
    pub name: String,
    pub kind: String,
    pub trust: String,
    pub declared_key: Option<String>,
    pub key: Option<String>,
    pub rows: Option<u64>,
}

pub(crate) struct RuleInfo {
    pub relation: String,
    pub id: String,
    pub kind: &'static str,
    pub field: String,
    pub target: Option<String>,
    pub target_field: Option<String>,
    pub condition: Option<(&'static str, &'static str)>,
}

fn declaration(kind: BindingKind, name: &str) -> Option<(&'static str, &'static [ReferenceRule])> {
    if matches!(
        kind,
        BindingKind::AdmittedEvidence | BindingKind::CandidateEvidence
    ) {
        crate::admission::Relation::ALL
            .into_iter()
            .find(|relation| relation.name() == name)
            .map(|relation| (relation.key(), relation.references()))
    } else if kind == BindingKind::FoldedRecords {
        crate::control::Table::ALL
            .into_iter()
            .find(|table| table.name() == name)
            .map(|table| (table.key(), table.references()))
    } else {
        None
    }
}

pub(crate) struct FieldInfo {
    pub relation: String,
    pub path: String,
    pub kind: String,
    pub nullable: bool,
    pub required_by_contract: bool,
    pub role: Option<String>,
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
) -> Result<(Tables, InventorySummary)> {
    use arrow_schema::DataType;
    use datafusion::{common::stats::Precision, datasource::MemTable};
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
                if let Some((key, references)) = declaration {
                    rules.push(RuleInfo {
                        relation: qualified.clone(),
                        id: format!("{name}.unique"),
                        kind: "unique",
                        field: key.into(),
                        target: None,
                        target_field: None,
                        condition: None,
                    });
                    for rule in references {
                        rules.push(RuleInfo {
                            relation: qualified.clone(),
                            id: rule.id.into(),
                            kind: if rule.when.is_some() {
                                "conditional_reference"
                            } else if rule.nullable {
                                "nullable_reference"
                            } else {
                                "reference"
                            },
                            field: rule.field.into(),
                            target: Some(
                                TableReference::full(
                                    catalog_name.clone(),
                                    schema_name.clone(),
                                    rule.target,
                                )
                                .to_string(),
                            ),
                            target_field: Some(rule.target_field.into()),
                            condition: rule.when,
                        });
                    }
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
                        id: rule.id.into(),
                        kind: "conditional",
                        field: declaration.map_or("", |(key, _)| key).into(),
                        target: None,
                        target_field: None,
                        condition: None,
                    });
                }
                let rule_bytes = rules
                    .iter()
                    .filter(|rule| rule.relation == qualified)
                    .map(|rule| {
                        rule.relation.len()
                            + rule.id.len()
                            + rule.field.len()
                            + rule.target.as_ref().map_or(0, String::len)
                            + rule
                                .condition
                                .map_or(0, |(field, value)| field.len() + value.len())
                            + 128
                    })
                    .sum::<usize>();
                let cost = qualified.len() * 2 + 256 + rule_bytes;
                if cost > MAX_BYTES.saturating_sub(bytes) {
                    rules.truncate(rule_start);
                    summary.truncated = true;
                    break 'catalogs;
                }
                bytes += cost;
                relations.push(RelationInfo {
                    catalog: catalog_name.clone(),
                    schema: schema_name.clone(),
                    name: name.clone(),
                    kind: format!("{:?}", table.table_type()),
                    trust: binding_kind.trust().into(),
                    declared_key: declaration.map(|(key, _)| key.to_owned()),
                    key,
                    rows,
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
                    .map(|field| (field.as_ref(), String::new(), 0usize))
                    .collect();
                while let Some((field, prefix, depth)) = pending.pop() {
                    if fields.len() == MAX_FIELDS || bytes >= MAX_BYTES || depth > 32 {
                        summary.truncated = true;
                        break;
                    }
                    let path = if prefix.is_empty() {
                        field.name().clone()
                    } else {
                        format!("{prefix}.{}", field.name())
                    };
                    // Container labels avoid recursively rendering the entire nested schema.
                    let kind = match field.data_type() {
                        DataType::Struct(_) => "Struct".into(),
                        DataType::List(_)
                        | DataType::LargeList(_)
                        | DataType::FixedSizeList(_, _) => "List".into(),
                        DataType::Map(_, _) => "Map".into(),
                        other => other.to_string(),
                    };
                    let role = field.metadata().get("enrichment.role").cloned();
                    let cost = qualified.len()
                        + path.len()
                        + kind.len()
                        + role.as_ref().map_or(0, String::len)
                        + 64;
                    if cost > MAX_BYTES.saturating_sub(bytes) {
                        summary.truncated = true;
                        break;
                    }
                    bytes += cost;
                    fields.push(FieldInfo {
                        relation: qualified.clone(),
                        path: path.clone(),
                        kind,
                        nullable: field.is_nullable(),
                        required_by_contract:
                            enrichment_core::evidence::arrow_model::checks::required(field),
                        role,
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
                                    .map(|child| (child.as_ref(), path.clone(), depth + 1)),
                            );
                        }
                        DataType::List(child)
                        | DataType::LargeList(child)
                        | DataType::FixedSizeList(child, _)
                        | DataType::Map(child, _) => {
                            pending.push((child.as_ref(), path, depth + 1))
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
        (
            "relations",
            crate::projection::contracts::relations(&relations)?,
        ),
        ("fields", crate::projection::contracts::fields(&fields)?),
        ("rules", crate::projection::contracts::rules(&rules)?),
        (
            "inventory",
            crate::projection::contracts::inventory(&summary)?,
        ),
    ] {
        tables.insert(
            name.into(),
            Arc::new(MemTable::try_new(batch.schema(), vec![vec![batch]])?)
                as Arc<dyn TableProvider>,
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
pub(crate) fn batch(
    session: &datafusion::prelude::SessionContext,
    name: &str,
    batch: arrow::record_batch::RecordBatch,
) -> Result<datafusion::dataframe::DataFrame> {
    let provider = Arc::new(datafusion::datasource::MemTable::try_new(
        batch.schema(),
        vec![vec![batch]],
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::{QueryLimits, QueryRuntime};
    use arrow::{
        array::{Int32Array, StringArray, StructArray},
        datatypes::{DataType, Field},
        record_batch::RecordBatch,
    };
    use datafusion::{catalog::MemorySchemaProvider, datasource::MemTable};

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
        let metadata = runtime.execute(session.sql("SELECT semantic_role FROM operation.metadata.fields WHERE field_path = 'nested.value'").await.unwrap()).await.unwrap();
        assert_eq!(metadata.rows, 1);
        assert_eq!(
            crate::projection::TextColumn::new(metadata.batches[0].column(0).as_ref())
                .unwrap()
                .get(0),
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
        fn references(self) -> &'static [ReferenceRule] {
            &[]
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
