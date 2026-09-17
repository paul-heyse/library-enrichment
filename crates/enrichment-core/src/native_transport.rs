//! Exact transport-size kernel. Native plans select candidates; this UDF only measures the
//! same mechanical envelope encoding as the transport, without building an intermediate JSON.
use crate::{
    native_union::{Cell, NativeStruct, Rule},
    operation::results::ResultRecord,
};
use arrow::{
    array::{Array, AsArray, UInt64Array},
    datatypes::{DataType, FieldRef},
};
use datafusion::{
    common::Result,
    execution::memory_pool::{MemoryConsumer, MemoryPool},
    logical_expr::{
        ColumnarValue, Expr, ReturnFieldArgs, ScalarFunctionArgs, ScalarUDF, ScalarUDFImpl,
        Signature, Volatility,
    },
    prelude::lit,
};
use std::{
    fmt,
    hash::{Hash, Hasher},
    sync::Arc,
};

pub fn delivery(
    pool: Arc<dyn MemoryPool>,
    result: Expr,
    request: &crate::wire::RequestId,
    profile: crate::mcp_delivery::DeliveryProfile,
) -> Expr {
    ScalarUDF::from(DeliveryBytes {
        signature: Signature::exact(
            vec![ResultRecord::data_type(), DataType::Utf8],
            Volatility::Immutable,
        ),
        pool,
        profile,
    })
    .call(vec![result, lit(request.as_str())])
}

struct DeliveryBytes {
    signature: Signature,
    pool: Arc<dyn MemoryPool>,
    profile: crate::mcp_delivery::DeliveryProfile,
}
impl fmt::Debug for DeliveryBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeliveryBytes")
            .field("codec", &"native-json/3")
            .field("profile", &self.profile)
            .finish()
    }
}
impl PartialEq for DeliveryBytes {
    fn eq(&self, other: &Self) -> bool {
        self.signature == other.signature && self.profile == other.profile
    }
}
impl Eq for DeliveryBytes {}
impl Hash for DeliveryBytes {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.signature.hash(state);
        // A typed transport profile is part of the concrete immutable function identity.
        std::mem::discriminant(&self.profile).hash(state);
        use crate::mcp_delivery::DeliveryProfile;
        match &self.profile {
            DeliveryProfile::Envelope => {}
            DeliveryProfile::McpStdio { era, framing_bytes } => {
                era.hash(state);
                framing_bytes.hash(state);
            }
            DeliveryProfile::McpResourceStdio {
                era,
                framing_bytes,
                uri,
            } => {
                era.hash(state);
                framing_bytes.hash(state);
                uri.hash(state);
            }
        }
    }
}

impl ScalarUDFImpl for DeliveryBytes {
    fn name(&self) -> &str {
        "native_delivery_bytes_v1"
    }
    fn signature(&self) -> &Signature {
        &self.signature
    }
    fn return_type(&self, _: &[DataType]) -> Result<DataType> {
        Ok(DataType::UInt64)
    }
    fn return_field_from_args(&self, args: ReturnFieldArgs) -> Result<FieldRef> {
        crate::native_schema::function_arguments(
            args.arg_fields,
            &[
                Arc::new(crate::native_union::field::<ResultRecord>(
                    "result",
                    Rule::Text,
                )),
                Arc::new(arrow::datatypes::Field::new(
                    "request_id",
                    DataType::Utf8,
                    false,
                )),
            ],
        )?;
        if args.scalar_arguments.len() != 2 {
            return datafusion::common::plan_err!("envelope measurement arity");
        }
        Ok(Arc::new(crate::native_union::field::<u64>(
            "bytes",
            Rule::Text,
        )))
    }
    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue> {
        crate::native_schema::function_call(self, &args)?;
        let input_bytes = args.args.iter().fold(0usize, |bytes, value| {
            bytes.saturating_add(match value {
                ColumnarValue::Array(value) => value.get_array_memory_size(),
                ColumnarValue::Scalar(value) => value.size().saturating_mul(args.number_rows),
            })
        });
        let allocation = input_bytes
            .saturating_mul(8)
            .saturating_add(args.number_rows.saturating_mul(8));
        let reservation = MemoryConsumer::new("native_delivery_measurement").register(&self.pool);
        reservation.try_grow(allocation)?;
        let record = args.args[0].clone().into_array(args.number_rows)?;
        let request = args.args[1].clone().into_array(args.number_rows)?;
        let records = record.as_struct_opt().ok_or_else(|| {
            datafusion::common::exec_datafusion_err!("envelope measurement record")
        })?;
        let request = request.as_string_opt::<i32>().ok_or_else(|| {
            datafusion::common::exec_datafusion_err!("envelope measurement request")
        })?;
        let batch = arrow::record_batch::RecordBatch::from(records.clone());
        let rows = crate::evidence::arrow_model::cells::RowSet::batch(&batch)?;
        let mut lengths = Vec::with_capacity(args.number_rows);
        for row in 0..args.number_rows {
            if records.is_null(row) || request.is_null(row) {
                return datafusion::common::exec_err!(
                    "envelope measurement requires present values"
                );
            }
            let request = request.value(row).to_owned().try_into().map_err(
                |error: crate::wire::ids::RequestIdError| {
                    datafusion::common::DataFusionError::External(Box::new(error))
                },
            )?;
            let record = <ResultRecord as NativeStruct>::decode(rows.row(row))?;
            lengths.push(self.profile.measure(&record.into_envelope(request))? as u64);
        }
        Ok(ColumnarValue::Array(Arc::new(UInt64Array::from(lengths))))
    }
}

/// Concrete format kernel capability; function names never authorize semantic erasure.
pub fn is_measurement(function: &ScalarUDF) -> bool {
    function.inner().downcast_ref::<DeliveryBytes>().is_some()
}
