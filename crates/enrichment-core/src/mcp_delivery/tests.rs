use super::*;
use crate::{
    native_union::{Cell, NativeStruct, Rule},
    operation::results::ResultRecord,
};
use std::sync::Arc;

fn fixtures() -> Vec<Envelope> {
    let mut result = Vec::new();
    for input in [
        include_str!("../../../../tests/fixtures/wire/ok.fixture.json"),
        include_str!("../../../../tests/fixtures/wire/partial.fixture.json"),
        include_str!("../../../../tests/fixtures/wire/pending.fixture.json"),
        include_str!("../../../../tests/fixtures/wire/error.fixture.json"),
    ] {
        let mut value: Envelope = serde_json::from_str(input).unwrap();
        value.summary = "é🦀 \"quote\" \\ slash\nline\tcontrol\u{0}\u{8}\u{c}\u{2028}".into();
        if let Some(error) = value.error_mut() {
            error.message = "failed 🦀\n\"specific\"".into();
        }
        result.push(value.clone());
        if value.status() != crate::wire::Status::Pending {
            value.data = Default::default();
            value.delivery = crate::wire::DeliveryDescriptor::retained(
                format!("art_{}", "ab".repeat(32)),
                vec![],
                crate::wire::DeliveryLimits {
                    requested_max_bytes: Some(usize::MAX),
                    effective_max_bytes: Some(usize::MAX),
                },
            );
            result.push(value.clone());
        }
        if value.error().is_some() {
            let mut outer = result[0].clone();
            outer.data = crate::wire::data::ToolData::from(crate::execution::JobData {
                job_id: "job_00112233445566778899aabbccddeeff".to_owned().try_into().unwrap(),
                state: crate::wire::JobState::Failed,
                stage: "complete".into(),
                interest_token: None,
                active_interests: 0,
                submitted_at: "2026-09-17T00:00:00.000000Z".to_owned().try_into().unwrap(),
                updated_at: "2026-09-17T00:00:00.000000Z".to_owned().try_into().unwrap(),
                result: Some(crate::execution::JobResult::from(&value)),
            });
            result.push(outer);
        }
    }
    result
}

#[test]
fn borrowed_envelope_and_nested_transport_match_owned_wire_for_all_outcomes() {
    for value in fixtures() {
        let record = ResultRecord::from_envelope(&value).unwrap();
        let array = <ResultRecord as NativeStruct>::encode(&[Some(&record)]).unwrap();
        let field = Arc::new(crate::native_union::field::<ResultRecord>(
            "result",
            Rule::Text,
        ));
        let native =
            native::NativeEnvelope::new(&field, array.as_ref(), 0, value.request_id.as_str())
                .unwrap();
        let mut actual = Vec::new();
        stream::write(&DeliveryProfile::Envelope, &native, &mut actual).unwrap();
        assert_eq!(actual, serde_json::to_vec(&value).unwrap());
        for era in [ProtocolEra::Classic, ProtocolEra::Modern] {
            for profile in [
                DeliveryProfile::McpStdio {
                    era,
                    framing_bytes: 35,
                },
                DeliveryProfile::McpResourceStdio {
                    era,
                    framing_bytes: 42,
                    uri: "library-evidence://artifacts/art_unicode_é".into(),
                },
            ] {
                let mut borrowed = Vec::new();
                let mut owned = Vec::new();
                stream::write(&profile, &native, &mut borrowed).unwrap();
                stream::write(&profile, &value, &mut owned).unwrap();
                assert_eq!(borrowed, owned, "{profile:?}");
                let pool = Arc::new(datafusion::execution::memory_pool::GreedyMemoryPool::new(
                    1024 * 1024,
                ))
                    as Arc<dyn datafusion::execution::memory_pool::MemoryPool>;
                let encoded = profile.project(&value, &pool).unwrap();
                let projected: serde_json::Value = serde_json::from_str(encoded.as_str()).unwrap();
                assert_eq!(
                    stream::measure(&profile, &native).unwrap(),
                    serde_json::to_vec(&projected).unwrap().len() + profile.framing_bytes()
                );
                if matches!(profile, DeliveryProfile::McpResourceStdio { .. }) {
                    assert_eq!(
                        projected["contents"][0]["text"],
                        serde_json::to_string(&value).unwrap()
                    );
                }
                if let Some(error) = value.error()
                    && matches!(profile, DeliveryProfile::McpStdio { .. })
                {
                    let preview: serde_json::Value =
                        serde_json::from_str(projected["content"][0]["text"].as_str().unwrap())
                            .unwrap();
                    assert_eq!(preview["error"], serde_json::to_value(error).unwrap());
                    assert_eq!(projected["isError"], true);
                }
            }
        }
    }
}

#[test]
fn borrowed_delivery_rejects_null_and_empty_request_without_hydration() {
    let field = Arc::new(crate::native_union::field::<ResultRecord>(
        "result",
        Rule::Text,
    ));
    let array = <ResultRecord as Cell>::encode(&[None]).unwrap();
    assert!(native::NativeEnvelope::new(&field, array.as_ref(), 0, "req").is_err());
    let record = ResultRecord::from_envelope(&fixtures()[0]).unwrap();
    let array = <ResultRecord as Cell>::encode(&[Some(&record)]).unwrap();
    assert!(native::NativeEnvelope::new(&field, array.as_ref(), 0, "").is_err());
}
