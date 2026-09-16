---
id: ADR-0046
title: Admit pyrefly alongside ty as a Python semantic engine, and permit ruff as analysis tooling
status: accepted
date: 2026-09-15
deciders: [paul-heyse]
level: decision
principles: [DM-04, DM-05, DM-13, DM-31, DM-43]
design: [§B1, §B4, §B13]
review: not-required: No design review raised this; the owner directed the change while planning a repo-agnostic capability skill, then applied it directly to AGENTS.md and the hook on 2026-09-15. This record was rewritten afterwards to describe what landed.
evidence: Interface-checked
supersedes: []
superseded-by: null
revisit: pyrefly is proposed as the production Python producer (a separate record with gate impact), or a produced `SemanticObservation` is traced to a producer other than ty or rust-analyzer.
verification: `scripts/test-hooks.sh` (the pyrefly case expects allow; pyright and mypy are asserted against `.claude/settings.json`'s deny list, which is what enforces them); `scripts/test-adr-lint.sh`; the `pyrefly-ruff` skill's `build/verify.py` offline-rebuild check, which proves no shipped artifact depends on either binary.

---

# ADR-0046: Admit pyrefly alongside ty as a Python semantic engine, and permit ruff as analysis tooling

## Context

Design §B4 names Astral **ty** as the Python semantic engine and adds "Not pyrefly, not pyright,
not mypy." `scripts/hooks/pre_bash.sh:13` enforces that as a blanket word-boundary denial of the
command names anywhere in the repository. That conflates two different things: *which producer
may emit a `SemanticObservation`* — a real binding decision — and *which programs a session may
execute at all*. The second was never the decision; it is an artifact of the cheapest available
oracle.

The conflation became load-bearing while building `.claude/skills/pyrefly-ruff/`, a repo-agnostic
capability repository indexing the ruff crate family and pyrefly as **libraries under study**, in
the same way `.claude/skills/datafusion/` indexes DataFusion and Arrow. Nothing it produces enters
the evidence model. Under the current hook the skill can still acquire rustdoc JSON — the regex
permits the GitHub URL, `pyrefly_*` crate names and every path — but it cannot execute either
binary to capture the machine-readable oracles that make such a skill trustworthy rather than
recalled. The owner directed on 2026-09-15 that the stipulation itself be changed.

**What the owner then applied is broader than what this record originally proposed**, and this
record has been rewritten to describe the change that landed rather than the one drafted. The
proposal was to keep ty as the sole evidence producer and gate execution of the others. The
applied edit instead makes `AGENTS.md` read "Astral **ty** or pyrefly, not pyright, not mypy",
adds "pyrefly may be used and may become the project's engine", and **deletes `pre_bash.sh`
rule 1 outright**. pyrefly is therefore an admissible engine, not merely a subject of study.

## Scope

Amends §B4's **admissible set**. ty remains the production Python semantic engine today and the
only producer of Python `SemanticObservation`s; gates P01–P10 are untouched and no gate result
changes. What changes is that pyrefly stops being excluded: it may be executed, indexed, and
proposed later as a producer.

Making pyrefly *the* production engine is still a separate record with gate impact, and this one
must not be read as having decided it. Nothing here qualifies pyrefly as a producer; it removes
the prohibition that made qualifying it impossible to even attempt.

## Drivers

Correctness: a capability index built from recollection instead of the tool's own JSON is the
failure mode these skills exist to prevent. Reproducibility: ruff and pyrefly emit versioned,
machine-readable catalogs that pin cleanly. Separation of concerns: a policy about evidence
producers should not be enforced as a prohibition on process execution. Repository boundary:
acquisition compiles and installs into a capsule outside every working repository, so §B1 is
unaffected.

## Options

- **Keep the blanket denial** — rejected: it forbids indexing tools the project wants to use, and
  its stated rationale ("NOT this project's engine") does not describe what it actually blocks.
- **Work around it** by invoking the binary from inside a Python subprocess, which the regex does
  not see — rejected: AGENTS.md says a denial "explains which blueprint section it comes from —
  read it rather than working around it", and a hook that can be bypassed by indirection enforces
  nothing.
- **Narrow the denial to the production Python tree and permit the tools elsewhere** — drafted,
  then superseded by the owner's own edit before it was applied.
- **Admit pyrefly outright and delete the rule** — **selected by the owner.** It matches the
  stated intent ("this is no longer aligned with our approach and so we should absolutely use
  pyrefly and ruff") without inventing a path classification that would drift as the repository
  grows. Its cost is recorded under Consequences: deleting the whole rule also un-enforced
  pyright and mypy, which §B4 still forbids.

## Decision

**pyrefly is admissible alongside ty.** It may be executed anywhere, indexed as a subject of
study, and proposed in a later record as a production producer. `ruff` likewise. pyright, basedpyright
and mypy remain excluded.

**ty is still the production engine today.** No `SemanticObservation` originates from pyrefly at
this record's date, gates P01–P10 are unchanged, and `uv run ty check` remains the command the
justfile runs.

As applied on 2026-09-15:

| File | Change |
|---|---|
| `AGENTS.md:21` | binding row now reads "Astral **ty** or pyrefly, not pyright, not mypy" |
| `AGENTS.md:65` | "Neither is this project's engine although pyrefly may be used and may become the project's engine" |
| `scripts/hooks/pre_bash.sh` | rule 1 deleted in full |
| `docs/design/DESIGN.md` §B4 | reconciled to the above, with pyrefly named as admissible |

The rule a later reader can check: **if a `SemanticObservation` in `enrichment-store` names a
producer other than ty or rust-analyzer, and no record after this one qualified it, this decision
has been broken.** Executing pyrefly, indexing its API, and linting with ruff are all permitted;
silently promoting it to producer is not.

### Consequences

Easier: the `pyrefly-ruff` skill captures `ruff rule --all --output-format json` (970 rules at the
pin), `ruff config --output-format json` and pyrefly's `--report-glean` / TSP surfaces from the
tools themselves rather than from prose. Downstream, `.claude/skills/fastmcp/` can close the
"call graphs and cross-file references are absent" limit its own `reference.md` declares, because
`--report-glean` emits `python.CalleeToCaller` and `python.XRefsViaNameByTarget` as JSON keyed by
fully-qualified dotted path.

Harder, in a smaller way than first recorded. Deleting rule 1 removed the *hook*-level guard on
pyright, basedpyright and mypy along with pyrefly's. This record originally called that an
enforcement gap and carried a patch to restore the rule. It is not a gap:
`.claude/settings.json` still denies `Bash(pyright*)`, `Bash(basedpyright*)`, `Bash(mypy*)` and
`Bash(uv run mypy*)`, and the permission system checks **each segment of a compound command**, so
`cd python && mypy .` is denied too -- verified by running it, 2026-09-15.

What actually changed is *which layer* enforces it, and `scripts/test-hooks.sh` was asserting the
old one. Its `mypy` case now asserts the deny list instead, alongside
`scripts/guardrails-check.sh`, which already digests `.claude/settings.json` as part of the
enforcement-layer manifest.

Restoring a hook rule for those three would add defence in depth and is a one-line edit, but it
is optional hardening rather than a correction.

Foreclosed: nothing. Reversing this is a revert of the AGENTS.md row plus restoring rule 1; no
schema, artifact or gate depends on it.

Note the asymmetry this exposes: §B13 already authorises "a local capsule build on a dated pinned
nightly" as the rustdoc fallback, so indexing pyrefly's unpublished crates was always permitted by
the design — only the hook disagreed.

### Compensating controls

`scripts/deps-policy-python.sh` still lists pyrefly in `WRONG_ENGINE`, and that is retained
deliberately: admitting a tool for execution is not the same as admitting it to `pyproject.toml`,
and promoting it to a declared dependency should require the follow-on record, not drift. ty
remains pinned in `uv.lock` and never installed as a global `uv tool`. The skills' acquisition
writes only into a capsule under `$LIBENR_CACHE_HOME`, and `verify.py` proves an offline rebuild
with neither binary on `PATH`, so no shipped artifact can silently depend on having run them.
`tests/gates.toml` is unchanged, so any drift in P01–P10 remains visible.

## Evidence

Retrieved 2026-09-15 from primary sources in this session. This is interface evidence about what
the tools expose; it is not a claim that either has been qualified as a producer.

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| The current denial is a word-boundary match on the command name, not a path rule | `scripts/hooks/pre_bash.sh:13` | 2026-09-15 | `(^\|[^[:alnum:]_./-])(pyrefly\|pyright\|basedpyright\|mypy)([^[:alnum:]_-]\|$)` |
| pyrefly ships no usable Rust crates, so indexing it requires a local rustdoc build | [crates.io](https://crates.io/api/v1/crates?q=pyre) | 2026-09-15 | `"name": "pyrefly", "max_version": "0.0.1", "updated_at": "2025-03-13T11:28:46.179295Z", "description": "Coming soon to crates.io."` |
| The ruff crate family is published and current | [crates.io](https://crates.io/api/v1/crates/ruff_linter/0.16.7/dependencies) | 2026-09-15 | `ruff_linter` 0.16.7 declares `ruff_python_ast ^0.0.13` and 17 further `ruff_*` deps at `^0.0.13` |
| docs.rs serves rustdoc JSON for those crates at a format version the repo already supports | `https://docs.rs/crate/ruff_python_ast/0.0.13/json` | 2026-09-15 | `HTTP/2 200`, `content-type: application/zstd`, decompressed `"format_version": 61` |
| §B13 already permits the local-nightly fallback this needs | `docs/design/DESIGN.md` §B13 | 2026-09-15 | "a local capsule build on a dated pinned nightly is the fallback, recorded as `locally_built_rustdoc`" |
| ruff exposes its own catalogs as JSON, so they need not be transcribed | `ruff rule --all --output-format json` (0.14.4) | 2026-09-15 | 934 objects, each with `code`, `linter`, `fix_availability`, `status`, `source_location{file,line}` |
| pyrefly exposes cross-reference and call-graph side channels | `pyrefly check --help` (0.51.1) | 2026-09-15 | `--report-glean <OUTPUT_FILE>  Generate a Glean-compatible JSON file for each module`; `--report-pysa <OUTPUT_FILE>` |

## Verification

`scripts/test-hooks.sh` is the live oracle: its `pyrefly` case expects **allow**, and pyright and
mypy are asserted against `.claude/settings.json`'s `permissions.deny`, which is the layer that
actually denies them now that hook rule 1 is gone. 71 cases pass.
`scripts/test-adr-lint.sh` covers this record's own front matter. The substantive
control is `.claude/skills/pyrefly-ruff/build/verify.py`'s offline-rebuild check, which runs
`build.py` and `verify.py` with no network, no cargo and neither binary on `PATH` — if a shipped
artifact needed either tool at read time, that check fails. Output lands in the skill's
`content/PROVENANCE.json` and in `docs/reports/acceptance.json` for the unchanged gates.

## Boundaries preserved

- **§B1 Repository boundary** — acquisition clones, compiles and installs only into a capsule
  outside every working repository; no repository under study is a subprocess working directory.
- **§B3 Python boundary** — unchanged; the FastMCP adapter and extraction worker stay thin and
  neither imports ruff or pyrefly.
- **§B4 Python semantic engine** — ty remains the production engine. The admissible set gains
  pyrefly; pyright and mypy stay excluded, enforced by the permission deny list.
- **§B5 Rust semantic engine** — unchanged.
- **§B12 Static extraction** — Griffe with `allow_inspection=False` remains the Python extractor.
- **§B13 Rust API source** — followed exactly: docs.rs first for the 31 ruff crates, a dated
  pinned nightly capsule build only for the 14 pyrefly crates docs.rs cannot serve.

## More information

- Design: `docs/design/DESIGN.md` §B4, §B13; blueprint §1.1, §5.4 (frozen).
- Prior art on the same boundary: ADR-0005 (ty's LSP capability), ADR-0007 (ruff's lint scope).
- Implemented by: `.claude/skills/pyrefly-ruff/`, whose `build/README.md` records the acquisition
  contract.
- Applied by the owner on 2026-09-15, because `pre_edit.sh` denies a session `AGENTS.md`,
  `CLAUDE.md`, `.claude/rules/`, `.claude/settings.json`, `scripts/hooks/` and `scripts/env.sh`.
  That list is unchanged and should stay unchanged: it is the reason this record exists at all
  rather than the edit having been made quietly.
- Consumed by: `.claude/skills/fastmcp/build/analysis.py`, which runs `check --report-glean`
  against a capsule to close that skill's documented cross-reference limit.

## Status history

- 2026-09-15 — proposed, as "permit pyrefly and ruff as analysis tooling" with execution gated
  behind `LIBENR_ALLOW_FOREIGN_CHECKER=1`.
- 2026-09-15 — **accepted**, with a broader decision than proposed. The owner applied the change
  directly to `AGENTS.md` and `scripts/hooks/pre_bash.sh`, admitting pyrefly as an engine rather
  than as a subject of study and deleting rule 1 rather than narrowing it. Title, Decision,
  Options, Consequences and Verification were rewritten to describe what landed; `DESIGN.md` §B4
  and `scripts/test-hooks.sh` were reconciled in the same pass.
