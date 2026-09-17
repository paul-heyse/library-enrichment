//! Reading the capability repository's own index files.
//!
//! The skill ships sixteen header-less TSVs written by a single function -- thirteen of which are
//! read here, with the three exclusions stated on [`SPECS`] -- which gives a contract
//! worth depending on: UTF-8, `\n`-terminated, whole-line lexicographically sorted, `\t` the only
//! delimiter, and no cell containing a tab, CR or LF. A naive `split('\t')` is therefore **total**
//! -- no quoting, no escaping, no edge cases.
//!
//! # Column names are not in the data
//!
//! They live in the skill's `reference.md`. This module carries its own copy in [`SPECS`] and
//! **cross-checks the field count of every row on load**, because a silently shifted column is
//! exactly the class of error this catalog exists to prevent: it would not fail, it would produce
//! confidently wrong answers.
//!
//! # Three vocabularies that must not be collapsed
//!
//! `regex.observed` is `confirmed | recorded | unknown | not-probed`; `behaviors.verdict` is
//! `confirmed | recorded | inconclusive`; `kinds.verdict` is `not-disputed | accepted | rejected`.
//! The skill's own reference says of these that "nothing should be inferred from them in either
//! direction". `not-probed` and `unknown` are different facts -- nothing was attempted, versus
//! something was attempted and did not settle -- and a reader that maps either to `false` destroys
//! the property the whole repository exists to provide. They map onto the `observed` lattice,
//! which is why that lattice exists.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// The index registry lives in `codesearch-model` so that `schema::all_tables` can derive one
/// `staging.*` table per family from the same declaration this reader validates against. Moving it
/// there is what keeps "what files exist" a single fact rather than two that can drift.
pub use codesearch_model::index_spec::{Denominator, IndexSpec, SPECS, Source, specs_for, specs_of};


/// One parsed row: column name to cell value. Empty cells are preserved as empty strings, because
/// the field count is fixed per file and a missing trailing cell would be a drift signal.
pub type Row = BTreeMap<&'static str, String>;

#[derive(Debug, thiserror::Error)]
pub enum IndexError {
    #[error("reading {0}: {1}")]
    Io(PathBuf, String),
    #[error(
        "{file} (from {flag}) line {line} has {found} fields, expected {expected} ({columns}).\nThe index schema has drifted from this reader's copy. Re-read that root's own reference rather than adjusting the parse -- a shifted column produces confidently wrong answers, not an error."
    )]
    FieldCount {
        file: String,
        /// Which root supplied it. Two roots may ship the same filename, so naming the file alone
        /// would leave a reader guessing which one to look at.
        flag: &'static str,
        line: usize,
        found: usize,
        expected: usize,
        columns: String,
    },
    #[error(
        "{file} (from {flag}) has {found} rows but that root's PROVENANCE.json records {expected}. The index and its provenance disagree; rebuild it before trusting either."
    )]
    RowCount {
        file: String,
        flag: &'static str,
        found: usize,
        expected: usize,
    },
}

/// Where the index files live inside a skill checkout.
pub fn index_dir(skill_root: &Path) -> PathBuf {
    skill_root.join("content/index")
}

/// Read one index, validating the field count of every row.
pub fn read_index(skill_root: &Path, spec: &IndexSpec) -> Result<Vec<Row>, IndexError> {
    let path = index_dir(skill_root).join(spec.file);
    let text =
        std::fs::read_to_string(&path).map_err(|e| IndexError::Io(path.clone(), e.to_string()))?;

    let mut rows = Vec::new();
    for (idx, line) in text.lines().enumerate() {
        if line.is_empty() {
            continue;
        }
        let cells: Vec<&str> = line.split('\t').collect();
        if cells.len() != spec.columns.len() {
            return Err(IndexError::FieldCount {
                file: spec.file.to_string(),
                flag: spec.source.flag(),
                line: idx + 1,
                found: cells.len(),
                expected: spec.columns.len(),
                columns: spec.columns.join(" / "),
            });
        }
        let row: Row = spec
            .columns
            .iter()
            .copied()
            .zip(cells.iter().map(|c| (*c).to_string()))
            .collect();
        rows.push(row);
    }
    Ok(rows)
}

/// The row counts the skill records for itself, from `content/PROVENANCE.json`.
///
/// Cheaper than hashing and enough to catch a half-written index. `generated_at` is deliberately
/// empty in that file, so it is never a freshness signal -- the pins and the per-file digests are.
pub fn recorded_counts(skill_root: &Path) -> Result<BTreeMap<String, usize>, IndexError> {
    let path = skill_root.join("content/PROVENANCE.json");
    let text =
        std::fs::read_to_string(&path).map_err(|e| IndexError::Io(path.clone(), e.to_string()))?;
    let value: serde_json::Value = serde_json::from_str(&text)
        .map_err(|e| IndexError::Io(path.clone(), format!("parsing JSON: {e}")))?;
    let mut out = BTreeMap::new();
    if let Some(counts) = value.pointer("/counts/index").and_then(|v| v.as_object()) {
        for (k, v) in counts {
            if let Some(n) = v.as_u64() {
                out.insert(k.clone(), n as usize);
            }
        }
    }
    Ok(out)
}

// `PinnedCrate` and `pinned_crates` lived here until the `cargo-metadata` family took over
// `program.package`. They read `pins.crates` -- a bare `name -> version` map -- and matched a
// repository by comparing a crate name to the last segment of `pins.repos`, which was a weak match
// producing `None` for most. The adapter reads the pins itself to check its subject manifest, and
// gets `repository` from cargo rather than from a name comparison. Deleted rather than kept
// unused: an unread reader is a second definition of what an input means.

/// Whether the producer that wrote this root had a resolve graph.
///
/// `cargo metadata --no-deps` sets `resolve` to null, and **a null `resolve` is not "no
/// dependencies"** (probe PL001). The adapter records which it had; this reads it back so the
/// value reaches every edge row rather than being inferred from the rows' shape.
///
/// Absent means `false`: a producer that did not say did not resolve. The opposite default would
/// let a silent omission read as a resolved graph, which is the exact failure the column exists
/// to prevent.
pub fn resolve_present(root: &Path) -> Result<bool, IndexError> {
    let path = root.join("content/PROVENANCE.json");
    let text =
        std::fs::read_to_string(&path).map_err(|e| IndexError::Io(path.clone(), e.to_string()))?;
    let value: serde_json::Value = serde_json::from_str(&text)
        .map_err(|e| IndexError::Io(path.clone(), format!("parsing JSON: {e}")))?;
    Ok(value
        .pointer("/resolve_present")
        .and_then(|v| v.as_bool())
        .unwrap_or(false))
}

/// Read one root's indexes, cross-checking each against that root's own recorded row count.
///
/// Takes the source as well as the path because `SPECS` spans both, and reading a producer's root
/// against the skill's spec list fails on the first file it does not ship. The two maps stay
/// separate for the same reason they are read separately -- see [`Source`] on stem collisions.
pub fn read_all(
    root: &Path,
    source: Source,
) -> Result<BTreeMap<&'static str, Vec<Row>>, IndexError> {
    let skill_root = root;
    let recorded = recorded_counts(skill_root)?;
    let mut out = BTreeMap::new();
    for spec in specs_of(source) {
        let rows = read_index(skill_root, spec)?;
        if let Some(expected) = recorded.get(spec.stem())
            && rows.len() != *expected
        {
            return Err(IndexError::RowCount {
                file: spec.file.to_string(),
                flag: spec.source.flag(),
                found: rows.len(),
                expected: *expected,
            });
        }
        out.insert(spec.stem(), rows);
    }
    Ok(out)
}

/// The tool identities that produced the skill's indexes, as a stable `k=v;k=v` string.
///
/// `PROVENANCE.json`'s `generated_at` is deliberately empty for byte-determinism and is never used
/// here. `serde_json` preserves object order, and the source file is itself deterministic, so the
/// join is stable without sorting -- but the sort is done anyway, because relying on someone
/// else's key order is a dependency nobody declared.
fn tool_pins(skill_root: &Path) -> Result<Vec<String>, IndexError> {
    let path = skill_root.join("content/PROVENANCE.json");
    let text =
        std::fs::read_to_string(&path).map_err(|e| IndexError::Io(path.clone(), e.to_string()))?;
    let value: serde_json::Value = serde_json::from_str(&text)
        .map_err(|e| IndexError::Io(path.clone(), format!("parsing JSON: {e}")))?;
    let mut parts: Vec<String> = Vec::new();
    if let Some(tools) = value.pointer("/tools").and_then(|v| v.as_object()) {
        for (k, v) in tools {
            if let Some(s) = v.as_str() {
                parts.push(format!("{k}={s}"));
            }
        }
    }
    parts.sort();
    if parts.is_empty() {
        parts.push("unpinned".to_string());
    }
    Ok(parts)
}

/// The interpretive context: **which tools** produced the indexes this build reads.
///
/// Distinct from the snapshot id, and the distinction is the point. Two skills carrying different
/// index content but built by the same `ast-grep` and `ripgrep` share a context and not a
/// snapshot: the frame is the same, the observations are not.
pub fn context_id_from_pins(skill_root: &Path) -> Result<String, IndexError> {
    let parts = tool_pins(skill_root)?;
    Ok(format!(
        "ctx:{}",
        &blake3::hash(parts.join(";").as_bytes()).to_hex()[..16]
    ))
}

/// A snapshot identifier derived from the skill's tool pins **and the bytes of every index file
/// this build reads**.
///
/// Derived rather than generated, so two builds of the same skill produce the same id and
/// `compare` reports no change. A timestamp here would make every rebuild look like a new world.
///
/// # Why the content and not just the pins
///
/// This used to hash `/tools` alone. That made the id a statement about the *toolchain*, so a
/// skill whose index files had changed while its tool versions had not produced the **same
/// snapshot id** — two different sets of observations wearing one identity. Every row derived from
/// them would then collide on `entity_id`, which is `blake3(entity_key ‖ snapshot_id)`, and
/// `compare` — whose entire job is to notice that the content moved — would have reported nothing.
///
/// Reading the digests from `PROVENANCE.json`'s own `files` block would have been cheaper and
/// wrong: that file records what the skill's builder *wrote*, and a copy edited afterwards still
/// carries the original digests. What this build can honestly identify is what it actually read.
pub fn snapshot_id_from_inputs(roots: &[(&Path, Source)]) -> Result<String, IndexError> {
    let mut parts: Vec<String> = Vec::new();
    // EVERY root, in the order given, and every file each one contributes. The identity has to
    // cover the bytes actually read or it is a lie: `entity_id` is `blake3(entity_key ‖
    // snapshot_id)`, so two different input sets wearing one id collide row for row and `compare`
    // reports nothing. Adding a second root without adding it here would be exactly that.
    for (root, source) in roots {
        parts.extend(tool_pins(root)?);
        // `specs_of` is a fixed, ordered list, so the inputs are enumerated the same way every
        // time and a file the build does not read cannot change the identity of what it produced.
        for spec in specs_of(*source) {
            let path = index_dir(root).join(spec.file);
            let bytes =
                std::fs::read(&path).map_err(|e| IndexError::Io(path.clone(), e.to_string()))?;
            parts.push(format!("{}={}", spec.file, blake3::hash(&bytes).to_hex()));
        }
    }
    Ok(format!(
        "snap:{}",
        &blake3::hash(parts.join(";").as_bytes()).to_hex()[..16]
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The skill this crate lives inside. Derived from the crate's own location, never from a
    /// host-repository path, so the tree stays copyable.
    fn skill_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../..")
            .canonicalize()
            .expect("the skill root is three levels above the crate")
    }

    /// The snapshot id is a function of the inputs, so a rebuild is not a new world.
    #[test]
    fn the_snapshot_id_is_derived_from_the_inputs_and_is_stable() {
        let root = skill_root();
        let a =
            snapshot_id_from_inputs(&[(root.as_path(), Source::Skill)]).expect("inputs readable");
        let b =
            snapshot_id_from_inputs(&[(root.as_path(), Source::Skill)]).expect("inputs readable");
        assert_eq!(a, b, "two reads of the same skill must agree");
        assert!(a.starts_with("snap:"));
    }

    /// Changing one index byte changes the snapshot, while leaving the context alone.
    ///
    /// This is the property the fixture in phase 7 rests on, and it is the one the pins-only
    /// derivation did not have: without it a modified copy of the skill shares an id with the
    /// original and `compare` has nothing to compare.
    #[test]
    fn an_edited_index_is_a_different_snapshot_but_the_same_context() {
        let root = skill_root();
        let scratch = tempfile::tempdir().expect("temp dir");
        let copy = scratch.path().join("skill");
        copy_tree(&root.join("content"), &copy.join("content")).expect("copy the skill's content");

        assert_eq!(
            snapshot_id_from_inputs(&[(root.as_path(), Source::Skill)]).unwrap(),
            snapshot_id_from_inputs(&[(copy.as_path(), Source::Skill)]).unwrap(),
            "an unmodified copy is the same snapshot, or the digest is reading something it should not"
        );

        let edited = index_dir(&copy).join("regex.tsv");
        let mut text = std::fs::read_to_string(&edited).expect("read the copy");
        text.push_str("probe-only\tx\tno\tno\tno\t\t\tnot-probed\tadded by a test\n");
        std::fs::write(&edited, text).expect("write the copy");

        assert_ne!(
            snapshot_id_from_inputs(&[(root.as_path(), Source::Skill)]).unwrap(),
            snapshot_id_from_inputs(&[(copy.as_path(), Source::Skill)]).unwrap(),
            "an edited index must be a different snapshot"
        );
        assert_eq!(
            context_id_from_pins(&root).unwrap(),
            context_id_from_pins(&copy).unwrap(),
            "editing an index does not change which tools produced it"
        );
    }

    /// Copy a directory tree. Only used by the test above.
    fn copy_tree(from: &Path, to: &Path) -> std::io::Result<()> {
        std::fs::create_dir_all(to)?;
        for entry in std::fs::read_dir(from)? {
            let entry = entry?;
            let target = to.join(entry.file_name());
            if entry.file_type()?.is_dir() {
                copy_tree(&entry.path(), &target)?;
            } else {
                std::fs::copy(entry.path(), target)?;
            }
        }
        Ok(())
    }

    #[test]
    fn every_index_parses_with_the_documented_field_count() {
        let root = skill_root();
        for spec in specs_of(Source::Skill) {
            let rows = read_index(&root, spec).unwrap_or_else(|e| panic!("{}: {e}", spec.file));
            assert!(!rows.is_empty(), "{} is empty", spec.file);
            for row in &rows {
                assert_eq!(
                    row.len(),
                    spec.columns.len(),
                    "{} row has the wrong arity",
                    spec.file
                );
            }
        }
    }

    #[test]
    fn row_counts_match_the_skills_own_provenance() {
        let root = skill_root();
        let all = read_all(&root, Source::Skill).expect("every index agrees with PROVENANCE.json");
        // Spot-check a few counts against the documented figures so a silent truncation of the
        // provenance file itself cannot make this test vacuous.
        assert_eq!(all["flags"].len(), 193);
        assert_eq!(all["regex"].len(), 44);
        assert_eq!(all["behaviors"].len(), 49);
        assert_eq!(all["rule-fields"].len(), 70);
    }

    /// The field-count check must actually fire. A validator that cannot fail proves nothing.
    #[test]
    fn a_short_row_is_rejected() {
        let dir = tempfile::tempdir().expect("temp dir");
        let index = dir.path().join("content/index");
        std::fs::create_dir_all(&index).expect("mkdir");
        // `flags` wants eight columns; give it three.
        std::fs::write(index.join("flags.tsv"), "rg\tsearch\t--pcre2\n").expect("write");

        let spec = SPECS.iter().find(|s| s.file == "flags.tsv").expect("flags");
        let err = read_index(dir.path(), spec).expect_err("a short row must be rejected");
        assert!(
            matches!(
                err,
                IndexError::FieldCount {
                    found: 3,
                    expected: 8,
                    ..
                }
            ),
            "unexpected error: {err}"
        );
    }

    /// The vocabularies that must survive intact. This asserts the real data still only contains
    /// values the `observed` lattice knows, so a new upstream value surfaces here rather than
    /// being silently mapped to the nearest known rank.
    #[test]
    fn observed_and_verdict_vocabularies_are_known_to_the_lattice() {
        let root = skill_root();
        let regex = read_index(&root, SPECS.iter().find(|s| s.file == "regex.tsv").unwrap())
            .expect("regex index");
        for row in &regex {
            let value = &row["observed"];
            assert!(
                codesearch_bridge::lattice::rank_of("observed", value).is_some(),
                "regex.tsv carries observed={value:?}, which the `observed` lattice does not \
                 model. Add it to the lattice rather than mapping it onto a neighbour."
            );
        }

        let behaviors = read_index(
            &root,
            SPECS.iter().find(|s| s.file == "behaviors.tsv").unwrap(),
        )
        .expect("behaviors index");
        for row in &behaviors {
            let value = &row["verdict"];
            // `inconclusive` is a first-class outcome in the skill and is NOT a failure. It maps
            // to the lattice's `unknown`, one rank above `not-probed`.
            let mapped = match value.as_str() {
                "inconclusive" => "unknown",
                other => other,
            };
            assert!(
                codesearch_bridge::lattice::rank_of("observed", mapped).is_some(),
                "behaviors.tsv carries verdict={value:?}, which does not map to the lattice"
            );
        }
    }
}
