# `bridge` -- everything added to the engine

**Generated** by `just bridge-catalog` against a live session. Do not edit: fix the code and regenerate.

This file exists so that "what have we added to DataFusion?" has an answer that cannot go stale. Every row below was read out of a running session rather than transcribed.

## Scalar functions

All are `Immutable`. PB06 measured why that is not a detail: a UDF that is not immutable loses its filter entirely, with no diagnostic.

| function | arguments | what it is for |
|---|---|---|
| `lattice_label('precision', precision_rank)` | `lattice_name`, `rank` | Render a stored lattice ordinal as its label. Display only -- filters compare the stored Int8 rank directly, which prunes without a function call. |
| `derived_entity_id(entity_key, snapshot_id)` | `entity_key`, `snapshot_id` | Recompute the per-snapshot entity id from the stable key and the snapshot. Used by projection.violations to check that stored ids really are derived; never in a CHECK constraint, which would make the table unwritable without this UDF. |
| `atom_truth(facet, op, value, value_list, negated, bindings)` | `facet`, `op`, `value`, `value_list`, `negated`, `bindings` | Evaluate one interaction literal against a request, as a rank in the truth lattice. A facet the request does not bind answers `unknown`, never `false`. |
| `entity_key_namespace(entity_key) = 'mech'` | `entity_key` | The namespace prefix of an entity_key, per the §1.2 grammar. Carries a preimage, so `entity_key_namespace(k) = 'mech'` is rewritten to a half-open range over the stored key column and can prune. |

## Projections

The retrieval surface, as registered views. A caller supplies predicates; the join discipline lives in `bridge/src/projection.rs` and is written once.

| view | what it answers |
|---|---|
| `projection.bindable` | Everything a surface binding may point at, on either side, with its kind. |
| `projection.capability` | The abstract abilities and their facets. |
| `projection.catalog_record` | Every canonical record a surface entry can be characterised by. |
| `projection.chain` | Bounded chains from the requested unit, recall folded, with a path witness. |
| `projection.closure` | Each closure with its edge count and depth, so an empty answer is not silence. |
| `projection.compare` | What changed between two snapshots. Joins on entity_key and on nothing else. |
| `projection.containment` | What a definition contains, transitively. Empty here -- see `closure` for why. |
| `projection.coverage_api` | The third coverage number: how much of the program model a mechanism can point at. |
| `projection.coverage_evidence` | How much of what is catalogued was actually run. Never combined with the above. |
| `projection.coverage_surface` | How much of the tool is catalogued. Denominator is independent of the numerator. |
| `projection.dependency_closure` | Which packages a package depends on, transitively. Refuses to traverse unless resolve_present. |
| `projection.discover` | Mechanisms reached through a capability. Inner join by design. |
| `projection.entity_all` | Every canonical row's identity and content digest, across every snapshot. The one relation that is not snapshot-scoped. |
| `projection.extraction_run` | Every extraction run, finished or not. Not filtered by visibility -- it is what explains an absence. |
| `projection.fragment_edge` | Which fragment may follow which. The composition rule, as a join. |
| `projection.identity` | Every canonical row's three identity columns, in one relation. |
| `projection.implements` | A mechanism and the code implementing it, with the evidence for the claim. |
| `projection.interaction` | How one option changes what another does, with its predicate rendered. |
| `projection.interaction_truth` | Every interaction evaluated against the request. min then max, over `truth`. |
| `projection.invariant` | Each invariant and how many rows it inspects, so a vacuous check is visible. |
| `projection.mechanism` | Mechanisms, with lattice ordinals and their labels. |
| `projection.mechanism_ability` | The abilities each mechanism serves, folded to one row. |
| `projection.mechanism_detail` | Everything known about a mechanism, in one row. |
| `projection.negative_space` | Capabilities known not to be reachable, with what to do instead. |
| `projection.newest_snapshot` | The newest snapshot with a visible run. Zero rows or one. |
| `projection.parameter` | Parameters, with the three defaults kept separate. |
| `projection.plan_fragment` | Composable steps, with recall rendered and obligations visible. |
| `projection.re_export_chain` | Where an item can be named from, following re-exports to their origin. |
| `projection.result_contract` | What a command's exit status means, and the trap in reading it. |
| `projection.search_domain` | What each tool can be pointed at, by name. |
| `projection.selected_snapshot` | Which snapshot a caller is asking about. Every base table is filtered through this one view. |
| `projection.validate` | Every invariant, what it inspected, and what it found. |
| `projection.violations` | Every violation, under the name of the invariant it breaks. |
| `projection.violations_dangling_assertion_subject` | An assertion claiming a mechanism subject that names none. |
| `projection.violations_dangling_binding` | A binding naming a capability or mechanism that does not exist. |
| `projection.violations_dangling_surface_entry` | A surface entry naming a mechanism that does not exist. |
| `projection.violations_duplicate_entity_id` | Two rows sharing a declared primary key, which makes the optimiser's rewrites unsound. |
| `projection.violations_mislabelled_kind` | A surface entry whose catalog_kind names a different table from its key's. |
| `projection.violations_orphan_mechanism` | A catalog record no surface entry accounts for, so no coverage number sees it. |
| `projection.violations_stale_mechanism_count` | A denormalised count that disagrees with the rows it counts. |
| `projection.violations_underived_entity_id` | A stored id that is not the digest of its own key and snapshot. |
| `projection.visible_run` | Which runs a caller may see. Every base table is filtered through this one view. |

## Provider wrapper

`CanonicalTable` declares `constraints()` -- the primary key on `entity_id` -- and deliberately does **not** implement `statistics()`. PB02 measured the optimiser acting on the declaration: the `Aggregate` disappears from `SELECT DISTINCT entity_id` and a self-join becomes a `LeftSemi Join`. PB02b measured delta-rs already supplying accurate statistics at the `ExecutionPlan` level, so adding them here would duplicate a working channel.

## Session

One session, built from delta-rs's `create_session()` and never from `SessionContext::new()` -- PB07 measured that a bare DataFusion session cannot execute a Delta write at all. Every delta-rs builder is given `SessionFallbackPolicy::RequireSessionState`, because the default silently discards the caller's session and everything registered on it (PB05).
