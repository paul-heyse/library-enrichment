---
id: ADR-0007
title: ruff does not lint the generated boundary DTOs
status: accepted
date: 2026-09-13
deciders: [paul-heyse]
level: decision
principles: [DM-52]
design: [§6.3]
review: not-required: backfilled; recorded before the design-review process existed (ADR-0009)
evidence: Implemented
supersedes: []
superseded-by: null
revisit: A second generated Python directory appears, or ruff gains a rule that should apply to generated code
verification: `just lint` covers the rest of `python/`; `uv run ty check` still covers `_generated`

---

# ADR-0007: ruff does not lint the generated boundary DTOs

## Context

`AGENTS.md` is unambiguous: "No suppressions: no `# noqa`, no `# type: ignore`, no
`#[allow(...)]`, no growing an ignore list. Fix the cause or record an ADR." This is the ADR.

Blueprint §6.3 fixes a one-directional generation chain: Rust wire types → versioned JSON
Schemas → Pydantic boundary DTOs. `just schemas-generate` runs it end to end, and
`.claude/rules/generated-artifacts.md` forbids hand-editing any output.

Descriptions flow along that chain. A Rust doc comment on a wire field becomes a `description`
in the emitted JSON Schema, and `datamodel-codegen` inlines it into
`Field(description="…")` as a single Python string literal — newlines and all. One of them is
currently 340 characters, because the Rust doc comment is three paragraphs. Ruff's `E501`
(line-length 100) fires on eight such lines.

There is no in-file remedy. The file is regenerated on every `just schemas-generate`, so any
edit is erased; adding `# noqa` is both forbidden and equally erased. The only way to satisfy
`E501` would be to shorten the Rust doc comments — letting Python's line-length budget dictate
how Rust wire types may be documented, permanently, for every field added from here on. That
trade is backwards: the doc comments are the authoritative documentation, and they are good.

Two other options were tried first, and both were genuine fixes worth keeping:

- `--field-constraints`, so the generator emits `Field(min_length=1)` instead of
  `constr(min_length=1)`. `ty` correctly rejects the latter — a function call is not a valid
  type expression — and it is Pydantic v1 idiom besides. This is now in
  `scripts/schemas-generate.sh` and fixed a real defect, but it does not touch line length.
- `--formatters ruff-format`, so generated output matches `just fmt-check`. Also kept. Neither
  black nor ruff splits a long string literal, so seven `E501`s survived it.

## Decision

`pyproject.toml` sets `[tool.ruff] extend-exclude = ["python/enrichment_mcp/_generated"]`.

Ruff governs hand-written Python. It does not govern machine-generated output whose content is
decided upstream in Rust and whose only legal edit is regeneration.

This is deliberately narrow. It is one directory, named explicitly; it is not a rule ignore, so
every ruff rule stays on everywhere else including the rest of `python/enrichment_mcp/`; and it
removes no check that anything could act on. The distinction from the suppressions `AGENTS.md`
bans is that those hide a finding in code an author controls, whereas this scopes a style
linter away from a build artifact.

`ty` is **not** excluded. Type errors in generated code are real defects in the generation
chain and must stay visible — that is exactly how the `constr(...)` problem above surfaced.

## Evidence

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| Generated files are never hand-edited | `.claude/rules/generated-artifacts.md` | 2026-09-13 | "Editing a generated file makes the three drift silently, which is exactly the failure the generation pipeline exists to prevent." |
| The chain is Rust → JSON Schema → Pydantic | `docs/blueprint/IMPLEMENTATION_BLUEPRINT.md` §6.3 | 2026-09-13 | "Generate versioned JSON Schemas from the Rust wire types; generate the small Pydantic boundary DTOs from those schemas where supported." |
| An ADR is the sanctioned route | `AGENTS.md` | 2026-09-13 | "No suppressions… no growing an ignore list. Fix the cause or record an ADR." |
| The violations are inlined descriptions, measured | `ruff check --isolated --line-length 100 --select E,F,I,UP,B,SIM,ANN,RUF,T20 python/enrichment_mcp/_generated/` | 2026-09-13 | 8 errors, all `E501`; the longest line is 340 characters, a three-paragraph Rust doc comment flattened into one string |

## Verification

`just lint` and `just fmt-check` pass with the exclusion and fail without it — that is the
executable consequence. `just typecheck` still covers the directory, so a regression in
generation fidelity (the `constr(...)` class of problem) still fails a gate.

The generated DTOs are not left unchecked: `tests/contract/test_mcp_catalog.py` and
`tests/e2e/test_mcp_daemon_handshake.py` validate every envelope the adapter actually emits
against the generated JSON Schema, under gates **C14** and **C19**. Correctness is asserted by
conformance, which is the right oracle for a generated artifact; style is not asserted at all,
which is the point.

## Consequences

Easier: a wire field can be documented properly in Rust without anyone weighing the sentence
against a Python column limit. The exclusion is stable — it does not need revisiting each time
a doc comment grows.

Harder: a genuine style problem in the generator's output template would now go unreported by
ruff. The mitigation is that `ty`, the conformance corpus, and the contract tests all still run
over this directory, and those catch defects rather than formatting.

Reversing this is deleting one line from `pyproject.toml`, at the cost of the constraint on
Rust documentation described above.

## Boundaries preserved

The generation chain is unchanged and still one-directional. Generated artifacts are still never
hand-edited. `ty` remains the Python semantic engine, applied to this directory as to every
other. No suppression comment was added anywhere in the repository — the ban on `# noqa` and
`# type: ignore` holds without exception, and the `post_edit` hook still enforces it.

## Status history

- 2026-09-13 — accepted.
