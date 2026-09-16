---
title: Complete hard pivot to a unified DataFusion and Delta runtime
status: in-progress
date: 2026-09-15
adrs: [ADR-0041, ADR-0042, ADR-0043, ADR-0044, ADR-0045]
phase: 6
---

# Complete hard pivot to a unified DataFusion and Delta runtime

**Remaining execution scope, reviewed 2026-09-16:**
[Plan 16 — Complete the remaining unified runtime pivot](16-unified-datafusion-delta-runtime-completion.md)
reconciles the implementation below with current source and receipts. Use it for the remaining
dependency order, concrete replacements, deletion and qualification work. This document retains
the original destination and obligations; no work package or terminal oracle is closed by the review.

## 1. Mandate, scope, and completion boundary

Implement the aggregate destination in the
[unified runtime review](../design_review/reviews/design_review_unified-datafusion-delta-runtime_2026-09-15.md)
and [Delta integration follow-up](../design_review/reviews/design_review_delta-integration-followup_2026-09-15.md).
This includes **F01–F14 and DFU-01–DFU-08**, the whole-service operation mapping, provider-contract
deployment, and both reviews' decision tables. Sections 7–9 below make their ownership and exits
explicit. CDF, retention, diagnostics, native physical execution and deletion are required scope.

The user selected a complete hard pivot on **2026-09-15**. DataFusion is the execution basis for
all service data operations, transformations, decisions and commands; Rust delta-rs supplies the
transactional tables. Preserve the ability to obtain useful, exact, bounded code insights over
MCP. Existing implementation cost, compatibility, historical state and obsolete policy choices
have no veto over this destination.

**Plan status:** implementation authorized and started on 2026-09-15. ADR-0041–ADR-0045
record the selected decisions at Interface-checked strength. WP00–WP09 contain implementation work;
no work package or new acceptance oracle is yet complete. This plan supersedes the narrower
scope of Plans 10–14; their qualification does not qualify this target.

### 1.1 What hard pivot requires

- One executable architecture and one active storage/contract epoch. No legacy/new feature flag,
  alternate engine, dual writer, conversion service, old-state reader, deprecated endpoint alias,
  compatibility DTO, journal importer, migration job or rollback executable in the delivered tree.
- Replace producers' semantic walkers, policy branches, whole-score kernels, object rebase,
  custom catalog persistence and procedural result composition with native relations and plans.
  Putting their old implementations inside large UDFs or opaque ExecutionPlans does not qualify.
- Reuse existing code only when its actual behavior belongs to the destination: native expressions,
  immutable provider inventories, proven lease forwarding, exact format decoders, bounded protocol
  encoding, cryptographic primitives and physical process/storage mechanisms. Reuse is a positive
  contract decision, never a reason to preserve an old subsystem.
- Delete replaced code, old-format fixtures, obsolete schemas, alternate launch paths and unused
  dependencies in the work package that replaces them. Do not move them to `legacy/`, an archive,
  a disabled Cargo feature, a test helper or a commented-out implementation. WP11 verifies the
  complete tree; it is not permission to defer all deletion until the end.
- Bootstrap fresh service state. Do not import old journals, Parquet catalogs, results, cursors,
  publication records, retained artifacts or caches. Reacquire source material through the target
  adapters. Activation removes the enumerated old service-owned state, installations and artifacts;
  there is no retained historical deployment to satisfy compatibility.
- New runtime evidence still has exact source bytes, provenance, command replay and version
  retention. These are requirements of the **new** service, not preservation of the old service.
  New Delta history is necessary for coherent reads, CDF and recovery.
- Source-controlled frozen specification provenance is not runtime compatibility. Do not edit the
  frozen blueprint/handoff or erase the two requested review inputs. Revise the living design,
  current instructions and active documentation through the decision process; do not create an
  archive of retired implementation artifacts. Git history need not be rewritten.

Implementation may temporarily be incomplete while a replacement is being edited. Such a state
is not a release, supported dual mode, completed slice or reason to restore the old mechanism.
Every work package has one executor: **Codex, acting for the sole project owner**.

### 1.2 Terminal definition of done

Plan 15 is complete only when:

1. Every production data transformation/decision serving MCP is inspectable native DataFusion
   execution, including acquisition selection, normalization, eligibility, jobs, publication,
   search, comparisons, results, retention and diagnostics.
2. Every durable relational authority is a qualified Delta table. Exact raw bytes use a narrow
   immutable content-addressed store whose inventory/ownership is in Delta. One control-table
   commit selects each complete publication and terminal result.
3. Every remaining bespoke kernel is bounded, typed, registered at a native function/source/sink/
   command boundary, and contains only unmatched parsing, encoding or external mechanism.
4. Native policy has one definition and real consumers at each appropriate provider/engine/storage
   level. Metadata is derived from enforcement, not mistaken for enforcement.
5. All nine MCP tools, source evidence, Rust/Python acquisition and qualified execution, offline
   reuse, durable jobs, cancellation, export and bounded retained results work on fresh target state.
6. WP00–WP12, deletion rows L01–L15, findings and mandatory acceptance oracles close against the
   final source; the installed service uses only the target. Missing required qualification remains
   visible and prevents a claim of complete implementation.

## 2. Baseline and exact upstream contracts

Planning baseline: HEAD `9c154de8b2ead1d082fd253399bb1694627395c1`; reviewed service-source digest
`5b2640ca15e4e1e6bb3355eda84edb89d4bcdab3e390897513648c0581d4026a`.
At planning start the two reviews/evidence directories were untracked, alongside the new Delta
skill and a ripgrep reference; deletion of the superseded August Delta reference was staged.
Those are existing work. This plan does not reset, delete, stage or claim them as implementation.
Record a fresh path-level baseline at execution start and preserve unrelated concurrent work.

| Component | Selected contract |
|---|---|
| DataFusion | **55.1.0**, including its matching split crates |
| Arrow / Parquet | **59.3.0**, one Rust type universe |
| object_store | **0.13.2** |
| delta-rs | Git revision **`58f07cd62bfbce3649a7e1c87c696288068ae184`**; package version 1.0.0 is not a published-release claim |
| Selected Buoyant kernel and engine | **`8ba063f8f84fec222000f66d40d70911d7c79675`**, packages 0.25.1; lock both, not merely `buoyant/main` |
| Rust / FastMCP | Existing pinned Rust **1.98.1** and FastMCP **4.0.3** |
| Python producer semantics | Existing pinned Griffe/ty and separate extraction worker; generated Arrow IPC contract |

These are the reviews' verified September 15 revisions, not an instruction to follow a moving
HEAD. Use `--locked` resolution and a dependency receipt. If a necessary upstream fix changes a
pin, record the exact replacement and requalify the affected routes before adoption. Do not fall
back to the August revision, a different kernel fork, older DataFusion or a parallel dependency
universe. The Python Arrow encoder needs its own exact compatible pin and IPC conformance; a
matching Python/Rust version number alone proves nothing.

Use the [DataFusion skill](../../.claude/skills/datafusion/SKILL.md),
[Delta skill](../../.claude/skills/deltalake/SKILL.md), and
[provider-contract map](../design_review/capability-maps/datafusion_provider_contracts.md).
Canonical Delta operations are methods on `deltalake_core::table::DeltaTable`; **do not use the
removed `DeltaOps` API**. The skill's
[operation catalog](../../.claude/skills/deltalake/content/catalogs/operations.md) and
[foreign implementations](../../.claude/skills/deltalake/content/catalogs/foreign-impls.md)
identify public versus private/deprecated integration routes. Use exact source/rustdoc or a
focused probe for a new consequential question, rather than restarting broad library research.
Context7 is not needed for this plan's selected, pinned surface.

Existing evidence is interface/mechanism evidence only:

- [Current-revision runtime probe](../design_review/reviews/evidence/unified-datafusion-delta-runtime-2026-09-15/)
  establishes important CHECK, planner, snapshot and transaction behavior.
- [CDF/codec follow-up probe](../design_review/reviews/evidence/delta-integration-followup-2026-09-15/)
  establishes bounded CDF and selective read-provider serialization behavior.
- Neither proves the new control protocol, complete producer fidelity, installed clients,
  retention, resource containment or power-loss durability.

## 3. Destination: one native authority per concern

### 3.1 Rust and Python ownership

Use the existing Rust workspace without introducing a generic platform layer. `enrichment-core`
owns shared semantic types, authoritative native contract definitions and narrow format/value
kernels. `enrichment-store` becomes the service's native execution and Delta implementation:
operation binding, normalization/query plans, command extensions, admission, publication and
result plans. Its current name is not a restriction to retrieval. The daemon owns transport,
startup and the mechanical OS/network/process drivers consumed by those contracts. It does not
independently decide eligibility, fold jobs, merge records or assemble evidence results.

Break any dependency cycle by putting the finite typed driver interface in the lower Rust layer,
with its implementation supplied at session construction. Do not solve it with JSON callbacks,
an arbitrary plugin bus or duplicate operation definitions. A move between crates counts only
when the implementation itself conforms to the target.

The Python MCP adapter remains mechanical FastMCP transport. The Griffe worker performs static
extraction with `allow_inspection=False` and explicit `search_paths`, then encodes generated Arrow
facts. Python contains no shared semantic normalization, ranking, policy or publication logic.

### 3.2 Authoritative definitions and provider hierarchy

Keep definitions as native Arrow `Schema`/`Field`, DataFusion `Expr`/`LogicalPlan`/`TableReference`,
native options and finite typed operation descriptors. A descriptor references these values and
declares input/output relations, effect class, preconditions and stable revision; it is not a
second expression language, query IR or workflow interpreter.

One definition path supplies:

- Fact and evidence fields, provenance, keys, references, tagged-record rules and semantic encoding.
- Operation inputs, native selection/validation, effect grants, result sections and diagnostics.
- Native configuration, Delta properties, source options and resource policy consumers.
- Derived provider metadata, capability records, generated transport schemas and MCP annotations.

| Provider level | Required target behavior |
|---|---|
| Root list | Private native `MemoryCatalogProviderList` assembly; publish no mutable root handle. Registration has no invented error semantics |
| Catalog | Immutable domains for captured inputs, evidence, control, candidates, results and metadata; no synchronous backend refresh |
| Schema | Complete captured table inventory; consistent cheap names/types/owners/metadata and asynchronous provider preparation |
| Table | Snapshot-native Delta provider, bounded Arrow input, transparent native view, result relation, or explicitly qualified command provider |
| Logical source | `provider_as_source` / native source forwarding; one schema, constraint/default and pushdown authority |
| Session | Concrete native `SessionState`, shared `RuntimeEnv`, native functions/optimizers, composed extension planner and one effective policy |
| Physical execution | Native scan/join/aggregate/window/recursion/sink nodes by default; finite owned effect/command extensions only where required |

Prepare each published inventory from a single control snapshot. Load exact table ID/URI/version,
validate protocol and semantic schema, then build
`table.table_provider().with_session(Arc::clone(&state)).await?`. Apply cohort selection as real
view/plan predicates, not a file-skipping hint. Install fully qualified references and retain all
dependencies. Latest-table lookup, ListingTable over Delta files, and codec bytes never select a
publication. Candidate/history providers expose only facts justified for those bindings.

### 3.3 Physical data and control model

Use purpose-specific Delta tables for registry facts, producer observations, normalized evidence,
coverage, execution observations, materialized projections, result rows and diagnostic observations.
Exact source/archive/log bytes remain immutable artifacts, acquired anew in this epoch. Their
identities, provenance, grants, references and retention obligations are typed Delta relations.

Use **one** Delta control table for records requiring a shared transaction. It is a finite tagged
union with explicit nullable structs and CHECK predicates selecting the valid payload; no arbitrary
JSON, serialized domain object or EAV payload. At minimum declare these record families:

| Family | Required typed content / derived views |
|---|---|
| Command and job transition | Command ID, operation revision, typed arguments/references, job key, predecessor sequence, policy/environment witnesses, outcome/result reference |
| Claim and attempt | Job key, owner, fence/sequence, attempt ID, grant, lease expiry and cleanup state |
| Interest / cancellation | Client interest identity, job reference, desired action, predecessor and explicit cancellation effect |
| Context head / publication | Expected predecessor, publication ID, exact table-version/cohort vector, semantic contracts/digests, artifact and result references |
| Projection checkpoint | Projection/rule revision, source table identities, inclusive processed version boundary and selected output version |
| Retention / cleanup | Protected publication/result/version/window, owner, horizon, cleanup obligation and maintenance generation |

Define current-state views with native windows/joins. Keep the event relation append-oriented;
checkpointing and bounded native compaction must preserve current state and the promised replay
witnesses. No in-memory map or second file journal is authoritative after restart.

### 3.4 Publication, claims and effects

1. Bind command eligibility and every expected predecessor to one control snapshot V. Produce
   accepted work or typed refusal rows through native plans.
2. Commit the transition against V with application transaction keys for **every contested
   predecessor**: logical job, context head, relevant interest/retention state. Distinct jobs racing
   one context must conflict. Native preconditions define validity; transaction actions fence races.
3. On conflict reload and recompute. Never reuse an eligibility decision after silently changing
   its read snapshot. Audit Delta's internal commit retries so a log-position retry cannot become
   an application-state rebase without the required checks.
4. Reconcile retained command/result identity plus `transaction_version` before fresh retries.
   A repeated application transaction can be accepted on a fresh snapshot at this pin; it is not
   deduplication by itself. Unknown commit acknowledgement first triggers reconciliation.
5. Execute effects through a native command/source/sink with an attempt/fence and a retained
   physical ownership guard. Planning, EXPLAIN and optimization perform no effects. Lease expiry
   alone never authorizes a second process while ownership remains unresolved.
6. Write candidate evidence and complete results through qualified Delta builders; capture exact
   versions. Validate keys/references and semantic contracts against that vector.
7. In **one control commit**, select the complete publication/vector, advance the expected head,
   select projection offsets where applicable, and record the terminal job/result. This is the
   visibility boundary. It is not a cross-table Delta ACID transaction.
8. Readers acquire protection through the same retention protocol before loading selected versions.
   Unselected candidate writes cannot appear through admitted views; failed attempts leave typed
   orphan/cleanup obligations for native maintenance.

Cancellation of one client interest is distinct from cancellation of shared work. Keep permits,
leases and cleanup ownership until streams, writers and external children actually finish. The
service promises reconciled effects and truthful unknown outcomes, not impossible general
exactly-once subprocess execution.

### 3.5 Mutation and schema decisions

**Default mutation route:** native logical input into complete `DeltaTable` operation builders,
including `WriteBuilder::with_input_plan`, with the captured concrete session, commit properties,
writer options and `SessionFallbackPolicy::RequireSessionState` where supported. Compose one
`DeltaExtensionPlanner` with finite service planners inside the existing retention planner.
Inherit Delta's validation, mapping, merge safeguards, metrics and writer cleanup through those
builders. Do not register their private sub-planners independently.

Raw Delta provider INSERT is not admitted as the service mutation route: the reviewed pin bypasses
CHECK validation there. Published evidence providers are read-only. Explicit command providers
expose only qualified operations. Internal SQL, DataFrame and direct-provider paths either reach
that qualified adapter or reject the operation before any write; unsupported DML is not advertised.
The MCP surface remains typed operations, not raw SQL or arbitrary shell.

Where a DataFusion DML hook supplies physical input that the complete builder cannot accept, keep
that hook unsupported and use the explicit logical-input command route for the product operation.
If a real required consumer cannot use that route, expose/qualify the upstream preparation seam
over `DeltaDataWriterExt::write_plan`. That writer expects prepared input and returns uncommitted
Add actions. Do not copy private Delta constraint, CDC, column-mapping or commit preparation code.

Define one versioned mapping for the entire Arrow model to Delta storage and back. Test nested
struct/list/map fields, null versus absent, tagged unions, dictionary/view strings, timestamps,
binary values, decimals and metadata actually used by service contracts. Reject or explicitly
encode unsupported types; no permissive cast-to-NULL. Preserve semantic field metadata in the
contract relation and reattach only after validation. Use native Delta schema/row validators;
semantic keys/references still require native aggregate/anti-join admission before trusted metadata.
Use `ProjectionStructPatchBuilder` for actual kernel-boundary schema/expression edits; application
business projections remain DataFusion expressions. Private `SchemaComparison` is not a service API.

Canonical identities use native typed projection/order/deduplication with a narrow canonical-byte
kernel and a correctly ordered digest aggregate. Declare domain separation, field order, null and
set/list semantics, numeric encoding and merge behavior. Do not reconstruct Rust domain objects
or JSON trees to hash. Physical table versions, CDF versions and file IDs are not semantic IDs.

## 4. Execution sequence

Architecture and removal precede broad regression replay. Run a small meaningful oracle for the
changed boundary, then continue the replacement. Do not repeatedly run full suites against the
same source or use old green gates as a substitute for architecture work.

| Work package | Dependencies | Concrete deliverable |
|---|---|---|
| WP00 | — | Decisions, native contract inventory, positive enforcement and new-epoch scope |
| WP01 | WP00 | One configured native runtime and complete planner/resource composition |
| WP02 | WP01 | Typed Arrow/Delta contracts, admission and canonical identities |
| WP03 | WP02 | Qualified Delta mutations, storage and immutable exact-version catalogs |
| WP04 | WP03 | Durable control transitions, ownership guard and atomic publication |
| WP05 | WP04 | Arrow acquisition/producer facts and native registry/requirement decisions |
| WP06 | WP05 | Complete native Rust/Python normalization and ingestion |
| WP07 | WP04, WP05, WP06 | Shared operation policy and all owned execution/verification paths |
| WP08 | WP06, WP07 | Native research/results/delivery and all nine MCP tools |
| WP09 | WP08 | Real CDF projections, full recomputation and command/read replay |
| WP10 | WP04, WP08, WP09 | Complete retention, maintenance, native telemetry and physical choices |
| WP11 | WP00–WP10 | Whole-tree deletion and architecture closure |
| WP12 | WP11 | Final product qualification, fresh activation and old-state removal |

Do not build a second reusable engine before using it. WP03 proves a real table write/read; WP04
proves publication and durable retry; WP05/WP06 immediately run actual Rust and Python evidence
through it. WP08 completes public journeys. Earlier executable slices use the target contracts;
they are not alternative runtimes retained after the next package.

### WP00 — Record target decisions and remove obstructing rules

**Changes**

- Write accepted ADRs for native execution/policy, Delta storage/control/publication, semantic
  Arrow/Delta mapping and identity, owned effects/resources, and replay/retention/new epoch. Allocate
  available ADR IDs at execution time and populate this plan's front matter. Supersede the affected
  mechanisms of ADR-0022/0023/0024/0030/0031/0038/0040 and other affected records precisely; retain
  their still-applicable functional outcomes. Update `docs/design/DESIGN.md` and active setup docs.
- Inventory every operation and dataflow from request through effects/results, with native owner,
  relation schema, policy, allowed residual kernel and L-row deletion assignment. Include CLI/export,
  background maintenance, recovery and error paths, not just successful MCP queries.
- Replace `scripts/architecture-check.py`'s store-only DataFusion and blanket Python Arrow bans with
  positive native execution/IPC boundaries. Replace frozen old-format schema comparisons with
  current generated-contract conformance; do not weaken evidence fidelity or generated ownership.
- Change dependency source policy to permit only the selected Delta/kernel sources and enforce
  exact lockfile revisions. Do not add a broad unknown-Git allowance or dependency ignore list.
- Replace active rules that prohibit native job operations or mandate the old publication/worker
  JSON format. Protected enforcement files are currently session-denied: supply an exact reviewed
  operator-applied rule patch where the guard requires it. This is a harness installation boundary,
  not an architecture approval queue; WP00 cannot close with a still-enforced contradictory rule.
- Declare one new storage/wire/result/cursor epoch. No old-format translator, retained-state rollout
  or historical-artifact import is in scope. Preserve typed MCP capabilities rather than old shape.

**Exit / deletion:** one field/rule/operation definition can identify all generated/native consumers;
new dependency policy and architectural fixtures pass. Obsolete active rule expectations and old
format generators are removed. ADR/design links resolve; frozen provenance bytes remain unchanged.

### WP01 — Build the shared native runtime and planner

**Changes**

- Extend `store/runtime.rs`, `leases.rs`, `native_policy.rs` and preparation to build one real
  SessionState from the shared RuntimeEnv, stores, native functions, optimizers, pool, spill and
  bounded metadata cache. Carry one operation context through plans, tasks and driver handles.
- Install `DeltaExtensionPlanner` plus finite service extension planners under `RetentionPlanner`.
  Preserve subquery/view leases and dependencies even when optimization eliminates a scan.
- Set identifier normalization, join/dynamic-filter settings and Delta options deliberately from
  one effective configuration. Pass the same session to Delta operations; disallow implicit default
  session construction where supported. Check store-registry collisions before registration.
- Define native extension contracts for child replacement, expressions, boundedness, execution
  eagerness, statistics, partition/order properties, cancellation and metrics. Use native streaming
  providers for finite fact streams; do not misuse an unbounded file stream as a job engine.
- Install bounded telemetry collection now. Charge native memory/spill through native reservations;
  separately account for decoder, kernel/default-engine, writer buffers and child-process budgets.
  Delta's `DataFusionEngine` is inherited from its scan path, not reimplemented.

**Exit / deletion:** a real Delta builder plan containing validation/metrics lowers and runs under
the same runtime as a retained read. Optimized-away COUNT still holds its source lease. Planning
an effect invokes nothing. Delete competing session factories/default policy copies and obsolete
blocking writer scopes as their consumers move.

### WP02 — Replace model conversion and identity machinery

**Changes**

- Define all observation/evidence/control/result schemas using the native contract path. Generate
  worker IPC schemas, wire projections and metadata from that source; no independent Python schema.
- Implement the complete storage/read mapping from §3.5, native row/tag checks, key/reference
  violation plans, and candidate-versus-admitted metadata. Protocol/schema-feature admission
  precedes provider registration and optimizer trust.
- Replace `semantic.rs` object reconstruction and `projection/decode.rs` semantic round trips with
  typed canonical encoding and native ordered aggregation. Keep mechanical final DTO encoding only.
- Reuse public kernel paired-patch construction where nested kernel output projection is required;
  inherit native Delta column mapping and validators elsewhere. Never copy private schema machinery.
- Define full dependency identities for contracts, functions, policies, producer/environment facts,
  source versions, publication cohorts, result encoding and effects. Native cache selection consumes
  those witnesses; a session clone or a filename does not establish equivalence.

**Exit / deletion:** round trips preserve the actual full service field inventory and independent
epistemic classes; malformed variants, lossy conversions, duplicates and invalid references fail
with bounded witnesses. Identity is stable under reordering, partitioning and equivalent encodings,
and changes for meaningful differences. Old canonical-object/hash and staging conversion paths go.

### WP03 — Install Delta storage, qualified mutations and captured catalogs

**Changes**

- Create target tables using native create/schema/feature/property builders, enabling CDF at creation
  for WP09 input tables. Derive native table properties and writer/data-skipping options from policy.
- Implement the logical-input mutation command route in §3.5 for actual append/update/delete/merge
  and maintenance consumers. Bind transaction properties and capture typed operation metrics.
  Enforce read-only admission on published providers and reject every unqualified DML bypass.
- Build the exact-version provider inventory in §3.2. Reuse native constraints/default/source
  forwarding and transparent views. Do not wrap Delta reads in a second file scanner or publish
  inaccurate ordering, uniqueness, cardinality or pushdown claims.
- Select a **local conditional-create and durable storage boundary** for the control log. Verify the
  actual object_store/log-store path; add a narrow conforming durability adapter for missing file/
  directory synchronization guarantees. Exclude `ConditionalPutShim` and unsafe overwrite/rename
  paths. No second transaction engine is retained to compensate for backend behavior.
- Reuse `DeltaTableFactory` for controlled current-table preparation where useful, then explicitly
  capture identity/version before import/publication. Native listing discovery is bounded and
  collision-checked; it cannot replace the publication vector. Remote Unity/S3 scope is §8.

**Exit / deletion:** real valid writes/read-back and invalid CHECK/NOT NULL/tag writes through all
exposed routes; duplicate merge matches refused and global keys separately admitted. Old catalog
head cannot leak into new sessions; concurrent underlying updates leave pinned results unchanged.
Remove custom ordinary Parquet table writers and legacy provider construction used by this slice.

### WP04 — Replace job journals and publication with the control table

**Changes**

- Implement the §3.3 control schemas and native current-state/precondition/refusal plans. Replace
  `daemon/jobs.rs`'s map/journal state machine, job-key decisions and `single_flight.rs` semantic
  authority with command/claim/interest relations. In-memory wakeups remain mechanical hints only.
- Implement snapshot-bound conditional transitions and all contested transaction keys from §3.4.
  Reconcile fresh retries, lost acknowledgement, restart and terminal results from durable records.
- Implement the native effect ownership guard used by acquisition first and all producers in WP07.
  It persists attempt/fence and cleanup obligations, reconciles physical ownership after restart,
  and retains permits until actual exit. No successful status is synthesized from an in-memory task.
- Replace catalog generation commit/rebase with validated candidate versions followed by one control
  commit containing head/publication/terminal. Native joins compute contribution/rebase decisions.
  Include complete result dependencies before claiming a successful terminal outcome.
- Implement reader-protection enrollment and maintenance-generation conflict fencing now. Protect
  the selected version before loading it, revalidate contested state, and prevent vacuum from
  racing new leases. WP10 completes policy and native reclamation on this protocol.

**Exit / deletion:** independent stale claimants, distinct jobs racing a head, stale owner, interest
cancellation, renewal, lost acknowledgement and process restart preserve the declared invariants.
Faults between every candidate/control commit expose either the previous complete publication or
the new complete one. Remove JSON jobs, custom current-pointer/manifest commit, object rebase and
their readers; do not run a shadow journal during qualification.

### WP05 — Acquire typed facts and make resolution native

**Changes**

- Change rustdoc/registry/archive/source decoders to emit bounded Arrow facts with processed-input,
  provenance and gap records. Use native JSON/file-format scans where source structure fits;
  unmatched rustdoc structures get a typed decoder, not a semantic Walker disguised as a parser.
- Change the Griffe worker from whole-response JSON to bounded Arrow IPC facts. Retain mechanical
  tree visitation and exact independent `.py`/`.pyi` observations. Validate schema, producer identity,
  stream limits, truncation and worker failure before admitting facts in Rust.
- Register acquisition/decoding as finite sources/effects under WP04 ownership. docs.rs rustdoc JSON
  precedes isolated fallback compilation. Studied repositories remain read-only source inputs and
  are never subprocess working directories, extraction destinations or installation targets.
- Model registry releases/dependencies/features, requirements/extras/marker AST and environment
  facts relationally. Native filters, joins, ordering/windows and top-k choose candidates and explain
  refusal/ambiguity. Keep SemVer/PEP 440 parsing/precedence as narrow typed kernels; translate markers
  into native expressions. Cargo/uv solving remains an observed owned effect.
- Move fetch freshness, offline reuse, source selection and dependency witnesses into native
  operation plans. Keep archive/network byte I/O and source-span extraction as mechanisms only.

**Exit / deletion:** real Rust and Python packages produce typed facts on fresh state; offline reuse,
partial/malformed input, conflicting source/stub facts, SemVer/PEP 440 ordering and marker conditions
have truthful outcomes. Delete JSON worker semantic interchange, procedural release neighbor/sort
selection and acquisition policy branches replaced by native plans.

### WP06 — Replace both normalizers and the ingestion pipeline

**Changes**

- Express Rust item/module/reexport/impl/type/path relationships as native joins, unions, recursive
  CTEs and grouped ambiguity. Preserve public binding versus definition identity and source spans.
- Express Python aliases, bases, overloads and visibility signals with native closure/UNNEST/
  grouping and conflict relations. Independent observations survive disagreements; no arbitrary
  first-value winner or fixed repeated map pass silently truncates semantics.
- Declare cycle/path guards, depth/work budgets and explicit partial-coverage rows. Test meaningful
  large/deep fixtures; the review's three-node cycle is not the scalability acceptance oracle.
- Stream the native normalization/admission DAG into validated Delta logical-input writes. Remove
  `ingest.rs` callback visitation and object→staging Parquet→query→object→writer loops.
- Generate coverage and diagnostic rules from the same definitions as selection/admission. Pure
  language rendering kernels may remain only with typed inputs, stable identity and bounded output.

**Exit / deletion:** representative real Rust/Python public APIs, aliases, ambiguity, overloads,
reexports, missing targets, cycles and source/stub conflicts match independent expected semantics.
Inspect plans to prove native operators, not just equivalent answers. Delete semantic normalizer
Walkers, repeated closure maps and obsolete row converters in the same change.

### WP07 — Unify operation policy and complete owned execution

**Changes**

- Replace request/readiness/verification/producer/profile branch duplication with one native
  operation/precondition contract. Effective policy revision feeds readiness, claim admission,
  execution grants, status and refusal diagnostics consistently.
- Bind engine options to SessionConfig/RuntimeEnv/TableOptions; persisted row rules to Delta
  properties/expressions; scope/eligibility to native plans; physical effect grants to the command.
  Information_schema, owner metadata, ConfigExtension and MCP annotations remain derived reporting.
- Route rustdoc fallback, rust-analyzer LSP, ty semantics, Cargo/uv capsules and runtime verification
  through explicit native effect sources/sinks/command plans. Preserve transport framing, process
  groups, OS sandboxing, timeout, resource enforcement and logs as mechanical drivers only.
- Define replay/revocation/cancellation and shared-interest behavior from the control snapshot.
  Capture exact environment, command, input, grant and producer witnesses in execution observations.
  Native plans shape evidence, evaluate qualifications and select usable results.
- Audit every daemon `ops/`, `execution/`, fetch and recovery path for semantic selection or merge
  outside plans. Replace those paths, including failures and resource-exhaustion decisions.

**Exit / deletion:** one policy change yields consistent readiness, admission and refusal across
all affected tools; callers cannot grant themselves execution profiles. Actual qualified Rust and
Python producer runs, incorrect snippets, cancellation, stream drop and restart retain ownership
and truthful outcomes. Remove old job dispatch/eligibility/semantic handoff code, keeping only
necessary physical lifecycle mechanisms behind the native boundary.

### WP08 — Make research, results and MCP delivery native

**Changes**

- Replace whole-score UDFs with query-token/factor relations and native string/CASE/aggregate/window
  plans. Derive explanations from the same factor rows. Normalize repeated text once per revision;
  retain deterministic tie-breaking, token caps and Utf8/LargeUtf8/Utf8View semantics.
- Implement overview, exact/fallback inspection, aspect selection/coverage, ambiguity, before/after
  comparison and environment contribution through qualified native plans. No handler sort/dedup or
  JSON/string shortcut for field-level comparison. Semantic claims remain evidence-scoped.
- Declare typed result sections, evidence refs, ordering, independent aspect cursors, diagnostics
  and retained result identity. Persist complete results before terminal publication and derive
  pages/selections natively. One sink measures actual escaped JSON/MCP byte costs and performs final
  mechanical serialization; it does not repeat semantic trimming/selection in DTO mutations.
- Drive all nine tools from generated operation bindings: `resolve_library`, `library_overview`,
  `search_evidence`, `inspect_symbol`, `compare_releases`, `verify_usage`, `read_artifact`,
  `job_control`, `service_status`. Preserve strict FastMCP 4.0.3 input/output conformance, typed
  ToolResult content/resources, bounded diagnostics and prompt startup without hidden acquisition.
- Rebuild artifact-window/source retrieval and export/verification against new publication
  descriptors and content-addressed handles. Artifact byte reads/encoding are bounded mechanisms;
  authorization, selection, lineage and retention are native plans. Never accept arbitrary paths.
- Regenerate wire/JSON schemas/Python DTOs together for the new epoch, then delete superseded ones.
  Update the product `skills/library-research/` to the target outputs and truthful follow-up actions.

**Exit / deletion:** actual MCP calls cover every tool, independent pages, empty versus unknown,
ambiguity, exact supporting source, cold comparison, offline reuse and durable completion. Large
escaped/Unicode/nested values respect byte bounds and retained replay. Delete score wrappers,
procedural result sections, handler selection and old delivery recovery/format branches.

### WP09 — Deploy CDF projections and correct replay

**Changes**

- Make **search-factor/materialized search projection maintenance** the first mandatory CDF
  consumer. A second native view exposes publication-scoped change explanations. These are actual
  product consumers, not a CDF demo or a spare capability flag.
- Build bounded `DeltaCdfTableProvider` inputs via `scan_cdf().with_starting_version(a)` and
  `.with_ending_version(b)`. Bind table identity, inclusive version window, schema/transform revision
  and source publication cohorts. Check signed/UInt64 conversion at the version boundary.
- Apply insert/delete/update preimage/postimage semantics with native joins/aggregates. Physical
  OPTIMIZE changes and unpublished candidate cohorts must not become semantic additions. Mixed
  multi-table inputs use a publication-selected source vector; CDF timestamps do not define it.
- Commit output first, then select its exact version **and input offsets** in the same control
  transaction. Reconcile retries and retained commands. Never advance an offset before output is
  visible, skip unavailable history, or equate an empty feed with a successful failed read.
- Implement native full recomputation from retained target input snapshots for schema/rule changes
  and expired CDF windows. This is the target rebuild algorithm, not a legacy fallback. Rebuild is
  explicit, atomically selected, and resumes CDF under its new revision.
- Persist typed commands and version/input descriptors as replay authority. Reconstruct physical
  plans under fresh policy/runtime handles. Limit `DeltaLogicalCodec` to a qualified immutable
  read-provider cache; validate schema/table/version/cohort after decode and rebind required handles.
  Preserve predicates in actual plans, not serde-skipped hints. Dispatch/reject unsupported nodes
  before TODO hooks. Reconstruct CDF from its descriptor; exclude `DeltaPhysicalCodec` entirely.
- Drive reuse/invalidation from complete input/function/policy/environment/contract witnesses;
  change one dependency at a time to prove cache invalidation rather than relying on a digest name.

**Exit / deletion:** incremental output equals full native recomputation across inserts/updates/
deletes, no-op maintenance, rule revisions and faults before/after checkpoint publication. Fresh
process replay survives lost handles and rejects wrong schemas, missing history and unsupported
codecs. Remove custom file-diff/incremental engines and generic persisted physical-plan assumptions.

### WP10 — Finish retention, native maintenance, telemetry and physical choices

**Changes**

- Use one retention definition for source artifacts, Delta files/logs/checkpoints, publication and
  query/result leases, CDF windows, command replay and transaction-marker duration. Make horizons
  finite/configurable with validated relationships; persist the effective values per operation.
  Durable command/result reconciliation must outlive the promised replay horizon even when native
  `set_transaction_retention_duration` expires transaction markers.
- Native plans derive protected versions and reclamation candidates; native checkpoint/log
  compaction/OPTIMIZE/vacuum perform table maintenance under owned command contracts. Inherit the
  kernel's private RetentionCalculator through those operations and properties. Do not implement
  its internals as an application policy engine.
- Qualify `with_keep_versions`, log reconstruction, CDF retention and enrollment/maintenance races
  before enabling automatic reclamation. An unqualified route stays disabled and its gate remains
  open; disabling all reclamation permanently does not complete this package.
- Complete the local storage synchronization/conditional-create oracle, including crash boundaries
  and the documented power-loss requirement. Process-kill tests alone cannot certify fsync behavior.
  Use a suitable filesystem/crash harness; an unavailable prerequisite is `blocked`, never passed.
- Reuse DeltaScanMetaExec, current DeltaScan/DeltaScanExec, native PruningStatistics, strict snapshot
  file selection, deletion vectors, native range/page/footer readers and DataFusionEngine. No custom
  COUNT index or bare Parquet shortcut for Delta membership. Missing files fail explicitly.
- Route Delta operation metrics, DataFusion metric sets and kernel ReportGeneratorLayer/
  MetricsReporter events into one bounded Arrow diagnostic relation with operation/attempt/
  publication lineage. Avoid parsing DisplayAs/EXPLAIN strings into authority. Metrics cannot prove
  a commit; the control record does. Preserve bounded dropped/truncated-diagnostic indicators.
- Measure real complete journeys and select native partitioning, clustering/Z-order when useful,
  file sizing, statistics columns, row-group/page/Bloom/dynamic-filter settings, metadata scans,
  IPC batching, caches and spill. Report latency, I/O/files, memory, retained history and semantic
  equivalence; do not infer a speedup from an API name. No duplicate service scheduler/kernel engine.

**Exit / deletion:** retained old **target-epoch** publications remain readable through OPTIMIZE,
checkpoint, marker expiry and concurrent cleanup; unprotected data is actually reclaimed safely.
Cancellation during decoding/write/commit/LSP returns resources only after real cleanup. Native
diagnostics explain actual work. Delete custom catalog compaction/vacuum, duplicated metrics,
file-count shortcuts and obsolete retention stores.

### WP11 — Close the hard pivot across the whole tree

**Changes**

- Close every L-row in §6 against actual imports, references, call paths and package builds. Audit
  recovery/error/CLI/maintenance and generated outputs as well as normal MCP handlers.
- Remove orphaned modules, DTOs, old wire schemas, scripts, executable entry points, launch configs,
  feature flags, fixtures, dependencies and documentation that instruct running retired mechanisms.
  Do not retain an executable differential baseline; expected semantics and current source fixtures
  are sufficient. Frozen provenance and the review inputs are documented source exceptions only.
- Run the skills' project-gap queries as leads; inspect findings and add the cheapest meaningful
  AST/behavior oracle. No blanket suppression and no claim that syntax proves native execution.
- Demonstrate an ordinary extension: add a real observation/aspect or policy rule through the
  authoritative definition and show native normalization/eligibility/results/diagnostics following
  it. It must not require a second semantic branch in Python, daemon status and a score/result kernel.
- Rewrite living architecture/setup/product documentation to describe only the delivered target.
  Record explicit integration dispositions from §8 and any remaining mandatory open gate.

**Exit / deletion:** architecture inventory has no unexplained imperative semantic path, duplicate
policy authority or legacy production reader/writer. All §6 rows close with evidence. The final
workspace builds using only target dependencies and interfaces; no compatibility mode is installed.

### WP12 — Qualify, activate fresh state, and remove the retired runtime

**Changes**

- Run §7's final mandatory qualification once the integrated source is ready. Build a locked,
  non-editable installed candidate and qualify actual MCP/producer/client behavior from an unrelated
  directory on a dedicated fresh state root. Record source digest, pins, configuration and commands.
- Exercise required execution profiles on the actual Linux containment backend. Missing credentials,
  images or platform support remain explicit. Do not narrow previously required Rust/Python
  functionality to static-only to obtain closure; supported production profile selection stays
  explicit and unauthorized profiles must be refused truthfully.
- Prepare a concrete service-owned deletion manifest: retired install directories, old XDG state
  and result/artifact/cache roots, old epoch configuration, obsolete client launch registrations and
  task-owned old development/probe artifacts. Resolve actual paths and ownership, active processes
  and shared-cache use first. Never glob user repositories, general Cargo/uv caches, unrelated
  workspaces, or current review/qualification evidence into the manifest.
- Stop the old daemon/adapters and finish physical cleanup. Activate only the qualified candidate
  with a fresh new state root and regenerated configuration/client bindings. No state import or old
  artifact copy. Apply the reviewed scoped deletion manifest; no historical installation/state
  backup is part of the target. If activation fails, stop and repair this target rather than switch
  to a legacy executable.
- Run deployed smoke against the fresh target, prove process/install/config identities and all
  client registrations, and verify zero unexplained jobs, children, leases or reservations afterward.
  Update STATUS, plan/index, design, acceptance and installation evidence from the actual source.

**Exit:** one installed native runtime, only new-epoch state, actual preserved MCP functionality,
completed deletion manifest and truthful final gate report. Release/client packaging qualification
is a terminal/manual boundary, not a new prerequisite for every local edit or commit.

## 5. Decision changes and enforcement placement

| Obstruction / old assumption | Required replacement | Owner |
|---|---|---|
| DataFusion confined to retrieval/store; normalization precedes Arrow | Native types/plans throughout Rust data/operation ownership; generated worker IPC | WP00, WP01, WP05, WP06 |
| Blanket Python Arrow prohibition | Mechanical generated IPC encoding only; Python semantic computation still prohibited | WP00, WP05 |
| Unknown Git policy rejects required Delta/kernel | Exact approved sources plus lockfile SHA/type-universe enforcement | WP00, WP03 |
| Plan 14's score kernel, existing writer and narrow finite scope treated as final | Full native normalization/scoring/commands/results and Delta persistence | WP00, WP03–WP08 |
| Daemon-only publication interpreted as directory/current-pointer writes | Daemon-authorized native commands; one Delta control commit; workers cannot publish | WP00, WP04 |
| Frozen old transport/storage shape or retained-state installation | New explicit epoch; generated replacement; fresh state and scoped deletion | WP00, WP08, WP12 |
| No generic workflow engine interpreted as no native job plans | Finite service commands expressed in DataFusion; no second interpreter | WP00, WP04, WP07 |
| Provider constraints, owner metadata, ConfigExtension or successful parse treated as enforcement | Native consumers and route-specific behavioral admission evidence | WP02, WP03, WP07 |
| Transaction marker, lease timeout or metric treated as successful job authority | Durable command reconciliation, real ownership and selected publication/result | WP04, WP09, WP10 |
| Guarded instruction/hook files prevent installing the replacement rules | Exact operator-applied enforcement patch where required; no silent contradiction or blanket guard bypass | WP00 |

AST rules catch code-shape regressions; native conformance tests prove semantics; `just` gates join
whole-repository/dependency/installed evidence. Prose is reserved for boundaries without a cheaper
executable oracle. Replace wrong rules, not high-level evidence, containment or truthfulness goals.

## 6. Legacy removal ledger

Paths below are baseline locations under `crates/enrichment-{core,store,daemon}/src/` unless
fully qualified. Remove the named obsolete mechanism, not target-compatible byte/driver code
merely because it shares a file. Rename/split such a file so ownership is clear; no dead legacy
body remains. All rows start open; implementation records exact deleted/replaced paths and receipts.

| ID | Baseline mechanism to remove | Target replacement | Close by |
|---|---|---|---|
| L01 | `store/catalog_generation.rs` generation log, manifests, pointer/CAS and bespoke compaction; `projection/catalog.rs` old folding | Delta snapshots + single control publication/maintenance | WP04, WP10 |
| L02 | `daemon/jobs.rs` JSON journal/state machine, `single_flight.rs` semantic ownership and restart readers | Control relations, native preconditions, claim/fence reconciliation | WP04 |
| L03 | `store/dataset.rs`, `record_writer.rs`, staging channels and ordinary custom ArrowWriter pipeline | Complete Delta logical-input writers/native output sinks | WP03, WP06 |
| L04 | `store/repository.rs` object rebase/contribution merge and generation publication; old `ops/publication.rs` logic | Native contribution/admission plans + control commit | WP04, WP06 |
| L05 | `core/producer/normalize.rs` semantic Walker/repeated passes and Python normalizer alias maps | Typed facts + native closure/ambiguity/coverage | WP06 |
| L06 | Worker whole-document semantic JSON and `store/ingest.rs` object visitation; encode/decode/staging round trips | Generated Arrow IPC and direct native DAG | WP05, WP06 |
| L07 | Registry neighbor/release sorting and requirement/marker eligibility branches | Native release/environment/marker plans + narrow version kernels | WP05 |
| L08 | `store/scoring.rs`, `projection/score.rs`, `core/search/mod.rs` whole-score algorithm/UDF | Native token/factor/score/explanation plans | WP08 |
| L09 | `store/semantic.rs`, `core/canonical.rs` domain-object/JSON hashing and outer ordered folds | Typed canonical kernel + native ordered aggregation | WP02 |
| L10 | Daemon `ops/` readiness/selection/verification/status policy duplication; semantic `execution/` branches | One operation policy, native effect contracts and derived diagnostics | WP07, WP08 |
| L11 | `store/result.rs`, `daemon/delivery.rs`, projection render and handler DTO trimming/selection/recovery | Native result relations/pages + one mechanical bounded sink | WP08 |
| L12 | Custom retained-file scans, count shortcuts, maintenance/lease metadata stores, incremental file diff and duplicate metrics | Delta scan/statistics/CDF/control retention and structured native telemetry | WP09, WP10 |
| L13 | Superseded wire/job/catalog/result/cursor schemas, aliases, conversion tests and generated DTOs | Sole current generated epoch and functional contract fixtures | WP00, WP08, WP11 |
| L14 | Old engine/state feature flags, fallback imports, dependency allowances, scripts/configs and inactive executable fixtures | Only native target modules, rules, dependencies and launch surface | WP11 |
| L15 | Old service-owned installations/state/artifacts/caches and retired client registrations | One fresh qualified target installation and state root | WP12 |

Final proof combines exact symbol/import search, Cargo dependency inspection, architecture rules
with fixtures, exercised cold/recovery call paths and installed process/path checks. A zero-match
search or a module rename alone does not prove removal of the behavior.

## 7. Acceptance oracles and efficient verification

The following **proposed recipe names are to be implemented**, not commands claimed to exist
today. Register them and their concrete tests/receipts in the acceptance system without inventing
passing results. Existing gate IDs retain their high-level functional meaning; any retirement or
replacement is recorded through the ADR/registry process. Do not carry Plan 14 passes onto new
source/epoch behavior.

| ID / proposed recipe | Required oracle | Primary packages |
|---|---|---|
| Q01 `just native-contracts-check` | Generated schemas/IPC/provider metadata agree; ordinary extension and one-policy-change demonstration; no second semantic owner | WP00–WP02, WP07, WP11 |
| Q02 `just delta-mutation-check` | Real planner composition; valid/invalid CHECK/NOT NULL/nested/tag inputs across every supported SQL/DataFrame/provider/command route; unsupported routes have no side effects; merge-match and global-key checks | WP01–WP03 |
| Q03 `just delta-control-check` | Independent processes racing claims and same-context heads; stale owners/predecessors; fresh duplicate requests; cancellation/renewal/interest races; unknown commit outcome and each publication crash boundary | WP04 |
| Q04 `just native-evidence-check` | Complete Arrow/Delta mapping, canonical identity metamorphism, native Rust/Python normalization, source/stub conflicts, coverage and depth/resource bounds against independent expected facts | WP02, WP05, WP06 |
| Q05 `just native-effect-check` | EXPLAIN/optimization zero effects; repeated execution and drop; actual child/writer cleanup; policy revocation; restart reconciliation and uncontained/unqualified refusal | WP04, WP07 |
| Q06 `just native-research-check` | Native score/factor tie equivalence; scope/ambiguity/comparison/aspect behavior; source references; exact encoded budgets, paging and retained completion through actual MCP | WP08 |
| Q07 `just delta-incremental-check` | CDF full/incremental equivalence including update/delete/preimage, unpublished data, no-op maintenance, rule/schema change, missing history and output/offset interruption | WP09 |
| Q08 `just native-replay-check` | Fresh-process typed-command replay; read-provider codec preserves pin and rejects wrong schema/identity; unsupported logical/physical/CDF codec paths rejected; dependency invalidation | WP09 |
| Q09 `just delta-retention-check` | Query/result/version/CDF protection, optimized-away scan lease, enrollment/vacuum race, marker expiry/checkpoint replay, safe actual reclamation and log reconstruction | WP04, WP10 |
| Q10 `just delta-storage-check` | Actual simultaneous conditional creates, backend registration identity, write/commit faults, filesystem synchronization and required power-loss evidence; no ConditionalPutShim | WP03, WP10 |
| Q11 `just native-resources-check` | Native pool/spill plus external decoder/kernel/writer/process budgets, strict missing files, DV-aware metadata/full-scan equivalence, bounded correlated metrics and cleanup under failure | WP01, WP07, WP10 |
| Q12 `just native-removal-check` | All L01–L15 closed at their applicable source/installation boundary; imports/call paths/configs/packages lack old authorities, fixtures and compatibility artifacts | WP11, WP12 |
| Q13 `just unified-runtime-qualify` | Final-source quality + real fresh-state installed Rust/Python, all nine tools, actual Codex/Claude clients, required execution profiles, offline/export and deployed smoke; join Q01–Q12 receipts | WP12 |

For Q04/Q06, use independent fixture facts, declared semantic properties and actual source evidence.
Any temporary comparison with the pre-pivot executable is development-only and removed before
WP11; a shipped legacy engine is not an acceptance oracle.

### Existing command surface and run discipline

- Start with `just --list` and `just toolchain-check`. Use focused affected-crate Clippy/tests for
  Rust changes and `uv run ruff check`, `uv run ty check`, affected contract tests for Python changes.
- After authoritative schema edits run `just schemas-generate` then `just schema-conformance`.
  After dependencies change run `just deps-policy` plus locked Cargo metadata/type-universe checks.
- Use `just architecture-check`, `just rules-test`, `just rules-scan`, `just adr-lint` and
  `just provenance-check` at their relevant boundaries. Rules must have meaningful fixtures.
- Each package runs its small decisive Q-oracle as soon as the new path exists. Expand only for new
  changes/failures. In particular, do not repeatedly replay installed clients while replacing a
  normalizer, or repeatedly rebuild all native dependencies to validate documentation.
- At WP12 run the final integrated quality/test surface, relevant execution and live tiers, actual
  `just client-acceptance` workflow, then `just acceptance-report` and `just acceptance-check` from
  recorded final-source evidence. Use the acceptance skill at that reporting boundary. Required
  producer/policy runs include C20 and `just state-leak-check` for every enabled execution profile.
- Keep exactly `passed`, `failed`, `blocked`, `not_run` for acceptance. Record command, log, source,
  pins, profile and bounds. Mocks, source inventories, native plan shape and upstream mechanism
  probes cannot alone pass installed functional/recovery gates. G1–G7 review readiness is separate.

Q12 has a source closure before candidate qualification and an installation closure after WP12
activation/removal. Q13's final aggregate is recorded after that second closure and deployed smoke;
do not require deletion of the active old installation to build and qualify the fresh candidate.

## 8. Complete native integration disposition

Maximum reuse does not mean instantiate every optional remote system or implement every private
trait. Every reviewed integration has a disposition and a current consumer or a precise boundary.
These dispositions do not defer any mandatory native service dataflow to procedural legacy code.

| Integration family | Required disposition / consumer | Owner |
|---|---|---|
| Delta TableProvider and CDF provider | Exact snapshot/cohort reads and actual bounded incremental projections | WP03, WP09 |
| Delta TableProviderFactory | Controlled opening/options where appropriate; explicit capture/admission before publication | WP03 |
| ListingSchemaProvider | Reuse for actual controlled discovery; bounded inventory, normalized-name collision/error handling, then immutable capture | WP03 |
| Unity SchemaProvider/CatalogProvider/CatalogProviderList | Explicitly not a required service dependency: no current Unity source. If introduced as an actual source, use upstream adapters and qualify failed-list/credential expiry/version capture before claiming support | WP03 capability record |
| DeltaPlanner / DeltaExtensionPlanner | One aggregate Delta extension planner composed with native service planners under retention; inherit five internal delegates | WP01, WP03 |
| Eight ExecutionPlan / nine DisplayAs implementations | Inherit current native validation/mapping/merge/metrics/scan paths; display for humans; retired physical DeltaScan wrapper excluded | WP01, WP03, WP10 |
| PruningStatistics implementations | Native partition/file/statistics pruning and exactness; no secondary file-membership or aggregate-answer authority | WP03, WP10 |
| DeltaLogicalCodec | Qualified read-provider payload only, checked and rebound; typed descriptors own job/CDF replay | WP09 |
| DeltaPhysicalCodec | Excluded: deprecated retired-node codec; regenerate current physical plans | WP09 |
| DataValidation / MergeBarrier / MergeValidation / MetricObserver logical nodes | Inherit via public operation builders, without copied node/planner registries | WP03 |
| MakeParquetArray / ToJson / ZOrderUDF | Inherit through their supported parent routes; no private helper imports, JSON canonical identity or Z-order semantic score | WP02, WP03, WP10 |
| DeltaDataWriterExt / native datafile writers | Logical-input builder is complete default; use existing physical writer only with qualified upstream preparation when an actual consumer requires it | WP03 |
| DataFusionEngine / kernel Engine | Inherit native scan integration; audit separate LogStore default-engine resource consumers | WP01, WP10 |
| ObjectStore implementations | Qualified local conditional/durable path mandatory; ConditionalPutShim excluded; S3/experimental DeltaIO only for an actual configured deployment and separate route qualification | WP03, WP10 |
| ParquetObjectReader / FileStream | Native ranges/footer/page indexes and strict finite streaming; no second file reader or silent skip-on-error | WP05, WP10 |
| Kernel SchemaComparison | Private; native Delta mapping/validation plus application semantic mapping tests, no copied compatibility engine | WP02, WP03 |
| Kernel RetentionCalculator | Private; inherit through native properties/checkpoint/log compaction, augment only service retention inputs | WP10 |
| ExpressionItem / SchemaPatchItem / ProjectionStructPatchBuilder | Public paired schema/expression edits for actual kernel boundary projection; ordinary rules remain DataFusion Expr | WP02 |
| Serde Deserialize / Serialize | Native protocol/action encodings and mechanical wire bytes; not semantic canonicalization or runtime-handle preservation | WP02, WP08, WP09 |
| TableFeature EnumCount / IntoEnumIterator | Derive feature inventory and join exact protocol, route, policy and conformance receipts; enumeration alone never advertises support | WP03, WP11 |
| TableMetadataUpdate Validate / ValidateArgs | Reuse native metadata-update validation for metadata commands; no generic row-rule interpretation | WP03 |
| ReportGeneratorLayer / MetricsReporter | Bounded structured kernel events joined with native operation/plan metrics | WP01, WP10 |
| Broader provider-map APIs | Native source/format/sink/adaptation/defaults/pushdown contracts from §3.2; async catalogs/UDTFs/statistics registry only with an actual consumer; native scheduling inherited | WP01–WP03, WP05, WP10 |
| DataFusion FFI | No cross-process use; worker transport is Arrow IPC. No in-process plugin requirement is invented to consume FFI | WP05, WP11 |

Remote catalogs/storage are the reviews' conditional capabilities, not a hidden implementation
backlog. Record them as unconfigured/unsupported for this local deployment. If the target already
has an enabled real consumer at execution start, it becomes required scope with route-specific
credential/error/consistency tests; do not silently remove that functionality to use this exemption.

## 9. Finding-to-delivery traceability

| Review finding | Required work | Closure evidence |
|---|---|---|
| F01 — Validated Delta mutation boundary | WP01–WP03 | Q02, Q10; L03 |
| F02 — Claims and coherent publication | WP04, WP09, WP10 | Q03, Q05, Q08–Q10; L01, L02, L04 |
| F03 — Semantic mappings and trusted metadata | WP02, WP03, WP06 | Q02, Q04; L06, L09 |
| F04 — Owned effects, not volatility | WP01, WP04, WP07 | Q03, Q05, Q11; L10 |
| F05 — Obstructing enforcement | WP00, WP11 | Q01, Q12; L13, L14 |
| F06 — Arrow early/native normalization | WP05, WP06 | Q04, Q13; L05–L07 |
| F07 — Complete operation/policy authority | WP00, WP07, WP08 | Q01, Q05, Q06; L10 |
| F08 — Native scoring | WP08 | Q06; L08 |
| F09 — Native canonical identity | WP02, WP10 | Q04, Q09; L09 |
| F10 — Native result composition | WP08 | Q06, Q13; L11, L13 |
| F11 — Delta persistence/maintenance | WP03, WP04, WP10, WP11 | Q02, Q03, Q09, Q12; L01–L04, L12 |
| F12 — Complete resource/lifetime accounting | WP01, WP04, WP07, WP10 | Q05, Q10, Q11; L03, L10, L12 |
| F13 — Route-specific capability evidence | WP00, WP03, WP09, WP11, WP12 | Q01–Q13; §8 dispositions |
| F14 — Planner/configuration composition | WP01, WP03 | Q02, Q05, Q11 |
| DFU-01 — Codec and replay boundary | WP09 | Q08; no generic physical replay |
| DFU-02 — Retention/conditional storage | WP03, WP04, WP10 | Q03, Q09, Q10 |
| DFU-03 — Complete builders/private node reuse | WP01, WP03 | Q02, Q12 |
| DFU-04 — Queryable incremental CDF | WP09, WP10 | Q07, Q09; L12 |
| DFU-05 — Discovery versus publication binding | WP03 | Q02, Q03; §8 conditional remote checks |
| DFU-06 — Native metadata/scan/kernel integration | WP01, WP03, WP10 | Q02, Q11 |
| DFU-07 — Paired schema/expression edits | WP02 | Q04 |
| DFU-08 — Native structured telemetry | WP01, WP08, WP10 | Q06, Q11 |

## 10. Execution recording and unresolved prerequisites

Record progress in this document when execution starts: each WP's actual changed/deleted paths,
remaining work, Q receipt and source identity. Add a separate ledger only if the execution volume
requires one; do not create competing status authorities. Update STATUS at meaningful checkpoints
with the exact implemented/remaining boundary. Never mark a WP complete because it only compiles,
because its scaffolding exists or because an earlier architecture passed a similar test.

The direction and mandatory scope are resolved. Implementation must still produce evidence for:

- Full Arrow/Delta schema fidelity and native recursive normalization at representative scale.
- Logical mutation/control integration, concurrency, process recovery and shared retention fencing.
- Actual local conditional creation and required durability, including power-loss limits.
- CDF/replay retention horizons and physical resource measurements on real workloads.
- Actual installed clients and required producer profiles using only the new state/architecture.

These are assigned work and qualification prerequisites, not invitations to retain legacy code.
Private/unsupported upstream APIs do not justify recreating Delta subsystems: use the selected
complete public route or make the smallest necessary upstream composition improvement with exact
pin/receipt. An unavailable mandatory oracle remains open; it cannot be recast as an optional
capability disposition. No additional user design decision is required to begin WP00.

### Implementation checkpoint — 2026-09-16

All WPs remain open. This records actual replacements, not package completion.

| Work | Changed/deleted source | Implemented boundary | Remaining |
|---|---|---|---|
| WP00 | ADR-0041–0045, Cargo/lock/source policy, architecture check, `15-native-enforcement.patch` | Exact Delta/kernel/DataFusion pins; native type-universe checks; remove utility duplicate skip list; worker Arrow boundary; superseded decisions | Apply protected harness patch, finish generated epoch/positive rules and operation inventory |
| WP01/WP03 | `store/runtime.rs`, `native_delta.rs`, `native_policy.rs`, `durable_store.rs` | Aggregate DeltaExtensionPlanner and native Arrow metadata planner inside retention planner; explicit shared native session; complete logical-input builder with CHECK and no silent rebase; acknowledged file/directory synchronization; shared row/byte/batch/file writer settings | Complete effect grants/ownership/resource accounting, qualified mutation/retention surface, power-loss qualification |
| WP02 | `native_delta.rs`, `semantic.rs`, core `native_identity.rs`/`native_key.rs`/`evidence/arrow_model` | Explicit UInt64→Decimal128 mapping; native schema registry; shared Arrow canonical byte kernel; 26 native identity contracts including evidence/provenance/policy/process; core Arrow schema authority; native field/tag/vocabulary/coordinate/ID checks; nullable nested read layout with enforced semantic presence; no missing-feature-knowledge reader fallback | Complete intrinsic/resource admission, policy/result identities and recursive schema patches |
| WP04 | `control.rs`, `control_jobs.rs`, daemon jobs and consumers; deleted `catalog_generation.rs` and JSON journal implementation | Typed Delta control publication and thirteen finite record families; native current-state views; snapshot-bound conflict re-evaluation; native claims/interests/cancellation and UTC timestamp fields; policy-derived lease times and fenced native renewal; owned heartbeat cancellation; live-lease publication admission; captured publication owner/fence and atomic terminal view; async daemon calls | Complete physical-owner cleanup and qualify native renewal/expiry; prove acknowledgement/restart/cancellation/recovery; implement retention/cleanup families |
| WP03/WP05 | `delta_evidence.rs`, `projection/publication.rs`, `repository.rs`, `bundle.rs`, Arrow ingress; removed ExactParquet and `parquet_admission.rs` | Ten purpose-specific Delta tables; epoch-7 typed exact table/version/cohort/schema vector in the control record; no evidence manifest file; fresh native bundle writes/rebinding; private Arrow IPC inputs | Complete native normalization, artifact/result authorities, durable retention; bounded upstream resource qualification |
| WP05 | core registry/Python facts and version kernels; store `registry.rs`/`python_registry.rs`; daemon consumers | Typed Arrow index/file facts; native SemVer/PEP 440 selection, neighbours, wheel/interpreter eligibility and marker expressions over supplied environments; deleted Rust sorting/tag/marker evaluators | Durable acquisition/registry authority, native dependency frontier, source policy and full normalization |
| WP06 | `repository.rs`, `coverage_plan.rs`, `leases.rs`, `python_normalize.rs`, `rust_normalize.rs`, core rustdoc facts, generated worker IPC and daemon consumers; deleted both semantic walkers, old worker responses, Rust JSONL transport, symbol/relationship object ingress and reference staging | Shared native contribution; native unions/environment derivation; exact-vector admission; native coverage; retained Arrow sources; Python recursive alias closure, membership joins, native evidence/IDs, UNNEST and failure dispositions; policy-bound closure depth; native Rust visibility/containment/use/member plans and independent signature alternatives; direct logical Delta writes | Procedural metadata/attempt merge, Python scale qualification and full resource/ownership qualification |
| WP09 | `DeltaStore::changes`, `EvidenceTables::change_plan`, `search_projection.rs`, manifest resource and generated DTOs | Native CDF change explanations; materialized API/fragment search surfaces maintained by lineage joins; atomic typed output/input checkpoint; build-derived complete local source/compiler/config/lock revision; native command revision guard and actual decoder-build provenance; recorded full rebuild after missing history, explicit CDF endpoint checks; fresh export projections | Complete CDF mutation/revision/fault matrix, typed replay/codecs and retention |
| WP08 | `scoring.rs`, `search_plan.rs`, `projection/score.rs`, core search | Removed whole-score UDF, Rust tokenizer, custom match-clause IR and ranking array encoder; native regex/string/array expressions supply tokens, eligibility, factors, score and explanation | Result relations/delivery and remaining research/policy operations |

Focused receipts under `.dev-state/plan15/` include actual native Delta writes/CHECK refusal,
control concurrency/pinning/compaction, native ranking and ordered identity execution. These are
boundary checks only; **Q01–Q13 remain not_run as terminal target gates**. Current source has not
been installed. The existing Plan 14 installation/state has not been modified or deleted.

Current receipts, 2026-09-15: workspace/all-target compilation, schema generation/conformance,
ADR/index/register lint, and four native scoring cases passed. `native-projection-publication.log`
passed real Delta publication/opening, foreign-table refusal, fragment-change explanations,
incremental/full search multiset equivalence, unpublished cohorts, actual OPTIMIZE, fresh export,
native checkpoint plus expired final CDF log, explicit definition/history rebuild and unselected-output
isolation. `native-jobs-tests.log` passed native claim construction, UTC Arrow timestamps,
policy-derived future leases, shared interests and stale-owner refusal. Strict all-target Clippy
passed after the CDF/timestamp edits. The shared native contribution and subsequent provider metadata passed strict all-target Clippy. These are scoped receipts; the full
conflict/crash/cancellation/maintenance matrix is still open.

Additional receipts: `native-value-kernel-tests.log` passed three Arrow/SemVer/PEP 440 cases;
`native-identity-tests.log` passed twelve cases with independently calculated typed-byte/SHA-256
vectors. `native-identity-schemas.log` passed regenerated schemas/DTOs and conformance for full-hash
identities. Three native Rust/Python selection and marker fixtures passed using native DataFrame UNNEST. These do not close Q04 or any work package.

September 16 additions: `native-evidence-arrow-tests.log` passed six core evidence round trips
including Parquet/DataFusion after the nested-layout change; `native-field-contract-tests.log` passed native invalid locator,
path and origin refusal. `native-identity-admission-tests.log` passed native binding identity
admission. `native-staging-ownership-tests.log` proved private directories survive optimized
physical plans and streams. `native-write-nullability-tests.log` proved full Delta writers admit
present values from nullable schemas and reject actual NULL without advancing the version.
`native-derived-environment-tests.log` passed source preservation and repeated derivation identity
(116 s). `native-plan-publication-tests.log` passed direct-plan publication/CDF/export (205 s),
before the latest field/layout/DTO changes; `native-direct-contribution-tests.log` passed the
shared-contribution path with those changes (203 s). `native-presence-contract-tests.log` refused
an actual NULL despite nullable nested Arrow layout, through the declared native presence rule.
Python additions: `native-python-evidence-tests.log` passed real isolated extraction through native
recursive/membership/UNNEST/aggregate plans and five full Delta writers/readbacks (5 s). Independent
core decoders prove identities and typed evidence; source/stub differences, chained aliases,
conflicting kinds, cycles, failed files, policy-bound depth exhaustion and truncated transport are
covered. `worker-python-contract-tests.log` passed four actual worker cases; generated Rust/Python
schemas and byte-identical packaged Arrow schema passed conformance. Strict core/store/daemon
all-target Clippy, worker Ruff/ty and the selected DataFusion capability-gap scan passed. The
worker's exact PyArrow 25.0.1 interface/wheel verification is in the compatibility matrix.

None is a terminal gate or full installed qualification.

`just doctor` passed. `just acceptance-check` failed because its stored report and logs refer to
the previous source digest; that report is not current target qualification. No old pass is carried
forward to Q01–Q13.

Known integration work remains explicit: old filesystem-based tests and architecture oracles must
follow the target contracts; physical claim cleanup and full renewal/expiry qualification are unfinished; durable retention,
native results and operation policy/effect ownership remain substantial work. Python uses generated Arrow IPC and native evidence plans; large/deep qualification and
remaining artifact/control/result authorities are open. Rust uses bounded Arrow facts and native normalization; artifact sidecars are now deleted, with native receipt/result qualification still open. Replay and the remaining projection fault/mutation
cases are open even though the product now consumes the materialized search checkpoints.
The protected enforcement patch is supplied but has not been applied. Reconcile it with concurrent
operator edits before applying; its earlier apply-check is not a current receipt. Protected paths
remain session-denied by the harness policy. This does not block independent work.

Additional implementation, 2026-09-16:

- Document/provenance ingestion now uses typed Arrow facts and native source/coverage/identity
  plans. Execution observations use native closure, coverage and result-to-attempt guards.
  Both enter `publish_native`. The old document object visitor and execution normalizer are deleted.
- Deleted the file-admission cache, semantic RelationWriter, dataset writer, old writer tests and
  unused cache/row-group knobs. Admission consumes native providers and uses native counts and
  canonical byte-length predicates. Raw document IPC remains a bounded mechanical source encoder.
- Complete native writes and query planning own their shared permit and operation context until
  task exit. Control current-state views now use native DataFrame windows/joins. Terminal artifact
  retention and transition share a separate bounded settlement scope.
- DeltaScanConfig supplies semantic read schemas, replacing
  the read-side ArrowContract. The runtime registers one durable local backend, shared by Delta
  builders and scans. No private Delta adapter or copied scan implementation is introduced.

Focused receipts: Rust four-stage normalization/five evidence readbacks passed (2.08 s), document
plans/Delta readbacks plus ambiguity/coverage refusal passed (1.45 s), seven native admission cases
passed (4.46 s), and bounded raw-fact IPC passed. Strict store/daemon all-target Clippy passed after
terminal settlement changes (3.19 s), before the latest scan changes. Full unsigned values,
read-provider DML refusal and identical registered backend handle passed (0.08 s).

Further implementation: EvidenceBatch/EvidenceSink and object publication APIs are deleted. All
fixtures use Arrow plans through publish_native. Documentation moved into its own Arrow column;
native projection now reduces leased/unleased inspection reads from about 1.02 MB to 21 KB while
preserving the rows (`native-column-projection-tests.log`, passed, 15.09 s).

The service now owns one bounded multi-thread native executor with explicit worker/blocking/stack
settings. Native startup, dispatch, metadata reads and complete writes execute there. Large futures
are boxed before tracing/task-local composition. The acquisition rerun reached terminal publication
without stack overflow (36.27 s), then failed because publication-completed jobs returned inline
content instead of a retained descriptor. Native transitions now carry the complete Artifact record
and job delivery consumes it directly. That rerun reached retained delivery but failed Rust declaration coverage; known external trait lookup classification has since passed the native fixture. The latest daemon journey is not yet requalified.

Latest replacement: finite NativeCommand logical/physical operators now execute all four job
drivers; the owned registry separates terminal publication from physical exit. Two native command
contract checks passed (0.01 s) and the pre-artifact driver ownership check passed (18.87 s).
Blocking-reader/sink permit ownership passed (0.08 s); stronger Rust declaration-scope evidence
passed (1.86 s), with bounded unresolved-input witnesses and explicit external inherited-detail
limits. Rustdoc format 61/59 pairing remains separately unqualified.

Fifteen native control families now include artifact_receipts and retained_results. These own
artifact lookup, section ranges and dependency/export closure. Blob sidecars, first-retrieval
metadata, prefix-directory scanning and procedural blob dependency traversal are deleted.
ArtifactHandle carries an exact acquisition receipt and full 64-digit content ID. Wire 3.0 schema
generation/conformance passed; full epoch cleanup and end-to-end qualification remain open. These
changes do not close WP08/WP10 or any terminal oracle.

Further architectural replacement, 2026-09-16:

- Wire 3.0/research-result/3 schemas and Python models have regenerated and passed conformance
  (four valid/five invalid cases), plus nine Python contract cases and read-only retained delivery.
- Native execution policy now joins captured receipt, effective configuration and raw process
  facts. Native regex/cast/arithmetic operations parse kernel resource output; native predicates
  own image/root/identity/resource/profile/cleanup prerequisites. Status, verification, semantic
  inspection and local rustdoc fallback consume the same policy. Independent ecosystems are
  qualified independently. The procedural qualification evaluator and resource parser are deleted.
  Three focused native policy checks passed (1.64 s); they do not qualify installed containment.
- NativeCommand implements the current statistics API and stable physical metric registration.
  It observes driver start/return/future-drop/refusal and wall time, separately from cleanup.
- Attempt inputs now remain Arrow/DataFusion through deduplication, actual-payload conflict
  checks, acquisition/log reference checks and control publication. Snapshot associations use
  the 19th shared native identity expression. Native metadata scope/configuration compatibility
  and aggregation replace object metadata rebase. ControlBatch accepts a native attempt plan.
- Control payloads use the shared nullable Delta read layout with required semantic values
  enforced through native field predicates. The existing ArrowContract/ProjectionExec boundary
  retains the complete tagged-struct schema. Focused comparison/export subsequently passed
  after native Delta staging removed repeated recursive planning (see the scoped receipt below).
- The latest daemon acquisition failed because its native worker executable was missing; that
  prerequisite was built. The complete source/worker/daemon journey remains to be rerun.

Further native acquisition work, 2026-09-16:

- Replaced dependency queue/map state with Arrow fact batches. DataFusion selects expansions,
  combines version demands/extras, detects retained-version conflicts, enforces bounds and emits
  ordered locks. Two focused closure checks and three registry checks passed.
- HTTP cache observations use a purpose-specific Delta table and control-owned body receipts.
  Native plans select representations, negative-cache age, validators and 304 metadata. The
  JSON cache and its separate body store are removed.
- Shared native network policy consumes a bounded URL syntax UDF and captured DNS answers.
  Redirects get fresh admission; the HTTP driver binds a fresh reqwest client to admitted addresses
  and one monotonic deadline. Exact reqwest 0.13.5 integration evidence is in the compatibility
  matrix. Native HTTP/cache tests and the actual loopback driver check passed.
- Export selects each typed record family before encoding the control union and stages the
  selected native DAG once in Delta. This replaces a repeated inline recursive plan that was
  interrupted after more than five minutes. Staged comparison publication and verified export
  subsequently passed the focused journey (174.11 s).

Further native command and resolution work, 2026-09-16:

- Native resolution routing now joins exact release/environment/context/head and publication
  facts, including local-build eligibility, offline and revalidation. Acquisition preserves Rust
  registry spelling and natively normalizes Python names. The focused routing check passed.
- One core Arrow contract owns all four command argument shapes and the complete configuration.
  Native policy/command/grant identities replace resolve/compare/inspect/verify JSON/Debug job keys.
  Commands bind exact table/version/contract policy witnesses; the complete configuration is
  stored once in `delta/operation_policies`. Identity coverage and dependency invalidation passed.
- Native command preconditions join current qualification/profile routes and exact execution
  context/snapshot inputs. Claims retain route/image, command/policy bindings and a grant ID.
  Renewal rechecks current policy and compiled operation. Private grants propagate through native
  effect/task scopes; HTTP and decoder entry points enforce live ownership. A changed fetcher
  configuration cannot consume another policy's grant.
- Control row packing now maps by declared field name. This fixes incorrect values when native
  projections reorder fields. The real Delta claim/interest/expiry/settlement fixture passed
  (92.05 s); strict all-target Clippy passed. Cold acquisition failed before offline replay
  (159.22 s); focused diagnosis confirmed operation-budget exhaustion during control admission
  (159.35 s). Moving policy into its own Delta relation removes wide repeated configuration from
  control validation. Exact policy capture/reuse and wrong identity/contract refusal passed
  (0.26 s). The next cold acquisition published but immediate reopen correctly hit the writer
  lock while its driver reconciled (137.29 s). Shared Service shutdown/drain now handles this
  lifecycle before reopen. Publication also materializes its finite Arrow contribution once
  before admission, avoiding repeated producer-DAG evaluation. These changes pass strict Clippy;
  the complete cold/offline journey with shutdown and durable settlement passed (271.14 s),
  recorded in `native-control-drain-offline-journey.log`.

- Contained process policy is now native: exact configuration/image/containment checks, explicit
  local-build opt-in and live command grants. A shared immutable Delta definition consumer retains
  policy, process operations and effects; physical runners consume the selected operation and
  record typed operation/authority witnesses. Native map ordering replaces operation JSON identity.
  Protocol 3 replaces the old identity contract. Warm LSP reuse rebinds to each command and live
  ownership is checked per request. Fixed operator qualification is separately admitted and cannot
  consume target inputs, outputs, caller argv or acquisition network. Four focused native route/
  grant checks passed (55.18 s); fixed qualification scope passed (0.08 s); generated schemas passed
  conformance. Exact retained-operation readback and grant revocation subsequently passed
  (54.74 s), recorded in `native-process-readback-tests.log`.

- Diagnostic observations now use one core Arrow schema and the Delta `native_events` relation.
  A bounded memory-reserved ingress/writer replaces Rust history queues, the JSON failure file and
  Rust summary counters. Native limits, window joins and aggregates serve history, failure support
  and operation totals. The shared executor/memory/spill runtime has one reserved diagnostic
  admission lane and no recursive self-observation. Status explicitly reports dropped observations;
  shutdown includes a persistence barrier. All 14 focused runtime contracts passed (0.84 s).
  Writer termination/fault/retention qualification and remaining daemon component counters are open.

These changes close no work package or terminal oracle. Complete contained-process and warm-LSP
physical qualification, full environment/input scope, native result composition, remaining acquisition policy,
retention/maintenance/replay, diagnostics persistence, remaining provider integrations,
old epoch cleanup and installed qualification remain required.

## Outcome (recorded after implementation)

Not complete. Continue from the boundaries above; do not treat compilation or these focused checks
as product acceptance, activate this tree, or retain the unfinished old authorities in the final design.
