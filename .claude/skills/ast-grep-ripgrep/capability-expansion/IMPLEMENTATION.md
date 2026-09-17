# Implementation handoff — `codesearch`

Updated 2026-09-16. Read this before touching [`codesearch/`](codesearch/). It exists so the next
session does not re-derive what is settled or re-make the mistakes already recorded.

**If you are starting cold:** §1 is the current state and how to verify it, §3 is the list of
things already measured (do not re-measure them), and **§4e is the next piece of work**. Everything
in §4 marked ✅ is built and green; the tree is uncommitted, so `git status` shows one untracked
directory rather than a diff.

**Reading order:** this file → [`codesearch/README.md`](codesearch/README.md) (why the code looks
the way it does) → [`PLAN-V2.md`](PLAN-V2.md) §12.1 for the sequence and §12.1b for what
implementation changed in the design → [`HANDOFF.md`](HANDOFF.md) for the probe findings the whole
thing rests on.

---

## 1. Where we stand

**PLAN-V2 §12.1 steps 1–5, 7 and half of 8 are complete, and step 6 all but its remaining
families.** Phases 0–7 of the approved plan are done. **Two snapshots now coexist in one catalog**,
which is what `compare` needed and what most of phase 7 actually cost.

```
8,616 index rows -> 20 canonical tables, 9,866 rows per snapshot, TWO extraction families
  catalog      8 capabilities · 323 mechanisms · 165 parameters · 319 bindings
               9 result contracts · 279 search domains · 10 negative-space records
               8 plan fragments · 32 interactions · 33 atoms
               621 surface entries · 49 behaviour assertions · 21 lattice rows
  library-api  23 packages · 716 definitions · 4,727 signatures
               2,241 implementations · 273 export paths · 5 unbound native handles
42 projections  resolved on demand; `codesearch-query projections` still lists them all
9 invariants    all 0 violations over 9,843 identity rows; 1 honestly vacuous
4 UDFs          lattice_label, derived_entity_id, atom_truth, entity_key_namespace
3 families      catalog, library-api, cargo-metadata -- the third fed by an ADAPTER
18 probes       the backlog is EMPTY; PB16 was the last
1 plan rule     IdentityDiscipline -- entity_key never joins to entity_id
3 ast-grep      the fourth enforcement tier, which was an empty directory
181 tests green 134 Rust (4 crates) + 47 Python, plus 3 rule fixture sets
25 tables       + package (155), crate_unit (869), feature_declaration (898),
                dependency_edge (369) -- all four from the adapter
p95 ~273 ms     `describe`, whole-process, debug build, against §8's 400 ms, with TWO
                snapshots and the analyzer rule. ~246 ms before the rule, ~200 ms with
                one snapshot. Both rises are named in PLAN-V2 §8; neither is a table count.
```

Everything is **uncommitted**. The tree is `?? .claude/skills/ast-grep-ripgrep/capability-expansion/`.

### Verify it still works

```bash
cd .claude/skills/ast-grep-ripgrep/capability-expansion/codesearch
just where               # confirm state paths are OUTSIDE any repository under study
just codesearch-check    # fmt + check + clippy + Rust tests + Python tests + ruff. The gate.
just catalog             # rebuild from ../.. (idempotent; a second run changes nothing)
just cli validate        # nine invariants, with what each inspected
just cli coverage        # THREE numbers, never a composite
just cli chains          # bounded chain enumeration over a cyclic fragment graph
just cli runs            # the extraction ledger: which runs finished, and where one stopped
just cli implements      # the catalog<->program join, with the evidence for each claim
just cli closures        # what each closure traversed, and what limits it
just cli re-exports      # where an item can be named from
just producers           # run the cargo-metadata adapter into a producer root
just cli closures        # THREE closures; dependency traverses 369 edges at depth 8
just cli discover --subject source-code --relationship containment --budget 5
just cli validate --invariant "entity_id is unique within its table"
just lint-rules          # the ast-grep corpus. Asserts on OUTPUT -- the status lies
just cli runs            # which snapshots exist, and which one a query resolves to
just fixture             # a modified copy of the skill, OUTSIDE every repository
just compare-fixture     # build it beside the real one and diff: SEVEN rows, no more
just bridge-catalog      # regenerate CATALOG.md; a staleness test fails if it is stale
just bench 20            # the serving criterion. Run it on an IDLE machine -- see §3.
```

The two probe crates are built separately, each against its own named toolchain:

```bash
cd ../evidence/probes/delta-arrow-probe
export CARGO_TARGET_DIR="${LIBENR_HOME:-$HOME/.cache/codesearch}/target/delta-arrow-probe"
cargo run --example pb17_replace_where    # arms A-D; arm B must destroy what A preserves
cargo run --example pb18_utf8_preimage    # a string prefix preimage, with its control

cd ../rustc-public-probe
export CARGO_TARGET_DIR="${LIBENR_HOME:-$HOME/.cache/codesearch}/target/rustc-public-probe"
cargo +nightly-2026-09-13 build && "$CARGO_TARGET_DIR/debug/rustc-public-probe"   # exit 0 = CONFIRMED
```

Host invariants that must stay green (all green now):

```bash
just lint-agents                                                    # from the repo root
cd .claude/skills/ast-grep-ripgrep && python3 build/verify.py --skip-rebuild
cargo metadata --format-version 1 --no-deps                         # must remain 3 members
```

---

## 2. What is built, by file

| File | What it settles |
|---|---|
| `bridge/src/lattice.rs` | the **six** lattices as `Int8` ranks; `lattice_label`; the `catalog.lattice` rows |
| `bridge/src/identity.rs` | key minting for every key kind (including the three that **degrade** — `pkg:`, `impl:`, `def:#fn:`), `entity_id = blake3(key ‖ snapshot)`, `derived_entity_id` so the derivation is checkable in SQL, and **`payload_digests`** — the content address `compare` decides `changed` by, with the rule for what it excludes |
| `bridge/src/interaction.rs` | `atom_truth` — one DNF literal against a request, as a `truth` rank |
| `bridge/src/session.rs` | the one session from delta-rs's `create_session()`; `information_schema` on; `udfs()` as the single inventory |
| `bridge/src/projection.rs` | **the whole retrieval surface**: 40 views, the join discipline, the `violations_*` / `invariant` / `validate` triple, the three bounded recursive CTEs, `install` (the run *and* snapshot filters, and the lazy schemas), `entity_all` / `compare` — the one relation and the one view that deliberately cross snapshots — and `check_all` |
| `build/src/runs.rs` | the run row, the reaping rule, and the status flip |
| `bridge/src/lazy.rs` | **`SchemaProvider` resolution on demand** — the registration cost, and why §7's eager argument survives it |
| `build/src/library_api.rs` | the second family: `program.*` from the skill's rustdoc index, and the never-fills set |
| `build/src/implementation_bindings.rs` | the authored catalog↔program join, validated against both halves |
| `bridge/src/provider.rs` | `CanonicalTable` — declares `constraints()`, omits `statistics()` |
| `bridge/CATALOG.md` | **generated** by `just bridge-catalog`. Never hand-edited |
| `model/src/schema.rs` | every Arrow schema and the field helpers that enforce the type rules |
| `model/src/ddl.rs` | `ensure_table` (create-if-absent, **verify**-if-present), `assert_fixed_point` |
| `model/tests/fixed_point.rs` | the Delta round trip, with a `UInt32` control |
| `model/tests/projections.rs` | **a fixture that breaks every invariant and every guard**, plus controls |
| `build/src/index.rs` | the TSV reader, field-count drift check, snapshot id from pins |
| `build/src/capabilities.rs` | the authored capability seed and the unbound-capability error |
| `build/src/interactions.rs` | derived-from-a-column vs authored-from-a-sentence, and the validation |
| `build/src/plan_fragments.rs` | the fragment seed, and the `preserving`-needs-a-confirmed-assertion rule |
| `build/src/catalog.rs` | index rows → canonical batches; the `Columns` builder that projects by name, and `never_fills` |
| `build/src/writer.rs` | Delta writes **scoped to one run** (`with_replace_where`), the two tables exempt from that and why, and `assert_single_writer` |
| `build/src/writer_tests.rs` | two snapshots coexisting, idempotence, the conformance guard, and the **control** that proves the predicate does work |
| `scripts/fixture.py` | a modified copy of the skill, outside every repository, so `compare` has something true to compare |
| `query/src/main.rs` | a choice of view, bound parameters, the `request` relation (now carrying `snapshot` and `compare`'s two endpoints), `catalog-md` |
| `python/codesearch_cli/render.py` | the **only** place terminal rendering happens |
| `scripts/bench.py` | whole-process latency against the 400 ms criterion |

Seeds — the **four** authored inputs, and the only ones. Each is validated against what the catalog
actually holds, so a seed naming something that does not exist is a build error rather than a
dangling row:

| Seed | Rows | What a bad row does |
|---|---|---|
| `seeds/capabilities.json` | 8 | a binding matching no mechanism fails the build |
| `seeds/interactions.json` | 6 of 32 | an interaction naming a missing mechanism or assertion fails |
| `seeds/plan_fragments.json` | 8 | `preserving` without a **confirmed** assertion fails |
| `seeds/implementation_bindings.json` | 14 | either end naming a key nothing holds fails |

---

## 3. Facts established by building — do not re-derive

### Design changes, all folded into PLAN-V2 §12.1b

1. **Lattices are `Int8` ranks, not `Utf8` labels.** Rank order and byte order differ.
2. **A fifth lattice, `observed`**, already in the skill's `regex.tsv`.
3. **Rule-field keys are scope-qualified.** 22 of 70 field names occur in more than one scope; the
   real scopes are `relation`, `ruleObject`, `ruleFile`, … — *not* `rule`/`constraints`.
4. **A regex construct in both engines is two surface entries.**
5. **A behaviour assertion's subject is a topic, not a mechanism.**
6. **`validate` reports `checked` beside `violations`.**
7. **`catalog.negative_space` is a new table.** Nothing else could say "known absent, for this
   reason, do this instead" without asserting something false.
8. **The coverage numerator is `catalog_key` + `catalog_kind`**, not `mechanism_key`.
9. **`preserving` requires a *confirmed* assertion**, not merely an existing one.
10. **Arrow extension metadata reaches the logical plan schema**, not only storage -- and it also
    survives the run-filter view, which is measured separately because putting a view under every
    projection is exactly what could have dropped it.
11. **The run filter lives at the base-table boundary.** Providers register as `<name>__all`;
    `<name>` is a view filtered to visible runs. Thirty projections, one statement of the rule.
12. **`visible_run` is evaluated once**, into a MemTable, not re-planned per reference. As a view
    it cost 59 ms of extra planning against a 400 ms budget.
13. **A rebuild of the same snapshot supersedes rather than being reaped.** The run key is
    `run:<snapshot>/<family>/0`, so `just catalog` stays idempotent.
14. **`mapping_basis` is `mir.rustc_public@runtime`** (PB15), and `built` / `analysis` are not
    reachable through either surface §3.6 named.
15. **A signature's identity includes its signature text.** `methods.tsv` records the trait PATH,
    not the trait REFERENCE, so `TryFrom<u16>` and `TryFrom<u64>` both arrive as
    `core::convert::TryFrom` — 34 owner/method/trait triples across 126 rows. Sixteen hex
    characters of blake3 over the exact signature text separate them.
16. **`catalog.surface_binding` is a subject/object relation**, not capability↔mechanism. That is
    what makes §3.8's claim — that this table is how the two halves meet — true of the table
    rather than only of the design.
17. **`IndexSpec` declares its coverage denominator.** An unhandled stem used to fall through a
    `_ => continue` and contribute nothing, silently; membership is now a stated fact per file.
18. **Registration is lazy.** A table or view is resolved the first time a query names it. §7's
    argument for eager registration is kept as a gate step rather than a per-caller cost.
19. **Every closure reports its edge count and its limit** (`projection.closure`). A closure that
    traversed nothing is otherwise indistinguishable from one that found nothing to traverse, and
    a caller reads the second meaning from the first.

20. **A canonical row carries a content digest.** `compare` joins on `entity_key`, which settles
    `added` and `removed` and leaves `changed` undecidable — `entity_id` differs between snapshots
    for an *unchanged* entity by construction. One column decides it for every table at once; the
    alternative was a comparison view per table, where a table added and forgotten is compared by
    nothing.
21. **The digest excludes by *type*, not by name.** Every column carrying
    `ARROW:extension:name = codesearch.entity_id` is a reference into a snapshot and moves with it.
    A name list would need extending by whoever adds the next family, which is the kind of
    obligation nobody discharges.
22. **`ord` is a fact about a sequence, not about a row.** Excluding it is what stops one removal
    being restated as a hundred changes.
23. **`snapshot_id` covers the index bytes; `context_id` covers the tool pins.** They used to be
    the same hash, which meant an edited skill could share a snapshot with the original — and then
    its rows collide on `entity_id` and `compare` has nothing to compare.
24. **Snapshot selection lives where run visibility lives.** Both are base-table rules and both
    have the same two exemptions, so they are stated together or they drift apart.

### API and environment facts that cost a cycle

- **delta-rs sets `schema_force_view_types = true`.** String columns arrive from a scan as
  `Utf8View` even though `assert_fixed_point` correctly reports the stored schema as `Utf8`. Both
  are true — storage versus read path — but a downcast assuming the stored type fails at runtime.
  `arrow::compute::cast` to `Utf8` accepts either. A UDF with `Signature::exact` is coerced for
  you; one with `Signature::any` is not.
- **A recursive CTE requires both terms to agree on field *metadata*.** A lattice column in the
  base term and a bare `CASE` in the recursive term do not match; `CAST(x AS TINYINT)` drops the
  metadata so they do. This is also the measurement behind fact 10.
- **A recursive CTE requires both terms to agree on column *names*.** Alias every column in both.
- **`information_schema` is off by default** and delta-rs does not enable it. `build_session` sets
  it in place via `ctx.state_ref().write().config_mut()` rather than rebuilding the context, which
  would risk losing delta-rs's `DeltaPlanner` (PB07).
- **A placeholder's type must be inferable.** `atom_truth` has six mixed-kind arguments including a
  `List<Utf8>`, so `$1` in its sixth position cannot be typed. The `request` relation — which §7.1
  specifies anyway — states the type by construction and keeps caller values out of SQL text.
- `ScalarUDFImpl` has no `as_any` and needs `DynEq + DynHash` → derive `PartialEq, Eq, Hash`.
- `WriteBuilder::with_session_state` wants `Arc<dyn datafusion::catalog::Session>`; cast explicitly.
- `arrow::array::Array` must be in scope for `.len()`, `.is_null()`, `.null_count()`.
- Serde enums default to externally tagged; `{"prefix": "…"}` needs `#[serde(untagged)]`.
- `CreateBuilder` fails if the table exists — hence `ensure_table`.
- **`Timestamp(Microsecond, Some("UTC"))` and a nested `Struct` are both fixed points** of the
  Delta conversion. `Timestamp(Nanosecond, _)` is not, and `None` for the zone is `timestamp_ntz`,
  a different type meaning "a reading on a clock somewhere".
- **`ensure_table` commits constraints separately.** A table with two CHECK constraints reaches
  `v3` on its first build: create, constraint, constraint, write. Anything watching the Delta log
  to detect a write must count *new* versions, not look for a fixed number — that cost a run of
  the crash test, which killed the build before it had written anything.
- **`ensure_table` verifies an existing table, it does not migrate one.** Changing an authored
  schema therefore fails the next build with a fixed-point mismatch until the table's directory is
  deleted. That is the guard working; the catalog is derived and `just catalog` rebuilds it.
- **`DeltaTableBuilder::load()` is the expensive half of opening a table**, and registration does
  not need it. `load()` runs `EagerSnapshot::try_new`, whose second phase replays every Add action
  into Arrow and parses statistics; the first phase alone gives protocol, metadata and schema.
  `TableProviderBuilder::build()` with only a log store does the first phase — which is the path
  delta-rs's own `DeltaTable::table_provider()` takes.
- **`SchemaProvider::table()` is `async` and demand-driven; `table_names()` is sync.** So a lazy
  schema still enumerates the whole surface for `information_schema` while resolving only what a
  query names. Override `table_type()` too, or `information_schema.tables` resolves everything —
  the trait's own docs say so.
- **`SessionContext::state_weak_ref()` hands back a `parking_lot::RwLock`**, which is not
  reentrant: planning a view from inside `table()` must clone the state and drop the guard first,
  or the first nested table reference deadlocks rather than failing.
- **Replacing the default schema discards what was registered into the old one.** `install` must
  run before `register_request`, not after.
- **A `List<Utf8>` path witness works across a recursive CTE.** `make_array` in the base term and
  `array_append` in the recursive one agree on type *and* field metadata. §7.2's `chain` uses
  string concatenation; that was a choice, not a constraint, and §7.3's required list shape was
  never blocked.
- **`array_length` returns an engine-chosen integer width.** Cast it in SQL (`CAST(... AS BIGINT)`)
  rather than guessing the Arrow type on the Rust side.
- **A recursive CTE may not reference a view that references it.** The closures read the base
  tables directly for that reason; `projection.closure` reads *them*, which is fine because the
  dependency runs one way.
- **`with_replace_where` is a `WriteBuilder` method and is SILENTLY IGNORED outside `Overwrite`
  mode** (`write/plan.rs:441-445` returns a passthrough plan, no error). That is why every test of
  it needs a control: a green result without one is equally consistent with a predicate that did
  nothing. `MergeBuilder` has no equivalent at the pinned rev.
- **delta-rs validates that every written row satisfies a `replace_where` predicate**, per row, at
  stream time (`data_validation.rs:737-759`), and treats NULL as a violation. The writer proves its
  own scope for free — but the failure fires mid-write, so orphan parquet is possible.
- **A `WriteBuilder` write always commits, even when there is nothing to do.** Unlike merge it has
  no empty-actions guard, so a re-run is idempotent in *rows* and not in *versions*.
- **Arrow's JSON writer omits a null field rather than emitting `null`.** Any renderer zipping
  values positionally against a header will shift every later cell one column left on the first row
  that has a null in the middle. `render.rows_table` takes the union of keys and looks up by name.
- **A scalar subquery under `coalesce` can plan as non-nullable and execute as null**, surfacing as
  `Column 'x' is declared as non-nullable but contains null values` from somewhere unrelated to the
  cause. `selected_snapshot` is two relations combined by `UNION ALL` for that reason, which leaves
  no gap between the declared shape and the executed one.
- **Where the 254 ms went, measured at phase 4 and again at phase 5**: session construction is
  1.2 ms and never the problem. At 20 tables and 52 views, eager registration cost 63 ms opening
  tables, 67 ms building base views and 118 ms planning projections — 250 ms of preparation for a
  query that reads four tables. Lazy resolution took whole-process p95 from ~330 ms to ~185 ms,
  and it no longer grows with the catalog, only with what a query touches.

### Harness

- **`just bench` must run on an idle machine.** A run started while `cargo test --workspace` was
  finishing reported p95 789 ms against a 400 ms criterion; the same binary measured 198 ms clean.
  A contended measurement is not a measurement of the thing.
- `#[allow(...)]` is blocked outright. Fix the cause — twice now that produced better code
  (an 8-argument function became two named structs; a duplicated composition rule was deleted).
- Ruff lints the scratchpad too: line-length 100, `ANN`, `T20`, `RUF001`.
- The `post_edit` and `pre_bash` hooks were adjusted by the operator on 2026-09-16 and no longer
  reject heredocs containing `///` or require a separate `rustfmt` call.

---

## 4. Remaining scope

The approved plan is at `~/.claude/plans/please-plan-out-the-generic-moon.md`. **Phases 0–7 are
done.** What is left is §4f, the analyzer rules — additive, and the only remaining item from the
plan. Everything else outstanding is in §4g (PB16) and §4h (deferred, with reasons).

### 4a. ✅ Phase 0 — the `rustc_public` probe. **Done.**

PB15 is in [`evidence/14-pb15-rustc-public.md`](evidence/14-pb15-rustc-public.md), the crate is at
`evidence/probes/rustc-public-probe/`, the capture is `evidence/probes/PB15-rustc-public.out.txt`.
`rustc_public` is reachable, `cfg_edge` is fillable, and **only the `runtime` phase is reachable
through either candidate surface** — one function is 14 / 8 / 6 blocks at `built` / `analysis` /
`runtime`. §3.6 and §12.3 are updated. PLAN-V2 §12.3 now has **one** open question, and it is the
`ra_ap_hir` one.

### 4b. ✅ Phase 4 — the execution model. **Done.**

`snapshot.extraction_run`, the sixth `completion` lattice, the run-row-first lifecycle, reaping,
and the visibility rule. Verified end to end by killing a build mid-write: the crashed run was left
`running`, the next build reaped it to `abandoned` while leaving a `complete` run untouched, and
its rows were absent from `discover` until `--include-partial` brought back exactly them.

### 4c. ✅ Phase 5a + Phase 5 — registration, and the program model. **Done.**

**Phase 5a, both steps.** Step A replaced `DeltaTableBuilder::load()` + `with_eager_snapshot` with
the log-store-only path: p95 322 → ~255 ms at 14 tables. Step B made registration lazy
(`bridge/src/lazy.rs`): p95 ~330 → **~185 ms at 20 tables and 52 views**, better than the 218 ms
recorded back at 13 tables. The cost no longer scales with the catalog.

`just codesearch-check` carries §7's "every projection plans" guarantee now, via
`projection::check_all` and the `information_schema` test. That is the trade, stated plainly: the
guarantee runs once per gate instead of once per caller.

**Recorded as available, not adopted:** delta-rs `Snapshot`/`EagerSnapshot` implement `Serialize`
and `Deserialize` (`kernel/snapshot/serde.rs`), with an upstream test proving deserialise plus
incremental `update()` avoids log replay. A cross-process warm cache is possible and was not built
— it adds a cache-invalidation surface to a tool whose isolation argument is the process boundary.
It is the cheaper cousin of §8's resident-server fallback and should be tried before that fallback.

**Phase 5** built the `library-api` family: 23 packages, 716 definitions, 4,727 signatures, 2,241
implementations, 273 export paths, 5 unbound native handles. Two families now coexist, each
bracketing its own tables with its own run row.

Four things it found that were not in the plan:

- **`methods.tsv` records the trait path, not the trait reference.** 34 owner/method/trait triples
  collide across 126 rows, all generic `From`/`TryFrom`/`PartialEq`. The signature text is what
  separates them, so it is in the key as a digest.
- **`pins.crates` is a `name -> version` object**, so `pkg:` keys have no source segment.
- **`unresolved.tsv` has one column** holding `A -> B`; `mapping_status` is assigned, not read.
- **`symbols.tsv.family` is an editorial bucket**, not a rustdoc fact, so it is read as
  `index_bucket` and goes nowhere near `program.definition`.

**The catalog↔program join is real**: `catalog.surface_binding` is now subject/object, and 14
authored bindings cross into `program.*`. `coverage --dimension api` reports 7 of 323 mechanisms
bound — small, and the honest number. A derived alternative (crate-name similarity) was rejected in
the plan for asserting a relationship nothing measured.

### 4d. ✅ Phase 6 — the two closure queries (§7.3). **Done.**

`projection.containment`, `projection.re_export_chain`, and `projection.closure`.

**The measurement that shaped them:** containment has **0 edges** over 716 nodes, because
`symbols.tsv` indexes items and every item's owner is a module. Re-export has 273 edges, **all
exactly one hop** — no `canonical_path` is itself an `access_path`.

So `projection.closure` exists: node count, edge count, deepest chain and the reason each stops.
It is `validate`'s `checked` column applied to reachability, and it earns its place on day one —
an empty `containment` that a caller reads as "this definition contains nothing" would be a
confidently wrong answer about 716 rows.

Two facts worth keeping:

- **The `List<Utf8>` path witness works in a recursive CTE.** `make_array` in the base term and
  `array_append` in the recursive one agree on type *and* field metadata. §7.2's `chain` uses
  string concatenation; that was a choice, not a constraint.
- **Bounded termination is tested where it can fail.** Both real graphs are acyclic and one is
  edgeless, so the fixture builds a two-node cycle and asserts 4 / 8 / 16 rows at `--max-depth`
  2 / 4 / 8. A test over the real data would have proved nothing.

The other four of the six stay unbuilt and are said to be: call-graph and CFG reachability need the
`hir` and `mir` families; dependency closure needs `cargo-metadata`'s `resolve_present` guard;
derivation provenance needs pass F.

### 4e. ✅ Phase 7 — `compare`, and the second snapshot. **Done.**

Two snapshots coexist in one catalog. The projection was the small half.

**The API the plan named does not exist.** PLAN-V2 §12.3 asked whether
`MergeBuilder::with_replace_where` commits in one transaction. `MergeBuilder` has no such method at
the pinned rev — `with_replace_where` is `WriteBuilder`'s (`write/mod.rs:233`). Evidence 05 §2 had
that right in round 1; the mis-attribution entered when PLAN-V2 and this file summarised it.
**A summary of a primary source is not a primary source**, and that is the transferable lesson.

So the write is one builder call: `SaveMode::Overwrite` plus
`with_replace_where("run_id = '<this run>'")`. A run id is `run:<snapshot>/<family>/0`, so one
predicate over one existing column carries both halves of the scope. The documented
`family = 'F' AND snapshot_id = 'S'` was unwritable — `family` is a column of
`snapshot.extraction_run` and of no canonical table.

**PB17** (evidence 15) measured the rest against a table shaped like a real one — CHECK constraint,
extension metadata, `Utf8` strings — because the scoped path unions the caller's batch with a scan
of the existing files, and that scan returns `Utf8View`. Four findings, all now load-bearing:
a scoped write preserves other runs (**and the control destroys them**); every written row must
satisfy the predicate or delta-rs rejects the batch; it is always one commit, and a no-op still
advances a version; a failure leaves orphan parquet that only `VacuumMode::Full` reclaims.

**The larger half was nobody's plan.** No view filtered on `snapshot_id` — all 37 silently unioned
every snapshot, so a second one would have returned two rows per `describe` and doubled every
coverage count, with nothing raising an error. The filter now sits beside the run filter in
`VISIBLE_RUN`'s neighbourhood: `newest_snapshot` and `selected_snapshot` resolve the request, and
`install` adds one clause to every base view. `runs` prints which snapshot a request resolved to,
because a catalog holding two and answering about one has to say which.

**Three things the fixture caught that reasoning had not.**

1. **The digest must exclude every column *typed* `codesearch.entity_id`, not just the row's own.**
   The first run reported all 165 `catalog.parameter` rows as changed when three things had
   changed: `mechanism_id` is an entity reference and moves with the snapshot. The rule is by
   extension type, so the next family inherits it without anyone remembering.
2. **`ord` is not content.** Removing one flag shifted every later sibling's position and restated
   one removal as a hundred changes. A position describes the sequence, not the element — and
   `interaction_atom`, the one table whose order is semantic, encodes it twice over.
3. **The Python renderer read cells positionally.** Arrow's JSON writer omits a null field rather
   than emitting `null`, so an added entity — which has no `before_id` — rendered its `after_id`
   one column left. `rows_table` now takes the union of keys and looks up by name.

**Green, measured:** `just compare-fixture` reports exactly seven rows — three added, three
removed, one changed — for three mutations, because one flag is a mechanism *and* a binding *and*
a surface entry. Comparing a snapshot with itself reports nothing. The reverse direction mirrors.
`describe` returns one row, and `validate` and `coverage` are unchanged at 621 / 319.

**`ENTITY_KEY` became optional**, a deliberate extension of §11: "these differences and nothing
else" is not a claim a single-key query can make, and it is the acceptance criterion.

### 4f. ✅ M1 — the enforcement gaps. **Done.**

Four pieces, none needing new data. §10.2's table of mechanical checks goes from six of twelve to
nine of twelve.

**`IdentityDiscipline` is built** (`bridge/src/rules.rs`) and registered through
`SessionContext::add_analyzer_rule`, which takes `&self` and so attaches to delta-rs's session
without a rebuild — PB07's reason for never rebuilding the context. It walks the plan, resolves
each join's operands against the two inputs merged, and refuses `entity_key ⋈ entity_id`.

**One thing it had to learn the hard way:** an analyzer rule runs *before* the optimizer, so a SQL
`JOIN ... ON a = b` arrives with the equality in `join.filter` — `ExtractEquijoinPredicate` is an
optimizer rule and moves it to `join.on` later. Reading only `on` made the rule fire on nothing
that came from SQL, and the first version of the test passed because `ctx.sql()` does not analyse
at all. The test now calls `into_optimized_plan()`, which is what `collect()` does in the binary.

**`LatticePropagation` is a test, not a rule**, as §12.1 already decided and this is where it
landed. It reads the *planned tree*: an aggregate whose argument is directly a lattice-typed column
must be `min` or `max`. Reading SQL text instead produced two false positives — a recursive CTE's
column list scans as a call to `chain`, and `sum(CASE WHEN observed_rank >= … END)` scans as `sum`
over a rank when it counts a boolean.

**The ast-grep tier exists**, and `queries/rules/` is no longer empty: three rules, three fixture
sets, and `just lint-rules` in the gate. Each rule was planted into real source and observed to
fail the gate.

**Two things that tier taught, both worth more than the rules:**

- **`ast-grep` exits 0 on failure.** On `0 passed; 4 failed`, and on a rule file that does not
  parse. Both were observed while writing this corpus. `scripts/lint_rules.py` parses counts and
  compares the rules tested against the rule files on disk, because a corpus that stopped being
  discovered is indistinguishable from a clean one.
- **A structural rule cannot see inside a macro.** `vec![...]` is a token tree, so a
  `no-uint32-byte-offset` rule matched the three `Field::new` calls written as plain expressions
  and none of the two hundred inside `table(vec![...])` — which is every schema here. It passed its
  own fixtures and caught nothing real. **It was deleted**; the invariant lives where it works, in
  `schema::tests::no_authored_column_uses_a_type_delta_would_narrow`, which sees the built schema
  rather than the syntax. The scan also needs explicit paths: ast-grep takes the config's own
  directory as the project root, so a path-less scan walks `queries/` and reports zero.

**A fourth UDF, and a correction to §10.1's inventory.** `entity_key_namespace` is built with the
`preimage` §10.1 claims, and **PB18** (evidence 16) measured that the claim is true: a `Utf8`
`Interval` is constructible, the rewrite fires, and it reaches the same predicate a person would
have written — with a control, identical but for returning `PreimageResult::None`, that does not.
Its boundary test is the first thing §10.2's `preimage` checks have ever had to check.

The other three "buildable now" UDFs turned out **not to be functions at all**.
`regex_engine_supports` would compile `regex.tsv`'s 44-row matrix into the binary where a skill
update could silently disagree with it; `supports_relationship` is what `projection.discover`
joins; `coverage_denominator` *is* `projection.coverage_surface`. Three of the twelve are relations
wearing a function's clothes — the same error as §5.3's analyzer rule — and that is recorded in
§12.1b rather than fixed by writing three functions that each duplicate a table.

**Also closed:** `bindings.tsv`'s undocumented exclusion (67 rows, a name-to-name map whose `role`
is uniformly `item`, so it supplies neither a `mapping_basis` nor a binding rank), and
`snapshot.source_anchor`'s declared-and-empty status, which is now stated as the one deliberate
exception to "declare a table when a family writes it".

### 4g. ✅ M4 (part) — the producer seam, proven. **Done.**

The question this answered was not "can an adapter run" but **"how do its rows enter the code"** —
and the honest answer at the time was that nobody had tried, so the design said "a producer is a
skill-shaped directory" without anything behind it.

**All 23 pinned crates are now local.** Eleven were missing — the whole `grep-*` family, `pcre2`,
and `regex`/`regex-automata`/`ignore` at other versions. They are pinned by
`producers/cargo-metadata-adapter/Cargo.lock`.

**And the pins are correct, which was not obvious.** Six of 23 look one patch behind crates.io, but
`acquire.py:14` resolves ripgrep's library crates from **ripgrep 15.2.0's own `Cargo.lock`**: *"the
version of `ignore` that ripgrep 15.2.0 actually links is the one whose behaviour the probes
observe; a newer `ignore` on crates.io would describe a tool nobody here is running."* All three
pinned tool releases — ripgrep 15.2.0, ast-grep 0.45.3, pcre2-10.48 — are the current ones, so the
skill is already at latest in the only sense that matters.

**The seam is the filesystem.** `producers/cargo-metadata-adapter` is its own workspace with its own
pin (`cargo_metadata = "=0.23.1"`, the version `rust-code-model` records and crates.io's latest).
It writes headerless TSVs plus a `PROVENANCE.json` into `$CODESEARCH_HOME/producers/…`, and
`codesearch-build` reads that through `index.rs` — no second reader, no new dependency on a binary
that resolves twelve and would have resolved 223 more for `ra_ap_*` alone.

**Four things had to change, and each was a real break, not a formality:**

1. **`read_all` hard-errors on a missing `SPECS` file**, so a producer root failed on `flags.tsv`.
   `IndexSpec` now carries a [`Source`], and `read_all(root, source)` reads only that root's specs.
2. **Stems collide.** `rust-code-model` ships five filenames ast-grep-ripgrep also uses, so one map
   keyed by stem would merge two subjects. The maps are per root.
3. **`snapshot_id_from_inputs` digested one root's bytes.** It takes every root now — an identity
   that does not cover what was read is a lie, and `entity_id` collisions are what it costs.
4. **Drift errors named a file but no root.** `FieldCount` and `RowCount` carry the flag to pass.

**And one the adapter had to fix in itself:** `src_path` came back as an absolute path under
`$HOME`. The snapshot id digests those bytes, so the same inputs would have had a different
identity on every machine. It emits paths relative to the crate root.

**§7.3's third closure is built, and it refuses to traverse.** `cargo metadata --no-deps` sets
`resolve` to null, and PL001 measured that a null resolve is not "no dependencies" — so every edge
carries `resolve_present`, and `projection.dependency_closure` checks it *in the view, not in the
caller*, exactly as §7.3 specifies. It reports 23 nodes, 0 edges, and why. Sixty-one of the 172
declared edges do name a crate the catalog holds, which is what a resolving family would work with.

**Measured:** three families, three run rows, 22,641 rows checked at 0 violations, `compare-fixture`
still exactly seven rows, p95 259 ms.

What M4 still owes: `program.crate_unit` and `program.feature_declaration` (130 rows are sitting in
the adapter's reach), and re-sourcing `program.package` from the adapter so `pkg:` regains the
source segment §3.1 specifies.

### 4h. ✅ M4b, M2, M3 — the adapter finished, the last probe, and §11's CLI. **Done.**

**M4b — the `cargo-metadata` family, complete.** Four tables, and the `pkg:` degradation retired.

The obvious route did not work, and the reason is worth keeping: running `cargo metadata --no-deps`
over vendored checkouts reports a registry crate as a **path** package with `source: null`, because
that is what it is on disk. Minting `pkg:path/ignore@0.4.29` would have recorded how the adapter
read the crate rather than what the crate is — the same error as the absolute `src_path`.

What recovers it is a **resolve**. `producers/cargo-metadata-adapter/subject/Cargo.toml` pins all
23 with `=`, its lockfile is committed, and the adapter asserts the resolve against the skill's own
`pins.crates` before emitting anything. That yields `pkg:crates.io/<name>@<version>` — §1.2 in full
— and `resolve_present = true`, so **§7.3's dependency closure traverses 369 edges over 155
packages at depth 8**, where it had 0.

Two corrections came with it. `program.target` was renamed **`program.crate_unit`**: §1.2's grammar
has a `crate:` namespace and no `target:`, so the first pass had invented a `#target:` key that
nothing checked — because nothing checks key shapes. And the seven authored `pkg:` bindings were
migrated; a binding naming a key nothing holds is a build error, so the migration could not be
half-done.

**M2 — PB16, and the probe backlog is empty.** Evidence 17. `enabled` is distinguishable;
`disabled` and `server_failed` are **not**, from what `load_workspace_at` returns — both are
`client = None`. The distinction is made at `load-cargo-lib.rs:111-134` and discarded at line 204
by `and_then(Result::ok)`, surviving only in tracing and in the **per-crate** `proc_macros` map.

So §3.2's value is reachable, and §3.2 needs two changes before the `project-load` family is built:
read the per-crate `ProcMacroLoadResult`, and treat `proc_macro_policy` as per crate rather than
per load. A family that recorded the returned `Option` would collapse two states evidence 06 says
must not be collapsed.

**The control failed first, in PB07's exact way** — all three arms returned `client=None`, so
nothing was measured. Not a missing server: every toolchain here ships one, but
`CargoConfig::default()` discovers no sysroot. Pointing both the failing and the working arm at
explicit paths holds the mechanism fixed and varies only the path.

**M3 — §11's CLI surface.** `discover` gained its six retrieval facets (`--subject`,
`--relationship`, `--result`, `--semantic-layer`, `--scope`, `--language`) and `--budget`, whose
`omitted` count is a window total **in the same query** so the two cannot disagree. `describe` is
variadic with `--field`; `validate` takes `--invariant` and still prints `checked`, so a narrowed
run cannot look clean by inspecting nothing; `coverage` takes `--tool`.

**`prune` exists**, on the *build* binary because it writes and §8 says the query side opens
read-only. PB17's mechanism: `Overwrite` plus `replace_where` with a zero-row input. Measured
removing 12,112 rows across 22 tables and leaving one complete snapshot at 0 violations.

**One `--language` bug worth remembering.** Written as a bare `tool`, the correlated reference
resolves *inside* the subquery to `sd.tool` — `sd.tool = sd.tool`, always true — so
`--language rust` returned pcre2 mechanisms. A wrong answer, not an error. It is qualified now and
pinned by a test that asserts on the correlation itself.

**Not done, and why:** `describe --stage` is §11's one remaining flag. It selects among §4.5's five
type-observation stages, which live in `program.type_observation` — a table pass D and E write and
nothing writes yet. Adding the flag now would mean a selector over one stage, which is not a
choice.

### 4i. M5 — the `syntax` adapter. **START HERE.**

§3.3 is the only family that works on code that does not compile (probe SY001, with a control:
broken source yields `ERROR` nodes, valid source contains none), and `ra_ap_syntax` parses a string
with no database and no network. All 23 crates are local now, so it covers the whole subject.

It fills **`snapshot.source_anchor`** — declared, constrained and empty since phase 2 — and with it
`anchor_contains` / `anchor_overlaps` (the last two `preimage`-bearing UDFs), the `impl:` key's
anchor segment (retiring the second key degradation and letting `program.implementation` rise above
`inexact`), and `program.syntax_node` / `token` / `attribute_occurrence` / `literal_occurrence`.

**Trivia survives only here** (SY002): `COMMENT` nodes exist at this layer and nowhere above.

The full remaining programme is in `~/.claude/plans/please-plan-out-the-generic-moon.md`.

### 4i. Probes whose crates are now on disk

Fetched and **verified to compile offline**, so none of these is blocked on the network any more:

| Crate set | Version | For |
|---|---|---|
| `ra_ap_*` (14 direct, 223 in the graph) | `=0.0.352` | PB16 — §12.3's **last** open question, now that PB17 closed the other |
| `cargo_metadata` | `=0.23.1` | the live `cargo-metadata` family (§3.1) |
| `rustdoc-types` | `=0.61.0` | the live `rustdoc-json` family (§3.5) |

`evidence/probes/ra-ap-hir-probe/` holds a stub `main.rs` and the real `Cargo.toml`; **PB16 itself
is not written.** The probe is: does `ra_ap_hir` report a proc-macro server failure, or does it
silently return incomplete resolutions? §3.2 reserves `proc_macro_policy='server_failed'` for the
first; if the second is what happens, the systematically-incomplete case is one the model cannot
currently express. The control is the same workspace loaded with a *working* server.

**One dependency-graph fact, already paid for:** `ra_ap_*` 0.0.352 does not resolve to a buildable
graph on its own. `ra-ap-rustc_lexer` asserts at compile time that `unicode-ident` and
`unicode-properties` share a Unicode version, and a fresh resolution pairs Unicode 18 with
Unicode 17. `unicode-ident = "=1.0.24"` is pinned in the probe's manifest to force the pair, with
the reason written there.

### 4j. Deferred, with the reason stated

| Item | Why |
|---|---|
| `hir` family | the crates are here now; the *probe* (PB16) still has to be written and answered first |
| live `cargo-metadata`, `rustdoc-json`, `mir` families | operator chose shipped indexes first; revisit after phase 5 |
| `dataflow`, `evidence.derivation`, `explain`, CDF re-derivation | all downstream of pass F, which is downstream of E |
| ingesting `rust-code-model` as a second skill | operator declined; transferability stays untested |
| `catalog.parameter`'s three defaults | the index records none; filling `effective_default` is §4.3's `grep-pcre2` question and needs a probe |
| `grammar_fragment` | `kinds.tsv` (3,024) and `fields.tsv` (631) are referenced-not-loaded per §3.8 |
| a `-Zdump-mir` adapter | the only way to reach `mir_phase` `built` or `analysis` (PB15). Not needed until a `mir` family exists |

---

## 5. Context that is easy to lose

**The tool must work against any copy of the skill.** `--skill-root` is required and never
inferred; tests locate the skill via `CARGO_MANIFEST_DIR`, never a host path.

**Four seeds are the only authored inputs.** Everything else derives from the skill's indexes.
The discipline is the same in all four: a capability binding that matches nothing, an interaction
naming a mechanism the catalog does not hold, a fragment citing an assertion that does not exist, a
`preserving` claim with no confirmed probe behind it, or an implementation binding whose subject or
object resolves to nothing is a **build error**.

**Derive a column, author a sentence.** 26 of 32 interactions come from `regex.tsv.reachable`
because that is machine-readable; the other 6 come from prose in `unreachable.tsv` and the topic
pages, and live in a seed where they can be reviewed as judgements.

**A write replaces one run's rows, not a whole table.** `with_replace_where("run_id = '…'")`, and
delta-rs rejects a batch whose rows do not all satisfy it — so the writer proves its own scope.
`catalog.lattice` and `snapshot.extraction_run` are exempt and each says why.
`assert_single_writer` still runs, guarding something narrower now: no table is *meant* to have two
writers, and one appearing by accident would silently change what every projection over it
returns.

**Both families run in ONE process.** `assert_single_writer` is a within-call duplicate check and
cannot see across processes, and `runs::merged` does a read-modify-write of `extraction_run` that
two racing builds would corrupt. `library-api` also builds *first*, because the authored bindings
in `catalog.surface_binding` are validated against `program.*` keys that must already exist —
validating against keys the build merely intends to mint would check nothing.

**Run visibility is about the write; `retracted` is about the row.** They are different facts and
both are kept. A retracted row was withdrawn; a hidden row was written by a run that did not
finish. `extraction_run` and `lattice` are the only tables exempt from the run filter, and
`only_the_two_declared_tables_are_exempt_from_the_run_filter` is what stops a third being added
quietly.

**An absence should be explicable from inside the catalog.** That is why `extraction_run` is
never itself filtered by visibility: `codesearch-query runs` is the answer to "why is this
empty?", and hiding the abandoned run would hide the answer along with the rows.

**Coverage reports three numbers and never a composite.** The CLI prints three tables; the JSON form
keys them separately so nothing invites a consumer to sum them.

**`validate` reports on the selected snapshot, not on the catalog.** It is a projection like any
other, so it inherits the snapshot filter — `checked` came to 22,195 with one snapshot in the
catalog and 22,195 with two, which is the right answer and not an obvious one. Validating a
different snapshot is `--snapshot <id> validate`, and both of the current pair report 0 violations
over 22,195 rows. Anything that wanted to assert "every snapshot is clean" has to iterate; nothing
does yet, and that is a gap rather than a decision.

**A query answers about one snapshot, and says which.** The default is the newest with a visible
run; `--snapshot` pins another. `runs` marks the resolved one, because a catalog holding two and
answering about one has to say which — the same argument that keeps `extraction_run` out of every
filter. `compare` is the single deliberate exception, and it reads `entity_all`, which is
run-filtered and *not* snapshot-filtered.

**`compare` reports differences, never a verdict.** Three kinds — `added`, `removed`, `changed` —
taken from `crates/enrichment-core/src/compare.rs`, whose own first line is "Deterministic
differences between immutable observations, not a compatibility proof." No severity, no score, for
the same reason coverage has no total.

**A check that cannot fail proves nothing.** `model/tests/projections.rs` plants a violation for
every invariant and every guard, and its `batch` helper takes the **last** entry for a column so a
test appends the one thing it is breaking. First-wins silently ignored four overrides and made
four tests pass against unmodified data.

**An empty answer needs a denominator too.** `projection.closure` reports each closure's edges and
what limits it, because a reachability query that traversed nothing is indistinguishable from one
that found nothing to traverse — and a caller reads the second meaning from the first.

**Print the denominator.** `checked` has now caught two things `0 violations` would have hidden:
an invariant scoped to rows that do not exist, and three tables whose 73 rows were outside every
identity invariant while `validate` reported nine clean checks.

**One rule, one statement.** `plan_fragments.rs` briefly carried a Rust mirror of the composition
join in `projection.fragment_edge`. It was deleted rather than kept: the SQL is what decides what
a caller sees, so the tests for the rule belong against the view.

**State lives outside the repository.** `just where` prints the paths. Build artifacts are ~7 GB
warm; the catalog is 2 MB. A cold build is ~10 minutes, and the probe crate at
`evidence/probes/delta-arrow-probe/` shares the dependency set, so warming one warms the other.
