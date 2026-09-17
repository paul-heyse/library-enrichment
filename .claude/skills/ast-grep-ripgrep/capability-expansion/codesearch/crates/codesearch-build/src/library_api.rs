//! The `library-api` extraction family: the search tools' own Rust API, from the skill's indexes.
//!
//! This is the second extraction family, and the first that is not about the tools' user-facing
//! surface. It reads the rustdoc-derived index the skill ships -- `symbols`, `methods`, `impls`,
//! `aliases`, `unresolved` -- and writes `program.*`. Offline, no toolchain, no network.
//!
//! # What this family cannot know, and says so
//!
//! `mapping_basis` is **`skill.index.symbols`**, never `rustdoc.json.v<n>`. The distinction is not
//! pedantry: evidence 06 established that `format_version` must be read per payload, and a TSV
//! carries none. Claiming a rustdoc format version here would assert something about a payload
//! this family never saw.
//!
//! Three whole columns stay NULL by construction, and [`Columns::never_fills`] makes writing them
//! a build error rather than a review comment:
//!
//! - `program.definition.visibility` / `.visibility_basis` -- §3.5 keeps these explicit because
//!   absence has three indistinguishable causes (not public, `cfg`-gated off, private module). An
//!   index of public items records which of the three applies to exactly nothing.
//! - every `source_anchor`-shaped column -- no byte offsets exist anywhere in the index.
//!
//! # Two keys degrade, and the degradation is detected rather than absorbed
//!
//! `impl:` keys lose the source anchor §1.2 embeds in them, and signature keys have to carry
//! `via_trait` because one owner can hold the same method name by several routes. Both are minted
//! from printed paths, so both can in principle collide. [`Collisions`] turns a collision into a
//! **build failure naming both sides**, because a collision is a finding about the index -- two
//! things this model cannot tell apart -- and merging them silently is the one response that
//! destroys the information.

use std::collections::{BTreeMap, BTreeSet};

use arrow::array::RecordBatch;
use codesearch_bridge::identity;

use crate::catalog::{CatalogError, Columns, Common, RunContext, push_common};
use crate::index::Row;

/// The provenance vocabulary this family may emit. Closed, not free text (§3 part 3).
const MAPPING_BASIS: &str = "skill.index.symbols";

/// Columns this family must leave NULL. See the module docs.
const NEVER_FILLS: &[&str] = &[
    "visibility",
    "visibility_basis",
    "source_anchor_id",
    "anchor_id",
    "source",
];

/// Every batch this family produces.
pub struct LibraryApiBatches {
    pub definition: RecordBatch,
    pub signature: RecordBatch,
    pub implementation: RecordBatch,
    pub export_path: RecordBatch,
    pub native_binding: RecordBatch,
}

/// A key that two different rows both claim.
///
/// Recorded rather than deduplicated: `impls.tsv` has no anchor, so if two impls print the same
/// the index genuinely cannot distinguish them, and the honest report is that fact.
struct Collisions {
    what: &'static str,
    seen: BTreeSet<String>,
    duplicate: Option<String>,
}

impl Collisions {
    fn new(what: &'static str) -> Self {
        Self {
            what,
            seen: BTreeSet::new(),
            duplicate: None,
        }
    }

    fn observe(&mut self, key: &str) {
        if !self.seen.insert(key.to_string()) && self.duplicate.is_none() {
            self.duplicate = Some(key.to_string());
        }
    }

    fn into_result(self) -> Result<(), CatalogError> {
        match self.duplicate {
            None => Ok(()),
            Some(key) => Err(CatalogError::KeyCollision {
                what: self.what.to_string(),
                key,
            }),
        }
    }
}

/// Everything already minted, so later tables can resolve references into `entity_id`s.
///
/// The id-vs-key rule (§1.1): anything authored by a person references an `entity_key`; anything
/// derived within a snapshot references an `entity_id`. These are all derived.
struct Definitions {
    /// canonical path -> its `entity_id`
    by_path: BTreeMap<String, String>,
}

impl Definitions {
    fn id_of(&self, path: &str) -> Option<String> {
        self.by_path.get(path).cloned()
    }
}

/// The owning path of a canonical path: everything before the last `::` segment.
///
/// A derived fact, not an observed one -- but a safe one, because Rust paths are `::`-separated by
/// definition. It is stored beside a nullable `owner_id` precisely because the *owner* usually is
/// not observed: `symbols.tsv` holds items, not modules.
fn owner_path(canonical_path: &str) -> Option<&str> {
    canonical_path.rfind("::").map(|i| &canonical_path[..i])
}

/// # This family no longer writes `program.package`
///
/// It used to, minting `pkg:<name>@<version>` from the skill's `pins.crates` -- a bare
/// `name -> version` map with no source, so §1.2's registry segment was missing and `source` was
/// never-filled. The `cargo-metadata` family resolves a pinned subject and recovers both, so the
/// table moved there and one of the three recorded key degradations is retired. Nothing here
/// referenced a package key: `program.definition` carries `crate_name`, not a key.
pub fn build(
    indexes: &BTreeMap<&'static str, Vec<Row>>,
    run: RunContext<'_>,
) -> Result<LibraryApiBatches, CatalogError> {
    // Definitions first: every other table resolves references against them.
    let (definition, defs) = build_definitions(indexes, run)?;
    Ok(LibraryApiBatches {
        definition,
        signature: build_signatures(indexes, &defs, run)?,
        implementation: build_implementations(indexes, &defs, run)?,
        export_path: build_export_paths(indexes, &defs, run)?,
        native_binding: build_native_bindings(indexes, run)?,
    })
}

fn build_definitions(
    indexes: &BTreeMap<&'static str, Vec<Row>>,
    run: RunContext<'_>,
) -> Result<(RecordBatch, Definitions), CatalogError> {
    let schema = codesearch_model::schema::program_definition();
    let mut cols = Columns::never_fills(NEVER_FILLS);
    let mut collisions = Collisions::new("program.definition");

    // Two passes. The first mints every key so the second can resolve `owner_id` against the same
    // set -- an owner that appears later in the file is still an owner.
    let rows: Vec<&Row> = indexes.get("symbols").into_iter().flatten().collect();
    let mut by_path = BTreeMap::new();
    for row in &rows {
        let key = identity::definition_key(&row["crate"], &row["canonical_path"]);
        collisions.observe(&key);
        by_path.insert(
            row["canonical_path"].clone(),
            identity::entity_id(&key, run.snapshot_id),
        );
    }
    collisions.into_result()?;
    let defs = Definitions { by_path };

    for row in rows {
        let path = &row["canonical_path"];
        let key = identity::definition_key(&row["crate"], path);
        push_common(
            &mut cols,
            run,
            Common {
                entity_key: &key,
                precision: "exact",
                observed: "recorded",
                coverage: "characterised",
                ord: None,
            },
        )?;
        cols.str_col("canonical_path", path.clone());
        cols.str_col("item_kind", row["kind"].clone());
        cols.str_col("crate_name", row["crate"].clone());
        let owner = owner_path(path);
        cols.str_col("owner_path", owner.map(str::to_string));
        // Resolves only where the owning path is ITSELF an indexed item. Modules are not in the
        // index, so most of these are null and §7.3's containment closure is shallow. That is the
        // honest shape: synthesising module rows would invent entities nobody extracted.
        cols.str_col("owner_id", owner.and_then(|o| defs.id_of(o)));
        cols.str_col("api_page", empty_to_none(&row["api_page"]));
        cols.str_col("summary", empty_to_none(&row["summary"]));
        cols.int_col("alias_count", row["alias_count"].parse::<i64>().ok());
        cols.int_col("method_count", row["method_count"].parse::<i64>().ok());
        cols.str_col("visibility", None);
        cols.str_col("visibility_basis", None);
        cols.str_col("source_anchor_id", None);
        cols.end_row();
    }
    Ok((cols.build(&schema)?, defs))
}

fn build_signatures(
    indexes: &BTreeMap<&'static str, Vec<Row>>,
    defs: &Definitions,
    run: RunContext<'_>,
) -> Result<RecordBatch, CatalogError> {
    let schema = codesearch_model::schema::program_signature();
    let mut cols = Columns::never_fills(NEVER_FILLS);
    let mut collisions = Collisions::new("program.signature");

    for row in indexes.get("methods").into_iter().flatten() {
        // `-` is the index's spelling for "inherent". Storing the sentinel would make a magic
        // string out of something the column holds honestly as null.
        let via_trait = if row["via_trait"] == "-" {
            None
        } else {
            Some(row["via_trait"].as_str())
        };
        let key = identity::signature_key(
            &row["owner_path"],
            &row["method"],
            via_trait,
            &row["signature"],
        );
        collisions.observe(&key);
        push_common(
            &mut cols,
            run,
            Common {
                entity_key: &key,
                precision: "exact",
                observed: "recorded",
                coverage: "characterised",
                ord: None,
            },
        )?;
        cols.str_col("owner_path", row["owner_path"].clone());
        cols.str_col("owner_id", defs.id_of(&row["owner_path"]));
        cols.str_col("method_name", row["method"].clone());
        cols.str_col("via_trait", via_trait.map(str::to_string));
        cols.str_col("signature_text", row["signature"].clone());
        cols.str_col("summary", empty_to_none(&row["summary"]));
        cols.end_row();
    }
    collisions.into_result()?;
    cols.build(&schema)
}

fn build_implementations(
    indexes: &BTreeMap<&'static str, Vec<Row>>,
    defs: &Definitions,
    run: RunContext<'_>,
) -> Result<RecordBatch, CatalogError> {
    let schema = codesearch_model::schema::program_implementation();
    let mut cols = Columns::never_fills(NEVER_FILLS);
    let mut collisions = Collisions::new("program.implementation");

    for row in indexes.get("impls").into_iter().flatten() {
        let key = identity::implementation_key(
            &row["crate"],
            &row["trait_path"],
            &row["implementor_path"],
        );
        collisions.observe(&key);
        push_common(
            &mut cols,
            run,
            Common {
                entity_key: &key,
                // INEXACT, and this is the one place in the family where it is. §1.2 wants a source
                // anchor in an `impl:` key so two impls for similarly-printed types cannot be
                // confused; the index has no offsets, so the key is the printed pair and nothing
                // more. `Collisions` proves none of the 2,241 actually collide -- but "no collision
                // observed" is not "cannot collide", and the lattice is where that difference goes.
                precision: "inexact",
                observed: "recorded",
                coverage: "characterised",
                ord: None,
            },
        )?;
        cols.str_col("trait_path", row["trait_path"].clone());
        cols.str_col("implementor_path", row["implementor_path"].clone());
        cols.str_col("crate_name", row["crate"].clone());
        cols.str_col("implementor_id", defs.id_of(&row["implementor_path"]));
        cols.str_col("anchor_id", None);
        cols.end_row();
    }
    collisions.into_result()?;
    cols.build(&schema)
}

fn build_export_paths(
    indexes: &BTreeMap<&'static str, Vec<Row>>,
    defs: &Definitions,
    run: RunContext<'_>,
) -> Result<RecordBatch, CatalogError> {
    let schema = codesearch_model::schema::program_export_path();
    let mut cols = Columns::never_fills(NEVER_FILLS);
    let mut collisions = Collisions::new("program.export_path");

    for row in indexes.get("aliases").into_iter().flatten() {
        let key = identity::export_path_key(&row["access_path"]);
        collisions.observe(&key);
        push_common(
            &mut cols,
            run,
            Common {
                entity_key: &key,
                precision: "exact",
                observed: "recorded",
                coverage: "characterised",
                ord: None,
            },
        )?;
        cols.str_col("access_path", row["access_path"].clone());
        cols.str_col("canonical_path", row["canonical_path"].clone());
        cols.str_col("item_kind", empty_to_none(&row["kind"]));
        cols.str_col("definition_id", defs.id_of(&row["canonical_path"]));
        cols.end_row();
    }
    collisions.into_result()?;
    cols.build(&schema)
}

fn build_native_bindings(
    indexes: &BTreeMap<&'static str, Vec<Row>>,
    run: RunContext<'_>,
) -> Result<RecordBatch, CatalogError> {
    let schema = codesearch_model::schema::snapshot_native_binding();
    let mut cols = Columns::never_fills(NEVER_FILLS);

    for row in indexes.get("unresolved").into_iter().flatten() {
        // The whole cell is the handle. `unresolved.tsv` has ONE column, holding
        // `ast_grep_core::tree_sitter::TSLanguage -> tree_sitter::Language` as text: an access path
        // and the out-of-set target it reaches. §1.3 says a native handle is "opaque; meaningful
        // only with run_id", and splitting this one into two columns the schema does not have
        // would lose the half that did not fit.
        let handle = &row["access_path"];
        let key = identity::native_binding_key("skill.index.reexport", handle);
        push_common(
            &mut cols,
            run,
            Common {
                entity_key: &key,
                precision: "exact",
                observed: "recorded",
                coverage: "characterised",
                ord: None,
            },
        )?;
        cols.str_col("native_namespace", "skill.index.reexport".to_string());
        cols.str_col("native_handle", handle.clone());
        // Null, and that is the whole point of the row. These re-exports leave the indexed set, so
        // no canonical entity exists for them to bind to. An unbound handle is a recorded fact --
        // it is how extraction coverage is measured -- not a failure to be cleaned up.
        cols.str_col("canonical_entity_id", None);
        cols.str_col("canonical_entity_key", None);
        cols.rank_col(
            "binding_rank",
            codesearch_bridge::lattice::rank_of("binding", "unbound").unwrap_or(0),
        );
        cols.str_col("mapping_basis", MAPPING_BASIS.to_string());
        cols.end_row();
    }
    cols.build(&schema)
}

fn empty_to_none(value: &str) -> Option<String> {
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_owner_path_is_the_canonical_path_minus_its_last_segment() {
        assert_eq!(
            owner_path("aho_corasick::ahocorasick::AhoCorasick"),
            Some("aho_corasick::ahocorasick")
        );
        // A crate root has no owner, and saying `Some("")` would invent one.
        assert_eq!(owner_path("memchr"), None);
    }

    /// The rule the `Collisions` type exists for.
    /// The never-fills rule, with a deliberately miswired column.
    ///
    /// §3 part 4 makes this a build-time assertion rather than a comment, so the test that matters
    /// is the one where an adapter writes a column its family cannot know and the build refuses.
    /// Both arms are here: the honest null passes, the fabricated value fails.
    #[test]
    fn writing_a_column_this_family_cannot_know_fails_the_build() {
        let schema = codesearch_model::schema::program_definition();
        let run = RunContext {
            snapshot_id: "snap:test",
            run_id: "run:test",
        };

        let row = |cols: &mut Columns, visibility: Option<String>| -> Result<(), CatalogError> {
            push_common(
                cols,
                run,
                Common {
                    entity_key: "def:c#p",
                    precision: "exact",
                    observed: "recorded",
                    coverage: "characterised",
                    ord: None,
                },
            )?;
            cols.str_col("canonical_path", "p".to_string());
            cols.str_col("item_kind", "struct".to_string());
            cols.str_col("crate_name", "c".to_string());
            cols.str_col("owner_path", None);
            cols.str_col("owner_id", None);
            cols.str_col("api_page", None);
            cols.str_col("summary", None);
            cols.int_col("alias_count", None);
            cols.int_col("method_count", None);
            cols.str_col("visibility", visibility);
            cols.str_col("visibility_basis", None);
            cols.str_col("source_anchor_id", None);
            cols.end_row();
            Ok(())
        };

        // A CONTROL -- the honest null builds.
        let mut cols = Columns::never_fills(NEVER_FILLS);
        row(&mut cols, None).expect("row");
        assert!(
            cols.build(&schema).is_ok(),
            "leaving a forbidden column null must be allowed -- it is how a family says it did not look"
        );

        // B -- a plausible-looking value this family never observed.
        let mut cols = Columns::never_fills(NEVER_FILLS);
        row(&mut cols, Some("public".to_string())).expect("row");
        assert!(
            matches!(
                cols.build(&schema),
                Err(CatalogError::ForbiddenColumn { .. })
            ),
            "writing `visibility` must fail: a TSV of public items records which of `cfg`-gated, \
             private-module or not-public applies to exactly nothing"
        );

        // C CONTROL -- without the forbidden set the SAME rows build, so arm B failed because of
        // the rule rather than because the fixture was malformed.
        let mut cols = Columns::default();
        row(&mut cols, Some("public".to_string())).expect("row");
        assert!(cols.build(&schema).is_ok());
    }

    #[test]
    fn a_repeated_key_is_a_build_error_not_a_merge() {
        let mut c = Collisions::new("program.implementation");
        c.observe("impl:a#T~U");
        c.observe("impl:b#T~U");
        assert!(
            c.into_result().is_ok(),
            "two different keys must not be a collision"
        );

        let mut c = Collisions::new("program.implementation");
        c.observe("impl:a#T~U");
        c.observe("impl:a#T~U");
        assert!(matches!(
            c.into_result(),
            Err(CatalogError::KeyCollision { .. })
        ));
    }
}
