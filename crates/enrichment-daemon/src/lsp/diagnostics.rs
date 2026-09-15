//! One open document's bounded protocol diagnostic state. Transport values normalize later.
use serde_json::{Value, json};
use std::io;

const MAX_ITEMS: usize = 1024;
const MAX_BYTES: usize = 512 * 1024;

#[derive(Default)]
pub struct Diagnostics {
    document: Option<(String, i32)>,
    pull: Option<Value>,
    push: Option<Value>,
}

impl Diagnostics {
    pub fn open(&mut self, uri: &str, version: i32) {
        self.document = Some((uri.into(), version));
        self.pull = None;
        self.push = None;
    }

    pub fn previous_id(&self) -> Option<&str> {
        self.pull.as_ref()?.get("resultId")?.as_str()
    }

    pub fn push(&mut self, message: &Value) -> io::Result<()> {
        if message.get("method").and_then(Value::as_str) != Some("textDocument/publishDiagnostics")
        {
            return Ok(());
        }
        let Some((uri, version)) = &self.document else {
            return Ok(());
        };
        let params = &message["params"];
        if params.get("uri").and_then(Value::as_str) != Some(uri)
            || params.get("version").and_then(Value::as_i64) != Some(i64::from(*version))
        {
            // An unversioned push cannot prove which reopening of a URI it describes.
            return Ok(());
        }
        validate_items(&params["diagnostics"])?;
        self.push = Some(json!({"kind":"full", "items":params["diagnostics"],
            "documentVersion": version, "source":"push"}));
        Ok(())
    }

    pub fn pushed(&self, uri: &str) -> Option<Value> {
        self.document.as_ref().filter(|(open, _)| open == uri)?;
        self.push.clone()
    }

    pub fn pulled(&mut self, uri: &str, report: Value) -> io::Result<Value> {
        if !self.document.as_ref().is_some_and(|(open, _)| open == uri) {
            return Err(invalid("diagnostics do not name the open document"));
        }
        match report.get("kind").and_then(Value::as_str) {
            Some("full") => {
                validate_items(&report["items"])?;
                if report.get("resultId").is_some_and(|v| !v.is_string()) {
                    return Err(invalid("invalid diagnostic result identity"));
                }
                let mut full = json!({"kind":"full", "items": report["items"]});
                if let Some(id) = report.get("resultId") {
                    full["resultId"] = id.clone();
                }
                self.pull = Some(full.clone());
                Ok(full)
            }
            Some("unchanged") => {
                let id = report
                    .get("resultId")
                    .and_then(Value::as_str)
                    .ok_or_else(|| invalid("unchanged diagnostics lack resultId"))?;
                if self.previous_id() != Some(id) {
                    return Err(invalid(
                        "unchanged diagnostics have no matching retained result",
                    ));
                }
                self.pull
                    .clone()
                    .ok_or_else(|| invalid("diagnostic result disappeared"))
            }
            _ => Err(invalid("unknown diagnostic report kind")),
        }
    }
}

fn validate_items(items: &Value) -> io::Result<()> {
    let items = items
        .as_array()
        .ok_or_else(|| invalid("diagnostics items must be a list"))?;
    if items.len() > MAX_ITEMS || serde_json::to_vec(items)?.len() > MAX_BYTES {
        return Err(invalid("diagnostics exceed result budget"));
    }
    Ok(())
}
fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unchanged_requires_matching_result_and_never_means_empty() {
        let mut state = Diagnostics::default();
        state.open("file:///capsule/a.py", 1);
        let unchanged = json!({"kind":"unchanged", "resultId":"a"});
        assert!(
            state
                .pulled("file:///capsule/a.py", unchanged.clone())
                .is_err()
        );
        let full = json!({"kind":"full", "resultId":"a", "items":[{"message":"error"}]});
        state.pulled("file:///capsule/a.py", full.clone()).unwrap();
        assert_eq!(
            state
                .pulled("file:///capsule/a.py", unchanged.clone())
                .unwrap(),
            full
        );
        assert!(
            state
                .pulled("file:///capsule/b.py", unchanged.clone())
                .is_err()
        );
        state.open("file:///capsule/a.py", 2);
        assert!(state.pulled("file:///capsule/a.py", unchanged).is_err());
    }
    #[test]
    fn pushes_replace_only_the_matching_uri_and_version() {
        let mut state = Diagnostics::default();
        state.open("file:///capsule/a.py", 2);
        let push = |uri: &str, version: Value, items: Value| {
            json!({"method":
            "textDocument/publishDiagnostics", "params": {"uri":uri,"version":version,
            "diagnostics":items}})
        };
        state
            .push(&push(
                "file:///capsule/a.py",
                json!(2),
                json!([{"message":"error"}]),
            ))
            .unwrap();
        for (uri, version) in [
            ("file:///capsule/b.py", json!(2)),
            ("file:///capsule/a.py", json!(1)),
            ("file:///capsule/a.py", Value::Null),
        ] {
            state.push(&push(uri, version, json!([]))).unwrap();
            assert_eq!(
                state.pushed("file:///capsule/a.py").unwrap()["items"]
                    .as_array()
                    .unwrap()
                    .len(),
                1
            );
        }
        state
            .push(&push("file:///capsule/a.py", json!(2), json!([])))
            .unwrap();
        assert!(
            state.pushed("file:///capsule/a.py").unwrap()["items"]
                .as_array()
                .unwrap()
                .is_empty()
        );
    }
}
