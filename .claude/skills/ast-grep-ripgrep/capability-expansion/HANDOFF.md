# Handoff — code-search capability catalog, round 1 complete, probe backlog closed

Written 2026-09-16; **updated the same day after PB05–PB12 ran.** Read this first; it exists so
the next session does not re-derive what is already settled or re-make the mistakes already
recorded.

**Status: both rounds are complete.** All twelve gating probes executed, and round 2's integrated
design is [`PLAN-V2.md`](PLAN-V2.md) — **the document to implement from.** [`PLAN.md`](PLAN.md)
remains the per-subsystem capability record; where the two disagree, v2 wins.

**Implementation: v2 §12.1 steps 1–5 are done** — the implementation handoff is
[`IMPLEMENTATION.md`](IMPLEMENTATION.md); read that before touching the code.

The code is in [`codesearch/`](codesearch/): the catalog builds end to end from the shipped
indexes and is queryable through a Typer/Rich CLI. **86 tests green** (41 Rust, 45 Python).
`just codesearch-check` is the gate; `just where` shows where state lives.

```
654 index rows  ->  8 capabilities · 323 mechanisms · 305 bindings
                    605 surface entries · 49 behaviour assertions
p95 48.8 ms for `describe` -- inside §8's 400 ms criterion, so one-shot serving stands
```

**Still unbuilt, deliberately:** the `LatticePropagation` and `IdentityDiscipline` analyzer rules
(§5.3, §1.4), and projections as registered `ViewTable`s rather than SQL composed in the query
binary (§7.4). Neither changes any result today; both are about making a guarantee structural
rather than conventional. See "what is deferred and why" in
[`codesearch/README.md`](codesearch/README.md).

Two things found by executing the design, both already folded back into PLAN-V2:

1. **§5's lattice encoding was wrong** — `Utf8` labels cannot reliably prune, because rank order
   and byte order differ. Amended to store `Int8` ranks, which removed three UDFs and made the
   fold plain SQL. Confirmed by a real Delta round trip, not by reading the kernel source.
2. **A fifth lattice exists** — `observed` (`not-probed → unknown → recorded → confirmed`), which
   the skill's own `regex.tsv` already carries. It needed no new machinery.

---

## 1. What this is

An elaboration of [`../Proposal_machine-queryable_code-search_ast-grep_ripgrep_pcre2.md`](../Proposal_machine-queryable_code-search_ast-grep_ripgrep_pcre2.md)
(1,802 lines) into an implementable plan. The operator framed it as **two rounds**:

- **Round 1 (DONE)** — library feature discovery in depth, per subsystem. Find out what
  DataFusion / Arrow / Delta / Typer / Rich / the seven rust-code-model families can actually do,
  so round 2 is not spent discovering it.
- **Round 2 (DONE)** — **design integration**, in [`PLAN-V2.md`](PLAN-V2.md). Cross-subsystem
  naming, the staging→canonical contracts, execution ordering, and the parts round 1 deliberately
  deferred (§4 below). Steps 1–5 of its build sequence are implemented.

Deliverables: [`PLAN.md`](PLAN.md) (round 1) + [`PLAN-V2.md`](PLAN-V2.md) (round 2, the design to
implement from) + [`IMPLEMENTATION.md`](IMPLEMENTATION.md) + [`evidence/`](evidence/) (13 dossiers)
+ [`evidence/probes/`](evidence/probes/) (**14 executed probes** with captures) +
[`codesearch/`](codesearch/) (the code).

---

## 2. Operator decisions — settled, do not re-litigate

| Decision | Value |
|---|---|
| Language split | **Rust core, Python Typer/Rich CLI shelling out.** Chosen because the `datafusion`/`deltalake` skills index the Rust crates. |
| Location | this directory, beside the proposal (touches nothing `verify.py` digests) |
| Sources | **only** the pinned skills — `datafusion`, `deltalake`, `rust-code-model`, `typer-rich`, `ast-grep-ripgrep` — plus executed probes. **No Context7, no web search.** The operator was explicit. |
| Round-1 priority | depth per subsystem over integration |

Pins (all verified aligned with this workspace, evidence 09 P02):
`datafusion =55.1.0` · `arrow 59.3.0` + `canonical_extension_types` ·
**`deltalake 1.0.0` at git rev `58f07cd62bfbce3649a7e1c87c696288068ae184`** (NOT crates.io) ·
`object_store 0.13.2` · `sqlparser 0.62.0` · `rich 15.0.0` · `typer 0.27.2` · rustc 1.98.1.

---

## 3. Facts established by execution — never re-derive these

Fourteen probes ran. Six overturned something written from reading indexes alone.

**The three silent-failure modes are the most valuable findings in this directory.** Each produces
a wrong answer with no error, no warning and an unchanged-looking plan:

1. **A `preimage` returning a closed upper bound drops rows.** `Interval` is nominally closed;
   the rewrite consumes it as half-open. Measured: 99 rows where the answer is 100.
2. **A `Volatile` UDF loses its filter entirely** — not merely un-rewritten; the predicate never
   reaches the scan, and nothing says so.
3. **A session handed to a delta-rs builder can be discarded with a log line.**
   `SessionFallbackPolicy::InternalDefaults` is the default.

A fourth is loud but fatal: **a Delta CHECK constraint naming another table panics the process**
(`get_table_source` is `unimplemented!()`), so constraint text must never be reachable from input.

### P01 — Rich silently deletes bracketed text (decisive)
11 of 24 forms altered. **`[a-z]`, `[a-zA-Z_]`, `[x]` are deleted**, as are all six ast-grep rule
field names `[pattern] [kind] [inside] [has] [stopBy] [regex]`. `[A-Z]`, `[0-9]`, `[^a-z]`,
`[[:alpha:]]` survive — one character flips it. Both remedies clean: `escape()` rescued all 11,
`markup=False` preserved all 24. → CLI rule is a **lint**, not a convention; `--json` never
touches a Console.

### PB01 — Arrow extension metadata **survives** Delta round trip
All four metadata keys returned byte-for-byte, control included. Extension identity is a
**storage property**. The storage/memory re-attachment split I originally designed was
unnecessary — already removed from the plan.

### PB03 — Delta **does not reject; it silently narrows** (biggest correction)
My evidence 04 §1 rejection list was derived from `PrimitiveType` (the conversion's *target*
vocabulary) and was wrong. Measured: 10 of 11 "absent" types are accepted and mapped —
`UInt32→Int32`, `UInt64→Int64`, `FixedSizeBinary(32)→Binary`, `Dictionary→Utf8`,
`Utf8View→Utf8`, `LargeUtf8→Utf8`, `LargeList→List`. Only `Float16` is rejected.
- **Byte offsets must be `Int64`.** `UInt32` silently truncates above `i32::MAX` (>2 GiB files).
- Nested child fields are **renamed** to Parquet conventions (`item`→`element`,
  `entries/keys/values`→`key_value/key/value`).
- **`normalize_for_delta` is not a preview of persistence** — it returned all 20 inputs unchanged,
  including every type the write path then narrows.
- Schema rule: **every column type must be a fixed point of the Delta conversion.**

### PB02 — the provider wrapper is justified, on `constraints()` alone
Declaring `Constraint::PrimaryKey([0])` propagates as a functional dependence over all columns
and re-plans two query shapes: `SELECT DISTINCT key` loses its `Aggregate` entirely; a self-join
on the key becomes a `LeftSemi Join`. **`statistics()` must be dropped from the wrapper** —
delta-rs already supplies accurate statistics at the `ExecutionPlan` level (`DeltaScanExec,
statistics=[Rows=Exact(n), …]`) regardless of what the provider returns.

### PB04 — `deltalake::DeltaOps` does **not** exist at this rev
Both `deltalake::DeltaOps` and `deltalake::operations::DeltaOps` → E0432. The individual
operation builders are the only entry point. The plan already assumed this; no change needed.

### API facts that cost compile cycles
- `try_from_arrow` is a **trait** method → `use deltalake::kernel::engine::arrow_conversion::TryFromArrow;`
- `TableProvider` in DF 55.1 has **no `as_any`**. Neither does **`ScalarUDFImpl`**.
- `ScalarUDFImpl` requires `DynEq + DynHash` → derive `PartialEq, Eq, Hash` on the impl struct.
- `ExecutionPlan::partition_statistics` is **deprecated**; use
  `StatisticsContext::compute(plan, &StatisticsArgs::new())`. `StatisticsContext` is `!Send`
  (`Rc<RefCell<_>>`) — confine each use to a block with no `await`.
- **`DeltaTable` has a 20-method fluent operation surface** (`merge`, `write`, `add_constraint`,
  `scan_cdf`, `optimize`, …). This refines PB04: `DeltaOps` does not exist, but "the individual
  builders are the only entry point" was also wrong — `DeltaTable::<op>()` is the idiomatic seam.
- **Delta writes need delta-rs's own session.** `SessionContext::new()` fails with *"No installed
  planner … MetricObserver"*; use `create_session().into_inner()`, register UDFs, then `.state()`.
- `CdfLoadBuilder::new` is `pub(crate)` → `DeltaCdfTableProvider::try_new(table.scan_cdf())`.
- `table.get_file_uris()?.count()` for a file count; there is no `get_files_count`.
- `DeltaTable` is **not** a `TableProvider`; use `deltalake::delta_datafusion::TableProviderBuilder`
  → `.with_log_store(..).with_eager_snapshot(..).build().await`.
- `DeltaTableBuilder::from_url(Url)`, not `from_uri`.
- `StructField::with_metadata` **replaces**, does not extend. Pass all entries in one call.
- `EagerSnapshot::arrow_schema()` is infallible; reach it via
  `table.snapshot()?.snapshot().arrow_schema()`.
- DataFusion does **not** re-export `async_trait`; add `async-trait = "0.1"` directly.
- `Constraint::PrimaryKey(Vec<usize>)` / `Unique(Vec<usize>)` — column indices.
- `displayable(plan).set_show_statistics(true)` is required programmatically;
  `datafusion.explain.show_statistics` only affects `EXPLAIN` text.

---

## 4. Remaining scope

### 4a. ~~Round 2 proper — design integration~~ — **DONE**, see [`PLAN-V2.md`](PLAN-V2.md)

All seven deferred items are decided. Five of them collapsed into one mechanism: `precision`,
interaction truth, prefilter recall and native-handle binding are all **ordered lattices**, so
composition is `min`, alternation is `max`, and every fold is SQL (v2 §5). Identity (v2 §1) and
serving (v2 §8) were the two genuinely separate decisions.

The original list, for the record:
1. **Cross-subsystem naming** — one identity vocabulary shared by `catalog.mechanism_id`,
   `snapshot.entity_id` and the seven families' native handles.
2. **Staging→canonical schema contract per family.** PLAN §6.1 gives the rules; the column lists
   are unwritten. This is the largest single piece.
3. **Execution ordering and failure semantics** across the six merge passes — what a partial
   failure leaves behind.
4. **The `Interaction` model** (proposal §3). Round 1 established only that interactions are
   ordinary rows with `Precision` + evidence, not a special structure. Needs its own design pass.
5. **`PlanFragment` composition** (proposal §10) — typed input/output units.
6. **Serving** — is the Rust core one-shot or a resident session?

### 4b. Probe backlog — **CLOSED**

All twelve ran. Results in [evidence 10](evidence/10-pb01-pb03-executed-results.md),
[11](evidence/11-pb02-provider-wrapper-results.md) and
[12](evidence/12-pb05-pb12-executed-results.md). Summary of the eight settled here:

| Probe | Verdict |
|---|---|
| **PB05** | **YES** — session UDFs reach both join and match-clause merge predicates |
| **PB06** | **YES, and it prunes** — four files to one. Three conditions, two silent (see §3) |
| **PB07** | UDF constraint **locks out every future writer**; a table reference **panics** |
| **PB08** | `Syntax` does **not** markup-parse (safe); **no regex lexer exists** in 602 pygments lexers |
| **PB09** | **NO** — outer bounds are not pushed in; a cyclic graph never terminates |
| **PB10** | accepts every filter as `Exact`; narrows **only** on partition columns |
| **PB11** | **resolved** — evidence 11 read a deprecated channel |
| **PB12** | **no** — constraints are SQL text; `Constraints` has no slot for a CHECK |

The two questions PB02 left open are both answered. `partition_statistics` is **deprecated**
(`StatisticsContext::compute` is the replacement, and it agrees with the rendered plan text);
`Constraints` cannot be derived from Delta's own constraints.

Round 2 added two more and closed the last two answerable questions
([evidence 13](evidence/13-pb13-pb14-closing-open-items.md)):

| Probe | Verdict |
|---|---|
| **PB13** | write and constraint are **always two commits**; a failed constraint-add is non-destructive, so write-first commits the violating row *and* records no constraint — DDL-first is the measured-correct order |
| **PB14** | `is_distinct` tames a cycle **only when the projection excludes `depth`**; the deduplicator runs over the full tuple, so the two mitigations are mutually exclusive |

**Fourteen probes, backlog empty.** The two questions that survive need a real workspace, not a
fixture: whether `rustc_public` is a usable MIR surface at the pinned nightly, and how `ra_ap_hir`
behaves when proc macros fail to build. Both belong to implementation step 6 (PLAN-V2 §12.3).

> **Superseded, 2026-09-16.** Half of that is wrong. **PB15** ([evidence 14](evidence/14-pb15-rustc-public.md))
> answered the `rustc_public` question offline against a three-function fixture: the crate ships
> in the `rustc-dev` rustup component, which was already installed here. `mapping_basis` is now
> decided as `mir.rustc_public@runtime`, and the probe also narrowed §3.6 — neither candidate
> surface can reach the `built` or `analysis` phases. Only the `ra_ap_hir` question survives.
> The lesson is the cheap one: a compiler-internal crate is absent from crates.io by construction,
> so its absence there is not evidence of anything.

### 4c. Not started at all
No implementation code exists. `bridge/`, the extraction adapters, the CLI — all specified in
PLAN.md, none written. That is correct for this stage; the operator asked for a plan.

---

## 5. How to resume the Rust probes

```bash
cd .claude/skills/ast-grep-ripgrep/capability-expansion/evidence/probes/delta-arrow-probe
export CARGO_TARGET_DIR=<scratchpad>/probe-target     # ~7 GB, OUTSIDE the repo — required
export PROBE_ROOT=<scratchpad>/probe-tables
cargo run --quiet                          # PB01 + PB03
cargo run --quiet --example pb02_constraints
cargo build --example pb04_deltaops        # expected to FAIL with E0432 — that is the result
RUST_BACKTRACE=0 cargo run --quiet --example pb05_pb07_session_exprs   # PB05 + PB07 + PB12
cargo run --quiet --example pb06_preimage                              # PB06 + PB11
RUST_BACKTRACE=0 cargo run --quiet --example pb09_pb10_recursion_cdf   # PB09 + PB10
```

`RUST_BACKTRACE=0` on the first and third: PB07e deliberately triggers a panic inside a spawned
task (the delta-rs `get_table_source` abort), and a backtrace buries the result under 200 lines.
PB09 takes ~25s because one arm is *supposed* to time out. The PB08 Python probe is separate:
`uv run python evidence/probes/PB08-rich-syntax-regex.py`.

The crate declares an empty `[workspace]` so it detaches from the repo's workspace (the repo does
the same for its acceptance fixtures). **Verify `cargo metadata --no-deps` still reports 3
workspace members after any change.**

If the scratchpad target dir is gone, a cold build is ~10 min and ~1,100 dep artifacts. All
sources are cached in `~/.cargo` including the delta-rs git checkout at
`~/.cargo/git/checkouts/delta-kernel-rs-*/393fbf6/` — reading that source directly was how the
PB03 mechanism was established before the compile finished, and it is faster than a build.

---

## 6. Harness traps that bit me — do not repeat

1. **`cargo build 2>&1 | tail -40` masks the exit code.** Cargo failed with 3 errors, `tail`
   returned 0, and the task notification said *"completed (exit code 0)"*. I reported a successful
   build that had not happened. Use `${PIPESTATUS[0]}` or `--message-format=short` with no pipe.
   A pipe through `tail` also withholds all output until exit, so progress is unobservable.
2. **A probe that cannot fail proves nothing.** PB02b first supplied `Statistics::new_unknown`
   (every field `Absent`), so "no change" was guaranteed. **PB06 repeated the mistake in a new
   shape:** its data had no row at the top of a bucket, so the correct and incorrect `preimage`
   both returned 100 and the off-by-one was invisible. It passed, wrongly, on the first run. Ask
   of every probe: *what value would have to exist for the arms to differ?*
3. **Check the observation channel before trusting a negative.** PB02b rendered plans with no
   statistics at all, so "same" meant *not observed*, not *not used*. The probe now asserts the
   channel is live first.
3a. **When the control fails the same way as the treatment, neither was measured.** PB07's first
   run had arms c *and* d failing with the same `MetricObserver` error — the session could not
   execute a Delta write at all, so the UDF never entered the question. The verdict looked like a
   finding.
3b. **A plan-shape observable that disagrees with execution is worse than none.** PB09's first
   heuristic reported "bound is inside the recursive term" for *both* arms while one of them timed
   out; it was matching the base term's unrelated filter. Trust the execution, then fix the
   observable to agree with it — never the reverse.
4. **The repo's `pre_bash` hook denies a bare `+nightly`** and blocks writes outside the repo
   boundary. Heredocs containing `///`, `->`, `<` or bare `/` can trip the redirect parser —
   use the Write tool or a Python script in the scratchpad instead.
5. **`post_edit` runs ruff/rustfmt on every write** and rejects the literal strings `# noqa`,
   `# type: ignore`, `#[allow(...)]` **even inside prose or docstrings**. Reword.
6. Repo Python conventions: no `print` (T201 — use `sys.stdout.write`), ruff line-length 100,
   full type annotations, no suppressions.
7. **Ruff lints the scratchpad too**, including `RUF001` (ambiguous en dash) and `E501`. Do not
   embed markdown prose inside a Python patch script — the em/en dashes and long doc lines will be
   rejected. Edit markdown with the Edit tool directly.
8. `grep`/heredoc arguments containing a bare `/` can trip the boundary parser (`grep -v '^ *at /'`
   was denied as a write outside the repository). Redirect to a file first, then read it.
9. `rustfmt --edition 2024 <file>` before any Rust write lands, or `post_edit` rejects it. The
   probe crate is edition 2024 but is **not** a workspace member, so plain `cargo fmt` at the repo
   root does not reach it.

---

## 7. State of the tree

**Nothing is committed.** `git status` shows:

```
 M .claude/skills/README.md      (rows added for rust-code-model + this work)
 M justfile                      (knowledge-* recipes parameterised, from earlier work)
?? .claude/skills/ast-grep-ripgrep/    (the whole skill, incl. this directory)
?? .claude/skills/rust-code-model/     (built earlier this session)
?? .claude/skills/typer-rich/          (pre-existing, untracked)
```

Checks that must stay green — run all three after any change here:

```bash
just lint-agents                                          # 13 skills, paths + recipes resolve
cd .claude/skills/ast-grep-ripgrep && python3 build/verify.py --skip-rebuild
cargo metadata --format-version 1 --no-deps               # must still be 3 workspace members
```

All three pass as of this handoff. The `capability-expansion/` directory sits outside
`content/`, so it does not affect the skill's digests.

---

## 8. Reading order for the next session

1. This file.
2. [`PLAN.md`](PLAN.md) §0 (settled), §1 (architecture), §10 (what was deferred), §11 (risks).
3. [`evidence/12`](evidence/12-pb05-pb12-executed-results.md) — the largest body of measured
   results, including all three silent-failure modes. Then
   [`evidence/10`](evidence/10-pb01-pb03-executed-results.md) and
   [`evidence/11`](evidence/11-pb02-provider-wrapper-results.md).
4. Only then the individual dossiers, as needed. Evidence 01–09 are round-1 readings; where a
   probe later contradicted one it is **marked in place**, not deleted — trust the marking.
   [`evidence/09`](evidence/09-probes-executed-and-open.md) now records every probe's verdict
   inline, so it is the fastest index into the executed results.
