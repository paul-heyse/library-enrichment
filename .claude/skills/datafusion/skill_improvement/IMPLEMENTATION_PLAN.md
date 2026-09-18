# Implementation and evaluation plan

Implementation completed 2026-09-18 for the seven stages below; see
[IMPLEMENTATION_REPORT.md](IMPLEMENTATION_REPORT.md) for artifacts, executed checks, pilot results
and explicit scope limits. The original proposal and initial assessment state remain below as provenance.

Proposed 2026-09-18. Changes belong inside the portable DataFusion skill. This plan does not modify
or qualify the host application. [The assessment](README.md) and [target design](TARGET_DESIGN.md)
define the intended result; [the worked examples](CAPABILITY_EXAMPLES.md) demonstrate its depth.

## Build in this dependency order

| Stage | Concrete work | Reviewable completion criterion |
|---|---|---|
| 1. Recover upstream contracts | Extend `build/model.py` and `build/emit.py` for module/member/field/variant docs, original type trees and source/link locators. Correct the documented overstatements in `build/topics.json` and authored query guidance. Regenerate rather than edit generated pages. | An agent can retrieve `TableProvider::scan`, `DataFrame::execute_stream`, `CastOptions::safe`, and module overviews without losing documentation present in rustdoc. Preservation tests compare source records to rendered/member records. |
| 2. Introduce useful routes | Add task vocabulary, semantic facets and a crate-role source; render task/crate/representation views. Include all 60 current packages, separating SQL syntax, logical planning, execution, Arrow kernels, storage and interchange. | Unfamiliar task phrases find credible built-in candidate sets; canonical and facade paths resolve; the Arrow routes lead to relevant Arrow crates. Existing direct lookup still works. |
| 3. Establish contract quality | Turn the seven examples into maintained capability records. Add built-in-function selection, joins/set operations, schema adaptation/metadata, and aggregate/window state. Each has alternatives, decisive conditions, full inputs/outputs, and explicit unknowns. | Reviewers can derive an implementation strategy from a brief without guessing consequential preconditions. Claims have source/probe links; unsupported generalizations and unexplained defaults fail review. |
| 4. Automate evidence gathering | Add source/doc/query adapters, registry captures and focused compile/runtime probes. Preserve raw evidence, configuration, controls and command logs. Use semantic resolution only where syntax cannot answer the question. | An evidence bundle can be reproduced in an isolated capsule, and distinguishes source findings, executable results and untested interpretations. No implicit full-codebase analysis is needed for ordinary reference use. |
| 5. Add bounded retrieval | Implement `find`, `show`, and `compare` over the reviewed records and typed operation index. Start with exact/alias/facet/lexical selection. Add an optional FTS index only if it improves actual task results. | Results expose why candidates matched, decisive differences, evidence pointers, and remaining unknowns within a bounded response. Raw files remain independently usable. |
| 6. Evaluate and expand | Run the paired task suite below against the current and proposed references. Expand semantic coverage based on missed decisions, not just unindexed symbols. | Better choices and correct integration are demonstrated; unresolved failures remain visible. No claim of improvement rests only on corpus size, regex matches, or page readability. |
| 7. Package and maintain | Produce portable reader/research bundles, source manifests, license notices, targeted invalidation, and an offline verification command. | A copied bundle supports the same lookups from an unrelated directory without host-project imports, services, absolute paths, or hidden cache dependencies. |

Stages 1–3 should produce useful reference improvements before building a larger extraction or
search system. Do not make a database, compiler framework, or generic ontology a prerequisite for
writing an accurate capability comparison. Preserve the existing working inventory throughout.

## Prioritize depth by decision value

The first semantic backlog should cover the seven examples plus:

1. Built-in scalar/nested/table functions versus Arrow kernels and custom UDFs: names, aliases,
   expression helpers, registration, coercion, return-field behavior and volatility.
2. Schema alignment and propagation: `Schema`/`DFSchema`, nested fields, extension metadata,
   physical expression adapters, batch construction versus domain validation.
3. Relational cardinality and ordering: joins, null keys, `DISTINCT`, bag/set variants, windows,
   unnesting, sorting, limit/top-k, and partition boundaries.
4. Aggregate/window implementation support: existing functions, accumulator helpers, partial/merge
   state, groups accumulators, retract requirements, and ordering/frame assumptions.
5. Repeated execution and storage: provider reuse, plan/view/cache differences, reader metadata,
   object-store mappings, writer/sink behavior, and serialization/registry dependencies.

Then use task failures and real unanswered questions to choose the next capabilities. Maintain
full-scope discovery, but do not label every indexed item as deeply characterized.

## Evaluate choices, not just retrieval

Use realistic, repo-agnostic Rust consumer fixtures and short design tasks. Hold the task,
dependency profile, tool access, and response budget constant between the existing reference and
the candidate. Record the retrieved pages, candidate set, decision, rationale, assumptions,
implementation, tool calls, elapsed time and context consumption. Keep evaluation answers out of
the material the evaluated agent sees.

Use paired tasks: change one material condition and see whether the recommendation changes when
it should. Multiple solutions can be correct; the oracle is the requirement and executable behavior,
not exact prose or one hand-selected symbol. Treat honest “unverified” answers as distinct from
confident errors. Use independent judging and repeated runs where nondeterminism matters.

The following twelve pairs provide 24 initial task cases. These evaluations are **not_run** in
this assessment; the seven small Arrow tests validate only their own assertions.

| Pair | Case A | Changed condition in case B | Expected decision distinction and oracle |
|---|---|---|---|
| E01 | Keep rows selected by a Boolean mask | Gather ordered indices with duplicates | Filter versus take; exact values, order and multiplicities |
| E02 | Same mask repeatedly applied to several columns/batches | One simple primitive array once | Consider reusable `FilterPredicate` versus direct filter; equal output, measured setup/reuse cost rather than blanket optimization |
| E03 | Gather from an ordinary batch | Batch has zero columns but nonzero row count | Account for row-count construction; executable zero-column fixture |
| E04 | Cast malformed numeric strings and preserve processing | Invalid conversion must reject the operation | Explicit null-on-failure versus error; assert malformed, valid and input-null cases |
| E05 | Compare local composite keys | Keys must be stable across releases and processes | Row comparison representation versus defined serialization contract; no unsupported durability claim |
| E06 | Row-encode dictionary values for comparisons | Decode must preserve dictionary physical types | Explain hydration and choose/compose accordingly; assert output types as well as values |
| E07 | Consume a large query result incrementally | Query includes a large blocking sort under a memory budget | Streaming versus whole-query resource planning; inspect operator behavior/metrics, no RSS guarantee from stream type |
| E08 | Expose already materialized batches | Expose partitioned Parquet files with pruning | Compare `MemTable` and file/listing/provider components before custom traits; execute source-to-result fixture |
| E09 | Provider exactly enforces a predicate | Provider only prunes conservative candidate groups | Exact versus inexact plus residual filtering; compare full results and inspect plans |
| E10 | Provider returns all columns | Output omits a filter column and has a small limit | Preserve filter → limit → projection contract; null/duplicate/control fixtures |
| E11 | Expression arrives through ordinary query analysis | Caller constructs an expression directly with incompatible operand types | Distinguish constructor, coercion, simplification and evaluation; compile/runtime success and expected errors |
| E12 | Parquet row selection over all row groups | Some row groups were excluded first | Rebase selection coordinates to retained groups; exact row identity plus I/O/decode observations |

Extend this suite with held-out tasks about built-in SQL/function discovery, metadata survival,
aggregate state, object storage, configuration lifetime, FFI ownership and plan serialization.
The initial pairs intentionally emphasize contracts already examined, so they are a pilot—not
a representative evaluation of the entire engine.

## Keep evaluation dimensions separate

| Dimension | Measurement |
|---|---|
| Candidate discovery | Relevant built-ins found, relevant alternatives missed, duplicate/irrelevant results |
| Selection quality | Requirements satisfied; decisive conditions and rejected alternatives explained |
| Contract fidelity | Correct shapes, null/order/schema/error/lifecycle details; unsupported claims listed |
| Integration correctness | Compiles in the declared profile; positive and negative behavior assertions pass |
| Evidence use | Claims trace to appropriate sources/probes; uncertainty and contradictions handled accurately |
| Efficiency | Pages/tool calls/context/elapsed time to a defensible choice; unnecessary infrastructure invoked |
| Generalization | Different wording and changed constraints, plus held-out tasks outside the worked examples |

Do not collapse these into a synthetic quality percentage. A fast answer with a wrong exactness
claim is worse than a slower correct one. A compile-only result is not runtime validation, and a
successful kernel example is not evidence of bounded query memory.

## Reference qualification checks

Run checks appropriate to the actual changes:

- Preserve raw-to-normalized method/module docs, type structure, source locations and link targets.
- Verify stable identity and re-export resolution without treating raw rustdoc IDs as global IDs.
- Assert complete declared task/crate routing coverage and disclose uncharacterized capabilities.
- Validate generated-file integrity and deterministic regeneration separately.
- Test structural queries with formatting variations, near misses and unrelated same-name calls.
- Compile small compositions; run semantic tests with controls for nulls, duplicates, empty shapes,
  errors and configuration. Use property/differential tests where they provide an independent oracle.
- Run targeted performance/resource experiments only for claims that require them; record workload
  and measurement scope instead of generalizing a microbenchmark.
- Test the copied bundle offline from an unrelated directory, with hidden caches unavailable.
- Run the comparative decision tasks before reporting measured improvement.

Existing useful checks should remain. The new tests address properties the current 35 navigation
regexes cannot prove; they should complement rather than replace integrity and lookup verification.

## Operating costs and optional tools

Standard-library Python and the existing `rg`/`ast-grep` tools are sufficient for the initial
authoring/build path. Existing Cargo is sufficient for isolated behavioral examples. Store probe
source, lockfiles, feature/target/toolchain context and logs; keep disposable compiler targets out
of the reader bundle. Python 3.14's standard-library zstd is available on this workstation; an
explicit zstd alternative can support older Python, as the current builder already does.

Add `cargo_metadata`, rustdoc format adapters, SQLite FTS5, rust-analyzer/HIR or benchmarking tools
only for a concrete implementation need, after checking their current primary documentation and
compatibility. Avoid prescribing their versions as part of this design. No host environmental
change is necessary to adopt the core reference improvements.

## What was delivered at assessment time

The assessment, target design, seven worked references, 60-crate route seed, proposed entry point,
and permanent evidence/probe bundle are present. The seven Arrow tests and query fixtures ran.
The extraction redesign, full capability records, richer retrieval commands, DataFusion runtime
probes, clean-install portability qualification, and comparative agent evaluation remain proposed.
