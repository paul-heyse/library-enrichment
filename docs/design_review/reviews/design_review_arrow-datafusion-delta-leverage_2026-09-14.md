# Design review — leveraging Arrow and DataFusion in the storage/retrieval slice

Reference: [`DATA_MODEL_DESIGN_CHARTER.md`](../design_principles/DATA_MODEL_DESIGN_CHARTER.md) ·
[`ADDENDUM.md`](../design_principles/ADDENDUM.md) · depth `standard`

> **Two passes, one review.** The first pass answered a Delta Lake adoption question and, in
> doing so, found that DataFusion is being used as a Parquet file reader. On that evidence the
> author closed the Delta Lake question (F6 stands as the record) and redirected the review to
> three specific capability areas: **(1)** hierarchical and programmatic schema, to make the
> evidence model genuinely relational; **(2)** complex Arrow types and metadata, for an
> integrated data fabric; **(3)** advanced planning, for querying and execution. Findings
> **F8–F13** and §§2–11's second-pass rows are that work. Nothing from the first pass was
> retracted.

## 1. Decision and scope

**Proposal:** Assess how the Rust Arrow / DataFusion stack should be used by
`crates/enrichment-store` and its callers. Originally also: whether `deltalake` should be
adopted for evidence snapshot storage — **closed, do not adopt** (F6).

**Status:** Mixed; labelled per claim below. The deltalake question is **Measured** (resolution
and policy gate actually run). The retrieval findings are **Implemented**-strength readings of
the executing path. No gate in `tests/gates.toml` was run — see Method.

**Reviewer:** design-review skill, this session. **Author of record:** operator.

**Affected revisions:** snapshot storage `3.0` (reads `1.0`/`2.0`, ADR-0020); wire envelope
`1.0` (frozen); `datafusion 55.1.0` / `arrow 59.3.0` / `parquet 59.3.0` / `object_store 0.13.2`
as pinned in `Cargo.toml`.

**Observable outcome sought:** that the three questions the retrieval tools ask — *what is in
this crate* (`library_overview`), *where is this capability* (`evidence.search`), *what do we
know about this symbol* (`inspect_symbol`) — are answered by the query engine that is already
compiled in, at a cost proportional to the answer rather than to the crate.

**Baseline:** §B7's mechanism is implemented and works. Three Parquet tables per snapshot,
written under `staging/`, verified by read-back, moved with one `rename`, and pointed at by a
one-rename context pointer (`snapshot.rs:101-149`, `catalog.rs:274-281`). A `SnapshotReader`
registers the three files in a `SessionContext` and answers with `WHERE` filters
(`query.rs:106-136`).

**Supported scope and non-goals.** In scope: `crates/enrichment-store/` in full, the evidence
record types it projects, and the four daemon operations that consume the reader (`overview`,
`search`, `inspect`, `compare`). Out of scope and not inspected: producers, the execution
policy and capsules, the LSP layer, the Python boundary, job state, and the MCP transport.
Not a goal: changing the wire envelope, the epistemic classes, or the publication protocol.

**Constraints.** `deny.toml` sets `[bans] multiple-versions = "deny"`, `wildcards = "deny"` and
`[sources] unknown-git = "deny"`. The workspace manifest states the ordering rule explicitly:
pin DataFusion first, derive arrow/parquet/object_store from it, one Arrow type universe.
`rust-toolchain.toml` pins stable 1.98.1.

### Method and coverage

**Read in full, at the grain cited:** `crates/enrichment-store/src/{lib,tables,snapshot,catalog,query}.rs`;
`crates/enrichment-core/src/search/mod.rs`; `crates/enrichment-daemon/src/ops/{search,overview}.rs`;
the reader-consuming sections of `ops/inspect.rs` and `ops/compare.rs`; `deny.toml`; the root
`Cargo.toml`; `docs/design/DESIGN.md` §6.1–§6.4, §8.2–§8.4 and the §B1–§B13 table.

**Measured this session** (commands run, outputs recorded in §9): dependency resolution and
`cargo deny check bans licenses sources` for `datafusion 55.1 + arrow 59.3 + parquet 59.3 +
object_store 0.13.2`, with and without `deltalake`; the crates.io version and dependency
metadata for `deltalake` / `deltalake-core`; the `delta-io/delta-rs` `main` and `rust-v0.32.4`
manifests; the DataFusion 55.1.0 and parquet 59.3.0 API surfaces read from the vendored sources
in `~/.cargo/registry`.

**Could not verify, and why.** **The working tree does not compile.**
`cargo build -p enrichment-store` fails at
`crates/enrichment-core/src/capsule_protocol/mod.rs:47` — `canonical::digest_hex(self)` expects
`&serde_json::Value` and receives `&Operation`. That directory is untracked (`git status` → `??`)
and is declared at `crates/enrichment-core/src/lib.rs:56`. Consequently **no test and no
acceptance gate was executed against this tree**, and the SessionStart banner's
"41 passed / 0 failed / 6 blocked / 1 not_run" describes a different tree than the one reviewed.
Every claim below about existing tests is therefore "the test exists and covers the case, read at
source" — never "the test passes".

**Guarantees I did not attack.** Content-key determinism (hash the same inputs twice across two
processes — blocked by the build); crash-during-`rename` on a real filesystem; a crafted
malicious Parquet file against `validate_snapshot_table`; concurrent publication of two
different snapshots to one context; cursor forgery beyond reading the checksum construction.
`validate_snapshot_table` is assessed by reading the path it executes, not by feeding it hostile
input.

**Prior work not re-derived.** Register row R-13 (execution-policy profile absent from the
snapshot key) and R-02 (nothing ties an acceptance report to the tree that produced it) are
already recorded; this review cites them rather than rediscovering them.

**Second pass — additional coverage.** Added to the reading: `crates/enrichment-core/src/evidence/model.rs`
(the record types in full), `crates/enrichment-core/src/compare.rs`, `crates/enrichment-daemon/src/ops/compare.rs`,
`crates/enrichment-core/src/wire/data.rs` and `wire/evidence.rs`, `crates/enrichment-store/src/catalog.rs`
re-read for its index structure, and the `justfile` recipe surface. Added to the library reading,
all confirmed at `file:line` in the vendored sources: `datafusion-55.1.0`,
`datafusion-common-55.1.0`, `datafusion-catalog-55.1.0`, `datafusion-session-55.1.0`,
`datafusion-optimizer-55.1.0`, `arrow-schema-59.3.0`, `arrow-array-59.3.0`, `parquet-59.3.0`.
Still not executed: anything requiring a build (F7). The second pass adds **no** measurement —
every performance statement below remains a hypothesis under DM-39, and each names the benchmark
that would settle it.

**The tree moved during the review.** `crates/enrichment-store/src/catalog.rs` changed while the
second pass was running — `write_atomic` gained `create_new(true)`, an `fsync` on the file and an
`fsync` on the parent directory (`catalog.rs:44-57`). That strengthens each *individual* write and
does not affect F9, whose subject is that the *set* of three index writes is not atomic;
`record_release` still chains three `write_json` calls with `?` (`catalog.rs:119-138`, verified
after the change). Citations were re-resolved against the post-change file. Every other cited
line was last verified during this session, but the tree is under active edit and a citation
should be re-checked before it is acted on.

**A note on what the second pass deliberately did not do.** It did not design a schema. The
findings name the Arrow type that matches each value the model already has, and stop there;
choosing between (say) `Map<Utf8,Utf8>` and a typed `Struct` for a locator is a decision for an
ADR with the producer authors, not for a reviewer. Nor did it look for uses of DataFusion's
deeper extension points — custom `OptimizerRule`s, `UserDefinedLogicalNode`, recursive CTEs —
beyond confirming they exist and are unused; §8 explains why recommending them would be the
DM-58 mistake this review otherwise argues against.

## 2. Authority and lifecycle map

Scoped to the storage/retrieval slice. Bold marks a fact no single site owns.

| Concept or fact | Semantic type and identity | Authority / owner | Revision boundary | Permitted update path | Derived representations |
|---|---|---|---|---|---|
| Evidence record shape | `Symbol`, `Relationship`, `EvidenceFragment` | `enrichment-core/src/evidence/` (Rust types) | `schema_version` | edit the Rust type | JSON Schema (generated); Arrow schema (hand-written) |
| Arrow column set | `SchemaRef` per table | `store/tables.rs:44-100` | `schema_version` + the historical-optional table at `tables.rs:379-390` | edit `*_schema()` and both converters together | Parquet file layout |
| Snapshot identity | `SnapshotId`, content-derived from `SnapshotInputs` | `core/identity` (`SnapshotId::derive`) | the snapshot itself | new inputs ⇒ new id | `snapshots/<id>/` directory name; Parquet key-value metadata |
| Which snapshot is current | `ContextId → SnapshotId` | `catalog.rs:274-281`, one file, one rename | per publication | `set_current_snapshot` only | — |
| **"path is within namespace `area`"** | predicate over `path` / `subject` | **none — three independent sites** (F1) | — | edit all three, uncoordinated | DataFusion `Expr`; two Rust closures |
| **"this row matches the query"** | predicate + score | **two — the narrowing predicate and the scorer** (F3) | — | edit both, uncoordinated | `Expr` at `query.rs:235-241`; `score_symbol` at `search/mod.rs:103-183` |
| Physical string layout | Arrow `Utf8` vs `Utf8View` | **`tables.rs`'s concrete downcasts**, imposed on the engine at `query.rs:117-120` (F5) | session-global | edit the `SessionConfig` | every query in the process |
| Parquet physical layout | compression, statistics, row groups, bloom filters | `tables.rs:483-511` (sets compression + metadata only) | per file | edit `write_parquet` | — |
| **Containment ("X is inside Y")** | hierarchy | **three — `Symbol.parent_path` (`model.rs:142`), the `::`/`.` path-prefix convention, and `RelationKind::MemberOf` edges (`normalize.rs:522`)** (F10) | — | edit whichever the caller happens to use | the overview uses `parent_path`; `area` uses prefixes; the edges are stored and never navigated |
| **Keys and referential integrity** | `symbol_id`, `definition_id`, `relationship_id`, `fragment_id`; FKs `relationships.source_id → symbols.symbol_id`, `fragments.subject → symbols.path` | **undeclared anywhere** (F9) — `TableProvider::constraints()` exists and returns `None` for a `register_parquet` table | — | — | — |
| **Snapshot provenance** (producer runs, input digests, coverage, counts) | `SnapshotManifest` (`model.rs`, 35 fields) | `manifest.json` sidecar, `snapshot.rs:151-153` | per snapshot | rewrite the file | **none — it is outside the query surface entirely** (F11) |
| Release → identity, `(ecosystem,name,version)`, `(ecosystem,registry,name,version)` | three JSON indexes under `releases/` | `catalog.rs:119-138`, three sequential `write_json` calls | none — **the set is not atomic** (F9) | `record_release` only | — |

**Deliberately opaque behavior.** The DataFusion physical plan. That is the right thing to leave
opaque — and §7's F5 is precisely that the design does not leave it opaque, while F13 is that
nothing ever asks the engine to *describe* what it chose.

**Identity behavior.** Snapshot identity is *content-derived*, not sequential. Re-publishing the
same identity is a no-op that keeps the first copy (`snapshot.rs:108-113`). This matters for §8:
it is a different identity model from a Delta commit log, not a weaker one.

## 3. Semantic contracts and invariants

| Contract or invariant | Representation | Enforcement boundary | Failure behavior | Verification evidence |
|---|---|---|---|---|
| A reader never sees a partial snapshot | staging dir + single `rename` + pointer rename | `snapshot::publish` | staging swept, nothing under `snapshots/` changed | tests at `snapshot.rs:313` and `snapshot.rs:344` exist and cover both cases — **not executed** (Method) |
| A registered table matches its manifest | column set, nullability, key-value metadata, row counts, and **every row decoded** | `tables.rs:364-476`, called at `query.rs:128` *before* `register_parquet` | `io::ErrorKind::InvalidData`, table not registered | read at source; not attacked with a crafted file |
| Historical snapshots read without mutation | `historical_optional` table at `tables.rs:379-390` + `READABLE_SNAPSHOT_VERSIONS` | `validate_snapshot_table`; `query.rs:109` | `QueryError::UnsupportedFormat` | `store/tests/historical_snapshots.rs` exists |
| A malformed stored record is an error, never an empty result | enum `parse` returning `Option`, `serde_json` on locators and Python observations | `symbols_from_batch` / `fragments_from_batch`, reached from `validate_snapshot_table` | error at open time | `tables.rs:633-661` and `retrieval_fixture.rs:898` exist |
| A cursor binds scope, query, sort and offset | checksummed struct | `Cursor::decode` | `INVALID_CURSOR` | `search/mod.rs:382-414` exists |
| **A scored field is a searched field** | — | **none** | silently absent rows (F3) | **none** |
| **One namespace membership rule** | — | **none** | silent divergence (F1) | **none** |
| **Every `relationships.source_id` names a symbol in this snapshot** | — | **none** — `validate_snapshot_table` validates each table in isolation | a dangling edge is admitted | **none** (F9) |
| **`deprecated_since`/`deprecated_note` are present only when `deprecated` is true** | — | **none** — the flattened encoding admits the contradiction; `Option<Deprecated>` in the model cannot express it | the flat columns are silently discarded on decode (`tables.rs:200-205`) | **none** (F11) |
| **One on-disk spelling for an enum column** | — | **none** | `evidence_class` stores `statically_extracted`; `source_version_match` stores `"exact"` *with JSON quotes* (`tables.rs:309-311` vs `288-296`) | **none** (F11) |
| The three release indexes agree | three JSON files | **none** — `catalog.rs:119-138` chains three writes with `?` | a failure after the first leaves `release(&id)` finding a release that `find_release` reports as absent | **none** (F9) |

**Absence and uncertainty.** The envelope distinguishes `ok` / partial / error, carries
`Coverage { indexed, missing, limitations }`, and `ops/search.rs:255-258` states explicitly that
an empty page is not proof of absence. That is genuinely well done and is a strength (§11).
F3 is a hole *inside* that discipline: the response advertises a `summary_token` factor
(`search/mod.rs:55`, rendered into every envelope at `ops/search.rs:288-293`) over a field
retrieval never consults.

**Equivalence.** The stated requirement is *semantic* round-trip, not byte equality —
`tables.rs:571-583` asserts `back == symbols` after a Parquet round trip. The DataFusion round
trip is claimed but not tested (F2).

## 4. Derivation and execution design

| Stage | Input dependencies | Output contract | Preconditions | Effects and mutable ownership | Provenance / invalidation |
|---|---|---|---|---|---|
| Project to Arrow | typed records | `RecordBatch` matching `*_schema()` | none | pure | — |
| Stage | `RecordBatch` | `<staging>/<table>.parquet` | staging dir exists | writes under `staging/` only | — |
| Verify | staged file | row count equals what was written | file readable | reads | `PublishError::Verify` |
| Publish | staged dir | `snapshots/<id>/` | id not already published | one `rename`, then one pointer `rename` | content-derived `SnapshotId` |
| Open | `snapshot_id` | a `SessionContext` with three tables | manifest present, `schema_version` readable | **registers a session-global `schema_force_view_types=false`** (F5) | — |
| Retrieve | `SessionContext` | `Vec<Symbol>` / `Vec<EvidenceFragment>` | table registered | pure | — |
| Rank, aggregate, fold, page | fully materialized `Vec` | ordered hits | — | pure, **in Rust** (F4) | — |

**Provider selection and limitations.** DataFusion 55.1.0 is the only query provider. The
capabilities it advertises and this code does not use are listed under F4; each was read from
the pinned source, not from recollection.

**Boundary contracts.** The Arrow↔record boundary is crossed **row by row** in a `for i in
0..batch.num_rows()` loop with a `column_by_name` + `downcast_ref` per column per row
(`tables.rs:195-236`, helpers at `tables.rs:161-181`). That is the opposite of DM-37's "coarse, typed units": the batch is the
coarse unit, and it is immediately dissolved.

**Coherent publication.** Correct and well-argued: staged, verified, one rename, pointer swept.
Nothing in this review proposes changing it, and §8 explains why a Delta commit log would not
improve it.

## 5. Representative journeys

### Ordinary extension — add a `stability` column to `symbols`

Add the field to `Symbol`; add a `Field` to `symbols_schema()`; add one arm to
`symbols_to_batch`; add one arm to `symbols_from_batch`; add a `historical_optional` entry for
`schema_version` `3.0`; regenerate the JSON Schema. Six coordinated edits, all in two files, all
mechanically checkable — this is close to charter §E's desired pattern and is a genuine
strength of the current design. Nothing outside `tables.rs` learns a new column name.

### Meaningful change — make `area` case-insensitive

Three edits in three crates that no test relates (F1): the DataFusion `Expr` at `query.rs:579`,
the closure at `query.rs:445`, and the `retain` at `ops/search.rs:151`. Getting two of three is
silent: `library_overview(area="Foo")` would list a namespace whose symbols
`evidence.search(area="Foo")` refuses to return, and the search envelope's `total_matches` would
count rows no cursor can reach. This is the extension-locality failure the charter is about, at
small scale.

### Boundary — a symbol whose only documentation is a synthetic summary

`producer/python/normalize.rs:186-189` emits namespace-contribution modules with
`signature: None`, `doc_summary: Some("Namespace contribution from this distribution only")`,
`docs: None`. `score_symbol` awards `summary_token` (80 points, `search/mod.rs:55`) for a token
in `doc_summary`. The narrowing predicate at `query.rs:235-241` consults `path`, `signature` and
`docs` — never `doc_summary`. In SQL `NULL LIKE '%x%'` is `NULL`, so the row is not returned; it
is scored by a scorer it never reaches. The meaning "this field participates in matching" is
lost between the two sites.

### Alternate representation — comparing two releases (second pass)

This is the journey that most exposes the shape of the problem, so it is worth walking in full.

`compare_releases` opens two snapshots. `SnapshotReader::open` builds **a fresh
`SessionContext` per snapshot** (`query.rs:120`), so the two never share a catalog and **no
query can see both**. Everything relational therefore happens in Rust:

1. `ops/compare.rs:175-183` pulls **every fragment** from both snapshots.
2. `compare.rs:136-170` `fragment_changes` groups each side into a `BTreeMap<(Scope, subject), Vec<_>>`,
   unions the key sets with `a.keys().chain(b.keys())`, and compares the values. That is
   `GROUP BY subject` + `FULL OUTER JOIN` + `IS DISTINCT FROM`, written out.
3. `ops/compare.rs:185-192` pulls **every symbol** from both snapshots.
4. `compare.rs:95-120` `api_changes` does the same again, keyed on `path`, with each group's
   value a canonical-JSON array — so "did this capability change" is decided by *string equality
   over serialized JSON*, a comparison no index, no statistic and no plan can help with.
5. `ops/compare.rs:194-204` then attaches signature fragments to each API delta with
   `before_fragments.iter().filter(|f| f.subject == delta.subject && …)` — **inside a loop over
   deltas**. That is a `LEFT JOIN` executed as a nested loop scan, O(deltas × fragments).
6. `ops/compare.rs:213-222` canonicalises every relationship edge to a JSON string, collects both
   sides into `BTreeSet`s, and compares the sets — `FULL OUTER JOIN` once more.

Four joins and two group-bys, all hand-written, all over fully materialized `Vec`s from two
snapshots, because the two snapshots are in different sessions. With a catalog
(`SessionContext::register_catalog`, `context/mod.rs:1916`) exposing each snapshot as a schema,
step 2–6 is one query per scope, the engine picks the join algorithm, and the comparison
semantics become inspectable instead of being buried in `BTreeMap` key equality. This is F8.

### Interruption — a crash between `rename` and pointer swap

The snapshot directory exists and is complete; the context still points at the previous
snapshot. A reader gets the old snapshot — coherent. A re-publication takes the
`is_published` branch at `snapshot.rs:108-113` and repoints. This path is designed correctly and
the test at `snapshot.rs:344` covers the staging half of it. Delta would not improve this
(§8).

## 6. Acceptance gates

Settled on their own evidence. Not averaged.

| Gate | Verdict | Evidence or scope rationale | Required action |
|---|---|---|---|
| **G1 — Authority** | **fail** | Four semantic facts have no owner. Namespace membership is independently editable at `query.rs:579`, `query.rs:445-451` and `ops/search.rs:151-155` (F1). "What counts as a match" is independently editable at `query.rs:235-241` and `search/mod.rs:103-183` (F3). **Containment** has three representations — `parent_path`, the `::`/`.` prefix convention, and `member_of` edges — and the typed one is never navigated (F10). **Release lookup** has three indexes written non-atomically, so `release(&id)` and `find_release(…)` can disagree about the same release (F9, `catalog.rs:119-138`). No derivation, generation step or equivalence test links any of them. | F1, F3, F9, F10 |
| **G2 — Semantic fidelity** | **fail** | Three independent losses. The `scoring` legend returned in every search envelope (`ops/search.rs:288-293`) advertises `summary_token` over a field the retrieval predicate never reads (F3). The Arrow projection flattens `Option<Deprecated>` into a form admitting a state the model cannot express, and the decode discards it silently (F11-i, `tables.rs:200-205`); two enum columns in the same table use two different on-disk spellings, one carrying literal JSON quotes (F11-v, `tables.rs:288-296` vs `309-311`). The epistemic-class vocabulary — this design's central distinction — is unqualified `Utf8` with no field metadata, so a published snapshot cannot state it (F12). None is covered by a declared loss. | F3, F11, F12 |
| **G3 — Validity** | **pass, scope narrowed** | `validate_snapshot_table` (`tables.rs:364-476`) runs *before* `register_parquet` (`query.rs:128-134`) and rejects unknown columns, missing/retyped columns outside the historical-optional table, nulls in required columns, metadata disagreeing with the manifest, row counts disagreeing with the manifest, and any row that fails typed decoding. Read at source; **not attacked** with a crafted file (Method). **Narrowing:** it validates each table *in isolation*. Cross-table referential integrity (`relationships.source_id → symbols.symbol_id`) and the correlation between flattened fields are unchecked (F9, F11-i). That does not fail the gate, because no current operation assumes those invariants — but the pass covers only what is checked, and that scope should travel with any citation of it. | narrow the claim; F9 |
| **G4 — Hidden behavior** | **pass, with a recorded concern** | No read path fetches, builds or mutates; `SnapshotReader::open` is an open, and publication's effects are declared. The concern is authority, not effect: `query.rs:117-120` sets a process-wide physical-planning flag on behalf of a decoder (F5). That is G1/DM-41 territory, not an undeclared effect. | F5 |
| **G5 — Consistency and recovery** | **pass** | Stage → read-back verify → single `rename` → pointer `rename` → sweep (`snapshot.rs:101-211`); re-publication of an identical identity is idempotent; staging is never read as evidence. Tests covering both the atomic and the swept-crash case exist at `snapshot.rs:313` and `snapshot.rs:344`. **Not executed** (Method) — so the claim is "the mechanism and its tests were read", not "they pass". | re-run once the tree builds |
| **G6 — Transformation and reuse** | **fail** | Two independent reasons. (a) *Pre-existing:* `DESIGN.md:597-599` states plainly that the execution-policy profile is **not** a declared input to the snapshot key — register row R-13. Content-key determinism was **not** attacked (Method), and R-13 is outside this review's correction set. (b) *Second pass:* the Arrow projection is a transformation that loses declared structure — `Deprecated`, `cfg_hints`, `python` and `locator` all arrive flattened or JSON-encoded with no declared loss or approximation policy (F11), and the physical layout that would make the lossy form pay for itself (F4) is absent too. A projection is exactly what this gate covers. | F11; F4; R-13 stands separately |
| **G7 — Truthful capability claims** | **fail** | `DESIGN.md:118-119` labels §B7 *Tested* and names `symbols_round_trip_through_datafusion` as the evidence; the identifier occurs nowhere in the repository except that line and the comment at `query.rs:116` that also cites it (F2). Charter §D requires a named test; the named test does not exist. Separately, the premise that a `deltalake` 1.0 pre-release is compatible with this pin set is refuted for every *obtainable* version (F6). | F2, F6 |

## 7. Principle findings

Ordered by severity: authority and correctness first, then semantic duplication and extension
difficulty, then cost.

| # | Finding | Principle IDs | Concrete evidence or gap | Consequence | Proposed correction | Verification |
|---|---|---|---|---|---|---|
| **F1** | "path is within namespace `area`" is an authoritative semantic fact with three independently mutable definitions and no reconciliation. | DM-02, DM-23, DM-56 · **G1** | `crates/enrichment-store/src/query.rs:579` `namespace_predicate` → `col(c).eq(lit(area)).or(col(c).like(lit(format!("{}::%", escape_like(area)))))…`; `crates/enrichment-store/src/query.rs:445-451` `in_area` → `s.path == area \|\| s.path.starts_with(&format!("{area}::")) \|\| s.path.starts_with(&format!("{area}."))`; `crates/enrichment-daemon/src/ops/search.rs:151-155` the same shape again in `candidates.retain`. The two Rust copies do literal `starts_with`; the `Expr` builds an escaped `LIKE`. No test relates them. | They agree today, so this is drift risk, not a live bug — but the drift is silent and asymmetric. Change one (case-folding, a Python `.`-vs-`::` distinction, a `%` in a module path) and `library_overview(area=X)` lists a namespace whose members `evidence.search(area=X)` refuses to return, while the search envelope's `total_matches` (`ops/search.rs:205`) counts rows no cursor can reach — a page that says "12 of 40" where 28 are unreachable. | One authority in `enrichment-core`: a `Namespace` newtype that emits **both** a `datafusion::prelude::Expr` and a `fn contains(&self, path: &str) -> bool`, with a test asserting they agree over a fixture path set. Then push `area` into the three reader methods that currently lack it and delete the `retain` at `ops/search.rs:151`. Surface: ~3 call sites, one new core module. Prefer `starts_with` (verified present: `datafusion-functions-55.1.0/src/string/starts_with.rs`) over `LIKE` so the `Expr` and the Rust predicate are the *same* primitive and the escaping asymmetry disappears. | **`ast-grep` rule** in `rules/` forbidding `starts_with(&format!("{…}::"))` and `.like(lit(format!("{…}::%")))` outside the one authority, with fixtures in `rule-tests/`; **plus** the agreement test above. Neither exists today. |
| **F2** | `DESIGN.md` §B7 and a load-bearing code comment both cite a test that does not exist, and the setting the comment justifies is therefore unguarded. | DM-59, DM-60, DM-53 · **G7** | `docs/design/DESIGN.md:118-119`: *"Evidence: Tested — … DataFusion reads the published tables back (`symbols_round_trip_through_datafusion`)"*. `crates/enrichment-store/src/query.rs:116`: *"Measured by `symbols_round_trip_through_datafusion`."* `grep -rn "round_trip_through_datafusion"` over the repository returns exactly those two lines. | Two failures. (a) A charter §D `Tested` label on a binding decision is unbacked — §B7's evidence is a name, not a test. (b) The `SessionConfig` at `query.rs:117-120` is load-bearing and nothing protects it: remove it and every string column arrives as `StringViewArray`; `tables.rs:161-165` downcasts to `StringArray`, returns `None`, `req_str` yields `""` for every required string, and `SymbolKind::parse("")` fails — so **every retrieval tool errors at snapshot open**, and no test names the setting. (The comment's own prediction, "would decode as no rows at all", is also wrong; it errors.) | Write the test the two documents already promise: open a published snapshot through `SnapshotReader`, `SELECT *`, and assert `symbols_from_batch` reproduces the input records — then F5 makes the setting unnecessary and the test outlives it. Until it exists, relabel §B7's Arrow/DataFusion clause **Implemented**, not Tested. | **`just` gate.** A check that every test identifier named in a `*Evidence: Tested*` line of `DESIGN.md` appears in `cargo test -- --list`. It would have caught this the day the label was written, and it generalises to every §B row. None exists. |
| **F3** | The narrowing predicate and the scoring function are two authorities for "this row matches", and they already disagree about `doc_summary`. | DM-02, DM-08, DM-42, DM-43 · **G1, G2** | Narrow, `query.rs:235-241`: `lower(path) LIKE p OR lower(signature) LIKE p OR lower(docs) LIKE p`. Score, `search/mod.rs:103-183`: also awards `summary_token` for a token in `doc_summary` (factor table, `search/mod.rs:55`). A row where the summary is the only text exists in production: `crates/enrichment-core/src/producer/python/normalize.rs:186-189` emits `signature: None`, `docs: None`, `doc_summary: Some("Namespace contribution from this distribution only")`. | A search for `namespace`, `contribution` or `distribution` scores those module rows at 85 (`summary_token` 80 + `definition_path` 5) — and never retrieves them, because `NULL LIKE '%x%'` is `NULL`. Worse than the missing rows: **every** envelope publishes a `scoring` legend (`ops/search.rs:288-293`) telling the calling agent that summaries are ranked. They are ranked and not searched. The caller cannot tell "no symbol matches" from "matching symbols were filtered out before scoring" — the exact inference `AGENTS.md` forbids. | Derive one from the other. The factor table already names the fields it scores; make the narrowing predicate a function *of that table* so adding a factor over a new column extends retrieval automatically. Minimum fix, if the table is not made declarative: add `doc_summary` to the predicate. | **Contract test** asserting that for every symbol field the factor table scores there is a corresponding clause in the narrowing predicate — plus an end-to-end case searching for a token present only in `doc_summary` and asserting a hit. **Plus** a `rules/` entry forbidding a new `lower(col(…))`/`like` clause in `query.rs` that is not derived from `search::FACTORS`. Neither exists. |
| **F4** | DataFusion is used as a Parquet file reader. Every aggregation, distinct-count, projection, ordering and limit the retrieval tools need is re-implemented in Rust over fully materialized rows, and the Parquet files are written without the physical structure that would let the engine prune. | DM-36, DM-37, DM-38, DM-26, DM-01 · **G6** | **Usage**: `grep` for `aggregate(`, `.select(`, `.limit(`, `.sort(`, `.join(`, `distinct_on` across `crates/` returns **nothing**. The entire DataFusion surface used is `register_parquet`, `ctx.sql`/`ctx.table`, `filter`, `collect`, `ScalarValue` parameters and `lower`. **Consequences in code**: `query.rs:444` `overview()` opens with `all_symbols()` = `SELECT * FROM symbols`, decoding every row including every `docs` string, then computes `definitions_by_kind` with a `BTreeMap<String, BTreeSet<String>>` (`query.rs:455-460`) and per-namespace `counts_by_kind` by looping over namespaces **and inside that over all symbols** (`query.rs:478-521`) — O(modules × symbols). `ops/inspect.rs:161` calls `all_symbols()` to compute `also_at` for **one** definition — a `WHERE definition_id = $1`. `ops/search.rs:134-205` materializes every `LIKE`-matching symbol and fragment with full `docs`/`text`, ranks all of them in Rust, and returns at most `limits.search_results = 12` within `inline_result_bytes = 12288` (`core/config.rs:99-100`). **Writer**: `tables.rs:483-511` sets compression and key-value metadata and nothing else — `set_bloom_filter_enabled` (`parquet-59.3.0/src/file/properties.rs:1128`), `set_column_bloom_filter_enabled` (1254), `set_sorting_columns` (818) and `set_max_row_group_size` (742) are all unused. **Reader**: `datafusion.execution.parquet.bloom_filter_on_read` defaults **true** (`datafusion-common-55.1.0/src/config.rs:1275`) — the read side already asks for filters the writer never writes — while `pushdown_filters` defaults **false** (1231) and is never enabled, so the `LIKE` predicates run after whole row groups are decoded. `ParquetReadOptions.file_sort_order` (`datafusion-55.1.0/src/datasource/file_format/options.rs:270`) is left default. | Correctness is unaffected; cost is not. `inspect_symbol` on one symbol decodes the whole crate's documentation text. `library_overview` is quadratic in module count. A 12-hit search page carries the decode cost of every `LIKE`-matching row's full docs. **End-to-end cost is unmeasured** (DM-39) — the only fixture is a toy crate and no benchmark exists — so this is a *hypothesis about magnitude* and a *certainty about shape*: the shape is readable from the loop nesting, the magnitude is not. | Push the work into the engine that is already linked, all verified present in 55.1.0: `DataFrame::aggregate` (`dataframe/mod.rs:645`) with `count(DISTINCT definition_id)` replaces both `definitions_by_kind` and the quadratic `counts_by_kind`; `DataFrame::select` (410) gives search a narrow projection so `docs` is never decoded during narrowing; `limit` (723) and `sort` (1241) bound the page in the engine; `distinct_on` (963) folds re-exports; `inspect`'s `also_at` becomes `WHERE definition_id = $1`. Then earn the pruning: enable `pushdown_filters`, write bloom filters on the exact-match columns (`path`, `symbol_id`, `definition_id`, `subject`, `source_id`, `target_id`), sort the `symbols` table by `path` at write time with `set_sorting_columns`, and declare it via `file_sort_order` so prefix predicates can prune row groups. Ranking itself stays ordinary Rust behind a contract — charter §F — because it is a specialized algorithm, not a query. | **`just` gate**: a benchmark recipe over a genuinely large crate (not `enr-fixture`) recording overview, search and inspect latency and bytes decoded, so the claim stops being a hypothesis and any regression is visible. **Plus** an `ast-grep` rule in `rules/` forbidding `all_symbols()` outside the comparison path, which is the one caller that legitimately needs the whole table. Neither exists. |
| **F5** | A hand-written row decoder dictates the query engine's physical plan, process-wide, and forfeits the Arrow layout designed for this workload. | DM-41, DM-36, DM-03 · **G4 (concern), G1** | `query.rs:117-120` sets `datafusion.execution.parquet.schema_force_view_types = false` on the `SessionConfig` because `tables.rs:161-181` downcasts to the concrete `StringArray` / `BooleanArray` / `UInt32Array`. The default is `true` (`datafusion-common-55.1.0/src/config.rs:1246`). The decoder is the reason; the engine's whole session is the scope. | DM-41 asks adapters to be mechanical — to adapt to the representation they are handed. This one inverts that: the projection layer decides the physical layout for every query in the process, including future ones it knows nothing about. It forfeits `StringView` precisely where it pays most — the substring/`LIKE`-heavy filtering of F4 — and it is unguarded (F2), so removing the line is a silent, total retrieval failure rather than a test failure. | Decode over **`arrow::array::StringArrayType`** (`arrow-array-59.3.0/src/array/mod.rs:693`, implemented for `&GenericStringArray<O>` at `:701` and `&StringViewArray` at `:710`) — write the row loop once against that bound and dispatch once at the boundary, which is exactly what arrow itself does in `arrow-cast-59.3.0/src/cast/string.rs:393` and `arrow-string-59.3.0/src/like.rs:298`. Note that `AsArray` alone does **not** unify the two (`as_string` and `as_string_view` return different types, `cast.rs:861`/`:886`) and there is no `downcast_string_array!` macro, so one two-arm `match array.data_type()` remains — one, in one place, instead of a session-wide override. Then delete the `SessionConfig` override and let the engine choose. Surface: the three `*_col` helpers at `tables.rs:161-181` and their call sites. | **`ast-grep` rule** in `rules/` forbidding `downcast_ref::<StringArray>()` in `enrichment-store`, with fixtures — the shape is exactly what a rule catches. **Plus** the F2 round-trip test parameterised over both settings. Neither exists. |
| **F6** | `deltalake` cannot be adopted under this workspace's pins and policy, and the semantics it would add largely duplicate mechanisms this design already owns. The stated premise — a 1.0 pre-release compatible with arrow 59.3 and datafusion 55.1 — does not hold for any obtainable version. | DM-58, DM-57, DM-31, DM-59 · **G7**, §B7, §B11 | **Measured, this session.** crates.io: latest published `deltalake` is **0.32.4** (2026-06-07); `deltalake-core` 0.32.4 declares `arrow ^58`, `parquet ^58`, `datafusion ^53.1.0`. Caret semantics exclude arrow 59.3 and datafusion 55.1. No pre-release version exists on crates.io. `delta-io/delta-rs` has no `rust-v1.0.0*` tag (`python-v1.0.0` is the **Python** package; `rust-v3.0.0` has no `crates/core/Cargo.toml` — HTTP 404). The arrow-59 / datafusion-55 combination is real but lives **only on the unreleased `main` branch**, where `deltalake-core` is version `1.0.0` and `delta_kernel` is `{ package = "buoyant_kernel", git = "https://github.com/buoyant-data/delta-kernel-rs", branch = "buoyant/main" }` — a branch of a third-party fork, with the crates.io alternative commented out one line below. **Resolution measured**: it *does* resolve cleanly — 421 packages, single arrow 59.3.0 / parquet 59.3.0 / datafusion 55.1.0 / object_store 0.13.2, so the compatibility intuition is correct. **Policy measured**: `cargo deny check bans licenses sources` with this repository's `deny.toml` over that graph → `sources FAILED` (6 × `source-not-allowed`: `deltalake`, `deltalake-core`, `deltalake-derive`, `buoyant_kernel`, `buoyant_kernel_derive`, `buoyant_kernel_engine`), `bans FAILED` (8 new duplicates: `alloc-no-stdlib`, `alloc-stdlib`, `brotli-decompressor`, `core-foundation`, `md-5`, `rand`, `rand_core`, **`reqwest`**). Licences pass — `aws-lc-sys` 0.45.0 clears the allowlist, so the workspace comment's licence rationale no longer applies to that version. **Cost measured**: +138 packages over the same graph without deltalake (283 → 421), including a second HTTP/TLS stack (`reqwest` 0.12.28 beside the workspace's deliberately configured 0.13.5, plus `hyper`, `quinn`, `rustls-native-certs`, `rustls-platform-verifier`, `schannel`, `security-framework`), `aws-lc-rs`/`aws-lc-sys` with a `cmake`/C-toolchain build, and `jni` + `jni-sys` — a JVM binding — in a local evidence cache. | Adopting it today means a git-branch dependency on a branch of a fork, six `[sources]` allowances, eight new `[bans] skip` entries including one for the project's own HTTP client, and a C toolchain — against a `deny.toml` whose comments say in terms *"Do NOT widen it"*. The alternative, pinning back to arrow 58 / datafusion 53.1, reverses the workspace's documented "pin DataFusion first" derivation for a dependency nothing currently needs. And the semantic case is weak: Delta's ACID commit log buys multi-writer optimistic concurrency for a design where **only the daemon publishes** (`DESIGN.md:567`, §B2); its time-travel-by-version buys addressing for snapshots already addressed by immutable content-derived directories; its schema evolution duplicates `validate_snapshot_table`'s historical-optional table and ADR-0020; and its file-level data skipping is obtainable directly from the `parquet` crate already in the graph (F4). Its version-linear commit numbering is a **second identity model** beside content-derived `SnapshotId` — adopting both is G1 by construction. | **Do not adopt.** Record the compatibility rows in `docs/architecture/compatibility-matrix.md` with today's date, and open a register row with a concrete revisit trigger: *a published `deltalake` release requiring `arrow >= 59` and `datafusion >= 55` with no git dependency in its graph*. Take the capability that was actually wanted — data skipping and column statistics — from `parquet` 59.3.0 directly, per F4. | **`just` gate — already exists and already fails.** `just deps-policy` / `cargo deny check bans licenses sources` rejects this graph on `sources` and `bans` today; that is the oracle, and it works. Add the register row so the decision has a trigger rather than being re-litigated. |
| **F7** | The reviewed tree does not compile, so no gate result can be reproduced from it — the recurrence of a finding already recorded. | DM-48, DM-60, DM-31 · bounds every label in this review | `cargo build -p enrichment-store` → `error[E0308]` at `crates/enrichment-core/src/capsule_protocol/mod.rs:47`, `canonical::digest_hex(self)`: expected `&serde_json::Value`, found `&Operation`. `git status` reports the directory `??` (untracked); `crates/enrichment-core/src/lib.rs:56` declares `pub mod capsule_protocol;`, so the breakage is workspace-wide. | Every acceptance claim about this tree is unverifiable, including the SessionStart banner's tally. This is the same shape as finding F2 of `design_review_phase-1-slice-and-design-spine_2026-09-13.md` — untracked source under `crates/` making green checks describe a tree git cannot name — and as register row R-02. The correction proposed there has not landed. | Fix or stash the in-progress module before any gate is reported. Independently, land the control the earlier review already specified. | **`just` gate**, already specified and still absent: `scripts/acceptance-check.py` refusing to promote any gate to `passed` when `git status --porcelain` shows untracked or modified paths under `crates/` or `python/`, and recording the commit in `acceptance.json`. A **hook** is the wrong tier — this needs whole-repo state. |

### Second pass — the three capability areas

One sentence before the rows, because it is the single most compact statement of the problem.
Counting occurrences across `crates/` of the capability families this section is about:

```
with_metadata 0   explain( 0        ScalarUDF 0        register_udf 0
SessionStateBuilder 0                OptimizerRule 0    AnalyzerRule 0
CatalogProvider 0  SchemaProvider 0  register_catalog 0 MemTable 0  ViewTable 0
Constraints 0      information_schema 0                 file_sort_order 0
collect_statistics 0                 Dictionary 0
StructArray 0      ListArray 0       MapArray 0
```

Twenty capability families; zero uses of any of them. The three findings below are that fact,
grouped by cause.

| # | Finding | Principle IDs | Concrete evidence or gap | Consequence | Proposed correction | Verification |
|---|---|---|---|---|---|---|
| **F8** | **(area 1)** There is one `SessionContext` per snapshot and no catalog hierarchy, so no query can span two snapshots — which is why the whole of release comparison is a hand-written set of joins. | DM-03, DM-10, DM-38, DM-56 · **G6** | `SnapshotReader::open` builds a fresh context per snapshot: `SessionContext::new_with_config(config)` at `query.rs:120`, then registers three tables under the bare names `symbols`/`relationships`/`fragments` (`query.rs:122-134`). No `CatalogProvider`, `SchemaProvider`, `register_catalog`, `MemTable` or `ViewTable` appears anywhere in `crates/` (0 occurrences each). The consequence is traced in §5's comparison journey: `ops/compare.rs:175-222` plus `compare.rs:95-170` implement, by hand, two `GROUP BY`s, three `FULL OUTER JOIN`s and one `LEFT JOIN` — the last as a nested-loop `filter` inside a loop over deltas (`ops/compare.rs:194-204`). **Available and unused**: `SessionContext::register_catalog` (`datafusion-55.1.0/src/execution/context/mod.rs:1916`), `register_catalog_list` (:2071), `MemoryCatalogProvider` (`datafusion-catalog-55.1.0/src/memory/catalog.rs:68`) with `register_schema` (:96), `MemorySchemaProvider` (`memory/schema.rs:28`) with `register_table` (:63), `ViewTable::new(LogicalPlan, Option<String>)` (`datafusion-catalog-55.1.0/src/view.rs:53`, `impl TableProvider` at :74). | Comparison cost is the sum of both snapshots materialized in full plus an O(deltas × fragments) attachment pass — and, more seriously, the *meaning* of "the same capability in two releases" is encoded as `BTreeMap` key equality over canonical-JSON strings (`compare.rs:104-107`). That is a join predicate no plan can show, no statistic can accelerate and no test can state relationally. Adding a fourth comparison scope means writing a fourth bespoke join. | Register each snapshot as a **schema** in a per-context **catalog**, so `"<context>"."<snapshot>".symbols` is addressable and a comparison is one query per scope. Named `ViewTable`s then carry the derived shapes the operations keep rebuilding — `definitions` (distinct by `definition_id`), `namespace_children`, `api_surface` — as saved logical plans rather than Rust functions. Surface: `SnapshotReader` gains a shared-context constructor; `ops/compare.rs` loses most of its body. | **Differential test**: the new query and the existing `api_changes`/`fragment_changes` must produce identical `Change` sets over the comparison fixtures already in `retrieval_fixture.rs:942-1069` — that is a real equivalence oracle and it can be written before any rewrite. **Plus** an `ast-grep` rule forbidding a new `BTreeMap`-keyed union-diff in `compare.rs`. Neither exists. |
| **F9** | **(area 1)** No keys, uniqueness, or referential integrity are declared anywhere — although the model has four identity columns and three foreign keys, DataFusion models constraints, and its optimizer consumes them. | DM-09, DM-07, DM-06, DM-53 · **G1, G3 (scope-narrowing)** | `TableProvider::constraints() -> Option<&Constraints>` (`datafusion-session-55.1.0/src/table.rs:62`); `Constraint::PrimaryKey(Vec<usize>)` / `Constraint::Unique(Vec<usize>)` (`datafusion-common-55.1.0/src/functional_dependencies.rs:30-36`); consumed by the optimizer in `replace_distinct_aggregate.rs`, `eliminate_join.rs` and `optimize_projections/mod.rs` (`datafusion-optimizer-55.1.0/src/`). `Constraints` occurs 0 times in `crates/`; `ctx.register_parquet` (`query.rs:129`) yields a listing table whose `constraints()` is `None`. The undeclared facts are real: `symbols.symbol_id`, `relationships.relationship_id`, `fragments.fragment_id` are unique by construction; `definition_id → (definition_path, defined_in_crate)` is a functional dependency the overview's distinct-count relies on; `relationships.source_id → symbols.symbol_id` and `fragments.subject → symbols.path` are foreign keys (the second joins on a *display path*, not an id). `validate_snapshot_table` (`tables.rs:364-476`) validates each table **in isolation** and never cross-checks. Separately, `catalog.rs:119-138` maintains three release indexes with three sequential `write_json` calls chained by `?` — each write is atomic, the set is not. | Two distinct costs. (a) *Optimizer*: every `DISTINCT`/group-by the overview would push down has to be computed from scratch because the engine is not told `definition_id` is a key — the very rules that would exploit it are compiled in and receive nothing. (b) *Integrity*: a relationship whose `source_id` names no symbol in the snapshot is admitted by the boundary whose stated job is to reject malformed evidence, and a `record_release` that fails after its first write leaves `release(&id)` succeeding while `find_release(…)` reports `Ok(None)` — the same release both present and absent depending on which index you ask. **No current operation depends on the referential invariant**, which is why this narrows G3's scope rather than failing it; the release-index split is a live G1 defect today. | Declare the constraints on a `TableProvider` wrapper (the same wrapper F8 needs), and add the cross-table checks to the admission boundary that already decodes every row — an anti-join per foreign key, which is cheap because the rows are already in hand. Make `record_release` write the secondary indexes first and the primary record last, so a partial failure is *invisible* rather than *contradictory*. | **`just` gate**: extend `validate_snapshot_table`'s test to include a snapshot with a dangling `source_id` and assert rejection. **Test**: a `record_release` fault-injection case asserting no index combination reports a release as both present and absent. **`ast-grep` rule** forbidding two or more `write_json` calls chained by `?` in `catalog.rs` without a comment naming the ordering rationale. None exists. |
| **F10** | **(area 1)** Containment is modelled three times — a string field, a path-separator convention, and a typed edge — and the typed edge, the only one the model calls authoritative, is never used to navigate. | DM-02, DM-09, DM-34, DM-01 · **G1** | (a) `Symbol.parent_path: Option<String>` (`model.rs:142`), used by the overview (`query.rs:495-499`). (b) The `::`/`.` prefix convention, used by `area` filtering at three sites (F1). (c) `RelationKind::MemberOf` (`model.rs:217`), emitted by the normalizer at `normalize.rs:522`, stored in the `relationships` table, surfaced by `relationships_for` in `inspect_symbol` (`ops/inspect.rs:242`) — and consulted for navigation by **nothing**. `Relationship.source_path` even carries a comment naming the motive: *"The source path, for display without a join"* (`model.rs:264`). | The charter's DM-34 asks that different relationship structures stay semantically distinct; here **one** relationship structure has three representations, two of them string conventions, and the three can disagree about whether a Python module `a.b` contains `a.b.c` when a name contains a separator. The comment at `model.rs:264` is the tell: a denormalization was chosen because a join looked expensive, in a system whose query engine does joins. | Decide which representation is authoritative and derive the rest. The typed edge is the model's own answer; `parent_path` then becomes a materialized convenience with a stated derivation, and the `area` predicate becomes a lookup against edges rather than a string prefix — which also collapses F1, because there is then only one place membership is decided. This is a §B-level decision and wants an ADR, not a patch. | **Consistency test**: for every symbol, `parent_path` must equal the target of its `member_of` edge, and the `::`/`.` prefix predicate must agree with both. It is three assertions over the existing fixtures and it is the oracle whether or not the representation changes. Does not exist. |
| **F11** | **(area 2)** Every structured value in the evidence model is flattened to `Utf8` or to JSON-in-`Utf8` in the Arrow projection. The Rust model is hierarchical and the wire contract is hierarchical; only the storage layer in between is flat, and the flattening both loses queryability and admits states the model cannot express. | DM-01, DM-06, DM-10, DM-42, DM-24 · **G2, G6** | Five instances of one cause, in `tables.rs`: **(i)** `Option<Deprecated{since,note}>` → three columns `deprecated: Boolean`, `deprecated_since: Utf8`, `deprecated_note: Utf8` (`tables.rs:55-57`, written at `120-140`, read at `200-205`). The flat form admits `deprecated=false` with a non-null `deprecated_since`; `Option<Deprecated>` cannot represent that, `validate_snapshot_table` does not reject it (both extra columns are nullable), and the decode silently discards the values. **(ii)** `cfg_hints: Vec<String>` → `utf8("cfg_hints", false)` holding `serde_json::to_string(…).unwrap_or_else(\|_\| "[]".into())` (`tables.rs:143-145`) — a fallible encode with a silent empty-list fallback. **(iii)** `python: Option<PythonSymbol{signature_conflict, observations: Vec<Observation>}>` → one nullable `Utf8` JSON blob (`tables.rs:146-150`); `python.signature_conflict` is not filterable without decoding every row in Rust. **(iv)** `EvidenceFragment.locator: Map<String,Value>` → `utf8("locator", false)` of canonical JSON (`tables.rs:271-274`). **(v)** enum columns are plain `Utf8` **with two different spellings**: `evidence_class` is written via `to_value().as_str()` → `statically_extracted` (`tables.rs:288-296`), while `source_version_match` is written via `serde_json::to_string` → `"exact"` **including the JSON quote characters** (`tables.rs:309-311`, read back with `from_str` at `346-348`). Both enums are `#[serde(rename_all="snake_case")]` unit enums (`wire/evidence.rs:38-43`, `71-78`). And the whole `SnapshotManifest` (35 fields, including `producer_runs: Vec<ProducerRun>` and `inputs: BTreeMap`) is a `manifest.json` sidecar, **not an Arrow table at all** (`snapshot.rs:151-153`). **Available and unused**: `DataType::Struct/List/Map/Dictionary` in `arrow-schema-59.3.0`; `StructArray`, `ListArray`, `MapArray` occur 0 times in `crates/`. | The six epistemic classes are this design's central distinction, and at the storage layer one of them is a quoted JSON literal and another is a bare token — in the same table. Anything wanting to ask "which fragments are `runtime_observed`" from outside this codebase must know which convention applies to which column. Structure that exists on both sides of the projection is destroyed in the middle, so `python.signature_conflict`, a `locator` key or a `cfg_hints` element can never be a predicate at all — today the only way to ask is to decode every row's JSON in Rust. And "which snapshots did normalizer version N produce" cannot be asked without opening every `manifest.json`. **Limit, stated plainly, because it bounds the recommendation:** nested types buy *expressibility and projection*, not *pruning*. Parquet's side is fine — statistics and bloom filters are decided per **leaf**, addressed by a dotted `ColumnPath` (`parquet-59.3.0/src/schema/types.rs:783`, built at `types.rs:1249-1259`), and the writer looks up every per-column property by `descr.path()` (`column/writer/encoder.rs:254`, `:266`, `:490`), so `set_column_bloom_filter_enabled(ColumnPath::from(vec!["python","signature_conflict"]), true)` genuinely targets a nested leaf. **DataFusion is the side that cannot use it**: `datafusion-pruning-55.1.0/src/pruning_predicate.rs:1230` says in terms *"PruningPredicate does not support pruning on nested fields yet."* Nested decoder pushdown exists only for null checks and `array_has*` (`datafusion-datasource-parquet-55.1.0/src/supported_predicates.rs:85`), and both that module and `nested_schema_pruning` are **private** (`mod` at `src/mod.rs:45` and `:33`; absent from the `pub use` block at `:51-68`). So the statistics would be written and ignored. F4's pruning argument applies to the **top-level** columns and not to these: the case for nested types here is meaning and queryability, and it must be made on that basis rather than on a performance claim it cannot support. One more constraint for the ADR: `DataType::Union` **panics** on write (`parquet-59.3.0/src/arrow/schema/mod.rs:855`, `unimplemented!("See ARROW-8817.")`) and an empty `Struct` is an error (`:803`) — neither is needed here, but neither should be reached for. | Use the Arrow types the values already have: `Struct` for `deprecated` and `python`, `List<Utf8>` for `cfg_hints`, `Map<Utf8,Utf8>` (or a typed `Struct`) for `locator`, a `Dictionary` for the six closed enum columns — which makes closed-domain-ness structural rather than a `parse()` returning `Option`, and is the right physical layout for low-cardinality group-by columns besides. (Two practical constraints for the ADR, both verified: a dictionary column round-trips as a dictionary only while a read does not span column chunks and every page is dictionary-encoded — `parquet-59.3.0/src/arrow/array_reader/byte_array_dictionary.rs:68-76` states both conditions and the mitigation; and `arrow-array-59.3.0/src/builder/mod.rs:589-625` restricts `make_builder`'s dictionary arm to `Int8`/`Int16`/`Int32`/`Int64` keys, so a `UInt8`-keyed dictionary panics there and must be built with `StringDictionaryBuilder` directly.) Reading them back is `get_field` / `get_field_path` (`datafusion-functions-55.1.0/src/core/mod.rs:162`, `:167`) and `DataFrame::unnest_columns` (`datafusion-55.1.0/src/dataframe/mod.rs:536`), which handles `List` and `Struct` alike. Promote the manifest's `producer_runs` and `inputs` to a fourth and fifth table so provenance is queryable on the same surface as the evidence. **Before any of this**, settle one enum-to-column convention, because that is a one-line divergence with no defence. Schema version `4.0`, ADR-0020's historical-read discipline applies. | **Round-trip test per nested column** in the style `tables.rs:571-583` already uses. **`ast-grep` rule** in `rules/` forbidding `serde_json::to_string` / `to_canonical_string` as the producer of a `StringArray` value in `enrichment-store` — that is the exact code shape, and it would have caught all four JSON-in-`Utf8` columns. **`ast-grep` rule** forbidding `unwrap_or_else` on an encode path in the projection. **Test** asserting one spelling for every enum column. None exists. |
| **F12** | **(area 2)** No Arrow field- or schema-level metadata is attached, so the semantics that qualify each column live only in file-level key-value pairs and in the manifest — never on the column itself. | DM-04, DM-06, DM-46, DM-55 · **G2** | `write_parquet` sets four file-level key-value entries (`schema_version`, `normalizer_version`, `snapshot_id`, `context_id` — `snapshot.rs:157-162`, `tables.rs:492-501`). `with_metadata` occurs **0 times** in `crates/`. **Available and unused**: `Field::with_metadata` (`arrow-schema-59.3.0/src/field.rs:376`), `Schema::with_metadata` (`schema.rs:244`), `Field::extension_type_name` (`field.rs:474`), `try_extension_type` (:575), `try_with_extension_type` (:603), the `ExtensionType` trait (`extension/mod.rs:187`), `EXTENSION_TYPE_NAME_KEY = "ARROW:extension:name"` (:29) and `EXTENSION_TYPE_METADATA_KEY` (:33), and the canonical extension types shipped in `extension/canonical/` — `json`, `uuid`, `bool8`, `opaque`, `fixed_shape_tensor`, `variable_shape_tensor`, `timestamp_with_offset`. **The mechanism round-trips**: `parquet-59.3.0/src/arrow/schema/mod.rs` serialises the whole Arrow schema, per-field metadata included, into the `ARROW:schema` key — `encode_arrow_schema` (:296), `add_encoded_arrow_schema_to_metadata` (:329), `get_arrow_schema_from_metadata` (:260). | A published snapshot is meant to be portable evidence — §B7 calls it "portable Arrow/Parquet/JSON for the future store", and ADR-0018 exports contexts as offline-verifiable bundles. Today the exported Parquet says a column is named `evidence_class` and holds text; it does not say that the text ranges over six closed classes that are categories and not confidences, nor which producer wrote which column, nor that `locator` is JSON. Every one of those facts is recoverable only by reading this repository. That is the opposite of the discoverability the service exists to provide (DM-55), and it is what makes today's bundles *not* tomorrow's graph evidence without a translation layer — blueprint §15 question 10, which routes to G1. | Attach the semantics where they belong: the epistemic-class vocabulary, the producer and normalizer identity, and the schema version as `Field` metadata on the columns they qualify; and a small project-defined `ExtensionType` for the class column if the vocabulary is worth naming formally. That much is available today: `EXTENSION_TYPE_NAME_KEY`, the `ExtensionType` trait and every `Field::*_extension_type*` method except `try_canonical_extension_type` are **ungated**. **The canonical types are not**: `mod canonical` sits behind `#[cfg(feature = "canonical_extension_types")]` (`arrow-schema-59.3.0/src/extension/mod.rs:20-23`), the feature is declared at `arrow/Cargo.toml:45` and `arrow-schema/Cargo.toml:43` but is absent from `arrow`'s `default` (`:48`), and this workspace pins `arrow = "59.3.0"` with no feature list (`Cargo.toml:33-35`). So using the canonical `arrow.json` tag on `locator` means enabling `arrow/canonical_extension_types` and `parquet/arrow_canonical_extension_types` (`parquet/Cargo.toml:49`) — a dependency-feature change that goes through `just deps-policy`, not a free win. Either way this is additive, costs no rows, and survives the Parquet round trip by the mechanism cited above. | **Round-trip test** asserting field metadata survives `write_parquet` → `read_parquet` → DataFusion scan. **`just` gate**: a bundle-inspection check asserting every exported snapshot's Arrow schema carries the class vocabulary, so an export cannot silently lose it. Neither exists. |
| **F13** | **(area 3)** The query plan is never inspected and never extended: no explain, no session-state hooks, no UDFs, no declared ordering — in a service whose product is explainable evidence. | DM-27, DM-04, DM-21, DM-49, DM-55, DM-26 · **G4 (concern)** | Zero occurrences in `crates/` of `explain(`, `SessionStateBuilder`, `AnalyzerRule`, `OptimizerRule`, `ScalarUDF`, `register_udf`, `information_schema`, `file_sort_order`. The reader constructs a bare `SessionContext::new_with_config` (`query.rs:120`) and never touches session state again. **Available and unused**: `DataFrame::explain(verbose, analyze)` (`datafusion-55.1.0/src/dataframe/mod.rs:1761`) and `explain_with_options` (:1795); `SessionStateBuilder` with ~35 `with_*` hooks (`datafusion-55.1.0/src/execution/session_state.rs:1195-1548`), of which `with_scalar_functions` (:1393) is the one that matters here; `SessionContext::register_udf` (`context/mod.rs:1634`); `datafusion.catalog.information_schema`, default **false** (`datafusion-common-55.1.0/src/config.rs:240`). | Two consequences, one squarely on the product. (a) *Explainability*: the search envelope already carries `scoring: Vec<ScoreFactor>` so a caller can see **why a hit ranked** (`ops/search.rs:288-293`). There is no counterpart for **why these rows were considered** — no predicate, no plan, no statement of what was scanned. That is the same explainability the design already committed to, one level down, and DataFusion hands it over for the cost of one call. It also strengthens the rule that an empty result is not proof of absence: a plan showing the predicate that actually ran is evidence; a prose limitation string is an assertion. (b) *Duplication*: a `ScalarUDF` is the mechanism that would make F1's namespace predicate and F3's match predicate single-sourced **by construction** — one Rust function, registered once, invoked identically from an `Expr` and from SQL. The two most severe findings in this review have the same unused fix. | Register two `ScalarUDF`s — `in_namespace(path, area)` and `matches_query(…)` — as the single authority for F1 and F3; that is a smaller change than either finding's standalone correction. Capture `DataFrame::explain` behind a debug flag and surface it in `Coverage` on request. Enable `information_schema` so the evidence schema is discoverable through the same surface as the evidence. **Do not** write custom optimizer, analyzer or physical rules; see §8. | **Contract test** that the UDF and the Rust predicate agree (this is F1's and F3's oracle, satisfied once). **Test** asserting the explain output names the `symbols` scan and the pushed predicate, so a change that silently stops pushing the filter is visible. Neither exists. |
| — | **Two claims corrected during the second pass**, recorded because the review would otherwise overstate the case. **(a)** *Statistics are already collected.* `datafusion.execution.collect_statistics` defaults to **`true`** (`datafusion-common-55.1.0/src/config.rs:847`), so the listing tables the reader registers do gather per-file statistics; the earlier framing of "no statistics" was wrong. What is missing is not collection but anything worth pruning on — unsorted files and no bloom filters (F4). **(b)** *Declaring `TableProvider::statistics()` would not help.* Its own doc says so: *"Although not presently used in mainline DataFusion, this allows implementation specific behavior for downstream repositories, in conjunction with specialized optimizer rules"* (`datafusion-session-55.1.0/src/table.rs:313-316`). Since §8 rejects custom optimizer rules, `statistics()` is dropped from the recommendation; only `constraints()` (F9) earns the `TableProvider` wrapper. | DM-59 | — | — | — | — |

**Applicability.** Groups **1** (authority), **2** (semantic types — F9, F11, F12), **3**
(identity/consistency), **4** (declarative composition — F8's views, F13's UDFs), **8**
(execution representations), **9** (boundaries and providers) and **11–12** (verification,
proportionality) carry this review. Group **6** (planning and mutable state) applies to
publication and came out clean (G5), and bears on F13 through DM-26/DM-27. Group **10**
(provenance) entered on the second pass through F12 and the manifest half of F11. Groups **5**
(compilation/lowering) and **7** (incrementality/concurrency) were **not applied**: producers,
normalizers and the job system are outside the stated scope, and their absence here is a scope
boundary, not a clean bill.

**Maturity assessment:** omitted. Gates G1, G2, G6 and G7 fail; a dimension score would only
create something to offset them with.

## 8. Alternatives and architectural leverage

| Alternative | Semantic duplication and extension locality | Correctness and operational risks | Cost | Performance evidence | Why selected or rejected |
|---|---|---|---|---|---|
| **Current baseline** — hand-rolled publication; DataFusion as a file reader; Rust as the query engine | Three namespace authorities (F1), two match authorities (F3), one decoder dictating physical plans (F5). Adding a facet means new Rust loops, not a new expression. | Publication is sound (G5). Retrieval is correct but its cost scales with the crate, not the answer (F4). | Already paid. | None — no benchmark exists on a real crate. | The baseline to improve, not to keep unchanged. |
| **Proposed — adopt `deltalake` for snapshot storage** | Would add a **second** identity model (version-linear commit log) beside content-derived `SnapshotId` — G1 by construction — and a second schema-evolution mechanism beside `validate_snapshot_table`'s historical-optional table. | Git-branch dependency on a branch of a fork; 6 `[sources]` violations; 8 new `[bans]` duplicates including a second `reqwest`; C toolchain via `aws-lc-sys`; a JVM binding. | **+138 packages** (283 → 421), measured. | None claimed, none found. What was wanted (data skipping) is available from `parquet` 59.3.0 already in the graph. | **Rejected.** Not obtainable under the pins, refused by the repo's own policy gate, and semantically duplicative of mechanisms §B7 already owns. See F6. |
| **Simpler viable alternative — change no dependencies; use the engine that is already linked** | Removes three of four authority defects at once: a `ScalarUDF` single-sources F1 and F3 by construction, a catalog hierarchy makes F8's four hand-written joins one query per scope, `Constraints` state the keys F9 leaves implicit. Nested Arrow types (F11) delete four JSON-encode/decode paths and their bespoke error handling; field metadata (F12) is additive. Deletes the Rust aggregation, dedup, sort and paging loops in favour of `aggregate`/`distinct_on`/`sort`/`limit`; deletes the `SessionConfig` override. | Lower risk than either alternative: no new crate, no new identity model, no policy exception, publication protocol untouched. The one real risk is a storage-schema revision (`4.0`), which ADR-0020's historical-read discipline already covers. | Confined to `enrichment-store`, `core/compare.rs`, and three call sites. Phased: UDFs and constraints need no schema change; nested types and metadata do. | Currently a hypothesis; the `just` benchmark under F4 is what turns it into evidence. The differential test under F8 is the correctness oracle and can be written first. | **Recommended.** The only one of the four that reduces the number of independently maintained semantic decisions, and it delivers the data skipping the Delta proposal was reaching for. |
| **Constructed counter-proposal — a "data fabric": custom `OptimizerRule`s, a `UserDefinedLogicalNode` for evidence traversal, recursive CTEs over the relationship graph, a generated schema registry** | Would look like more leverage and deliver less: a custom logical node needs its own planner, its own serialization and its own conformance obligation (DM-44) before it does anything the three tables cannot. | Every rule is a place the engine can be made to disagree with itself, in a system that currently has no query it cannot express. | High, and permanent. | None — there is no workload that needs it. | **Rejected**, and named here so it is rejected on the record rather than drifted into. §B11 already excludes a graph database; a graph *engine* assembled out of DataFusion extension points is the same decision by another route (DM-58). Everything F8–F13 recommends is a **built-in** with a one-call or one-declaration surface. The line is: use what the engine already does; do not teach it new things. |

**Abstractions justified by current needs.** Three, each with a named consumer that exists today
and each a single declaration rather than a layer:

1. **A `TableProvider` wrapper** over the registered Parquet tables. It is the one place
   `constraints()` (F9) can be declared, and the thing a catalog (F8) registers. One type, one
   trait method beyond delegation. (Not `statistics()` — see F13's correction note: it is inert
   without a custom optimizer rule, and §8 rejects those. `MemTable` offers the same two
   declarations directly — `with_constraints` at `datafusion-catalog-55.1.0/src/memory/table.rs:112`,
   `with_sort_order` at `:136` — if a wrapper turns out to be more than a delegation shim.)
2. **Two `ScalarUDF`s** — `in_namespace`, `matches_query`. Justified because they are the
   *cheapest* correction available for F1 and F3, not because UDFs are desirable in general.
3. **A snapshot catalog** — a `MemoryCatalogProvider` per context, a `MemorySchemaProvider` per
   snapshot. Its consumer is `compare_releases`, which exists and is four hand-written joins.

Not justified, and explicitly not recommended: a `Namespace` policy type richer than a newtype,
a predicate registry, a schema-generation layer for Arrow (the schema registry in `tables.rs` is
already single-file and readable), custom optimizer rules, or any logical-node extension.

**What remains ordinary code.** The ranking algorithm (`search/mod.rs`) — a specialized,
explainable, well-tested scorer behind a complete contract, exactly charter §F's "ordinary
well-tested code behind that contract". Do not turn it into query-engine expressions. The
narrowing predicate that *feeds* it is a query and belongs in the engine; the scoring is not.
Likewise the publication protocol: staged, verified, renamed — do not replace a correct 60-line
mechanism with a transaction log. And the `Change` vocabulary in `core/compare.rs` stays: F8
replaces how the two sides are *matched*, not what a change *means*.

## 9. Verification and measurement plan

| Claim or risk | Evidence label | Test / analysis / benchmark | Conditions and expected result | Current result or gap |
|---|---|---|---|---|
| `deltalake` is not adoptable under these pins | **Measured** | `cargo generate-lockfile` + `cargo deny check bans licenses sources` with this repo's `deny.toml`, over `datafusion 55.1.0 + arrow 59.3.0 + parquet 59.3.0 + object_store 0.13.2 + deltalake{git=delta-io/delta-rs, branch=main, features=["datafusion"]}`, 2026-09-14 | resolve, then fail policy | Resolved: 421 packages, one arrow/parquet/datafusion/object_store each. `sources FAILED` (6), `bans FAILED` (8), licences pass. Baseline without deltalake: 283 packages. |
| Delta's API on the only compatible version | **Interface-checked** | Read from the resolved git checkout `delta-rs@1181cf7`, `crates/core/` | the items exist | `open_table(Url)` lib.rs:132; `open_table_with_version` lib.rs:156; `DeltaTable::load_version` table/mod.rs:233; `DeltaTable::table_provider() -> TableProviderBuilder` delta_datafusion/table_provider.rs:471; `CommitProperties::with_metadata` kernel/transaction/mod.rs:572; `WriteBuilder::with_schema_mode` operations/write/mod.rs:228 |
| DataFusion/Parquet capabilities named in F4 exist in the pinned versions | **Interface-checked** | Read from `~/.cargo/registry` at `datafusion-55.1.0`, `datafusion-common-55.1.0`, `parquet-59.3.0` | the items exist with the stated defaults | All confirmed at the line numbers cited in F4 and F5. |
| DataFusion round-trips the published tables | **Proposed** | the test `DESIGN.md:119` already names | open a snapshot, `SELECT *`, assert `symbols_from_batch` reproduces the records; parameterised over `schema_force_view_types` | **Does not exist** (F2). §B7's `Tested` label is unbacked. |
| One namespace membership rule | **Proposed** | agreement test over a fixture path set, plus an `ast-grep` rule | the `Expr` and the Rust predicate classify identically | **Does not exist** (F1). |
| A scored field is a searched field | **Proposed** | contract test over `search::FACTORS` vs the narrowing predicate; end-to-end search for a `doc_summary`-only token | the summary-only row is returned and scored | **Does not exist**; fails today (F3). |
| Publication is atomic and crash-safe | Tests exist; **not executed** | `snapshot.rs:313` and `snapshot.rs:344` | atomic publish + swept staging | Read at source. Blocked by F7. |
| An invalid table cannot be registered | **Implemented**; not attacked | `tables::validate_snapshot_table` | rejects before `register_parquet` | Read at source. A crafted-Parquet negative test is the gap. |
| Content-key determinism (no wall-clock, no temp paths) | **Proposed** | hash the same inputs twice in two processes | identical `SnapshotId` | Not attempted — F7. §6.3 forbids the inputs; nothing proves their absence. |
| End-to-end retrieval cost | **Proposed** (explicitly a hypothesis, DM-39) | a `just` benchmark on a large real crate, not `enr-fixture` | overview/search/inspect latency and bytes decoded, before and after F4 | No benchmark exists. The loop shape in `query.rs:478-521` is certain; the magnitude is not. |
| **Second pass** — the capability families named in F8–F13 exist in the pinned versions | **Interface-checked** | read from the vendored sources | each item resolves at the cited line | All confirmed: `register_catalog` `context/mod.rs:1916`; `MemoryCatalogProvider` `memory/catalog.rs:68`; `MemorySchemaProvider` `memory/schema.rs:28`; `ViewTable` `view.rs:36,53,74`; `TableProvider` `datafusion-session/table.rs:52` with `constraints` :62, `supports_filters_pushdown` :303, `statistics` :317; `Constraint`/`Constraints` `functional_dependencies.rs:30,40`; `DataFrame::explain` `dataframe/mod.rs:1761`; `information_schema` default false `config.rs:240`; `Field::with_metadata` `field.rs:376`, `try_with_extension_type` :603; `ExtensionType` `extension/mod.rs:187`; `EXTENSION_TYPE_NAME_KEY` :29; canonical types `json`, `uuid`, `bool8`, `opaque`, `fixed_shape_tensor`, `variable_shape_tensor`, `timestamp_with_offset`; `AsArray` `arrow-array/cast.rs:835`. |
| Arrow field metadata survives the Parquet round trip (F12's premise) | **Interface-checked** | `parquet-59.3.0/src/arrow/schema/mod.rs` | the full Arrow schema is encoded into `ARROW:schema` and restored | `encode_arrow_schema` :296, `add_encoded_arrow_schema_to_metadata` :329, `get_arrow_schema_from_metadata` :260. Bloom filters can target nested leaves via multi-segment `ColumnPath::from(vec![…])` (`parquet/schema/types.rs:753`); sort order reads back via `RowGroupMetaData::sorting_columns()` (`file/metadata/mod.rs:674`). **Not** demonstrated end to end in this repository — that is the test under F12. |
| Twenty capability families are unused | **Tested** (a command ran; output in §7) | `grep -rn <name> crates/ --include=*.rs \| wc -l` for each | some non-zero | Every one returned **0**. This is the second pass's single load-bearing measurement and it is cheap to re-run. |
| A rewritten comparison agrees with the current one | **Proposed** | differential test against `api_changes`/`fragment_changes` over the fixtures at `retrieval_fixture.rs:942-1069` | identical `Change` sets | Does not exist. **Write this before touching `compare.rs`** — it is the only thing that makes F8 a safe change. |
| Referential integrity of a published snapshot | **Proposed** | extend `validate_snapshot_table` with an anti-join per foreign key; negative fixture with a dangling `source_id` | rejected at open | Does not exist (F9). |
| Nested columns round-trip semantically | **Proposed** | per-column round-trip tests in the style of `tables.rs:571-583` | records reproduce exactly | Does not exist (F11). |
| One enum spelling per column | **Proposed** | assertion over the written batch | every enum column holds a bare token, never a quoted JSON literal | Fails today: `source_version_match` stores `"exact"` with quotes (F11-v). |

**Cost accounting.** The material category is *decode*: how many Parquet bytes become Arrow
arrays become Rust `String`s per request. Today it is bounded by the crate; under F4 it would be
bounded by the answer. Storage, publication and recovery costs are unaffected by every
recommendation here.

## 10. Exceptions and unresolved decisions

No new SHOULD-level exception is proposed. Two standing items are cited rather than reopened:

- **R-13** — execution-policy profile absent from the snapshot key (`DESIGN.md:597-599`). Owner
  and trigger already recorded; this review adds nothing to it beyond noting it is why G6 fails.
- **R-02** — nothing structurally ties an acceptance report to the tree that produced it. F7 is
  that row firing again, with a concrete instance.

## 11. Decision and implementation changes

**Decision: Revise.**

**Reason.** Four MUST-level authority gaps on behavior the design claims to support (DM-02:
F1, F3, F9, F10) and one unbacked `Tested` label on a binding decision (DM-59, F2) fail G1, G2,
G6 and G7. The Delta Lake question resolves cleanly and negatively and is closed (F6).

The second pass changes the *shape* of the conclusion, not its direction. The first pass said
DataFusion was being used as a file reader. The second says something more specific and more
actionable: **the defects this review opened with are the same defects the unused capabilities
would close.** F1 and F3 are two predicates with no single authority — a `ScalarUDF` is what
single-sources a predicate. F8's four hand-written joins exist because there is no catalog. F9's
missing keys are a `Constraints` declaration the optimizer already knows how to consume. F11's
four JSON-in-`Utf8` columns exist because `Struct`, `List` and `Map` were not used. That is why
the correction list below is shorter than the finding list: several findings share a fix.

Twenty capability families, zero uses, is not in itself a defect — under-use is only a problem
where it *costs* something. The cost here is concrete and stated per finding: lost authority
(F1, F3, F8, F9, F10), lost meaning at the storage boundary (F11, F12), and lost explainability
in a service whose product is explainable evidence (F13).

The publication protocol, the admission-boundary validation, the absence-vocabulary discipline
in the envelope, the explained ranking factors, and the single-file locality of the Arrow schema
registry are real strengths, and they are what would break if the recommendations below were
taken too far. **Do not** let "use the engine more" become a reason to move ranking, publication
or validation into it, and do not let "make it relational" become the graph engine §B11
excludes. §8's fourth row is the record of that line.

| Priority | Change | Principle IDs | Acceptance evidence | Regression protection |
|---|---|---|---|---|
| 1 — correctness/authority | One `Namespace` authority in `enrichment-core` emitting both an `Expr` (via `starts_with`) and a Rust predicate; push `area` into the reader; delete the two copies (F1) | DM-02, DM-23 · G1 | agreement test over a fixture path set; `overview(area)` and `search(area)` return consistent membership | new `ast-grep` rule in `rules/` + fixtures in `rule-tests/` |
| 2 — correctness/authority | Derive the search narrowing predicate from `search::FACTORS`, or at minimum add `doc_summary` (F3) | DM-02, DM-08, DM-43 · G1, G2 | end-to-end search for a token present only in `doc_summary` returns the row | contract test asserting field-set equality; `rules/` entry on new `like` clauses in `query.rs` |
| 3 — truthfulness | Write `symbols_round_trip_through_datafusion`; until it exists, relabel `DESIGN.md` §B7's clause **Implemented** (F2) | DM-59, DM-60 · G7 | the named test runs | `just` gate checking every `*Evidence: Tested*` identifier in `DESIGN.md` against `cargo test -- --list` |
| 4 — unblock verification | Fix or stash `crates/enrichment-core/src/capsule_protocol/`; land the untracked-source check the 2026-09-13 review specified (F7) | DM-48, DM-60 | `cargo build` succeeds; `acceptance.json` records a commit | `scripts/acceptance-check.py` refusing promotion on a dirty tree (R-02) |
| 5 — semantic leverage | Decode over `AsArray`/`StringArrayType`; delete the `schema_force_view_types` override (F5) | DM-41, DM-36 · G4 | the round-trip test from change 3 passes under both settings | `rules/` entry forbidding `downcast_ref::<StringArray>()` in `enrichment-store` |
| 6 — measured performance | Push projection, aggregation, distinct, sort and limit into DataFusion; write bloom filters and a sorted `symbols` table; enable `pushdown_filters`; declare `file_sort_order` (F4) | DM-36, DM-37, DM-38, DM-26, DM-39 · G6 | a `just` benchmark on a large real crate, before and after, with conditions recorded | the benchmark recipe itself; `rules/` entry confining `all_symbols()` to the comparison path |
| 7 — record the decision | Compatibility-matrix rows for `deltalake` 0.32.4 and `delta-rs@main`, dated 2026-09-14; a register row with the revisit trigger (F6) | DM-31, DM-58, DM-59 | rows present with URLs and retrieval date | `just deps-policy` already rejects the graph — the oracle exists and works |

Second pass. Sequenced so each step is independently landable and the risky ones come last.
Note that **1′ subsumes priorities 1 and 2 above** — it is the same correction by a cheaper
route, so do those as UDFs rather than twice.

| Priority | Change | Principle IDs | Acceptance evidence | Regression protection |
|---|---|---|---|---|
| **1′ — authority, replaces 1 and 2** | Register `in_namespace(path, area)` and `matches_query(…)` as `ScalarUDF`s and call them from both the `Expr` builders and the Rust paths (F13, closing F1 and F3 structurally) | DM-02, DM-19, DM-23 · G1, G2 | one agreement test per UDF; `overview(area)` and `search(area)` agree; a `doc_summary`-only token returns a hit | `ast-grep` rule forbidding an inline `starts_with(&format!(…))` or a new `lower()/like` clause outside the UDFs |
| **8 — authority** | Declare `Constraints` and cross-table referential checks on a `TableProvider` wrapper; make `record_release` write secondary indexes before the primary record (F9) | DM-09, DM-07, DM-53 · G1, G3 | a dangling `source_id` is rejected at open; no index combination reports a release both present and absent | negative fixture; fault-injection test; `ast-grep` rule on chained `write_json` in `catalog.rs` |
| **9 — semantic leverage** | Per-context catalog with one schema per snapshot; rewrite `compare_releases` as one query per scope (F8) | DM-03, DM-10, DM-38, DM-56 · G6 | the differential test against today's `api_changes`/`fragment_changes` passes over the existing comparison fixtures | that differential test, kept permanently; `ast-grep` rule against a new `BTreeMap` union-diff in `compare.rs` |
| **10 — meaning at the boundary** | Storage schema `4.0`: `Struct` for `deprecated`/`python`, `List<Utf8>` for `cfg_hints`, `Map` or `Struct` for `locator`, `Dictionary<UInt8,Utf8>` for the six enum columns; **first**, one enum spelling (F11) | DM-01, DM-06, DM-10, DM-42 · G2, G6 | per-column semantic round-trip tests; historical `1.0`/`2.0`/`3.0` still read under ADR-0020 | `ast-grep` rule forbidding `serde_json::to_string` as the source of a `StringArray` value in `enrichment-store`; rule forbidding `unwrap_or_else` on an encode path |
| **11 — portable meaning** | `Field`/`Schema` metadata carrying the epistemic-class vocabulary, producer and normalizer identity; canonical `arrow.json` extension type on any column that stays JSON (F12) | DM-04, DM-06, DM-46, DM-55 · G2 | metadata survives write → read → DataFusion scan; an exported bundle carries the vocabulary | round-trip test; a bundle-inspection check in `just export`'s gate |
| **12 — containment** | ADR choosing one authoritative representation of containment and deriving the other two (F10) | DM-02, DM-09, DM-34 · G1 | `parent_path`, the prefix predicate and `member_of` edges agree over the fixtures | the three-way consistency test, written now regardless of which representation wins |
| **13 — inspectability** | `DataFrame::explain` behind a debug flag, surfaced in `Coverage` on request; enable `information_schema` (F13) | DM-27, DM-49, DM-55 | the explain output names the scan and the pushed predicate | a test asserting the predicate is still pushed — it catches a silent regression of priority 6 |

**Final check.** The scope reviewed is the storage/retrieval/evidence slice; findings outside it
were not sought and its silences are bounded by the Method note. The claims above are labelled at
the strength of the evidence actually gathered — `Measured` where a command ran and its output is
recorded, `Interface-checked` where a library surface was read at a pinned version,
`Proposed` for every oracle that does not yet exist. Six of the seven findings name an oracle
that is absent today; that absence, not the defects themselves, is the most durable output here.
