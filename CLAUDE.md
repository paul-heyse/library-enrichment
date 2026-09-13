@AGENTS.md

# Claude harness notes

`AGENTS.md` is the canonical instruction source; this file only adds harness specifics.
Start from `STATUS.md`.

The `SessionStart` hook reports the environment and redirects service state to `.dev-state/`.
It does not install tools, activate a plan, or run CI.

`PreToolUse` hooks enforce the boundaries that have a cheap oracle: the Python semantic engine,
unpinned tool execution, writes outside the repository, user-scope configuration changes,
bare `cargo +nightly`, and banned dependency classes. A denial explains which blueprint section
it comes from — read it rather than working around it. `PostToolUse` formats and lints the
single edited file and runs the `rules/` corpus against it.

Detailed constraints are in `.claude/rules/`, scoped with `paths:` frontmatter so they load
when you touch matching files rather than every session.

Subagents: `upstream-verifier` for primary-source version checks, `acceptance-auditor` to
re-run and downgrade unreproducible gate claims, `boundary-reviewer` for diff review.
Commands: `/phase-gate`, `/adr`, `/verify-upstream`, `/acceptance-report`, `/handoff`.

Skills are discovered from `.claude/skills` (development, currently empty); Codex reads the same
source through symlinks. The shipped skill at `skills/library-research/` is the product and is
governed by the blueprint, not by this harness.
