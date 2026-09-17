//! Authoritative typed identity expressions, shared by native plans and bounded DTO ingress.
use arrow::{
    datatypes::{DataType, Field, Schema, SchemaRef},
    record_batch::RecordBatch,
};
use datafusion::{
    common::{DFSchema, ScalarValue},
    error::{DataFusionError, Result},
    functions::{crypto::expr_fn::sha256, encoding::expr_fn::encode, string::expr_fn::concat},
    logical_expr::{Expr, simplify::SimplifyContext},
    optimizer::simplify_expressions::ExprSimplifier,
    prelude::{col, lit},
};
use std::sync::Arc;

/// An ordered contract is data; the native function graph is identical at every consumer.
#[derive(Debug, Clone, Copy)]
pub enum Key {
    Release,
    Environment,
    Context,
    Snapshot,
    SnapshotDescriptor,
    Projection,
    PublicPath,
    PublicBinding,
    Definition,
    ApiObservation,
    TextFragment,
    Citation,
    InputArtifact,
    Coverage,
    Relationship,
    ExecutionObservation,
    ReleaseMetadata,
    ProducerBinding,
    ProducerPlan,
    ArtifactReceipt,
    SnapshotAttempt,
    OperationPolicy,
    OperationCommand,
    EffectGrant,
    ExecutionConfiguration,
    ProcessOperation,
    ProcessEffect,
    ProcessBinding,
}
fn text(name: &str, nullable: bool) -> Field {
    Field::new(name, DataType::Utf8, nullable)
}
fn strings(name: &str) -> Field {
    Field::new(name, DataType::List(Arc::new(text("item", false))), false)
}
fn map(name: &str) -> Field {
    Field::new(
        name,
        DataType::Map(
            Arc::new(Field::new(
                "entries",
                DataType::Struct(vec![text("key", false), text("value", true)].into()),
                false,
            )),
            false,
        ),
        false,
    )
}
impl Key {
    pub fn prefix(self) -> &'static str {
        match self {
            Self::Release => "rel",
            Self::Environment => "env",
            Self::Context => "ctx",
            Self::Snapshot => "snap",
            Self::SnapshotDescriptor => "snapshot_descriptor",
            Self::Projection => "projection",
            Self::PublicPath => "path",
            Self::PublicBinding => "symbol",
            Self::Definition => "def",
            Self::ApiObservation => "obs",
            Self::TextFragment => "fragment",
            Self::Citation => "ev",
            Self::InputArtifact => "input",
            Self::Coverage => "coverage",
            Self::Relationship => "relationship",
            Self::ExecutionObservation => "exec",
            Self::ReleaseMetadata => "metadata",
            Self::ProducerBinding => "producer",
            Self::ProducerPlan => "producer_plan",
            Self::ArtifactReceipt => "receipt",
            Self::SnapshotAttempt => "association",
            Self::OperationPolicy => "policy",
            Self::OperationCommand => "command",
            Self::EffectGrant => "grant",
            Self::ExecutionConfiguration => "execution_config",
            Self::ProcessOperation => "process",
            Self::ProcessEffect => "process_effect",
            Self::ProcessBinding => "process_binding",
        }
    }
    pub fn schema(self) -> SchemaRef {
        match self {
            Self::Definition => return Arc::new(Schema::new(<crate::evidence::model::DefinitionIdentity as crate::native_union::NativeStruct>::fields())),
            Self::Citation => return Arc::new(Schema::new(<crate::wire::evidence::CitationIdentity as crate::native_union::NativeStruct>::fields())),
            Self::Projection => return Arc::new(Schema::new(<crate::operation::projections::ProjectionIdentity as crate::native_union::NativeStruct>::fields())),
            Self::SnapshotDescriptor => return Arc::new(Schema::new(<crate::evidence::snapshot::SnapshotDescriptor as crate::native_union::NativeStruct>::fields())),
            Self::OperationPolicy => return crate::operation::policy_schema(),
            Self::OperationCommand => return crate::operation::command_key_schema(),
            Self::EffectGrant => return crate::operation::grant_key_schema(),
            Self::ExecutionConfiguration => return crate::operation::execution_schema(),
            Self::ProcessOperation => return crate::operation::process_schema(),
            Self::ProcessEffect => return crate::operation::process_effect_schema(),
            _ => {}
        }
        if matches!(self, Self::ArtifactReceipt) {
            return Arc::new(Schema::new(vec![Field::new(
                "artifact",
                crate::evidence::arrow_model::acquisitions::data_type(),
                false,
            )]));
        }
        use crate::evidence::arrow_model::{encode, execution, metadata, provenance, relations};
        if matches!(self, Self::ProducerBinding | Self::ProducerPlan) {
            let batch = provenance::producer_fields(&[]).expect("native producer contract");
            let mut names = vec![
                "producer",
                "producer_version",
                "config_digest",
                "inputs",
                "profile",
            ];
            if matches!(self, Self::ProducerBinding) {
                names.extend(["outcome", "gaps"]);
            }
            return Arc::new(Schema::new(
                names
                    .into_iter()
                    .map(|name| {
                        batch
                            .schema()
                            .field_with_name(name)
                            .expect("native producer field")
                            .clone()
                    })
                    .collect::<Vec<_>>(),
            ));
        }
        let native = match self {
            Self::ApiObservation => Some((encode::observations_fields(&[]), "observation_id")),
            Self::TextFragment => Some((relations::fragments_fields(&[]), "fragment_id")),
            Self::InputArtifact => Some((provenance::input_artifacts_fields(&[]), "input_id")),
            Self::Coverage => Some((provenance::coverage_fields(&[]), "coverage_id")),
            Self::Relationship => Some((relations::relationships_fields(&[]), "relationship_id")),
            Self::ExecutionObservation => Some((execution::fields(&[]), "observation_id")),
            Self::ReleaseMetadata => Some((metadata::fields(&[]), "metadata_id")),
            _ => None,
        };
        if let Some((batch, key)) = native {
            let batch = batch.expect("declared Arrow evidence contract");
            return Arc::new(Schema::new(
                batch
                    .schema()
                    .fields()
                    .iter()
                    .filter(|field| field.name() != key)
                    .cloned()
                    .collect::<Vec<_>>(),
            ));
        }
        let fields = match self {
            Self::ProcessBinding => <crate::operation::jobs::ProcessBinding as crate::native_union::NativeStruct>::fields().iter().map(|field| field.as_ref().clone()).collect(),
            Self::SnapshotAttempt => vec![text("snapshot_id", false), text("attempt_id", false)],
            Self::Release => vec![
                text("ecosystem", false),
                text("registry", false),
                text("package", false),
                text("version", false),
                text("artifact_digest", true),
            ],
            Self::Environment => vec![
                text("resolution", false),
                text("toolchain", true),
                text("target", true),
                crate::native_union::field::<Vec<String>>(
                    "features",
                    crate::native_union::Rule::Set,
                )
                .with_nullable(false),
                Field::new("features_known", DataType::Boolean, false),
                Field::new("default_features", DataType::Boolean, true),
                text("lock_digest", true),
            ],
            Self::Context => vec![
                text("release_id", false),
                text("environment_id", false),
                text("mode", false),
            ],
            Self::Snapshot => vec![
                text("schema_version", false),
                text("normalizer_version", false),
                text("context_id", false),
                map("input_digests"),
                map("producers"),
            ],
            Self::PublicPath => vec![text("ecosystem", false), strings("components")],
            Self::PublicBinding => vec![
                text("package", false),
                text("ecosystem", false),
                strings("components"),
                text("kind", false),
                text("qualifier", true),
            ],

            _ => unreachable!("native evidence schema returned above"),
        };
        Arc::new(Schema::new(fields))
    }
    /// Build a native identity expression over the named contract fields.
    pub fn expression(self) -> Expr {
        self.expression_for(
            self.schema()
                .fields()
                .iter()
                .map(|field| col(field.name()))
                .collect(),
        )
    }
    /// Bind the declared fields to expressions supplied by a native normalization plan.
    pub fn bind(self, inputs: Vec<Expr>) -> Result<Expr> {
        if inputs.len() != self.schema().fields().len() {
            return Err(DataFusionError::Plan(
                "identity contract input arity".into(),
            ));
        }
        Ok(self.expression_for(inputs))
    }
    fn expression_for(self, inputs: Vec<Expr>) -> Expr {
        let fields = self.schema().fields().clone();
        let bytes = crate::native_identity::canonical_bytes(
            format!("enrichment/identity/8/{}", self.prefix()),
            fields,
        )
        .call(inputs);
        concat(vec![
            lit(format!("{}_", self.prefix())),
            encode(sha256(bytes), lit("hex")),
        ])
    }
    /// Mechanical single-record ingress, then DataFusion constant folding of the same graph.
    /// Corpus computation uses `expression`; this path never creates a private session/runtime.
    /// Derive identity from generated native fields without a Serde/JSON intermediate.
    pub fn record<T: crate::native_union::NativeStruct>(self, record: &T) -> Result<String> {
        self.batch_value(&T::batch(std::slice::from_ref(record))?)
    }
    pub fn value<T: serde::Serialize>(self, record: &T) -> Result<String> {
        let schema = self.schema();
        let mut decoder = arrow::json::ReaderBuilder::new(schema)
            .with_batch_size(1)
            .build_decoder()?;
        decoder.serialize(std::slice::from_ref(record))?;
        let batch: RecordBatch = decoder
            .flush()?
            .ok_or_else(|| DataFusionError::Internal("identity ingress has no row".into()))?;
        self.batch_value(&batch)
    }
    /// Derive a bounded boundary key from the authoritative mechanical Arrow fields.
    pub fn batch_value(self, batch: &RecordBatch) -> Result<String> {
        if batch.num_rows() != 1 {
            return Err(DataFusionError::Plan(
                "identity boundary requires exactly one row".into(),
            ));
        }
        let inputs = self
            .schema()
            .fields()
            .iter()
            .map(|field| {
                let column = batch.column_by_name(field.name()).ok_or_else(|| {
                    DataFusionError::Plan(format!("missing identity field {}", field.name()))
                })?;
                ScalarValue::try_from_array(column, 0).map(lit)
            })
            .collect::<Result<Vec<_>>>()?;
        let value = constant(self.expression_for(inputs))?;
        match value {
            Expr::Literal(
                ScalarValue::Utf8(Some(value)) | ScalarValue::Utf8View(Some(value)),
                _,
            ) => Ok(value),
            _ => Err(DataFusionError::Internal(
                "native identity failed to reduce to one text scalar".into(),
            )),
        }
    }
}

fn constant(expression: Expr) -> Result<Expr> {
    let context = SimplifyContext::builder()
        .with_schema(Arc::new(DFSchema::empty()))
        .build();
    ExprSimplifier::new(context).simplify(expression)
}

/// Mechanical one-value protocol boundary around the native ordered-set expression.
pub fn ordered_strings(values: &[String]) -> Result<Vec<String>> {
    use datafusion::functions_nested::expr_fn::{array_distinct, array_sort};
    let values = values
        .iter()
        .map(|value| ScalarValue::Utf8(Some(value.clone())))
        .collect::<Vec<_>>();
    let input = ScalarValue::List(ScalarValue::new_list(&values, &DataType::Utf8, false));
    let sorted = constant(array_sort(
        array_distinct(lit(input)),
        lit("ASC"),
        lit("NULLS FIRST"),
    ))?;
    let Expr::Literal(ScalarValue::List(values), _) = sorted else {
        return Err(DataFusionError::Internal(
            "native ordered set did not reduce".into(),
        ));
    };
    let values = values.value(0);
    let values = values
        .as_any()
        .downcast_ref::<arrow::array::StringArray>()
        .ok_or_else(|| DataFusionError::Internal("native ordered set type".into()))?;
    values
        .iter()
        .map(|value| {
            value
                .map(str::to_owned)
                .ok_or_else(|| DataFusionError::Internal("native ordered set contains null".into()))
        })
        .collect()
}
