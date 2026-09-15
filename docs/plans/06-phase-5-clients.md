---
title: Qualify the skill and real clients
status: in-progress
date: 2026-09-13
adrs: []
phase: 5
---

# Qualify the skill and real clients

## Context

Part 5 of the user-authorized implementation of all remaining blueprint Phases 1-6.
The frozen blueprint and accepted ADRs govern; implementation is not acceptance.
Depends on Phase 4. Linux is the complete execution acceptance target.

Not started at the 2026-09-13 user-requested stop; implemented on 2026-09-14 from
[the resumption handoff](08-phase-4-6-resumption.md). The harness and the six scenarios exist and
run; the gates are **`blocked` on client credentials**, which is the truthful state and not a
pass. See the Outcome below.

## Decisions

Rust remains the sole core/publication authority. Preserve old snapshots and the frozen envelope.
Current pins remain unless verified incompatibility requires a decision. R08/P06 move to Phase 3
under ADR-0011; P08 stays retired under ADR-0005. Runtime is explicitly enabled only.
Contract or policy changes require the ADR/design-review workflow before acceptance.

## Plan

Reproducible absolute-path installation, daemon auto-start locking, controlled skill installation/update/uninstall, contract synchronization, real Codex and Claude plus separate Context7 traces covering known symbol, discovery, upgrade, disagreement, runtime Python and unavailable service. Gates A01-A06; never substitute mocked clients.

## Verification

Run affected crate Clippy/tests or Python Ruff/ty/contract tests, then `just gate-phase 5`.
Generate schemas from Rust deliberately; compare regenerated artifacts without mutating them.
Separate deterministic, live and client logs. Every recorded result identifies its source tree,
locks, executed command, time, exit status and evidence digest. Missing prerequisites remain
blocked/not_run. Full suite and independent acceptance audit precede final delivery.

## Open items

The above scope remains open until executed tests and the phase gate establish it.
No user-global client/skill installation occurs during ordinary tests. Optional HTTP, native
MCP tasks, graph/vector systems and automatic project edits are excluded.

## Outcome (recorded after implementation)

### What was built

`scripts/client-acceptance.py` and `just client-acceptance`, preview-first. `--apply` builds a
throwaway user root under `$LIBENR_HOME/clients/<timestamp>/` with `HOME`, `XDG_CONFIG_HOME` and
`XDG_DATA_HOME` redirected into it, installs the product skill, registers the service by absolute
path and Context7 as a separate connection, starts the daemon, drives the six scenarios
non-interactively through `codex exec` and `claude -p`, and writes every transcript to
`$LIBENR_HOME/clients/<timestamp>/traces/`. `tests/client/` drives that harness and
`just test-client` runs it; `scripts/acceptance-report.py` gained a `--client` input beside
`--nextest/--pytest/--live`.

The operator's real `~/.claude`, `~/.codex` and `~/.agents` are digested before and after and the
run fails if any changed — the same approach as `scripts/state-leak-check.sh`.

Four adapter defects the plan listed were also fixed: the jobs resource URI now matches the frozen
`library-evidence://jobs/{job_id}/result` template; the stale ADR-0010 reference in
`daemon_client.py` is gone; `snapshot.manifest` goes through `validate_request` like every other
method; and `service_status` now has a `StatusData` variant in `tool-data.schema.json`, so its
payload is checked against the same generated contract as every other tool's.

### The gates are blocked, not passed

**A01–A06 report `blocked`.** The sandboxed `HOME` has no credentials of its own, so neither
client can authenticate, and every scenario skips with the prerequisite named:

> no client credentials: export `ANTHROPIC_API_KEY`, or set
> `LIBENR_CLIENT_USE_OPERATOR_CREDENTIALS=1` to let the harness copy the operator's credential
> file into the throwaway user directory

`--use-operator-credentials` exists and is deliberately **opt-in, never the default**. Reusing a
personal login spends the operator's account, which is their decision and not the harness's. No
mock was substituted, and no A gate was converted into a pass — `.claude/rules/evidence-truthfulness.md`
is explicit that a mocked client is never a pass for A01 or A02.

`just gate-phase 5` therefore reports 0 passed / 0 failed / **6 blocked** / 0 not_run, and does
not pass. That is the accurate report of a real prerequisite, and it is what the phase gate is
for.

### A mistake made and corrected

None specific to this slice. The one thing worth flagging for a reader is a trap that was
deliberately avoided rather than fallen into: the sandbox inherits `ANTHROPIC_API_KEY` from the
environment if the operator exported one, but it never inherits a *credential file*. Copying that
file is what `--use-operator-credentials` does, and nothing else does it. Had the harness read
the file implicitly, the flag would have been decorative and the "throwaway `HOME`" claim false.

### Deviations from the plan, deliberate

**Transcripts go to service state, not `docs/reports/`.** The plan said
`docs/reports/clients/<timestamp>/`, and the boundary review was right that this is service
output landing inside a working repository (§2.3). The sharper reason is what a
`--use-operator-credentials` transcript contains: the complete stdout and stderr of a real,
authenticated client session, in the one directory on the machine whose contents are routinely
committed. The default is now `$LIBENR_HOME/clients/<timestamp>/traces/`; `--out` still accepts
any path, because an operator naming a destination is making a deliberate choice about their
own files.

Otherwise none. The scope is implemented as planned; only its acceptance evidence is
outstanding, and for a stated prerequisite rather than a design change.
