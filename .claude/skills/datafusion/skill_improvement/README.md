# Improving the DataFusion skill as a programming reference

Implementation is now documented in [IMPLEMENTATION_REPORT.md](IMPLEMENTATION_REPORT.md).
This assessment remains the historical basis for the changes.

Assessed 2026-09-18. Scope: the portable DataFusion/Arrow skill, independently of its host
repository. This is an assessment and target design, with executed evidence and worked reference
examples. It does not replace the current skill or implement the proposed reference generator.

## Recommendation

Keep the existing API inventory and build machinery. Add a **task-to-capability decision layer**
and an **operation contract layer**, and repair the documentation that the generator currently
discards. The most valuable unit of reference should be a capability with its conditions,
alternatives, input/output semantics, and composition requirements—not a type with a method count.

As the intended reader, I want to answer these questions before designing code:

1. Which existing operation already performs the transformation I need, at the abstraction level
   where my data currently lives?
2. What other built-ins might be a better fit if ordering, nulls, representation, scale, or
   lifecycle requirements differ?
3. What exactly enters and leaves the operation, including schema, cardinality, ownership,
   errors, allocation, and execution phase?
4. Which responsibilities remain mine? What does the engine infer, optimize, validate, or enforce?
5. What must be configured or registered, and what would a small discriminating example prove?

The solution should make those answers cheap to retrieve while retaining the full underlying
reference for deeper investigation. It should preserve an intelligent agent's ability to choose;
conditional comparisons are more useful than blanket instructions to use a particular API.

## Read the proposal

| Document | Purpose |
|---|---|
| [Target design](TARGET_DESIGN.md) | Organization, capability/operation contracts, retrieval, extraction, tooling, and portability |
| [Worked capability references](CAPABILITY_EXAMPLES.md) | Seven examples of the depth and decision support the new layer should provide |
| [Implementation and evaluation](IMPLEMENTATION_PLAN.md) | Dependency-ordered work, concrete deliverables, and realistic decision-quality evaluation |
| [Proposed entry point](examples/SKILL.proposed.md) | A compact replacement entry point, illustrated using this assessment bundle |
| [Crate routes](examples/crate-roles.tsv) | Task-oriented roles for all 60 packages in the existing inventory |
| [Example capability record](examples/capability-record.json) | Concrete machine-readable separation of operations, claims, conditions, evidence, and choices |
| [Evidence guide](evidence/README.md) | Retained source, commands, query fixtures, probes, results, and reproduction |

## What is already strong

The live inventory contains 60 crates, 7,134 canonical items, 21,047 method rows, 6,406 alias
rows, and 2,335 implementation edges. It includes 15 topic pages, 30 trait pages, 79 example
files, and 59 guide files. Canonical definitions and re-export paths are already distinguished.
The builder is self-contained, the indexes are inexpensive to search, and its topic seeds must
resolve rather than silently disappearing. These are substantial assets, not something to replace.

The existing verification command passed: recorded digests matched 2,822 files, 13 rule groups
passed, and all 35 navigation probes found their expected text. This run used `--skip-rebuild`:
it verified recorded content integrity, **not a fresh regeneration**. Those checks establish
internal consistency and known-term navigation; they do not establish good capability choices.
See [inventory](evidence/inventory.json) and [verification log](evidence/logs/existing-skill-check.log).

## Findings that materially affect reference value

| Priority | Observed gap | Consequence and proposed change |
|---|---|---|
| 1 | Full method documentation is lost. `Method` stores a signature and short summary; `_method_block` renders signatures. | `scan`'s 4,254-character rustdoc contract and `execute_stream`'s lifecycle documentation are available in the input but absent as method documentation in the API pages. Preserve complete member documentation and link directly to individual methods. |
| 1 | Module documentation is not emitted as module documentation; modules are excluded from `ITEM_KINDS`. | Crate/module overviews, intended use, and examples can disappear. Preserve module prose, typed fields, variant documentation, and intra-doc links as first-class reference material. |
| 1 | The Arrow topic has only `DFSchema` and `ScalarValue` as entry points. | Existing `RowConverter`, `FilterBuilder`, selection, sorting, casting, and `MutableArrayData` symbols are not discoverable through those topic routes. Split Arrow into task-oriented families. |
| 1 | Several curated recommendations overstate guarantees. | Repair these at their authoring source, then add evidence-linked conditions and counterexamples. Concrete cases are below. |
| 2 | The crate map leads with item/trait counts and notable extension traits. | It says little about which crate solves an implementation task. `arrow-row`'s nine items can be more relevant than a much larger crate. Add roles, input/output forms, and nearest alternatives. |
| 2 | The model retains reconstructed signatures, not structured parameter/result type trees or behavioral contracts. | An agent cannot reliably find “accepts arrays, returns row indices,” inspect nested `Result`/stream errors, or distinguish a planning API from execution. Retain type trees and author semantic contracts separately. |
| 2 | Topics emphasize entry points, extension points, settings, and examples; comparisons are thin. | Agents can find `TableProvider` or `ScalarUDFImpl` without first considering providers/functions already supplied by the libraries. Add built-in candidate sets and explain when customization is necessary. |
| 2 | Configuration-to-builder joins use matching names (`with_` plus the key suffix). | A plausible name is not proof of equivalent scope, units, defaults, or effects. Mark these joins as candidates until source or a probe confirms them. |
| 2 | The navigation checks mostly match a known regex in a predetermined file. | They do not test unfamiliar task wording, alternatives, semantic suitability, or executable integration. Add realistic task pairs and implementation checks. |
| 3 | Project rules match syntax without receiver resolution. | A non-DataFusion telemetry client's awaited `collect()` triggers the collection hint. Keep such output as a candidate, or resolve receiver identity before making a DataFusion-specific recommendation. |

Evidence: [generator model](../build/model.py), [renderer](../build/emit.py),
[topic generator](../build/topics.py), [configuration join](../build/catalogs.py),
[raw documentation audit](evidence/documentation-audit.json),
[retained rustdoc fragments](evidence/rustdoc-fragments.json),
[Arrow index matches](evidence/logs/arrow-discovery.log), and
[structural query results](evidence/query-runs.json).

The documentation finding is especially consequential: adding more prose manually before fixing
that loss would repeatedly reconstruct information already supplied by upstream.

## Statements to correct first

| Current guidance | Better characterization |
|---|---|
| Streaming “bounds memory.” | Streaming changes output consumption. Sorts, joins, aggregates, providers, consumers, and retained buffers still determine memory use. The runtime builder itself documents limits that are not respected in every case. |
| Default `RuntimeEnv` has “no spill path.” | In the retained 55.1.0 source the memory pool defaults to unbounded, while `DiskManagerBuilder` defaults to `OsTmpDirectory`. Spill location, memory-pressure signaling, and operator spill support are separate decisions. |
| `Inexact` pushdown is almost always better and an approximate answer is safe. | It is useful only for sound candidate reduction: no qualifying row may be discarded. Residual filtering removes false positives; it cannot recover false negatives. Costs and limit interaction also matter. |
| Implementing only three required provider methods is “correct and slow,” with no limit pushdown. | Method count proves neither correctness nor speed. The required `scan` already receives a limit. Correctness depends on implementing its contract; performance depends on the source and workload. |
| Expression helpers apply coercion rules. | Do not attribute analysis to every constructor. The retained `ScalarUDF::call` builds an expression; `ExprSimplifier::simplify` documents compatible operand types as a precondition and provides a separate coercion operation. Verify each helper's actual stage. |
| A syntax match identifies a DataFusion opportunity. | It identifies a source shape. Receiver/type resolution or explicit context is needed before attributing that shape to DataFusion. |

These findings are supported by [the worked references](CAPABILITY_EXAMPLES.md) and retained
sources, not by a claim that the whole existing skill is wrong. The current corpus contains much
useful material; the main problem is which information is promoted into its navigational layer.

## What I would build first

First repair method/module documentation retention and the specific overstatements. Then deliver
the task router, crate roles, and a small set of deeply characterized capabilities spanning both
DataFusion and Arrow. Expand from a broad inventory into reviewed semantic depth, prioritizing
operations with many plausible alternatives or consequential contracts. Measure improvement on
paired tasks where a changed requirement should change the recommendation.

The initial implementation does not need a new service, embedding model, database, or whole-crate
compiler analysis. Those are optional mechanisms with specific triggers in the target design.
The scarce resource is accurate semantic characterization and useful comparisons.

## Evidence and completion boundary

This assessment produced a retained inventory, selected source files from 12 exact-release
packages, raw rustdoc fragments, reproducible lexical/structural queries, and seven passing Arrow
59.3.0 contract tests. One additional structural rule group passed with three matching and two
non-matching fixtures. Its source scan found nine declarations; a separate control demonstrated
one false DataFusion attribution by the existing `collect` hint.

DataFusion behavior described here is supported by retained source/documentation; no DataFusion
runtime experiment or comparative agent benchmark was run. The reference redesign, complete
contract catalog, richer extraction model, and measured agent-decision gains remain proposed.
The existing skill entry point, generator, generated content, and sibling skills were preserved.
