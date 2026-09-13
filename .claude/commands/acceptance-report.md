---
description: Regenerate and audit the acceptance gate report.
allowed-tools: Bash, Read, Grep, Glob, Task
---

Regenerate the acceptance report and verify it is honest.

1. `just acceptance-report` — regenerates `docs/reports/acceptance.json` from real test output.
   Every gate not matched by an executed test defaults to `not_run`.
2. `just acceptance-check` — fails if any gate claims `passed` without a command and a log, if
   any `blocked` gate does not name its prerequisite, or if the ID set has drifted from the
   frozen `tests/ACCEPTANCE_PLAN.md`.
3. Launch the `acceptance-auditor` subagent to re-run the recorded command for every gate
   claimed `passed` and downgrade any that does not reproduce.

Report the tally as `n passed / n failed / n blocked / n not_run` out of 46.

Do not hand-edit `docs/reports/acceptance.json`. It is generated; editing it is precisely the
untruthful-reporting failure this pipeline exists to prevent. If a gate should be passing but
is not, register its test in `tests/gates.toml` and make the test pass.
