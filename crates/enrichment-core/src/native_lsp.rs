//! Bounded LSP format decoding and explicit protocol-coordinate kernels.
//! No filesystem, server execution or evidence policy is permitted in these functions.
use crate::{
    evidence::execution::{ExecutionDiagnostic, SemanticMethod, Utf8Position, Utf8Range},
    native_semantics::{PositionEncoding, ProtocolPosition, Utf16Position},
    native_union::{Cell, NativeStruct, Rule},
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
use serde_json::Value;
use std::sync::Arc;

crate::native_struct! { pub struct ProtocolRange {
    start: ProtocolPosition => Rule::Text,
    end: ProtocolPosition => Rule::Text,
} }
crate::native_struct! { pub struct Location {
    uri: String => Rule::DocumentUri,
    range: ProtocolRange => Rule::Text,
} }
crate::native_struct! { pub struct Input {
    method: SemanticMethod => Rule::Text,
    answer: String => Rule::Text,
    document: String => Rule::Text,
    encoding: PositionEncoding => Rule::Text,
} }
crate::native_vocabulary! { pub enum ResponseKind { Value = "value", Null = "null", Unsupported = "unsupported" } }
crate::native_struct! { pub struct Decoded {
    kind: ResponseKind => Rule::Text,
    hover: Option<String> => Rule::Text,
    locations: Vec<Location> => Rule::SequenceBounds {min:0,max:256},
    diagnostics: Vec<ExecutionDiagnostic> => Rule::SequenceBounds {min:0,max:256},
    issue: Option<String> => Rule::Text,
} }
crate::native_struct! { pub struct RangeInput {
    document: String => Rule::Text,
    range: ProtocolRange => Rule::Text,
} }
crate::native_struct! { pub struct Utf8RangeInput {
    document: String => Rule::Text,
    range: Utf8Range => Rule::Text,
} }

crate::native_struct! { pub struct UriInput { uri: String => Rule::DocumentUri } }
crate::native_struct! { pub struct FileUri { path: String => Rule::NonEmpty } }
fn file_uri(input: UriInput) -> Result<FileUri> {
    if input.uri.len() > 16_384 {
        return datafusion::common::exec_err!("semantic location URI exceeds bound");
    }
    let uri = url::Url::parse(&input.uri)
        .map_err(|e| datafusion::common::DataFusionError::Execution(e.to_string()))?;
    if uri.query().is_some() || uri.fragment().is_some() {
        return datafusion::common::exec_err!("semantic file URI has a query or fragment");
    }
    let path = uri.to_file_path().map_err(|()| {
        datafusion::common::DataFusionError::Execution(
            "semantic location is not a local file URI".into(),
        )
    })?;
    let path = path.into_os_string().into_string().map_err(|_| {
        datafusion::common::DataFusionError::Execution("semantic location is not UTF-8".into())
    })?;
    if path.len() > 4096 || path.contains(['\0', '\n', '\r']) {
        return datafusion::common::exec_err!("semantic file path exceeds bound or has controls");
    }
    Ok(FileUri { path })
}

fn position(
    value: &Value,
    encoding: PositionEncoding,
) -> std::result::Result<ProtocolPosition, String> {
    let coordinate = |name: &str| {
        value[name]
            .as_u64()
            .and_then(|n| u32::try_from(n).ok())
            .ok_or_else(|| format!("invalid LSP {name} coordinate"))
    };
    let line = coordinate("line")?;
    let character = coordinate("character")?;
    Ok(match encoding {
        PositionEncoding::Utf8 => ProtocolPosition::Utf8 {
            value: Utf8Position {
                line,
                byte: character,
            },
        },
        PositionEncoding::Utf16 => ProtocolPosition::Utf16 {
            value: Utf16Position {
                line,
                code_unit: character,
            },
        },
    })
}
fn protocol_range(
    value: &Value,
    encoding: PositionEncoding,
) -> std::result::Result<ProtocolRange, String> {
    let range = ProtocolRange {
        start: position(&value["start"], encoding)?,
        end: position(&value["end"], encoding)?,
    };
    let axes = |p: &ProtocolPosition| match p {
        ProtocolPosition::Utf8 { value } => (value.line, value.byte),
        ProtocolPosition::Utf16 { value } => (value.line, value.code_unit),
    };
    if axes(&range.end) < axes(&range.start) {
        return Err("reversed LSP range".into());
    }
    Ok(range)
}
fn byte_position(
    document: &str,
    position: &ProtocolPosition,
) -> std::result::Result<Utf8Position, String> {
    match position {
        ProtocolPosition::Utf8 { value } => {
            value.validate(document)?;
            Ok(*value)
        }
        ProtocolPosition::Utf16 { value } => {
            let line = crate::evidence::text::line(document, value.line)?;
            let mut units = 0u32;
            for (byte, ch) in line
                .char_indices()
                .map(|(b, c)| (b, Some(c)))
                .chain(std::iter::once((line.len(), None)))
            {
                if units == value.code_unit {
                    return Ok(Utf8Position {
                        line: value.line,
                        byte: u32::try_from(byte).map_err(|e| e.to_string())?,
                    });
                }
                units = units
                    .checked_add(ch.map_or(0, |c| c.len_utf16() as u32))
                    .ok_or("LSP coordinate overflow")?;
            }
            Err("LSP position is outside the line or splits a character".into())
        }
    }
}
fn range(input: &RangeInput) -> std::result::Result<Utf8Range, String> {
    if input.document.len() > 1024 * 1024 {
        return Err("semantic source exceeds 1 MiB coordinate bound".into());
    }
    byte_range(&input.document, &input.range)
}
fn byte_range(document: &str, range: &ProtocolRange) -> std::result::Result<Utf8Range, String> {
    let value = Utf8Range {
        start: byte_position(document, &range.start)?,
        end: byte_position(document, &range.end)?,
    };
    value.validate()?;
    Ok(value)
}
fn markup(value: &Value, depth: usize) -> std::result::Result<String, String> {
    if depth > 4 {
        return Err("hover markup nesting exceeds bound".into());
    }
    let text = match value {
        Value::String(value) => value.clone(),
        Value::Array(values) => values
            .iter()
            .map(|v| markup(v, depth + 1))
            .collect::<std::result::Result<Vec<_>, _>>()?
            .join("\n"),
        Value::Object(_) => value["value"]
            .as_str()
            .ok_or("invalid hover markup")?
            .to_owned(),
        _ => return Err("invalid hover contents".into()),
    };
    if text.len() > 32 * 1024 {
        return Err("hover text exceeds its retained byte budget".into());
    }
    Ok(text)
}
fn decode(input: &Input) -> Decoded {
    let mut output = Decoded {
        kind: ResponseKind::Value,
        hover: None,
        locations: vec![],
        diagnostics: vec![],
        issue: None,
    };
    let parsed = (|| -> std::result::Result<(), String> {
        if input.answer.len() > 256 * 1024 || input.document.len() > 1024 * 1024 {
            return Err("semantic response or source exceeds its decode bound".into());
        }
        let value: Value = serde_json::from_str(&input.answer)
            .map_err(|e| format!("invalid LSP response: {e}"))?;
        if value.is_null() {
            output.kind = ResponseKind::Null;
            return Ok(());
        }
        if value["kind"] == "unsupported" {
            output.kind = ResponseKind::Unsupported;
            return Ok(());
        }
        match input.method {
            SemanticMethod::Hover => {
                let text = markup(
                    value.get("contents").ok_or("hover result lacks contents")?,
                    0,
                )?;
                output.hover = (!text.is_empty()).then_some(text);
            }
            SemanticMethod::Diagnostics => {
                if value["kind"] != "full" {
                    return Err("diagnostic result is not a resolved full report".into());
                }
                let items = value["items"]
                    .as_array()
                    .ok_or("diagnostic items missing")?;
                if items.len() > 256 {
                    return Err("diagnostic result exceeds 256 items".into());
                }
                for item in items {
                    let range = byte_range(
                        &input.document,
                        &protocol_range(&item["range"], input.encoding)?,
                    )?;
                    let severity = item
                        .get("severity")
                        .map(|v| {
                            v.as_u64()
                                .and_then(|n| u32::try_from(n).ok())
                                .filter(|n| (1..=4).contains(n))
                                .ok_or("invalid diagnostic severity")
                        })
                        .transpose()?;
                    let code = match item.get("code") {
                        None | Some(Value::Null) => None,
                        Some(Value::String(v)) => Some(v.clone()),
                        Some(Value::Number(v)) => Some(v.to_string()),
                        _ => return Err("invalid diagnostic code".into()),
                    };
                    let source = item
                        .get("source")
                        .map(|v| {
                            v.as_str()
                                .map(str::to_owned)
                                .ok_or("invalid diagnostic source")
                        })
                        .transpose()?;
                    output.diagnostics.push(ExecutionDiagnostic {
                        range,
                        severity,
                        code,
                        source,
                        message: item["message"]
                            .as_str()
                            .ok_or("diagnostic message missing")?
                            .to_owned(),
                    });
                }
            }
            SemanticMethod::Definition
            | SemanticMethod::Implementation
            | SemanticMethod::References => {
                let items = value
                    .as_array()
                    .map_or_else(|| std::slice::from_ref(&value), Vec::as_slice);
                if items.len() > 256 {
                    return Err("location result exceeds 256 items".into());
                }
                for item in items {
                    let (uri, span) = if let Some(uri) = item.get("uri") {
                        (uri, item.get("range"))
                    } else {
                        (
                            &item["targetUri"],
                            item.get("targetSelectionRange")
                                .or_else(|| item.get("targetRange")),
                        )
                    };
                    output.locations.push(Location {
                        uri: uri.as_str().ok_or("invalid LSP location URI")?.into(),
                        range: protocol_range(
                            span.ok_or("LSP location range missing")?,
                            input.encoding,
                        )?,
                    });
                }
            }
        }
        Ok(())
    })();
    output.issue = parsed.err();
    output
}

pub fn decoder() -> ScalarUDF {
    ScalarUDF::from(Function {
        kind: Kind::Decode,
        signature: Signature::exact(vec![Input::data_type()], Volatility::Immutable),
    })
}
pub fn coordinates() -> ScalarUDF {
    ScalarUDF::from(Function {
        kind: Kind::Range,
        signature: Signature::exact(vec![RangeInput::data_type()], Volatility::Immutable),
    })
}
pub fn utf8_coordinates() -> ScalarUDF {
    ScalarUDF::from(Function {
        kind: Kind::Utf8Range,
        signature: Signature::exact(vec![Utf8RangeInput::data_type()], Volatility::Immutable),
    })
}
pub fn uri() -> ScalarUDF {
    ScalarUDF::from(Function {
        kind: Kind::Uri,
        signature: Signature::exact(vec![UriInput::data_type()], Volatility::Immutable),
    })
}
pub(crate) fn is_kernel(function: &ScalarUDF) -> bool {
    function.inner().downcast_ref::<Function>().is_some()
}
#[derive(Debug, PartialEq, Eq, Hash)]
enum Kind {
    Decode,
    Range,
    Utf8Range,
    Uri,
}
#[derive(Debug, PartialEq, Eq, Hash)]
struct Function {
    kind: Kind,
    signature: Signature,
}
impl ScalarUDFImpl for Function {
    fn name(&self) -> &str {
        match self.kind {
            Kind::Decode => "semantic_response",
            Kind::Range => "semantic_response_range",
            Kind::Utf8Range => "semantic_utf8_range",
            Kind::Uri => "semantic_file_uri",
        }
    }
    fn signature(&self) -> &Signature {
        &self.signature
    }
    fn return_type(&self, _: &[DataType]) -> Result<DataType> {
        Ok(match self.kind {
            Kind::Decode => Decoded::data_type(),
            Kind::Range | Kind::Utf8Range => Utf8Range::data_type(),
            Kind::Uri => FileUri::data_type(),
        })
    }
    fn return_field_from_args(&self, args: ReturnFieldArgs) -> Result<FieldRef> {
        let input = match self.kind {
            Kind::Decode => Input::data_type(),
            Kind::Range => RangeInput::data_type(),
            Kind::Utf8Range => Utf8RangeInput::data_type(),
            Kind::Uri => UriInput::data_type(),
        };
        crate::native_schema::function_output(
            &args,
            &[Arc::new(Field::new("input", input, false))],
            self.name(),
            self.return_type(&[])?,
            false,
        )
    }
    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue> {
        crate::native_schema::function_call(self, &args)?;
        let input = args.args[0].clone().into_array(args.number_rows)?;
        let input = datafusion::common::cast::as_struct_array(&input)?;
        if input.null_count() != 0 {
            return datafusion::common::exec_err!("semantic response input missing");
        }
        let batch = RecordBatch::from(input.clone());
        let rows = crate::evidence::arrow_model::cells::RowSet::batch(&batch)?;
        let output = match self.kind {
            Kind::Decode => {
                let values = (0..batch.num_rows())
                    .map(|row| {
                        <Input as NativeStruct>::decode(rows.row(row)).map(|input| decode(&input))
                    })
                    .collect::<std::result::Result<Vec<_>, _>>()?;
                <Decoded as NativeStruct>::encode(&values.iter().map(Some).collect::<Vec<_>>())?
            }
            Kind::Uri => {
                let values = (0..batch.num_rows())
                    .map(|row| file_uri(<UriInput as NativeStruct>::decode(rows.row(row))?))
                    .collect::<Result<Vec<_>>>()?;
                <FileUri as NativeStruct>::encode(&values.iter().map(Some).collect::<Vec<_>>())?
            }
            Kind::Range => {
                let values = (0..batch.num_rows())
                    .map(|row| {
                        range(&<RangeInput as NativeStruct>::decode(rows.row(row))?)
                            .map_err(datafusion::error::DataFusionError::Execution)
                    })
                    .collect::<Result<Vec<_>>>()?;
                <Utf8Range as NativeStruct>::encode(&values.iter().map(Some).collect::<Vec<_>>())?
            }
            Kind::Utf8Range => {
                let values = (0..batch.num_rows())
                    .map(|row| {
                        let input = <Utf8RangeInput as NativeStruct>::decode(rows.row(row))?;
                        if input.document.len() > 1024 * 1024 {
                            return datafusion::common::exec_err!(
                                "semantic source exceeds 1 MiB coordinate bound"
                            );
                        }
                        input
                            .range
                            .validate()
                            .and_then(|()| input.range.start.validate(&input.document))
                            .and_then(|()| input.range.end.validate(&input.document))
                            .map_err(datafusion::error::DataFusionError::Execution)?;
                        Ok(input.range)
                    })
                    .collect::<Result<Vec<_>>>()?;
                <Utf8Range as NativeStruct>::encode(&values.iter().map(Some).collect::<Vec<_>>())?
            }
        };
        Ok(ColumnarValue::Array(output))
    }
}
