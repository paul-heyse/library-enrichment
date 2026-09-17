//! Emit `cargo metadata` for a skill's pinned crates, as a skill-shaped directory.
//!
//! §3.1 is the first of §6.2's passes and the cheapest of the eight families: `cargo metadata` is
//! the one producer that compiles nothing. This writes what `codesearch-build` reads.
//!
//!     cargo-metadata-adapter --skill-root <skill> --out $CODESEARCH_HOME/producers/cargo-metadata
//!
//! # Why a directory rather than a library call
//!
//! `codesearch-build` resolves twelve dependencies. Linking the producers into it would add
//! `cargo_metadata` here, `ra_ap_*` next -- 223 crates and 1.7 GB of build artifacts, measured --
//! and `rustc_public` after that, each needing its own toolchain. The seam is the filesystem
//! instead: headerless TSVs plus a `PROVENANCE.json`, in exactly the shape a skill ships, read
//! through `index.rs` with no second reader.
//!
//! # Why it resolves, rather than running `--no-deps` on each vendored crate
//!
//! The first version did the latter, and two things were wrong with it, both measured:
//!
//! 1. **`source` came back null.** A registry crate read straight out of `~/.cargo/registry/src`
//!    is a PATH package as far as cargo is concerned -- its id is `path+file:///…`. §1.2's key
//!    wants the registry, and minting `pkg:path/ignore@0.4.29` would record how this adapter read
//!    the crate rather than what the crate is.
//! 2. **`resolve` was null**, and probe PL001 measured that a null resolve is not "no
//!    dependencies". Every edge then carried `resolve_present = false` and §7.3's closure
//!    correctly refused to traverse -- correct, and useless.
//!
//! So `subject/Cargo.toml` pins all 23 with `=` and is resolved instead. Its lockfile is
//! committed, the pins are asserted against the skill's own `pins.crates` before anything is
//! emitted, and the resolve is offline after the first fetch.
//!
//! The synthetic root package is **excluded**. It exists to give cargo something to resolve and is
//! an artifact of the method, not a fact about the subject -- the same reason `src_path` is
//! emitted relative to the crate root rather than as a path under this machine's `$HOME`.

use std::collections::{BTreeMap, BTreeSet};
use std::io::Write as _;
use std::path::PathBuf;

use cargo_metadata::{DependencyKind, Metadata, Package, PackageId};

/// The synthetic workspace root, excluded from everything it produces.
const SUBJECT_ROOT: &str = "cargo-metadata-subject";

/// A cell that cannot break the TSV contract: no tabs, no newlines.
///
/// The index format is headerless and tab-separated, so either would shift every later column --
/// which `read_index`'s field-count check would catch as schema drift, blaming the reader for an
/// unescaped value. Replacing rather than escaping keeps the format simple and the failure
/// impossible.
fn cell(value: &str) -> String {
    value.replace(['\t', '\n', '\r'], " ")
}

/// §1.2's registry segment: `crates.io`, or `path` for a local one.
///
/// Normalised rather than emitted raw. §1.2's example is `pkg:crates.io/serde@1.0.200`, and the
/// full index URL would put a transport and a host into every key for no gain -- keys are compared,
/// joined and read by people.
fn registry_of(package: &Package) -> &'static str {
    match package.source.as_ref() {
        Some(source) if source.repr.contains("crates.io") => "crates.io",
        Some(_) => "registry",
        None => "path",
    }
}

/// `pkg:<registry>/<name>@<version>` -- §1.2, with the source segment it specifies.
fn package_key(package: &Package) -> String {
    format!(
        "pkg:{}/{}@{}",
        registry_of(package),
        package.name,
        package.version
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut skill_root: Option<PathBuf> = None;
    let mut out: Option<PathBuf> = None;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--skill-root" => skill_root = args.next().map(PathBuf::from),
            "--out" => out = args.next().map(PathBuf::from),
            other => return Err(format!("unknown argument `{other}`").into()),
        }
    }
    let skill_root = skill_root.ok_or("--skill-root is required and is never inferred")?;
    let out = out.ok_or("--out is required")?;

    // The subject manifest lives beside this binary's source, not beside the skill: it is this
    // adapter's own statement of what it reads.
    let subject = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("subject/Cargo.toml");
    let metadata: Metadata = cargo_metadata::MetadataCommand::new()
        .manifest_path(&subject)
        .other_options(["--offline".to_string()])
        .exec()?;

    // ---- the pins must still be what the skill says they are ---------------------------------
    //
    // Asserted before anything is emitted. A subject manifest that drifted from `pins.crates`
    // would describe a different set of crates under the same snapshot, and the first symptom
    // would be `library-api` and this family disagreeing about a version.
    let provenance = skill_root.join("content/PROVENANCE.json");
    let value: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&provenance)?)?;
    let pins: BTreeMap<String, String> = value
        .pointer("/pins/crates")
        .and_then(|v| v.as_object())
        .ok_or("the skill ships no `pins.crates` object, so there is no subject set to check")?
        .iter()
        .filter_map(|(k, v)| Some((k.clone(), v.as_str()?.to_string())))
        .collect();

    let resolved: BTreeMap<&str, &Package> = metadata
        .packages
        .iter()
        .map(|p| (p.name.as_str(), p))
        .collect();
    let mut drift: Vec<String> = Vec::new();
    for (name, version) in &pins {
        match resolved.get(name.as_str()) {
            Some(p) if p.version.to_string() == *version => {}
            Some(p) => drift.push(format!("{name}: skill pins {version}, subject resolved {}", p.version)),
            None => drift.push(format!("{name}: skill pins {version}, subject resolved nothing")),
        }
    }
    if !drift.is_empty() {
        return Err(format!(
            "subject/Cargo.toml has drifted from the skill's `pins.crates`:\n  {}\nRegenerate it \
             from the skill rather than adjusting one side.",
            drift.join("\n  ")
        )
        .into());
    }

    // ---- emit --------------------------------------------------------------------------------
    let by_id: BTreeMap<&PackageId, &Package> =
        metadata.packages.iter().map(|p| (&p.id, p)).collect();
    let pinned: BTreeSet<&str> = pins.keys().map(String::as_str).collect();

    let mut packages: Vec<String> = Vec::new();
    let mut units: Vec<String> = Vec::new();
    let mut features: Vec<String> = Vec::new();
    let mut edges: Vec<String> = Vec::new();

    for package in &metadata.packages {
        if package.name.as_str() == SUBJECT_ROOT {
            continue;
        }
        let key = package_key(package);
        let name = package.name.to_string();
        let version = package.version.to_string();
        packages.push(
            [
                cell(&key),
                cell(&name),
                cell(&version),
                cell(registry_of(package)),
                cell(package.repository.as_deref().unwrap_or_default()),
                cell(&package.edition.to_string()),
                cell(&package.rust_version.as_ref().map(|v| v.to_string()).unwrap_or_default()),
                cell(package.license.as_deref().unwrap_or_default()),
                cell(if pinned.contains(name.as_str()) { "true" } else { "false" }),
            ]
            .join("\t"),
        );

        for target in &package.targets {
            let kind = target.kind.iter().map(|k| k.to_string()).collect::<Vec<_>>().join("+");
            units.push(
                [
                    cell(&key),
                    cell(&target.name),
                    cell(&kind),
                    // RELATIVE to the crate root, never absolute. The snapshot id digests these
                    // bytes, so an absolute path under `$HOME` would give identical inputs a
                    // different identity on every machine.
                    cell(
                        target
                            .src_path
                            .as_std_path()
                            .strip_prefix(package.manifest_path.parent().unwrap().as_std_path())
                            .unwrap_or(target.src_path.as_std_path())
                            .to_string_lossy()
                            .as_ref(),
                    ),
                    cell(&target.edition.to_string()),
                ]
                .join("\t"),
            );
        }

        for (feature, implies) in &package.features {
            features.push(
                [
                    cell(&key),
                    cell(feature),
                    // Semicolon-joined rather than a list column: the reader's `Row` is flat, and
                    // the build splits it back into a `List<Utf8>`.
                    cell(&implies.join(";")),
                ]
                .join("\t"),
            );
        }
    }

    // Resolved edges, from `resolve.nodes`. This is the graph §7.3's closure walks, and it exists
    // only because the subject is resolved rather than read with `--no-deps`.
    let resolve = metadata
        .resolve
        .as_ref()
        .ok_or("the subject resolved no graph; `--no-deps` must not be set here")?;
    for node in &resolve.nodes {
        let Some(from) = by_id.get(&node.id) else {
            continue;
        };
        if from.name.as_str() == SUBJECT_ROOT {
            continue;
        }
        let from_key = package_key(from);
        for dep in &node.deps {
            let Some(to) = by_id.get(&dep.pkg) else {
                continue;
            };
            // A dependency may be resolved for several kinds and targets at once; each is its own
            // edge, because `normal` and `dev` are different facts about the same pair.
            for kind in &dep.dep_kinds {
                let kind_name = match kind.kind {
                    DependencyKind::Normal => "normal",
                    DependencyKind::Development => "dev",
                    DependencyKind::Build => "build",
                    _ => "unknown",
                };
                // The declared requirement, matched by name and kind. Cargo resolves one package
                // per (name, kind, target), so this is exact rather than best-effort.
                let req = from
                    .dependencies
                    .iter()
                    .find(|d| d.name == to.name.as_str() && d.kind == kind.kind)
                    .map(|d| d.req.to_string())
                    .unwrap_or_default();
                edges.push(
                    [
                        cell(&from_key),
                        cell(&package_key(to)),
                        cell(&to.name),
                        cell(&req),
                        cell(kind_name),
                        cell(&kind.target.as_ref().map(|t| t.to_string()).unwrap_or_default()),
                    ]
                    .join("\t"),
                );
            }
        }
    }

    let index_dir = out.join("content/index");
    std::fs::create_dir_all(&index_dir)?;
    let write = |name: &str, rows: &[String]| -> std::io::Result<()> {
        let mut f = std::fs::File::create(index_dir.join(name))?;
        for row in rows {
            writeln!(f, "{row}")?;
        }
        Ok(())
    };
    write("packages.tsv", &packages)?;
    write("crate_units.tsv", &units)?;
    write("features.tsv", &features)?;
    write("dep_edges.tsv", &edges)?;

    let cargo = String::from_utf8_lossy(
        &std::process::Command::new("cargo").arg("--version").output()?.stdout,
    )
    .trim()
    .to_string();
    let counts: BTreeMap<&str, usize> = [
        ("packages", packages.len()),
        ("crate_units", units.len()),
        ("features", features.len()),
        ("dep_edges", edges.len()),
    ]
    .into_iter()
    .collect();
    let provenance_out = serde_json::json!({
        "counts": {"index": counts},
        "tools": {"cargo": cargo, "adapter": concat!("cargo-metadata-adapter@", env!("CARGO_PKG_VERSION"))},
        // True, and it is what lets §7.3's dependency closure traverse at all.
        "resolve_present": true,
        "subject": {
            // The skill's NAME, not the path this run was given: where a copy sits is not a fact
            // about it, and the snapshot id digests this file.
            "skill": skill_root.canonicalize().ok()
                .and_then(|p| p.file_name().map(|n| n.to_string_lossy().into_owned()))
                .unwrap_or_else(|| "unknown".to_string()),
            "crates_pinned": pins.len(),
            "packages_resolved": packages.len(),
        },
    });
    std::fs::write(
        out.join("content/PROVENANCE.json"),
        serde_json::to_string_pretty(&provenance_out)? + "\n",
    )?;

    println!("out       {}", out.display());
    println!("subject   {} pinned crates, all resolving at their pin", pins.len());
    println!("wrote     {:>5}  packages.tsv    ({} of them the skill pinned)", packages.len(), pins.len());
    println!("wrote     {:>5}  crate_units.tsv", units.len());
    println!("wrote     {:>5}  features.tsv", features.len());
    println!("wrote     {:>5}  dep_edges.tsv   (resolved; resolve_present=true)", edges.len());
    Ok(())
}
