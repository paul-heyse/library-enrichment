//! Borrowed transport view over the admitted ResultRecord; no DTO or text copies.
use super::stream::{EnvelopeView, Failure, JsonValue};
use arrow::{
    array::{Array, AsArray, StructArray},
    datatypes::FieldRef,
};
use std::io::{self, Write};

pub(crate) enum Value<'a> {
    Owned(&'a dyn JsonValue),
    Arrow(ArrowValue<'a>),
    Text(&'a str),
    Null,
}
impl JsonValue for Value<'_> {
    fn write_json(&self, writer: &mut dyn Write) -> io::Result<()> {
        match self {
            Self::Owned(value) => value.write_json(writer),
            Self::Arrow(value) => value.write_json(writer),
            Self::Text(value) => value.write_json(writer),
            Self::Null => writer.write_all(b"null"),
        }
    }
}
#[derive(Clone, Copy)]
pub(crate) struct ArrowValue<'a> {
    field: &'a FieldRef,
    array: &'a dyn Array,
    row: usize,
}
impl JsonValue for ArrowValue<'_> {
    fn write_json(&self, writer: &mut dyn Write) -> io::Result<()> {
        crate::native_json::value(writer, self.field, self.array, self.row)
    }
}
fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}
impl<'a> ArrowValue<'a> {
    pub(crate) fn value(self) -> Value<'a> {
        Value::Arrow(self)
    }
    pub(crate) fn required(self) -> io::Result<Value<'a>> {
        if self.array.is_null(self.row) {
            return Err(invalid("required transport value is null"));
        }
        Ok(self.value())
    }
    fn structure(self) -> io::Result<&'a StructArray> {
        if self.array.is_null(self.row) {
            return Err(invalid("required transport record is null"));
        }
        self.array
            .as_struct_opt()
            .ok_or_else(|| invalid("transport record expected"))
    }
    pub(crate) fn child(self, name: &str) -> io::Result<Self> {
        let structure = self.structure()?;
        let (index, field) = structure
            .fields()
            .find(name)
            .ok_or_else(|| invalid("transport field missing"))?;
        Ok(Self {
            field,
            array: structure.column(index).as_ref(),
            row: self.row,
        })
    }
    fn optional(self) -> Option<Self> {
        (!self.array.is_null(self.row)).then_some(self)
    }
    fn text(self) -> io::Result<&'a str> {
        if self.array.is_null(self.row) {
            return Err(invalid("transport text is null"));
        }
        self.array
            .as_string_opt::<i32>()
            .map(|value| value.value(self.row))
            .ok_or_else(|| invalid("transport text expected"))
    }
    fn variant(self, tag: &str, expected: &str) -> io::Result<Option<Self>> {
        if self.child(tag)?.text()? != expected {
            return Ok(None);
        }
        self.child(expected).map(Some)
    }
}

pub(crate) struct NativeEnvelope<'a> {
    result: ArrowValue<'a>,
    request: &'a str,
}
impl<'a> NativeEnvelope<'a> {
    /// The UDF's complete field contract is checked once before invoking this row view.
    pub(crate) fn new(
        field: &'a FieldRef,
        array: &'a dyn Array,
        row: usize,
        request: &'a str,
    ) -> io::Result<Self> {
        if row >= array.len() || field.data_type() != array.data_type() || request.is_empty() {
            return Err(invalid("native envelope input contract"));
        }
        let value = Self {
            result: ArrowValue { field, array, row },
            request,
        };
        value.result.structure()?;
        Ok(value)
    }
    pub(crate) fn schema_version(&self) -> Value<'_> {
        Value::Owned(&crate::wire::envelope::SchemaVersion::Current)
    }
    pub(crate) fn request_value(&self) -> Value<'_> {
        Value::Text(self.request)
    }
    pub(crate) fn result(&self) -> ArrowValue<'a> {
        self.result
    }
    pub(crate) fn header(&self) -> io::Result<ArrowValue<'a>> {
        self.result.child("header")
    }
    pub(crate) fn outcome(&self) -> io::Result<ArrowValue<'a>> {
        self.header()?.child("outcome")
    }
    pub(crate) fn outcome_member(&self, name: &str) -> io::Result<Value<'a>> {
        let outcome = self.outcome()?;
        let status = outcome.child("status")?.text()?;
        if name == "error" && status != "error" {
            return Ok(Value::Null);
        }
        Ok(outcome.child(status)?.child(name)?.value())
    }
    fn read(delivery: ArrowValue<'a>) -> io::Result<Option<Value<'a>>> {
        delivery
            .variant("mode", "artifact")?
            .map(|value| value.child("read").map(ArrowValue::value))
            .transpose()
    }
}
impl JsonValue for NativeEnvelope<'_> {
    fn write_json(&self, writer: &mut dyn Write) -> io::Result<()> {
        crate::wire::envelope::write_native(self, writer)
    }
}
impl EnvelopeView for NativeEnvelope<'_> {
    fn request_id(&self) -> &str {
        self.request
    }
    fn artifact_id(&self) -> io::Result<Option<&str>> {
        self.result
            .child("delivery")?
            .variant("mode", "artifact")?
            .map(|value| value.child("artifact_id")?.text())
            .transpose()
    }
    fn failure(&self) -> io::Result<Option<Failure<'_>>> {
        let delivery = self.result.child("delivery")?;
        if let Some(error) = self.outcome()?.variant("status", "error")? {
            return Ok(Some(Failure {
                error: error.child("error")?.required()?,
                job_id: None,
                read: Self::read(delivery)?,
            }));
        }
        if let Some(job) = self.result.child("data")?.variant("tool", "job_control")?
            && let Some(result) = job.child("result")?.optional()
            && let Some(error) = result.child("outcome")?.variant("status", "error")?
        {
            return Ok(Some(Failure {
                error: error.child("error")?.required()?,
                job_id: Some(job.child("job_id")?.required()?),
                read: Self::read(result.child("delivery")?)?.or(Self::read(delivery)?),
            }));
        }
        Ok(None)
    }
}
