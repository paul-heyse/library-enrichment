---
name: acceptance-report
description: Regenerate and audit the acceptance gate report. Use when asked for the current gate tally, before declaring a phase complete, before any delivery report, and whenever gate results are being summarised.
allowed-tools: Bash, Read, Grep, Glob, Task
---

# Acceptance report

Regenerate the acceptance report and verify it is honest.

1. `just acceptance-report` — regenerates `docs/reports/acceptance.json` from real test output.
   Every gate not matched by an executed test defaults to `not_run`.
2. `just acceptance-check` — fails if any gate claims `passed` without a command and a log, if
   any `blocked` gate does not name its prerequisite, or if the ID set has drifted from the
   frozen `tests/ACCEPTANCE_PLAN.md`.
3. Launch the `acceptance-auditor` subagent to re-run the recorded command for every gate
   claimed `passed` and downgrade any that does not reproduce.

Report the tally as `n passed / n failed / n blocked / n not_run` out of the number of gates
`tests/gates.toml` actually registers — read it, do not carry a remembered total. The frozen
plan has 46 IDs and the registry has more, because ADR-0005 retired P08 in place and registered
P08a and P08b. A hardcoded count goes stale the first time a gate is added.

**Distinguish an unwired gate from an unverified capability.** A gate reads `not_run` when its
`tests` field is empty, whether or not tests for it exist and pass. Those are different reports
and conflating them understates working code exactly as badly as overstating it.

Do not hand-edit `docs/reports/acceptance.json`. It is generated; editing it is precisely the
untruthful-reporting failure this pipeline exists to prevent. If a gate should be passing but
is not, register its test in `tests/gates.toml` and make the test pass.

Nothing structurally ties a report to the tree that produced it — the logs carry no commit hash
and `generated_at` is the join time, not the test time (register rows R-02 and R-15). Re-run
rather than trusting a report you did not just generate.
