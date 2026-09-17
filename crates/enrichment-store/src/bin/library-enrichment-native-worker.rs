//! Trusted resource-bounded native decoder. No public query or producer interface.
fn main() -> std::process::ExitCode {
    match enrichment_store::native_worker::worker_main() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error) => {
            eprintln!(
                "{}",
                error.to_string().chars().take(4096).collect::<String>()
            );
            std::process::ExitCode::FAILURE
        }
    }
}
