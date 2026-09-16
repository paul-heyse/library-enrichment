//! Trusted resource-bounded native decoder. No public query or producer interface.
fn main() -> std::process::ExitCode {
    match enrichment_store::native_worker::worker_main() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(_) => std::process::ExitCode::FAILURE,
    }
}
