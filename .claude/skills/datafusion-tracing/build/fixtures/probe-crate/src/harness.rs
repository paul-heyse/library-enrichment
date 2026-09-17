//! Capture the JSON span stream a query produces, deterministically.
//!
//! Mirrors upstream's own harness (`tests/test_utils/in_memory_writer.rs` and
//! `init_subscriber` in `tests/integration_tests.rs`): an `fmt` layer formatting as JSON with
//! `flatten_event(true)` and `without_time()`, writing into a shared buffer.
//!
//! Determinism is not free. Even `without_time()` leaves `time.busy`, `time.idle` and the
//! metric timestamps, which differ on every run, so `normalise` replaces them with a
//! placeholder before anything is digested. What is stored is the normalised capture, which is
//! stable while the claim holds and changes when it stops holding -- the sensitivity a drift
//! check wants.

use std::io::Write;
use std::sync::{Arc, Mutex, MutexGuard};
use tracing_subscriber::fmt::MakeWriter;

#[derive(Clone)]
pub struct Buffer(pub Arc<Mutex<Vec<u8>>>);

impl Buffer {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(Vec::new())))
    }
    pub fn take(&self) -> String {
        String::from_utf8_lossy(&self.0.lock().unwrap()).into_owned()
    }
}

pub struct Guard<'a> {
    guard: MutexGuard<'a, Vec<u8>>,
}

impl Write for Guard<'_> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.guard.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.guard.flush()
    }
}

impl<'a> MakeWriter<'a> for Buffer {
    type Writer = Guard<'a>;
    fn make_writer(&'a self) -> Self::Writer {
        Guard { guard: self.0.lock().unwrap() }
    }
}

/// Replace everything that legitimately varies between runs.
pub fn normalise(captured: &str) -> String {
    let mut out = Vec::new();
    for line in captured.lines() {
        let Ok(mut value) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };
        scrub(&mut value);
        out.push(serde_json::to_string(&value).unwrap_or_default());
    }
    out.sort();
    out.join("\n")
}

fn scrub(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            let keys: Vec<String> = map.keys().cloned().collect();
            for key in keys {
                let varies = key == "time.busy"
                    || key == "time.idle"
                    || key.ends_with("_timestamp")
                    || key.ends_with("elapsed_compute")
                    || key.ends_with("_time")
                    || key == "threadId"
                    || key == "threadName";
                if varies {
                    map.insert(key, serde_json::Value::String("<VARIES>".into()));
                } else if let Some(inner) = map.get_mut(&key) {
                    scrub(inner);
                }
            }
        }
        serde_json::Value::Array(items) => items.iter_mut().for_each(scrub),
        _ => {}
    }
}
