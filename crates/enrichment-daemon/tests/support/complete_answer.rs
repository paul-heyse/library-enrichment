//! Exercise the public paging contract without relaxing inline budgets or reading store files.
use enrichment_daemon::{server, service::Service};
use serde_json::Value;

/// Resolution is a durable job and can exceed the inline wait, including under test load.
/// Preserve semantic assertions on its terminal answer through the public job/paging contract.
pub async fn wait_for_answer(service: &Service, response: Value) -> Value {
    let response = complete_answer(service, response).await;
    if response["status"] != "pending" {
        return response;
    }
    let id = response["data"]["job_id"]
        .as_str()
        .expect("pending job identity")
        .to_owned();
    // The public pending contract owns the configured job and cleanup deadlines. Its
    // completion waiter must cover that contract; individual RPC/read budgets stay bounded.
    let seconds = service
        .config
        .network
        .acquisition_timeout_seconds
        .max(service.config.execution.deadline_seconds)
        .saturating_add(service.config.execution.cleanup_deadline_seconds)
        .saturating_add(service.config.arrow.query_deadline_seconds);
    tokio::time::timeout(std::time::Duration::from_secs(seconds), async {
        loop {
            let frame = serde_json::json!({"jsonrpc":"2.0","id":3,"method":"job.control",
                "params":{"job_id":id,"action":"wait","wait_seconds":1}})
            .to_string();
            let response = server::dispatch(service, &frame)
                .await
                .response
                .expect("job RPC");
            assert!(response.error.is_none(), "{:?}", response.error);
            let record = complete_answer(service, response.result.expect("job envelope")).await;
            assert_eq!(record["data"]["job_id"], id);
            if record["data"]["result"].is_object() {
                let terminal = &record["data"]["result"];
                assert_eq!(
                    terminal["delivery"]["mode"], "artifact",
                    "terminal results retain a direct descriptor"
                );
                let mut descriptor = record.clone();
                descriptor["delivery"] = terminal["delivery"].clone();
                descriptor["context_id"] = terminal["context_id"].clone();
                descriptor["snapshot_id"] = terminal["snapshot_id"].clone();
                return complete_answer(service, descriptor).await;
            }
            assert!(
                matches!(
                    record["data"]["state"].as_str(),
                    Some("queued" | "running" | "cancel_requested")
                ),
                "{record}"
            );
        }
    })
    .await
    .expect("bounded concrete job completion")
}

pub async fn complete_answer(service: &Service, response: Value) -> Value {
    complete_answer_measured(service, response).await.0
}

#[derive(Default, serde::Serialize)]
pub struct DeliveryObservation {
    pub artifact_calls: usize,
    pub artifact_response_bytes: usize,
    pub complete_content_bytes: usize,
    pub read_micros: u128,
}

pub async fn complete_answer_measured(
    service: &Service,
    response: Value,
) -> (Value, DeliveryObservation) {
    let started = std::time::Instant::now();
    let mut observed = DeliveryObservation::default();
    if response["delivery"]["mode"] != "artifact" {
        return (response, observed);
    }
    let id = response["delivery"]["artifact_id"].as_str().unwrap();
    let mut cursor = None::<String>;
    let mut content = String::new();
    tokio::time::timeout(std::time::Duration::from_secs(30), async {
        loop {
            let frame = serde_json::json!({"jsonrpc":"2.0","id":2,"method":"artifact.read",
                "params":{"artifact_id":id,"cursor":cursor,"max_bytes":4096}})
            .to_string();
            let result = server::dispatch(service, &frame)
                .await
                .response
                .expect("artifact RPC");
            assert!(result.error.is_none(), "{:?}", result.error);
            let page = result.result.expect("artifact envelope");
            observed.artifact_calls += 1;
            observed.artifact_response_bytes += serde_json::to_vec(&page)
                .expect("encoded native result")
                .len();
            assert_eq!(page["status"], "ok", "{page}");
            let slice = page["data"]["content"].as_str().expect("UTF-8 answer JSON");
            assert_eq!(
                page["data"]["content_digest"],
                enrichment_core::canonical::sha256_hex(slice.as_bytes())
            );
            assert!(
                content.len() + slice.len() <= 32 * 1024 * 1024,
                "result contract bound"
            );
            content.push_str(slice);
            let next = page["data"]["page"]["next_cursor"]
                .as_str()
                .map(str::to_owned);
            let Some(next) = next else {
                break;
            };
            assert_ne!(cursor.as_ref(), Some(&next), "artifact cursor advances");
            cursor = Some(next);
        }
    })
    .await
    .expect("bounded artifact delivery");
    let document: Value = serde_json::from_str(&content).expect("complete indexed answer JSON");
    assert_eq!(document["index"]["format"], "research-result/3");
    let mut decoded = document["result"].clone();
    assert_eq!(decoded["context_id"], response["context_id"]);
    assert_eq!(decoded["snapshot_id"], response["snapshot_id"]);
    decoded["request_id"] = response["request_id"].clone();
    observed.complete_content_bytes = content.len();
    observed.read_micros = started.elapsed().as_micros();
    (decoded, observed)
}
