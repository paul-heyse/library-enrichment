//! Streaming format mechanics shared by native measurement and the RPC projection.
use std::io::{self, Write};

pub(crate) trait JsonValue {
    fn write_json(&self, writer: &mut dyn Write) -> io::Result<()>;
}
impl<T: serde::Serialize> JsonValue for T {
    fn write_json(&self, writer: &mut dyn Write) -> io::Result<()> {
        serde_json::to_writer(writer, self).map_err(io::Error::from)
    }
}
pub(crate) struct Object<'a> {
    writer: &'a mut dyn Write,
    first: bool,
}
impl<'a> Object<'a> {
    pub(crate) fn new(writer: &'a mut dyn Write) -> io::Result<Self> {
        writer.write_all(b"{")?;
        Ok(Self {
            writer,
            first: true,
        })
    }
    pub(crate) fn member(&mut self, name: &str, value: &dyn JsonValue) -> io::Result<()> {
        if !self.first {
            self.writer.write_all(b",")?;
        }
        self.first = false;
        name.write_json(self.writer)?;
        self.writer.write_all(b":")?;
        value.write_json(self.writer)
    }
    pub(crate) fn finish(self) -> io::Result<()> {
        self.writer.write_all(b"}")
    }
}

/// Escape an encoded JSON document directly into a JSON string. Byte scanning is safe here:
/// only ASCII bytes need escaping and every other UTF-8 byte is forwarded unchanged. This
/// works even if an upstream writer splits a Unicode scalar over several writes.
pub(crate) struct JsonText<'a>(pub(crate) &'a dyn JsonValue);
impl JsonValue for JsonText<'_> {
    fn write_json(&self, writer: &mut dyn Write) -> io::Result<()> {
        writer.write_all(b"\"")?;
        self.0.write_json(&mut Escape(writer))?;
        writer.write_all(b"\"")
    }
}
struct Escape<'a>(&'a mut dyn Write);
impl Write for Escape<'_> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let mut start = 0;
        for (index, byte) in bytes.iter().copied().enumerate() {
            if byte >= 0x20 && byte != b'"' && byte != b'\\' {
                continue;
            }
            self.0.write_all(&bytes[start..index])?;
            match byte {
                b'"' => self.0.write_all(b"\\\"")?,
                b'\\' => self.0.write_all(b"\\\\")?,
                b'\x08' => self.0.write_all(b"\\b")?,
                b'\t' => self.0.write_all(b"\\t")?,
                b'\n' => self.0.write_all(b"\\n")?,
                b'\x0c' => self.0.write_all(b"\\f")?,
                b'\r' => self.0.write_all(b"\\r")?,
                other => {
                    const HEX: &[u8; 16] = b"0123456789abcdef";
                    self.0.write_all(&[
                        b'\\',
                        b'u',
                        b'0',
                        b'0',
                        HEX[usize::from(other >> 4)],
                        HEX[usize::from(other & 15)],
                    ])?;
                }
            }
            start = index + 1;
        }
        self.0.write_all(&bytes[start..])?;
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

pub(crate) struct Failure<'a> {
    pub(crate) error: super::native::Value<'a>,
    pub(crate) job_id: Option<super::native::Value<'a>>,
    pub(crate) read: Option<super::native::Value<'a>>,
}
pub(crate) trait EnvelopeView: JsonValue {
    fn request_id(&self) -> &str;
    fn failure(&self) -> io::Result<Option<Failure<'_>>>;
    fn artifact_id(&self) -> io::Result<Option<&str>>;
}
struct ErrorPreview<'a> {
    request: &'a str,
    failure: &'a Failure<'a>,
}
impl JsonValue for ErrorPreview<'_> {
    fn write_json(&self, writer: &mut dyn Write) -> io::Result<()> {
        let mut object = Object::new(writer)?;
        object.member("format", &"research-error-preview/2")?;
        object.member("request_id", &self.request)?;
        object.member("error", &self.failure.error)?;
        if let Some(job) = &self.failure.job_id {
            object.member("job_id", job)?;
        }
        if let Some(read) = &self.failure.read {
            object.member("read", read)?;
        }
        object.finish()
    }
}
struct Content<'a, V: EnvelopeView> {
    value: &'a V,
    failure: Option<Failure<'a>>,
}
impl<V: EnvelopeView> JsonValue for Content<'_, V> {
    fn write_json(&self, writer: &mut dyn Write) -> io::Result<()> {
        writer.write_all(b"[")?;
        if let Some(failure) = &self.failure {
            let mut object = Object::new(writer)?;
            object.member("type", &"text")?;
            object.member(
                "text",
                &JsonText(&ErrorPreview {
                    request: self.value.request_id(),
                    failure,
                }),
            )?;
            object.finish()?;
        }
        if let Some(artifact) = self.value.artifact_id()? {
            if self.failure.is_some() {
                writer.write_all(b",")?;
            }
            let mut object = Object::new(writer)?;
            object.member("type", &"resource_link")?;
            object.member("name", &"Complete result")?;
            object.member("uri", &ArtifactUri(artifact))?;
            object.member("mimeType", &"application/json")?;
            object.finish()?;
        }
        writer.write_all(b"]")
    }
}
struct ArtifactUri<'a>(&'a str);
impl JsonValue for ArtifactUri<'_> {
    fn write_json(&self, writer: &mut dyn Write) -> io::Result<()> {
        writer.write_all(b"\"library-evidence://artifacts/")?;
        Escape(writer).write_all(self.0.as_bytes())?;
        writer.write_all(b"\"")
    }
}
struct Resource<'a, V: EnvelopeView> {
    value: &'a V,
    uri: &'a str,
}
impl<V: EnvelopeView> JsonValue for Resource<'_, V> {
    fn write_json(&self, writer: &mut dyn Write) -> io::Result<()> {
        writer.write_all(b"[")?;
        let mut object = Object::new(writer)?;
        object.member("uri", &self.uri)?;
        object.member("mimeType", &"application/json")?;
        object.member("text", &JsonText(self.value))?;
        object.finish()?;
        writer.write_all(b"]")
    }
}

pub(crate) fn write<V: EnvelopeView>(
    profile: &super::DeliveryProfile,
    value: &V,
    writer: &mut dyn Write,
) -> io::Result<()> {
    use super::{DeliveryProfile, ProtocolEra};
    profile.validate()?;
    let era = match profile {
        DeliveryProfile::Envelope => return value.write_json(writer),
        DeliveryProfile::McpStdio { era, .. } | DeliveryProfile::McpResourceStdio { era, .. } => {
            era
        }
    };
    let mut object = Object::new(writer)?;
    match profile {
        DeliveryProfile::Envelope => unreachable!(),
        DeliveryProfile::McpStdio { .. } => {
            let failure = value.failure()?;
            let is_error = failure.is_some();
            object.member("content", &Content { value, failure })?;
            object.member("structuredContent", value)?;
            object.member("isError", &is_error)?;
        }
        DeliveryProfile::McpResourceStdio { uri, .. } => {
            object.member("contents", &Resource { value, uri })?;
        }
    }
    if *era == ProtocolEra::Modern {
        object.member("resultType", &"complete")?;
        if matches!(profile, DeliveryProfile::McpResourceStdio { .. }) {
            object.member("cacheScope", &"private")?;
            object.member("ttlMs", &0u32)?;
        }
        object.member(
            "_meta",
            &super::ServerMeta {
                server_info: super::ServerIdentity {
                    name: super::SERVER_NAME,
                    version: super::SERVER_VERSION,
                },
            },
        )?;
    }
    object.finish()
}

pub(crate) fn measure<V: EnvelopeView>(
    profile: &super::DeliveryProfile,
    value: &V,
) -> io::Result<usize> {
    let mut counter = crate::native_json::BoundedWriter::new(io::sink(), usize::MAX);
    write(profile, value, &mut counter)?;
    counter
        .written()
        .checked_add(profile.framing_bytes())
        .ok_or_else(|| io::Error::other("delivery measurement overflow"))
}

impl EnvelopeView for crate::wire::Envelope {
    fn request_id(&self) -> &str {
        self.request_id.as_str()
    }
    fn artifact_id(&self) -> io::Result<Option<&str>> {
        Ok(match &self.delivery {
            crate::wire::DeliveryDescriptor::Artifact { artifact_id, .. } => Some(artifact_id),
            _ => None,
        })
    }
    fn failure(&self) -> io::Result<Option<Failure<'_>>> {
        use super::native::Value;
        fn read(delivery: &crate::wire::DeliveryDescriptor) -> Option<Value<'_>> {
            match delivery {
                crate::wire::DeliveryDescriptor::Artifact { read, .. } => Some(Value::Owned(read)),
                _ => None,
            }
        }
        if let Some(error) = self.error() {
            return Ok(Some(Failure {
                error: Value::Owned(error),
                job_id: None,
                read: read(&self.delivery),
            }));
        }
        if let crate::wire::data::ToolData::JobControl(job) = &self.data
            && let Some(result) = &job.result
            && let crate::execution::TerminalOutcome::Error { error } = &result.outcome
        {
            return Ok(Some(Failure {
                error: Value::Owned(error),
                job_id: Some(Value::Owned(&job.job_id)),
                read: read(&result.delivery).or_else(|| read(&self.delivery)),
            }));
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nested_json_text_matches_serde_for_every_control_byte_and_split_unicode() {
        struct Split;
        impl JsonValue for Split {
            fn write_json(&self, writer: &mut dyn Write) -> io::Result<()> {
                for byte in "é🦀\"\\\n"
                    .as_bytes()
                    .iter()
                    .chain((0..32u8).collect::<Vec<_>>().iter())
                {
                    writer.write_all(&[*byte])?;
                }
                Ok(())
            }
        }
        let mut encoded = Vec::new();
        JsonText(&Split).write_json(&mut encoded).unwrap();
        let mut source = Vec::new();
        Split.write_json(&mut source).unwrap();
        assert_eq!(
            encoded,
            serde_json::to_vec(&String::from_utf8(source).unwrap()).unwrap()
        );
    }
}
