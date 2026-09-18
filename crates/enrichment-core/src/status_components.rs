//! Finite component declarations consumed by the native status plan and configuration view.
use crate::native_union::Rule;

crate::native_vocabulary! { pub enum Kind { Producer = "producer", Feature = "feature" } }
crate::native_vocabulary! { pub enum Requirement {
    Store = "store", PythonWorker = "python_worker", RustExecution = "rust_execution",
    PythonExecution = "python_execution", AnyExecution = "any_execution",
} }
crate::native_struct! { pub struct Definition {
    ordinal: u32 => Rule::Coordinate(crate::native_union::Unit::Ordinal),
    kind: Kind => Rule::Text,
    name: String => Rule::NonEmpty,
    requirement: Requirement => Rule::Text,
    version: String => Rule::NonEmpty,
    detail: String => Rule::NonEmpty,
} }

pub fn declarations() -> Vec<Definition> {
    use Kind::{Feature, Producer};
    use Requirement::{AnyExecution, PythonExecution, PythonWorker, RustExecution, Store};
    [
        (Producer, "rustdoc-json", Store, crate::producer::rustdoc::NORMALIZER_VERSION,
            "Hosted docs.rs rustdoc JSON normalized by the matching native decoder; signatures use the pinned public-api renderer"),
        (Producer, "crates-io-registry", Store, crate::producer::cratesio::VERSION,
            "Sparse index, exact version records and crate tarballs retained as immutable artifacts"),
        (Producer, "griffe", PythonWorker, crate::producer::python::worker::GRIFFE,
            "Pinned static worker completed a validated job in this daemon process"),
        (Producer, "pypi-registry", Store, crate::producer::python::VERSION,
            "Exact distribution selection, bounded archive inspection and immutable snapshots"),
        (Producer, "rust-analyzer", RustExecution, crate::execution::producer::STABLE,
            "Explicit semantic queries use the admitted Rust route; build scripts, proc macros and check-on-save remain disabled"),
        (Producer, "ty", PythonExecution, "0.0.80",
            "Explicit typecheck and semantic queries use the admitted Python route"),
        (Feature, "evidence-search", Store, crate::SCHEMA_VERSION,
            "Native lexical ranking over exact snapshots with recorded factors and bound cursors"),
        (Feature, "release-comparison", Store, crate::SCHEMA_VERSION,
            "Typed API, document and configuration comparison over exact snapshot pairs"),
        (Feature, "usage-verification", AnyExecution, crate::SCHEMA_VERSION,
            "Compile, typecheck and runtime probes use operator-enabled, qualified execution routes; each request requires its own grant"),
        (Feature, "jobs", Store, crate::SCHEMA_VERSION,
            "Durable native claims, bounded queue, independent client interests and physical cleanup"),
    ].into_iter().enumerate().map(|(ordinal, (kind, name, requirement, version, detail))| Definition {
        ordinal: ordinal as u32, kind, name: name.into(), requirement,
        version: version.into(), detail: detail.into(),
    }).collect()
}

/// No service exists in the configuration-only CLI view. This uniform absence projection
/// performs no availability inference; an open runtime uses the native component plan.
pub fn without_store(kind: Kind) -> Vec<crate::wire::status::ComponentStatus> {
    declarations()
        .into_iter()
        .filter(|item| item.kind == kind)
        .map(|item| crate::wire::status::ComponentStatus {
            name: item.name,
            available: false,
            version: None,
            detail: "No evidence store is open in this process.".into(),
        })
        .collect()
}
