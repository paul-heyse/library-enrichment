//! The exact initialization options each pinned server was probed with.
//!
//! Every value here was read off a real transcript or the release's own source, recorded in
//! `docs/architecture/compatibility-matrix.md` under "Phase 4 exact LSP settings" on 2026-09-13.
//! They are copied rather than paraphrased: a plausible-looking editor setting that the server
//! silently ignores would leave us analysing the wrong interpreter, or running build scripts we
//! believe we disabled, with nothing in the result to show for it.
//!
//! Two findings from that probe are load-bearing and easy to undo by accident:
//!
//! * ty's options are **flattened** into `initializationOptions`. There is no outer `ty` or
//!   `settings` wrapper; adding one makes every value below disappear without an error.
//! * rust-analyzer has **no** `cargo.offline` setting. Offline behaviour comes from
//!   `cargo.extraArgs` and `cargo.extraEnv`, and `buildScripts.enable` is OR-ed with
//!   `procMacro.enable` in the release's own config source -- so both must be disabled, not one.

use serde_json::{Value, json};

/// Which server a session speaks to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Server {
    /// Astral ty, the Python semantic engine (§1.1 B4).
    Ty,
    /// rust-analyzer, over stdio, from the pinned toolchain.
    RustAnalyzer,
}

impl Server {
    /// The name recorded in an observation's provenance.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Ty => "ty",
            Self::RustAnalyzer => "rust-analyzer",
        }
    }

    /// The argv that starts it inside the capsule image.
    ///
    /// `ty server` takes no configuration flags -- its only listed option is `--help` -- and
    /// rust-analyzer starts its server with no subcommand at all. Both were checked against the
    /// installed binaries, not inferred from documentation.
    #[must_use]
    pub fn argv(self) -> Vec<String> {
        match self {
            Self::Ty => vec!["/opt/producers/bin/ty".to_owned(), "server".to_owned()],
            Self::RustAnalyzer => vec!["/usr/local/cargo/bin/rust-analyzer".to_owned()],
        }
    }

    /// The initialization options for this server in a capsule.
    #[must_use]
    pub fn initialization_options(self) -> Value {
        match self {
            Self::Ty => json!({
                // An untrusted workspace, so nothing in the studied package's own configuration
                // can widen what the server will do. The release's source returns
                // `UseUv::Off` for an untrusted workspace anyway; both are set explicitly.
                "untrustedWorkspace": true,
                "experimental": {"useUv": "off"},
                // A trusted, empty configuration file we wrote. The LSP probe showed the server
                // still consults user configuration afterwards, which is why the container also
                // gets an immutable empty XDG_CONFIG_HOME.
                "configurationFile": "/capsule/probe-config/ty.toml",
                "configuration": {
                    "environment": {
                        "python": "/usr/local/bin/python3",
                        "extra-paths": ["/capsule/python"],
                        "python-version": "3.14",
                        "python-platform": "linux"
                    }
                },
                // Only what we open. A workspace-wide sweep of a studied package is work nobody
                // asked for, on code we did not write.
                "diagnosticMode": "openFilesOnly"
            }),
            Self::RustAnalyzer => json!({
                "cargo": {
                    "extraArgs": ["--offline", "--locked"],
                    "extraEnv": {"CARGO_NET_OFFLINE": "true"},
                    "features": [],
                    "noDefaultFeatures": false,
                    "target": "x86_64-unknown-linux-gnu",
                    "targetDir": "/capsule/target",
                    // Build scripts and proc macros execute crate-authored code. That belongs to
                    // the `build` profile's sandbox, and inside it we still do not need it to
                    // answer a navigation question.
                    "buildScripts": {"enable": false}
                },
                "procMacro": {"enable": false},
                "checkOnSave": false
            }),
        }
    }
}

/// The client capabilities this service advertises.
///
/// `workspace.configuration` is **false** on purpose. The probe showed ty using
/// `initializationOptions` directly when the client declines to serve configuration requests;
/// advertising `true` would oblige us to answer `workspace/configuration` correctly for every
/// section a server asks about, and answering it wrongly is silent.
///
/// Both position encodings are offered, most-preferred first. The pinned ty and rust-analyzer
/// can select UTF-8; UTF-16 is the protocol default only when negotiation is absent.
/// The session records whichever was chosen rather than assuming either --
/// byte offsets and UTF-16 offsets diverge at the first non-ASCII character.
#[must_use]
pub fn client_capabilities() -> Value {
    json!({
        "general": {"positionEncodings": ["utf-8", "utf-16"]},
        "experimental": {"serverStatusNotification": true},
        "workspace": {"configuration": false, "workspaceFolders": true},
        "textDocument": {
            "synchronization": {"dynamicRegistration": false},
            "definition": {"linkSupport": true},
            "implementation": {"linkSupport": true},
            "references": {},
            "diagnostic": {"dynamicRegistration": false, "relatedDocumentSupport": false}
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ty_options_are_flattened_with_no_wrapper_object() {
        // The probe finding this guards: `InitializationOptions` flattens `ClientOptions`, so a
        // `ty` or `settings` wrapper makes every value below invisible to the server -- and it
        // does not complain, it just uses defaults.
        let options = Server::Ty.initialization_options();
        assert!(options.get("ty").is_none(), "{options}");
        assert!(options.get("settings").is_none(), "{options}");
        assert_eq!(options["untrustedWorkspace"], true);
        assert_eq!(options["experimental"]["useUv"], "off");
        assert_eq!(
            options["configuration"]["environment"]["python-version"],
            "3.14"
        );
        assert_eq!(
            options["configuration"]["environment"]["extra-paths"][0],
            "/capsule/python"
        );
    }

    #[test]
    fn rust_analyzer_gets_offline_cargo_without_a_setting_that_does_not_exist() {
        // `rust-analyzer.cargo.offline` is absent from the release's emitted config schema.
        // Offline behaviour has to come from the arguments and the environment.
        let options = Server::RustAnalyzer.initialization_options();
        assert!(options["cargo"].get("offline").is_none(), "{options}");
        assert_eq!(options["cargo"]["extraArgs"][0], "--offline");
        assert_eq!(options["cargo"]["extraEnv"]["CARGO_NET_OFFLINE"], "true");
    }

    #[test]
    fn both_kinds_of_crate_authored_execution_are_disabled() {
        // The release computes build-script enablement as buildScripts OR procMacro, so
        // disabling one and leaving the other would still run crate-authored code.
        let options = Server::RustAnalyzer.initialization_options();
        assert_eq!(options["cargo"]["buildScripts"]["enable"], false);
        assert_eq!(options["procMacro"]["enable"], false);
        assert_eq!(options["checkOnSave"], false);
    }

    #[test]
    fn the_client_offers_both_encodings_and_declines_configuration_requests() {
        let capabilities = client_capabilities();
        assert_eq!(capabilities["general"]["positionEncodings"][0], "utf-8");
        assert_eq!(capabilities["general"]["positionEncodings"][1], "utf-16");
        assert_eq!(capabilities["workspace"]["configuration"], false);
    }

    #[test]
    fn neither_server_is_started_with_a_configuration_flag() {
        // Both were probed: `ty server` has no options but `--help`, and rust-analyzer needs no
        // subcommand. Settings travel over LSP, not the command line.
        assert_eq!(Server::Ty.argv(), ["/opt/producers/bin/ty", "server"]);
        assert_eq!(
            Server::RustAnalyzer.argv(),
            ["/usr/local/cargo/bin/rust-analyzer"]
        );
    }
}
