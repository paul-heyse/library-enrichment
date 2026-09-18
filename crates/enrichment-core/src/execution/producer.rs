//! Finite producer requests. Command bytes and output roots belong to the native plan.
use crate::{execution::ProbeMode, identity::Ecosystem, native_union::Rule};

pub const STABLE: &str = "1.98.1";
pub const RUSTDOC_TOOLCHAIN: &str = "nightly-2026-09-13";
pub const RUST_TARGET: &str = "x86_64-unknown-linux-gnu";
pub const RUSTDOC_RELEASE: &str = "1.100.0-nightly";
pub const RUSTDOC_COMMIT: &str = "809936eac";
pub const PYTHON_VERSION: &str = "3.14.7";
pub const TY_VERSION: &str = "0.0.80";

crate::native_vocabulary! { pub enum PreparationState {
    Ready = "ready", ProcessFailed = "process_failed", IdentityMismatch = "identity_mismatch",
} }
crate::native_struct! { pub struct PreparationVerdict {
    state: PreparationState => Rule::Text,
    detail: String => Rule::Text,
} }

crate::native_union! {
    pub enum Invocation {
        PythonIdentity = "python_identity",
        RustIdentity = "rust_identity",
        TyIdentity = "ty_identity",
        RustdocIdentity = "rustdoc_identity",
        PythonInstall = "python_install",
        RustFetch = "rust_fetch",
        Probe = "probe" { ecosystem: Ecosystem => Rule::Text, mode: ProbeMode => Rule::Text },
        LanguageServer = "language_server" { ecosystem: Ecosystem => Rule::Text },
        RuntimeObject = "runtime_object",
        RustdocFetch = "rustdoc_fetch" { source_root: String => Rule::NonEmpty },
        RustdocBuild = "rustdoc_build" {
            source_root: String => Rule::NonEmpty,
            lib_name: String => Rule::NonEmpty,
            features: Vec<String> => Rule::Set,
            default_features: bool => Rule::Text,
        },
    }
}
crate::native_struct! { pub struct Selection {
    invocation: Invocation => Rule::Text,
} }
crate::native_struct! { pub struct RustdocOptions {
    target: String => Rule::NonEmpty,
    features: Vec<String> => Rule::Set,
    default_features: bool => Rule::Text,
} }
crate::native_struct! { pub struct Command {
    mode: crate::capsule_protocol::Mode => Rule::Text,
    network: crate::capsule_protocol::Network => Rule::Text,
    argv: Vec<String> => Rule::SequenceBounds { min: 1, max: 1024 },
    files: Vec<String> => Rule::Set,
    directories: Vec<String> => Rule::Set,
    required_files: Vec<String> => Rule::Set,
} }
impl Command {
    /// Mechanical executor handoff, after native selection. No additional output policy.
    pub fn outputs(
        &self,
    ) -> std::collections::BTreeMap<String, crate::capsule_protocol::OutputKind> {
        self.files
            .iter()
            .map(|path| (path.clone(), crate::capsule_protocol::OutputKind::File))
            .chain(
                self.directories
                    .iter()
                    .map(|path| (path.clone(), crate::capsule_protocol::OutputKind::Directory)),
            )
            .collect()
    }
}

/// Exact compiled runtime observer program, shared by preparation and native admission.
pub const RUNTIME_OBJECT_HELPER: &str = include_str!("runtime_object.py");
