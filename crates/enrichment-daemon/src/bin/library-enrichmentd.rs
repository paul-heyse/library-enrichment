//! The daemon CLI: `library-enrichmentd start|status|stop` (blueprint §2.2).
//!
//! Three subcommands do not justify an argument-parsing dependency: `clap` is absent from the
//! lock and would add roughly eight crates to read one word of input.
//!
//! All output here goes to stderr or the daemon log, never stdout of an MCP process -- this
//! binary is not the MCP adapter, but the habit is the one the whole service keeps (§7.4).

use std::process::ExitCode;
use std::sync::Arc;

use enrichment_core::config::Config;
use enrichment_daemon::paths::DaemonPaths;
use enrichment_daemon::server;
use enrichment_daemon::service::Service;
use enrichment_store::StatePaths;

const USAGE: &str = "usage: library-enrichmentd \
<start|status|stop|validate [FILE]|socket-path|export CONTEXT DIR|verify-bundle DIR|cleanup cache|evidence [--plan|--apply]|reset-development ROOT [--plan|--apply]>";

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let command = args.next();

    if matches!(
        command.as_deref(),
        Some("execution-describe" | "execution-probe")
    ) {
        return execution_operator(
            command.as_deref() == Some("execution-probe"),
            args.collect(),
        );
    }

    if matches!(command.as_deref(), Some("cleanup" | "reset-development")) {
        let operand = args.next();
        let mode = args.next();
        if operand.is_none()
            || args.next().is_some()
            || !matches!(mode.as_deref(), None | Some("--plan" | "--apply"))
        {
            eprintln!("library-enrichmentd: invalid cleanup arguments\n{USAGE}");
            return ExitCode::FAILURE;
        }
        return maintenance(
            command.as_deref() == Some("reset-development"),
            operand.as_deref().unwrap_or_default(),
            mode.as_deref() == Some("--apply"),
        );
    }

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

    // `export` and `verify-bundle` read the store directly and need no running daemon: a
    // bundle is a copy of what is already on disk, and taking one should not require the
    // service to be up.
    if command.as_deref() == Some("export") {
        let (context, out) = (args.next(), args.next());
        let (Some(context), Some(out)) = (context, out) else {
            eprintln!("library-enrichmentd: export needs a context id and a directory\n{USAGE}");
            return ExitCode::FAILURE;
        };
        return export(&context, std::path::Path::new(&out));
    }
    if command.as_deref() == Some("verify-bundle") {
        let Some(dir) = args.next() else {
            eprintln!("library-enrichmentd: verify-bundle needs a directory\n{USAGE}");
            return ExitCode::FAILURE;
        };
        return verify_bundle(std::path::Path::new(&dir));
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
        Some("start") => runtime.block_on(start(&paths, config)),
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

fn execution_operator(probe: bool, args: Vec<String>) -> ExitCode {
    let result = (|| -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        if args.len() != if probe { 3 } else { 1 } {
            return Err("execution-describe ROOT or execution-probe ROOT ECOSYSTEM IMAGE".into());
        }
        let root = std::path::Path::new(&args[0]);
        let mut execution = Config::from_env()?.execution;
        execution.storage_root = Some(root.to_owned());
        if let Some(path) = std::env::var_os("LIBENR_BROKER") {
            execution.broker_path = Some(path.into());
        }
        if probe {
            let paths = enrichment_daemon::execution::description::qualification_paths(root)?;
            enrichment_store::state::initialize(&paths)?;
            let cache = paths.cache_root;
            let runtime = tokio::runtime::Runtime::new()?;
            Ok(serde_json::to_value(runtime.block_on(
                enrichment_daemon::execution::description::probe(
                    &execution, &cache, &args[1], &args[2],
                ),
            )?)?)
        } else {
            Ok(serde_json::to_value(
                enrichment_daemon::execution::description::describe(&execution, root)?,
            )?)
        }
    })();
    match result.and_then(|value| Ok(serde_json::to_string_pretty(&value)?)) {
        Ok(value) => {
            println!("{value}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("library-enrichmentd: execution setup: {error}");
            ExitCode::FAILURE
        }
    }
}

fn maintenance(development: bool, operand: &str, apply: bool) -> ExitCode {
    let result = (|| -> Result<_, Box<dyn std::error::Error>> {
        let config = Config::from_env()?;
        if development {
            Ok(enrichment_daemon::maintenance::reset_development(
                &config,
                std::path::Path::new(operand),
                apply,
            )?)
        } else {
            let scope = match operand {
                "cache" => enrichment_daemon::maintenance::Scope::Cache,
                "evidence" => enrichment_daemon::maintenance::Scope::Evidence,
                _ => return Err("cleanup scope must be cache or evidence".into()),
            };
            Ok(enrichment_daemon::maintenance::cleanup(
                &config,
                &StatePaths::from_env()?,
                scope,
                apply,
            )?)
        }
    })();
    match result {
        Ok(report) => match serde_json::to_string_pretty(&report) {
            Ok(output) => {
                println!("{output}");
                if report.error.is_none() {
                    ExitCode::SUCCESS
                } else {
                    ExitCode::FAILURE
                }
            }
            Err(error) => {
                eprintln!("library-enrichmentd: cannot render cleanup report: {error}");
                ExitCode::FAILURE
            }
        },
        Err(error) => {
            eprintln!("library-enrichmentd: cleanup refused: {error}");
            ExitCode::FAILURE
        }
    }
}

/// Write a portable bundle for one context, reading the store directly (ADR-0018).
///
/// No daemon is needed and none is contacted: a bundle is a copy of immutable evidence that is
/// already on disk, and requiring the service to be up to take one would make export unavailable
/// in exactly the situation -- a wedged or stopped service -- where an operator most wants it.
fn export(context_id: &str, out: &std::path::Path) -> ExitCode {
    let paths = match StatePaths::from_env() {
        Ok(paths) => paths,
        Err(err) => {
            eprintln!("library-enrichmentd: {err}");
            return ExitCode::FAILURE;
        }
    };
    match enrichment_daemon::export::export(&paths, context_id, out) {
        Ok(exported) => {
            eprintln!(
                "library-enrichmentd: exported {} ({} file(s), {} artifact(s)) to {}",
                exported.context_id,
                exported.files,
                exported.artifacts,
                exported.root.display()
            );
            eprintln!(
                "  verify it anywhere with: library-enrichmentd verify-bundle {}",
                exported.root.display()
            );
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("library-enrichmentd: export failed: {err}");
            ExitCode::FAILURE
        }
    }
}

/// Check a bundle against its own manifest. Needs nothing but the bundle.
fn verify_bundle(root: &std::path::Path) -> ExitCode {
    match enrichment_daemon::export::verify(root) {
        Ok(problems) if problems.is_empty() => {
            eprintln!(
                "library-enrichmentd: {} verifies against its manifest",
                root.display()
            );
            ExitCode::SUCCESS
        }
        Ok(problems) => {
            eprintln!("library-enrichmentd: {} does NOT verify:", root.display());
            for problem in problems {
                eprintln!("  - {problem}");
            }
            ExitCode::FAILURE
        }
        Err(err) => {
            eprintln!("library-enrichmentd: {err}");
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
///
/// The state roots are resolved here, once, and only for `start`: `status` and `stop` never
/// touch them. A root that cannot be resolved -- or would be relative -- refuses to start, for
/// the same reason the socket resolver does (blueprint §2.3, gate C20).
async fn start(paths: &DaemonPaths, config: Config) -> ExitCode {
    let state = match StatePaths::from_env() {
        Ok(state) => state,
        Err(err) => {
            eprintln!("library-enrichmentd: {err}");
            return ExitCode::FAILURE;
        }
    };
    eprintln!(
        "library-enrichmentd: configuration {}",
        config.source.describe()
    );
    eprintln!(
        "library-enrichmentd: cache {} data {}",
        state.cache_root.display(),
        state.data_root.display()
    );
    let service = match Service::open(config, state) {
        Ok(service) => Arc::new(service),
        Err(err) => {
            eprintln!("library-enrichmentd: {err}");
            return ExitCode::FAILURE;
        }
    };

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

    match server::serve(service, paths, shutdown).await {
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
/// Phase 1 has no durable jobs, so this is a plain shutdown. When jobs land in Phase 4 this
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
