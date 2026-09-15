# Executor and output handoff implementation review

## 1. Decision and scope

**Decision: Source findings corrected; regression validation remains unresolved.** The inspected
containment and command-mode protocol protection follow ADR-0021's accepted design. This review
found four concrete defects: late output-overflow classification, uncounted implicit output
directories, creation exceeding the host deadline, and operation records escaping their cleanup
lifetime. H1–H4 now have source corrections inspected during this review, including the adjacent
pre-registration ownership-record error path. Focused regression validation for these changes
is separate from the earlier real-container receipt; no remaining source defect is asserted by
these four findings.

**Evidence strength:** Implemented / Interface-checked for inspected source; Tested only for
the exact assertions and conditions in the inspected receipts in §9. No implementation edits,
tests or containers were run by this reviewer. The accepted proposed contract is unchanged.

**Target:** `crates/enrichment-core/src/capsule_protocol/{mod,inventory}.rs`,
`crates/enrichment-core/src/bin/library-enrichment-executor.rs`, and
`crates/enrichment-daemon/src/execution/{mod,handoff,budget}.rs` on 2026-09-14.

### Method and coverage

Traced host operation construction, input capture/copy, actual mount flags, executor PID and
proc-FD protection, child wait/output races, descendant quiescence, frame validation, quarantine
writes, host deadlines, cleanup, promotion and operation-record recovery. Inspected relevant
negative parser tests and the real boundary fixture's assertions and logs.

Known incomplete work remains explicitly outside any closure claim: reservation coverage of
initial host preparation/generation replacement, shared Rust broker/setup/qualification binding,
and W3 operation cancellation propagation. These are acknowledged work, not new findings here.
The current host generation promotion is staging code and is not accepted as ADR-0021's final
atomic publication implementation. No W2 or phase-completion claim is made.

## 2. Authority and lifecycle map

| Fact | Inspected owner | Boundary / lifetime |
|---|---|---|
| Operation identity | Host-generated `Operation::id()` | Exact argv, input inventory, output roots, limits and host binding |
| Input bytes | Host inventory plus read-only `/inputs` mount | Executor checks full inventory before making its private scratch copy |
| Protocol authority | Command-mode executor PID 1 | Target stdout/stderr use separate pipes; helper proc-FD/memory access is restricted |
| Container ownership | Existing continuous lease and guard | Retains capacity until absence or explicit quarantine |
| Target completion | Executor wait and bounded readers | Host treats this as a report, separately enforcing deadline and cleanup |
| Stable output set | Executor namespace quiescence | Only executor remains before inventory/read |
| Captured host bytes | `handoff::receive` and private reservation root | Host checks allowed paths, sizes, modes, contents and terminal inventory |
| Operation document | `OperationRecord` local owner, then registered container cleanup | Pre-registration errors remove operation/fresh ownership records; recovery removes the matching operation after absence |

## 3. Semantic contracts and invariants

| Invariant | Source mechanism | Current assessment |
|---|---|---|
| All command-mode target writes stay off host input trees | Read-only root/input/operation/helper mounts; explicit `/capsule` tmpfs; automatic tmpfs/image volumes disabled | Actual boundary receipt inspected; exhaustive platform qualification remains open |
| A target cannot write directly to helper protocol stdout through `/proc/1/fd/1` | `PR_SET_DUMPABLE=0` before spawning target; separate target pipes | Source/interface checked and this exact denied-open assertion exists in the passing real fixture |
| Main-child exit cannot leave a writer during capture | PID-1-only kill/reap loop plus `/proc` inventory; bounded failure | Passing detached-child fixture inspected; repeated fork/mutation stress during capture remains useful |
| Returned paths cannot escape or silently expand directory ownership | Canonical paths, output-root allowlist, bounded count, explicit parent validation after H2 fix | Corrected at Implemented strength |
| Actual overflow remains an output-limit observation | Normalize final exit_code after readers join | Corrected at Implemented strength; H1 race oracle remains required |
| Expired host deadline cannot authorize target start or promotion | Create timeout capped by host deadline, pre-start check, post-capture override after H3 fix | Corrected at Implemented strength; delayed-create oracle remains required |
| Every temporary operation document has an owner through deletion | OperationRecord before registration; container/restart cleanup afterward | H4 source correction inspected; fault/inventory regression evidence pending |

These conclusions apply to the command executor. They do not assert that a later language-server
`exec` preserves the command helper's process attributes, or certify semantic-server result
integrity against arbitrary code in that server's container.

## 4. Derivation and execution design

The helper verifies PID 1 and the matching procfs view before any target spawn or namespace-wide
signal. It disables dumpability, validates the bounded operation and compares its input
inventory. Targets receive separate stdin/stdout/stderr bindings; control-looking target output
is serialized as data in the completion frame.

The executor reaps its direct child before namespace-wide reaping, preserving the observed
exit result. It kills/reaps remaining descendants and checks that `/proc` exposes only PID 1
before inventory/read. Returned files must be regular, single-linked, bounded and digest-stable.
The host independently verifies their sizes/digests and one matching terminal inventory.

Host-created quarantine is never mounted. Cleanup is independent of protocol completion;
only a successful capture with confirmed cleanup enters the current staging promotion path.
The host checks for trailing stream bytes after container/broker teardown. Failed or partial
capture cannot publish a success. Durable final-generation promotion and complete reservation
accounting remain the implementer's acknowledged next work.

## 5. Representative journeys

- **Ordinary capture:** A target changes only its scratch copy, emits one selected file and
  leaves a detached child holding stdout. The actual fixture requires the child to be reaped,
  input bytes unchanged on the host, output captured and container cleanup confirmed.
- **Fast output overflow:** A finite burst may finish before the reader thread signals overflow.
  The original finalizer produced `OutputLimit` with `Some(0)`; the host correctly rejected that
  contradictory frame. H1 now normalizes the outcome after joining readers.
- **Deep output tree:** A malicious stream could formerly omit intermediate directory entries
  and cause `create_dir_all` to allocate them without counting/manifesting them. H2 permits only
  bounded ancestors implied by host-selected roots; container-selected parents need prior entries.
- **Delayed creation:** A short configured deadline could previously expire while the separate
  15-second create wait continued, followed by target start. H3 now prevents this and prevents a
  ready terminal frame from winning over an already expired host deadline.
- **Refused admission:** Previously the operation document was written before reservation. A
  capacity refusal left it without a container guard. Admission failure now creates no document;
  later failures use local RAII cleanup until registration transfers ownership (H4).

## 6. Acceptance gates

| Gate | Verdict | Evidence / remaining action |
|---|---|---|
| G1 — Authority | Pass in inspected scope | Host owns request, bounds, validation and cleanup; executor supplies scoped reports |
| G2 — Semantic fidelity | Pass at corrected source strength | H1 normalizes output-limit reporting; process, protocol and cleanup remain separate |
| G3 — Validity | Unresolved for corrected regression validation | H2 restores parent/count validation in source; its exact missing-parent/count oracle has not yet been inspected as executed |
| G4 — Hidden behavior | Pass in inspected containment path | Explicit read-only inputs/tmpfs and output selection; no new target host-write route found |
| G5 — Consistency and recovery | Unresolved for corrected regression validation | H3/H4 source defects are corrected; delayed-create and pre-registration/recovery inventory oracles remain to be validated |
| G6 — Transformation and reuse | Unresolved beyond this narrow boundary | Final generation publication and complete reservation/reuse obligations are known incomplete |
| G7 — Truthful capability claims | Pass only for the stated checkpoint | Prior actual tests are named; source corrections and unrun regressions are not treated as full qualification |

## 7. Principle findings

| Finding | Principles / verdict | Concrete evidence and consequence | Correction / disposition | Regression oracle |
|---|---|---|---|---|
| **H1 Medium — output-overflow/child-exit race. Source correction inspected.** | DM-07, DM-08, DM-30; Satisfied at corrected source strength | Executor waits for direct child before readers finish. The original final reader overflow update changed only `end`, leaving `Some(exit_code)`; daemon rejects this combination as an invalid completion. A real limit event therefore became a protocol failure. | Executor now clears exit_code for final non-Exited outcomes and handles the child-exit/kill race. Preserve this normalization after all reader outcomes are known. | **Unit or controlled process fixture:** delay reader overflow publication until after child-exit observation; require accepted `OutputLimit`, bounded captured bytes and no retained outputs. Infinite-output stress alone does not force this schedule. |
| **H2 High — implicit parent directories bypassed host inventory/count bounds. Source correction inspected.** | DM-07, DM-29, DM-35, DM-45; Satisfied at corrected source strength | Original `handoff.rs:80–84` used `create_dir_all` for each container-selected file path. A declared directory root followed by deeply nested files could create many undeclared, uncounted directories while terminal inventory validation still passed. | Current reader precomputes/counts only ancestors implied by host-selected output roots. Every other parent must be an earlier Directory entry. This preserves legitimate nested file roots used by rustdoc. | **Parser fixture:** declared root plus nested file with missing intermediate directory rejects before creating it; explicit intermediates pass; nested host-selected single-file root passes; combined implicit/explicit count is bounded. |
| **H3 High — creation could outlast the host deadline and still start target code. Source correction inspected.** | DM-28, DM-30, DM-35; Satisfied at corrected source strength | Original `mod.rs:612/641/666–685` started the deadline but waited a separate 15 seconds for creation and checked only cancellation before target start. A ready capture could also beat an expired deadline in `select!`. | Create now uses the earlier of host deadline and 15 seconds; target start checks expiry; post-capture expiry overrides completion before cleanup/promotion. No success may supersede a host timeout. | **Actual broker barrier:** configure a short deadline, delay creation beyond it, later resume the creator and prove target marker never appears; also make deadline and terminal completion ready together and require Deadline/no promotion. |
| **H4 Medium — operation documents could lose cleanup ownership. Source correction inspected.** | DM-29, DM-30, DM-35; Satisfied at corrected source strength | The original `operation()` wrote the document before reservation/guard, and startup recovery deleted owned records without their operation documents. Repeated capacity refusals therefore accumulated unowned files. A related registration failure could leave a fresh `creation_in_progress=true` owned record even though no creator had started. | Current operation construction does not write a document; reservation/argument checks precede `OperationRecord::write`. Local RAII owns the document and fresh ownership record until guard registration transfers them. Startup recovery deletes the matched operation only after confirmed container absence. | **Deterministic lifecycle fixture:** repeated reservation/argument/registration failures leave operation and fresh owned-record inventories unchanged; interrupted owned-container recovery removes only its own matching operation document; delayed creation retains the document until creator quiescence/absence. |

**Applicability:** Validation, effect authority, mutable ownership, partial failure, concurrency,
resource bounds and truthful observations are directly applicable. Numerical semantics, public
query design, snapshot migrations and semantic evidence accuracy are outside this checkpoint.
No parser mock constitutes real containment proof.

## 8. Alternatives and architectural leverage

| Choice | Assessment |
|---|---|
| One command executor with a host-owned bounded protocol | Retain; it provides a small, inspectable capture boundary without a workflow platform |
| Infer completion from EOF or broker exit | Reject; helper lifetime deliberately extends beyond target completion |
| Permit arbitrary implicit parent creation | Reject; declared directory ownership/counts are cheaper and safer than post-hoc reconciliation |
| Separate operation-document RAII ownership before container registration | Preferred local correction for H4; no new scheduler or datastore is required |

## 9. Verification and evidence limits

Inspected `.dev-state/phase456-completion-validation/execution-scratch-boundary.{json,log}`:
actual `cargo test -p enrichment-daemon --test execution_boundary -- --ignored --test-threads=1`
completed with **4 passed in 13.60 s**, dated 2026-09-14. Receipt records no new owned records and
unchanged preexisting records. Its tests cover isolation/resource stress, ordinary timeout and
cancellation, selected output, read-only inputs, denied `/proc/1/fd/1` access and detached-child
cleanup. The earlier `executor-handoff-first.{json,log}` records the isolated handoff test.
These receipts precede the review-driven H1–H4 corrections and do not validate their exact races.
The reported nested-MCP stack-buffer correction is present (`handoff.rs` uses a heap Vec). The
subsequent `execution-scratch-retained-fixed.{json,log}` receipt was inspected: its one selected
retained-capsule restart/integrity MCP test passed in 11.99 s, dated 2026-09-14, with ownership
inventories unchanged. This supports that exact rerun, not all MCP or new H1–H4 paths.

Checked primary Linux interface documentation on 2026-09-14:
[PR_SET_DUMPABLE](https://man7.org/linux/man-pages/man2/PR_SET_DUMPABLE.2const.html),
[proc PID file descriptors](https://man7.org/linux/man-pages/man5/proc_pid_fd.5.html), and
[PID namespaces](https://man7.org/linux/man-pages/man7/pid_namespaces.7.html).
These support the intended command-mode process boundary; the actual denied-open fixture is the
behavioral evidence for this host. They do not prove all future runtime/kernel configurations.

Run the named H1–H4 regressions and rerun affected actual boundary tests after fixes. Keep the
broader qualification/reservation/publication gates separate. This reviewer ran none of them.

## 10. Exceptions and open scope

No exception is granted for unowned accumulating operation documents or resource-bound bypass.
H1–H4's inspected corrections do not establish their unrun regressions or imply that the known
incomplete W2/W3 work is finished. The user-approved indefinite evidence retention rule is unchanged; these operation
records are temporary execution state, not retained library facts.

## 11. Decision and required changes

**Revise the validation checkpoint before implementation acceptance.** All four source findings
are corrected within the inspected scope. Establish their regression evidence before treating
this narrow executor lifecycle as accepted; this decision requests no architectural replacement.

| Priority | Change | Principles | Required evidence |
|---|---|---|---|
| 1 | Preserve H2/H3 validation and deadline fixes | DM-07, DM-29, DM-30, DM-35 | Missing-parent/count and delayed-create/no-start oracles |
| 2 | Preserve H1 final outcome normalization | DM-07, DM-08, DM-30 | Finite overflow/exit race fixture |
| 2 | Own operation files before registration and through restart cleanup | DM-29, DM-30, DM-35 | H4 early-failure and recovery inventory oracles |

Only review documentation was changed. ADR-0021's scoped proposed-design acceptance remains;
this document is an implementation checkpoint, not W2 or phase acceptance.
