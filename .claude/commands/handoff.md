---
description: Rewrite STATUS.md from the actual state of the tree.
allowed-tools: Bash, Read, Grep, Glob, Write, Edit
---

Rewrite `STATUS.md` so the next session can start without re-deriving anything.

Build it from what is actually true right now, not from what this session intended:

- `just acceptance-check` for the current tally
- `git log --oneline -10` and `git status --porcelain` for what changed and what is uncommitted
- `just doctor` for which tools are genuinely present
- `docs/architecture/compatibility-matrix.md` for which pins are verified and which are not

Record:

1. **Current phase** and its gate tally.
2. **Last gate run** — the exact command, its date, and its result.
3. **Known failures and blocks**, by gate ID, each with the specific prerequisite or fix.
   Known failures stay visible; do not quietly drop one because it is inconvenient.
4. **Next unmet work** — the specific next thing, not a restatement of the phase.
5. **Pinned tool versions** actually in use.

Date every verification claim. "Verified 2026-09-13: rustup default is nightly" stays useful a
month later; "verified" does not.

Keep it short enough to read in full at the start of a session. `STATUS.md` is a handoff, not
a changelog — the git history is the changelog.
