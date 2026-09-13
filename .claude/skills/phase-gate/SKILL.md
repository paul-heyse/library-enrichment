---
name: phase-gate
description: Run the acceptance gate for one implementation phase (0-6) and report it truthfully. Use when asked whether a phase is complete, to run or re-run a phase gate, or to report a gate tally.
allowed-tools: Bash, Read, Grep, Glob
---

# Phase gate

**Argument:** the phase number, 0–6.

1. `just gate-phase <n>`
2. Read the per-gate results it prints for this phase.
3. Report the outcome using only the four states: `passed`, `failed`, `blocked`, `not_run`.

**Do not declare the phase complete** if any gate for it is `not_run` without a stated reason,
or `passed` without a recorded command and log. Say which gates remain and what each needs.

A gate whose target does not exist yet is `not_run` — that is the correct result for an
unimplemented phase, not a problem to work around. Do not weaken a gate, relax an assertion, or
register a placeholder test to make the tally look better.

A gate can also read `not_run` because its `tests` field in `tests/gates.toml` is empty even
though tests for it exist and pass. That is an unwired gate, not an unverified capability, and
the two are different reports. Say which it is.

If any gate is `blocked`, name the missing prerequisite and give the exact command that would
install or enable it.

Finally, update `STATUS.md` with the dated result: the phase, the tally, the command you ran,
and the specific next unmet work.
