# Execution lease and warm-session implementation review

## 1. Decision and scope

**Decision: Revise this W2 checkpoint.** The 2026-09-14 re-review closes E1 for verification
completion and deferred workspace destruction, E4 for the capacity reservation race, and E5 for
caller abort/timeout during creation. E2 still needs bounded daemon shutdown. E3 is corrected for
verification admission; semantic and local-rustdoc operation cancellation remains unwired. E6
records a newly visible recovery problem in the durable creation flag.

**Evidence strength:** Implemented / Interface-checked from current source inspection;
Tested only for the exact assertions in the inspected execution receipts below. This reviewer ran
no implementation tests or containers. The implementer's receipts record actual commands and
exit status, and the reviewer inspected their logs and corresponding test assertions. They do not
bind a source-tree digest or establish the complete W2 scope. No phase-completion claim is made.

**Target:** Plan 09 W2's continuous execution leases and warm-session capacity changes;
`execution/cleanup.rs`, Runner in `execution/mod.rs`, `lsp/mod.rs`, LSP initialization/stop,
semantic startup, verification admission and local rustdoc admission. Source references below
are relative to `crates/enrichment-daemon/src/` unless otherwise stated.

### Method and coverage

Traced admission, success, final-interest cancellation, task drop, failed removal, warm hits and
misses, creation timeout, workspace destruction and daemon shutdown. Re-read the new test
assertions and receipts. The initial review's counterexamples are superseded by the current
finding states in §7; they are not assertions that the corrected code still has those defects.

Retained-capsule integrity/single-flight, bounded scratch/reservations, qualification binding,
LSP semantic accuracy and complete W2 acceptance remain separate plan obligations. Only this
review document was edited.

## 2. Authority and lifecycle map

| Fact | Owner and implementation | Lifetime / derivation |
|---|---|---|
| Execution slot | `cleanup::Lease` owns one `OwnedSemaphorePermit` | Shared by admitted Runner and registered container; last lease drop releases the slot |
| Active container for one lease | `Lease.active: Mutex<Option<String>>` | Registration rejects reuse until removal resolves the existing name |
| Cleanup ownership | `Supervisor.outstanding` retains `Arc<Lease>` | Running, removing and abandoned stages; abandoned ownership keeps capacity reserved |
| Warm server | `Session` owns `ServedSession.guard` and its Runner lease | Survives requests; stop/eviction transfers unresolved cleanup to supervisor |
| Verification lifecycle | Rust journal plus `execute` cleanup funnel | Keeps running/cancel_requested until this lease is absent; abandonment becomes explicit Failed/POLICY_DENIED |
| Disposable workspace | Capsule/Staging/Scratch delegate to `Runner::discard_workspace` | Deferred destruction waits for the lease's container; failure retains the root |
| Container creation | Detached creator task plus durable `creation_in_progress` | Removal refuses to retire ownership while the flag is true; interrupted recovery remains E6 |

No Arc ownership cycle or conflicting lock order was found in the inspected lease paths.
Registration, resolution and cleanup waiting acquire outstanding ownership before lease state.
The supervisor's retained lease on abandonment is an intentional quarantine obligation.

## 3. Semantic contracts and invariants

| Invariant | Current mechanism | Verdict / failure behavior |
|---|---|---|
| A live container retains its original slot | Registration stores the admitted lease in outstanding ownership | Satisfied at Implemented strength |
| One lease cannot run concurrent containers | `cleanup.rs:238–273` rejects an already active lease | Satisfied at Implemented strength |
| Retry exhaustion cannot release unsafe capacity | Abandonment retains the lease and quarantines admission | Satisfied at Implemented strength; real cleanup failure oracle inspected |
| Warm hits require no second permit | Lookup precedes miss-only startup admission | Tested for worker counts 1 and 2 |
| Terminal verification cancellation implies confirmed absence | `ops/verify.rs:209–252` waits for its lease cleanup | Tested for cancellation with injected removal failure and later recovery |
| Queue cancellation does not depend on unrelated execution | `Manager::execution_lease` selects cancellation over lock/permit waits | Tested for verification via ordinary MCP; source unit oracles also exist |
| Eviction and slot reservation form one operation | Manager lock spans eviction and `supervisor.lease()` | E4 resolved at Implemented strength; adversarial contention oracle remains useful |
| Creation cannot retire ownership on transient absence | Durable creating flag blocks ordinary removal and restart reconciliation | Tested for caller abort and actual create timeout with delayed real creation |
| Shutdown has one total bound | Shutdown awaits manager before starting its deadline | Violated; E2 remainder |
| Interrupted ownership is recoverable without guessing | A true creating flag has no persisted creator identity or supported reconciliation path | E6 recovery gap |

## 4. Derivation and execution design

One semaphore is shared across execution profiles. The warm-session limit is clamped to worker
concurrency. A warm hit retains its existing lease; a miss admits once before preparation, and
the resulting server guard carries that lease through warm retention and cleanup.

Producer admission now holds the manager lock through idle eviction and actual permit
reservation. Cancellation selects across the entire admission future, including lock acquisition.
This removes the former check/acquire window. The manager still serializes queries and startup,
so shutdown must independently bound manager acquisition; bounded individual LSP exchanges do
not impose a total bound on that lock wait.

Verification's outer execution funnel waits for its own lease, rather than global idleness,
before persisting terminal status. Workspace destructors defer deletion through the same cleanup
supervisor. A cleanup failure retains uncertainty and mutable state explicitly. This does not
establish that later preparation cannot overwrite a retained same-key root; that is the separate
single-flight obligation.

Creation is supervised after its request owner disappears. Its durable flag prevents early
absence checks from releasing ownership. The flag is a safe rejection witness, but without a
recoverable creator identity it can outlive every task capable of clearing it.

## 5. Representative journeys

- **Warm reuse:** Real MCP inspection starts one ty session; a second semantic request reuses
  it with worker concurrency 1 and 2. Signature-only inspection starts no server.
- **Queued cancellation:** A heartbeat probe keeps the only execution slot. Cancelling a second
  job returns terminal cancellation within the test's two-second bound; the heartbeat advances.
- **Removal failure:** The same fixture refuses broker removal. The cancelled probe stays
  `cancel_requested`, has no terminal result, and retains its capsule and ownership record.
  Restoring removal permits terminal cancellation and eventual scratch deletion.
- **Creation abort/timeout:** A wrapper pauses before real Podman creation. The caller is
  aborted or hits the actual 15-second timeout. Ownership and capacity survive an absent-name
  check. Resumed creation is then removed, no target code starts, and capacity returns.
- **Interrupted creator:** A spawn error exits before clearing the flag; a daemon crash can
  similarly leave a true flag after its supervising task is gone. Startup refuses it forever.
  Refusal is safe, but automatic "restart to reconcile" advice is insufficient.

## 6. Acceptance gates

| Gate | Verdict | Evidence / required action |
|---|---|---|
| G1 — Authority | Pass within inspected scope | Rust owns the semaphore, cleanup registry, creation record and journals |
| G2 — Semantic fidelity | Pass for corrected verification completion | Terminal cancellation follows confirmed absence; abandonment is explicit failure |
| G3 — Validity | Pass within inspected lease registration and delayed creation | Invalid lease reuse rejects; creating ownership survives transient absence |
| G4 — Hidden behavior | Pass within inspected change | No new provider, implicit profile or target execution path introduced |
| G5 — Consistency and recovery | Fail | E2 shutdown ordering, E3 unwired operation cancellation, E6 interrupted-creator recovery |
| G6 — Transformation and reuse | Pass for capacity handoff at Implemented strength | Slot reservation occurs under the same manager lock as eviction; broader retained reuse not reviewed |
| G7 — Truthful capability claims | Unresolved for a complete bounded lifecycle claim | New exact oracles support narrower claims; shutdown/recovery and remaining W2 acceptance are open |

## 7. Principle findings

| Finding | Principle IDs | Current evidence / consequence | Required correction or disposition | Regression evidence |
|---|---|---|---|---|
| **E1 High — resolved within reviewed verification and destructor scope.** | DM-29, DM-30 | `ops/verify.rs:209–252` waits for cleanup before terminal state. `execution/mod.rs:105–149` retains workspace until cleanup succeeds and retains it on abandonment. Capsule, Staging and rustdoc Scratch call that helper. | Preserve this shared completion boundary. Retained-capsule replacement/integrity remains a separate W2 obligation; source correction alone does not certify every preparation/drop combination. | Real MCP cancellation/removal-failure fixture asserts nonterminal state, preserved root/ownership, then terminal cancellation and eventual deletion. |
| **E2 High — partially resolved; daemon shutdown ordering still violates the total bound.** | DM-29, DM-30, DM-35 | `lsp/client.rs:419–467` bounds initial write, read loop and server-request replies; cancellation is capped at 100 ms. Notifications have a five-second bound. But `server.rs:77` awaits `lsp.shutdown()` before job cancellation at `:78` and before the deadline at `:79`. Manager shutdown first awaits its mutex, which an admission waiter can hold while awaiting capacity. A blocked operation therefore delays both cancellation and the advertised shutdown clock. | Start cancellation and one absolute shutdown deadline before awaiting manager work. Bound lock acquisition and every stop under that remaining budget; transfer unresolved cleanup to retained ownership on expiry. | Duplex tests cover non-reading peers and blocked server-response writes; their new source was inspected, not run here. Add a deterministic shutdown test with manager/capacity held, plus bounded real-container shutdown verification. |
| **E3 Medium — verification admission resolved; semantic/fallback cancellation remains open.** | DM-27, DM-30, DM-35 | `lsp/mod.rs:164–188` selects real cancellation across lock and permit waits; verification supplies its job flag. `ops/resolve.rs:673–675` supplies an always-false flag and `:705` uses another for execution. `ops/semantics.rs:147/153` uses uncancellable admission and an always-false preparation flag. | Propagate operation/shutdown cancellation into semantic and local-rustdoc work as their durable operation paths are completed. Do not claim all producer admission is cancellable from the verification fix. E2 must not wait for this work before signaling shutdown. | Real MCP held-slot cancellation passes. Unit source covers a held manager lock and an occupied semaphore. Semantic/fallback final-interest cancellation oracles are still absent. |
| **E4 Medium — resolved at Implemented strength.** | DM-26, DM-35 | `lsp/mod.rs:170–177` holds the manager lock through eviction and semaphore reservation; a competing warm miss cannot take the freed slot between those steps. Warm lookup still precedes admission. | Preserve atomic reservation. No additional scheduler abstraction is required by this finding. | Real warm reuse passes at concurrency 1/2. A barrier-controlled competing warm-miss/producer fixture would strengthen starvation/progress evidence; it is not claimed by the reuse tests. |
| **E5 High — resolved for caller abort and creation timeout.** | DM-29, DM-30, DM-54 | `execution/mod.rs:159–174` supervises creation independently of the caller; `:625–629` refuses removal while creation is active. The real delayed-create fixture holds ownership through an early absent-name check and later removes the created container. | Preserve durable creating ownership until creator completion and final absence. Interrupted-daemon recovery is separated into E6 rather than treating safe rejection as a leak. | Actual broker test runs both owner abort and real 15-second timeout, resumes real creation, confirms absence/record removal/permit release and no target execution. |
| **E6 Medium — creator completion flag can strand recoverable ownership permanently. Violated recovery guarantee.** | DM-29, DM-30, DM-54 | `execution/mod.rs:166` returns immediately on `command.output().await` error, before clearing `creation_in_progress`. A spawn failure has no live creator but leaves the flag true. A crash while the task is running leaves no surviving task to clear it. `recover_owned:681–684` unconditionally refuses any such record; no creator process identity or supported quiescence reconciliation is recorded. | Distinguish definitely-not-spawned from uncertain creation and persist the safe completion result even on spawn failure. For interrupted creation, provide an ownership-bound creator identity/quiescence oracle or an explicit supported operator reconciliation that proves creation cannot still occur before checking absence. Never clear the flag based only on transient container absence. | Add a nonexistent/unexecutable broker spawn case that can recover safely, and a fresh-process restart after killing the daemon during the creation barrier. The existing test's recovery refusal occurs while the original creator task is still alive; it does not cover recovery after that task is gone. |

The applicable principles concern owned mutable state, completion meaning, cancellation,
concurrency and regression evidence. Numerical precision, storage migration, query ranking and
client evidence quality are outside this slice. These findings require behavior oracles rather
than source-shape hooks.

## 8. Alternatives and architectural leverage

| Alternative | Correctness and locality | Decision |
|---|---|---|
| Best-effort permit reacquisition during cleanup | Can lose the original admitted slot | Reject; keep continuous shared ownership |
| Shared lease and bounded global manager mutex | Small surface; serializes startup/query/eviction and capacity reservation | Suitable for this scope once shutdown and operation waits are bounded |
| Separate per-session ownership and locks | Allows unrelated queries to proceed concurrently | Consider only when required by measured concurrency needs; not necessary merely to close E4 |
| Clear creating flag on a single absent-name check | Reintroduces E5's transient-absence error | Reject; recovery must establish creator quiescence first |

## 9. Verification and measurement

Inspected receipts, both dated 2026-09-14:

- `.dev-state/phase456-completion-validation/execution-lifecycle-mcp.json` and `.log`: exact
  pytest invocation selecting the two parametrized warm-reuse cases and combined cancellation
  fixture; **3 passed in 8.77 s**. Receipt reports no new ownership records and unchanged
  preexisting records.
- `.dev-state/phase456-completion-validation/execution-creation-cleanup.json` and `.log`:
  `cargo test -p enrichment-daemon --test execution_cleanup -- --ignored --test-threads=1`;
  **4 passed in 19.98 s**. This includes delayed creation, task-drop cleanup, injected removal
  failure and pre-start cancellation/restart reconciliation. Receipt reports no new ownership
  records and unchanged preexisting records.

These are scoped execution results, not the 46-gate acceptance report. This reviewer did not
rerun them. The earlier [environment repair report](../../reports/user-session-environment-2026-09-14.md)
records prerequisites and the preceding three-case cleanup run.

A smaller existing gap remains: `cleanup::begin_removal` does not notify `changed`, although
entering whole-capacity removal changes admission to quarantine. A waiter already sleeping on
capacity therefore does not receive that state transition until resolve/abandon. Add an oracle
that starts the waiter before removal and requires the intended immediate quarantine response,
or document that queued waiters intentionally remain pending through transient removal.

Next behavioral evidence should cover E2's total shutdown bound, E3's actual operation flags,
E6's spawn-error/crash recovery, retained-root single-flight, and contention. Execute real-container
cases serially with pre/post ownership inventories. Existing green checks do not replace them.

## 10. Exceptions and unresolved decisions

No exception is granted for unbounded shutdown or silently asserting creation quiescence.
Conservative refusal of uncertain creation is appropriate, but the operator needs a concrete
recovery path. Retained exact library evidence is unaffected by this review: only regenerable
scratch is eligible for automatic deletion, and validated evidence does not expire with age.

The implementer owns the remaining corrections. This review does not change ADRs, close W2 or
alter the accepted W1 contract in the earlier completion review.

## 11. Decision and implementation changes

**Revise the bounded execution checkpoint.** Keep the corrected lease, completion, capacity and
creation ownership mechanisms. Resolve the narrower remaining shutdown/cancellation/recovery
issues before accepting the full lifecycle claim.

| Priority | Change | Principles | Acceptance evidence / regression protection |
|---|---|---|---|
| 1 | Begin cancellation and one absolute deadline before manager shutdown | DM-29, DM-30, DM-35 | Held-manager/held-capacity shutdown test plus bounded real containment cleanup |
| 2 | Make creation completion/recovery distinguish absent creator from uncertain creator | DM-29, DM-30, DM-54 | Spawn failure and fresh-process interrupted-creation recovery |
| 2 | Wire semantic/local-rustdoc operation cancellation | DM-27, DM-30, DM-35 | Cancel queued and active producer work through its actual operation owner |

No implementation files, qualification receipts or acceptance reports were changed.
