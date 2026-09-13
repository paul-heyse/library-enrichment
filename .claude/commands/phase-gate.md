---
description: Run the acceptance gate for one implementation phase (0-6) and report it truthfully.
argument-hint: <0-6>
allowed-tools: Bash, Read, Grep, Glob
---

Run the acceptance gate for phase $1.

1. `just gate-phase $1`
2. Read the per-gate results it prints for this phase.
3. Report the outcome using only the four states: `passed`, `failed`, `blocked`, `not_run`.

**Do not declare the phase complete** if any gate for it is `not_run` without a stated reason,
or `passed` without a recorded command and log. Say which gates remain and what each needs.

A gate whose target does not exist yet is `not_run` — that is the correct result for an
unimplemented phase, not a problem to work around. Do not weaken a gate, relax an assertion, or
register a placeholder test to make the tally look better.

If any gate is `blocked`, name the missing prerequisite and give the exact command that would
install or enable it.

Finally, update `STATUS.md` with the dated result: the phase, the tally, the command you ran,
and the specific next unmet work.
