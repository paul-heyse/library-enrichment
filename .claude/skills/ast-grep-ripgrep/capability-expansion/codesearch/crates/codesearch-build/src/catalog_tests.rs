//! Building the catalog from the real shipped indexes.
//!
//! These run against the skill this crate lives inside, so they exercise the actual 193 flags,
//! 70 rule fields, 44 regex constructs and 49 behaviour probes rather than a fixture that agrees
//! with the code by construction.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use arrow::array::{Array, Int8Array, StringArray};

use crate::capabilities::{self, Seed};
use crate::catalog::{self, RunContext};
use crate::index;

fn skill_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../../..")
        .canonicalize()
        .expect("the skill root is three levels above the crate")
}

fn seed() -> Seed {
    capabilities::load(&Path::new(env!("CARGO_MANIFEST_DIR")).join("../../seeds/capabilities.json"))
        .expect("the authored capability seed loads")
}

fn interaction_seed() -> crate::interactions::Seed {
    crate::interactions::load(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../seeds/interactions.json"),
    )
    .expect("the authored interaction seed loads")
}

fn fragment_seed() -> crate::plan_fragments::Seed {
    crate::plan_fragments::load(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../seeds/plan_fragments.json"),
    )
    .expect("the authored fragment seed loads")
}

fn run() -> RunContext<'static> {
    RunContext {
        snapshot_id: "test-snapshot",
        run_id: "test-run",
    }
}

fn column<'a>(batch: &'a arrow::array::RecordBatch, name: &str) -> &'a StringArray {
    batch
        .column_by_name(name)
        .unwrap_or_else(|| panic!("no column {name}"))
        .as_any()
        .downcast_ref::<StringArray>()
        .unwrap_or_else(|| panic!("{name} is not a string column"))
}

fn rank_column<'a>(batch: &'a arrow::array::RecordBatch, name: &str) -> &'a Int8Array {
    batch
        .column_by_name(name)
        .unwrap_or_else(|| panic!("no column {name}"))
        .as_any()
        .downcast_ref::<Int8Array>()
        .unwrap_or_else(|| panic!("{name} is not an Int8 column"))
}

#[test]
fn the_catalog_builds_from_the_shipped_indexes() {
    let indexes = index::read_all(&skill_root(), index::Source::Skill).expect("indexes read");
    let batches = catalog::build(
        &indexes,
        &seed(),
        &interaction_seed(),
        &fragment_seed(),
        // No authored implementation bindings: these tests are about the index-derived half.
        &[],
        run(),
    )
    .expect("catalog builds");

    // 193 flags minus any with no long form, 70 rule fields, plus one mechanism per
    // construct-engine pair.
    assert!(
        batches.mechanism.num_rows() > 250,
        "expected mechanisms from flags, rule fields and both regex engines; got {}",
        batches.mechanism.num_rows()
    );
    assert_eq!(
        batches.behavior_assertion.num_rows(),
        49,
        "every behaviour probe becomes an assertion"
    );
    assert_eq!(
        batches.lattice.num_rows(),
        codesearch_bridge::lattice::LATTICES
            .iter()
            .map(|l| l.values.len())
            .sum::<usize>()
    );
}

/// The denominator must cover every row of every loaded index, or coverage understates itself.
///
/// One index row is not always one entry. A regex construct supported by both engines is **two**
/// inventory items, because "available in the Rust engine" and "available in PCRE2" are two
/// separately-true facts the index records in two columns. Emitting one entry per row made the
/// denominator disagree with the mechanism side by exactly those 16 constructs.
#[test]
fn every_loaded_index_row_becomes_at_least_one_surface_entry() {
    let indexes = index::read_all(&skill_root(), index::Source::Skill).expect("indexes read");
    // The expectation asks the SPEC which denominator each file belongs to, rather than naming the
    // exclusions here. Two sets of exclusions that have to be kept in step is how a test stops
    // testing the thing: `behaviors.tsv` is evidence about mechanisms rather than a feature the
    // tool exposes, and the five `library-api` sources describe the tool's own code rather than
    // its surface -- and all six facts now live in one place.
    let expected: usize = index::specs_for(index::Denominator::ToolSurface)
        .map(|s| {
            let rows = indexes.get(s.stem()).map(Vec::as_slice).unwrap_or(&[]);
            if s.stem() == "regex" {
                rows.iter()
                    .map(|r| {
                        let engines = usize::from(r["rust_regex"] == "yes")
                            + usize::from(r["pcre2"] == "yes");
                        // Supported by neither is still a documented construct, and dropping it
                        // would shrink the denominator by exactly the least-characterised rows.
                        engines.max(1)
                    })
                    .sum()
            } else {
                rows.len()
            }
        })
        .sum();
    let batches = catalog::build(
        &indexes,
        &seed(),
        &interaction_seed(),
        &fragment_seed(),
        // No authored implementation bindings: these tests are about the index-derived half.
        &[],
        run(),
    )
    .expect("catalog builds");
    assert_eq!(
        batches.surface_entry.num_rows(),
        expected,
        "the coverage denominator must be the whole inventory"
    );
    assert!(
        expected > indexes.values().filter(|v| !v.is_empty()).count(),
        "the expectation must be computed, not trivially satisfied"
    );
}

/// Every canonical record a family produced, with the kind it belongs to.
fn all_catalog_keys(batches: &catalog::CatalogBatches) -> BTreeMap<String, &'static str> {
    let mut out = BTreeMap::new();
    for (batch, kind) in [
        (&batches.mechanism, catalog::KIND_MECHANISM),
        (&batches.result_contract, catalog::KIND_RESULT_CONTRACT),
        (&batches.search_domain, catalog::KIND_SEARCH_DOMAIN),
        (&batches.negative_space, catalog::KIND_NEGATIVE_SPACE),
    ] {
        let keys = column(batch, "entity_key");
        for i in 0..keys.len() {
            out.insert(keys.value(i).to_string(), kind);
        }
    }
    out
}

/// Every canonical record is accounted for by some surface entry.
///
/// This is the Rust-side counterpart of `projection.violations_orphan_mechanism`, and it is the
/// invariant that caught the regex denominator being wrong: 16 `rg` pattern mechanisms existed,
/// were returned by `discover`, and appeared in no coverage number at all -- a catalog quietly
/// reporting less than it holds.
#[test]
fn no_catalog_record_is_missing_from_the_denominator() {
    let indexes = index::read_all(&skill_root(), index::Source::Skill).expect("indexes read");
    let batches = catalog::build(
        &indexes,
        &seed(),
        &interaction_seed(),
        &fragment_seed(),
        // No authored implementation bindings: these tests are about the index-derived half.
        &[],
        run(),
    )
    .expect("catalog builds");

    let entry_keys = column(&batches.surface_entry, "catalog_key");
    let claimed: std::collections::BTreeSet<&str> = (0..entry_keys.len())
        .filter(|i| entry_keys.is_valid(*i))
        .map(|i| entry_keys.value(i))
        .collect();

    let minted = all_catalog_keys(&batches);
    let orphans: Vec<&String> = minted
        .keys()
        .filter(|k| !claimed.contains(k.as_str()))
        .collect();
    assert!(
        orphans.is_empty(),
        "{} records are in no surface entry, starting with {:?}",
        orphans.len(),
        &orphans[..orphans.len().min(5)]
    );
    assert!(!claimed.is_empty(), "nothing was checked");
}

/// A surface entry never names a record that was not minted, and never mislabels its kind.
///
/// The other direction of the same join, and the one that would make coverage overstate: a
/// dangling key counts as characterised in any formulation that trusts the entry side's claim
/// rather than joining against the record tables. The kind is checked alongside because it is
/// denormalised, and a denormalised column that disagrees is worse than no column at all.
#[test]
fn no_surface_entry_names_a_record_that_does_not_exist() {
    let indexes = index::read_all(&skill_root(), index::Source::Skill).expect("indexes read");
    let batches = catalog::build(
        &indexes,
        &seed(),
        &interaction_seed(),
        &fragment_seed(),
        // No authored implementation bindings: these tests are about the index-derived half.
        &[],
        run(),
    )
    .expect("catalog builds");
    let minted = all_catalog_keys(&batches);

    let entry_keys = column(&batches.surface_entry, "catalog_key");
    let entry_kinds = column(&batches.surface_entry, "catalog_kind");
    let mut checked = 0usize;
    for i in 0..entry_keys.len() {
        if !entry_keys.is_valid(i) {
            continue;
        }
        checked += 1;
        let key = entry_keys.value(i);
        let kind = minted
            .get(key)
            .unwrap_or_else(|| panic!("surface entry names no record: {key}"));
        assert_eq!(
            entry_kinds.value(i),
            *kind,
            "`{key}` is labelled {} but is a {kind}",
            entry_kinds.value(i)
        );
    }
    assert!(checked > 600, "only {checked} entries were checked");
}

/// The three record kinds beyond `mechanism` each cover their whole index.
#[test]
fn the_other_record_kinds_cover_their_whole_index() {
    let indexes = index::read_all(&skill_root(), index::Source::Skill).expect("indexes read");
    let batches = catalog::build(
        &indexes,
        &seed(),
        &interaction_seed(),
        &fragment_seed(),
        // No authored implementation bindings: these tests are about the index-derived half.
        &[],
        run(),
    )
    .expect("catalog builds");

    assert_eq!(
        batches.result_contract.num_rows(),
        indexes["exit-codes"].len()
    );
    assert_eq!(
        batches.search_domain.num_rows(),
        indexes["file-types"].len() + indexes["languages"].len()
    );
    assert_eq!(
        batches.negative_space.num_rows(),
        indexes["unreachable"].len()
    );
}

/// `no` and `partial` are different facts, and the `truth` lattice keeps them apart.
///
/// This is the first data any lattice beyond `precision` and `observed` carries. A build mapping
/// `partial` onto `false` would report `--regex-size-limit` as simply unreachable, when the truth
/// is that it binds the default engine and not PCRE2 -- which is the distinction the record exists
/// to preserve.
#[test]
fn partial_reachability_is_unknown_and_not_false() {
    let indexes = index::read_all(&skill_root(), index::Source::Skill).expect("indexes read");
    let batches = catalog::build(
        &indexes,
        &seed(),
        &interaction_seed(),
        &fragment_seed(),
        // No authored implementation bindings: these tests are about the index-derived half.
        &[],
        run(),
    )
    .expect("catalog builds");

    let texts = column(&batches.negative_space, "capability_text");
    let ranks = rank_column(&batches.negative_space, "truth_rank");
    let unknown = codesearch_bridge::lattice::rank_of("truth", "unknown").unwrap();
    let no = codesearch_bridge::lattice::rank_of("truth", "false").unwrap();

    let mut saw_partial = false;
    let mut saw_no = false;
    for i in 0..texts.len() {
        let source = indexes["unreachable"]
            .iter()
            .find(|r| r["capability"] == texts.value(i))
            .expect("every row traces to its index row");
        match source["cli_reachable"].as_str() {
            "partial" => {
                saw_partial = true;
                assert_eq!(ranks.value(i), unknown, "{}", texts.value(i));
            }
            "no" => {
                saw_no = true;
                assert_eq!(ranks.value(i), no, "{}", texts.value(i));
            }
            other => panic!("unhandled reachability `{other}`"),
        }
    }
    assert!(saw_partial && saw_no, "both arms must be exercised");
}

/// An accepted language with no rule-schema entry stays two separate facts.
#[test]
fn parseable_and_schema_listed_are_not_collapsed() {
    let indexes = index::read_all(&skill_root(), index::Source::Skill).expect("indexes read");
    let split: Vec<&str> = indexes["languages"]
        .iter()
        .filter(|r| r["status"] == "accepted" && r["rule_schema"] != "yes")
        .map(|r| r["language"].as_str())
        .collect();
    assert!(
        !split.is_empty(),
        "the shipped index must contain such a language for this to mean anything"
    );

    let batches = catalog::build(
        &indexes,
        &seed(),
        &interaction_seed(),
        &fragment_seed(),
        // No authored implementation bindings: these tests are about the index-derived half.
        &[],
        run(),
    )
    .expect("catalog builds");
    let names = column(&batches.search_domain, "name");
    let kinds = column(&batches.search_domain, "domain_kind");
    let ranks = rank_column(&batches.search_domain, "truth_rank");
    let schema = batches
        .search_domain
        .column_by_name("rule_schema_available")
        .expect("column")
        .as_any()
        .downcast_ref::<arrow::array::BooleanArray>()
        .expect("boolean");
    let accepted = codesearch_bridge::lattice::rank_of("truth", "true").unwrap();

    let i = (0..names.len())
        .find(|i| names.value(*i) == split[0] && kinds.value(*i) == "language")
        .expect("the language is a search domain");
    assert_eq!(ranks.value(i), accepted, "it parses");
    assert!(!schema.value(i), "and yet has no rule schema entry");
}

/// Parameters exist for exactly the mechanisms whose index row declares an argument.
///
/// The count is derived from the source rather than written down, so adding a flag to the skill
/// changes both sides together. A parameter that went missing would present as a flag that takes
/// no argument, which is a confident wrong answer rather than a gap.
#[test]
fn a_parameter_exists_for_every_declared_argument() {
    let indexes = index::read_all(&skill_root(), index::Source::Skill).expect("indexes read");
    let with_argument = indexes["flags"]
        .iter()
        .filter(|r| !r["long"].is_empty() && !r["arg"].is_empty())
        .count();
    let rule_fields = indexes["rule-fields"].len();
    assert!(with_argument > 0 && rule_fields > 0, "nothing to check");

    let batches = catalog::build(
        &indexes,
        &seed(),
        &interaction_seed(),
        &fragment_seed(),
        // No authored implementation bindings: these tests are about the index-derived half.
        &[],
        run(),
    )
    .expect("catalog builds");
    assert_eq!(
        batches.parameter.num_rows(),
        with_argument + rule_fields,
        "every flag with an argument and every rule field is a parameter"
    );

    // The three defaults are unknown, and saying so is the point: §4.3's worked example is that
    // the library default and the effective default on the chosen surface differ, so a guess here
    // would be wrong exactly where it matters.
    for name in [
        "declared_default",
        "application_override",
        "effective_default",
    ] {
        let col = column(&batches.parameter, name);
        assert_eq!(
            col.null_count(),
            col.len(),
            "{name} must be null -- the index records no default"
        );
    }
}

/// A rule field's obligation comes from the index, not from a default.
#[test]
fn a_required_rule_field_is_marked_required() {
    let indexes = index::read_all(&skill_root(), index::Source::Skill).expect("indexes read");
    let batches = catalog::build(
        &indexes,
        &seed(),
        &interaction_seed(),
        &fragment_seed(),
        // No authored implementation bindings: these tests are about the index-derived half.
        &[],
        run(),
    )
    .expect("catalog builds");

    let required = batches
        .parameter
        .column_by_name("required")
        .expect("required column")
        .as_any()
        .downcast_ref::<arrow::array::BooleanArray>()
        .expect("boolean");
    let n_required = (0..required.len()).filter(|i| required.value(*i)).count();
    let expected = indexes["rule-fields"]
        .iter()
        .filter(|r| r["required"] == "required")
        .count()
        + indexes["flags"]
            .iter()
            .filter(|r| !r["long"].is_empty() && !r["arg"].is_empty())
            .filter(|r| !r["arg"].starts_with('='))
            .count();
    assert_eq!(n_required, expected);
    assert!(
        n_required < required.len(),
        "some parameters must be optional, or this is measuring a constant"
    );
}

/// A behaviour probe's subject is a topic, and the row says so.
///
/// It used to claim `subject_kind = 'mechanism'` and point at `mech:<tool>/cli/<topic>`, which
/// named nothing: no flag, rule field or construct is called `structural-patterns`. Forty-nine
/// references to non-existent mechanisms is the kind of wrongness that reads as data.
#[test]
fn an_assertion_subject_is_a_topic_and_is_not_a_fabricated_mechanism() {
    let indexes = index::read_all(&skill_root(), index::Source::Skill).expect("indexes read");
    let batches = catalog::build(
        &indexes,
        &seed(),
        &interaction_seed(),
        &fragment_seed(),
        // No authored implementation bindings: these tests are about the index-derived half.
        &[],
        run(),
    )
    .expect("catalog builds");

    let kinds = column(&batches.behavior_assertion, "subject_kind");
    assert!(kinds.len() > 0, "nothing to check");
    for i in 0..kinds.len() {
        assert_eq!(kinds.value(i), "topic");
    }

    let mechanism_ids: std::collections::BTreeSet<&str> = {
        let ids = column(&batches.mechanism, "entity_id");
        (0..ids.len()).map(|i| ids.value(i)).collect()
    };
    let subjects = column(&batches.behavior_assertion, "subject_entity_id");
    for i in 0..subjects.len() {
        assert!(
            !mechanism_ids.contains(subjects.value(i)),
            "a topic subject must not collide with a mechanism id"
        );
    }
}

/// A construct in both engines is two mechanisms, because the engines differ semantically.
#[test]
fn a_construct_available_in_both_engines_yields_two_mechanisms() {
    let indexes = index::read_all(&skill_root(), index::Source::Skill).expect("indexes read");
    let both: Vec<String> = indexes["regex"]
        .iter()
        .filter(|r| r["rust_regex"] == "yes" && r["pcre2"] == "yes")
        .map(|r| r["construct"].clone())
        .collect();
    assert!(
        !both.is_empty(),
        "the fixture data must contain such a construct"
    );

    let batches = catalog::build(
        &indexes,
        &seed(),
        &interaction_seed(),
        &fragment_seed(),
        // No authored implementation bindings: these tests are about the index-derived half.
        &[],
        run(),
    )
    .expect("catalog builds");
    let keys = column(&batches.mechanism, "entity_key");
    let all: Vec<&str> = (0..keys.len()).map(|i| keys.value(i)).collect();

    let construct = &both[0];
    assert!(
        all.contains(&format!("mech:rg/pattern/{construct}").as_str()),
        "missing the default-engine mechanism for {construct}"
    );
    assert!(
        all.contains(&format!("mech:pcre2/pattern/{construct}").as_str()),
        "missing the PCRE2 mechanism for {construct}"
    );
}

/// The epistemic vocabulary must survive. A never-probed construct must not be reported as
/// though someone had looked.
#[test]
fn a_not_probed_construct_keeps_its_rank() {
    let indexes = index::read_all(&skill_root(), index::Source::Skill).expect("indexes read");
    let not_probed: Vec<String> = indexes["regex"]
        .iter()
        .filter(|r| r["observed"] == "not-probed")
        .map(|r| r["construct"].clone())
        .collect();
    assert!(
        !not_probed.is_empty(),
        "the shipped index must contain a not-probed construct for this to mean anything"
    );

    let batches = catalog::build(
        &indexes,
        &seed(),
        &interaction_seed(),
        &fragment_seed(),
        // No authored implementation bindings: these tests are about the index-derived half.
        &[],
        run(),
    )
    .expect("catalog builds");
    let keys = column(&batches.mechanism, "entity_key");
    let ranks = rank_column(&batches.mechanism, "observed_rank");
    let expected = codesearch_bridge::lattice::rank_of("observed", "not-probed").unwrap();

    let target = format!("mech:rg/pattern/{}", not_probed[0]);
    let idx = (0..keys.len())
        .find(|i| keys.value(*i) == target)
        .unwrap_or_else(|| panic!("no mechanism {target}"));
    assert_eq!(
        ranks.value(idx),
        expected,
        "a not-probed construct must stay at the bottom of the observed lattice, not be \
         promoted to `recorded` because a row existed"
    );
}

/// Confirmed probes are exact; anything weaker is not.
#[test]
fn assertion_precision_follows_the_probe_verdict() {
    let indexes = index::read_all(&skill_root(), index::Source::Skill).expect("indexes read");
    let batches = catalog::build(
        &indexes,
        &seed(),
        &interaction_seed(),
        &fragment_seed(),
        // No authored implementation bindings: these tests are about the index-derived half.
        &[],
        run(),
    )
    .expect("catalog builds");

    let status = column(&batches.behavior_assertion, "status");
    let precision = rank_column(&batches.behavior_assertion, "precision_rank");
    let exact = codesearch_bridge::lattice::rank_of("precision", "exact").unwrap();
    let inexact = codesearch_bridge::lattice::rank_of("precision", "inexact").unwrap();

    let mut seen_confirmed = false;
    let mut seen_weaker = false;
    for i in 0..status.len() {
        match status.value(i) {
            "confirmed" => {
                seen_confirmed = true;
                assert_eq!(precision.value(i), exact);
            }
            _ => {
                seen_weaker = true;
                assert_eq!(precision.value(i), inexact);
            }
        }
    }
    assert!(seen_confirmed, "the index has confirmed probes");
    assert!(
        seen_weaker,
        "the index has 6 recorded probes; if this fails the test is only checking one branch"
    );
}

/// Every mechanism's id is the hash of its key and the snapshot, and ids are unique.
#[test]
fn mechanism_ids_are_derived_and_unique() {
    let indexes = index::read_all(&skill_root(), index::Source::Skill).expect("indexes read");
    let batches = catalog::build(
        &indexes,
        &seed(),
        &interaction_seed(),
        &fragment_seed(),
        // No authored implementation bindings: these tests are about the index-derived half.
        &[],
        run(),
    )
    .expect("catalog builds");

    let keys = column(&batches.mechanism, "entity_key");
    let ids = column(&batches.mechanism, "entity_id");

    let mut seen = std::collections::BTreeSet::new();
    for i in 0..keys.len() {
        let expected = codesearch_bridge::identity::entity_id(keys.value(i), "test-snapshot");
        assert_eq!(ids.value(i), expected, "id must be derived from the key");
        assert!(
            seen.insert(ids.value(i).to_string()),
            "duplicate entity_id for {} -- the primary key declaration would be false",
            keys.value(i)
        );
    }
}
