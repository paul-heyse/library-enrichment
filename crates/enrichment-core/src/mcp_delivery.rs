//! Native MCP projection and exact stdio sizing. The adapter supplies transport facts only.
use crate::{native_union::Rule, wire::Envelope};
use serde::Serialize;
use std::io;
pub(crate) mod native;
pub(crate) mod stream;

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
        stream::measure(self, value)
    }

    pub(crate) fn framing_bytes(&self) -> usize {
        match self {
            Self::Envelope => 0,
            Self::McpStdio { framing_bytes, .. } | Self::McpResourceStdio { framing_bytes, .. } => {
                *framing_bytes as usize
            }
        }
    }

    /// The final RPC format boundary. Its exact buffer reservation follows the
    /// selected bytes through transport; no JSON Value tree is reconstructed.
    pub fn project(
        &self,
        value: &Envelope,
        pool: &std::sync::Arc<dyn datafusion::execution::memory_pool::MemoryPool>,
    ) -> io::Result<crate::json_output::JsonOutput> {
        let bytes = self
            .measure(value)?
            .checked_sub(self.framing_bytes())
            .ok_or_else(|| io::Error::other("delivery framing underflow"))?;
        crate::json_output::JsonOutput::write(pool, bytes, |mut writer| {
            stream::write(self, value, &mut writer)
        })
    }
}

#[derive(Serialize)]
struct ServerMeta {
    #[serde(rename = "io.modelcontextprotocol/serverInfo")]
    server_info: ServerIdentity,
}
#[derive(Serialize)]
struct ServerIdentity {
    name: &'static str,
    version: &'static str,
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
mod framing_tests {
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
        let pool = std::sync::Arc::new(datafusion::execution::memory_pool::GreedyMemoryPool::new(
            1024 * 1024,
        )) as std::sync::Arc<dyn datafusion::execution::memory_pool::MemoryPool>;
        let result = DeliveryProfile::McpStdio {
            era: ProtocolEra::Modern,
            framing_bytes: 35,
        }
        .project(&value, &pool)
        .unwrap();
        let result: serde_json::Value = serde_json::from_str(result.as_str()).unwrap();
        assert_eq!(result["isError"], true);
        let preview: serde_json::Value =
            serde_json::from_str(result["content"][0]["text"].as_str().unwrap()).unwrap();
        assert_eq!(
            preview["error"],
            serde_json::to_value(value.error().unwrap()).unwrap()
        );
        assert_eq!(preview["request_id"], value.request_id.as_str());
        assert!(preview.get("details_omitted").is_none());
    }
}

#[cfg(test)]
mod tests;
