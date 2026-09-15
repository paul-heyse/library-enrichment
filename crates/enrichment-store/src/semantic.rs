//! Native semantic-key projection, distinct and ordering with an incremental canonical hash.

use crate::{admission::Relation, projection, runtime::QueryRuntime};
use arrow::{array::StringArray, datatypes::DataType, record_batch::RecordBatch};
use datafusion::{
    dataframe::DataFrame,
    error::{DataFusionError, Result},
    logical_expr::{
        ColumnarValue, ScalarFunctionArgs, ScalarUDF, ScalarUDFImpl, Signature, Volatility,
    },
    prelude::col,
};
use enrichment_core::canonical::{self, StringArrayDigest};
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq, Hash)]
struct RecordKey {
    symbols: bool,
    signature: Signature,
}

impl ScalarUDFImpl for RecordKey {
    fn name(&self) -> &str {
        if self.symbols {
            "binding_semantic_key_v1"
        } else {
            "definition_semantic_key_v1"
        }
    }
    fn signature(&self) -> &Signature {
        &self.signature
    }
    fn return_type(&self, _: &[DataType]) -> Result<DataType> {
        Ok(DataType::Utf8)
    }
    fn return_field_from_args(
        &self,
        _args: datafusion::logical_expr::ReturnFieldArgs,
    ) -> Result<arrow::datatypes::FieldRef> {
        Ok(Arc::new(
            arrow::datatypes::Field::new(self.name(), DataType::Utf8, false).with_metadata(
                std::collections::HashMap::from([(
                    "enrichment.function".into(),
                    self.name().into(),
                )]),
            ),
        ))
    }
    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue> {
        let relation = if self.symbols {
            Relation::Symbols
        } else {
            Relation::Definitions
        };
        let columns = args
            .args
            .into_iter()
            .map(|arg| arg.into_array(args.number_rows))
            .collect::<Result<Vec<_>>>()?;
        let batch = RecordBatch::try_new(relation.schema()?, columns)?;
        let keys = if self.symbols {
            keys(&projection::decode::bindings(&batch)?)?
        } else {
            keys(&projection::decode::definitions(&batch)?)?
        };
        Ok(ColumnarValue::Array(Arc::new(StringArray::from(keys))))
    }
}

fn keys(rows: &[impl serde::Serialize]) -> Result<Vec<String>> {
    rows.iter()
        .map(|row| {
            serde_json::to_value(row)
                .map(|v| canonical::digest_hex(&v))
                .map_err(|e| DataFusionError::External(Box::new(e)))
        })
        .collect()
}

fn ordered_keys(relation: Relation, plan: DataFrame) -> Result<DataFrame> {
    let key = match relation {
        Relation::Definitions | Relation::Symbols => {
            let schema = relation.schema()?;
            ScalarUDF::from(RecordKey {
                symbols: relation == Relation::Symbols,
                signature: Signature::exact(
                    schema
                        .fields()
                        .iter()
                        .map(|f| f.data_type().clone())
                        .collect(),
                    Volatility::Immutable,
                ),
            })
            .call(schema.fields().iter().map(|f| col(f.name())).collect())
        }
        Relation::ProducerRuns => col("producer_binding_id"),
        _ => col(relation.key()),
    };
    plan.select(vec![key.alias("semantic_key")])?
        .distinct()?
        .sort(vec![col("semantic_key").sort(true, false)])
}

pub(crate) async fn digest(
    relation: Relation,
    plan: DataFrame,
    runtime: &QueryRuntime,
    max_rows: usize,
) -> Result<String> {
    let mut digest = StringArrayDigest::default();
    runtime
        .visit(ordered_keys(relation, plan)?, max_rows, |batch| {
            let keys = projection::TextColumn::new(batch.column(0).as_ref())?;
            for row in 0..batch.num_rows() {
                digest.push(keys.required(row)?)?;
            }
            Ok(())
        })
        .await?;
    Ok(digest.finish())
}
