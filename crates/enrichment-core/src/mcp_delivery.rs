//! Native MCP projection and exact stdio sizing. The adapter supplies transport facts only.
use crate::{
    native_union::Rule,
    wire::{DeliveryDescriptor, Envelope, ErrorDetail, RecoveryAction, data::ToolData},
};
use serde::Serialize;
use std::io;

pub const SERVER_NAME: &str = "library-enrichment";
pub const SERVER_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const MAX_FRAMING_BYTES: u64 = 16 * 1024;

crate::native_vocabulary! {
    #[derive(Hash)]
    pub enum ProtocolEra { Classic = "classic", Modern = "modern" }
}
crate::native_union! { @tag "kind";
    /// The delivery boundary is part of the measurement, independent of the evidence result.
    #[derive(Default)]
    pub enum DeliveryProfile {
        #[default]
        Envelope = "envelope",
        McpStdio = "mcp_stdio" {
            era: ProtocolEra => Rule::Text,
            framing_bytes: u64 => Rule::UnsignedRange { min: 35, max: MAX_FRAMING_BYTES },
        },
        McpResourceStdio = "mcp_resource_stdio" {
            era: ProtocolEra => Rule::Text,
            framing_bytes: u64 => Rule::UnsignedRange { min: 35, max: MAX_FRAMING_BYTES },
            uri: String => Rule::NonEmpty,
        },
    }
}
impl DeliveryProfile {
    pub fn validate(&self) -> io::Result<()> {
        if let Self::McpStdio { framing_bytes, .. } | Self::McpResourceStdio { framing_bytes, .. } =
            self
            && !(35..=MAX_FRAMING_BYTES).contains(framing_bytes)
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "MCP framing outside admitted bounds",
            ));
        }
        if let Self::McpResourceStdio { uri, .. } = self
            && (!uri.starts_with("library-evidence://") || uri.len() > MAX_FRAMING_BYTES as usize)
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "resource URI outside admitted transport bounds",
            ));
        }
        Ok(())
    }

    /// Counting never aborts merely because a candidate will not fit. Native selection must
    /// still be able to choose another candidate or report the actual required minimum.
    pub fn measure(&self, value: &Envelope) -> io::Result<usize> {
        self.validate()?;
        match self {
            Self::Envelope => crate::canonical::serialized_size(value, usize::MAX),
            Self::McpStdio { era, framing_bytes } => {
                crate::canonical::serialized_size(&project(value, era)?, usize::MAX)?
                    .checked_add(*framing_bytes as usize)
                    .ok_or_else(|| io::Error::other("MCP measurement overflow"))
            }
            Self::McpResourceStdio {
                era,
                framing_bytes,
                uri,
            } => {
                crate::canonical::serialized_size(&project_resource(value, era, uri)?, usize::MAX)?
                    .checked_add(*framing_bytes as usize)
                    .ok_or_else(|| io::Error::other("resource measurement overflow"))
            }
        }
    }
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Content {
    Text {
        text: String,
    },
    ResourceLink {
        name: &'static str,
        uri: String,
        #[serde(rename = "mimeType")]
        mime_type: &'static str,
    },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CallResult<'a> {
    pub content: Vec<Content>,
    pub structured_content: &'a Envelope,
    pub is_error: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_type: Option<&'static str>,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<ServerMeta>,
}
#[derive(Serialize)]
pub struct ServerMeta {
    #[serde(rename = "io.modelcontextprotocol/serverInfo")]
    server_info: ServerIdentity,
}
#[derive(Serialize)]
pub struct ServerIdentity {
    name: &'static str,
    version: &'static str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceResult<'a> {
    contents: Vec<ResourceContent<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result_type: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_scope: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl_ms: Option<u32>,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    meta: Option<ServerMeta>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ResourceContent<'a> {
    uri: &'a str,
    mime_type: &'static str,
    text: String,
}

pub fn project_resource<'a>(
    value: &Envelope,
    era: &ProtocolEra,
    uri: &'a str,
) -> io::Result<ResourceResult<'a>> {
    let modern = *era == ProtocolEra::Modern;
    Ok(ResourceResult {
        contents: vec![ResourceContent {
            uri,
            mime_type: "application/json",
            text: serde_json::to_string(value)?,
        }],
        result_type: modern.then_some("complete"),
        cache_scope: modern.then_some("private"),
        ttl_ms: modern.then_some(0),
        meta: modern.then_some(ServerMeta {
            server_info: ServerIdentity {
                name: SERVER_NAME,
                version: SERVER_VERSION,
            },
        }),
    })
}

#[derive(Serialize)]
struct ErrorPreview<'a> {
    format: &'static str,
    request_id: &'a crate::wire::RequestId,
    error: &'a ErrorDetail,
    #[serde(skip_serializing_if = "Option::is_none")]
    job_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<&'a RecoveryAction>,
}
fn read(delivery: &DeliveryDescriptor) -> Option<&RecoveryAction> {
    match delivery {
        DeliveryDescriptor::Artifact { read, .. } => Some(read),
        _ => None,
    }
}

/// A format projection, also used by the native byte kernel. No content is trimmed to fit.
/// Recovery remains present for hosts that hide structuredContent when isError is true.
pub fn project<'a>(value: &'a Envelope, era: &ProtocolEra) -> io::Result<CallResult<'a>> {
    let terminal = match &value.data {
        ToolData::JobControl(job) => job.result.as_ref().and_then(|result| {
            if let crate::execution::TerminalOutcome::Error { error } = &result.outcome {
                Some((error, job.job_id.as_str(), &result.delivery))
            } else {
                None
            }
        }),
        _ => None,
    };
    let failure = value
        .error()
        .map(|error| (error, None, &value.delivery))
        .or_else(|| terminal.map(|(error, job, delivery)| (error, Some(job), delivery)));
    let mut content = Vec::new();
    if let Some((error, job_id, delivery)) = failure {
        content.push(Content::Text {
            text: serde_json::to_string(&ErrorPreview {
                format: "research-error-preview/2",
                request_id: &value.request_id,
                error,
                job_id,
                read: read(delivery).or_else(|| read(&value.delivery)),
            })?,
        });
    }
    if let DeliveryDescriptor::Artifact { artifact_id, .. } = &value.delivery {
        content.push(Content::ResourceLink {
            name: "Complete result",
            uri: format!("library-evidence://artifacts/{artifact_id}"),
            mime_type: "application/json",
        });
    }
    let modern = *era == ProtocolEra::Modern;
    Ok(CallResult {
        content,
        structured_content: value,
        is_error: failure.is_some(),
        result_type: modern.then_some("complete"),
        meta: modern.then_some(ServerMeta {
            server_info: ServerIdentity {
                name: SERVER_NAME,
                version: SERVER_VERSION,
            },
        }),
    })
}

/// Generated adapter facts; there is no independently maintained server/framing policy.
pub fn contract() -> serde_json::Value {
    serde_json::json!({
        "serializer": "fastmcp-4.0.3/mcp-2.2.0/stdio-1",
        "server_name": SERVER_NAME, "server_version": SERVER_VERSION,
        "max_framing_bytes": MAX_FRAMING_BYTES,
        "profile_schema": crate::native_wire::schema(schemars::schema_for!(DeliveryProfile).to_value()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oversized_candidate_is_measured_instead_of_aborting_selection() {
        let mut value: Envelope =
            serde_json::from_str(include_str!("../../../tests/fixtures/wire/ok.fixture.json"))
                .unwrap();
        value.summary = "x".repeat(crate::operation::results::MAX_BYTES as usize + 1);
        assert!(
            DeliveryProfile::Envelope.measure(&value).unwrap()
                > crate::operation::results::MAX_BYTES as usize
        );
    }

    #[test]
    fn framing_is_a_bounded_fact_and_failure_recovery_is_not_trimmed() {
        let value: Envelope = serde_json::from_str(include_str!(
            "../../../tests/fixtures/wire/error.fixture.json"
        ))
        .unwrap();
        for framing_bytes in [0, 34, MAX_FRAMING_BYTES + 1, u64::MAX] {
            assert!(
                DeliveryProfile::McpStdio {
                    era: ProtocolEra::Classic,
                    framing_bytes
                }
                .measure(&value)
                .is_err()
            );
        }
        let result = project(&value, &ProtocolEra::Modern).unwrap();
        assert!(result.is_error);
        let Content::Text { text } = &result.content[0] else {
            panic!("mandatory recovery")
        };
        let preview: serde_json::Value = serde_json::from_str(text).unwrap();
        assert_eq!(
            preview["error"],
            serde_json::to_value(value.error().unwrap()).unwrap()
        );
        assert_eq!(preview["request_id"], value.request_id.as_str());
        assert!(preview.get("details_omitted").is_none());
    }
}
