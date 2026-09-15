# Plans

A plan records **how work is sequenced and verified**. A decision record
([`../adr/`](../adr/README.md)) records **what was decided and why**. The two have different
lifecycles, which is why they live in different directories.

**Current direction, 2026-09-14:** [Plan 12](12-architecture-first-completion.md) is functionally
complete. Native architecture and legacy decommission precede the completed Phase 4–6 capabilities.
Full implementation CI and actual producer/client checks passed. The user stopped redundant replay
after a registry-only recording correction; current-registry gate certification is not claimed.
Performance tuning is deferred. Earlier plan labels are historical; Plan 12 records final scope,
validation and deliberate deviations.

| Plan | Covers | ADRs | Status |
|---|---|---|---|
| [01 — Port the design process](01-port-the-design-process.md) | The charter, the living design document, decision-record front matter and lint, the deferred-decision register, the review skill, and the `just` surface for all of it | ADR-0009 | done |
| [2 — Close the static Rust slice](02-phase-1-closure.md) | Blueprint Phase 1 | ADR-0011, ADR-0012 | done |
| [3 — Deliver static Python evidence](03-phase-2-python.md) | Blueprint Phase 2 | ADR-0013 | done |
| [4 — Deliver comparisons and bounded research](04-phase-3-research.md) | Blueprint Phase 3 | ADR-0014, ADR-0015 | done |
| [5 — Deliver isolated semantics and durable verification](05-phase-4-verification.md) | Blueprint Phase 4 | ADR-0017 (supersedes ADR-0016) | done; gate 13/13 |
| [6 — Qualify the skill and real clients](06-phase-5-clients.md) | Blueprint Phase 5 | — | completed through Plan 12; actual client journeys passed |
| [7 — Harden publication and operations](07-phase-6-operations.md) | Blueprint Phase 6 | ADR-0018 | done; gate 4/4 |
| [8 — Resume remaining Phase 4–6](08-phase-4-6-resumption.md) | Checkpoint, commands, findings and remaining gates | ADR-0005, ADR-0017 | done; its scope was executed on 2026-09-14 |
| [9 — Complete Phase 4–6](09-phase-4-6-completion.md) | Reopened correctness and product completion requirements | ADR-0020, ADR-0021 | remaining functional scope completed through Plan 12 |
| [10 — Arrow/DataFusion target architecture](10-arrow-datafusion-architecture.md) | Hard pivot: typed evidence/catalog, batch execution, publication, clean development reset and full completion | ADR-0022–ADR-0027 | target implementation completed through Plan 12 |
| [11 — Complete architecture and remove legacy paths](11-arrow-datafusion-completion-and-legacy-removal.md) | Architecture work, explicit deletion ledger and independent completion exits; progress in the [execution ledger](11-arrow-datafusion-execution-ledger.md) | ADR-0022–ADR-0032 | remaining functional/deletion scope completed through Plan 12 |
| [12 — Architecture-first completion](12-architecture-first-completion.md) | Remaining architecture gaps and legacy removal first, then Phase 4–6 functionality and integrated acceptance | ADR-0022–ADR-0035 | done; user stopped redundant registry-only replay; [execution ledger](12-architecture-first-execution-ledger.md) |

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
