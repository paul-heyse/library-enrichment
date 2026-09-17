//! Isolated cross-language wire oracle inputs; no daemon, table, job or MCP session is opened.
use enrichment_core::{
    execution::{JobData, JobResult},
    mcp_delivery::{DeliveryProfile, ProtocolEra, project},
    wire::{DeliveryDescriptor, DeliveryLimits, Envelope, JobState, data::ToolData},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for (name, fixture) in [
        (
            "ok",
            include_str!("../../../tests/fixtures/wire/ok.fixture.json"),
        ),
        (
            "partial",
            include_str!("../../../tests/fixtures/wire/partial.fixture.json"),
        ),
        (
            "pending",
            include_str!("../../../tests/fixtures/wire/pending.fixture.json"),
        ),
        (
            "error",
            include_str!("../../../tests/fixtures/wire/error.fixture.json"),
        ),
    ] {
        let mut value: Envelope = serde_json::from_str(fixture)?;
        value.summary = "Unicode é 🦀 \"quote\" \\ slash\nline\tcontrol\u{8}\u{c}\u{2028}".into();
        let summary = value.summary.clone();
        if let Some(error) = value.error_mut() {
            error.message = summary;
        }
        emit(name, &value)?;
        value.delivery = DeliveryDescriptor::retained(
            format!("art_{}", "ab".repeat(32)),
            vec![],
            DeliveryLimits::default(),
        );
        emit(&format!("{name}_artifact"), &value)?;
        if value.error().is_some() {
            let terminal = JobResult::from(&value);
            let mut outer: Envelope =
                serde_json::from_str(include_str!("../../../tests/fixtures/wire/ok.fixture.json"))?;
            outer.data = ToolData::from(JobData {
                job_id: "job_fixture".into(),
                state: JobState::Failed,
                stage: "complete".into(),
                interest_token: None,
                active_interests: 0,
                submitted_at: "2026-09-17T00:00:00Z".into(),
                updated_at: "2026-09-17T00:00:00Z".into(),
                result: Some(terminal),
            });
            emit("terminal_job_error", &outer)?;
        }
    }
    Ok(())
}

fn emit(name: &str, value: &Envelope) -> Result<(), Box<dyn std::error::Error>> {
    for (protocol, era) in [
        ("2025-11-25", ProtocolEra::Classic),
        ("2026-07-28", ProtocolEra::Modern),
    ] {
        // The Python oracle replaces this with independently measured actual RPC IDs.
        let profile = DeliveryProfile::McpStdio {
            era,
            framing_bytes: 35,
        };
        println!(
            "{}",
            serde_json::to_string(&serde_json::json!({
                "case": name, "protocol": protocol, "method": "tools/call", "result": project(value, &era)?,
                "delivery_bytes": profile.measure(value)?,
            }))?
        );
        let uri = "library-evidence://artifacts/art_fixture";
        let profile = DeliveryProfile::McpResourceStdio {
            era,
            framing_bytes: 35,
            uri: uri.into(),
        };
        println!(
            "{}",
            serde_json::to_string(&serde_json::json!({
                "case": name, "protocol": protocol, "method": "resources/read", "uri": uri,
                "result": enrichment_core::mcp_delivery::project_resource(value, &era, uri)?,
                "delivery_bytes": profile.measure(value)?,
            }))?
        );
    }
    Ok(())
}
