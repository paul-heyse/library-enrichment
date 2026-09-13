# Plans

A plan records **how work is sequenced and verified**. A decision record
([`../adr/`](../adr/README.md)) records **what was decided and why**. The two have different
lifecycles, which is why they live in different directories.

| Plan | Covers | ADRs | Status |
|---|---|---|---|
| [01 — Port the design process](01-port-the-design-process.md) | The charter, the living design document, decision-record front matter and lint, the deferred-decision register, the review skill, and the `just` surface for all of it | ADR-0009 | done |

## Norms

- **Naming.** `docs/plans/NN-kebab-case.md`, numbered in the order they were started. Numbers
  are never reused. `just plan <slug>` creates the stub.
- **Front matter.** `title`, `status` (`draft` | `in-progress` | `done` | `abandoned`), `date`,
  `adrs` (the decision records the plan implements), and `phase` (the delivery phase from
  `DESIGN.md` §13, 0–6).
- **Living until done.** A plan is edited while it is being executed. That is the difference
  from an ADR, which is immutable once accepted.
- **Outcome.** When the work lands, append `## Outcome (recorded after implementation)` with
  three sub-headings and fill all three:
  - *What was built* — what actually exists now, with the charter §D evidence label for each
    claim.
  - *A mistake made and corrected* — at least one. A plan whose outcome records no mistake was
    either not executed or not read honestly.
  - *Deviations from the plan, deliberate* — what was done differently and why. A deviation
    that changed a decision needs an ADR, not a paragraph here.
- **Authority.** *When the code and a plan disagree, the code is what runs.* A plan is never
  cited as the reason something behaves the way it does; `DESIGN.md` and the ADRs are.
- **Plans live here**, in the repository, not in an agent's scratch directory. A plan in a home
  directory is invisible to Codex and to the next session.

## Relationship to the harness's own plan mode

Claude Code's plan mode writes to `~/.claude/plans/`, which `pre_edit.sh` exempts from the
outside-repository guard (ADR-0008). That is a scratch surface for a single session's proposal.
It is not a plan in the sense above: it is not numbered, not versioned with the code, not
readable by the other client, and it records no outcome. When a session's plan turns into work
that will land, `just plan <slug>` gives it a home here.

Register row R-11 tracks the `.claude/rules/decisions.md` entry that would state this at the
point of use; until the operator installs it, this section is where it is written down.
