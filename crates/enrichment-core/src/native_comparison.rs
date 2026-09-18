//! Bounded Arrow field-fact lowering. The kernel emits paths and exact native value bytes;
//! DataFusion owns equality, set difference and ordering. No JSON or equality decisions here.
use crate::{
    compare::{ComparisonPathStep, ComparisonValue},
    native_union::{Cell, Rule, Unit},
};
use arrow::{
    array::{Array, ArrayRef, AsArray},
    datatypes::{DataType, Field, FieldRef},
};
use datafusion::{
    common::Result,
    execution::memory_pool::{MemoryConsumer, MemoryPool},
    logical_expr::{
        ColumnarValue, ReturnFieldArgs, ScalarFunctionArgs, ScalarUDF, ScalarUDFImpl, Signature,
        Volatility,
    },
};
use std::{
    fmt,
    hash::{Hash, Hasher},
    sync::Arc,
};

const MAX_FACTS: usize = crate::compare::MAX_COMPARISON_FIELDS;
const MAX_BYTES: usize = 64 * 1024 * 1024;
const WORKING_BYTES: usize = 512 * 1024 * 1024;
crate::native_struct! { pub struct FieldFact {
    steps:Vec<ComparisonPathStep> => Rule::SequenceBounds {min:0,max:crate::compare::MAX_FIELD_PATH_STEPS as u64},
    value:crate::native_bytes::NativeBytes => Rule::Text,
} }

pub fn fields(pool: Arc<dyn MemoryPool>) -> ScalarUDF {
    ScalarUDF::from(Fields {
        signature: Signature::exact(vec![ComparisonValue::data_type()], Volatility::Immutable),
        pool,
    })
}
pub(crate) fn is_projection(function: &ScalarUDF) -> bool {
    function.inner().downcast_ref::<Fields>().is_some()
}
struct Fields {
    signature: Signature,
    pool: Arc<dyn MemoryPool>,
}
impl fmt::Debug for Fields {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ComparisonFields")
            .field("signature", &self.signature)
            .finish()
    }
}
impl PartialEq for Fields {
    fn eq(&self, other: &Self) -> bool {
        self.signature == other.signature
    }
}
impl Eq for Fields {}
impl Hash for Fields {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.signature.hash(state);
    }
}
impl ScalarUDFImpl for Fields {
    fn name(&self) -> &str {
        "native_comparison_fields_v1"
    }
    fn signature(&self) -> &Signature {
        &self.signature
    }
    fn return_type(&self, _: &[DataType]) -> Result<DataType> {
        datafusion::common::plan_err!("comparison fields require full field contract")
    }
    fn return_field_from_args(&self, args: ReturnFieldArgs) -> Result<FieldRef> {
        crate::native_schema::function_output(
            &args,
            &[Arc::new(crate::native_union::field::<ComparisonValue>(
                "value",
                Rule::Text,
            ))],
            self.name(),
            <Vec<FieldFact>>::data_type(),
            false,
        )
    }
    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue> {
        crate::native_schema::function_call(self, &args)?;
        // Admit the declared temporary envelope before row/path/codec allocations. Output
        // arrays then pass through the runtime's normal native stream/result accounting.
        let memory = MemoryConsumer::new("comparison-field-lowering").register(&self.pool);
        memory.try_grow(WORKING_BYTES)?;
        let array = args.args[0].clone().into_array(args.number_rows)?;
        let field = crate::native_union::field::<ComparisonValue>("value", Rule::Text);
        let mut output = Vec::with_capacity(args.number_rows);
        let mut budget = Budget { facts: 0, bytes: 0 };
        for row in 0..args.number_rows {
            let mut facts = Vec::new();
            visit(
                &field,
                array.as_ref(),
                row,
                &mut Vec::new(),
                &mut facts,
                &mut budget,
            )?;
            output.push(facts);
        }
        let values =
            <Vec<FieldFact> as Cell>::encode(&output.iter().map(Some).collect::<Vec<_>>())?;
        Ok(ColumnarValue::Array(values))
    }
}
struct Budget {
    facts: usize,
    bytes: usize,
}
fn visit(
    field: &Field,
    array: &dyn Array,
    row: usize,
    path: &mut Vec<ComparisonPathStep>,
    facts: &mut Vec<FieldFact>,
    budget: &mut Budget,
) -> Result<()> {
    if path.len() > crate::compare::MAX_FIELD_PATH_STEPS || budget.facts >= MAX_FACTS {
        return datafusion::common::resources_err!("comparison field fact count/depth bound");
    }
    let value = crate::native_identity::field_value(&field.clone().with_name("value"), array, row)?;
    let path_bytes = path
        .iter()
        .map(|step| match step {
            ComparisonPathStep::Field { name } => name.len() + 64,
            ComparisonPathStep::Item { .. } => 64,
        })
        .sum::<usize>();
    let bytes = value
        .len()
        .checked_add(path_bytes)
        .and_then(|n| n.checked_add(128))
        .and_then(|n| budget.bytes.checked_add(n));
    if bytes.is_none_or(|n| n > MAX_BYTES) {
        return datafusion::common::resources_err!("comparison field fact byte bound");
    }
    budget.bytes = bytes.expect("checked field bytes");
    budget.facts += 1;
    facts.push(FieldFact {
        steps: path.clone(),
        value: crate::native_bytes::NativeBytes::new(value)
            .map_err(datafusion::common::DataFusionError::Execution)?,
    });
    if array.is_null(row) {
        return Ok(());
    }
    match field.data_type() {
        DataType::Struct(fields) => {
            let structure = array.as_struct();
            for (index, child) in fields.iter().enumerate() {
                path.push(ComparisonPathStep::Field {
                    name: child.name().clone(),
                });
                visit(
                    child,
                    structure.column(index).as_ref(),
                    row,
                    path,
                    facts,
                    budget,
                )?;
                path.pop();
            }
        }
        DataType::List(item) | DataType::LargeList(item) | DataType::FixedSizeList(item, _) => {
            if let DataType::Struct(fields) = item.data_type()
                && fields.find("ordinal").is_some_and(|(_, field)| {
                    field.data_type() == &DataType::UInt32
                        && field
                            .metadata()
                            .get("enrichment.rule")
                            .is_some_and(|encoded| {
                                serde_json::from_str::<Rule>(encoded).is_ok_and(|rule| {
                                    matches!(rule, Rule::Coordinate(Unit::Ordinal))
                                })
                            })
                })
            {
                let values: ArrayRef = match field.data_type() {
                    DataType::List(_) => array.as_list::<i32>().value(row),
                    DataType::LargeList(_) => array.as_list::<i64>().value(row),
                    DataType::FixedSizeList(_, _) => array.as_fixed_size_list().value(row),
                    _ => unreachable!(),
                };
                let ordinals = values
                    .as_struct()
                    .column_by_name("ordinal")
                    .expect("declared ordinal")
                    .as_primitive::<arrow::datatypes::UInt32Type>();
                for index in 0..values.len() {
                    if values.is_null(index) || ordinals.is_null(index) {
                        return datafusion::common::exec_err!(
                            "comparison sequence item has no declared ordinal"
                        );
                    }
                    path.push(ComparisonPathStep::Item {
                        ordinal: ordinals.value(index),
                    });
                    visit(item, values.as_ref(), index, path, facts, budget)?;
                    path.pop();
                }
            }
        }
        _ => {}
    }
    Ok(())
}
