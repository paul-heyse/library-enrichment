//! Protocol observations retained between requests. Silence is never an indexing barrier.
use serde_json::Value;
use std::io;

#[derive(Default)]
pub struct Notifications {
    pub diagnostics: super::diagnostics::Diagnostics,
    analyzer: Option<AnalyzerStatus>,
}
struct AnalyzerStatus {
    quiescent: bool,
    health: String,
    message: Option<String>,
}
impl Notifications {
    pub fn accept(&mut self, message: &Value) -> io::Result<()> {
        self.diagnostics.push(message)?;
        if message["method"] == "experimental/serverStatus" {
            let p = &message["params"];
            let quiescent = p["quiescent"]
                .as_bool()
                .ok_or_else(|| io::Error::other("invalid rust-analyzer readiness flag"))?;
            let health = p["health"]
                .as_str()
                .filter(|h| matches!(*h, "ok" | "warning" | "error"))
                .ok_or_else(|| io::Error::other("invalid rust-analyzer health"))?
                .to_owned();
            let message = match p.get("message") {
                None | Some(Value::Null) => None,
                Some(Value::String(s)) if s.len() <= 8192 => Some(s.clone()),
                _ => return Err(io::Error::other("invalid rust-analyzer status message")),
            };
            self.analyzer = Some(AnalyzerStatus {
                quiescent,
                health,
                message,
            });
        }
        Ok(())
    }
    pub fn indexing_gap(&self) -> Option<String> {
        match &self.analyzer {
            None => Some("rust-analyzer has not reported an indexing readiness state.".into()),
            Some(status) if !status.quiescent || status.health != "ok" => Some(format!(
                "rust-analyzer reports quiescent={}, health={}: {}",
                status.quiescent,
                status.health,
                status
                    .message
                    .as_deref()
                    .unwrap_or("workspace analysis has not settled successfully")
            )),
            Some(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn readiness_is_a_persistent_observed_state_and_health_is_part_of_it() {
        let mut state = Notifications::default();
        assert!(state.indexing_gap().is_some());
        for (quiescent, health, ready) in [
            (false, "ok", false),
            (true, "warning", false),
            (true, "ok", true),
        ] {
            state.accept(&serde_json::json!({"method":"experimental/serverStatus", "params":{"quiescent":quiescent,"health":health}})).unwrap();
            assert_eq!(state.indexing_gap().is_none(), ready);
        }
        state.diagnostics.open("file:///capsule/consumer.py", 1);
        assert!(
            state.indexing_gap().is_none(),
            "opening a document does not invent a new unknown workspace state"
        );
        assert!(state.accept(&serde_json::json!({"method":"experimental/serverStatus", "params":{"quiescent":true,"health":"unknown"}})).is_err());
    }
}
