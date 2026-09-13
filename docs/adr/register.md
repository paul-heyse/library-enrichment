# Deferred-decision register

Every item here was deliberately **deferred with a stated trigger** rather than decided. A
deferral that nobody revisits is an omission, so each row carries the observable trigger, a check
that can be run, an owner, and the date the check is next due.

This is not a backlog. The 45 gates reading `not_run` are future phases and are already tracked
by `tests/gates.toml` and `STATUS.md`; a row belongs here only when a *decision* was consciously
postponed.

| Column | Meaning |
|---|---|
| `R-NN` | Row id. Cited from ADRs and from `DESIGN.md`; ids are never reused. |
| `item` | What was deferred, in one line. |
| `ADR` | The decision record that deferred it, or `—` when the deferral is a design position with no separate record. |
| `trigger` | The observable event that ends the deferral. Not a date. |
| `check` | How to tell whether the trigger has fired. A cell starting with `$ ` is a shell command that `just register-check` runs and reports; anything else is a manual check. |
| `owner` | Who answers for the row. |
| `last-checked` | The date the check was last actually run. |
| `next-check` | When it is due again. Phase-gated where the trigger is a phase; dated otherwise. |
| `status` | `open` (deferred, waiting), `watch` (the trigger may fire soon), `closed` (decided — the ADR that decided it is in the `ADR` column). |

`python3 scripts/check_register.py --lint` runs inside `just adr-lint`: it validates the dates,
checks that every referenced ADR exists, and fails when a row that is not `closed` has a
`next-check` in the past. `just register-check` runs the `$` checks and reports what they said.

**A row is closed by the ADR that decides it, never by deleting it.** Closing a row means
writing the ADR its trigger called for, or moving `next-check` forward with a reason.

| R-NN | item | ADR | trigger | check | owner | last-checked | next-check | status |
|---|---|---|---|---|---|---|---|---|
| R-01 | The deterministic fixture corpus `tests/ACCEPTANCE_PLAN.md` asks for (two-release Rust crate, two-release Python distribution with a differing import name, namespace package, extension/stub-only, stub/runtime disagreement, Sphinx inventory, changelogs) — deferred as premature at Phase 0 | — | Phase 1 begins; the corpus serves Phases 1–3 and cannot be deferred past the first gate that needs it | $ `ls tests/fixtures/` | paul-heyse | 2026-09-13 | 2026-10-15 | watch |
| R-02 | Nothing structurally ties an acceptance result to the tree that produced it: `docs/reports/logs/*.json` carry no commit hash and `generated_at` is the join time, not the test time | — | a report is cited as evidence for a phase claim without a same-session re-run, or a second person needs to verify a tally they did not produce | $ `jq -r '.generated_at' docs/reports/acceptance.json` | paul-heyse | 2026-09-13 | 2026-11-15 | open |
| R-03 | The generated Pydantic DTO does not enforce the envelope's root `allOf` conditionals, so `envelope.validate_document` validates against the generated *schema* instead | — | a `datamodel-code-generator` release emits the if/then validators, at which point the indirection can go and `test_the_generated_dto_alone_is_not_a_sufficient_validator` fails | $ `uv run datamodel-codegen --version` | paul-heyse | 2026-09-13 | 2026-12-15 | open |
| R-04 | rustdoc `format_version` drift: the five recorded nightlies span 57–61 and docs.rs may serve any of them | — | a new nightly is pinned in `config/toolchains.toml`, or docs.rs serves a format the pinned `public-api` cannot parse | $ `grep -c 'format_version' config/toolchains.toml` | paul-heyse | 2026-09-13 | 2026-10-15 | watch |
| R-05 | Native FastMCP tasks, deferred in favour of ordinary `pending` envelopes plus `job_control` | — | a client reaches protocol `2026-07-28` **and** the separate `fastmcp-tasks` package is a dependency we are willing to take | $ `uv tree --depth 1 2>/dev/null \| grep -i fastmcp` | paul-heyse | 2026-09-13 | 2026-12-15 | open |
| R-06 | The HTTP adapter and any non-loopback deployment (§12.4) | — | a requirement for use from a host that cannot reach a local Unix socket | manual: has a remote or cloud-hosted agent been asked to use this service? | paul-heyse | 2026-09-13 | 2027-03-01 | open |
| R-07 | `ty` availability as a hard prerequisite with no fallback: if it is absent the affected gates are `blocked`, and substituting another type checker is never the answer | ADR-0005 | `ty` becomes unavailable on this workstation, or a release changes the CLI/LSP boundary the producer adapter depends on | $ `ty --version` | paul-heyse | 2026-09-13 | 2026-11-15 | open |
| R-08 | Review findings are supposed to land in `rules/` as new executable oracles. The 2026-09-13 review produced the first one (`no-service-phase-claim-in-adapter`, from F3); F1, F2 and F5 named oracles that still do not exist | ADR-0009 | a design review produces a §7 finding whose Verification column says no oracle exists, and the next commit adds none | $ `ls rules/ \| wc -l` | paul-heyse | 2026-09-13 | 2026-11-15 | watch |
| R-09 | `AGENTS.md`'s nine-row binding-decision table is not the same set as blueprint §1.1: it adds §B12 and §B13 and omits §B1, §B9, §B10, §B11. Two documents state the binding set | ADR-0009 | the operator applies the replacement text in `docs/design/decisions-rule.draft.md`, pointing `AGENTS.md` at `DESIGN.md` §1.1 | manual: does `AGENTS.md` still carry its own binding table? | paul-heyse | 2026-09-13 | 2026-11-15 | open |
| R-10 | `AGENTS.md` says 46 acceptance IDs (the `acceptance-report` skill no longer hardcodes a count); `tests/gates.toml` and `STATUS.md` say 48. The two extras are P08a and P08b, registered by ADR-0005 against a frozen plan that has 46 | ADR-0005 | any further gate registration or retirement, which widens the gap again | $ `grep -c '^\[gates\.' tests/gates.toml` | paul-heyse | 2026-09-13 | 2026-11-15 | open |
| R-11 | `.claude/rules/decisions.md` — the path-scoped rule for `docs/adr/`, `docs/design/`, `docs/plans/` and `docs/design_review/` — is drafted but not installed, because `.claude/rules/` is hook-denied to a session | ADR-0009 | the operator installs it and re-records `config/guardrails.sha256` | $ `ls .claude/rules/` | paul-heyse | 2026-09-13 | 2026-11-15 | open |
| R-12 | `schemas/generated/` is committed but the `git diff --exit-code` half of `schema-conformance.sh` remains a weaker oracle than the Rust test that covers the same invariant without depending on git state | — | the emitted schema changes, or a second generated artifact is added whose only check is the git diff | $ `git ls-files schemas/generated \| wc -l` | paul-heyse | 2026-09-13 | 2026-12-15 | open |
| R-13 | The execution-policy profile is not a declared input to `SnapshotId`, though blueprint §8.2 names `policy_profile` in the dedup key. It appears covered transitively by the producer map, but that argument is a reviewer's, not the design's | — | a producer whose *output* depends on the execution profile without its *identity* doing so; or the first report of evidence gathered under a profile the caller did not select | manual: does any producer read the profile without recording it in `SnapshotInputs.producers`? | paul-heyse | 2026-09-13 | 2026-12-15 | open |
| R-14 | Whether `DESIGN.md` should carry per-section evidence labels at all, or generate them from `service_status` and `acceptance.json`. The 2026-09-13 review's §8 argues the generated form is the stronger design; the spine was already two phases stale once | ADR-0009 | the labels are found stale a second time, or a phase lands without the spine being updated in the same commit | manual: compare `DESIGN.md`'s labels against `just acceptance-report` output at the next phase gate | paul-heyse | 2026-09-13 | 2026-12-15 | open |
| R-15 | `just acceptance-check` does not refuse to promote a gate when the tree has untracked or modified source, and `acceptance.json` records no commit — so a report can grade a tree git cannot describe (review finding F2; extends R-02) | — | the next acceptance report cited as evidence for a phase claim | $ `git status --porcelain -- crates python \| wc -l` | paul-heyse | 2026-09-13 | 2026-11-15 | open |
| R-16 | `.claude/rules/process-immutable.md` lists `.claude/commands/` in its "what you can change freely" table. That directory was removed by ADR-0010; the rule file is enforcement-layer and hook-denied, so the path is allowlisted in `.claude/path-allowlist.txt` until the operator corrects the table | ADR-0010 | the operator applies the correction and re-records `config/guardrails.sha256` | $ `grep -c 'claude/commands' .claude/rules/process-immutable.md` | paul-heyse | 2026-09-13 | 2026-11-15 | open |
