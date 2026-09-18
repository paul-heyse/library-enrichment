//! Exact transport-size kernel. Native plans select candidates; this UDF only measures the
//! same mechanical envelope encoding as the transport, without building an intermediate JSON.
use crate::{
    native_union::{Cell, Rule},
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
        // Both scalar and array inputs are borrowed, including scalar structs; never
        // expand one large scalar into N copied candidate results.
        let allocation = args
            .number_rows
            .checked_mul(std::mem::size_of::<u64>())
            .ok_or_else(|| {
                datafusion::common::exec_datafusion_err!("delivery output allocation overflow")
            })?;
        let reservation = MemoryConsumer::new("native_delivery_measurement").register(&self.pool);
        reservation.try_grow(allocation)?;
        let (record, scalar): (&dyn Array, bool) = match &args.args[0] {
            ColumnarValue::Array(value) => (value.as_ref(), false),
            ColumnarValue::Scalar(datafusion::common::ScalarValue::Struct(value)) => {
                (value.as_ref(), true)
            }
            _ => return datafusion::common::exec_err!("delivery requires a native result record"),
        };
        if record.len() != if scalar { 1 } else { args.number_rows } {
            return datafusion::common::exec_err!("delivery record length mismatch");
        }
        let mut lengths = Vec::with_capacity(args.number_rows);
        for row in 0..args.number_rows {
            let request = match &args.args[1] {
                ColumnarValue::Scalar(datafusion::common::ScalarValue::Utf8(Some(value))) => {
                    value.as_str()
                }
                ColumnarValue::Array(value) => {
                    let values = value.as_string_opt::<i32>().ok_or_else(|| {
                        datafusion::common::exec_datafusion_err!("delivery request must be UTF-8")
                    })?;
                    if values.len() != args.number_rows || values.is_null(row) {
                        return datafusion::common::exec_err!(
                            "delivery request length/presence mismatch"
                        );
                    }
                    values.value(row)
                }
                _ => return datafusion::common::exec_err!("delivery request must be present"),
            };
            let value = crate::mcp_delivery::native::NativeEnvelope::new(
                &args.arg_fields[0],
                record,
                if scalar { 0 } else { row },
                request,
            )?;
            lengths.push(crate::mcp_delivery::stream::measure(&self.profile, &value)? as u64);
        }
        Ok(ColumnarValue::Array(Arc::new(UInt64Array::from(lengths))))
    }
}

/// Concrete format kernel capability; function names never authorize semantic erasure.
pub fn is_measurement(function: &ScalarUDF) -> bool {
    function.inner().downcast_ref::<DeliveryBytes>().is_some()
        || function.inner().downcast_ref::<ValueBytes>().is_some()
}

/// Measure an exact borrowed Arrow value in the final JSON codec. Policy (inline versus
/// artifact, page prefix and budget) belongs to the calling native plan.
pub fn value_bytes(pool: Arc<dyn MemoryPool>, field: FieldRef, limit: usize) -> ScalarUDF {
    ScalarUDF::from(ValueBytes {
        signature: Signature::exact(vec![field.data_type().clone()], Volatility::Immutable),
        field,
        pool,
        limit,
    })
}
#[derive(Debug)]
struct ValueBytes {
    signature: Signature,
    field: FieldRef,
    pool: Arc<dyn MemoryPool>,
    limit: usize,
}
impl PartialEq for ValueBytes {
    fn eq(&self, other: &Self) -> bool {
        self.signature == other.signature && self.field == other.field && self.limit == other.limit
    }
}
impl Eq for ValueBytes {}
impl Hash for ValueBytes {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.signature.hash(state);
        self.field.hash(state);
        self.limit.hash(state);
    }
}
impl ScalarUDFImpl for ValueBytes {
    fn name(&self) -> &str {
        "native_json_value_bytes_v1"
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
            std::slice::from_ref(&self.field),
        )?;
        Ok(Arc::new(crate::native_union::field::<u64>(
            "bytes",
            Rule::Text,
        )))
    }
    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue> {
        crate::native_schema::function_call(self, &args)?;
        let conversion = match &args.args[0] {
            ColumnarValue::Array(_) => 0,
            ColumnarValue::Scalar(value) => {
                value.size().checked_mul(args.number_rows).ok_or_else(|| {
                    datafusion::common::exec_datafusion_err!(
                        "value measurement scalar size overflow"
                    )
                })?
            }
        };
        let bytes = args
            .number_rows
            .checked_mul(std::mem::size_of::<u64>())
            .and_then(|v| v.checked_add(conversion))
            .ok_or_else(|| {
                datafusion::common::exec_datafusion_err!("value measurement size overflow")
            })?;
        let memory = MemoryConsumer::new("native_value_measurement").register(&self.pool);
        memory.try_grow(bytes)?;
        let array = args.args[0].clone().into_array(args.number_rows)?;
        let mut lengths = Vec::with_capacity(args.number_rows);
        for row in 0..args.number_rows {
            lengths.push(crate::native_json::write_value(
                std::io::sink(),
                self.limit,
                &self.field,
                array.as_ref(),
                row,
            )? as u64);
        }
        Ok(ColumnarValue::Array(Arc::new(UInt64Array::from(lengths))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::native_union::NativeStruct;
    use datafusion::{common::ScalarValue, execution::memory_pool::GreedyMemoryPool};

    #[test]
    fn borrowed_size_kernel_does_not_expand_scalar_results_or_reserve_input_copies() {
        let mut envelope: crate::wire::Envelope = serde_json::from_str(include_str!(
            "../../../tests/fixtures/wire/error.fixture.json"
        ))
        .unwrap();
        envelope.summary = "é\"🦀\n".repeat(200_000);
        let record = ResultRecord::from_envelope(&envelope).unwrap();
        let array = <ResultRecord as NativeStruct>::encode(&[Some(&record)]).unwrap();
        let struct_array = Arc::new(array.as_struct().clone());
        // This pool pays only for fixed-width output lengths. The already-owned large
        // input remains borrowed for both array and scalar calling conventions.
        let pool: Arc<dyn MemoryPool> = Arc::new(GreedyMemoryPool::new(128));
        for profile in [
            crate::mcp_delivery::DeliveryProfile::Envelope,
            crate::mcp_delivery::DeliveryProfile::McpStdio {
                era: crate::mcp_delivery::ProtocolEra::Modern,
                framing_bytes: 35,
            },
            crate::mcp_delivery::DeliveryProfile::McpResourceStdio {
                era: crate::mcp_delivery::ProtocolEra::Modern,
                framing_bytes: 35,
                uri: "library-evidence://jobs/job_fixture".into(),
            },
        ] {
            let udf = DeliveryBytes {
                signature: Signature::exact(
                    vec![ResultRecord::data_type(), DataType::Utf8],
                    Volatility::Immutable,
                ),
                pool: pool.clone(),
                profile: profile.clone(),
            };
            for (record, rows) in [
                (ColumnarValue::Array(array.clone()), 1),
                (
                    ColumnarValue::Scalar(ScalarValue::Struct(struct_array.clone())),
                    8,
                ),
            ] {
                let result = udf
                    .invoke_with_args(ScalarFunctionArgs {
                        args: vec![
                            record,
                            ColumnarValue::Scalar(ScalarValue::Utf8(Some(
                                envelope.request_id.as_str().into(),
                            ))),
                        ],
                        arg_fields: vec![
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
                        number_rows: rows,
                        return_field: Arc::new(crate::native_union::field::<u64>(
                            "bytes",
                            Rule::Text,
                        )),
                        config_options: Default::default(),
                    })
                    .unwrap();
                let result = result.into_array(rows).unwrap();
                let values = result.as_primitive::<arrow::datatypes::UInt64Type>();
                assert_eq!(
                    values.values().as_ref(),
                    vec![profile.measure(&envelope).unwrap() as u64; rows]
                );
                assert_eq!(pool.reserved(), 0);
            }
        }
    }
}
