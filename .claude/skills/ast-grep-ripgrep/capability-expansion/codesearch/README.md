# codesearch

A machine-queryable catalog of code-search capabilities, built from the `ast-grep-ripgrep` skill's
own pinned indexes. Implements steps 1–5 and 7 of [`../PLAN-V2.md`](../PLAN-V2.md) §12.1: thirteen
canonical tables, thirty registered projections, nine cross-table invariants.

This tree is **its own Cargo workspace** and imports nothing from whatever repository hosts the
skill, so it travels with the skill. Verify with `cargo metadata --no-deps` in the host: its member
count must not change.

## Use

```bash
just build                 # compile
just catalog               # build the catalog from ../.. (the skill this lives in)
just cli coverage          # two numbers, never a composite score
just cli validate          # the cross-table invariants, with what each one inspected
just cli discover --capability constrain-target-by-contained-syntax
just cli negative-space    # what is known NOT to be reachable, and what to do instead
just cli interactions --bind engine=pcre2     # unbound facets answer `unknown`, not `false`
just cli chains --max-depth 8                 # bounded composition over a cyclic graph
just cli projections       # the retrieval surface, read from information_schema
just cli runs              # the extraction ledger -- why a projection is empty
just cli implements        # a mechanism and the Rust code implementing it, with the evidence
just cli closures          # what each closure traversed, and what limits it
just cli re-exports        # where an item can be named from
just cli --include-partial discover   # also serve rows from runs that did not finish
just codesearch-check      # the gate
just bridge-catalog        # regenerate crates/codesearch-bridge/CATALOG.md
just where                 # where artifacts and catalog tables actually live (outside the repo)
```

`--skill-root` is required on `codesearch-build` and never inferred: the tool works against **any**
copy of the skill, not only the one it sits inside.

## Shape

| crate | holds |
|---|---|
| `codesearch-bridge` | everything that extends DataFusion: the lattices, identity, the session, the provider wrapper |
| `codesearch-model` | Arrow schemas and the Delta DDL, plus the schema fixed-point check |
| `codesearch-build` | index reader, catalog assembly, writer (a binary) |
| `codesearch-query` | one-shot retrieval (a binary) |
| `python/` | the Typer/Rich front end; shells out, never opens a table |

## Why the code looks the way it does

Nearly every unusual decision here traces to a probe that measured a **silent** failure — one that
produces a wrong answer with no error. They are documented at the point of use, but the short list:

- **Lattices are stored as `Int8` ranks, not labels.** Rank order and byte order differ, so a
  label column cannot reliably prune. Storing the ordinal also makes the fold plain `min()`/`max()`
  with no UDF. (`bridge/src/lattice.rs`)
- **Every column type is a fixed point of the Delta conversion.** Delta does not reject types, it
  narrows them silently — `UInt32` truncates above `i32::MAX`. A round-trip test proves it rather
  than a reading. (`model/src/schema.rs`, `model/tests/fixed_point.rs`)
- **Constraints go on at creation, never after.** Write-first leaves the violating row committed
  *and* records no constraint. (`model/src/ddl.rs`)
- **The session comes from delta-rs, not from DataFusion**, and every builder is given
  `RequireSessionState` — the default silently discards the caller's session and its UDFs.
  (`bridge/src/session.rs`)
- **Every non-literal string reaching a terminal is escaped.** Rich's markup parser deletes
  `[a-z]` and all six ast-grep rule field names, with no error. (`python/codesearch_cli/render.py`)
- **Every projection is a registered view, and every invariant is one of them.** A projection
  composed at the call site cannot be enumerated, explained or checked; a registered one appears
  in `information_schema`, and `bridge/CATALOG.md` is generated from that list rather than
  transcribed. (`bridge/src/projection.rs`)
- **`validate` reports what each invariant inspected, not only what it found.** An invariant that
  examined no rows reports zero violations, which is indistinguishable from passing unless the
  denominator is printed beside it. One of the eight genuinely inspects nothing today, and the
  report says so.
- **Caller values bind as placeholders.** Which predicates appear is chosen from the arguments
  supplied; what they compare against never enters the SQL text. (`query/src/main.rs`)

- **Three lattice values that are all real.** A plan fragment's `recall` is `preserving` only with
  a **confirmed** probe behind it; `heuristic` where a condition is still owed, with the condition
  written down; and `unknown` where nobody has looked. `ast-grep -U --rewrite` holds `unknown`
  because no probe establishes whether a rewrite reaches every match it found.
  (`build/src/plan_fragments.rs`)
- **An unbound facet answers `unknown`, never `false`.** An interaction that depends on a choice
  the caller has not made is a choice still open, and reporting it as inapplicable would hide it.
  `min` then `max` over the `truth` lattice is the whole implementation. (`bridge/src/interaction.rs`)
- **A recursion's bound is inside the recursive term.** PB09 measured the alternative timing out at
  20 s on a cyclic graph where this form returns in milliseconds, and the fragment graph genuinely
  has a cycle: `rg -r` takes a line set and produces one. (`bridge/src/projection.rs`)

### Nothing is registered until a query names it

A one-shot binary builds a session, registers everything, answers one question and exits — and
registering everything was most of what it did. At twenty tables and fifty-two views, a query that
reads four tables spent 250 ms preparing before ~80 ms of answering: 63 ms opening Delta tables,
67 ms building run-filtered base views, 118 ms planning projections. Session construction, the
usual suspect, is 1.2 ms.

So the base tables and every view are behind a `SchemaProvider` that resolves on demand.
`table_names()` is sync, so the whole surface is still enumerable and `just cli projections` still
lists all 34; `table()` is async and called only for references the parser found, so a table nobody
named is never opened and a view nobody read is never planned. p95 went from ~330 ms to ~185 ms,
and it no longer grows with the catalog — only with what a query touches.

The argument for the old behaviour was that a projection which will not plan is a broken retrieval
surface, and finding that out per caller is how a catalog rots. That is right, so the guarantee
moved rather than being dropped: `just codesearch-check` plans every projection. A developer-time
guarantee belongs in the gate, not in every caller's latency budget.

### A run that did not finish serves nothing

Every canonical row carries the `run_id` of the extraction run that wrote it, and a run that did
not finish wrote rows nobody should be served. That rule could have been written into each of the
thirty projections, which is thirty chances to write it differently — so instead the Delta
providers register as `<name>__all` and the *unsuffixed* name is a view over them filtered to the
visible runs. Every projection reads `mechanism` and `surface_entry` exactly as before, and every
one of them is filtered, because there is no unfiltered name to read by accident.

`extraction_run` and `lattice` are exempt, and both exemptions are load-bearing. Filtering the run
table by visibility would be circular, and would hide the only record that some rows are
unaccounted for — `just cli runs` is the answer to "why is this empty?", and it has to keep
working when the answer is "a build died". `lattice` has no `run_id` at all: rank-to-label is not
an observation about a snapshot.

The run row is written **before** the rows it accounts for, so a crash leaves rows that are
attributable rather than orphaned. This is the same argument PB13 settled for constraints: when
two writes cannot share a transaction, the order is the entire mitigation.

## Two snapshots, and the rule that had to arrive with them

`compare` asks what changed between two snapshots, so two have to coexist -- and the moment they
do, every other question becomes ambiguous. None of the thirty-seven views filtered on
`snapshot_id`, because until phase 7 there was only ever one. A second would have returned two rows
per `describe` and doubled every coverage count, and nothing would have errored.

So the snapshot filter sits exactly where run visibility sits, and for the same reason: written
into each view it is thirty-seven chances to write it differently. `selected_snapshot` resolves the
request -- the caller's `--snapshot`, or the newest with a visible run -- and `install` adds one
clause to every base view. The two rules stay separate because they are different facts: a
complete run of an older snapshot is *visible* and *not selected*, and conflating them would let
`--include-partial` silently change which snapshot you are looking at.

The selection is never silent. `codesearch-query runs` marks which snapshot the request resolved
to, because a catalog holding two and answering about one has to say which -- the same argument
that keeps `extraction_run` out of every filter.

**Writing is scoped too, by run rather than by snapshot.** A run id is
`run:<snapshot>/<family>/0`, so `with_replace_where("run_id = '…'")` names this family and this
snapshot in one predicate over a column that already exists. Two things make that trustworthy
rather than merely written down: delta-rs rejects a batch whose rows do not all satisfy the
predicate, so the writer proves its own scope; and the predicate is silently ignored outside
`Overwrite` mode, so every test of it carries a control that must lose the rows the treatment
keeps. (`build/src/writer.rs`, PB17.)

## How `compare` decides that something changed

It joins on `entity_key` and on nothing else. That is not a simplification -- `entity_id` is
`blake3(entity_key ‖ snapshot_id)`, so it differs between two snapshots for the *same* entity by
construction, and joining on it would report a total rewrite of everything.

Which leaves `changed` undecidable from identity alone, so every canonical row carries a
`payload_digest` over its own content. What that digest excludes is the interesting part, and both
exclusions were found by running it rather than by thinking about it:

- **every column typed `codesearch.entity_id`**, not merely the row's own. `catalog.parameter`
  points at its mechanism, and that pointer moves with the snapshot -- the first run reported all
  165 parameters as changed when three things had changed. The rule is by extension type, so the
  next family inherits it without anyone remembering to extend a list.
- **`ord`**, because a position describes the sequence rather than the element. Removing one flag
  shifted every later sibling and restated one removal as a hundred changes. Nothing is lost:
  `interaction_atom`, the one table whose order is semantic, encodes it in its key *and* in
  dedicated columns.

The output is `added` / `removed` / `changed` and nothing else -- the host project's own vocabulary
(`crates/enrichment-core/src/compare.rs`), whose first line is "Deterministic differences between
immutable observations, not a compatibility proof." No severity, no score, for the same reason
coverage reports three numbers and never a total.

## What validation found

The `violations_*` views were written to guard against defects. Writing them found two, both wrong
*data* that no CHECK constraint could have caught, because both are cross-table facts -- exactly
the class PLAN-V2 §9.1 moved out of constraints and into queries.

- **16 regex mechanisms were in no coverage denominator at all.** A construct supported by both
  engines is two mechanisms, but `catalog.surface_entry` emitted one row per index row and picked
  an engine for it. So `tool` read `rg` while `mechanism_key` named a PCRE2 mechanism, and the
  `rg` mechanism appeared in no coverage number -- a catalog quietly reporting less than it holds.
  A construct available in both engines is now two inventory items, because "available in the Rust
  engine" and "available in PCRE2" are two separate columns the index carries and both are true.
- **49 behaviour assertions named mechanisms that do not exist.** `behaviors.tsv` records the
  topic page a probe belongs to -- `structural-patterns`, `pcre2-advanced-regex` -- and the
  subject was minted as `mech:<tool>/cli/<topic>` and labelled a mechanism. No flag, rule field or
  construct has those names. The subject is now a topic and says so. A mechanism-level link is
  derivable from the probe's command line, but only by parsing it, and a heuristic link in an
  evidence catalog is worse than an honest absence.

## What is deferred, and why

The full remaining scope, phase by phase, is in [`../IMPLEMENTATION.md`](../IMPLEMENTATION.md) §4.
In short:

**`explain`** is the next projection, and it needs `evidence.derivation` rows, which only pass F
writes.

**`IdentityDiscipline` is buildable and its prerequisite is measured.** A recursive CTE refused to
plan because one term carried the `codesearch.lattice` extension name and the other did not, which
establishes that Arrow field metadata reaches the logical plan schema and not merely storage --
exactly what a rule rejecting `entity_key ⋈ entity_id` needs. **`LatticePropagation` should not be
an analyzer rule**: parent-child is not decidable from a plan, so a safe version catches nothing
and an aggressive one blocks legitimate queries. A test over the thirty registered views is the
same guarantee, more cheaply.

**`explain` needs `evidence.derivation` rows**, which only pass F writes. The seven Rust extraction
families and CDF re-derivation are step 6 and the rest of step 8.

## State

Everything lives outside the repository under study, honouring the host's development sandbox when
there is one:

```
$CODESEARCH_HOME/target/codesearch   build artifacts (~7 GB warm)
$CODESEARCH_HOME/catalog             the Delta tables (~500 KB)
```
