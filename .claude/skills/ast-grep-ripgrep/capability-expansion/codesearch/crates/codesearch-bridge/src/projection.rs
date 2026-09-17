//! The retrieval surface, as registered views rather than as SQL assembled in a binary.
//!
//! # Why a view and not a string in `main`
//!
//! A projection composed at the call site is invisible: nothing can enumerate it, nothing can
//! explain it, and every caller re-invents the join discipline. A registered view is a real object
//! in the session, so `information_schema.views` lists the retrieval surface, `EXPLAIN
//! projection.discover` works with no extra code, and a caller supplies only predicates. The join
//! discipline is then written once and cannot be got wrong differently in two places.
//!
//! # The join discipline, in one place
//!
//! - **Prerequisite completeness is an `INNER JOIN`.** A mechanism reachable only through a
//!   capability cannot appear without that capability, because [`DISCOVER`] joins through the
//!   binding relation. Filtering afterwards would leave the unbound rows in the result.
//! - **Optional context is a `LEFT JOIN`.** A mechanism that takes no argument has no
//!   `catalog.parameter` row, and absence there means "takes none" -- not "unknown".
//! - **Coverage joins against an independent denominator.** `catalog.surface_entry` is built from
//!   the index rows and `catalog.mechanism` from the same rows by a different rule; neither is
//!   derived from the other, which is what makes the `LEFT JOIN` between them unable to drift.
//!   Counting `surface_entry.mechanism_key IS NOT NULL` instead would count a *dangling* key as
//!   characterised, which is the confidently-wrong answer this catalog exists to avoid.
//! - **`retracted = false` everywhere**, which is about a *row*: this fact was withdrawn.
//! - **Run visibility is about the *write* that produced the row**, and it is enforced once, at
//!   the base-table boundary, rather than in each of the thirty views. See below.
//!
//! # Run visibility, stated once
//!
//! Every canonical row carries the `run_id` of the extraction run that wrote it, and a run that
//! did not finish wrote rows that nobody should be served. The rule is therefore
//! `completion = 'complete'` — but writing it into thirty views would be thirty chances to write
//! it differently, and the one view that forgot would be the one nobody noticed.
//!
//! So the Delta providers are registered under `<name>__all`, and [`register_visibility`] defines
//! `<name>` as a view over `<name>__all` filtered to the visible runs. Every projection below
//! reads `mechanism`, `parameter`, `surface_entry` and the rest exactly as it did before, and
//! every one of them is filtered, because there is no unfiltered name for them to read by
//! accident. `<name>__all` still exists, deliberately, for the two things that need it.
//!
//! Two tables are exempt, and the exemption is worth stating rather than inferring:
//!
//! - **`extraction_run`** decides visibility, so filtering it by visibility would be circular —
//!   and an abandoned run must remain *visible as abandoned*, or the only record that some rows
//!   are unaccounted for would itself be hidden.
//! - **`lattice`** is a reference table. It has no `run_id`, because rank-to-label is not an
//!   observation about a snapshot.
//!
//! `--include-partial` reaches the rule through the `request` relation, the same way `--bind` and
//! `--max-depth` do, so no SQL is assembled per invocation.
//!
//! # Validation is a query
//!
//! Probe PB07 measured why: a Delta CHECK constraint naming a second table panics the process
//! outright, and one calling a UDF makes the table unwritable by any session lacking that UDF. So
//! single-row, single-table, plain-column-algebra invariants are CHECK constraints, and everything
//! cross-row or cross-table is a `violations_*` view here. Views may use UDFs freely, because a
//! view lives in this process and dies with it.
//!
//! **An invariant that examined nothing must not look like an invariant that passed.** That is
//! what [`INVARIANT`] is for: it declares each invariant with the number of rows it inspects, and
//! [`VALIDATE`] joins the two, so `checked = 0` is visible rather than presenting as a clean bill
//! of health. The join is a `FULL OUTER` one so that the reverse mistake -- a violation reported
//! under a name nobody declared, which would otherwise vanish -- shows up as a null `checked`.

use datafusion::common::Result as DFResult;
use datafusion::prelude::SessionContext;

use crate::lazy;

/// The schema every projection is registered under.
pub const SCHEMA: &str = "projection";

/// One registered view: its unqualified name, why it exists, and the SQL that defines it.
#[derive(Debug, Clone, Copy)]
pub struct Projection {
    pub name: &'static str,
    pub about: &'static str,
    pub sql: &'static str,
}

/// Every extraction run, finished or not.
///
/// Deliberately **not** filtered by visibility: this is the view that explains why something is
/// missing, and a run hidden because it did not complete would make "where did my rows go?"
/// unanswerable from inside the catalog.
///
/// `scope`'s fields are projected flat because a caller reads them, and kept as a struct in
/// storage because a caller filters on them. Both matter, and they are not in tension.
const EXTRACTION_RUN: &str = "\
SELECT r.entity_key AS run_id, r.snapshot_id, r.family, r.extractor_identity, r.context_id, \
       r.scope['skill_root'] AS skill_root, \
       r.scope['index_files'] AS index_files, \
       r.scope['index_rows'] AS index_rows, \
       r.completion_rank, \
       lattice_label('completion', r.completion_rank) AS completion, \
       r.started_at, r.finished_at, r.failed_pass \
FROM extraction_run r \
WHERE r.retracted = false";

/// Mechanisms, with their lattice ordinals rendered for display.
///
/// Both the rank and the label are projected. The rank is what a filter compares -- a function
/// between the column and the comparison is what would cost the file pruning the `Int8` encoding
/// exists to keep -- and the label is what a reader needs.
const MECHANISM: &str = "\
SELECT m.entity_id, m.entity_key, m.snapshot_id, m.tool, m.surface, m.natural_key, \
       m.invocation_form, m.native_schema_ref, m.summary, m.coverage_status, \
       m.precision_rank, m.observed_rank, \
       lattice_label('precision', m.precision_rank) AS precision, \
       lattice_label('observed', m.observed_rank) AS observed \
FROM mechanism m \
WHERE m.retracted = false";

/// Parameters, joined to their mechanism by `entity_id`.
///
/// The three default columns stay separate on purpose. The declared library default, the
/// application override and the effective behaviour on the chosen surface are three different
/// facts, and collapsing them makes the catalog confidently wrong on exactly the question it
/// exists to answer.
const PARAMETER: &str = "\
SELECT p.entity_key, p.mechanism_id, p.name, p.value_domain, p.required, \
       p.declared_default, p.application_override, p.effective_default, p.coverage_status \
FROM parameter p \
WHERE p.retracted = false";

/// The abilities a mechanism serves, folded to one row per mechanism.
const MECHANISM_ABILITY: &str = "\
SELECT b.object_key AS mechanism_key, \
       count(*) AS ability_count, \
       array_to_string(array_agg(c.ability ORDER BY c.ability), ', ') AS abilities \
FROM surface_binding b \
JOIN capability c ON c.entity_key = b.subject_key AND c.retracted = false \
WHERE b.retracted = false AND b.subject_kind = 'capability' AND b.object_kind = 'mechanism' \
GROUP BY b.object_key";

/// Everything known about a mechanism, in one row.
///
/// The parameter join is `LEFT` and one-to-at-most-one: in this slice a mechanism's parameter is
/// a CLI flag's argument or a rule field's value, so there is never more than one. A mechanism
/// with none is a flag that takes no argument, which is a fact rather than a gap.
const MECHANISM_DETAIL: &str = "\
SELECT m.*, \
       p.name AS parameter, p.value_domain AS parameter_domain, \
       p.required AS parameter_required, p.effective_default AS parameter_default, \
       coalesce(a.abilities, '') AS abilities, \
       coalesce(a.ability_count, 0) AS ability_count \
FROM projection.mechanism m \
LEFT JOIN projection.parameter p ON p.mechanism_id = m.entity_id \
LEFT JOIN projection.mechanism_ability a ON a.mechanism_key = m.entity_key";

/// Mechanisms reached through a capability, with the four retrieval facets §11 filters on.
///
/// The facets are projected here rather than joined at query time because they belong to the
/// capability and a mechanism is only reachable through one -- so a facet predicate and the
/// binding are the same join, asked once.
///
/// `INNER JOIN` by design: a mechanism the requested
/// ability does not claim must not appear, and it does not, because the row does not exist.
const DISCOVER: &str = "\
SELECT c.ability, c.topic AS capability_topic, b.role, \
       c.facet_subject, c.facet_relationship, c.facet_result, c.facet_semantic_layer, \
       d.* \
FROM projection.mechanism_detail d \
JOIN surface_binding b ON b.object_key = d.entity_key AND b.retracted = false \
                      AND b.subject_kind = 'capability' AND b.object_kind = 'mechanism' \
JOIN capability c ON c.entity_key = b.subject_key AND c.retracted = false";

/// The abstract abilities, with the facets an agent filters on.
const CAPABILITY: &str = "\
SELECT c.entity_key, c.ability, c.topic, c.summary, \
       c.facet_subject, c.facet_relationship, c.facet_result, c.facet_semantic_layer, \
       c.mechanism_count \
FROM capability c \
WHERE c.retracted = false";

/// Interactions, with their atoms folded back into a readable predicate.
///
/// The predicate text is for a human. Evaluation never parses it -- that is what
/// [`INTERACTION_TRUTH`] does, over the atoms themselves.
const INTERACTION: &str = "\
SELECT i.entity_key, i.kind, i.affected_key, i.effect, i.ordering_requirement, \
       i.assertion_keys, \
       array_to_string(array_agg( \
         a.facet || ' ' || a.op || coalesce(' ' || a.value, '') \
         ORDER BY a.term_ord, a.atom_ord), ' AND ') AS predicate \
FROM interaction i \
JOIN interaction_atom a ON a.interaction_key = i.entity_key AND a.retracted = false \
WHERE i.retracted = false \
GROUP BY i.entity_key, i.kind, i.affected_key, i.effect, i.ordering_requirement, i.assertion_keys";

/// Evaluating every interaction against the request, in two `GROUP BY`s and nothing else.
///
/// `min` within a `term_ord` is AND; `max` across them is OR; both are over the `truth` lattice,
/// so Kleene three-valued logic is a consequence of the encoding rather than something anyone
/// implemented. An atom whose facet the request does not bind answers `unknown`, `min` carries it
/// through the conjunct, and an interaction finishing at `unknown` reaches the caller as a choice
/// still open instead of being quietly treated as false.
///
/// The request arrives as the `request` relation rather than as a bound placeholder. That is what
/// §7.1 specifies, and it is also the only form that types: `atom_truth` takes six arguments of
/// mixed kinds including a `List<Utf8>`, so its signature cannot tell the planner what a
/// placeholder in the sixth position would be. A one-row table says so by construction, and the
/// caller's values live in an Arrow array rather than anywhere near the SQL text.
const INTERACTION_TRUTH: &str = "\
WITH atom AS ( \
  SELECT a.interaction_key, a.term_ord, \
         atom_truth(a.facet, a.op, a.value, a.value_list, a.negated, r.bindings) AS t \
  FROM interaction_atom a CROSS JOIN request r \
  WHERE a.retracted = false \
), \
term AS ( \
  SELECT interaction_key, term_ord, min(t) AS t FROM atom GROUP BY interaction_key, term_ord \
), \
folded AS ( \
  SELECT interaction_key, max(t) AS truth_rank FROM term GROUP BY interaction_key \
) \
SELECT i.entity_key, i.kind, i.affected_key, f.truth_rank, \
       lattice_label('truth', f.truth_rank) AS holds, \
       i.ordering_requirement, i.effect \
FROM interaction i \
JOIN folded f ON f.interaction_key = i.entity_key \
WHERE i.retracted = false";

/// Plan fragments, with their recall rendered and their obligations visible.
const PLAN_FRAGMENT: &str = "\
SELECT f.entity_key, f.name, f.accepted_input, f.produced_output, f.scope_assumption, \
       f.coordinate_preservation, f.recall_rank, \
       lattice_label('recall', f.recall_rank) AS recall, \
       f.ordering_requirement, f.mechanism_keys, f.assertion_keys, f.unresolved_obligations \
FROM plan_fragment f \
WHERE f.retracted = false";

/// Which fragment may follow which. **This is the composition rule, and it is a join.**
///
/// The proposal warns: do not feed only matching lines into a structural stage that requires
/// complete enclosing syntax. Here that is not advice -- a `line_set` producer has no row in this
/// relation paired with a `node_set` consumer, so the chain cannot be built. Losing or remapping
/// coordinates breaks the join for the same reason even when the unit types agree.
const FRAGMENT_EDGE: &str = "\
SELECT a.entity_key AS from_key, a.name AS from_name, \
       b.entity_key AS to_key, b.name AS to_name, \
       a.produced_output AS unit, b.scope_assumption \
FROM plan_fragment a JOIN plan_fragment b \
  ON b.accepted_input = a.produced_output \
 AND ( b.scope_assumption = 'none' \
    OR (b.scope_assumption IN ('complete_enclosing_syntax', 'original_coordinates') \
        AND a.coordinate_preservation = 'preserved') \
    OR (b.scope_assumption = 'whole_file' AND a.coordinate_preservation = 'preserved' \
        AND a.produced_output IN ('file_set', 'file_content')) ) \
WHERE a.retracted = false AND b.retracted = false";

/// Every chain from the requested starting unit, bounded, with its recall folded and its path.
///
/// # The bound is inside the recursive term, and that is not a style choice
///
/// PB09 measured the alternative: with `depth < :max` in an outer `WHERE`, a cyclic graph timed
/// out at 20 s where this form returns in 24 ms. Fragment composition **is** cyclic in general --
/// a capture set can feed a textual predicate that produces another capture set -- so this is not
/// hypothetical.
///
/// PB14 then measured that `UNION` is not an alternative to the bound: `RecursiveQueryExec`'s
/// deduplicator runs over the full output tuple, so a projection carrying `depth` is unique by
/// construction and `is_distinct` is inert. The two mitigations are mutually exclusive, `depth` is
/// exactly what a bound needs, and every recursive query here carries one -- so `UNION ALL`
/// throughout, and its lack of cycle tolerance is not a latent risk.
///
/// # The base term casts its rank, and the reason is worth knowing
///
/// A recursive CTE requires both terms to agree on their output fields **including Arrow field
/// metadata**. `plan_fragment.recall_rank` carries the `codesearch.lattice` extension name, and
/// the `CASE` expression in the recursive term produces a bare `Int8`, so the two did not match
/// and DataFusion refused the plan. The `CAST` drops the metadata on the base term so both sides
/// agree.
///
/// That refusal is also a useful measurement: **the extension metadata reaches the logical plan
/// schema**, not merely storage. That is the prerequisite an `IdentityDiscipline` analyzer rule
/// needs in order to reject an `entity_key` joined to an `entity_id` at plan time, and it is now
/// established by something failing rather than by reading source.
///
/// `recall_rank` folds with `min`: a chain containing one heuristic prefilter is heuristic
/// overall, automatically. The path witness is required rather than optional -- a reachability
/// answer without the path is the opaque convenience result the proposal forbids.
const CHAIN: &str = "\
WITH RECURSIVE chain(head, tail, tail_output, tail_coordinates, depth, recall_rank, path) AS ( \
    SELECT f.entity_key AS head, f.entity_key AS tail, f.produced_output AS tail_output, \
           f.coordinate_preservation AS tail_coordinates, \
           1 AS depth, CAST(f.recall_rank AS TINYINT) AS recall_rank, f.name AS path \
      FROM plan_fragment f CROSS JOIN request r \
     WHERE f.retracted = false AND f.accepted_input = r.start_unit \
  UNION ALL \
    SELECT c.head AS head, n.entity_key AS tail, n.produced_output AS tail_output, \
           n.coordinate_preservation AS tail_coordinates, \
           c.depth + 1 AS depth, \
           CASE WHEN n.recall_rank < c.recall_rank THEN n.recall_rank \
                ELSE c.recall_rank END AS recall_rank, \
           c.path || ' -> ' || n.name AS path \
      FROM chain c \
      CROSS JOIN request r \
      JOIN plan_fragment n \
        ON n.accepted_input = c.tail_output \
       AND n.retracted = false \
       AND ( n.scope_assumption = 'none' \
          OR (n.scope_assumption IN ('complete_enclosing_syntax', 'original_coordinates') \
              AND c.tail_coordinates = 'preserved') \
          OR (n.scope_assumption = 'whole_file' AND c.tail_coordinates = 'preserved' \
              AND c.tail_output IN ('file_set', 'file_content')) ) \
     WHERE c.depth < r.max_depth \
) \
SELECT head, tail, tail_output AS produces, depth, recall_rank, \
       lattice_label('recall', recall_rank) AS recall, path \
FROM chain";

// ================================================================================================
// §7.3's closure queries. Two of the six; the other four need families that are not built.
//
// All three shape rules come from measurements, not taste:
//   * `depth` is bounded INSIDE the recursive term (PB09 -- the outer-`WHERE` form timed out at
//     20 s where this returns in 24 ms);
//   * `UNION ALL`, never `UNION` (PB14 -- dedup is inert the moment `depth` is projected, because
//     the deduplicator runs over the whole tuple);
//   * every column is aliased in BOTH terms, and a lattice column is `CAST` to drop its extension
//     metadata, because the two terms must agree on field metadata as well as on type.
//
// The path witness is a `List<Utf8>`, which §7.3 requires rather than suggests: a reachability
// answer without the path is exactly the opaque convenience result the proposal forbids.
// ================================================================================================

/// What a definition contains, transitively.
///
/// # This is empty, and the emptiness is the finding
///
/// `symbols.tsv` holds **items** -- structs, enums, traits, functions -- and not modules. Every
/// item's owner is therefore a module, and no module is an indexed definition, so
/// `definition.owner_id` resolves for **zero of 716** rows and this closure has no edges at all.
///
/// The view is still here, and still correct, for two reasons. It is the shape §7.3 specifies and
/// a family that indexes modules would populate it without a line changing; and an empty
/// projection that nobody can distinguish from "nothing contains anything" is precisely the
/// silence this catalog exists to refuse. `projection.closure` reports the edge count beside the
/// reason, so a caller reads "no edges, because modules are not indexed" rather than nothing.
const CONTAINMENT: &str = "\
WITH RECURSIVE containment(root_key, root_path, member_key, member_path, depth, path) AS ( \
    SELECT o.entity_key AS root_key, o.canonical_path AS root_path, \
           d.entity_key AS member_key, d.canonical_path AS member_path, \
           1 AS depth, make_array(o.canonical_path, d.canonical_path) AS path \
      FROM definition d \
      JOIN definition o ON o.canonical_path = d.owner_path AND o.retracted = false \
     WHERE d.retracted = false \
  UNION ALL \
    SELECT c.root_key AS root_key, c.root_path AS root_path, \
           n.entity_key AS member_key, n.canonical_path AS member_path, \
           c.depth + 1 AS depth, array_append(c.path, n.canonical_path) AS path \
      FROM containment c \
      CROSS JOIN request r \
      JOIN definition n ON n.owner_path = c.member_path AND n.retracted = false \
     WHERE c.depth < r.max_depth \
) \
SELECT root_key, root_path, member_key, member_path, depth, path FROM containment";

/// Where an item can be named from, following re-exports to their origin.
///
/// Unlike the containment closure this one has real edges -- 273 of them -- but **every chain is
/// exactly one hop**: no `canonical_path` in `aliases.tsv` is itself an `access_path`, so the
/// recursive term matches nothing and the recursion terminates on its first pass.
///
/// That is worth stating rather than discovering. The rows answer a real question ("`globset::Glob`
/// is defined at `globset::glob::Glob`"), and the recursion is correct and bounded for an index
/// that does chain -- but a reader must not conclude from `max_depth = 1` that re-export chains
/// cannot be longer, only that these are not.
const RE_EXPORT_CHAIN: &str = "\
WITH RECURSIVE reexport(origin_key, access_path, canonical_path, depth, path) AS ( \
    SELECT e.entity_key AS origin_key, e.access_path AS access_path, \
           e.canonical_path AS canonical_path, 1 AS depth, \
           make_array(e.access_path, e.canonical_path) AS path \
      FROM export_path e \
     WHERE e.retracted = false \
  UNION ALL \
    SELECT c.origin_key AS origin_key, c.access_path AS access_path, \
           n.canonical_path AS canonical_path, c.depth + 1 AS depth, \
           array_append(c.path, n.canonical_path) AS path \
      FROM reexport c \
      CROSS JOIN request r \
      JOIN export_path n ON n.access_path = c.canonical_path AND n.retracted = false \
     WHERE c.depth < r.max_depth \
) \
SELECT origin_key, access_path, canonical_path, depth, path FROM reexport";

/// Each closure, the edges it walks, and the deepest chain it found.
///
/// The same discipline as `projection.invariant`: **a closure that traversed nothing must not look
/// like a closure that found nothing to traverse.** `validate` reports `checked` beside
/// `violations` for exactly this reason, and an empty reachability answer is the same hazard --
/// worse, because a caller naturally reads it as "there is nothing there".
///
/// `limited_by` is a constant per closure rather than a derived value, and it is the honest place
/// for it: the limit is a property of what the extraction family could observe, not of any row.
/// §7.3's third closure: which packages a package depends on, transitively.
///
/// **It is guarded, and today the guard is what it returns.** `cargo metadata --no-deps` sets
/// `resolve` to null, and probe PL001 measured that **a null `resolve` is not "no dependencies"**
/// -- so every edge row carries `resolve_present`, and this view refuses to traverse when it is
/// false. Without that check the closure would return an empty graph a caller reads as a fact
/// about the crates rather than about the extraction.
///
/// §7.3 states the rule and where it belongs: *"§3.1's `resolve_present` must be `true`, or the
/// query returns no rows for a reason rather than an empty graph that reads as 'no dependencies'.
/// That check is in the view definition, not in the caller."*
///
/// The other half of why it is empty: a declared edge names a crate and a **requirement**
/// (`bstr ^1.12`), not a package. Matching that to a pinned `bstr 1.13.0` means deciding whether
/// the requirement is satisfied, which is semver arithmetic rather than a join -- so
/// `dep_package_key` stays null until a family resolves something, and there is nothing to walk.
/// Sixty-one of the 172 edges do name a crate the catalog holds, which is what that family would
/// have to work with.
///
/// The recursive shape is PB09's: `depth` bounded INSIDE the recursive term, `UNION ALL` per
/// PB14, and a `List<Utf8>` path witness because a reachability answer without the path is the
/// opaque result §12 forbids.
const DEPENDENCY_CLOSURE: &str = "\
WITH RECURSIVE walk(root_key, node_key, depth, path) AS ( \
    SELECT e.package_key, e.dep_package_key, 1, \
           make_array(e.package_key, e.dep_package_key) \
    FROM dependency_edge e CROSS JOIN request q \
    WHERE e.retracted = false AND e.resolve_present AND e.dep_package_key IS NOT NULL \
  UNION ALL \
    SELECT w.root_key, n.dep_package_key, w.depth + 1, \
           array_append(w.path, n.dep_package_key) \
    FROM walk w \
    JOIN dependency_edge n ON n.package_key = w.node_key \
    CROSS JOIN request q \
    WHERE n.retracted = false AND n.resolve_present AND n.dep_package_key IS NOT NULL \
      AND w.depth < q.max_depth \
) \
SELECT root_key, node_key, depth, path FROM walk";

const CLOSURE: &str = "\
SELECT 'containment' AS closure, \
       'program.definition.owner_path -> canonical_path' AS walks, \
       (SELECT count(*) FROM definition WHERE retracted = false) AS nodes, \
       (SELECT count(*) FROM projection.containment WHERE depth = 1) AS edges, \
       (SELECT coalesce(max(depth), 0) FROM projection.containment) AS max_depth, \
       'symbols.tsv indexes items, not modules, so every owner_path names something that is not \
a definition. A family that indexes modules would fill this unchanged.' AS limited_by \
UNION ALL \
SELECT 'dependency', \
       'program.dependency_edge.package_key -> dep_package_key', \
       (SELECT count(*) FROM package WHERE retracted = false), \
       (SELECT count(*) FROM projection.dependency_closure WHERE depth = 1), \
       (SELECT coalesce(max(depth), 0) FROM projection.dependency_closure), \
       'resolved from a pinned subject manifest, so resolve_present is true and this closure \
traverses. It refuses when that is false -- PL001 measured that a null resolve is not the same as \
none, which is the answer --no-deps leaves behind.' \
UNION ALL \
SELECT 'reexport', \
       'program.export_path.access_path -> canonical_path', \
       (SELECT count(*) FROM export_path WHERE retracted = false), \
       (SELECT count(*) FROM projection.re_export_chain WHERE depth = 1), \
       (SELECT coalesce(max(depth), 0) FROM projection.re_export_chain), \
       'every alias resolves to a definition in one hop; no canonical_path is itself an \
access_path, so nothing chains. Longer chains are representable, not present.'";

/// Every canonical record a surface entry can be characterised by.
///
/// Coverage used to join `surface_entry.mechanism_key` against `catalog.mechanism`, which asked
/// whether each inventory item had become a *mechanism*. Three quarters of the inventory never
/// can: an exit code becomes a result contract, a file type or language a search domain, an
/// unreachable capability a negative-space row. Joining against the union asks the question
/// coverage was always meant to ask -- has this been turned into something -- and the `kind`
/// column keeps the four distinguishable rather than blurring them into one count.
const CATALOG_RECORD: &str = "\
SELECT 'mechanism' AS kind, entity_key, entity_id FROM mechanism WHERE retracted = false \
UNION ALL SELECT 'result_contract', entity_key, entity_id \
FROM result_contract WHERE retracted = false \
UNION ALL SELECT 'search_domain', entity_key, entity_id \
FROM search_domain WHERE retracted = false \
UNION ALL SELECT 'negative_space', entity_key, entity_id \
FROM negative_space WHERE retracted = false";

/// Everything a `catalog.surface_binding` may legally point at, on either side.
///
/// `surface_binding` became a subject/object relation when the `library-api` family arrived, so
/// that §3.8's claim -- that this table is "the entire answer to how the capability catalog and the
/// Rust program model meet" -- is true of the table rather than only of the design. The two halves
/// mint keys in one grammar (§1.2), so the bridge is an ordinary join and this view is the domain
/// both ends are checked against.
///
/// `kind` is carried beside the key rather than inferred from the key's prefix. A key namespace is
/// a convention the minting functions keep; a column is a fact the row asserts, and
/// `violations_dangling_binding` can then catch the two disagreeing.
const BINDABLE: &str = "\
SELECT 'capability' AS kind, entity_key FROM capability WHERE retracted = false \
UNION ALL SELECT 'mechanism', entity_key FROM mechanism WHERE retracted = false \
UNION ALL SELECT 'definition', entity_key FROM definition WHERE retracted = false \
UNION ALL SELECT 'package', entity_key FROM package WHERE retracted = false";

/// What a command's exit status means, and the trap in reading it across tools.
const RESULT_CONTRACT: &str = "\
SELECT r.entity_key, r.tool, r.command, r.code, r.meaning, r.trap \
FROM result_contract r \
WHERE r.retracted = false";

/// What each tool can be pointed at, by name.
const SEARCH_DOMAIN: &str = "\
SELECT d.entity_key, d.tool, d.domain_kind, d.name, d.selector, \
       d.truth_rank, lattice_label('truth', d.truth_rank) AS supported, \
       d.grammar_kind_count, d.grammar_field_count, d.rule_schema_available \
FROM search_domain d \
WHERE d.retracted = false";

/// Capabilities known **not** to be reachable, with what to do instead.
///
/// This is what lets a query return a useful negative. An agent asking for in-place rewriting with
/// ripgrep is told that ripgrep never modifies a file by design and pointed at `ast-grep -U`, in
/// place of an empty result set it would read as "nothing found, search harder".
const NEGATIVE_SPACE: &str = "\
SELECT n.entity_key, n.tool, n.capability_text, n.why, n.instead_text, n.instead_key, \
       n.truth_rank, lattice_label('truth', n.truth_rank) AS reachable \
FROM negative_space n \
WHERE n.retracted = false";

/// Surface coverage: how much of the tool is catalogued at all.
const COVERAGE_SURFACE: &str = "\
SELECT s.tool, s.source_table, \
       count(*) AS surface_entries, \
       count(c.entity_key) AS characterised, \
       count(*) - count(c.entity_key) AS uncharacterised, \
       array_to_string(array_agg(DISTINCT c.kind ORDER BY c.kind), ', ') AS characterised_as \
FROM surface_entry s \
LEFT JOIN projection.catalog_record c ON c.entity_key = s.catalog_key \
WHERE s.retracted = false \
GROUP BY s.tool, s.source_table";

/// API coverage: how much of the program model any capability mechanism is bound to.
///
/// The **third** independent number, and like the other two it is never summed with them. Surface
/// coverage asks how much of the tool a user can see is catalogued; evidence coverage asks how much
/// of that rests on something that ran; this asks how much of the tool's own code the catalog can
/// point at from a capability. They have different denominators and different remedies.
///
/// It is expected to be small, and small is the honest answer. Nothing the skill ships links a
/// flag to a crate, so every row here is authored with its evidence attached -- and a number that
/// says "fourteen of 323" is worth more than a derived number that says "all of them" on the
/// strength of a crate-name match.
const COVERAGE_API: &str = "\
SELECT 'definition' AS object_kind, \
       (SELECT count(*) FROM definition WHERE retracted = false) AS program_rows, \
       count(DISTINCT b.subject_key) AS mechanisms_bound \
FROM surface_binding b \
WHERE b.retracted = false AND b.subject_kind = 'mechanism' AND b.object_kind = 'definition' \
UNION ALL \
SELECT 'package', \
       (SELECT count(*) FROM package WHERE retracted = false), \
       count(DISTINCT b.subject_key) \
FROM surface_binding b \
WHERE b.retracted = false AND b.subject_kind = 'mechanism' AND b.object_kind = 'package'";

/// A mechanism and the code that implements it, with the claim's evidence attached.
///
/// This is §3.8's join made queryable: "how do the capability catalog and the Rust program model
/// meet". Every row carries `evidence_note` and both lattices, because the whole point is that
/// these claims differ in strength -- a crate the skill's own prose names is not the same kind of
/// fact as a builder type inferred from its name, and a reader has to be able to tell.
const IMPLEMENTS: &str = "\
SELECT b.subject_key AS mechanism_key, b.role, b.object_kind, b.object_key, \
       lattice_label('precision', b.precision_rank) AS precision, \
       lattice_label('observed', b.observed_rank) AS observed, \
       b.evidence_note \
FROM surface_binding b \
WHERE b.retracted = false AND b.subject_kind = 'mechanism' \
ORDER BY b.subject_key, b.object_kind, b.object_key";

/// Evidence coverage: how much of what is catalogued rests on something that was actually run.
///
/// The `confirmed` threshold is read from `catalog.lattice` rather than written as a literal, so
/// adding a value to the `observed` lattice cannot leave a stale bound behind. `weakest` is the
/// `min` fold of §5 applied to a real question: the strongest claim a group as a whole supports is
/// its weakest member's.
const COVERAGE_EVIDENCE: &str = "\
WITH threshold AS ( \
  SELECT rank AS confirmed_rank FROM lattice WHERE lattice = 'observed' AND label = 'confirmed' \
) \
SELECT m.tool, m.surface, \
       count(*) AS mechanisms, \
       sum(CASE WHEN m.observed_rank >= t.confirmed_rank THEN 1 ELSE 0 END) AS confirmed, \
       min(m.observed_rank) AS weakest_rank, \
       lattice_label('observed', min(m.observed_rank)) AS weakest \
FROM mechanism m CROSS JOIN threshold t \
WHERE m.retracted = false \
GROUP BY m.tool, m.surface";

/// Every canonical row's identity columns and its content digest, across **every** snapshot.
///
/// The one relation in the catalog that deliberately is not snapshot-scoped, because `compare`
/// needs two snapshots at once and a base view only ever serves the selected one. It reads the
/// `__all` providers directly for that reason, the same way `visible_run` does, and re-states the
/// run filter itself rather than inheriting it.
///
/// **A table added to the model and forgotten here is checked by nothing**, and that is not
/// hypothetical: `plan_fragment`, `interaction` and `interaction_atom` were each written, queried
/// and shipped before anyone noticed their 73 rows were outside every identity invariant. Nothing
/// failed -- `validate` reported nine clean invariants over 1,769 rows while the catalog held
/// 1,842. The `checked` column is what made it visible, which is the argument for printing a
/// denominator beside every count.
///
/// `snapshot.extraction_run` is exempt from the run filter here for the same reason it is exempt
/// at the base-table boundary: filtering the table that decides visibility by visibility is
/// circular. The exemption is spelled out in the predicate rather than read from [`UNFILTERED`],
/// because a base view is generated per table and this is one statement over nineteen -- so the
/// list cannot be consulted, only restated, and restating it visibly beats hiding it.
const ENTITY_ALL: &str = "\
SELECT * FROM ( \
  SELECT 'catalog.capability' AS source_table, entity_id, entity_key, snapshot_id, run_id, retracted, payload_digest \
  FROM capability__all \
  UNION ALL SELECT 'catalog.mechanism', entity_id, entity_key, snapshot_id, run_id, retracted, payload_digest \
  FROM mechanism__all \
  UNION ALL SELECT 'catalog.surface_binding', entity_id, entity_key, snapshot_id, run_id, retracted, payload_digest \
  FROM surface_binding__all \
  UNION ALL SELECT 'catalog.surface_entry', entity_id, entity_key, snapshot_id, run_id, retracted, payload_digest \
  FROM surface_entry__all \
  UNION ALL SELECT 'catalog.parameter', entity_id, entity_key, snapshot_id, run_id, retracted, payload_digest \
  FROM parameter__all \
  UNION ALL SELECT 'catalog.result_contract', entity_id, entity_key, snapshot_id, run_id, retracted, payload_digest \
  FROM result_contract__all \
  UNION ALL SELECT 'catalog.search_domain', entity_id, entity_key, snapshot_id, run_id, retracted, payload_digest \
  FROM search_domain__all \
  UNION ALL SELECT 'catalog.negative_space', entity_id, entity_key, snapshot_id, run_id, retracted, payload_digest \
  FROM negative_space__all \
  UNION ALL SELECT 'catalog.plan_fragment', entity_id, entity_key, snapshot_id, run_id, retracted, payload_digest \
  FROM plan_fragment__all \
  UNION ALL SELECT 'catalog.interaction', entity_id, entity_key, snapshot_id, run_id, retracted, payload_digest \
  FROM interaction__all \
  UNION ALL SELECT 'catalog.interaction_atom', entity_id, entity_key, snapshot_id, run_id, retracted, payload_digest \
  FROM interaction_atom__all \
  UNION ALL SELECT 'evidence.behavior_assertion', entity_id, entity_key, snapshot_id, run_id, retracted, payload_digest \
  FROM behavior_assertion__all \
  UNION ALL SELECT 'snapshot.extraction_run', entity_id, entity_key, snapshot_id, run_id, retracted, payload_digest \
  FROM extraction_run__all \
  UNION ALL SELECT 'snapshot.native_binding', entity_id, entity_key, snapshot_id, run_id, retracted, payload_digest \
  FROM native_binding__all \
  UNION ALL SELECT 'program.package', entity_id, entity_key, snapshot_id, run_id, retracted, payload_digest \
  FROM package__all \
  UNION ALL SELECT 'program.definition', entity_id, entity_key, snapshot_id, run_id, retracted, payload_digest \
  FROM definition__all \
  UNION ALL SELECT 'program.signature', entity_id, entity_key, snapshot_id, run_id, retracted, payload_digest \
  FROM signature__all \
  UNION ALL SELECT 'program.implementation', entity_id, entity_key, snapshot_id, run_id, retracted, payload_digest \
  FROM implementation__all \
  UNION ALL SELECT 'program.export_path', entity_id, entity_key, snapshot_id, run_id, retracted, payload_digest \
  FROM export_path__all \
  UNION ALL SELECT 'program.crate_unit', entity_id, entity_key, snapshot_id, run_id, retracted, payload_digest \
  FROM crate_unit__all \
  UNION ALL SELECT 'program.feature_declaration', entity_id, entity_key, snapshot_id, run_id, retracted, payload_digest \
  FROM feature_declaration__all \
  UNION ALL SELECT 'program.dependency_edge', entity_id, entity_key, snapshot_id, run_id, retracted, payload_digest \
  FROM dependency_edge__all \
) e \
WHERE e.retracted = false \
  AND (e.source_table = 'snapshot.extraction_run' \
       OR e.run_id IN (SELECT run_id FROM projection.visible_run))";

/// Every canonical row's three identity columns, in the snapshot the caller is asking about.
///
/// Existing so the identity invariants are written once instead of once per table. It is now one
/// line over [`ENTITY_ALL`]: the nineteen arms live in exactly one place, so the "added and
/// forgotten" failure above has one place to happen rather than two.
const IDENTITY: &str = "\
SELECT source_table, entity_id, entity_key, snapshot_id \
FROM projection.entity_all \
WHERE snapshot_id IN (SELECT snapshot_id FROM projection.selected_snapshot)";

/// What changed between two snapshots, joined on `entity_key` and on nothing else.
///
/// The only projection that crosses snapshots, and the reason the two-key identity design is
/// load-bearing rather than asserted. `entity_id` is `blake3(entity_key ‖ snapshot_id)`, so it
/// differs between two snapshots for the *same* entity by construction -- joining on it would
/// return zero rows and report a total rewrite of everything. `entity_key` is stable across
/// snapshots on purpose, and this is what that purpose is for.
///
/// Three kinds, and they are the surrounding project's own vocabulary rather than a new one:
/// `crates/enrichment-core/src/compare.rs` fixes `added` / `removed` / `changed` under a module
/// whose first line is *"Deterministic differences between immutable observations, not a
/// compatibility proof."* This view emits no severity, no score and no verdict, for the same
/// reason §5.4 refuses a composite coverage number.
///
/// `changed` is decided by `payload_digest`, because nothing else can decide it uniformly. The
/// alternative -- comparing the epistemic columns -- would have caught a verdict upgrade and
/// silently missed a changed summary, and a false "no change" is the failure direction this
/// catalog exists to refuse.
///
/// # Two deliberate exclusions
///
/// **`snapshot.extraction_run` is excluded.** A run key is `run:<snapshot>/<family>/0`, so it
/// embeds the snapshot and can never match across one: every run would report as both added and
/// removed, on every comparison, forever. That is churn by construction, and it is the same
/// argument §1.2 uses for keeping versions out of mechanism keys.
///
/// **A retracted row is already gone** -- [`ENTITY_ALL`] filters it -- so a retraction shows up
/// here as `removed`. That is the right answer and it is worth saying why it is not double
/// counting: `retracted` withdraws a fact *within* a snapshot, while absence from a later snapshot
/// is a different observation about a different extraction. Both mechanisms are kept; only one of
/// them is a `compare` concern.
const COMPARE: &str = "\
SELECT coalesce(a.source_table, b.source_table) AS source_table, \
       coalesce(a.entity_key, b.entity_key)     AS entity_key, \
       CASE WHEN a.entity_key IS NULL THEN 'added' \
            WHEN b.entity_key IS NULL THEN 'removed' \
            ELSE 'changed' END                  AS kind, \
       a.entity_id AS before_id, \
       b.entity_id AS after_id \
FROM   (SELECT * FROM projection.entity_all \
        WHERE snapshot_id = (SELECT from_snapshot FROM request) \
          AND source_table <> 'snapshot.extraction_run') a \
FULL OUTER JOIN \
       (SELECT * FROM projection.entity_all \
        WHERE snapshot_id = (SELECT to_snapshot FROM request) \
          AND source_table <> 'snapshot.extraction_run') b \
  ON   b.entity_key = a.entity_key AND b.source_table = a.source_table \
WHERE  a.entity_key IS NULL \
   OR  b.entity_key IS NULL \
   OR  a.payload_digest <> b.payload_digest";

/// A surface entry naming a mechanism that does not exist.
///
/// This is the one that makes the coverage `LEFT JOIN` honest: without it, a dangling key would
/// count as characterised in any formulation that trusted the entry side's own claim.
const V_DANGLING_SURFACE_ENTRY: &str = "\
SELECT 'surface_entry.catalog_key resolves' AS invariant, \
       s.entity_key AS subject, \
       s.catalog_kind || ' ' || s.catalog_key AS detail \
FROM surface_entry s \
LEFT JOIN projection.catalog_record c ON c.entity_key = s.catalog_key \
WHERE s.retracted = false AND s.catalog_key IS NOT NULL AND c.entity_key IS NULL";

/// A surface entry whose `catalog_kind` names a different table from the one its key is in.
///
/// The kind is denormalised so a reader does not have to search four tables to find a record. A
/// denormalised column that disagrees with the thing it describes is worse than no column at all,
/// so the disagreement is checked rather than assumed away.
const V_MISLABELLED_KIND: &str = "\
SELECT 'surface_entry.catalog_kind names the right table' AS invariant, \
       s.entity_key AS subject, \
       'says ' || s.catalog_kind || ', found in ' || c.kind AS detail \
FROM surface_entry s \
JOIN projection.catalog_record c ON c.entity_key = s.catalog_key \
WHERE s.retracted = false AND s.catalog_kind <> c.kind";

/// A binding naming a capability or a mechanism that does not exist.
const V_DANGLING_BINDING: &str = "\
SELECT 'surface_binding.subject_key resolves' AS invariant, \
       b.entity_key AS subject, b.subject_kind || ' ' || b.subject_key AS detail \
FROM surface_binding b \
LEFT JOIN projection.bindable r ON r.entity_key = b.subject_key AND r.kind = b.subject_kind \
WHERE b.retracted = false AND r.entity_key IS NULL \
UNION ALL \
SELECT 'surface_binding.object_key resolves', b.entity_key, \
       b.object_kind || ' ' || b.object_key \
FROM surface_binding b \
LEFT JOIN projection.bindable r ON r.entity_key = b.object_key AND r.kind = b.object_kind \
WHERE b.retracted = false AND r.entity_key IS NULL";

/// A canonical record no surface entry accounts for.
///
/// The coverage denominator is `catalog.surface_entry`, so a record missing from it is invisible
/// to every coverage number while still being returned by `discover` -- a catalog that reports
/// less than it contains. This fires when the two sides mint keys by rules that have drifted
/// apart, which is precisely the failure the two-independent-sides design is meant to expose
/// rather than hide.
const V_ORPHAN_MECHANISM: &str = "\
SELECT 'every catalog record appears in surface_entry' AS invariant, \
       c.entity_key AS subject, \
       c.kind AS detail \
FROM projection.catalog_record c \
LEFT JOIN surface_entry s ON s.catalog_key = c.entity_key AND s.retracted = false \
WHERE s.entity_key IS NULL";

/// A denormalised count that no longer agrees with the rows it counts.
const V_STALE_MECHANISM_COUNT: &str = "\
SELECT 'capability.mechanism_count agrees with its bindings' AS invariant, \
       c.entity_key AS subject, \
       'stored ' || cast(c.mechanism_count AS VARCHAR) \
         || ', actual ' || cast(coalesce(b.n, 0) AS VARCHAR) AS detail \
FROM capability c \
LEFT JOIN ( \
  SELECT subject_key, count(*) AS n FROM surface_binding \
  WHERE retracted = false AND subject_kind = 'capability' AND object_kind = 'mechanism' \
  GROUP BY subject_key \
) b ON b.subject_key = c.entity_key \
WHERE c.retracted = false AND c.mechanism_count <> coalesce(b.n, 0)";

/// An assertion whose subject claims to be a mechanism but names none.
///
/// Scoped to `subject_kind = 'mechanism'` deliberately: an assertion about a topic is not a
/// dangling mechanism reference, it is a reference into a kind the catalog does not yet model.
/// Conflating the two would report a modelling gap as a data error.
const V_DANGLING_ASSERTION_SUBJECT: &str = "\
SELECT 'behavior_assertion.subject resolves' AS invariant, \
       a.entity_key AS subject, \
       a.subject_entity_id AS detail \
FROM behavior_assertion a \
LEFT JOIN mechanism m ON m.entity_id = a.subject_entity_id AND m.retracted = false \
WHERE a.retracted = false AND a.subject_kind = 'mechanism' AND m.entity_id IS NULL";

/// Two rows sharing one `entity_id`.
///
/// `entity_id` is the declared primary key, and PB02 measured the optimiser acting on that
/// declaration -- eliminating an aggregation from `SELECT DISTINCT` and turning a self-join into a
/// semi-join. A duplicate therefore does not merely misreport a count; it makes the engine's
/// rewrites unsound. A key rule that collided in two scopes is how this was first found.
const V_DUPLICATE_ENTITY_ID: &str = "\
SELECT 'entity_id is unique within its table' AS invariant, \
       source_table || ' ' || entity_id AS subject, \
       cast(count(*) AS VARCHAR) || ' rows share this id' AS detail \
FROM projection.identity \
GROUP BY source_table, entity_id \
HAVING count(*) > 1";

/// A stored id that is not the digest of its own key and snapshot.
///
/// `entity_id` is *defined* as a function of the other two names, so the definition is checkable
/// rather than merely documented. Today the only writer derives it correctly by construction; this
/// is what keeps that true when there is a second one.
const V_UNDERIVED_ENTITY_ID: &str = "\
SELECT 'entity_id is the digest of its key and snapshot' AS invariant, \
       source_table || ' ' || entity_key AS subject, \
       'stored ' || entity_id AS detail \
FROM projection.identity \
WHERE entity_id <> derived_entity_id(entity_key, snapshot_id)";

/// Every violation, under the name of the invariant it breaks.
const VIOLATIONS: &str = "\
SELECT * FROM projection.violations_dangling_surface_entry \
UNION ALL SELECT * FROM projection.violations_dangling_binding \
UNION ALL SELECT * FROM projection.violations_orphan_mechanism \
UNION ALL SELECT * FROM projection.violations_mislabelled_kind \
UNION ALL SELECT * FROM projection.violations_stale_mechanism_count \
UNION ALL SELECT * FROM projection.violations_dangling_assertion_subject \
UNION ALL SELECT * FROM projection.violations_duplicate_entity_id \
UNION ALL SELECT * FROM projection.violations_underived_entity_id";

/// Each invariant, and how many rows it inspects.
///
/// This is the denominator that stops a vacuous check reading as a passing one. An invariant whose
/// `checked` is zero examined nothing, and saying so is the same discipline the surrounding skills
/// apply to search results: silence is not absence.
const INVARIANT: &str = "\
SELECT 'surface_entry.catalog_key resolves' AS invariant, \
       (SELECT count(*) FROM surface_entry \
        WHERE retracted = false AND catalog_key IS NOT NULL) AS checked \
UNION ALL SELECT 'surface_entry.catalog_kind names the right table', \
       (SELECT count(*) FROM surface_entry \
        WHERE retracted = false AND catalog_key IS NOT NULL) \
UNION ALL SELECT 'surface_binding.subject_key resolves', \
       (SELECT count(*) FROM surface_binding WHERE retracted = false) \
UNION ALL SELECT 'surface_binding.object_key resolves', \
       (SELECT count(*) FROM surface_binding WHERE retracted = false) \
UNION ALL SELECT 'every catalog record appears in surface_entry', \
       (SELECT count(*) FROM projection.catalog_record) \
UNION ALL SELECT 'capability.mechanism_count agrees with its bindings', \
       (SELECT count(*) FROM capability WHERE retracted = false) \
UNION ALL SELECT 'behavior_assertion.subject resolves', \
       (SELECT count(*) FROM behavior_assertion \
        WHERE retracted = false AND subject_kind = 'mechanism') \
UNION ALL SELECT 'entity_id is unique within its table', \
       (SELECT count(*) FROM projection.identity) \
UNION ALL SELECT 'entity_id is the digest of its key and snapshot', \
       (SELECT count(*) FROM projection.identity)";

/// Every invariant, what it inspected, and what it found.
///
/// `FULL OUTER` rather than `LEFT`, so the two failure directions are both visible: an invariant
/// declared but never violated shows `violations = 0`, and a violation reported under a name no
/// invariant declares shows `checked` as null instead of disappearing. The second case is a typo
/// in one of two places that must agree, which is exactly the drift a join can catch and a
/// convention cannot.
const VALIDATE: &str = "\
SELECT coalesce(i.invariant, v.invariant) AS invariant, \
       i.checked, \
       coalesce(v.violations, 0) AS violations \
FROM projection.invariant i \
FULL OUTER JOIN ( \
  SELECT invariant, count(*) AS violations FROM projection.violations GROUP BY invariant \
) v ON v.invariant = i.invariant";

/// Every projection, in dependency order.
///
/// Order matters: a view referencing another must be registered after it, because the planner
/// resolves the reference when the `CREATE VIEW` is planned rather than when it is queried.
pub const PROJECTIONS: &[Projection] = &[
    Projection {
        name: VISIBLE_RUN_NAME,
        about: "Which runs a caller may see. Every base table is filtered through this one view.",
        sql: VISIBLE_RUN,
    },
    Projection {
        name: NEWEST_SNAPSHOT_NAME,
        about: "The newest snapshot with a visible run. Zero rows or one.",
        sql: NEWEST_SNAPSHOT,
    },
    Projection {
        name: SELECTED_SNAPSHOT_NAME,
        about: "Which snapshot a caller is asking about. Every base table is filtered through this one view.",
        sql: SELECTED_SNAPSHOT,
    },
    Projection {
        name: "extraction_run",
        about: "Every extraction run, finished or not. Not filtered by visibility -- it is what explains an absence.",
        sql: EXTRACTION_RUN,
    },
    Projection {
        name: "mechanism",
        about: "Mechanisms, with lattice ordinals and their labels.",
        sql: MECHANISM,
    },
    Projection {
        name: "parameter",
        about: "Parameters, with the three defaults kept separate.",
        sql: PARAMETER,
    },
    Projection {
        name: "mechanism_ability",
        about: "The abilities each mechanism serves, folded to one row.",
        sql: MECHANISM_ABILITY,
    },
    Projection {
        name: "mechanism_detail",
        about: "Everything known about a mechanism, in one row.",
        sql: MECHANISM_DETAIL,
    },
    Projection {
        name: "discover",
        about: "Mechanisms reached through a capability. Inner join by design.",
        sql: DISCOVER,
    },
    Projection {
        name: "capability",
        about: "The abstract abilities and their facets.",
        sql: CAPABILITY,
    },
    Projection {
        name: "plan_fragment",
        about: "Composable steps, with recall rendered and obligations visible.",
        sql: PLAN_FRAGMENT,
    },
    Projection {
        name: "fragment_edge",
        about: "Which fragment may follow which. The composition rule, as a join.",
        sql: FRAGMENT_EDGE,
    },
    Projection {
        name: "chain",
        about: "Bounded chains from the requested unit, recall folded, with a path witness.",
        sql: CHAIN,
    },
    Projection {
        name: "interaction",
        about: "How one option changes what another does, with its predicate rendered.",
        sql: INTERACTION,
    },
    Projection {
        name: "interaction_truth",
        about: "Every interaction evaluated against the request. min then max, over `truth`.",
        sql: INTERACTION_TRUTH,
    },
    Projection {
        name: "bindable",
        about: "Everything a surface binding may point at, on either side, with its kind.",
        sql: BINDABLE,
    },
    Projection {
        name: "containment",
        about: "What a definition contains, transitively. Empty here -- see `closure` for why.",
        sql: CONTAINMENT,
    },
    Projection {
        name: "re_export_chain",
        about: "Where an item can be named from, following re-exports to their origin.",
        sql: RE_EXPORT_CHAIN,
    },
    Projection {
        name: "dependency_closure",
        about: "Which packages a package depends on, transitively. Refuses to traverse unless resolve_present.",
        sql: DEPENDENCY_CLOSURE,
    },
    Projection {
        name: "closure",
        about: "Each closure with its edge count and depth, so an empty answer is not silence.",
        sql: CLOSURE,
    },
    Projection {
        name: "catalog_record",
        about: "Every canonical record a surface entry can be characterised by.",
        sql: CATALOG_RECORD,
    },
    Projection {
        name: "result_contract",
        about: "What a command's exit status means, and the trap in reading it.",
        sql: RESULT_CONTRACT,
    },
    Projection {
        name: "search_domain",
        about: "What each tool can be pointed at, by name.",
        sql: SEARCH_DOMAIN,
    },
    Projection {
        name: "negative_space",
        about: "Capabilities known not to be reachable, with what to do instead.",
        sql: NEGATIVE_SPACE,
    },
    Projection {
        name: "coverage_surface",
        about: "How much of the tool is catalogued. Denominator is independent of the numerator.",
        sql: COVERAGE_SURFACE,
    },
    Projection {
        name: "coverage_api",
        about: "The third coverage number: how much of the program model a mechanism can point at.",
        sql: COVERAGE_API,
    },
    Projection {
        name: "implements",
        about: "A mechanism and the code implementing it, with the evidence for the claim.",
        sql: IMPLEMENTS,
    },
    Projection {
        name: "coverage_evidence",
        about: "How much of what is catalogued was actually run. Never combined with the above.",
        sql: COVERAGE_EVIDENCE,
    },
    Projection {
        name: "entity_all",
        about: "Every canonical row's identity and content digest, across every snapshot. The one relation that is not snapshot-scoped.",
        sql: ENTITY_ALL,
    },
    Projection {
        name: "identity",
        about: "Every canonical row's three identity columns, in one relation.",
        sql: IDENTITY,
    },
    Projection {
        name: "compare",
        about: "What changed between two snapshots. Joins on entity_key and on nothing else.",
        sql: COMPARE,
    },
    Projection {
        name: "violations_dangling_surface_entry",
        about: "A surface entry naming a mechanism that does not exist.",
        sql: V_DANGLING_SURFACE_ENTRY,
    },
    Projection {
        name: "violations_dangling_binding",
        about: "A binding naming a capability or mechanism that does not exist.",
        sql: V_DANGLING_BINDING,
    },
    Projection {
        name: "violations_orphan_mechanism",
        about: "A catalog record no surface entry accounts for, so no coverage number sees it.",
        sql: V_ORPHAN_MECHANISM,
    },
    Projection {
        name: "violations_mislabelled_kind",
        about: "A surface entry whose catalog_kind names a different table from its key's.",
        sql: V_MISLABELLED_KIND,
    },
    Projection {
        name: "violations_stale_mechanism_count",
        about: "A denormalised count that disagrees with the rows it counts.",
        sql: V_STALE_MECHANISM_COUNT,
    },
    Projection {
        name: "violations_dangling_assertion_subject",
        about: "An assertion claiming a mechanism subject that names none.",
        sql: V_DANGLING_ASSERTION_SUBJECT,
    },
    Projection {
        name: "violations_duplicate_entity_id",
        about: "Two rows sharing a declared primary key, which makes the optimiser's rewrites \
                unsound.",
        sql: V_DUPLICATE_ENTITY_ID,
    },
    Projection {
        name: "violations_underived_entity_id",
        about: "A stored id that is not the digest of its own key and snapshot.",
        sql: V_UNDERIVED_ENTITY_ID,
    },
    Projection {
        name: "violations",
        about: "Every violation, under the name of the invariant it breaks.",
        sql: VIOLATIONS,
    },
    Projection {
        name: "invariant",
        about: "Each invariant and how many rows it inspects, so a vacuous check is visible.",
        sql: INVARIANT,
    },
    Projection {
        name: "validate",
        about: "Every invariant, what it inspected, and what it found.",
        sql: VALIDATE,
    },
];

/// The suffix the raw providers are registered under, so the unsuffixed name can be the filtered
/// view. Exported because the query binary and the fixture tests must both use it.
pub const RAW_SUFFIX: &str = "__all";

/// Base tables neither the run filter nor the snapshot filter applies to.
///
/// Both exemptions are load-bearing and both now cover two rules rather than one.
/// `extraction_run` decides visibility, so filtering it by visibility would be circular -- and
/// scoping it to one snapshot would hide the very rows that tell a caller which snapshots exist,
/// which is the question `runs` is there to answer. `lattice` has neither `run_id` nor
/// `snapshot_id`: rank-to-label is not an observation about a snapshot.
pub const UNFILTERED: &[&str] = &["lattice", "extraction_run"];

/// The name of the projection that decides what every other projection can see. It is created by
/// [`register_visibility`] rather than by [`register`], because the base views are defined over it
/// and a view's references resolve when its `CREATE` is planned.
pub const VISIBLE_RUN_NAME: &str = "visible_run";

/// Which runs a caller may see.
///
/// The completion test is a join to the published `lattice` rows rather than a literal ordinal or
/// a `lattice_label` call. The ordinal would drift silently the first time a value was inserted
/// into the lattice; the UDF would put a function between the column and the comparison, which is
/// the pruning loss the `Int8` encoding exists to avoid. The join asks the reference table, which
/// is why that table is published as data.
const VISIBLE_RUN: &str = "\
SELECT r.entity_key AS run_id, r.family, r.completion_rank, l.label AS completion \
FROM extraction_run__all r \
CROSS JOIN request q \
JOIN lattice__all l ON l.lattice = 'completion' \
                   AND CAST(l.rank AS TINYINT) = CAST(r.completion_rank AS TINYINT) \
WHERE r.retracted = false AND (l.label = 'complete' OR q.include_partial)";

/// The name of the projection resolving the newest snapshot a caller could be shown.
pub const NEWEST_SNAPSHOT_NAME: &str = "newest_snapshot";

/// The name of the projection deciding **which snapshot** every other projection reads.
pub const SELECTED_SNAPSHOT_NAME: &str = "selected_snapshot";

/// The newest snapshot with a visible run: zero rows or one, never more.
///
/// Split out from [`SELECTED_SNAPSHOT`] rather than inlined as a scalar subquery, and not for
/// tidiness. A scalar subquery under a `coalesce` plans with a schema claiming the column is
/// non-nullable and then decorrelates into a join that can produce a null anyway, which surfaces
/// as `Column 'snapshot_id' is declared as non-nullable but contains null values` from somewhere
/// with no obvious relationship to the cause. Two relations combined by `UNION ALL` leave no such
/// gap between the declared shape and the executed one.
///
/// "Newest" is by the latest `started_at` among the runs that wrote it -- started rather than
/// finished, because a snapshot's families finish at different times and the question is which
/// extraction is most recent, not which one took longest.
const NEWEST_SNAPSHOT: &str = "\
SELECT r.snapshot_id AS snapshot_id \
FROM extraction_run__all r \
JOIN projection.visible_run v ON v.run_id = r.entity_key \
WHERE r.retracted = false \
GROUP BY r.snapshot_id \
ORDER BY max(r.started_at) DESC \
LIMIT 1";

/// Which snapshot a caller is asking about.
///
/// Until phase 7 the catalog held one snapshot and no view mentioned `snapshot_id`, so every view
/// silently unioned all of them -- correct for exactly as long as there was only one. With two,
/// `describe` would return two rows per key and every `coverage_*` count would double, with
/// nothing raising an error. The rule therefore lives here, in one statement, for the same reason
/// the run filter does: thirty-seven views is thirty-seven chances to write it differently, and
/// the one that forgot would be the one nobody noticed.
///
/// The two branches are mutually exclusive by construction, so the `UNION ALL` yields exactly one
/// row whenever an answer exists. When it yields **none** -- no visible run, and no snapshot named
/// -- every base table serves nothing, which is the conclusion the run filter reaches on its own
/// and is the honest answer to "what does a catalog with no finished build contain".
///
/// An empty `request.snapshot` means "whichever is newest". A default rather than an error,
/// because the ordinary catalog holds one snapshot and making every caller name it would be
/// ceremony -- but never a *silent* default: `codesearch-query runs` prints which snapshot a
/// request resolved to, because a catalog holding two and answering about one has to say which.
const SELECTED_SNAPSHOT: &str = "\
SELECT q.snapshot AS snapshot_id FROM request q WHERE q.snapshot <> '' \
UNION ALL \
SELECT n.snapshot_id FROM projection.newest_snapshot n CROSS JOIN request q WHERE q.snapshot = ''";

/// Install the whole retrieval surface on a session, resolved on demand.
///
/// Replaces what used to be two eager passes -- `CREATE OR REPLACE VIEW` for twenty base views and
/// thirty-two projections. Both are now [`lazy::LazySchema`]s: enumerable without being built, and
/// built only for the names a query actually mentions. See that module for the measurement.
///
/// `raw` resolves the `<name>__all` providers. The query binary supplies one that opens Delta
/// tables; the fixture tests pass [`lazy::Injected`] and register `MemTable`s directly, so they
/// exercise these exact views rather than a parallel registration path.
pub fn install(
    ctx: &SessionContext,
    base_tables: &[&str],
    raw: std::sync::Arc<dyn lazy::Resolver>,
) -> DFResult<()> {
    // The run-filtered base views, one per table.
    //
    // The pass-through for an exempt table is written out rather than skipped, so that every base
    // name resolves the same way -- through a view over `<name>__all` -- and an exemption is a
    // visible `SELECT *` instead of a missing registration nobody can find.
    let base_views: Vec<(String, String)> = base_tables
        .iter()
        .map(|t| {
            let sql = if UNFILTERED.contains(t) {
                format!("SELECT * FROM {t}{RAW_SUFFIX}")
            } else {
                // Two rules, one place. Run visibility answers "was this row written by a build
                // that finished"; snapshot selection answers "is this row from the snapshot the
                // caller is asking about". They are independent facts -- a complete run of an
                // older snapshot is visible and not selected -- and conflating them would make
                // `--include-partial` silently change which snapshot you are looking at.
                format!(
                    "SELECT * FROM {t}{RAW_SUFFIX} \
                     WHERE run_id IN (SELECT run_id FROM {SCHEMA}.{VISIBLE_RUN_NAME}) \
                       AND snapshot_id IN (SELECT snapshot_id FROM {SCHEMA}.{SELECTED_SNAPSHOT_NAME})"
                )
            };
            ((*t).to_string(), sql)
        })
        .collect();

    let default_schema = std::sync::Arc::new(lazy::LazySchema::new(std::sync::Arc::new(
        lazy::Chain(vec![
            std::sync::Arc::new(lazy::SqlViews::new(base_views, ctx.state_weak_ref())),
            raw,
        ]),
    )));
    let projections: Vec<(String, String)> = PROJECTIONS
        .iter()
        .map(|p| (p.name.to_string(), p.sql.to_string()))
        .collect();
    let projection_schema = std::sync::Arc::new(lazy::LazySchema::new(std::sync::Arc::new(
        lazy::SqlViews::new(projections, ctx.state_weak_ref()),
    )));

    let catalog = ctx
        .catalog("datafusion")
        .ok_or_else(|| datafusion::error::DataFusionError::Plan("no default catalog".into()))?;
    catalog.register_schema("public", default_schema)?;
    catalog.register_schema(SCHEMA, projection_schema)?;
    Ok(())
}

/// Plan every projection, and fail if any will not.
///
/// §7's argument for eager registration: *"a projection that will not plan is a broken retrieval
/// surface, and discovering that at query time -- per caller, per invocation -- is how a catalog
/// ends up with a projection nobody has run in months."* That is right, and [`install`] does not
/// abandon it -- it moves it here, where a gate runs it once instead of every caller paying it.
///
/// Returns the number planned, so a caller can report a denominator rather than a bare "ok".
pub async fn check_all(ctx: &SessionContext) -> DFResult<usize> {
    let mut planned = 0;
    for p in PROJECTIONS {
        ctx.sql(&format!("SELECT * FROM {SCHEMA}.{} LIMIT 0", p.name))
            .await
            .map_err(|e| {
                datafusion::error::DataFusionError::Plan(format!(
                    "projection `{}` does not plan: {e}",
                    p.name
                ))
            })?;
        planned += 1;
    }
    Ok(planned)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_projection_has_a_unique_name() {
        let mut names: Vec<&str> = PROJECTIONS.iter().map(|p| p.name).collect();
        let before = names.len();
        names.sort_unstable();
        names.dedup();
        assert_eq!(before, names.len(), "two projections share a name");
    }

    /// A view referencing another must come after it, because the reference is resolved when the
    /// `CREATE VIEW` is planned. Getting this wrong is an error at registration rather than a
    /// wrong answer, but it is an error every caller would hit, so it is worth a cheap check.
    #[test]
    fn a_projection_only_references_projections_declared_before_it() {
        for (i, p) in PROJECTIONS.iter().enumerate() {
            for later in &PROJECTIONS[i + 1..] {
                let reference = format!("{SCHEMA}.{}", later.name);
                assert!(
                    !p.sql.contains(&reference),
                    "{} references {reference}, which is registered later",
                    p.name
                );
            }
        }
    }

    /// The names emitted by `violations_*` and the names declared by `invariant` must agree, or an
    /// invariant reports nothing forever while its rows pile up under a name nothing joins to.
    /// `validate`'s `FULL OUTER JOIN` exposes that at runtime; this exposes it without a catalog.
    #[test]
    fn every_violation_name_is_declared_as_an_invariant() {
        let declared = invariant_names(
            PROJECTIONS
                .iter()
                .find(|p| p.name == "invariant")
                .expect("the invariant view exists")
                .sql,
        );
        assert_eq!(
            declared.len(),
            9,
            "the extractor found {declared:?}; if the view changed shape, fix the extractor \
             rather than the expectation"
        );

        let mut emitted = Vec::new();
        for p in PROJECTIONS
            .iter()
            .filter(|p| p.name.starts_with("violations_"))
        {
            let names = invariant_names(p.sql);
            assert!(!names.is_empty(), "{} emits no invariant name", p.name);
            for name in names {
                assert!(
                    declared.contains(&name),
                    "{} emits `{name}`, which `invariant` does not declare",
                    p.name
                );
                emitted.push(name);
            }
        }

        // And the other direction: a declared invariant with no view behind it would always report
        // zero violations, which reads exactly like a passing check.
        for name in &declared {
            assert!(
                emitted.contains(name),
                "`{name}` is declared but no violations view can ever emit it"
            );
        }
    }

    /// The invariant names in a view's SQL.
    ///
    /// A name is a single-quoted literal that is either followed by ` AS invariant` or is the
    /// first item of a `UNION ALL SELECT`, which is how every view here is written. Extracting by
    /// shape rather than by guessing at content keeps `'stored '` and `'observed'` out of the
    /// result.
    fn invariant_names(sql: &str) -> Vec<&str> {
        const TAIL: &str = " AS invariant";
        const HEAD: &str = "UNION ALL SELECT '";
        let mut out = Vec::new();
        let mut rest = sql;
        while let Some(at) = rest.find(TAIL) {
            let before = &rest[..at];
            if before.ends_with('\'')
                && let Some(start) = before[..before.len() - 1].rfind('\'')
            {
                out.push(&before[start + 1..before.len() - 1]);
            }
            rest = &rest[at + TAIL.len()..];
        }
        let mut rest = sql;
        while let Some(at) = rest.find(HEAD) {
            let after = &rest[at + HEAD.len()..];
            if let Some(end) = after.find('\'') {
                out.push(&after[..end]);
            }
            rest = after;
        }
        out.sort_unstable();
        out.dedup();
        out
    }

    /// `CATALOG.md` is generated, so it goes stale the moment a function or a view is added
    /// without regenerating it.
    ///
    /// The full check is `just bridge-catalog` and a diff, but that needs a built catalog. This is
    /// the half that does not: every name that ought to appear is present. It catches the common
    /// staleness -- something new that was never published -- on a clean checkout, in a
    /// millisecond, with no Delta tables.
    #[test]
    fn the_generated_catalog_lists_every_projection_and_function() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("CATALOG.md");
        let Ok(published) = std::fs::read_to_string(&path) else {
            panic!("{} is missing; run `just bridge-catalog`", path.display());
        };
        for p in PROJECTIONS {
            assert!(
                published.contains(&format!("`{SCHEMA}.{}`", p.name)),
                "{} is not in CATALOG.md -- run `just bridge-catalog`",
                p.name
            );
        }
        for udf in crate::session::udfs() {
            assert!(
                published.contains(udf.name()),
                "{} is not in CATALOG.md -- run `just bridge-catalog`",
                udf.name()
            );
        }
    }

    /// A control for the extractor: it must actually pull names out, and must not pull out the
    /// string fragments that share a view with them. Without this the test above would pass just
    /// as happily on an extractor that returned nothing.
    #[test]
    fn the_invariant_name_extractor_finds_names_and_not_fragments() {
        let names = invariant_names(V_STALE_MECHANISM_COUNT);
        assert_eq!(
            names,
            vec!["capability.mechanism_count agrees with its bindings"]
        );
        assert!(
            !names.iter().any(|n| n.starts_with("stored ")),
            "`'stored '` is a display fragment, not an invariant name"
        );
        assert_eq!(invariant_names(V_DANGLING_BINDING).len(), 2);
    }
}
