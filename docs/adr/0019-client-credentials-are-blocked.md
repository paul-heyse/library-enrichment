---
id: ADR-0019
title: Absent client credentials report blocked, not not_run
status: accepted
date: 2026-09-14
deciders: [paul-heyse]
level: should-deviation
principles: [DM-08, DM-30, DM-59]
design: [§13, §14.2]
review: not-required: A reporting-vocabulary decision between two frozen documents that disagree on one word. It changes no code path, no producer and no policy; it changes which of four permitted states one unattempted-for-a-reason gate reports.
evidence: Tested
supersedes: []
superseded-by: null
revisit: A third state is needed for "attempted, prerequisite absent", or the frozen plan is ever reissued.
verification: tests/client/test_client_acceptance.py skips with a named prerequisite; `just acceptance-check` refuses any `blocked` gate whose `limitation` is empty; `just gate-phase 5` reports blocked and exits non-zero.

---

# ADR-0019: Absent client credentials report `blocked`, not `not_run`

## Context

Two frozen documents give different answers for one case.

`tests/ACCEPTANCE_PLAN.md` (frozen, digest-verified) says, in "Required test report":

> Mark tests `passed`, `failed`, `blocked`, or `not_run`. Client credentials not available means
> `not_run`, not passed via a mocked client.

`AGENTS.md` (also frozen) says:

> A missing tool, absent credentials, or an unsupported platform is `blocked` with the
> prerequisite named. Never attempted is `not_run`.

Gates A01–A06 are exactly this case. The client-acceptance harness runs, checks its
prerequisites, finds no credentials in the throwaway `HOME`, and skips. The 2026-09-14 audit
found the report saying `blocked` where the frozen plan's sentence says `not_run`, with nothing
recording the choice.

## Scope

This binds which of the four permitted states an *attempted* gate reports when its prerequisite
is absent. It does not add a state, change the four-state vocabulary, or touch any gate's
scenario or assertion. It does not weaken the rule both documents actually agree on, which is the
one that matters: **a mocked client is never a pass.**

## Drivers

- **The two documents agree on the danger and differ on the label.** The plan's sentence draws
  its contrast against "passed via a mocked client", not against `blocked`. Both states are
  non-passes.
- **An operator needs to know which one to act on.** `not_run` and `blocked` send different
  people to different places: nobody ran it, versus somebody ran it and it needs a credential.
- **`blocked` is mechanically stronger here.** `scripts/acceptance-check.py` refuses a `blocked`
  gate whose `limitation` is empty; it demands nothing of a `not_run` gate. Choosing `blocked`
  means the reason must be written down or the check fails.

## Options

- **Report `not_run`, following the plan's literal sentence.** Loses the distinction between
  "nobody tried" and "tried, needs a credential", and loses the mechanical requirement to name
  the prerequisite. Rejected.
- **Report `blocked`, following AGENTS.md (taken).** Keeps both facts and is checkable.
- **Add a fifth state.** Four states, and only four, is a stated invariant of the whole reporting
  discipline. Rejected without further thought.
- **Do nothing and leave the choice unrecorded.** What the audit found. Rejected: a silent
  reinterpretation of a frozen document is the failure the deviation process exists to prevent.

## Decision

A gate whose test **ran and skipped on a named, absent prerequisite** reports `blocked`, with the
prerequisite in its `limitation`. A gate whose test was never executed reports `not_run`. That
applies to client credentials exactly as it applies to a missing binary or an unsupported
platform, which is what makes it a rule rather than an exception for A01–A06.

The frozen plan's sentence is read as forbidding the *mocked pass*, which this preserves
absolutely, rather than as assigning a label to a case its surrounding paragraph does not
otherwise distinguish.

### Consequences

Easier: `just gate-phase 5` says what to do, and `acceptance-check` will not let the reason go
unwritten.

Harder: the report no longer reads word-for-word off the frozen plan on this one point, so a
reader comparing them finds a difference and has to come here. That cost is why this record
exists; the alternative was the same difference with nothing to find.

Nothing is foreclosed. Reversing this is a one-line change in
`scripts/acceptance-report.py`'s status mapping.

### Compensating controls

`tests/client/test_client_acceptance.py` has no mock fallback — the fixture skips, and the module
docstring says why. `scripts/acceptance-check.py` fails any `blocked` gate with an empty
`limitation`. `just gate-phase 5` counts `blocked` as incomplete and exits non-zero, so this
choice cannot make a phase look finished.

## Evidence

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| The frozen plan says `not_run` for absent client credentials | `tests/ACCEPTANCE_PLAN.md`, "Required test report" | 2026-09-14 | "Client credentials not available means `not_run`, not passed via a mocked client." |
| The frozen agent instructions say `blocked` for absent credentials | `AGENTS.md`, "Truthful reporting" | 2026-09-14 | "A missing tool, absent credentials, or an unsupported platform is `blocked` with the prerequisite named. Never attempted is `not_run`." |
| `blocked` is the state the checker constrains | `scripts/acceptance-check.py` | 2026-09-14 | `if status == "blocked" and not g.get("limitation")` → "marked 'blocked' without naming the missing prerequisite" |
| The divergence was found by audit, not asserted | `acceptance-auditor` re-run of all 48 gates | 2026-09-14 | "this is a deliberate house rule — but it reinterprets a manifested file (digest verified, 15/15) with no ADR recording it" |

## Verification

`just acceptance-check` fails if any `blocked` gate carries no `limitation`, which is the
mechanical half. `just gate-phase 5` prints the six A gates with their prerequisite and exits
non-zero. `tests/client/test_client_acceptance.py` is the test that skips; it drives the real
harness and has no recorded or mocked path, so the only way it reports a pass is a real client
run.

## Boundaries preserved

§B1–§B13 unchanged. No producer, policy, schema or evidence-model change; no code path differs.
The four-state vocabulary is unchanged and no gate ID is renumbered, retired or invented. The
rule both frozen documents share — a mocked client is never a pass for A01 or A02 — is preserved
exactly.

## More information

- `docs/plans/06-phase-5-clients.md`, "The gates are blocked, not passed".
- `STATUS.md`, "Phase 5 is blocked, and on what".
- Register row R-10 tracks the related count divergence between `AGENTS.md` and `gates.toml`.

## Status history

- 2026-09-14 — accepted, after the acceptance audit found the divergence unrecorded.
