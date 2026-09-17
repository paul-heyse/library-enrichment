# Detailed plan — machine-queryable code-search capability catalog

> **Frozen provenance — read, never edit.** This is the record of what was decided about each
> library *before* contact with the integrated design. Its value is that it has not been revised to
> agree with what was built. Corrections belong in [`PLAN-V2.md`](PLAN-V2.md); where the two
> disagree, v2 wins. v2 §0.1 carries the ledger of which theses below are still binding, which were
> superseded and by what evidence, and which were restored after being lost in the exchange.

**Round 1 of 2.** Elaborates
[`../Proposal_machine-queryable_code-search_ast-grep_ripgrep_pcre2.md`](../Proposal_machine-queryable_code-search_ast-grep_ripgrep_pcre2.md).

Round 1's brief is **library feature discovery in depth**, per subsystem, so that round 2 can
spend itself on integration rather than on finding out what the libraries do. Every capability
claim below is backed by a dossier in [`evidence/`](evidence/), read from the pinned
`datafusion`, `deltalake`, `rust-code-model`, `typer-rich` and `ast-grep-ripgrep` skills, plus
**the full probe backlog executed** — P01 and P02 from the first pass, then PB01–PB12 compiled and
run against the pinned Rust stack. No claim here comes from recollection, Context7 or the web.

Six of those probes **contradicted** the round-1 reading of the indexes, and three uncovered
failure modes that produce **wrong answers with no error**: a `preimage` returning a closed upper
bound drops rows, a `Volatile` UDF loses its filter entirely, and a session handed to a delta-rs
builder can be discarded with only a log line. All corrections are applied in place and the
superseded text is kept, marked, rather than deleted.

---

> **Round 2 is done and lives in [`PLAN-V2.md`](PLAN-V2.md) — implement from there.**
> Steps 1–5 of it are built; see [`IMPLEMENTATION.md`](IMPLEMENTATION.md). This
> document remains the per-subsystem capability record: it is where to look for *what a library
> can do and why*. v2 is the integrated design, and where the two disagree, **v2 wins**. §10 below
> ("what round 1 deliberately did not settle") is superseded in full.
>
> **Resuming?** Read [`HANDOFF.md`](HANDOFF.md) first — it carries the settled decisions, the
> facts established by execution, the remaining scope, and the harness traps.

## 0. Settled before planning

| Decision | Value | Why |
|---|---|---|
| Language split | **Rust core, Python CLI shelling out** | the `datafusion` and `deltalake` skills index the Rust crates; typer-rich is Python. Operator decision. |
| Location | this directory, beside the proposal | touches nothing `verify.py` digests |
| Pins | datafusion `=55.1.0`, arrow `59.3.0` (+`canonical_extension_types`), deltalake `1.0.0` **at git rev `58f07cd6…`**, object_store `0.13.2`, sqlparser `0.62.0`, rich `15.0.0`, typer `0.27.2` | evidence 09 P02 — all already aligned with this workspace |

**The deltalake pin is a git rev, not a crates.io release.** A `1.0.0` from crates.io is a
different artifact (evidence 09 P02).

---

## 1. Architecture at a glance

```
                      ┌──────────────────────────────────────────┐
   extraction         │  seven families (evidence 06)            │
   (Rust adapters)    │  rustdoc-json · syntax · hir · load-cargo │
                      │  mir · dataflow · cargo-metadata         │
                      └───────────────────┬──────────────────────┘
                                          │  Arrow RecordBatches, typed at the boundary
                                          ▼
   staging schema     ┌──────────────────────────────────────────┐
                      │  codesearch.staging.*   (Delta)          │
                      └───────────────────┬──────────────────────┘
                                          │  six MERGE passes, each source = a DataFrame
                                          ▼
   canonical model    ┌──────────────────────────────────────────┐
                      │  codesearch.snapshot / program /         │
                      │            catalog / evidence  (Delta)   │
                      │  + CHECK constraints, generated columns  │
                      └───────────────────┬──────────────────────┘
                                          │  ViewTable — no storage
                                          ▼
   projections        ┌──────────────────────────────────────────┐
                      │  codesearch.projection.*                 │
                      │  discover · describe · compare ·         │
                      │  validate · explain                      │
                      └───────────────────┬──────────────────────┘
                                          │  Arrow IPC / JSON on stdout
                                          ▼
   interface          ┌──────────────────────────────────────────┐
                      │  Typer + Rich CLI (separate process)     │
                      └──────────────────────────────────────────┘

   cross-cutting      ┌──────────────────────────────────────────┐
                      │  bridge/  — every UDF, provider wrapper, │
                      │  rule, extension type, in one place      │
                      └──────────────────────────────────────────┘
```

One `SessionState` is threaded through all of it — extraction, merge, constraint evaluation,
optimisation, projection and serving. Evidence 05 §3 establishes that delta-rs's builders all
accept `Arc<dyn Session>`, which is what makes that possible.

---

## 2. Subsystem A — the data fabric: catalog topology

*Goal (2): use the full catalog/schema/table-provider framework.*
*Evidence: 01.*

### A.1 One catalog, six schemas

```
catalog  codesearch
  ├── snapshot     Artifact · ExtractionRun · Entity · NativeBinding · SourceAnchor
  ├── program      Package … FlowFact — the typed program records (proposal §8.2)
  ├── catalog      Capability · Mechanism · Contract · Parameter · Interaction · PlanFragment
  ├── evidence     Evidence · Derivation · BehaviorAssertion · Coverage
  ├── staging      one table per extraction family, replaced per run
  └── projection   views only — zero storage
```

Registered through `CatalogProviderList` → `CatalogProvider` → `SchemaProvider` →
`TableProvider`, all four of which are traits in `datafusion_session` (evidence 01 §1).
`MemoryCatalogProvider` and `MemorySchemaProvider` are used by composition; only the leaf
`TableProvider` is custom.

**Why six schemas and not one wide table.** The proposal asks for "one canonical structure — not
seven mirrored models". Schemas are the unit of *namespacing*, not of storage separation: one
catalog means one qualified name space, one optimiser, one UDF registry, one set of extension
types. Splitting by record family keeps `information_schema` legible and lets the retrieval layer
grant a projection access to `catalog` and `evidence` without exposing `staging`.

### A.2 `projection` is `ViewTable`, not a function library

`datafusion_catalog::view::ViewTable` is "an implementation of `TableProvider` that uses another
logical plan" (evidence 01 §1). Every retrieval operation the proposal names — `discover`,
`describe`, `compare`, `validate`, `explain` — is registered as a view in `projection`.

Consequences, all of which are goal (4):

- a projection is **planned and optimised with the query that uses it**, so a filter the CLI
  applies is pushed into the projection's own scan rather than applied after materialisation;
- a projection is introspectable through `information_schema` — the CLI can list what it can ask
  without a parallel registry;
- `TableProvider::get_logical_plan` returns the view's plan, so `explain` is free;
- nothing can call a projection "wrong", because there is no calling convention.

### A.3 The provider wrapper — the one custom `TableProvider`

**Evidence 05 §1 is the finding that forces this.** delta-rs's `DeltaScan` implements only
`scan`, `schema`, `table_type`, `supports_filters_pushdown`, `insert_into`, `get_logical_plan`
and `get_table_definition`. It does **not** implement `constraints`, `statistics`,
`get_column_default`, `scan_with_args`, `merge_into`, `update`, `delete_from` or `truncate`.

So `bridge::provider::CanonicalTable` wraps `DeltaScan` and adds:

| Method | What the wrapper supplies |
|---|---|
| `constraints()` | the **declared** key as `Constraints::new_unverified`. **Measured to change the plan — probe PB02, evidence 11.** Not *derived* from the table's Delta `CheckConstraints`: probe PB12 (evidence 12) shows those are stored as SQL text while `Constraints` models only `PrimaryKey`/`Unique` over column indices, so an arbitrary CHECK has no slot in the type. The key is declared in both places and reconciled by a build-time consistency check. |
| `get_column_default()` | the Delta column default, so DataFusion and Delta agree |
| `scan_with_args()` | delegates to `scan`, honouring `StatisticsRequest` |
| `merge_into()` | delegates to `MergeBuilder` so callers see **one** seam |
| ~~`statistics()`~~ | **removed — delta-rs already supplies them at the `ExecutionPlan` level** (PB02b). `DeltaScanExec` reports `Rows=Exact(n)` from the transaction log plus per-column min/max/null/distinct from Parquet, whether or not the provider implements `statistics()`. |
| everything else | delegates unchanged |

This is one file, and it is the only place the Delta/DataFusion impedance mismatch is handled.

**PB02 settles that it is worth building, on `constraints()` alone.** Declaring
`Constraint::PrimaryKey([0])` propagates into the scan schema as a functional dependence covering
every column, and two query shapes the retrieval layer leans on re-plan as a result:

```
SELECT DISTINCT entity_id FROM t      Aggregate + TableScan   ->   TableScan
                                      (the aggregation is eliminated outright)

self-join on the key                  Inner Join + Projection ->   LeftSemi Join
```

So every canonical table declares its key, and the matching Delta `CheckConstraints` (§4.2) is
what makes the declaration true — `Constraints::new_unverified` is trusted, not verified.

### A.4 Session assembly

One `SessionStateBuilder` call site, in `bridge::session`. Evidence 01 §4 enumerates the
registration surface; the plan uses:

```
with_catalog_list            the topology of A.1
with_table_factories         DeltaTableFactory, so CREATE EXTERNAL TABLE works
with_scalar_functions        bridge::udf::scalar::*        (§5)
with_aggregate_functions     bridge::udf::aggregate::*
with_window_functions        bridge::udf::window::*
with_table_functions         bridge::udf::table::*
with_extension_type_registry bridge::types::*              (§3.2)
with_analyzer_rule           bridge::rules::PrecisionPropagation
with_optimizer_rule          bridge::rules::MechanismPredicatePushdown
with_statistics_registry     bridge::stats::DeltaLogStatistics
with_object_store            local filesystem for round 1
with_config                  SessionConfig::with_information_schema(true)
```

`information_schema` on is deliberate: it makes the catalog self-describing, which the CLI's
`describe` uses instead of a hand-maintained manifest.

**Two corrections from probes PB05 and PB07 (evidence 12), both about how that one session
survives contact with delta-rs:**

1. **The session is built from delta-rs's `create_session()`, not `SessionContext::new()`.** Delta
   writes plan through delta-rs's own `MetricObserver` node, so a bare DataFusion session cannot
   execute a Delta write at all — it fails with *"No installed planner was able to convert the
   custom node to an execution plan"*. `create_session().into_inner()` yields a `SessionContext`
   on which the registrations above are made before taking `.state()`.
2. **Every delta-rs builder call site sets `with_session_fallback_policy(RequireSessionState)`.**
   The default is `InternalDefaults`, which — per `delta_datafusion/session.rs:125` — *"logs a
   warning and uses internal defaults"* when the supplied session is not a `SessionState`. That
   silently discards the whole UDF registry, extension types and custom rules, which is precisely
   the failure this section exists to prevent. `RequireSessionState` turns it into an error.

---

## 3. Subsystem B — the canonical schema

*Goal (1): complex schemas, typed data, metadata, predicates.*
*Evidence: 03 (Arrow), 04 §1 (what Delta will actually store).*

### B.1 Every column type must be a fixed point of the Delta conversion

**Measured, not derived — probes PB01 and PB03 (evidence 10).** The round-1 derivation in
evidence 04 §1 was wrong: Delta does not reject the types the plan wanted. It **accepts almost
all of them and silently narrows several**, which is the more dangerous outcome, because
`normalize_for_delta` reports *no change* for exactly the types the write path then narrows.

| written as | read back as | verdict |
|---|---|---|
| `Utf8`, `Int32`, `Int64`, `Boolean`, `Binary`, `Timestamp(µs)`, `Date32`, `Decimal128` | unchanged | **fixed point — use these** |
| `Struct`, `List`, `Map` | unchanged in type; **child field names rewritten** | use, with the Parquet spellings |
| `UInt32` | `Int32` | **silent range loss — never use** |
| `UInt64` | `Int64` | **silent range loss — never use** |
| `FixedSizeBinary(32)` | `Binary` | width lost; re-impose with a CHECK constraint |
| `Dictionary(Int32,Utf8)` | `Utf8` | flattened; in-memory only |
| `Utf8View` / `BinaryView` / `LargeUtf8` / `LargeList` | `Utf8` / `Binary` / `Utf8` / `List` | narrowed |
| `Float16` | — | the only outright rejection |

So the schema rule is a single sentence: **choose each column's Arrow type so that the type
written and the type read back are identical.**

```
        persisted (Delta)          in-memory (Arrow)                    boundary
id      Utf8 + extension type      extension codesearch.entity_id       NATIVE — PB01
hash    Utf8 + extension type      extension codesearch.content_hash    NATIVE — PB01
enum    Utf8                       Dictionary(Int32, Utf8)              flattened on write
range   Struct<start:Int64,        Struct<start:Int64,end:Int64>        NATIVE
               end:Int64>
child   List<Struct<…>>            List<Struct<…>>                      NATIVE; child named `element`
free    Map<Utf8,Utf8>             Map<Utf8,Utf8>                       NATIVE; `key_value`/`key`/`value`
opaque  Variant                    —                                    kernel-native
```

Three consequences worth stating separately:

1. **Byte offsets are `Int64`.** The plan previously wanted `UInt32`. A source file over 2 GiB
   produces offsets above `i32::MAX`, and `UInt32` → `Int32` is a schema mapping with no value
   check and no error. This is the single most consequential correction in round 1.
2. **Nested child field names are rewritten to Parquet conventions** (`item` → `element`,
   `entries`/`keys`/`values` → `key_value`/`key`/`value`). Any schema-equality check must compare
   normalised schemas, or the plan must author the Parquet spellings from the start. It does.
3. **`normalize_for_delta` is not a preview of persistence.** It returned all twenty inputs
   unchanged, including every type the write path narrows. Do not use it as a check.

### B.2 Extension types carry identity

`arrow_schema::extension::ExtensionType` requires `supports_data_type` and provides `validate`
(evidence 03 §2) — so the type validates its own storage. Registered into the session via
`ExtensionTypeRegistry` (evidence 01 §4), a UDF's `Signature` can then demand
`codesearch.entity_id` rather than `Utf8`, making "this is an entity id, not a string that looks
like one" a **plan-time error**.

```
codesearch.entity_id      over Utf8
codesearch.content_hash   over Utf8            (hex; FixedSizeBinary loses its width — PB03)
codesearch.mechanism_id   over Utf8
codesearch.source_anchor  over Struct<artifact_id: Utf8, start: Int64, end: Int64>
codesearch.native_handle  over Struct<run_id: Utf8, namespace: Utf8, handle: Utf8>
```

**Probe PB01 confirms these persist.** `ARROW:extension:name` and `ARROW:extension:metadata`
round-trip through a Delta write/read cycle byte-for-byte, with an ordinary metadata key as the
control. Extension identity is a *storage* property here, not something re-attached on read — so
the types are declared once at table-create time and are still there when the table is reopened.

The last one enforces the proposal's §8.1 rule structurally: *"Native handles must be run-scoped.
Rustdoc IDs, HIR handles, compiler `DefId`s, and MIR local numbers do not share a namespace."*
With `run_id` inside the type, an unscoped handle **cannot be represented**. Evidence 06 §2
supplies the concrete reason: probe RD004 shows a rustdoc `Id` shifts from `0` to `41` when an
unrelated item is inserted before it.

### B.3 `Precision` is the epistemic spine

`datafusion_common::stats::Precision = Exact | Inexact | Absent`, with propagating arithmetic
(`add`, `multiply`, `min`, `max`, `to_inexact`, `with_estimated_selectivity`) — evidence 03 §4.

Mapped onto the proposal's §1 classification:

| Proposal class | Carrier |
|---|---|
| `exact_under_stated_preconditions` | `Precision::Exact` |
| `candidate_generation_or_approximation` | `Precision::Inexact` |
| `uncharacterized` | `Precision::Absent` |
| `not_supported_through_this_surface` | a `Contract` row, **not** a statistic — it is a fact about the surface, not a degree of confidence |

**The propagation is the point.** The proposal's §10 prefilter condition —
`final_match(file) ⇒ prefilter_selects(file)`, "when that implication is merely plausible, mark
the prefilter heuristic" — becomes arithmetic the engine already performs, provided recall is
modelled as a statistic. Composing an `Exact` mechanism with an `Inexact` one yields `Inexact`
with nobody writing that rule.

The same three-valued vocabulary appears at two more levels, which is why it unifies the design:
`TableProviderFilterPushDown = Exact | Inexact | Unsupported` (evidence 01 §2) and every field of
`ColumnStatistics` (evidence 03 §5). One vocabulary runs from the capability contract down to the
file scan.

### B.4 Constraints, dependencies and metadata

- **Keys.** `Constraint::PrimaryKey` on every `*_id`; `Constraint::Unique` on
  `(run_id, native_namespace, native_handle)` — the proposal's crosswalk invariant.
  `Constraints::new_unverified` is the only constructor (evidence 01 §3), so these are
  declarations DataFusion trusts. **Verification is Delta's job** (§4.2), not DataFusion's.
  These are not documentation: probe PB02 measured a declared key eliminating a `DISTINCT`
  aggregation and converting an inner join to a semi-join (evidence 11).
- **Functional dependencies.** `FunctionalDependence::new(source_indices, target_indices,
  nullable).with_mode(Dependency::Single)` declares e.g. `entity_id → (kind, owner_id,
  snapshot_id)`, letting `GROUP BY entity_id` skip re-grouping dependents.
- **Field metadata.** Every column carries `FieldMetadata` (a `BTreeMap`, therefore
  deterministic — evidence 03 §3) naming the extraction family permitted to write it, the
  proposal record type, and its evidence requirement. `merge_options` gives a defined answer when
  a projection combines two provenances. Writable and durable via
  `UpdateFieldMetadataBuilder` (evidence 04 §3).
- **Schema metadata** carries the proposal's §1 snapshot manifest, and is additionally committed
  as Delta `DomainMetadata` (evidence 04 §2) so it is versioned with the data.

### B.5 Nested, not shredded

Ordered child records — generic arguments, projections, syntax children, `Derivation.input_fact_ids` —
stay as `List<Struct<…>>`. Order is intrinsic to the type, so there is no `ordinal` column and no
side table. They remain queryable in place via `Unnest`, `flatten`, `array_filter`,
`array_transform`, `array_any_match` and `set_ops` (evidence 02 §5), the higher-order forms taking
`Expr::Lambda`.

This is goal (1)'s "minimise repeats and discontinuities" made concrete: the alternative is a
dozen join tables whose only purpose is to restore an order the type already knew.

---

## 4. Subsystem C — persistence

*Goal (3): contain persistence in a Delta basis.*
*Evidence: 04, 05.*

### C.1 Table features to enable, and why

From the 30 available (evidence 04 §2):

| Feature | Use |
|---|---|
| `CheckConstraints` | structural invariants enforced at write (§4.2) |
| `GeneratedColumns` | partition and sort keys computed by the writer, never by a caller |
| `RowTracking` | stable row identity across `OPTIMIZE` — serves the proposal's §8.1 |
| `ChangeDataFeed` | incremental re-derivation (§6.3) |
| `DomainMetadata` | the snapshot manifest, versioned with the data |
| `DeletionVectors` | merge passes touch many small updates; avoids whole-file rewrites |
| `ClusteredTable` | locality on high-cardinality `entity_id` without a partition hierarchy |
| `ColumnMapping` (`Name`) | cheap renames; prerequisite for `TypeWidening` |
| `TypeWidening` | schema growth as families are added, without rewrite |

Protocol versions are pinned explicitly using `MAX_VALID_READER_VERSION` /
`MAX_VALID_WRITER_VERSION` rather than accepting what the writer negotiates. `Preview` features
(`TypeWideningPreview`, `VariantTypePreview`, `VariantShreddingPreview`, `CatalogOwnedPreview`)
are **not** enabled in round 1.

### C.2 Validation lives in the table

`ConstraintBuilder::with_constraint(name, expression)` takes a DataFusion `Expression` and commits
it (evidence 05 §2). The proposal's structural invariants become named CHECK constraints:

```
evidence_has_run          evidence.extraction_run IS NOT NULL
derivation_is_versioned   derivation.rule_version IS NOT NULL
anchor_is_ordered         source_anchor.native_range.start <= source_anchor.native_range.end
precision_is_known        precision IN ('exact','inexact','absent')
assertion_has_support     behavior_assertion.status <> 'supported'
                            OR cardinality(supporting_evidence) > 0
mechanism_surface_valid   mechanism.surface IN ('ast_grep_cli','ast_grep_rule','rg_cli','rg_pattern')
```

A row violating one cannot be committed. **No validation pass exists, because there is nothing
left for it to check** — which is goal (1) reached at its endpoint.

**Two hard limits, both measured by probe PB07 (evidence 12), and both stronger than expected.**

1. **A CHECK expression must not reference another table — the attempt aborts the process.**
   `DeltaContextProvider::get_table_source` is `unimplemented!()`
   (`delta_datafusion/expr.rs:237`), so any Delta expression naming a relation panics rather than
   returning an error. Referential invariants are therefore `projection.violations_*` views the
   CLI queries — still relational, still zero bespoke code, just not write-blocking. Because the
   failure mode is a process abort, **constraint text must never be reachable from input.**

2. **A CHECK expression must not reference a UDF, even though it is accepted.** delta-rs stores
   the constraint as SQL *text* and re-parses it at write time against *the writer's* session
   (`data_validation.rs:319`). A UDF-bearing constraint therefore makes the table unwritable by
   any session that does not register that UDF — durable, table-level coupling to this tool's
   code, outliving the session that created it. Every constraint above is plain column algebra
   for that reason; none calls a `bridge::udf`.

### C.3 Physical layout

```
snapshot.artifact            cluster artifact_id
snapshot.extraction_run      partition family          cluster run_id
snapshot.entity              partition kind            cluster entity_id
snapshot.native_binding      partition native_namespace cluster canonical_entity_id
snapshot.source_anchor       cluster artifact_id
program.*                    partition record_kind     cluster owner_id, entity_id
evidence.evidence            partition observation_kind
evidence.derivation          partition rule_id
evidence.behavior_assertion  partition subject_kind
catalog.*                    small — no partitioning
```

Partition on low-cardinality vocabulary columns (the ones every decision-packet query filters on);
cluster on high-cardinality identity columns (the join keys). `OptimizeType::ZOrder` on
`(entity_id, kind)` after merge passes (evidence 05 §2).

---

## 5. Subsystem D — `bridge/`: the consolidated extension catalog

*Goal (5): bridge gaps with UDFs and custom extensions, organised in one place with a clear
catalog.*
*Evidence: 02 §4, 01 §4.*

**Everything that extends DataFusion lives under one module tree and nowhere else.** This is a
hard structural rule, so that "what have we added to the engine?" has a directory as its answer.

```
bridge/
  session.rs      the single SessionStateBuilder call site (§2.4)
  types/          Arrow extension types + ExtensionTypeRegistration  (§3.2)
  provider/       CanonicalTable — the DeltaScan wrapper             (§2.3)
  stats/          StatisticsProvider reading the Delta log
  udf/
    scalar/       ScalarUDFImpl
    aggregate/    AggregateUDFImpl
    window/       WindowUDFImpl
    table/        TableFunctionImpl
  rules/          AnalyzerRule / OptimizerRule / PhysicalOptimizerRule
  planner/        ExprPlanner · RelationPlanner · TypePlanner
  nodes/          UserDefinedLogicalNode + ExtensionPlanner, if needed
  CATALOG.md      generated inventory — see below
```

### D.1 A UDF here is a full optimizer participant

Evidence 02 §4: `ScalarUDFImpl` has 24 methods, 20 overridable. The plan requires every custom
scalar UDF to implement, beyond the four mandatory ones:

- `documentation()` — so `information_schema` and the CLI's `describe` are self-serving;
- `return_field_from_args()` — returning a **field**, so the UDF stamps provenance metadata on its
  own output rather than losing it;
- `simplify()` where a constant-folding rule exists;
- `preimage()` wherever the UDF may appear in a filter — this is what lets `f(col) = lit` rewrite
  into a prunable predicate on `col` and reach the Delta scan.

**Probe PB06 (evidence 12) confirms it fires, and prunes** — a bucketing UDF rewrote to a range on
the bare column and cut four Delta files to one, matching a hand-written range exactly. Domain UDFs
are therefore admissible in hot filters and the fallback to generated columns is not needed.

But the probe also produced **three conditions, two of which fail silently and produce wrong
answers**. They are requirements, not advice:

| Rule | Why | What happens if broken |
|---|---|---|
| The UDF is `Volatility::Immutable` | `expr_simplifier.rs:2111` returns `PreimageResult::None` for anything else | not merely un-rewritten — the predicate loses its `partial_filters` and never reaches the scan at all. No diagnostic. |
| `preimage` returns a **half-open** upper bound | `udf_preimage.rs:47` rewrites `Eq` to `expr >= lower AND expr < upper`, so `Interval`'s nominally-closed upper is consumed as exclusive | **rows are silently dropped.** The measured arm returned 99 where the answer was 100 — same plan, same pruning, no error. |
| `preimage` returns a predicate on a **bare stored column** | Delta min/max statistics are keyed on stored columns | rewritten but **not pruned**. The built-in `floor` does exactly this: its preimage names `CAST(off AS Float64) / 4096`, and the scan still reads every file. |

Rewriting and pruning are two different achievements; a UDF can pass the first and fail the second,
and the plan looks optimised either way. So each UDF's tests carry a **boundary-value case at the
top of a bucket** — without one, the half-open error is invisible.

### D.2 The initial UDF inventory

| Function | Kind | Job | Why it cannot be built-in |
|---|---|---|---|
| `precision_and(a, b)` | scalar | compose two recall classes | `Precision` arithmetic is Rust-side; SQL needs it as a function |
| `precision_or(a, b)` | scalar | the lattice's other join | as above |
| `anchor_contains(outer, inner)` | scalar | source-range containment | `Struct` comparison with the right semantics |
| `anchor_overlaps(a, b)` | scalar | range intersection | as above |
| `mechanism_satisfies(contract, request)` | scalar | the core selection predicate of proposal §2 | domain logic over two `Struct`s |
| `supports_relationship(mech, rel)` | scalar | `textual_equality` … `data_flow` (proposal §1) | domain vocabulary |
| `kind_is_a(lang, kind, super)` | scalar | grammar node-kind subsumption | grammar metadata lookup |
| `regex_engine_supports(construct, engine)` | scalar | reads `regex.tsv` semantics | two-engine matrix |
| `coverage_denominator(surface)` | aggregate | the honest denominator for proposal §8 | needs the surface roster |
| `evidence_chain(fact_id)` | table | derivation provenance as rows | recursive walk exposed as a relation |
| `probe_replay(probe_id)` | table | re-execute a behaviour probe | invokes a binary; must be a table function |

`evidence_chain` is listed as a table function **and** is expressible as a recursive CTE (§6.4);
the table function exists so the CLI has a stable name for the common case, and its
implementation is the CTE.

### D.3 `CATALOG.md` is generated, not written

A build step queries `information_schema` plus each UDF's `documentation()` and writes
`bridge/CATALOG.md`. The consequence is that the inventory of custom functionality cannot drift
from the code — which is the point of asking for a consolidated location in the first place.

---

## 6. Subsystem E — extraction, merge and derivation

*Goal (4): make relational things programmatically relational.*
*Evidence: 06 (families), 05 §2 (merge), 02 §1 (recursion).*

### E.1 Adapters produce Arrow, not objects

Per the proposal: *"The source libraries' objects exist only inside extraction adapters."* Each of
the seven families has one adapter whose only output is `RecordBatch`es conforming to a staging
schema. The adapter is the **only** code that knows `rustdoc_types::Id`, `ra_ap_hir::Semantics` or
a MIR dump exists.

Each adapter writes with `WriteBuilder::with_input_plan` and
`with_replace_where(family = 'F' AND snapshot_id = 'S')` (evidence 05 §2), which makes re-running
one family idempotent without touching the others.

**Schema consequences forced by evidence 06 §1**, each a `cannot_answer` made structural:

- every typed record carries a `precision` companion, because four of seven families have a
  documented blind spot that produces a plausible-looking null;
- `MirLocal` has no `name`; it has an optional `debug_annotation`, named for what it is;
- features from `cargo_metadata` are `declared`, never `resolved` (probe PL003);
- `Resolution` records **which `descend_into_macros` variant produced it** — there are seven and
  they disagree by construction (evidence 06 §2);
- `MirBody` carries `MirPhase`, because "the MIR" is ambiguous across `built` / `analysis` /
  `runtime` (probe DF003).

### E.2 The six passes are six merges

`MergeBuilder::new(log_store, snapshot, predicate, source: DataFrame)` — **the source is a
DataFusion DataFrame** (evidence 05 §2), so each of the proposal's §9 passes is a query over the
catalog, not a procedure:

| Pass | Target | Source DataFrame | Clauses |
|---|---|---|---|
| A identity | `snapshot.entity` | staging cargo-metadata + artifacts | matched update, not-matched insert |
| B declarations | `program.definition` | staging rustdoc + hir, ordered by depth | matched update, not-matched insert |
| C crosswalk | `snapshot.native_binding` | all staging native handles | not-matched insert only |
| D semantics | `program.*` | staging hir + mir | matched update, not-matched insert |
| E types | `program.type_term` | staging, conservatively normalised | matched update |
| F derivation | `evidence.*` | derived facts + `Derivation` rows | all five clause kinds |

`when_not_matched_by_source_update` / `…_delete` cover retraction — when a re-run no longer
reports a fact, the row is marked rather than silently surviving. `with_merge_schema(true)` gives
schema evolution at merge time.

**Probe PB05 (evidence 12) confirms domain predicates work in both positions** — the join
predicate and the match-clause predicate — provided the session carries the UDF, which §2.4 now
guarantees. Match-clause predicates are resolved separately from the join predicate, and they are
where a pass like E (conservative type normalisation) would actually place a domain condition.

### E.3 Re-derivation is incremental, via CDF

`DeltaCdfTableProvider` **implements `TableProvider`** (evidence 05 §1) — the change feed is a
queryable table. So "re-derive what changed" is a join against a table between two versions, using
`CdfLoadBuilder`'s version/timestamp bounds, not a diffing routine. It is reached as
`DeltaCdfTableProvider::try_new(table.scan_cdf())`; `CdfLoadBuilder::new` itself is `pub(crate)`.

**Probe PB10 (evidence 12) measured what narrows it, and the answer is narrower than "filter
pushdown works".** `supports_filters_pushdown` returns `Exact` for *every* filter unconditionally,
so the filter always vanishes from the plan above the scan — which is not evidence of anything.
What was measured:

| Filter | rows | files scanned | narrowed? |
|---|---|---|---|
| none | 6 | 3 | — |
| `family = 'hir'` (**partition column**) | 4 | **2** | yes, with a real `pruning_predicate` |
| `n >= 20` (data column) | 2 | 3 | no |
| `_commit_version >= 2` (CDF metadata) | 4 | 3 | no |

Every answer is correct — a `FilterExec` inside the provider's own plan restores row-level
correctness. Only partitions eliminate files. Two consequences:

- **partition the canonical tables on `family`**, the column a re-run narrows to (already the case
  for `snapshot.extraction_run` in §4.3; it now extends to every table CDF is read from);
- **narrow the change range with `with_starting_version` / `with_ending_version`, never with a
  predicate on `_commit_version`** — the predicate is correct and reads the whole range anyway.

The CDF schema is `[…user columns…, _change_type, _commit_version, _commit_timestamp]`, so those
metadata columns are available for projection and joining; they are just not a narrowing device.

### E.4 Graph questions are recursive CTEs

`LogicalPlan::RecursiveQuery { name, static_term, recursive_term, is_distinct, schema }`
(evidence 02 §1). This is what discharges the proposal's §11 position that a graph database is not
required:

| Question | Recursive CTE over |
|---|---|
| module / definition containment closure | `Definition.owner_id` |
| call-graph reachability | `CallSite → CallTarget → Definition` |
| dependency transitive closure (**replaces Guppy**) | `DependencyEdge` |
| CFG reachability | `CfgEdge` |
| re-export chains | `ExportPath` |
| derivation provenance | `Derivation.input_fact_ids` |

**Probe PB09 (evidence 12) settles the bounding question, and the answer makes the mitigation
mandatory.** An outer predicate is **not** pushed into the recursive term: DataFusion leaves
`Filter: depth <= 3` above `RecursiveQuery`, so the recursion runs to fixpoint first. On a graph
with a cycle it therefore never terminates — the measured arm **timed out at 20s**, while the same
bound written *inside* the recursive term returned in 24ms.

So every one of the six closure queries above:

- carries an explicit `depth` column, and
- bounds it **inside the recursive term** (`WHERE c.depth < :max_depth` in the recursive select),
  never in an outer `WHERE`.

The earlier claim that *"`is_distinct` gives cycle tolerance without a written visited-set"* is
**withdrawn**. The measured plan shows `is_distinct=false` for a `UNION ALL` recursion, and this
probe did not test `UNION`-flavoured recursion at all. Whether `is_distinct=true` terminates on a
cyclic graph is unmeasured, and the design does not depend on it.

---

## 7. Subsystem F — retrieval and decision packets

*Evidence: 01 §2 (pushdown), 02 §3 (optimizer), 03 §4 (Precision).*

### F.1 The five operations are five views

`projection.discover`, `.describe`, `.compare`, `.validate`, `.explain` — each a `ViewTable`
(§2.2). The CLI passes parameters as `Expr::Placeholder` bindings, never string interpolation.

### F.2 Prerequisite completeness is a join, not a policy

The proposal's §9 requires that retrieval "should not decide which prerequisites are safe to
omit". Expressed relationally: a decision packet is a view whose definition **inner-joins** its
prerequisites. A mechanism with an unsatisfied required parameter cannot appear without it,
because the row would not exist. Optional explanation is a `LEFT JOIN` and may be projected away
under a token budget; mandatory context cannot be, by construction.

### F.3 Token budget is `LIMIT`, applied to candidates only

Reducing the packet means fewer candidate mechanisms (`LIMIT` on the candidate CTE), never fewer
columns of a candidate. The proposal: *"Do not silently truncate mandatory conditions. If the
essential context cannot fit, return a smaller complete candidate set and indicate what was
omitted."* The `omitted` count comes from the same query as a windowed total, so it cannot
disagree with what was returned.

---

## 8. Subsystem G — the CLI

*Evidence: 07, and round-1 probe P01.*

### G.1 Command surface

```
codesearch discover   --subject --relationship --result --surface --language [--json]
codesearch describe   MECHANISM_ID... [--field ...] [--json]
codesearch compare    MECHANISM_ID MECHANISM_ID [--dimension ...] [--json]
codesearch validate   --rule FILE | --pattern TEXT [--fixture ...] [--json]
codesearch explain    ASSERTION_ID... [--depth N] [--json]
codesearch catalog    tables | columns | functions          # information_schema passthrough
codesearch snapshot   build | status | vacuum | optimize
```

Typer for parsing and exit codes; the Rust binary does the work; Arrow IPC or JSON crosses the
boundary.

### G.2 The markup rule — measured, not assumed

Round-1 probe **P01** (evidence 09) measured 24 bracketed forms against rich 15.0.0. **Eleven are
silently deleted**, including `[a-z]`, `[a-zA-Z_]`, and every ast-grep rule field name:
`[pattern]`, `[kind]`, `[inside]`, `[has]`, `[stopBy]`, `[regex]`. `[A-Z]`, `[0-9]`, `[^a-z]` and
`[[:alpha:]]` survive — a one-character difference changes the outcome, so review cannot catch it.

Both remedies were controlled and both are clean: `escape()` rescued all 11; `markup=False`
preserved all 24.

**Therefore:**

1. Any value originating from the catalog, a pattern, a user or a probe is printed through
   `rich.markup.escape` or with `markup=False`. Only CLI-authored literals may carry markup.
2. This is enforced by an **ast-grep rule** in the repository's existing `project-*` corpus, not
   by convention — the failure is invisible, so a human gate is the wrong instrument.
3. `--json` never touches a `Console`. A Console applies width, wrapping and markup; probe `E003`
   confirms width is a Console property and the skill records that an explicit width still loses
   to `TERM=dumb`, so a piped packet could wrap differently per machine.
4. Patterns render through `rich.syntax.Syntax`, which takes content as code rather than markup
   (**probe PB08** confirms the lexer and the parsing behaviour).

### G.3 Renderable assignment

| Packet element | Renderable |
|---|---|
| candidates and why each matched | `Table`, `Precision` as the colour key |
| distinguishing alternatives | `Columns` of `Panel` |
| parameters and effective defaults | `Table`, defaults dimmed |
| interactions | `Tree` rooted at the mechanism |
| a constructed rule or pattern | `Syntax` — **markup-safe by construction** (PB08) |
| guarantees / non-guarantees | `Panel`, border keyed to `Precision` |
| evidence handles | `Table` of ids — never the evidence itself (proposal §11) |

**`Syntax` is the one renderable that needs no escaping.** Probe PB08 (evidence 12) put the eight
forms P01 measured as deleted through it: **0 of 8 altered**, with both controls behaving as P01
recorded. That makes it the right carrier for exactly the content most at risk — patterns and rule
fragments — while every `Console.print` still needs `escape()` or `markup=False` (§8.2).

It will not highlight them, though. **There is no regex lexer**: of 602 lexers in pygments 2.21.0
none covers regex syntax, and `Syntax(code, "regex")` resolves to `None` *identically to a
nonsense lexer name*, with no error. Patterns therefore render as safe, unhighlighted text.
Writing a small pygments `RegexLexer` is the only route to colour and is deferred — correctness is
free here, colour is not.

Progress and prompts come from Rich, not Typer: `typer.progressbar` is the vendored Click one and
fights a live Console, and Typer's prompts bypass the Console and are overwritten under `Live`
(evidence 07 §2). `typer.echo` is used for literal output and `typer.Exit` for chosen exit codes.
`rich.__version__` does not exist at 15.0.0 — use `importlib.metadata.version`.

### G.4 Output fidelity is tested by capture

`Console(record=True).export_text()` captures a rendered packet; captures are committed and
re-captured on build. Combined with `typer.testing.CliRunner` for the Typer layer — noting it pins
the plain formatter's width but not the Rich one, so Rich assertions pin a `Console` width
explicitly, as every probe in the typer-rich skill does.

---

## 9. Coverage accounting

The proposal's §8 asks for "two independent kinds of completeness". Both are relations, not
reports:

- **Surface coverage** — `catalog.mechanism` LEFT JOIN the surface roster extracted from the
  binaries' own help. The denominator is the roster, so "how much of ripgrep is catalogued" has an
  answer that cannot drift.
- **Evidence coverage** — `catalog.contract` LEFT JOIN `evidence.behavior_assertion`. A contract
  facet with no supporting assertion is `Precision::Absent`, and appears as such rather than
  being absent from the output.

This addresses the existing skill's known limit 2 — *"`regex.tsv` covers the constructs an agent
asks about, not every construct PCRE2 has"* (evidence 08 §3). A catalog that silently omits is
worse than one that reports its own denominator.

---

## 10. What round 1 deliberately did not settle

Per the brief, integration is round 2's job. Named here so nothing is lost:

1. **Cross-subsystem naming** — one identity vocabulary shared by `catalog.mechanism_id`,
   `snapshot.entity_id` and the seven families' native handles.
2. **The staging→canonical schema contract** per family. §6.1 gives the rules; the column lists
   are round 2.
3. **Execution ordering and failure semantics** across the six merge passes — what a partial
   failure leaves behind.
4. ~~**Whether the provider wrapper (§2.3) earns its keep**~~ — **ANSWERED by PB02** (evidence 11):
   yes, on `constraints()` alone.
5. **The `Interaction` model** (proposal §3). The proposal's typed interaction records need their
   own design pass; round 1 established only that they are ordinary rows with `Precision` and
   evidence, not a special structure.
6. **`PlanFragment` composition** (proposal §10) — the typed input/output units.
7. **Serving** — whether the Rust core is a one-shot process or a resident session.

**The probe backlog is closed.** PB01–PB12 all ran; evidence 09's backlog section now has nothing
open. Two questions remain that no probe answered:

- whether `WriteBuilder::with_input_plan` shares a transaction with a subsequent
  `ConstraintBuilder` call, or whether constraint addition is always a separate commit
  (evidence 05 §4, question 4);
- whether a `UNION`-flavoured recursive CTE with `is_distinct=true` terminates on a cyclic graph.
  PB09 measured only `UNION ALL`, and §6.4's claim about `is_distinct` was withdrawn rather than
  relied on.

---

## 11. Risk register

| Risk | Impact | Probe | Mitigation if it goes badly |
|---|---|---|---|
| ~~Extension metadata stripped by Delta~~ | — | **PB01 — done** | **CLOSED: metadata round-trips byte-for-byte.** Extension identity is a storage property |
| ~~Arrow→Delta type rejections broader than derived~~ | — | **PB03 — done** | **CLOSED, and inverted: the hazard is silent narrowing, not rejection** |
| **Silent type narrowing on write** (`UInt32`→`Int32`) | corrupt byte offsets above `i32::MAX`, nothing raised | **PB03 — done** | every column type is a fixed point of the conversion (§3.1); offsets are `Int64` |
| **Nested child field renaming** (`item`→`element`) | schema-equality checks fail on names | **PB03 — done** | author Parquet spellings; compare normalised schemas |
| ~~`preimage` does not fire on Delta scans~~ | — | **PB06 — done** | **CLOSED: it fires and prunes four files to one.** Generated-column fallback not needed |
| **A `preimage` returning a closed upper bound drops rows** | silently wrong results; measured 99 where the answer is 100 | **PB06 — done** | half-open upper bound is a §5.1 requirement; every UDF test carries a bucket-boundary case |
| **A `Volatile` UDF loses its filter entirely** | predicate never reaches the scan, no diagnostic | **PB06 — done** | every `bridge::udf::scalar` is `Immutable` |
| **A `preimage` on a derived expression does not prune** | looks optimised, reads everything | **PB06 — done** | `preimage` must name a bare stored column |
| ~~CHECK cannot express referential invariants~~ | — | **PB07 — done** | **CLOSED, and worse than expected: it PANICS** (`get_table_source` is unimplemented). `projection.violations_*` views; constraint text never reachable from input |
| **A UDF in a CHECK constraint locks out every writer without it** | the table becomes unwritable by other sessions, permanently | **PB07 — done** | constraints are plain column algebra only (§4.2) |
| **`SessionFallbackPolicy` defaults to discarding the caller's session** | UDFs, extension types and rules silently dropped with a log line | **PB05 — done** | `RequireSessionState` at every builder call site (§2.4) |
| ~~`DeltaOps` absent at the pinned rev~~ | — | **PB04 — done** | **CLOSED: it does not exist (E0432).** The individual builders are the only entry point, which the plan already assumes |
| ~~Provider wrapper may not earn its keep~~ | — | **PB02 — done** | **CLOSED: `constraints()` eliminates an aggregation and narrows a join.** `statistics()` dropped — delta-rs already supplies them |
| ~~Recursive CTEs unbounded on large corpora~~ | — | **PB09 — done** | **CLOSED, confirmed: an outer bound is NOT pushed in and a cyclic graph never terminates** (timed out at 20s vs 24ms). Explicit depth column, bounded inside the recursive term |
| ~~CDF change range cannot be narrowed~~ | — | **PB10 — done** | **CLOSED: partition columns narrow (3 files to 2); data and `_commit_version` columns do not.** Partition on `family`; bound by version, not by predicate |
| Rich markup corrupts catalog output | silently wrong docs and packets | **P01 — done** | escape / `markup=False`, enforced by ast-grep rule |
| ~~No safe renderable for patterns~~ | — | **PB08 — done** | **CLOSED: `Syntax` does not markup-parse** (0 of 8 lost). No regex lexer exists, so patterns render unhighlighted |

---

## 12. Evidence index

| # | Dossier | Covers |
|---|---|---|
| 01 | [catalog, session, providers](evidence/01-datafusion-catalog-session-providers.md) | the four provider traits, `TableProvider`'s full surface, `Constraints`, `SessionStateBuilder`'s 30+ extension points, `FunctionFactory`, `ExprPlanner`/`RelationPlanner`/`TypePlanner`, `ExtensionTypeRegistry` |
| 02 | [plans, optimizer, functions](evidence/02-datafusion-plans-optimizer-functions.md) | `LogicalPlan::RecursiveQuery`, `Expr`'s 36 variants incl. `Lambda`, 25 built-in optimizer rules, `ScalarUDFImpl`'s 24 methods, the nested/higher-order function families |
| 03 | [Arrow types, metadata, `Precision`](evidence/03-arrow-types-schema-metadata-precision.md) | 41 `DataType` variants, `ExtensionType`, `DFSchema`/`FieldMetadata`, `Precision` and `Statistics` |
| 04 | [Delta protocol, types, features](evidence/04-deltalake-protocol-types-and-features.md) | the 18 `PrimitiveType`s, the 30 `TableFeature`s, column mapping, physical layout |
| 05 | [Delta operations and the DataFusion seam](evidence/05-deltalake-operations-and-datafusion-seam.md) | **the `DeltaScan` correction**, `MergeBuilder`, `WriteBuilder`, `ConstraintBuilder`, `CdfLoadBuilder`, `OptimizeBuilder` |
| 06 | [the seven extraction families](evidence/06-rust-code-model-extraction-families.md) | each family's contract and blind spot, probe-backed; the `descend_into_macros` variants; MIR phases |
| 07 | [Typer and Rich](evidence/07-typer-rich-cli-surface.md) | the markup hazard, Typer-vs-Rich routing, renderables, export |
| 08 | [what the existing skill supplies](evidence/08-ast-grep-ripgrep-catalog-inputs.md) | ~12,000 existing rows mapped to canonical records; the gap list |
| 09 | [probes executed and open](evidence/09-probes-executed-and-open.md) | **P01** markup survival (executed, decisive), **P02** version alignment, and the probe backlog |
| 10 | [PB01/PB03/PB04 executed](evidence/10-pb01-pb03-executed-results.md) | **three gating probes answered.** Extension metadata survives; Delta narrows silently rather than rejecting; `DeltaOps` does not exist. Corrects evidence 03 and 04 |
| 11 | [PB02 executed](evidence/11-pb02-provider-wrapper-results.md) | **the wrapper is justified — on `constraints()` alone.** A declared key eliminates an aggregation and turns an inner join into a semi-join; `statistics()` is redundant |
| 12 | [PB05–PB12 executed](evidence/12-pb05-pb12-executed-results.md) | **the rest of the backlog.** `preimage` fires and prunes, under three conditions two of which fail silently; session UDFs reach merge predicates; a UDF in a CHECK constraint locks out future writers and a table reference panics; recursive CTEs do not push outer bounds in; CDF narrows only on partitions; `Syntax` is markup-safe; the PB11 statistics disagreement resolved |

| 13 | [PB13/PB14 executed](evidence/13-pb13-pb14-closing-open-items.md) | **the last two answerable questions, closed in round 2.** Write and constraint are always two commits and a failed constraint-add is non-destructive, so DDL-first is the measured-correct order; `is_distinct` tames a cyclic recursion only when the projection excludes `depth` |

Probe sources and captures: [`evidence/probes/`](evidence/probes/).
