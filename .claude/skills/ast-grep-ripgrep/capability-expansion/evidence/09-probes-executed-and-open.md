# Evidence 09 — Probes executed in round 1, and the probe backlog for round 2

The surrounding skills establish a discipline: a claim that can be executed should be, and every
probe carries a control that must come out the other way. This dossier records what round 1
actually ran, and names the probes round 2 must run before the design can be called settled.

Probe sources live in `evidence/probes/`, captured output beside them.

---

## P01 — Which bracketed forms survive Rich markup parsing? **EXECUTED, decisive**

- Script: `probes/P01-rich-markup-bracket-survival.py`
- Capture: `probes/P01-rich-markup-bracket-survival.out.txt`
- Subject: rich 15.0.0 (the pinned version; `.venv/lib/python3.14/site-packages/rich`)
- Construction pinned: `width=80, force_terminal=False, color_system=None,
  legacy_windows=False, _environ={}` — change any and the bytes change.

**Why it was run:** evidence 07 §6 left `[a-z]` and `[[:alpha:]]` untested, and those are the two
most common things a regex capability catalog prints. The typer-rich skill's own probe M002
tested eight forms and covered neither.

### Result — 11 of 24 forms are silently altered

| Altered (content deleted) | Survives intact |
|---|---|
| `[x]` `[a-z]` `[a-zA-Z_]` | `[A-Z]` `[0-9]` `[^a-z]` |
| `[pattern]` `[kind]` `[inside]` | `[[:alpha:]]` `[[:^digit:]]` `[\d-[1]]` |
| `[has]` `[stopBy]` `[regex]` | `[PATH]` `[--flag]` `[-n]` `[FILE]` |
| `[path]` `[glob]` | `[]]` `[^]]` `x[0]y` |

### What this settles

1. **`[a-z]` is deleted.** The most common character class in any regex catalog vanishes without
   error. `[A-Z]` and `[0-9]` survive, so the bug is invisible to spot-checking.
2. **`[a-zA-Z_]` is deleted** — the standard identifier class.
3. **Every ast-grep rule field name in brackets is deleted**: `pattern`, `kind`, `inside`, `has`,
   `stopBy`, `regex`. That is the entire ast-grep rule vocabulary, which this tool exists to
   describe.
4. POSIX classes survive, because the doubled bracket does not parse as a style name.
5. `[^a-z]` survives — the caret saves it. So two forms differing by one character behave
   differently, which is exactly why this cannot be left to reviewer attention.

### Controls, both clean

- `rich.markup.escape` rescued **all 11** altered forms; none failed.
- `markup=False` preserved **all 24** forms; no failures.

### Consequence for the plan

Not a style guideline — an enforced rule. Every `Console.print` whose argument is not an authored
literal must pass `escape()` or `markup=False`, and that is mechanically checkable with an
ast-grep rule in the repository's existing `project-*` corpus. The `--json` path must not touch a
Console at all (evidence 07 §4).

---

## P02 — Version alignment between the skills and a buildable workspace. **EXECUTED**

Read from `Cargo.toml` and the skills' `PROVENANCE.json`:

| Library | Skill pin | Repo dependency | Aligned |
|---|---|---|---|
| datafusion | 55.1.0 | `datafusion = "=55.1.0"` | yes |
| arrow | 59.3.0 | `arrow = { version = "59.3.0", features = ["canonical_extension_types"] }` | yes |
| object_store | 0.13.2 | (transitive) | yes |
| sqlparser | 0.62.0 | (transitive) | yes |
| deltalake | 1.0.0+58f07cd6 | `rev = "58f07cd62bfbce3649a7e1c87c696288068ae184"`, features `["datafusion", "rustls"]` | yes |
| rich | 15.0.0 | installed 15.0.0 | yes |
| typer | 0.27.2 | **not installed** | — |

Two findings:

- **`canonical_extension_types` is already an enabled arrow feature**, which confirms the
  extension-type design in evidence 03 §2 is buildable rather than aspirational.
- **`deltalake` is pinned to a git rev, not a crates.io release.** The plan must pin the same rev;
  a `1.0.0` from crates.io is a different artifact.
- typer is absent from this workspace, which is expected — the CLI is a separate Python component
  and will bring its own pin.

---

## Probe backlog — ALL EXECUTED

These were the questions the round-1 evidence could not settle from the indexes alone. Each needed
a control. PB01–PB04 are in [evidence 10](10-pb01-pb03-executed-results.md) and
[evidence 11](11-pb02-provider-wrapper-results.md); PB05–PB12 are in
[evidence 12](12-pb05-pb12-executed-results.md); **PB13 and PB14**, added in round 2 to close the
last two answerable questions, are in [evidence 13](13-pb13-pb14-closing-open-items.md).
**Seventeen probes have executed and nothing in this backlog remains open.** **PB15**, added
later still, is in [evidence 14](14-pb15-rustc-public.md); **PB17**, added in phase 7, is in
[evidence 15](15-pb17-replace-where.md); **PB18**, added while building the UDF inventory, is in
[evidence 16](16-pb18-utf8-preimage.md); **PB16**, the last one, is in
[evidence 17](17-pb16-proc-macro.md).

**The probe backlog is empty.** Eighteen probes have executed and every question in this dossier
has an answer.

> **Correction, 2026-09-16.** This section used to say that the two surviving questions "need a
> real workspace rather than a synthetic fixture". That was true of one of them. PB15 answered the
> `rustc_public` question with a three-function fixture and no workspace at all, because the crate
> ships in the `rustc-dev` rustup component, which was already installed. The claim was made by
> checking crates.io and a component list — the wrong registry for a compiler-internal crate.
> One question survives, and it is the `ra_ap_*` one; see PLAN-V2 §12.3.

### PB01 — Does an Arrow extension type survive a Delta write/read cycle? ✅ **DONE — YES**
Evidence 03 §6 and 04 §5. Write a table whose field carries
`ARROW:extension:name = codesearch.entity_id`, read it back through `DeltaScan`, and check whether
the field metadata is present. **Control:** a field with ordinary metadata, to distinguish
"extension metadata is stripped" from "all field metadata is stripped".
*Decides whether typed identity is a storage property or a re-attachment step.*

### PB02 — Does a wrapping `TableProvider` supplying `constraints()` actually change the plan? ✅ **DONE — YES for constraints, NO for statistics**
Evidence 05 §1/§4. Wrap `DeltaScan`, return `Constraints::new_unverified(vec![Constraint::PrimaryKey(…)])`,
and compare `EXPLAIN` output for a `SELECT DISTINCT entity_id` against the unwrapped provider.
**Control:** the same query with the wrapper returning `None`.
*Decides whether the wrapper is worth building.*

### PB03 — Which Arrow types does delta-rs actually reject on write? ✅ **DONE — almost none; it narrows silently instead**
Evidence 04 §1 derives the list from the kernel's `PrimitiveType` enum. Confirm by attempting to
write each of `Dictionary(Int32, Utf8)`, `FixedSizeBinary(32)`, `UInt32`, `Utf8View`,
`RunEndEncoded`, `LargeList`. **Control:** `Utf8`, `Int64`, `List<Struct>`, which must succeed.
*Decides the persisted schema. Currently derived, not measured.*

### PB04 — Is `DeltaOps` reachable in the pinned delta-rs? ✅ **DONE — NO (E0432)**
Evidence 04 §5. It is absent from the skill's `symbols.tsv`, `aliases.tsv`, `unresolved.tsv` and
`unnameable.tsv`. Either it does not exist at this rev or the skill did not index it. A one-line
compile settles it. *Decides the shape of every call site.*

### PB05 — Do `MergeBuilder` predicates see UDFs registered on the supplied session? ✅ **DONE — YES**
Evidence 05 §4. Confirmed in **both** the join predicate and match-clause predicates; the control
(the same merge with a session lacking the UDF) fails with `Invalid function`. Also established:
`SessionFallbackPolicy::InternalDefaults` is the **default** and discards a non-`SessionState`
session with only a log line, so every call site should set `RequireSessionState`.
→ [evidence 12](12-pb05-pb12-executed-results.md).

### PB06 — Does `ScalarUDFImpl::preimage` fire for a Delta-backed scan? ✅ **DONE — YES, and it prunes**
Evidence 02 §6. Rewrites to a range on the bare column, reaches `partial_filters`, and prunes four
files to one — matching the hand-written gold standard exactly. **Three conditions, two of which
fail silently:** the UDF must be `Immutable`; the returned interval is consumed as **half-open**
(a closed upper bound returned 99 rows where the answer is 100); and the returned predicate must
name a **bare stored column** (the built-in `floor` is rewritten but prunes nothing).
→ [evidence 12](12-pb05-pb12-executed-results.md).

### PB07 — Can a Delta `CheckConstraints` expression reference a UDF or another table? ✅ **DONE — UDF yes but hazardous; table NO, it panics**
Evidence 04 §5. A UDF-bearing constraint is stored as SQL **text** and re-parsed by every future
writer, so the table becomes unwritable by any session that does not register that UDF. A
constraint naming another table **aborts the process** — `DeltaContextProvider::get_table_source`
is `unimplemented!()`. Referential invariants are therefore scheduled queries, not constraints.
→ [evidence 12](12-pb05-pb12-executed-results.md).

### PB08 — Does `Syntax` have a `regex` lexer, and does it markup-parse its content? ✅ **DONE — no lexer; does NOT markup-parse**
Evidence 07 §6. Of **602** pygments lexers none is a regex lexer, and asking for one returns `None`
identically to a deliberately fake name. But `Syntax` does **not** markup-parse: 0 of 8 P01-lossy
forms were altered, with both controls clean. Safe renderable, no highlighting.
→ [evidence 12](12-pb05-pb12-executed-results.md).

### PB09 — Does recursive-CTE execution push predicates into the recursive term? ✅ **DONE — NO**
Evidence 02 §6. An outer bound is left **above** `RecursiveQuery`; on a cyclic graph the query
never terminates (timed out at 20s, against 24ms for the same bound written inside the recursive
term). An explicit depth bound inside the recursive term is mandatory, not optional.
→ [evidence 12](12-pb05-pb12-executed-results.md).

### PB13 — Does `WriteBuilder::with_input_plan` share a transaction with `ConstraintBuilder`? ✅ **DONE — NO, always two commits**
Evidence 05 §4 q4, open since round 1. Create `v0` → write `v1` → constraint `v2`. A constraint add
that fails validation rolls back **only itself**, leaving the violating row committed and no
constraint recorded; adding the constraint first rejects the write and nothing lands. DDL-first is
the measured-correct order, not a dodge. → [evidence 13](13-pb13-pb14-closing-open-items.md).

### PB14 — Does a `UNION` recursive CTE (`is_distinct=true`) terminate on a cyclic graph? ✅ **DONE — only without a `depth` column**
Plan v2 §12.3 item 2; PB09 had measured only `UNION ALL`. The deduplicator runs over the **full
output tuple**, so a projection carrying `depth` is unique by construction and `is_distinct` is
inert — the same cyclic graph still timed out with the flag confirmed in the plan. Dropping `depth`
terminates in 26 ms. The two mitigations are **mutually exclusive**.
→ [evidence 13](13-pb13-pb14-closing-open-items.md).

### PB10 — Does `DeltaCdfTableProvider` support filter pushdown? ✅ **DONE — accepts everything, narrows only on partition columns**
Evidence 05 §4. `supports_filters_pushdown` returns `Exact` **unconditionally**, so the filter
always disappears from the plan — which is not evidence of narrowing. Measured: a partition-column
filter cut 3 files to 2; data-column and `_commit_version` filters read all 3 while still
answering correctly via `FilterExec`.
→ [evidence 12](12-pb05-pb12-executed-results.md).

---

## Discipline these probes follow

Taken from the surrounding skills, and applied here:

- **A probe without a control proves nothing.** P01's controls (`escape()` rescues; `markup=False`
  preserves) are what turn "11 forms changed" into "the mechanism is markup parsing, and there are
  exactly two correct remedies".
- **Pin the construction and quote it with the output.** P01 pins width, colour system, terminal
  forcing and environment; the typer-rich skill records that a first draft of its own M002 came
  out inconclusive because the environment was not isolated.
- **`recorded` is weaker than `confirmed`, and the difference is stated.** P01 is confirmed;
  P02 is a reading, not an experiment, and is labelled as such.
