# Target design: a decision-oriented DataFusion and Arrow reference

Proposed 2026-09-18. This document specifies the improved skill; it does not describe an
implemented retrieval system. The [assessment](README.md) establishes the observed starting point.

## 1. Organize around the decision an experienced agent is making

The primary route should be **task → relevant built-ins → distinguishing contracts → composition**.
Keep symbol lookup and crate browsing as equally accessible shortcuts. An agent that already knows
`RowConverter` should not have to complete a questionnaire or traverse a fixed research ladder.

Five useful entry modes:

| What the agent has | What the reference should return first |
|---|---|
| A task or desired effect | A short candidate set, conditions that distinguish candidates, and the best next reference |
| An input/output shape | Operations accepting that representation and producing the desired result, including conversions required |
| A symbol or method | Complete contract, canonical owner, usable import paths, examples, and alternatives |
| A crate or subsystem | Its role, high-value capabilities, boundaries, and neighboring crates |
| Existing custom code | Possible built-in replacements, prerequisites for equivalence, and evidence still needed |

Use both task language and API terminology: “gather rows,” “retain matching rows,” “encode composite
keys,” “align heterogeneous schemas,” “avoid reading excluded pages,” “reuse expression work,”
“preserve field metadata,” and “consume output incrementally.” Neither crate names nor exact symbol
names should be necessary to discover a capability.

The first response to a task should be small: typically two to five plausible options, a preferred
option under stated conditions, and a link to its contract. Broaden if the context is insufficient.
Do not pretend to rank universally: distribution, semantics, and data size may change the answer.

## 2. Use three complementary maps

### Task map

Organize capabilities by the effects the caller needs. The following is the proposed coverage
backlog, not a claim that all these areas have been characterized by this assessment.

| Task family | Built-ins to expose and compare | Discriminating questions |
|---|---|---|
| Represent tabular data | Arrays/builders, `RecordBatch`, `Schema`/`Field`, `DFSchema`, `ScalarValue`, `ColumnarValue` | Array vs batch vs expression? Scalar broadcasting? Qualified names? Metadata and nullability? |
| Select and rearrange | `slice`, `project`, `filter`, reusable `FilterPredicate`, `take`, interleave, concat, batch coalescing | Contiguous range vs Boolean mask vs index vector? Duplicate/order semantics? Allocation and zero-column batches? |
| Compare and encode keys | `arrow-ord` comparators/sort indices, `arrow-row` converters, DataFusion sort/group/distinct | One batch vs a relation? Null/NaN ordering? Composite keys? Stable external encoding required? |
| Convert and normalize | Casts and cast options, timestamp/decimal conversions, dictionary/view/nested representations, schema adapters | Lossy conversion permitted? Invalid value becomes null or error? Which metadata survives? |
| Compute built-in values | Scalar, nested, aggregate, window, and table functions; Arrow arithmetic/string kernels | Row-local vs group/window/set semantics? SQL name vs expression helper vs direct kernel? Registration and features? |
| Build and transform expressions | `Expr` helpers, tree traversal/rewrite, type inspection, simplification/coercion, physical expressions | Building vs analyzing vs evaluating? Schema required? Volatility, aliasing, nullability? |
| Form relational queries | `DataFrame`, `LogicalPlanBuilder`, SQL planning; joins, set operations, grouping, windows, unnest | Bag/set semantics? Correlation? Null keys? Output schema and ordering? |
| Expose existing data | `MemTable`, `ViewTable`, listing/streaming providers, catalogs, table functions | Materialized vs lazy data? Repeated reads? Source schema, ownership, registration scope? |
| Scan files efficiently | File sources, Parquet pruning, projection, page indexes, row selections/filters, schema adaptation | Which work is skipped: files, row groups, pages, decoding, or output rows? Residual predicate needed? |
| Integrate storage | `object_store` access, ranges, metadata, conditional writes, multipart/upload adapters | Object identity? Scheme/authority registration? Consistency and retry assumptions? |
| Execute and control resources | Streams, partitioned streams, collection, runtime pools, disk manager, caches, task context | Output retention vs operator state? Ordering? Spill participation? Cancellation and concurrency? |
| Observe and tune | Logical/physical explain, metrics, statistics, partitioning/ordering properties, configuration | Planning estimate vs observation? Which stage consumes the setting? What measurement discriminates choices? |
| Write and modify | DataFrame writers, `COPY`, sinks, provider DML, Arrow/Parquet writers | File format vs table mutation? Schema adaptation? Failure/partial output? Commit and transactional responsibility? |
| Exchange and persist | Arrow IPC/FFI/Flight, DataFusion FFI, protobuf, Substrait, Parquet | Arrays vs executable plans? Buffer ownership? Extension codecs? Registry/environment dependencies? |
| Extend only the missing behavior | UDF/UDAF/window hooks, provider/file source, planner/optimizer/execution traits | Which existing building blocks can be reused? Which correctness promises does the hook introduce? |

An extension trait is one possible implementation mechanism. Put existing providers, functions,
operators, kernels, and adapters alongside it. DataFusion built-in functions may themselves be
represented by UDF machinery; “UDF” in an implementation type does not mean the caller must author
a new function.

### Crate-role map

For every package, show its purpose, principal input/output forms, concrete entry points, and nearby
alternatives. Keep definition ownership distinct from import convenience and dependency advice.
The fact that a symbol is defined in `datafusion-session` is not by itself a reason to add that
direct dependency to an application already using a compatible facade re-export.

[The proposed 60-crate map](examples/crate-roles.tsv) is a seed for this view. It covers the complete
existing package set, with deliberately compact role descriptions. Expand a package into its
capabilities on demand. Do not use symbol count, trait count, or method count as relevance ranking.

### Representation and phase map

Make the bridges explicit:

```text
SQL / programmatic expressions
       → logical plan → analysis/coercion → optimization
       → physical plan → execution → streams of RecordBatch
                                          ↕
                         Arrow arrays / buffers / kernels / row encoding
                                          ↕
                              file formats / IPC / FFI / storage
```

This is an orientation diagram, not a promise that every API enforces every stage. Each operation
states where it acts and what work it performs eagerly. Provider discovery, schema inference,
metadata access, and planning can have effects before record-batch execution starts.

Cross-links should answer “what normally feeds this?” and “what usually consumes this?” For example,
sorting indices feeds `take`; a logical `Expr` is not a ready-to-run `PhysicalExpr`; Arrow row bytes
are not a substitute for a stable external serialization contract. Shared Rust type names alone
do not prove composability.

## 3. Make a capability contract the main explanatory unit

Use a short human-readable brief backed by structured records. A capability can have several
operations or implementation paths. Avoid one enormous page for every method and avoid copying
the same null/order/memory explanation into hundreds of function pages.

The standard brief should answer:

| Field | What the agent needs |
|---|---|
| Purpose and trigger | The effect this performs, with task aliases that do not require knowing the API |
| Choose when | Positive conditions under which it is a good fit |
| Alternatives and boundaries | Closest built-ins, the conditions favoring each, and what this capability does not do |
| Entry points | Canonical definition, practical imports, required registration/configuration, and relevant feature profile |
| Inputs | Rust parameter types plus semantic shape, schema, lengths, nullability, ordering, partitioning, ownership/lifetimes, and prerequisites |
| Outputs | Return type plus cardinality, row order, schema/metadata changes, null representation, allocation/sharing, and downstream-compatible forms |
| Execution/effects | Construction vs planning vs execution; synchronous vs async; I/O, caching, mutation, state retention, cancellation |
| Failure contract | `Result`, stream-item errors, documented panics, unsupported types, empty inputs, invalid values, and partial-output behavior |
| Implementation considerations | Lifecycle placement, reuse opportunities, optimizer visibility, resource accounting, concurrency, and caller obligations |
| Evidence and unknowns | Exact source/doc/member links, tested assertions and controls, configuration scope, unresolved conflicts |

Do not require irrelevant fields to become filler. “Not applicable: this function does no I/O” is
useful when relevant; fifty empty schema fields on a one-line constructor are not. Render a compact
decision brief first and the detailed contract only when requested.

### Preserve the type contract structurally

Keep the original rustdoc type trees for parameters, receiver, result, generic parameters, bounds,
associated types, and relevant impl context. The existing string signatures remain a convenient
display projection. Do not reconstruct semantic structure later by parsing those strings.

An operation identity should include package/profile, defining item, member, and impl/trait context.
`Self`, `Result`, generic parameters, associated types, and renamed imports must resolve in their
own context. Retain unresolved references as unresolved. A rustdoc-local item ID belongs only to
its artifact; it is not a durable symbol identity.

For `async fn ... -> Result<SendableRecordBatchStream>`, distinguish failure to create the stream
from failures yielded by its `Result<RecordBatch>` items. For `&mut self`, preserve state mutation.
For `Arc<dyn Array>` output, distinguish owning the handle from copying buffers. Generic signatures
can nominate a composition; a compiled example establishes only the instantiated composition.

### Author semantics as claims, not decorations

Keep the following records small and explicit:

| Record | Purpose |
|---|---|
| `Capability` | Task vocabulary, purpose, candidate entry points, selection conditions |
| `Operation` | Exact member/type structure, full docs, source location, observed availability |
| `ContractClaim` | A statement with conditions, scope, evidence kind, and source/probe references |
| `Choice` | Candidate set, decisive constraints, alternatives, rationale, and unresolved assumptions |
| `Composition` | Producer/consumer operations, bridging conversion, preserved/changed properties, example |
| `Probe` | Exact inputs/configuration, assertions, controls, observed outputs, run status, limits |

Use relationships such as `implements_capability`, `alternative_to`, `accepts`, `produces`,
`requires_registration`, `configured_by`, `composes_with`, and `supported_by`. JSON/JSONL and generated
TSV views are sufficient; this relationship model does not require a graph database.

Differentiate documented promises, source observations, compiled examples, runtime observations,
and authored interpretations. A local probe can demonstrate behavior without establishing a
general guarantee. An interpretation should name its premises. Store a conflict when documentation
and execution differ, and let the brief expose the practical consequence.

The [worked examples](CAPABILITY_EXAMPLES.md) illustrate this structure and its evidence boundaries.
The [illustrative capability record](examples/capability-record.json) shows how a gather operation,
its observed behavior, its documented panic condition, and its conditional recommendation remain
distinct. It is an authoring example, not an implemented schema or generated type model.

## 4. Deep characterization should concentrate on semantic seams

Broad discovery should cover the full available public surface. Deep review should first cover
places where similarly named operations can change correctness or architecture.

| Seam | Questions to characterize and probe |
|---|---|
| Schemas and metadata | Arrow `Schema` vs `DFSchema`; field vs schema metadata; nested field identity; qualifiers; extension types; schema merge/adaptation; which constructor/operator validates which invariant |
| Nulls and values | SQL three-valued logic vs kernel variants; null keys; null/empty nested values; dictionary nulls; NaN and signed zero; overflow, decimal scale, timezone and cast failure policy |
| Cardinality and order | Mask filtering vs gather indices; duplicates; unstable sorting/ties; bag vs set operations; ordering within/across partitions; zero-row and zero-column batches |
| Planning and execution | Constructor, analysis, optimization, physical lowering, execution, collection, cache; volatile functions and constant folding; when schema or function registry is required |
| Optimizer promises | Exact/inexact predicates, statistics precision, ordering/equivalence, constraints, partitioning, boundedness, extension-node visibility; what can go wrong if a promise is false |
| Storage/decode boundary | File/row-group/page pruning vs row filtering; projection root/leaf semantics; schema evolution; metadata availability; synchronous/async readers; object-store registration |
| Resources | Retained input buffers vs copied output; output streaming vs operator state; pool-accounted memory vs process RSS; spill support; cache ownership; cancellation and resource release |
| Functions | SQL aliases, expression constructors, argument coercion, return field, nullability, volatility, scalar/array broadcasting, aggregate state/merge/retract, window ordering/frame requirements |
| Persistence and interchange | Data representation vs executable plan; extension registries/codecs; buffer release ownership; semantic and version limits of row bytes, IPC, FFI, protobuf and Substrait |

These dimensions should be searchable facets, with prose explaining why a facet matters. They are
not a universal claim that every capability has been exhaustively proven across every Arrow type.

## 5. Repair and extend the evidence pipeline

### Recover information already acquired

Extend `build/model.py` to retain full member docs, module docs, field types/docs, enum payloads/docs,
typed signatures, original intra-doc link mappings, and source spans. Retain impl bounds instead of
flattening context. Avoid counting synthesized/derived methods as equivalent to distinct capabilities.

Extend `build/emit.py` to emit addressable member/field/variant sections or small member pages, and
rewrite rustdoc links to local targets when resolvable. Preserve unresolved links with an upstream
fallback and a diagnostic. The current method-signature blocks remain useful summaries.

Keep raw inputs and loss diagnostics: an unsupported type shape should not quietly become `_`
without a visible record. Field and variant names without types or documentation should not be
presented as complete input/output characterization.

### Build broad candidates, then review meaning

1. Join API/module docs, features, re-exports, upstream guides, and examples into candidate families.
2. Acquire published source and selected upstream tests/benchmarks as needed. Preserve manifests,
   archive checksums, source revision information, license notices, and source spans.
3. Mine doc sections and source for error branches, default values, null/order handling, registration,
   builder behavior, conversions, and implementation examples. These are candidates, not conclusions.
4. Use a DataFusion-specific extractor for function registries, signatures/aliases, optimizer rules,
   provider factories, and configuration. Macro expansion or a runtime registry capture can reveal
   entries missed by source text; record the selected profile and stage.
5. Write/review capability claims and choice comparisons. Link every consequential statement to
   evidence or identify it as a proposed hypothesis.
6. Run focused probes where an answer changes an implementation choice; render bounded reference
   pages and searchable views from the reviewed records.

Extraction should surface missing coverage. A new symbol, new registry member, or new usable built-in
can invalidate an old recommendation even when its previously selected API is unchanged. Track
candidate-set dependencies as well as direct evidence dependencies.

### Use each analysis tool for its actual question

This adopts capabilities documented by the ast-grep/ripgrep and Rust code-model skills. It proposes
no changes to those skills and does not require them to be installed beside a copied DataFusion skill.

| Question | Tool/source to use | What it does not establish |
|---|---|---|
| Where is terminology or a documented guarantee? | `rg`, explicit file inventories, module/member docs | Absence or semantic identity from an empty/matching search |
| Where are constructors, defaults, registrations, impls, or error branches written? | `ast-grep` with kind/field rules and positive/negative fixtures | Name resolution, dispatch, macro expansion, or behavioral equivalence |
| What is the public type contract? | Hosted rustdoc JSON, manifests, re-export resolution | Function bodies or runtime behavior |
| What dependency/features/target profile was selected? | Resolved `cargo metadata`, manifest/lock data, recorded build arguments | Code behavior; declared features alone do not prove selected features |
| What does this receiver/call/type refer to? | Focused rust-analyzer LSP or a pinned HIR capsule | Universal behavior or conclusions for configurations not loaded |
| Does this combination compile? | Small isolated consumer crate | Runtime correctness or resource bounds |
| What happens on these inputs? | Executed assertions with discriminating controls | Universal correctness or performance outside that experiment |
| Does a specific lowering/drop/control-flow issue need investigation? | Targeted MIR/dataflow, only when simpler evidence is insufficient | General library semantics or complete dynamic dispatch |

Do not use MIR as the default way to answer “what does this capability do.” Upstream prose, source,
tests, and a small probe are usually more directly useful. Do not bring up an HIR project for a
signature already available in rustdoc. The source/query examples in [evidence](evidence/README.md)
show the inexpensive end of this pipeline.

## 6. Storage, retrieval, and package layout

Proposed layout within the portable skill:

```text
SKILL.md                         short router and interpretation rules
reference.md                     layout and exact lookup recipes
authoring/
  tasks.json                     task vocabulary and decision routes
  capabilities/*.json            reviewed claims and comparisons
  crate-roles.json                curated roles joined to generated package facts
content/
  tasks/                         generated task routes
  capabilities/                  generated decision briefs and contract detail
  comparisons/                   generated cross-capability comparisons
  operations/                    generated member-level documentation
  modules/                       preserved module-level documentation
  index/                         existing TSVs plus tasks/operations/contracts/choices
  model/                         typed API records and resolved reference edges
  api/ traits/ catalogs/ corpus/  retained existing reference, with repaired rendering
  coverage.json                  discovery, characterization, and probe coverage separately
scripts/                         bounded find/show/compare helpers, optional to use
build/                           acquisition, extraction, joins, validation, generation
evidence/                        claim-linked source, query recipes, probes and run receipts
```

Keep authoring sources separate from generated content so a rebuild cannot erase semantic work.
Give a claim one authoritative home; a task route or comparison links to it rather than restating
its entire contract. Generate direct stable links so an agent usually needs only a route and one
or two capability pages before reaching the implementation detail.

Suggested query interface, to implement only after the records and briefs are useful:

```text
find --task "gather rows preserving repeated indices"
find --input RecordBatch --output RecordBatch --property preserves-schema
show arrow.selection.take --view contract
compare arrow.selection.filter arrow.selection.take
show datafusion.scan.provider --view evidence
```

Return matched conditions, alternatives, evidence pointers, coverage limits, and a bounded number
of complete results. Prefer exact symbols and reviewed task aliases; diversify results across
re-exports and avoid flooding the answer with every method. Task aliases are authored synonyms,
not rules that silently exclude neighboring capabilities.

Start with JSON/TSV plus `rg` and a small standard-library helper. Optional SQLite FTS5 is reasonable
when prose search or structured joins measurably improve retrieval. The machine inventory should
also remain available without the database. DataFusion/Arrow themselves can analyze large evidence
tables offline if needed; requiring the full engine to read this skill would add unnecessary setup.

Optional Rust build tools include `rustdoc-types` with explicit adapters for the encountered format
versions, `cargo_metadata` for resolved profiles, and `ra_ap_*` only for focused semantic extraction.
The current corpus spans rustdoc formats 57, 59, 60, and 61; one newest parser is not automatically
compatible with all four. Exact tool selection/pins belong to implementation verification. No
global toolchain, GPU, or system migration is required by the proposed reference.

Embedding search is a later option only if evaluated task recall remains poor after task aliases,
semantic facets, and lexical retrieval. It cannot author or verify contracts. Likewise, a generic
knowledge platform, persistent compiler service, or whole-corpus MIR graph must justify its cost
against a concrete reference question before becoming part of this skill.

## 7. Portability and bounded upkeep

The shipped skill should be readable offline with no daemon, credentials, MCP connection, host
repository imports, or absolute workspace paths. Context7 can supplement discovery when available;
it is not a runtime prerequisite. Keep the target repository's actual dependency profile separate
from the reference's recorded profile. A different pin calls for checking the relevant claim,
not automatically upgrading the application.

Ship a reader bundle containing the reference, indexes, small examples, and evidence locators. Offer
an optional research bundle with source archives and probe dependencies. Avoid making readers copy
compiler targets or hidden build caches. Preserve enough selected evidence locally that important
claims remain inspectable when a network link disappears.

Default upkeep should rebuild affected projections and rerun affected probes. Broader qualification
belongs to a reference release or a materially changed claim. Report discovery coverage, reviewed
contract coverage, compilation coverage, behavioral coverage, and evaluation results independently;
none is a substitute for the others.
