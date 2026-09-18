//! Native language-server consumer construction from the selected symbol and command.
use crate::{
    evidence::execution::{SemanticMethod, Utf8Position},
    identity::Ecosystem,
    native_union::{Cell, NativeStruct, Rule},
    request::InspectionOptions,
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

crate::native_struct! {
    pub struct Consumer {
        text: String => Rule::Text,
        position: Option<Utf8Position> => Rule::Text,
        uri: String => Rule::DocumentUri,
        methods: Vec<SemanticMethod> => Rule::SequenceBounds { min: 1, max: SemanticMethod::VALUES.len() as u64 },
    }
}
crate::native_struct! {
    pub struct Input {
        path: String => Rule::NonEmpty,
        name: String => Rule::NonEmpty,
        ecosystem: Ecosystem => Rule::Text,
        options: InspectionOptions => Rule::Text,
    }
}

crate::native_vocabulary! { pub enum PositionEncoding { Utf8 = "utf-8", Utf16 = "utf-16" } }
crate::native_struct! {
    pub struct Utf16Position {
        line: u32 => Rule::Coordinate(crate::native_union::Unit::LineZeroBased),
        code_unit: u32 => Rule::Coordinate(crate::native_union::Unit::Utf16CodeUnit),
    }
}
crate::native_union! {
    pub enum ProtocolPosition {
        Utf8 = "utf8" { value: Utf8Position => Rule::Text },
        Utf16 = "utf16" { value: Utf16Position => Rule::Text },
    }
}
crate::native_struct! {
    pub struct PositionInput {
        text: String => Rule::Text,
        position: Option<Utf8Position> => Rule::Text,
        encoding: PositionEncoding => Rule::Text,
    }
}
crate::native_struct! {
    pub struct Scope {
        grant_id: crate::identity::GrantId => Rule::Text,
        context_id: crate::identity::ContextId => Rule::Text,
        snapshot_id: crate::identity::SnapshotId => Rule::Text,
        environment_id: crate::identity::EnvironmentId => Rule::Text,
        symbol_id: String => Rule::Reference(crate::native_union::Domain::Symbol),
        consumer: Consumer => Rule::Text,
    }
}
crate::native_struct! {
    pub struct Conversation {
        scope: Scope => Rule::Text,
        process_operation_id: crate::identity::ProcessOperationId => Rule::Text,
        process_effect_id: crate::identity::ProcessEffectId => Rule::Text,
        encoding: PositionEncoding => Rule::Text,
        position: Option<ProtocolPosition> => Rule::Text,
    }
}

pub fn protocol_position() -> ScalarUDF {
    ScalarUDF::from(PositionFunction {
        signature: Signature::exact(vec![PositionInput::data_type()], Volatility::Immutable),
    })
}
#[derive(Debug, PartialEq, Eq, Hash)]
struct PositionFunction {
    signature: Signature,
}
impl ScalarUDFImpl for PositionFunction {
    fn name(&self) -> &str {
        "semantic_protocol_position"
    }
    fn signature(&self) -> &Signature {
        &self.signature
    }
    fn return_type(&self, _: &[DataType]) -> Result<DataType> {
        Ok(ProtocolPosition::data_type())
    }
    fn return_field_from_args(&self, args: ReturnFieldArgs) -> Result<FieldRef> {
        if args.arg_fields.len() != 1
            || args.arg_fields[0].data_type() != &PositionInput::data_type()
            || args.arg_fields[0].is_nullable()
        {
            return datafusion::common::plan_err!("semantic position input contract");
        }
        Ok(Arc::new(Field::new(
            self.name(),
            ProtocolPosition::data_type(),
            true,
        )))
    }
    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue> {
        crate::native_schema::function_call(self, &args)?;
        let array = args.args[0].clone().into_array(args.number_rows)?;
        let structure = datafusion::common::cast::as_struct_array(&array)?;
        if structure.null_count() != 0 {
            return datafusion::common::exec_err!("semantic position input missing");
        }
        let batch = RecordBatch::from(structure.clone());
        let rows = crate::evidence::arrow_model::cells::RowSet::batch(&batch)?;
        let output = (0..batch.num_rows())
            .map(|row| {
                let input = <PositionInput as NativeStruct>::decode(rows.row(row))?;
                let Some(position) = input.position else {
                    return Ok(None);
                };
                position
                    .validate(&input.text)
                    .map_err(datafusion::error::DataFusionError::Execution)?;
                Ok(Some(match input.encoding {
                    PositionEncoding::Utf8 => ProtocolPosition::Utf8 { value: position },
                    PositionEncoding::Utf16 => {
                        let line = crate::evidence::text::line(&input.text, position.line)
                            .map_err(datafusion::error::DataFusionError::Execution)?;
                        let code_unit =
                            u32::try_from(line[..position.byte as usize].encode_utf16().count())
                                .map_err(|e| {
                                    datafusion::error::DataFusionError::External(Box::new(e))
                                })?;
                        ProtocolPosition::Utf16 {
                            value: Utf16Position {
                                line: position.line,
                                code_unit,
                            },
                        }
                    }
                }))
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(ColumnarValue::Array(ProtocolPosition::encode(
            &output.iter().map(Option::as_ref).collect::<Vec<_>>(),
        )?))
    }
}

pub const DEFAULT_METHODS: &[SemanticMethod] = &[
    SemanticMethod::Hover,
    SemanticMethod::Definition,
    SemanticMethod::References,
    SemanticMethod::Diagnostics,
];

pub fn methods(options: &InspectionOptions) -> Vec<SemanticMethod> {
    if options.methods.is_empty() {
        DEFAULT_METHODS.to_vec()
    } else {
        options.methods.clone()
    }
}

impl Input {
    fn consumer(&self) -> std::result::Result<Consumer, String> {
        let options = &self.options;
        let ecosystem = self.ecosystem;
        let needs_position = options.methods.is_empty()
            || options
                .methods
                .iter()
                .any(|m| *m != crate::evidence::execution::SemanticMethod::Diagnostics);
        let (text, generated) = match &options.snippet {
            Some(text) => (text.clone(), false),
            None => {
                let path = &self.path;
                let valid = path
                    .split(if ecosystem == Ecosystem::Rust {
                        "::"
                    } else {
                        "."
                    })
                    .all(|part| {
                        let part = part.strip_prefix("r#").unwrap_or(part);
                        let mut chars = part.chars();
                        chars.next().is_some_and(|c| c == '_' || c.is_alphabetic())
                            && chars.all(|c| c == '_' || c.is_alphanumeric())
                    });
                if !valid {
                    return Err("this qualified path needs an explicit consumer snippet; no source expression was guessed".into());
                }
                (
                    match ecosystem {
                        Ecosystem::Rust => {
                            format!("use {path} as _libenr_target;\nfn main() {{}}\n")
                        }
                        Ecosystem::Python => format!(
                            "import {}\n{path}\n",
                            path.split('.').next().ok_or("missing import root")?
                        ),
                    },
                    true,
                )
            }
        };
        let position = if !needs_position {
            None
        } else if let Some(position) = options.position {
            position.validate(&text)?;
            Some(position)
        } else {
            let matches: Vec<_> = text
                .match_indices(&self.name)
                .filter(|(offset, name)| {
                    let identifier = |c: char| c == '_' || c.is_alphanumeric();
                    text[..*offset]
                        .chars()
                        .next_back()
                        .is_none_or(|c| !identifier(c))
                        && text[*offset + name.len()..]
                            .chars()
                            .next()
                            .is_none_or(|c| !identifier(c))
                })
                .collect();
            let offset = if generated {
                matches.last().map(|(offset, _)| *offset)
            } else if matches.len() == 1 {
                Some(matches[0].0)
            } else { None }.ok_or("consumer anchor is ambiguous or absent; provide one exact UTF-8 position or request diagnostics only")?;
            let (line, byte) = crate::evidence::text::position(&text, offset)?;
            Some(Utf8Position { line, byte })
        };
        Ok(Consumer {
            methods: methods(options),
            text,
            position,
            uri: match ecosystem {
                Ecosystem::Rust => "file:///capsule/src/main.rs".into(),
                Ecosystem::Python => "file:///capsule/consumer.py".into(),
            },
        })
    }
}

/// Finite source-language construction is an immutable native function; admission,
/// symbol selection and command ownership remain relational consumers.
pub fn consumer() -> ScalarUDF {
    ScalarUDF::from(ConsumerFunction {
        signature: Signature::exact(vec![Input::data_type()], Volatility::Immutable),
    })
}
#[derive(Debug, PartialEq, Eq, Hash)]
struct ConsumerFunction {
    signature: Signature,
}
impl ScalarUDFImpl for ConsumerFunction {
    fn name(&self) -> &str {
        "semantic_consumer"
    }
    fn signature(&self) -> &Signature {
        &self.signature
    }
    fn return_type(&self, _: &[DataType]) -> Result<DataType> {
        Ok(Consumer::data_type())
    }
    fn return_field_from_args(&self, args: ReturnFieldArgs) -> Result<FieldRef> {
        if args.arg_fields.len() != 1
            || args.arg_fields[0].data_type() != &Input::data_type()
            || args.arg_fields[0].is_nullable()
        {
            return datafusion::common::plan_err!("semantic consumer input contract");
        }
        Ok(Arc::new(Field::new(
            self.name(),
            Consumer::data_type(),
            false,
        )))
    }
    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue> {
        crate::native_schema::function_call(self, &args)?;
        let array = args.args[0].clone().into_array(args.number_rows)?;
        let structure = datafusion::common::cast::as_struct_array(&array)?;
        if structure.null_count() != 0 {
            return datafusion::common::exec_err!("semantic consumer input missing");
        }
        let batch = RecordBatch::from(structure.clone());
        let rows = crate::evidence::arrow_model::cells::RowSet::batch(&batch)?;
        let output = (0..batch.num_rows())
            .map(|row| {
                let input = <Input as NativeStruct>::decode(rows.row(row))?;
                input
                    .consumer()
                    .map_err(datafusion::error::DataFusionError::Execution)
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(ColumnarValue::Array(<Consumer as NativeStruct>::encode(
            &output.iter().map(Some).collect::<Vec<_>>(),
        )?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use datafusion::prelude::{SessionContext, col};
    crate::native_struct! { struct Positioned { value: Option<ProtocolPosition> => Rule::Text } }

    async fn positions(inputs: &[PositionInput]) -> Result<Vec<Positioned>> {
        let session = SessionContext::new();
        let frame = session.read_batch(PositionInput::batch(inputs)?)?;
        let input = crate::evidence::arrow_model::expressions::record(
            &PositionInput::data_type(),
            &PositionInput::fields()
                .iter()
                .map(|field| (field.name().as_str(), col(field.name())))
                .collect::<Vec<_>>(),
        )?;
        let batches = frame
            .select(vec![protocol_position().call(vec![input]).alias("value")])?
            .collect()
            .await?;
        let mut result = Vec::new();
        for batch in batches {
            let rows = crate::evidence::arrow_model::cells::RowSet::batch(&batch)?;
            for row in 0..batch.num_rows() {
                result.push(<Positioned as NativeStruct>::decode(rows.row(row))?);
            }
        }
        Ok(result)
    }

    #[tokio::test]
    async fn plan19_protocol_coordinates_preserve_encoding_and_character_boundaries() -> Result<()>
    {
        let text = "a\r\r\n🌎é\r\n";
        let mut input = Vec::new();
        let mut expected = Vec::new();
        for (byte, units) in [(0, 0), (4, 2), (6, 3)] {
            let point = Utf8Position { line: 2, byte };
            for encoding in [PositionEncoding::Utf8, PositionEncoding::Utf16] {
                input.push(PositionInput {
                    text: text.into(),
                    position: Some(point),
                    encoding,
                });
                expected.push(Some(match encoding {
                    PositionEncoding::Utf8 => ProtocolPosition::Utf8 { value: point },
                    PositionEncoding::Utf16 => ProtocolPosition::Utf16 {
                        value: Utf16Position {
                            line: 2,
                            code_unit: units,
                        },
                    },
                }));
            }
        }
        input.push(PositionInput {
            text: text.into(),
            position: None,
            encoding: PositionEncoding::Utf16,
        });
        expected.push(None);
        assert_eq!(
            positions(&input)
                .await?
                .into_iter()
                .map(|v| v.value)
                .collect::<Vec<_>>(),
            expected
        );
        for (line, byte) in [(2, 1), (2, 2), (2, 3), (2, 5), (2, 7), (0, 2), (4, 0)] {
            assert!(
                positions(&[PositionInput {
                    text: text.into(),
                    position: Some(Utf8Position { line, byte }),
                    encoding: PositionEncoding::Utf16
                }])
                .await
                .is_err(),
                "invalid coordinate {line}:{byte}"
            );
        }
        Ok(())
    }
}
