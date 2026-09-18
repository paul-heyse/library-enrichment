//! Continuations over an immutable relation's unique, totally ordered identity.
use super::CursorError;
use crate::{identity::SnapshotId, native_key::Key, native_union::Rule};

pub const MAX_BYTES: usize = crate::native_cursor::MAX_BYTES;

crate::native_struct! {
    pub struct RowCursor {
        after: String => Rule::NonEmpty,
        snapshot: SnapshotId => Rule::Text,
        selection: String => Rule::NonEmpty,
        contract: String => Rule::NonEmpty,
        check: String => Rule::NonEmpty,
    }
}
crate::native_struct! {
    pub(crate) struct RowCursorBinding {
        snapshot: SnapshotId => Rule::Text,
        selection: String => Rule::NonEmpty,
        contract: String => Rule::NonEmpty,
        after: String => Rule::NonEmpty,
    }
}

impl RowCursor {
    fn contract() -> String {
        format!(
            "{};{}",
            crate::request::Operation::Inspect.contract_id(),
            crate::request::Operation::Overview.contract_id()
        )
    }
    pub fn encode(
        snapshot: &SnapshotId,
        selection: &str,
        after: String,
    ) -> Result<String, serde_json::Error> {
        let mut value = Self {
            after,
            snapshot: snapshot.clone(),
            selection: selection.into(),
            contract: Self::contract(),
            check: String::new(),
        };
        value.check = value.checksum();
        crate::native_cursor::Kind::Rows.encode(&value)
    }
    fn checksum(&self) -> String {
        Key::RowCursor
            .record(&RowCursorBinding {
                snapshot: self.snapshot.clone(),
                selection: self.selection.clone(),
                contract: self.contract.clone(),
                after: self.after.clone(),
            })
            .expect("declared native cursor identity")
    }
    pub fn decode(text: &str, snapshot: &SnapshotId, selection: &str) -> Result<Self, CursorError> {
        let value: Self = crate::native_cursor::Kind::Rows.decode(text)?;
        if value.contract != Self::contract()
            || value.checksum() != value.check
            || value.after.is_empty()
        {
            return Err(CursorError::Malformed);
        }
        if &value.snapshot != snapshot {
            return Err(CursorError::Mismatch {
                field: "selected snapshot",
            });
        }
        if value.selection != selection {
            return Err(CursorError::Mismatch {
                field: "aspect, subject or limits",
            });
        }
        Ok(value)
    }
}

crate::native_struct! {
    pub struct CursorInput {
        snapshot: SnapshotId => Rule::Text,
        selection: String => Rule::NonEmpty,
        after: Option<String> => Rule::Text,
    }
}

/// Bounded wire encoding only. Native plans choose the snapshot, selection and boundary.
pub fn encoder() -> datafusion::logical_expr::ScalarUDF {
    use crate::native_union::Cell;
    datafusion::logical_expr::ScalarUDF::from(EncodeCursor {
        signature: datafusion::logical_expr::Signature::exact(
            vec![CursorInput::data_type()],
            datafusion::logical_expr::Volatility::Immutable,
        ),
    })
}
pub(crate) fn is_encoder(function: &datafusion::logical_expr::ScalarUDF) -> bool {
    function.inner().downcast_ref::<EncodeCursor>().is_some()
}
#[derive(Debug, PartialEq, Eq, Hash)]
struct EncodeCursor {
    signature: datafusion::logical_expr::Signature,
}
impl datafusion::logical_expr::ScalarUDFImpl for EncodeCursor {
    fn name(&self) -> &str {
        "native_row_cursor_v10"
    }
    fn signature(&self) -> &datafusion::logical_expr::Signature {
        &self.signature
    }
    fn return_type(
        &self,
        _: &[arrow::datatypes::DataType],
    ) -> datafusion::common::Result<arrow::datatypes::DataType> {
        Ok(arrow::datatypes::DataType::Utf8)
    }
    fn return_field_from_args(
        &self,
        args: datafusion::logical_expr::ReturnFieldArgs,
    ) -> datafusion::common::Result<arrow::datatypes::FieldRef> {
        use crate::native_union::Cell;
        crate::native_schema::function_output(
            &args,
            &[std::sync::Arc::new(arrow::datatypes::Field::new(
                "input",
                CursorInput::data_type(),
                false,
            ))],
            self.name(),
            arrow::datatypes::DataType::Utf8,
            true,
        )
    }
    fn invoke_with_args(
        &self,
        args: datafusion::logical_expr::ScalarFunctionArgs,
    ) -> datafusion::common::Result<datafusion::logical_expr::ColumnarValue> {
        use crate::native_union::NativeStruct;
        use arrow::array::Array;
        crate::native_schema::function_call(self, &args)?;
        let input = args.args[0].to_array(args.number_rows)?;
        let input = datafusion::common::cast::as_struct_array(&input)?;
        if input.null_count() != 0 {
            return datafusion::common::exec_err!("cursor input missing");
        }
        for name in ["selection", "after"] {
            let values =
                datafusion::common::cast::as_string_array(input.column_by_name(name).ok_or_else(
                    || datafusion::common::exec_datafusion_err!("cursor input field missing"),
                )?)?;
            if values.iter().flatten().any(|value| value.len() > MAX_BYTES) {
                return datafusion::common::exec_err!("cursor input exceeds wire bound");
            }
        }
        let batch = arrow::record_batch::RecordBatch::from(input.clone());
        let rows = crate::evidence::arrow_model::cells::RowSet::batch(&batch)?;
        let mut output = arrow::array::StringBuilder::new();
        for index in 0..batch.num_rows() {
            let value = <CursorInput as NativeStruct>::decode(rows.row(index))?;
            let encoded = value
                .after
                .map(|after| RowCursor::encode(&value.snapshot, &value.selection, after))
                .transpose()
                .map_err(|error| datafusion::common::DataFusionError::External(Box::new(error)))?;
            output.append_option(encoded);
        }
        Ok(datafusion::logical_expr::ColumnarValue::Array(
            std::sync::Arc::new(output.finish()),
        ))
    }
}
