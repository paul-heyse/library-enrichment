# Target design: a Delta Lake decision reference

Proposed 2026-09-18. This is a design for the portable skill, independent of its host repository.
The active skill remains unchanged during this assessment. See the
[implementation plan](IMPLEMENTATION_PLAN.md) for sequencing and acceptance.

## Start with the implementation decision

The reference should answer: **given this representation, table state, required effect and
execution context, which built-in should I use, what does it consume and produce, and what must
I preserve around it?** An experienced agent needs distinctions it cannot safely infer from a
method name, rather than tutorials on Rust, ACID or DataFusion.

The DataFusion implementation supplies a useful architecture: broad exact API coverage,
selective reviewed capability contracts, several small routes into those contracts, bounded
retrieval, executable controls, and portable reader/research bundles. Delta needs a different
semantic center. Its operations change durable state; outputs include **table versions, log
actions, object-store effects and failure phase**, as well as Rust values.

Use a short entrypoint with this flow:

1. Find a capability by task or by the representation already in hand.
2. Read its decision brief: prerequisites, alternatives, inputs, outputs, effects and limits.
3. Open exact member/trait contracts and a small composition only as needed.
4. Inspect source or run a retained probe when an unresolved condition affects the design.

Keep symbol/alias lookup available throughout. Do not force every caller to traverse every
route, or require compiler tooling for an ordinary reference lookup.

## Four views over the same reviewed records

| View | Questions it answers | Example distinctions |
|---|---|---|
| Task | What already implements this requirement? | Query a current snapshot; replay an event batch; consume changes; compact files; remove expired objects |
| Representation | What can I do with what I already have? | URL/options → table handle → loaded snapshot → provider → plan → stream; batches/plan → write; actions → prepared commit |
| Crate and integration | Who owns this capability and how do I access it? | Facade/core/kernel/engine/backends/catalogs; public entrypoint versus internal DataFusion implementation |
| Lifecycle and effects | What state changes, and when? | Read-only planning versus file staging versus log publication versus post-commit work; historic read versus restore |

These are projections, not four independently authored copies. The proposed
[crate roles](examples/crate-roles.tsv) cover all 13 acquired packages, including the facade that
has no locally defined canonical items. Broad coverage does not mean every backend has live
behavioral qualification.

Representation routes should explicitly distinguish:

- Loaded and unloaded `DeltaTable`; cheap handle construction versus log replay; eager versus
  lazy snapshot; historic version versus latest resolved at a particular time.
- A provider fixed to a snapshot, a logical/physical plan, a `SendableRecordBatchStream`, and
  materialized batches. Their lifetimes and retention costs differ.
- Delta log `Add`/`Remove`/transaction actions, selected file metadata, Parquet objects, and
  logical rows. Reading a directory of Parquet is not equivalent to reading a Delta snapshot.
- Delta/kernel schema and predicates versus Arrow schema/arrays and DataFusion expressions.
  Record explicit conversion edges and fidelity limits rather than matching names.
- Staged writer data, a prepared commit, a published version, and the returned updated table.
  A successful intermediate step is not completion of all later steps.

## One small contract model, with Delta-specific effects

Reuse the useful DataFusion fields—stable ID, task aliases, conditional alternatives,
operation references, inputs/outputs, claims, evidence, unknowns and invalidation dependencies.
Add optional operation-specific sections rather than making every schema conversion describe
transactions. The [sample record](examples/capability-record.json) demonstrates the shape.

| Section | Required information when applicable |
|---|---|
| Choice | Requirements under which this capability wins; simpler alternative; condition that reverses the choice; limitations of each alternative |
| Access | Canonical owner and verified import path; facade and Cargo aliases; required trait imports; direct public / inferred returned type / internal-only / feature-excluded |
| Inputs | Concrete types with generics and lifetimes; borrowing/consumption; table loadedness/version; Arrow/Delta schema; nullability and casting; SQL string versus DataFusion/kernel expression |
| Construction and execution | Constructor; fallible clause configuration; `IntoFuture::Output` or explicit `build`/`execute`; execution context; defaults and their binding time |
| Outputs | Updated table and its version; metrics fields/units/meaning; schema and row semantics; stream/plan consumption; whether no-op can return without a new version |
| Effects | Log/data files read, staged, added, removed, physically deleted; commit count; version publication; cleanup; old snapshot/provider behavior |
| Errors | Phase at which failure occurs; known durable effects; pre-commit validation versus conflict versus post-commit failure; supported recovery observation |
| Configuration | Feature profile, table properties, per-operation options, session/runtime policy; which wins; which defaults merely target size rather than guarantee a bound |
| Resource behavior | Materialization, buffering, merge source reuse, per-file concurrency, spill/runtime ownership and cache scope; measured facts versus unknown costs |
| Evidence | Exact source/doc locator, interpreted claim, observed test assertion and conditional recommendation as distinct records |

Never synthesize idempotency, ordering, uniqueness, atomicity across tables, bounded memory,
or schema preservation from a signature. A persisted application transaction action and a
caller replay-suppression policy are separate facts.

For errors, use a compact operation timeline with links to the relevant methods:

`construct → validate/plan → stage objects → publish log version → refresh/checkpoint/hooks → return`

Each operation may omit or repeat stages. Optimize may involve multiple commits; low-level
writers can stage files before their commit call. Do not turn this illustration into a universal
single-commit state machine. State whether cancellation/error leaves effects known, absent,
possible or unmeasured; avoid inventing cleanup guarantees.

## First reviewed capability set

The initial release should deeply characterize the following 18 families. Coverage may use
several small briefs within a family; a minimum count of pages is not the acceptance criterion.

| ID | Decision and required characterization |
|---|---|
| delta.open | Handle construction, loaded state, version/timestamp selection, refresh and cheap opening paths; backend/options requirements |
| delta.read | Delta provider versus `scan_table` versus raw Parquet; snapshot freshness, projection/filter/limit semantics, partition reconstruction and deletion-vector boundaries |
| delta.session | Caller `SessionState` versus another `Session`; fallback policy; runtime/store registration, planner/UDF preservation and resource policy |
| delta.write | Batches versus input plan versus low-level writer; staging/flush/commit; save mode, partitions, file sizing and returned table |
| delta.replace | Full overwrite versus `with_replace_where`; data validation and preservation of unmatched rows; interaction with schema/partition replacement |
| delta.schema | Arrow ↔ kernel schema conversions; schema merge/overwrite, add columns/metadata, field identity, casts/nulls/timestamps and physical/logical names |
| delta.merge | Ordered matched/unmatched/by-source clauses; fallible closures; aliases, duplicate matches, null keys, source materialization, schema evolution and metrics |
| delta.dml | Delete/update versus merge; predicate scope, file rewrites, no-op behavior, constraints and returned metrics |
| delta.replay | Transaction marker persistence and lookup, caller replay checks, concurrent conflicts and ambiguous outcomes; retention of markers |
| delta.commit | High-level operation versus low-level commit APIs; snapshot assumptions, conflicts/retries, publication, post-commit hooks and recovery evidence |
| delta.cdf | CDF provider versus explicit plan; enablement, version/timestamp bounds, change schema/metadata, row filtering, out-of-range behavior, resume progress and retention |
| delta.features | Recognized protocol tokens versus read/write admission versus operation support; feature-specific prerequisites and evolution consequences |
| delta.optimize | Compact versus Z-order; selection, target size, resource policy, metrics, multiple commits and rewrite semantics |
| delta.retention | Vacuum full/lite, dry run, retention, kept versions; checkpoint/log cleanup versus physical data deletion; restore after deletion |
| delta.history | Historic snapshot read versus restore-as-new-commit; missing files, protocol changes, audit history and CDF consequences |
| delta.storage | Scheme/handler/factory selection, root versus prefixed object stores, options/credentials, conditional publication and backend-specific guarantees |
| delta.catalog | Direct table URL versus Glue/Unity catalog lookup/provider integration; discovery, credentials, freshness and optional features |
| delta.kernel | Use delta-rs high-level operations versus kernel snapshot/scan/engine/committer interfaces; delegated functionality, custom engine cost and internal API caveats |

Keep conversion-to-Delta, filesystem check, manifest generation, constraints, observability and
less-used maintenance APIs discoverable in the broad operation index, with explicit depth status.
Promote them to reviewed briefs when failed tasks or real questions justify it. Python APIs remain
a separate surface: the pinned workspace's Python crate is excluded from this Rust acquisition;
do not borrow Python semantics as proof for Rust builders.

## Recover facts before authoring summaries

Preserve full module/member docs, associated types/constants, fields and variant payloads,
generics/bounds, raw type trees, visibility/attributes, source spans and artifact-scoped links.
The existing rustdoc acquisition already contains much of this information. The active model
keeps only name/signature/summary/via-trait on methods; the full contract must remain addressable.

Delta's private supplement requires extra care:

- Public capture determines public membership. The private capture can recover methods/impls of
  a public-returned unnameable type, and separately index internal integration machinery.
- Join captures through validated owner/item structure and canonical identity, never rustdoc
  numeric IDs across artifacts. Keep raw IDs with their artifact digest for traceability.
- An internal trait implemented by a public type does not make all trait methods callable.
  Public-looking `fn` records within private trait impls require accessibility classification.
- Compile a positive inferred-builder use and negative direct-import/hidden-trait controls.
  Resolve ambiguous paths conservatively; make ambiguity visible instead of minting imports.
- Follow structured return types, not the final `->` substring in a formatted signature.
  Merge closure parameter/result types are not constructors of standalone update/delete operations.

Discover operations through the public table/writer API, explicit async methods, trait-associated
outputs and registration paths. `IntoFuture` and `with_*` remain useful indexes but cannot define
the complete capability surface: CDF uses explicit building and merge uses `when_*` clauses.

## Protocol and integration need two separate matrices

The protocol matrix should have rows keyed by feature and columns for recognition, protocol
requirement, compiled admission, scan interpretation, write/preservation support, DML support,
maintenance constraints and executed fixtures. Each cell has its own evidence and profile.
An enum member, admission-set membership, and correct handling of a real fixture are different
claims. Record unsupported, feature-excluded, unknown and not-run distinctly.

The integration matrix should join foreign trait implementation → implementing type → visibility
→ public entrypoint → session/registration prerequisites → useful task → trait contract. Retain
all implementations, including internal `ExecutionPlan`, logical node, UDF and stream machinery.
Do not suggest implementing or importing those internals merely because they appear in the index.

Bridge to the companion DataFusion skill by package identity and symbol, with a compatibility
check. Bundle the small integration contracts needed for ordinary Delta tasks so a copied Delta
reader still works without a sibling skill. Avoid duplicating the whole Arrow/DataFusion corpus.

## Commit-based acquisition and reproducibility

Keep the chosen delta-rs commit, kernel commit, registry closure, feature resolution, compiler,
target, raw artifacts and source bytes together. The synthetic package version `1.0.0+58f07cd6`
is a reference label, not a published docs.rs version. Kernel aliases are Cargo access names:
`delta_kernel` → `buoyant_kernel`; `delta_kernel_default_engine` → `buoyant_kernel_engine`.

The current acquisition's locked registry versions match the DataFusion reference:
DataFusion 55.1.0, Arrow/Parquet 59.3.0 and object_store 0.13.2. Keep separate profiles for the broad
13-package documentation capture and the narrower executed local probe. Compatible major versions
alone do not prove one Rust type universe, feature composition, or consumer compilation.

Separate three commands conceptually:

1. **Replay** an existing capture using its retained Cargo.lock, exact sources and dated toolchain.
2. **Refresh** a selected profile intentionally, producing a new immutable acquisition and review diff.
3. **Check branch drift** as advisory maintenance; never silently update a reader or probe.

The existing builder records a lock hash but the tracked acquired directory does not contain that
lockfile; this assessment recovered and retained the matching lock. Package it in future research
bundles. Do not make branch re-resolution the replay path: it can fail after the branch moves even
when all original source commits are still available. A cache/envelope identity should bind the
resolved lock as well as requested features. Preserve both public/private capture identities and
actual resolved features. Rustdoc generation remains a separate dated-nightly producer; normal
consumer probes use stable Rust.

Acquire sources outside any consumer working tree. Do not edit upstream manifests to conceal
branch dependencies. If a future explicit patch is needed, give the patched capture its own
identity and retain the patch. Reader/rebuild operations should not trigger acquisition.

## Retrieval, packaging and maintenance

Adapt the DataFusion reader's `find`, `show`, `compare` and continuation mechanics, keeping direct
Markdown/JSON/TSV access. Default hits should show a few candidates and their decisive conditions;
raw API trees and full evidence are opt-in. Add facets for effect, snapshot policy, representation,
backend, protocol prerequisite and evidence status only where they improve actual tasks.

Reuse tested preservation, identity, receipt, packaging and invalidation mechanics by explicitly
porting small modules or extracting a minimal shared component. The Delta skill must not import
host-project modules or require an installed sibling skill. Do not start by creating a universal
ontology, database, service, HIR index or new dependency framework.

Ship two bundles:

- **Reader:** routes, reviewed briefs, full admitted API contracts, concise source/evidence excerpts,
  minimal integration contracts, bounded reader, provenance and license notices. No compiler needed.
- **Research:** reader plus authoring, retained public/private rustdoc, source archives or explicit
  replay inputs, locks, query definitions/controls, Rust probes, profile receipts and evaluation data.
  Offline rebuild and offline Rust compilation are separate promises; vendor/cache the full dependency
  closure only if the latter is offered and tested.

Qualify a copied bundle from `/` with a base Python interpreter and `-I -S`, minimal environment,
hidden original/cache paths and file/network tracing. Python environment hooks can import the host
application even with `-I`; the DataFusion qualification caught that. Use a separate reproducible
research-build launcher where script-local imports are required. Measure sizes/startup/output;
avoid recursive bundling of extracted test bundles, targets or evaluation workspaces.

Invalidate claims when referenced source/API/defaults change, when profile/protocol/backend
assumptions change, or when the candidate capability set gains an alternative. Regenerate facts
automatically; review conditional recommendations and rerun affected controls before republishing.

## What the DataFusion pilot changes about evaluation

The prior pilot found richer contracts, but both reference arms selected supported core approaches
and the candidate required more commands/time. Therefore optimize for fewer unsupported assumptions
and correct choice reversals; measure retrieval cost separately. Do not promise faster answers from
more pages. Freeze candidate and baseline, keep oracles hidden, record actual accessed material,
compile evaluator compositions, and test held-out tasks. The
[evaluation specification](EVALUATION.md) defines the Delta cases and release decisions.
