# Plans

A plan records **how work is sequenced and verified**. A decision record
([`../adr/`](../adr/README.md)) records **what was decided and why**. The two have different
lifecycles, which is why they live in different directories.

**Active pivot, remaining scope reviewed 2026-09-16:**
[Plan 15](15-unified-datafusion-delta-runtime-hard-pivot.md) consolidates the unified DataFusion/Delta
runtime review and its Delta integration follow-up. It specifies the complete native execution
pivot, Delta control/publication, Arrow producer facts, native normalization/results, CDF, retention,
and explicit legacy removal. It requires fresh state with no compatibility or historical-runtime
retention. Implementation is in progress; target acceptance remains **not_run**. Plan 14's
completed installation is the baseline, not qualification of this new target.
[Plan 16](16-unified-datafusion-delta-runtime-completion.md) reconciles the current implementation
and receipts into the remaining dependency order, with concrete architecture, deletion,
qualification and activation obligations.
[Plan 17](17-schema-governed-unified-runtime-hard-pivot.md) is the **combined execution plan**:
it carries forward all Plan 16 scope, all S01–S12 schema-review findings, and additional schema
opportunities qualified through the pinned skills, exact source and a bounded library probe.
Its 17 packages put shared semantic contracts, typed values, planning checks and generated wire
encoding ahead of their runtime consumers. Use Plan 17 to resume the remaining work; all final
target qualification remains `not_run`.

**Completed and activated hard pivot, 2026-09-15:**
[Plan 14](14-datafusion-catalog-policy-hard-pivot.md) implements the DataFusion capability/catalog
review through immutable catalog/schema providers, one declaration and policy binding path,
native statistics/coercion/index properties, and bounded diagnostics. It selects direct replacement
and deletion, with no compatibility program. The plan is **done**: P0–P9, D01–D13 and J01–J17 are closed;
all nine physical decisions are recorded. The installed candidate, real clients and independent
regression replay passed. [Qualification](../reports/plan14-final-qualification-2026-09-15.md)
records the source and explicit external-profile limits.

**Current direction, 2026-09-14:** [Plan 12](12-architecture-first-completion.md) is functionally
complete. Native architecture and legacy decommission precede the completed Phase 4–6 capabilities.
Full implementation CI and actual producer/client checks passed. The user stopped redundant replay
after a registry-only recording correction; current-registry gate certification is not claimed.
Performance tuning is deferred. Earlier plan labels are historical; Plan 12 records final scope,
validation and deliberate deviations.

**Completed and deployed, 2026-09-15:** [Plan 13](13-datafusion-research-operations-hard-pivot.md)
replaces research-operation and FastMCP delivery contracts in a hard pivot, with shared native
coverage/selection, structured diagnostics, explicit deletion and functional acceptance.
Native replacement, installed 55.1.0 family outcomes, measured operation-local reuse, actual
Codex/Claude tasks and the fresh-generation production activation passed. All 47 active acceptance
gates passed; retired P08 remains not_run. See the [execution ledger](13-research-operations-execution-ledger.md)
and [final qualification](../reports/plan13-final-qualification-2026-09-15.md).

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
| [13 — DataFusion research operations hard pivot](13-datafusion-research-operations-hard-pivot.md) | Shared scope/selection, native query integration, safe revision reachability, structured errors, durable delivery and faithful FastMCP; no compatibility paths | ADR-0036–ADR-0039 accepted | done and deployed; all required journeys and active gates passed |
| [14 — DataFusion catalog and policy hard pivot](14-datafusion-catalog-policy-hard-pivot.md) | Full immutable catalog/schema binding, shared declarations/policy, physical statistics, native string coercion, completed-index facts, bounded diagnostics and measured layout choices; explicit replacement and deletion | Retains ADR-0022–0024, ADR-0031, ADR-0033–0034, ADR-0038 boundaries; ADR-0040 accepted | done; implemented, independently verified and activated |
| [15 — Unified DataFusion and Delta runtime hard pivot](15-unified-datafusion-delta-runtime-hard-pivot.md) | Entire aggregate of F01–F14 and DFU-01–DFU-08; original destination, work packages, deletion obligations and native/installed acceptance | ADR-0041–ADR-0045 | in progress; source-integrated replacements with focused receipts; all full work packages and final qualification remain open |
| [16 — Complete the remaining unified runtime pivot](16-unified-datafusion-delta-runtime-completion.md) | Current-tree review of Plan 15; 14 remaining packages covering architecture, deletion, qualification and fresh activation | ADR-0041–ADR-0045 | planning checkpoint; execution sequence integrated into Plan 17 |
| [17 — Schema-governed unified runtime hard pivot](17-schema-governed-unified-runtime-hard-pivot.md) | All Plan 16 obligations plus S01–S12 and E01–E09: one semantic contract, typed storage/identities, native planning/admission, generated wire, complete runtime, L01–L24 removal and final qualification | ADR-0041–ADR-0045; schema/planning/wire decision updates required before implementation | draft; combined plan requested 2026-09-16; library probes recorded, product implementation and final acceptance remain open |

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
