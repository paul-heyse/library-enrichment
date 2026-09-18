//! The external runtime helper's bounded JSON is decoded once at the native value boundary.
//! Selection, process outcome and publication rules are DataFusion expressions around it.
use crate::{
    evidence::{
        execution::{ExecutionPayload, RuntimeObject},
        relational::SubjectRef,
    },
    execution::ProcessObservation,
    native_union::{Cell, NativeStruct, Rule},
    request::RuntimeSelection,
};
use arrow::{
    array::Array,
    datatypes::{DataType, Field, FieldRef},
    record_batch::RecordBatch,
};
use datafusion::{
    common::Result,
    logical_expr::{
        ColumnarValue, ReturnFieldArgs, ScalarFunctionArgs, ScalarUDF, ScalarUDFImpl, Signature,
        Volatility,
    },
};
use std::sync::Arc;

crate::native_struct! { pub struct Capture {
    selection: RuntimeSelection => Rule::Text,
    subject: SubjectRef => Rule::Text,
    observation: ProcessObservation => Rule::Text,
} }
crate::native_struct! { pub struct ReportInput {
    report: Option<String> => Rule::Text,
    subject: SubjectRef => Rule::Text,
} }

crate::native_struct! {
    pub struct Report {
        result: RuntimeObject => Rule::Text,
        stdout: String => Rule::Text,
        stderr: String => Rule::Text,
        output_truncated: bool => Rule::Text,
    }
}

pub fn report() -> ScalarUDF {
    ScalarUDF::from(DecodeReport {
        signature: Signature::exact(vec![ReportInput::data_type()], Volatility::Immutable),
    })
}

pub(crate) fn is_decoder(function: &ScalarUDF) -> bool {
    function.inner().downcast_ref::<DecodeReport>().is_some()
        || function.inner().downcast_ref::<EncodeSelection>().is_some()
}
#[derive(Debug, PartialEq, Eq, Hash)]
struct DecodeReport {
    signature: Signature,
}
impl ScalarUDFImpl for DecodeReport {
    fn name(&self) -> &str {
        "runtime_object_report"
    }
    fn signature(&self) -> &Signature {
        &self.signature
    }
    fn return_type(&self, _: &[DataType]) -> Result<DataType> {
        Ok(Report::data_type())
    }
    fn return_field_from_args(&self, args: ReturnFieldArgs) -> Result<FieldRef> {
        crate::native_schema::function_output(
            &args,
            &[Arc::new(Field::new(
                "input",
                ReportInput::data_type(),
                false,
            ))],
            self.name(),
            Report::data_type(),
            true,
        )
    }
    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue> {
        crate::native_schema::function_call(self, &args)?;
        let input = args.args[0].clone().into_array(args.number_rows)?;
        let input = datafusion::common::cast::as_struct_array(&input)?;
        if input.null_count() != 0 {
            return datafusion::common::exec_err!("runtime report input missing");
        }
        let batch = RecordBatch::from(input.clone());
        let rows = crate::evidence::arrow_model::cells::RowSet::batch(&batch)?;
        let mut output = Vec::with_capacity(batch.num_rows());
        for row in 0..batch.num_rows() {
            let input = <ReportInput as NativeStruct>::decode(rows.row(row))?;
            let value = if let Some(report) = input.report {
                if report.len() > 1_048_576 {
                    return datafusion::common::exec_err!(
                        "runtime report exceeds its 1 MiB decode bound"
                    );
                }
                let report: Report = serde_json::from_str(&report).map_err(|error| {
                    datafusion::error::DataFusionError::Execution(format!(
                        "runtime producer returned invalid bounded JSON: {error}"
                    ))
                })?;
                ExecutionPayload::RuntimeObject(report.result.clone())
                    .validate(&input.subject)
                    .map_err(datafusion::error::DataFusionError::Execution)?;
                Some(report)
            } else {
                None
            };
            output.push(value);
        }
        Ok(ColumnarValue::Array(<Report as NativeStruct>::encode(
            &output.iter().map(Option::as_ref).collect::<Vec<_>>(),
        )?))
    }
}

crate::native_struct! { pub struct SelectionTransport {
    selection: RuntimeSelection => Rule::Text,
    max_output_bytes: u64 => Rule::UnsignedRange { min: 1024, max: 1_048_576 },
} }

pub fn selection_transport() -> ScalarUDF {
    ScalarUDF::from(EncodeSelection {
        signature: Signature::exact(vec![SelectionTransport::data_type()], Volatility::Immutable),
    })
}
#[derive(Debug, PartialEq, Eq, Hash)]
struct EncodeSelection {
    signature: Signature,
}
impl ScalarUDFImpl for EncodeSelection {
    fn name(&self) -> &str {
        "runtime_selection_transport"
    }
    fn signature(&self) -> &Signature {
        &self.signature
    }
    fn return_type(&self, _: &[DataType]) -> Result<DataType> {
        Ok(DataType::Utf8)
    }
    fn return_field_from_args(&self, args: ReturnFieldArgs) -> Result<FieldRef> {
        crate::native_schema::function_output(
            &args,
            &[Arc::new(Field::new(
                "input",
                SelectionTransport::data_type(),
                false,
            ))],
            self.name(),
            DataType::Utf8,
            false,
        )
    }
    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue> {
        crate::native_schema::function_call(self, &args)?;
        let input = args.args[0].clone().into_array(args.number_rows)?;
        let input = datafusion::common::cast::as_struct_array(&input)?;
        if input.null_count() != 0 {
            return datafusion::common::exec_err!("runtime selection transport input missing");
        }
        let batch = RecordBatch::from(input.clone());
        let rows = crate::evidence::arrow_model::cells::RowSet::batch(&batch)?;
        let mut values = Vec::with_capacity(batch.num_rows());
        for row in 0..batch.num_rows() {
            let input = <SelectionTransport as NativeStruct>::decode(rows.row(row))?;
            if input.selection.module.len() > 4096
                || input.selection.attributes.len() > 256
                || input
                    .selection
                    .attributes
                    .iter()
                    .any(|part| part.len() > 4096)
            {
                return datafusion::common::exec_err!("runtime selection transport exceeds bounds");
            }
            let value = serde_json::to_value(&input)
                .map_err(|e| datafusion::common::DataFusionError::Execution(e.to_string()))?;
            values.push(crate::canonical::to_canonical_string(&value));
        }
        Ok(ColumnarValue::Array(Arc::new(
            arrow::array::StringArray::from(values),
        )))
    }
}
