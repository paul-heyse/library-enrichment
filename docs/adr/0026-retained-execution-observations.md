---
id: ADR-0026
title: Retain typed execution observations through durable concrete jobs
status: accepted
date: 2026-09-14
deciders: [paul-heyse]
level: decision
principles: [DM-02, DM-06, DM-07, DM-12, DM-24, DM-29]
design: [§3.3, §6.3, §B7]
review: docs/design_review/reviews/design_review_retained-execution-observations_2026-09-14.md
evidence: Proposed
supersedes: []
superseded-by: null
revisit: A concrete executed producer needs an observation payload outside the three closed operation variants.
verification: durable_execution_jobs; typed_execution_roundtrip; retained_semantics; retained_runtime_objects; retained_probe_results
---

# ADR-0026: Retain typed execution observations through durable concrete jobs

## Context

Plan 10 T6 incorporates Plan 09's unfinished durable inspection and producer requirements.
The current semantic inspector returns transient wire observations, while verification derives
a static child snapshot before executing and never contributes its completed probe results.
API declarations alone cannot represent a consumer-snippet diagnostic or a scoped runtime probe
without falsely turning it into a claim about the library's public API.

## Scope

Extend the typed evidence design with one `execution_observations` relation and concrete durable
job variants. Add typed inspection controls and returned execution evidence within the existing
nine tools and envelope. There is one target reader; no old development snapshot or journal
migration. This extends ADR-0022/0023, preserving their publication and semantic-identity rules.

## Drivers

Reproducible answers after restart, explicit execution scope, source/stub/runtime separation,
bounded scheduling and lifecycle ownership, and native Arrow retrieval of retained results.

## Options

- Keep responses and logs as the only result authority: rejected; consumers must reinterpret
  arbitrary producer documents and cannot query execution evidence through the target model.
- Put every execution result in API payload fields: rejected; a snippet result is not a public
  declaration, and query failure must not become an empty API observation.
- Add one closed typed execution relation and concrete jobs: selected. Specialized producer
  adapters normalize bounded results once; native plans retrieve them by scope and identity.

## Decision

`ExecutionObservation` has an immutable observation ID, typed subject, resolved environment ID,
qualified `FactSource`, and one discriminated payload:

- **Semantic query:** selected method (`hover`, `definition`, `implementation`, `references`, `diagnostics`),
  exact consumer document artifact/digest and query position, server identity, fixed UTF-8 coordinate
  contract, outcome, bounded hover text/diagnostics/locations and explicit limitations.
- **Runtime object:** a Python public path and the exact import/attribute selection scope,
  outcome, actual object kind, optional signature/docstring and bounded attribute names. Do not
  serialize arbitrary object graphs or merge observed values into source/stub declarations.
- **Usage probe:** mode, exact snippet artifact, process end/exit result and bounded stdout/stderr.
  The result says what happened to that consumer under that environment, never that an entire
  library or project is compatible. Process cleanup certainty is retained separately with the
  actual attempt, and incomplete cleanup cannot produce an unqualified successful job result.

Variant fields are native Arrow structs/lists/scalars, with conditional admission checks;
first-party observations are not JSON payload strings in Parquet. Add coverage kinds for
semantic queries and usage probes; runtime-object coverage uses `runtime_api`. Coverage attaches
to the actual symbol/snippet query scope, not an assertion of complete library coverage.
The new relation enters manifests, ID closure, foreign-key checks, native repository unions,
query/export views and resource budgets. Advance the snapshot schema contract and reject old
formats; no compatibility reader is introduced.

Store canonical normalized producer results as immutable artifacts used by the observation
source. Bind semantic identity to exact release, inputs, snippet/query, requested and observed
environment, producer/normalizer, image, helper/protocol/containment and result. Actual process
logs, negotiated protocol encoding, query document version, attempt clocks and generation-specific capsule paths remain
attempt attribution and must not perturb equivalent semantic results. A changed observation
produces a coherent successor snapshot; equivalent attempts can append attribution to the same
semantic snapshot. Attempt log blobs remain mandatory export closure.

The subject is a `Symbol` for runtime object observations with an admitted static binding, or an
exact `Document` plus selection/query for objects without such a binding. Semantic queries and
usage probes bind an exact `Document`; a semantic query may additionally name an admitted symbol
as its anchor. Each referenced symbol/document/environment/source must belong to the same admitted
snapshot closure. Remove `ApiOrigin::Semantic` and `ApiOrigin::Runtime`: execution results have one
authority in this relation, and never enter declaration observations through a second path.

Canonical result artifacts contain the normalized payload only, excluding the eventual artifact
ID, observation ID and source/producer binding. Hash the payload first, bind the normalizer and
source second, then mint the observation ID; the hash graph must be acyclic. Normalize only
adapter-owned URI/path/coordinate fields. Target stdout/stderr is literal observed content;
changed printed paths are changed results, not text to silently rewrite.

Use versioned closed job specifications: verification, execution-backed inspection and expensive
resolution. Verification/inspection pin their existing context/snapshot and normalized request
before enqueueing. A first resolution has neither: its concrete unresolved job specification
pins the normalized registry selector, package/version requirement, requested environment and
policy. Exact immutable release/input identities become a durable stage record after resolution;
mutable selectors are never claimed to be immutable release identities or evidence cache keys. Existing
`job_control` provides bounded wait/status/cancel and independent caller interests. One caller's
detachment cannot cancel another subscriber's result. Do not replay side-effecting runtime work
on restart; reconcile containers first and report interrupted jobs explicitly. Load terminal
journals lazily with a bounded cache, bound journal bytes and caller-interest counts, and never
substitute an empty success after a queue/resource/deadline failure.

Inspection accepts a bounded consumer snippet, selected semantic methods, and a locally enabled
execution profile. A coordinate is optional advanced input: zero-based line and UTF-8 byte offset
at a character boundary. Diagnostics need no coordinate; otherwise Rust derives a deterministic
selected-symbol/generated-probe anchor or reports explicit ambiguity. Never require the caller
to manufacture coordinates. Convert input positions to the negotiated LSP encoding and normalize
returned ranges to artifact-qualified zero-based UTF-8 byte positions, rejecting invalid or
unmappable ranges as explicit limitations. Locations outside retained documents use typed
external/unresolved targets qualified by release/image/server source scope and relative path;
never invent an artifact span or put a capsule UUID in semantic identity. Without a supplied snippet,
the Rust adapter constructs a bounded consumer from the selected symbol. Signature/docs reads
remain static. Runtime object execution requires explicit runtime intent and enabled runtime
policy. Expensive work returns `pending` with a durable job receipt within the inline wait budget.
No caller may supply host paths, shell commands, or arbitrary LSP method strings.

Carry cancellation through admission, capsule preparation, session mutex/capacity waits and LSP
exchange. Send LSP cancellation for an outstanding request; drain or discard the session if the
server does not settle within a bound. Record the negotiated position encoding. Distinguish
empty results, unsupported methods, unresolved dependencies, incomplete indexing and cancellation.
Handle full/unchanged pull diagnostics and URI/version-bound replacement push diagnostics;
`unchanged` reuses a matching prior diagnostic result, never invents an empty one.

Publish normalized execution facts only after the producer and required cleanup have settled.
A successful warm LSP query settles at its matching response and document-version boundary while
the retained session continues to own its execution lease. It does not require container absence.
Cancellation requires a matching drained response or confirmed session discard/cleanup before
terminal cancellation. One-shot execution requires confirmed container cleanup.

When the actual resolved environment differs from the input context, create a derived context
and successor snapshot. Inherit static facts with their original qualified sources. Static API observations are
contextually rebound to the child environment (their IDs are recomputed); the preserved source
records which build was actually documented, and this rebinding makes no execution claim. Bind
new executed facts to the actual environment. Never retag prior execution observations from the
source snapshot: exclude them from the derived snapshot and keep them readable through their
original context. An identical environment permits contribution to the existing context.

The catalog commit must include a job publication association naming its normalized result
artifact, terminal outcome and published context/snapshot. Startup reconciles this association
before marking an interrupted journal failed: a committed result repairs its terminal journal
without re-execution; a precommit interruption fails explicitly. Cancellation after commit cannot
erase or relabel committed evidence. The terminal journal is a delivery cache, not a competing
publication authority.

Retained reads require no execution qualification or enabled runtime policy.
Inspection defaults to retained reads for an exact query/environment/producer binding, with
bounded, deterministically ordered alternative results; no retained result is an explicit gap.
Execution-backed inspection requires explicit execution intent on a miss or explicit rerun intent
when evidence exists. Runtime object execution and usage verification are explicit effectful
requests. Terminal jobs and ordinary retained reads replay the published snapshot/result rather
than re-running a library producer because a capsule, provider, cache, or warm session was evicted.

### Consequences

There is a tenth relation and an explicit execution vocabulary to maintain. Existing development
fixtures must be regenerated. Native retrieval can now select execution evidence by subject,
environment and query before hydration, and future unchanged contexts retain all qualified
observations without age-based expiry. An execution failure remains useful scoped evidence.

### Compensating controls

Finite method/payload vocabularies, per-record/batch/output and job bounds, independent source and
runtime alternatives, resolved-environment foreign keys, leases through publication/cleanup,
negative admission fixtures and real ty/rust-analyzer/container tests. No new scheduling framework.

## Evidence

These sources establish interfaces, not that our integration works. Pinned Cargo/ty/analyzer
verification is recorded in the compatibility matrix, retrieved 2026-09-14.

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| Cancellation does not remove the response obligation | [LSP 3.17 cancellation](https://raw.githubusercontent.com/microsoft/language-server-protocol/gh-pages/_specifications/lsp/3.17/specification.md) | 2026-09-14 | “A request that got canceled still needs to return from the server” |
| Diagnostic reuse is a distinct response | [LSP pull diagnostics](https://raw.githubusercontent.com/microsoft/language-server-protocol/gh-pages/_specifications/lsp/3.17/language/pullDiagnostics.md) | 2026-09-14 | `kind: 'unchanged'` |
| Frozen Cargo builds forbid resolution changes and network | [Pinned Cargo options](https://raw.githubusercontent.com/rust-lang/cargo/7941be6fb416b4cd9666aef7b858dfea25587a8c/doc/man/includes/options-locked.md) | 2026-09-14 | “Equivalent to specifying both `--locked` and `--offline`.” |

## Verification

The named tests above are required target oracles, not claimed executions. They must demonstrate
native Arrow roundtrip and malformed-variant rejection; same-request identity under attempt and
batch reorder; real typed ty/analyzer results including Unicode/diagnostics; native runtime/stub
disagreement; interrupted jobs and surviving subscribers; equivalent-result reuse after session
and process restart; qualified successor snapshots; and complete offline result/log export.
Tie the existing R/P/C gates to these stronger scenarios without deleting IDs. Run generated
wire conformance and the actual ignored containment/client tier on matching source.

## Boundaries preserved

§B1–§B13 remain binding: Rust owns core state/schema/query/jobs/publication; Python remains the
thin FastMCP adapter and separate static worker; ty and rust-analyzer remain the semantic engines;
hosted rustdoc stays first; Arrow/Parquet/DataFusion remain the storage/query stack; Context7
remains a separate caller connection; execution stays capsule-bound and repository-isolated.
The six epistemic classes and nine public tools remain unchanged.

## More information

[Plan 10 T6](../plans/10-arrow-datafusion-architecture.md),
[compatibility matrix](../architecture/compatibility-matrix.md),
[cleanup/fallback review](../design_review/reviews/design_review_cleanup-rust-fallback_2026-09-14.md).

## Status history

- 2026-09-14 — proposed; review and implementation outstanding.
- 2026-09-14 — accepted at Proposed contract scope after E1–E6 were resolved; implementation and qualification remain open.
