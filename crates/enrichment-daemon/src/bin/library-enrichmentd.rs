//! The daemon CLI: `library-enrichmentd start|status|stop` (blueprint §2.2).
//!
//! Three subcommands do not justify an argument-parsing dependency: `clap` is absent from the
//! lock and would add roughly eight crates to read one word of input.
//!
//! All output here goes to stderr or the daemon log, never stdout of an MCP process -- this
//! binary is not the MCP adapter, but the habit is the one the whole service keeps (§7.4).

use std::process::ExitCode;

use enrichment_core::config::Config;
use enrichment_daemon::paths::DaemonPaths;
use enrichment_daemon::server;

const USAGE: &str = "usage: library-enrichmentd <start|status|stop|validate [FILE]|socket-path>";

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let command = args.next();

    // `validate` is the only subcommand that takes an operand, so it is handled before the
    // arity check the others share.
    if command.as_deref() == Some("validate") {
        let source = args.next();
        if args.next().is_some() {
            eprintln!("library-enrichmentd: validate takes at most one file\n{USAGE}");
            return ExitCode::FAILURE;
        }
        return validate(source.as_deref());
    }

    if args.next().is_some() {
        eprintln!("library-enrichmentd: too many arguments\n{USAGE}");
        return ExitCode::FAILURE;
    }

    let paths = match DaemonPaths::from_env() {
        Ok(paths) => paths,
        Err(err) => {
            eprintln!("library-enrichmentd: {err}");
            return ExitCode::FAILURE;
        }
    };
    // Refuse to start on unparsable configuration rather than running with different limits
    // than the operator wrote. An unset or absent LIBENR_CONFIG is not an error.
    let config = match Config::from_env() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("library-enrichmentd: {err}");
            return ExitCode::FAILURE;
        }
    };

    let runtime = match tokio::runtime::Runtime::new() {
        Ok(runtime) => runtime,
        Err(err) => {
            eprintln!("library-enrichmentd: cannot start the async runtime: {err}");
            return ExitCode::FAILURE;
        }
    };

    match command.as_deref() {
        Some("start") => runtime.block_on(start(&paths, &config)),
        Some("status") => runtime.block_on(status(&paths)),
        Some("stop") => runtime.block_on(stop(&paths)),
        Some("socket-path") => {
            // Prints the resolved socket and exits. The Python adapter re-implements this
            // resolution order, so `tests/contract/test_socket_resolution.py` uses this
            // subcommand to compare the two across every branch -- the duplication is real, and
            // this is what makes a divergence a test failure instead of a silent misconnect.
            println!("{}", paths.socket.display());
            ExitCode::SUCCESS
        }
        Some(other) => {
            eprintln!("library-enrichmentd: unknown command `{other}`\n{USAGE}");
            ExitCode::FAILURE
        }
        None => {
            eprintln!("{USAGE}");
            ExitCode::FAILURE
        }
    }
}

/// Validate a candidate response envelope from a file, or from stdin when no file is given.
///
/// This is acceptance gate C19's **CLI** boundary. It needs no daemon, because it is the same
/// `enrichment_daemon::validate::validate` the `wire.validate` RPC method calls -- one
/// implementation reached two ways, which is what lets the two boundaries be compared rather
/// than merely both reject something.
///
/// Prints the verdict as JSON on stdout and exits 0 for a conforming document, 1 otherwise, so
/// a shell can branch on it.
fn validate(source: Option<&str>) -> ExitCode {
    let document = match source {
        Some(path) => match std::fs::read_to_string(path) {
            Ok(text) => text,
            Err(err) => {
                eprintln!("library-enrichmentd: cannot read {path}: {err}");
                return ExitCode::FAILURE;
            }
        },
        None => {
            let mut buffer = String::new();
            if let Err(err) = std::io::Read::read_to_string(&mut std::io::stdin(), &mut buffer) {
                eprintln!("library-enrichmentd: cannot read stdin: {err}");
                return ExitCode::FAILURE;
            }
            buffer
        }
    };

    let verdict = enrichment_daemon::validate::validate(&document);
    let rendered =
        serde_json::to_string_pretty(&verdict).unwrap_or_else(|_| "{\"valid\":false}".to_owned());
    println!("{rendered}");

    if verdict.valid {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

/// Run in the foreground until SIGINT or SIGTERM.
///
/// Deliberately not self-daemonizing: a supervised foreground process is easier to log, test
/// and stop, and `just` or a service manager can background it.
async fn start(paths: &DaemonPaths, config: &Config) -> ExitCode {
    let shutdown = async {
        let mut term =
            match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
                Ok(signal) => signal,
                Err(err) => {
                    eprintln!("library-enrichmentd: cannot listen for SIGTERM: {err}");
                    return;
                }
            };
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {}
            _ = term.recv() => {}
        }
        eprintln!("library-enrichmentd: shutting down");
    };

    eprintln!(
        "library-enrichmentd: configuration {}",
        config.source.describe()
    );
    match server::serve(paths, config.limits.rpc_message_bytes, shutdown).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("library-enrichmentd: {err}");
            ExitCode::FAILURE
        }
    }
}

/// Report whether a daemon is reachable, and what it says about itself.
///
/// "Not running" is an ordinary, successful answer to the question -- it is reported on stdout
/// and exits non-zero so a script can branch, but it is not an error condition.
async fn status(paths: &DaemonPaths) -> ExitCode {
    match server::request_status(paths).await {
        Ok(response) => {
            let rendered =
                serde_json::to_string_pretty(&response).unwrap_or_else(|_| "{}".to_owned());
            println!("{rendered}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            println!(
                "library-enrichmentd: not running ({}): {err}",
                paths.socket.display()
            );
            ExitCode::FAILURE
        }
    }
}

/// Stop a running daemon.
///
/// Phase 0 has no durable jobs, so this is a plain shutdown. When jobs land in Phase 4 this
/// must not cancel work another caller still needs (§2.2, §8.2).
async fn stop(paths: &DaemonPaths) -> ExitCode {
    match server::request_shutdown(paths).await {
        Ok(()) => {
            eprintln!("library-enrichmentd: stopped");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!(
                "library-enrichmentd: not running ({}): {err}",
                paths.socket.display()
            );
            ExitCode::FAILURE
        }
    }
}
