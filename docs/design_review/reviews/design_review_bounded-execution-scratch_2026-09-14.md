# Bounded execution scratch and output handoff design review

## 1. Decision and scope

**Decision: Accept the scoped ADR-0021 design at Proposed/Interface-checked strength.**
Read-only admitted inputs, one explicitly bounded tmpfs, a small Rust executor and host-side
quarantine form a coherent implementation route. The 2026-09-14 amendment resolves S1–S3:
target quiescence before capture, conservative reservation/generation recovery, and complete
preparation invalidation are now explicit. All seven gates pass for this proposed contract;
implementation and real containment acceptance remain open.

The subsequent [executor implementation checkpoint](design_review_executor-handoff_2026-09-14.md)
records source findings and their corrections separately from this proposed-contract acceptance.

**Evidence strength:** Proposed, with the selected Podman mount behavior Interface-checked
against its exact 4.9.3 documentation on 2026-09-14. This review assesses whether the contract
is sufficiently decidable to implement; it does not certify containment or scratch behavior.
No implementation files or ADRs were edited and no tests or containers were run.

**Target:** [ADR-0021](../../adr/0021-bounded-execution-scratch.md), Plan 09 W2, and completion
review F3/F4. The observable outcome is bounded target writes without writable host capsule
mounts, validated reusable preparation, and retained results admitted only after cleanup.
The protocol is an internal execution boundary; the public MCP contract and execution-profile
permissions remain unchanged.

### Method and coverage

Read ADR-0021, relevant completion/lease findings, Runner containment arguments, current Python
preparation, the updated lease/retained-input paths, and the charter/directive/addendum/template.
Traced ordinary preparation, hostile or malformed output, a surviving target descendant,
replacement while capacity is nearly full, helper changes, cancellation and crash recovery.

Verified the exact upstream claim governing automatic writable mounts. Did not audit every
Podman/runtime implementation detail, package output format, protocol implementation or broker
qualification command. Those require implementation review and the named real execution gates.

Related source corrections were inspected: `server.rs:74–86` starts one shutdown deadline and
requests job cancellation before manager shutdown; `lsp/mod.rs:192–205` bounds lock acquisition
and stopping together. `execution/mod.rs:175–185` records failed spawn as no creation in progress;
`:708–717` permits uncertain-creator reconciliation only after a different boot. PID/boot
identity is recorded for diagnosis. E2's source defect and E6's spawn-error defect are corrected;
E6's automatic recovery scope is explicitly narrowed to a different boot. Same-boot uncertainty
fails closed. The per-key file lock retained by `cleanup::Lease` also prevents another admitted
preparation from overwriting a root while cleanup still owns its lease. These source findings do
not promote the implementer's reported 100-unit-test run to independently inspected evidence.
W3 semantic/local-rustdoc cancellation propagation remains open.

## 2. Authority and lifecycle map

| Concept | Authority / identity | Lifetime and representation |
|---|---|---|
| Execution policy | Rust normalized broker description | Concrete paths, environment, mounts and limits; setup/service/qualification derive from it |
| Preparation request | Rust operation record and preparation key | Binds selected source/environment/image/configuration and allowed outputs |
| Admitted inputs | Host-only exact inventory manifest | Read-only container mount; changes invalidate reuse |
| Execution workspace | One continuous lease and bounded tmpfs | Writable only during the owned container lifetime |
| Executor | First-party immutable binary digest | Internal implementation of a concrete operation; does not authorize new effects |
| Captured output | Untrusted framed bytes in host-private quarantine | Host validates paths, limits, digests and required outputs; no direct publication |
| Admission/publication | Rust host after confirmed cleanup | Atomically replaces one manifest-bound generation, retaining old valid state until commit |
| Storage capacity | Serialized host inventory and reservations | Counts retained/quarantine/new generations including overlap; restart reconstructs conservatively |
| Qualification | Receipt binding root/images/runtime/helper/containment | Changed binding invalidates readiness; failed requalification cannot preserve old admission |

The executor's report is producer evidence. Host-selected operation, image, policy, input and
attempt identities remain authoritative and must not be replaced by payload-supplied values.
Process completion, protocol completion and container absence are distinct facts.

## 3. Semantic contracts and invariants

| Invariant | Proposed enforcement | Failure behavior / assessment |
|---|---|---|
| Target writes cannot grow a host capsule | Read-only root/inputs, disabled automatic tmpfs, ignored image volumes, explicit bounded `/capsule` tmpfs | Exhaustion fails; no writable host fallback; Proposed |
| Runtime-managed writable mounts do not bypass the declared policy | `--read-only-tmpfs=false` with read-only root | Podman 4.9.3 documents `/dev` and `/dev/shm` read-only too; Interface-checked, real mount inventory still required |
| Container output cannot directly write admitted host state | Bounded daemon parser into an unmounted private quarantine | Invalid paths/kinds/counts/frames reject; Proposed |
| Captured files represent a coherent producer outcome | Executor must be the only live container process before inventory/read, after bounded termination/reaping | Failure rejects; S1 resolved at Proposed strength |
| Partial output cannot become reusable input | Cleanup and full validation before one manifest-bound atomic generation replacement | Old valid generation survives until commit; S2 resolved at Proposed strength |
| Storage reservations are a hard aggregate limit | Reserve before host writes; count retained/quarantine/new overlap; reconstruct from disk | Failure/cancellation releases only after quiescence and actual deletion; S2 resolved |
| Meaningful executor/preparation changes invalidate reuse | Key explicitly includes helper/protocol and normalized operation/containment semantics | Changed helper cannot reuse the old preparation key; S3 resolved |
| Retained input integrity survives warm reuse | Exact manifest and per-key lease lock | Invalid inputs rebuild; active/uncleaned roots cannot be overwritten; Proposed |

Unknown output kind, incomplete stream, failed producer, resource exhaustion, failed validation
and unconfirmed cleanup must stay distinguishable from a valid empty output. A digest proves
the exact captured bytes, not that a library claim is correct or that concurrent files formed
one consistent outcome.

## 4. Derivation and execution design

The host selects a concrete operation and reserves its writable host storage. Admitted inputs
and operation metadata enter the container read-only. The executor copies them into bounded
scratch and runs the selected target. Normal probes retain bounded observations; preparation
and rustdoc alone return declared file outputs. LSP mode replaces the executor after scratch
initialization, preserving its ordinary protocol rather than multiplexing a second control
language into LSP traffic.

The handoff reader must validate framing and bounds before allocating or writing the declared
size, compute content digests itself, and keep its host-selected bindings. A complete terminal
record, a failed protocol, a timed-out target and successful cleanup must remain separately
observable. The existing operation/cleanup deadlines must also bound handoff and descendant
termination; a helper deliberately remaining alive must not imply waiting for EOF.

The amended contract requires a stable producer output set before declared regular
files/directories are captured. The daemon validates quarantine and confirms whole-container
absence before atomically publishing its manifest-bound generation. Retained, quarantine and
replacement-overlap bytes remain counted; restart reconstructs conservative accounting from
disk. No generic scheduler, shell tool, archive extractor or new publication authority is needed.

## 5. Representative journeys

- **Python preparation:** Existing code uses `uv pip install --target=/capsule/python` with
  `--link-mode=copy`, not a symlinked venv. Returning its bounded target directory can preserve
  the proposed no-link handoff rule. A package that requires unsupported link output fails
  explicitly; the handoff must not silently dereference arbitrary paths.
- **Local rustdoc:** Declared JSON output is captured while tmpfs exists, retained in host
  quarantine, and admitted after cleanup. Its content/schema/provenance validation remains
  separate from transport acceptance.
- **Surviving descendant:** The main command exits while a child alternates a lockfile and
  package contents. Per-file digests can all pass while the captured tree mixes generations.
  The amended contract requires bounded descendant quiescence before capture (S1).
- **Replacement near capacity:** An old retained tree remains valid while a replacement fills
  quarantine. The amended accounting rule counts this overlap and reconstructs occupied bytes after a crash;
  atomic replacement preserves the old valid generation until commit (S2).
- **Helper requalification:** A changed executor may alter preparation/copy semantics while the
  release, image and declared environment remain fixed. The amended retained key includes helper/protocol identity and therefore changes without
  relying on a separately maintained preparation-version constant (S3).

## 6. Acceptance gates

| Gate | Verdict | Evidence / required action |
|---|---|---|
| G1 — Authority | Pass at Proposed strength | One Rust broker/policy owner; host controls inputs, parser, reservations and publication |
| G2 — Semantic fidelity | Pass at Proposed strength | S1 requires quiescence; host-bound identities, executor observations, terminal completion and host cleanup are separate facts |
| G3 — Validity | Pass at Proposed strength for declared parser boundary | Exact allowlisted outputs, host limits, invalid kinds/paths/streams rejected, private quarantine and cleanup prerequisite |
| G4 — Hidden behavior | Pass at Proposed strength | Writable scratch and explicit handoff are declared effects; repository/home mounts remain excluded |
| G5 — Consistency and recovery | Pass at Proposed strength | S1 rejects live mutation during capture; S2 defines conservative recovery, actual-byte accounting and atomic manifest-bound promotion |
| G6 — Transformation and reuse | Pass at Proposed strength | S3 includes helper/protocol/normalized operation dependencies; exact manifest and lock validation govern reuse |
| G7 — Truthful capability claims | Pass for a proposed implementation route | Exact Podman mount behavior was checked; real qualification remains explicitly unclaimed |

## 7. Principle findings

| Finding | Principle IDs / verdict | Concrete gap and consequence | Proposed correction | Verification oracle |
|---|---|---|---|---|
| **S1 High — resolved by explicit quiescence and completion boundaries.** | DM-07, DM-14, DM-29, DM-35; Satisfied at Proposed strength | ADR-0021:75–98 requires the executor to be the sole remaining live process after bounded descendant termination/reaping before inventory/read. It separates host-bound inputs, executor-reported outcomes, exactly one matching terminal frame and host deadlines/cleanup. The former mixed-live-capture ambiguity is closed. | Implement these rejection boundaries; a main-child exit alone must never authorize capture. Terminal framing does not supersede host timeout or cleanup failure. | **just gate / actual executor fixture:** a child outlives its parent and mutates related output files or repeatedly forks. Require quiescence before capture or explicit rejection, no mixed admitted generation, and cleanup within the declared bound. |
| **S2 High — resolved by conservative accounting and atomic manifest-bound generation publication.** | DM-14, DM-29, DM-30, DM-35; Satisfied at Proposed strength | ADR-0021:112–121 counts retained/quarantine/new generations including overlap, reserves before host writes, reconstructs from disk on restart, and releases occupied space only after safe actual deletion. An old valid generation survives until atomic replacement after cleanup. | Implement one coherent generation boundary and conservative crash recovery. An in-memory token drop cannot authorize more bytes while its occupied storage remains. | **just gate / concurrent and crash fixtures:** nearly-full capacity with two replacements; interruption before/after manifest/rename and reservation updates; require no budget overshoot and either old or new complete generation on restart. Unreadable inventory fails closed. |
| **S3 Medium — resolved by explicit helper/protocol/operation invalidation.** | DM-15, DM-31, DM-32, DM-48; Satisfied at Proposed strength | ADR-0021:102–110 explicitly includes helper/protocol identity and normalized operation/containment semantics in the retained key; requalifying a changed helper cannot select the old preparation under the same key. | Derive the key from this one semantic projection. Keep run IDs/timestamps out, and bind warm reuse to the exact validated manifest. | **unit/metamorphic fixtures and real reuse:** change each meaningful helper/protocol/policy field and require a miss; reorder equivalent configuration and require the same key; warm reuse must bind the same concrete preparation manifest. |

**Applicability:** Identity, validation, mutable ownership, effects, concurrency, trust,
provenance, reproducibility and adversarial verification directly apply. Numerical semantics,
query planning/ranking and cross-backend query transformations are outside this internal
execution change. The findings cross runtime/disk lifetimes and need behavior tests; a
source-shape hook would not prove them.

## 8. Alternatives and architectural leverage

| Alternative | Correctness / locality | Cost and decision |
|---|---|---|
| Writable host capsule plus preflight measurement | Does not enforce growth during target execution | Reject |
| Explicit bounded tmpfs plus one small executor protocol | Centralizes copy/run/capture; host remains authority and owns validation | Preferred after S1–S3; actual copy/memory cost unmeasured |
| One regular output file per operation over the same bounded protocol | Simplest capture form for rustdoc and ordinary observations | Prefer wherever sufficient; bounded directory output is justified by installed Python/Rust preparation trees |
| Host loopback filesystem/quota machinery | Can provide a different hard storage bound but needs additional privileged setup and lifecycle | Defer; not necessary to make this design viable |

A private concrete operation enum and framing format are sufficient. The need to copy several
files does not justify a general workflow language or generic filesystem manipulation API.

## 9. Verification and measurement plan

| Claim | Current evidence | Required oracle |
|---|---|---|
| Automatic writable paths are disabled | Exact [Podman 4.9.3 read-only-tmpfs documentation](https://docs.podman.io/en/v4.9.3/markdown/podman-run.1.html#read-only-tmpfs), retrieved 2026-09-14 | Actual mount inventory and failed writes to inputs, root, `/dev/shm`, `/tmp`, `/run` and `/var/tmp`; explicit scratch alone writable |
| Tmpfs bounds target writes | Proposed | Filesystem type, near-bound growth, inode/file exhaustion, OOM and stdout flood; no host-storage fallback |
| Handoff rejects hostile output | Proposed | Unknown/duplicate/escaping paths, links/devices, malformed lengths, truncation, overflowing counters, forged digests, missing terminal/required output, control-looking target stdout |
| Capture and promotion are coherent | Proposed, S1/S2 contract resolved | Descendant mutation, cancellation during each handoff stage, cleanup failure, and restart during publication |
| Retained reuse is valid | Proposed, S3 contract resolved | Same input reuse; changed/extra/missing content, mode/lock mismatch, helper/policy change and two concurrent same-key requests |
| Qualification derives from one policy owner | Proposed | Setup/service description equality, root mismatch, helper/runtime/flag change invalidation and failed requalification with previous receipt present |

Measure peak memory, scratch bytes, reserved/actual host bytes, cold preparation, warm startup
and handoff time for the actual Rust and Python fixtures. Copying an admitted tree into tmpfs
has a concrete memory cost; 512 MiB is a proposed default, not evidence that every dependency
closure fits. Reject larger unsupported work truthfully rather than weakening the bound.

## 10. Exceptions and unresolved decisions

No SHOULD exception is needed for the selected small executor. S1–S3 are resolved as proposed
contracts. Exact byte encoding, routine parser structure and the concrete generation layout
remain implementation choices constrained by those contracts. Their real behavior must still
be verified before the service advertises readiness.

The user-approved evidence-retention rule remains: validated library facts do not expire, and
scratch/capacity cleanup does not delete them. Same-boot uncertain creator recovery fails closed;
this review does not extend that source scope or certify fresh-process recovery behavior.

## 11. Decision and implementation changes

**Accept this scoped proposed design.** The amendment fixes the three material contract gaps;
no architectural replacement or additional policy authority is required. Implementation should
preserve these boundaries and establish the following behavior with real evidence.

| Priority | Implementation obligation | Principles | Acceptance evidence / regression protection |
|---|---|---|---|
| 1 | Require target quiescence before handoff and separate completion meanings | DM-07, DM-14, DM-29, DM-35 | S1 descendant/timeout/malformed-terminal fixtures |
| 1 | Specify conservative reservation accounting and atomic generation recovery | DM-14, DM-29, DM-30, DM-35 | S2 saturation/concurrency/crash fixtures |
| 2 | Bind helper/protocol/normalized operation semantics to reuse identity | DM-15, DM-31, DM-32, DM-48 | S3 identity mutation and concrete reuse fixtures |

Only review documentation was changed. This review supports accepting ADR-0021's contract;
its author owns the ADR status change. It does not close F4/W2 implementation, modify the frozen
wire contract or claim a phase gate passed.
