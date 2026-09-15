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
    tokio::time::timeout(std::time::Duration::from_secs(30), async {
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
                return complete_answer(service, record["data"]["result"].clone()).await;
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
    if response["error"]["code"] != "BUDGET_EXCEEDED"
        || !response["data"]["result_artifact_id"].is_string()
    {
        return response;
    }
    let id = response["data"]["result_artifact_id"].as_str().unwrap();
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
            let next = page["pagination"]["next_cursor"]
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
    let mut decoded: Value = serde_json::from_str(&content).expect("complete answer JSON");
    assert_eq!(decoded["context_id"], response["context_id"]);
    assert_eq!(decoded["snapshot_id"], response["snapshot_id"]);
    decoded["request_id"] = response["request_id"].clone();
    decoded
}
