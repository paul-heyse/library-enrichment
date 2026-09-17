# Design review: Plan 17 schema-contract decision

## 1. Decision and scope

**Decision: Accept the specified document-stage design in §§2–4, including the bounded-encoding
selection below.** This accepts an implementation contract, not completed implementation or product
acceptance. The decision is **Proposed**, with the named library interfaces **Interface-checked**.

**Target:** [Plan 17](../../plans/17-schema-governed-unified-runtime-hard-pivot.md), §3.2–§3.4 and
FP01–FP04. **Reviewer:** Codex schema-decision reviewer. **Date:** 2026-09-16.

The observable objective is one Rust-owned semantic declaration whose native admission, Arrow,
Delta and wire projections preserve meaning without independent field policies. Existing code has
typed Arrow contracts, but `arrow_model/checks.rs` still assembles vocabulary/variant consumers,
`store/preparation.rs:466–477` compares selected metadata, and
`store/admission.rs:261–285` separately writes scoped reference SQL. These are baseline observations,
not a claim that the concurrently changing tree still has every original review defect.

### Method and coverage

Read the design-review charter, directive, template and repository addendum; Plan 17's foundation
packages and related encoding requirements; the DataFusion, Delta Lake and FastMCP skills; the
existing [source/probe evidence](evidence/combined-schema-runtime-plan-2026-09-16/README.md); and the
principal pinned sources listed in §9. Independently inspected application consumers at the grain
cited above and the URL UDF at `crates/enrichment-core/src/native_url.rs:17–98`.

No Context7, builds, tests, new probes, dependency changes or product edits occurred in this review.
The existing probe log/receipt was read, not rerun. It tests selected Int64/Struct fixtures, not the
target fixed-binary contracts. This review does not certify current code, all Plan 17 packages,
concurrent publication, retention, contained effects, performance or installed MCP behavior. Those
remain implementation and qualification obligations. Frozen provenance and protected files were
not changed. The reviewer selected the semantically load-bearing foundation, not a random sample.

## 2. Authority and lifecycle map

| Concept | Authority and permitted change | Revision boundary | Derived consumers |
|---|---|---|---|
| Semantic fields, variants, domains, collections and references | One finite Rust declaration using native Arrow fields, Rust enums and DataFusion expressions | Deterministic semantic contract identity | Native predicates, semantic/read/write schemas, worker fields, wire schemas and encoding descriptors |
| Physical mapping | A declared projection of that contract; no independent nullable-schema policy | Mapping revision and selected Delta protocol/features | Write layout and read restoration |
| Producer observation | Actual producer values with scope and provenance; annotations alone establish no authority | Producer/build/input witness | Admitted native evidence and explicit gaps |
| Candidate versus admitted field | Admission decides trust; physical or extension metadata alone cannot promote a value | Exact table/cohort/contract witness | Trusted provider constraints and query fields |
| Wire representation | Generated field projection plus finite mechanical encoding rules | Codec revision | MCP structured object, resources, retained results and export |

Canonical identity separates domain, semantic interpretation, physical mapping, codec and actual
artifact/attempt witnesses. Harmless repartitioning does not change semantic identity; a changed
domain, collection rule or interpretation does. Cosmetic metadata is separately identified. The
hard pivot creates a fresh epoch and rejects old contracts; it promises no historical conversion.
That is a defined incompatibility policy, not silent reinterpretation (DM-51).

Specialized URL/version parsing, cryptographic framing and byte encoding remain ordinary bounded
Rust kernels behind declared contracts. The proposal adds no general schema DSL, expression IR or
policy interpreter. Python remains a mechanical transport/extraction boundary.

## 3. Semantic contracts and invariants

| Contract | Representation and enforcement boundary | Failure behavior | Oracle |
|---|---|---|---|
| Variant and presence meaning | Tag plus one nullable per-variant Struct; generated active-payload and parent-aware rules | Unknown tag, extra/inactive payload or missing active child refuses before mutation | SC01/SC03 |
| Domain compatibility | One Arrow extension descriptor per field, separate scoped-reference declaration; pre-coercion semantic rule and derived/output validation | Cross-domain comparison or undeclared domain erasure produces a typed diagnostic | SC04/SC05 |
| Scoped references | Source/target paths, domain, scope keys, guard, null policy and cardinality generate native anti/semi-joins | ID found only in a different producer/cohort/environment remains invalid | SC06 |
| Collection meaning | Declared sequence/set/map, order, duplicates, element nulls and actual bounds | Invalid duplicates/children refuse; no silent compaction or default filling | SC02/SC03/SC06 |
| Conditional requiredness | Sound physical NOT NULL, generated total-Boolean predicates and active native Delta CHECK feature; collection-child pre-admission where CHECK cannot express it | NULL parent may be valid; present parent with required NULL child is invalid; bypass routes refuse | SC03 |
| Exact values and fields | Timestamp micros UTC, fixed binary digest/domain IDs, coordinate unit/base, exact child names and admitted conversion | Wrong width/domain, overflow, missing renamed child or undeclared loss refuses before coercion | SC02/SC05 |
| Wire fidelity | Generated closed discriminated schemas and exact value codec | Full-range integers/decimals use declared strings unless numeric safety is proved; no lossy JSON numbers or invented defaults | SC07 |

NULL collection means unknown/unavailable, empty means known empty, and coverage records explain
the reason. Absent optional Struct, present Struct with unknown optional children, inactive variant,
NULL repeated item and missing required child remain distinguishable. Semantic equality is not
byte equality of stored files; exact wire encoding is a separately versioned contract.

## 4. Derivation and execution design

The declared path is semantic definition → bounded schema/metadata validation → generated native
candidate admission → complete Delta mutation → exact version/read restoration → semantic native
planning → generated bounded wire projection. Native NOT NULL/CHECK and native joins have distinct
responsibilities. The registry does not claim that storing a constraint installs or enforces it.

The Arrow extension trait checks datatype/metadata construction. DataFusion 55.1's extension registry
currently supplies formatting behavior. Consequently one finite AnalyzerRule checks the original
semantic expressions before coercion, and the same contract machinery verifies analyzed, optimized,
physical and output fields. A same-domain key/reference is valid; casting both sides to an untyped
primitive does not authorize an otherwise invalid operation.

The URL and PEP 440 UDFs use `return_field_from_args` for full fields and truthful nullability.
Their parsed component fields are not equivalent to the whole input argument, so they must not
claim `struct_field_mapping`. Other optimizer hooks require an actual proven relationship.

### Selected bounded-encoding implementation contract

This selection was made by the implementing agent during review; it requires no user clarification.
Arrow JSON's `Writer::write` encodes an entire row into a growable `Vec` before calling the output
writer. A capped downstream `Write` alone therefore cannot enforce the allocation bound.

Before row/value encoding, perform checked escaped-byte sizing or establish a proven conservative
bound, obtain the native/external allocation reservation, and use guarded incremental output. Use
`EncoderFactory` delegation only for rows whose complete allocation is bounded in advance. Where
that cannot be guaranteed, use a bounded mechanical per-field encoder with the same generated value
rules. Count UTF-8, escaping, separators, field names, nesting, null/omission, envelope and actual MCP
frame overhead. Feed size observations into the native page plan; encoding must not choose domain
policy or trim evidence. An impossible minimum cap produces the declared budget refusal.

FastMCP receives explicit `content=[]` or a separately bounded generated text block alongside
`structured_content`. Omitting `content` makes FastMCP derive it from structured content, creating
an additional text representation. The selected explicit-content path avoids that implicit copy.
The structured object and any intentional text/resources still count toward the actual frame cap.

## 5. Representative journeys

| Journey | Required behavior |
|---|---|
| Ordinary extension | Add a Locator variant/reference once; generated native checks, Delta rules, decoder dispatch, IPC and closed wire branches change together. A genuinely new parser kernel remains explicit code |
| Meaningful change | Change a field's domain or collection semantics; the contract change relation identifies the path, new witnesses invalidate cursors/read caches/replay, and the fresh epoch refuses old values |
| Boundary | Optional typed evidence crosses worker IPC → admission → Delta → provider → MCP with exact coordinates/IDs, parent presence and full-range numeric values preserved |
| Failure | Invalid nested child or wrong-scope reference is rejected before data publication; an unbounded row is refused before serialization allocation; neither failure becomes a successful published result |

These are proposed journeys with named tests, not executed end-to-end evidence.

## 6. Acceptance gates

Gate verdicts assess whether this specified proposal has one decidable meaning and an enforcement
route. They are distinct from the repository's runtime gate states and do not certify behavior.

| Gate | Verdict | Independent evidence and scope | Required implementation action |
|---|---|---|---|
| G1 — Authority | Pass | FP01 and §2 assign one owner; schemas/decoders/metadata and wire artifacts are derived; old competing owners are deletion obligations | Prove SC01 on a real extension and remove prior owners |
| G2 — Semantic fidelity | Pass | §3 distinguishes domains, presence, ordering, exact values and admitted field conversions; FP03 rejects metadata erasure | Execute SC02/SC04/SC05/SC07 across actual providers |
| G3 — Validity | Pass | Conditional guards, feature-enabled CHECK, scoped joins and candidate/admitted distinction identify refusal boundaries; unsupported bypass writes refuse | Execute SC03/SC06 on every admitted mutation route |
| G4 — Hidden behavior | Pass | Pure generation/validation and mechanical kernels have explicit inputs; metadata cannot authorize effects; codec does not select policy | Preserve operation witnesses and verify no encoding/admission side effects |
| G5 — Consistency and recovery | Pass | Within this scope, only fully admitted exact-version contracts become visible; codec failure cannot publish success; old epochs refuse | Integrate with FP10/FP11 publication/recovery before claiming runtime closure; these packages are not reviewed here |
| G6 — Transformation and reuse | Pass | FP02 records semantic/mapping/function/codec witnesses, permits only declared conversions and invalidates on meaningful changes | Execute SC08 against clean recomputation and actual cache/cursor consumers |
| G7 — Truthful capability claims | Pass | §9 confirms specific public/private boundaries; formatting registration, CHECK properties and encoder factories are not credited with automatic semantic enforcement or bounds | Keep capability qualification route-specific; no unexecuted route advertised as passed |

## 7. Principle findings

| Finding / verdict | Principle IDs | Concrete evidence or gap | Consequence | Correction / selected treatment | Verification |
|---|---|---|---|---|---|
| F1 — Bounded output needs a preallocation boundary; resolved in this proposal | DM-22, DM-37, DM-42, DM-45 | Arrow JSON 59.3 `writer/mod.rs:377–417` calls `encoder.encode(idx, &mut buffer)` before `write_all`; FP04's factory alone cannot cap that row allocation | A single huge escaped value can allocate beyond the declared cap even if the downstream writer later refuses | §4 selects preflight/reservation plus guarded output or bounded mechanical encoding | Test under Q11/SC07: one oversized scalar and nested repeated value must refuse before excess allocation; exact frame cap oracle. Not run |
| F2 — Explicit MCP content is required for the intended cost model; resolved in this proposal | DM-41, DM-42 | FastMCP 4.0.3 `tools/base.py:121–126` sets `content = structured_content` when content is omitted | Automatic text duplicates the payload and invalidates a frame estimate that counts structured data once | §4 explicitly supplies empty or separately bounded content | Real FastMCP contract test asserts no implicit text copy, plus SC07 actual frame measurement. Not run |
| F3 — Semantic registry is not an enforcement engine; proposed treatment satisfied | DM-06, DM-07, DM-24, DM-43 | Native extension/constraint/UDF interfaces in §9; existing planning probe shows CAST metadata erasure and cross-domain JOIN acceptance | Domain-separated IDs could otherwise join as interchangeable primitives | Keep FP03's pre-coercion rule, full-field UDFs and boundary validation, including valid key/reference domain pairs | SC04 negative/positive operator matrix and AST protection for project UDF field hooks. Not run |
| F4 — Storage constraints require complete native activation and parent scope; proposed treatment satisfied | DM-07, DM-08, DM-09, DM-53 | Delta validation inspects active CheckConstraints and its scalar path lacks optional-parent guards; complete ConstraintBuilder updates protocol | Metadata-only rules admit invalid values; blanket child NOT NULL rejects legal absent variants | Keep FP02/FP03's generated guards, actual add_constraint operation, collection-child admission and bypass refusal | SC03 invalid writes and absent-parent cases on each used route, with actual protocol check. Not run |
| F5 — Generation is justified only as a projection of finite meaning; proposed treatment satisfied | DM-02, DM-16, DM-52, DM-56, DM-58 | FP01 describes finite Rust declarations and rejects a second generic DSL; existing scoped reference SQL is a concrete repeated consumer | A new schema platform could merely move duplicate rules into another independently edited representation | Generate mechanics, preserve specialized kernels and distinct non-reference native predicates; require one real extension before terminal claims | SC01/SC06 and source deletion review under Q01/Q12. Not run |

**Applicability:** Groups 1–5 and 9–12 directly govern authority, semantic types, identity, native
lowering, interchange, generation and regression controls. Groups 6–7 apply at pre-admission,
publication handoff and dependency invalidation; full effect/recovery/concurrency protocols were
not inspected and receive no implementation assurance. Group 8 applies to bounded typed transfer
and honest cost claims; no performance benefit is measured or asserted here. No numerical score
would add evidence to this bounded decision.

## 8. Alternatives and architectural leverage

| Alternative | Semantic duplication / locality | Correctness and cost | Performance evidence | Decision |
|---|---|---|---|---|
| Current separate Arrow/Rust/Delta/wire consumers | Field and scope meaning can be repeated in each consumer | Smaller immediate edit, recurring drift and coordinated changes | No comparative measurement | Reject as final state under the hard pivot |
| Proposed finite declaration and native derivations | One field/variant/reference change generates mechanics; genuine algorithms stay explicit | Requires a bounded generator and complete semantic admission but removes independent policy copies | Hypothesis only; preflight adds measurable work | Select |
| Simpler viable alternative: Rust field table, explicit native checks, handwritten mechanical codecs guarded by exhaustive conformance tests | Shares the field table, but changes to presence/wire/validation still require several synchronized edits | Can preserve correctness at small scope with strong independent tests; cheaper initial generator work | No comparative measurement | Viable for a smaller service; reject here because current worker, Delta, identity and nine-tool consumers already repeat the same stable schema semantics |

The extra layer is justified by those existing consumers, not hypothetical plugins. An independent
generic IR, runtime schema scripting language or automatic compatibility service has no role.

## 9. Verification and primary-source evidence

All sources below were inspected locally on **2026-09-16** at the exact version/commit shown; URLs
identify the corresponding primary source. No moving HEAD or Context7 result is used. Short quotes
are evidence of interface semantics, not proof of the service's implementation.

| Claim / canonical API | Primary source and short quotation | Observed consequence |
|---|---|---|
| `datafusion_common::types::extension::DFExtensionType` | [55.1.0 source](https://docs.rs/datafusion-common/55.1.0/src/datafusion_common/types/extension.rs.html): “Pretty-printing values in record batches” | Current custom operation is formatting; application domain enforcement is separate |
| `arrow_schema::extension::ExtensionType::validate` | [59.3.0 source](https://docs.rs/arrow-schema/59.3.0/src/arrow_schema/extension/mod.rs.html): “Validate this extension type for a field with the given data type and metadata.” | Signature receives no array values; native value admission is required |
| `datafusion_common::functional_dependencies::Constraint` | [55.1.0 source](https://docs.rs/datafusion-common/55.1.0/src/datafusion_common/functional_dependencies.rs.html): `PrimaryKey(Vec<usize>)`, `Unique(Vec<usize>)` | No foreign-key variant; use declared native reference joins |
| `datafusion_expr::udf::ScalarUDFImpl::return_field_from_args` and `StructFieldMapping` | [55.1.0 source](https://docs.rs/datafusion-expr/55.1.0/src/datafusion_expr/udf.rs.html): “specifying nullability”; “equivalent to sorting by the source column” | Full fields and literal arguments are available; parsed substrings do not justify argument-equivalence metadata |
| Cast field behavior | [55.1.0 expression-schema source](https://docs.rs/datafusion-expr/55.1.0/src/datafusion_expr/expr_schema.rs.html): “Cast to a type without extension metadata strips extension metadata” | Check original domains before coercion and validate output contracts |
| Delta internal `validation_predicates` and requiredness helpers | [delta-rs pinned source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/data_validation.rs): `is_feature_enabled(&TableFeature::CheckConstraints)`; “paths that reach past a list/map element” | Active feature gates CHECK; scalar child predicates lack parent guards and repeated child coverage has limits |
| `deltalake_core::operations::constraints::ConstraintBuilder` | [delta-rs pinned source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/constraints.rs): “Put all the constraint into one commit” | Complete operation validates existing data and commits metadata/protocol actions; raw metadata alone is insufficient |
| `arrow_json::writer::encoder::EncoderFactory` | [59.3.0 source](https://docs.rs/arrow-json/59.3.0/src/arrow_json/writer/encoder.rs.html): “overrides the default encoder for a specific field and array” | Finite field-aware representation dispatch and default delegation are available; this trait does not promise a bounded allocation |
| `arrow_json::writer::Writer::write` | [59.3.0 source](https://docs.rs/arrow-json/59.3.0/src/arrow_json/writer/mod.rs.html): `encoder.encode(idx, &mut buffer)` | Whole-row growable buffer precedes writes; §4's preflight/reservation is required |
| `fastmcp.tools.base.ToolResult` / `Tool.output_schema` | [FastMCP v4.0.3 source](https://github.com/PrefectHQ/fastmcp/blob/v4.0.3/src/fastmcp/tools/base.py): “structured_content must be a dict or None”; `content = structured_content` | Importable `fastmcp.tools.ToolResult` accepts the structured object; explicit content avoids automatic text derivation |

Local source roots: Cargo registry `index.crates.io-1949cf8c6b5b557f`, Delta checkout
`delta-rs-dcb716bfdc369320/58f07cd`, installed FastMCP
`.venv/lib/python3.14/site-packages/fastmcp/tools/base.py`. Exact artifact hashes for the earlier
probe are in its [receipt](evidence/combined-schema-runtime-plan-2026-09-16/receipt.json).
The paired kernel remains `8ba063f8f84fec222000f66d40d70911d7c79675`; this review makes no new
kernel capability claim.

SC01–SC08 specify the necessary field-addition, negative, metamorphic, round-trip and invalidation
oracles. Run them on the actual final field contracts through the shared session and Delta routes;
record precision/NULL/domain adversarial cases independently from the generator. Measure preflight,
encoding allocation, frame size and actual transport as part of Q11/SC07. Existing probe receipts
do not settle these product obligations. No acceptance gate status is updated by this review.

## 10. Exceptions and unresolved decisions

No in-scope MUST gap or undecided semantic choice remains in the **specified proposal**. F1/F2's
mechanical selection is explicit in §4 and must be recorded in the implementing ADR. The inability
to use a library encoder on an unbounded row is a capability limit, not an exception permitting
unbounded output. Native pre-admission where Delta CHECK cannot express an invariant is a declared
enforcement route, not a weaker invariant. No SHOULD exception is requested.

Implementation correctness, complete operation coverage and final qualification remain open work.
If an actual required operator, collection shape or write route cannot preserve the declared
semantics, it must refuse until a qualified native or bounded kernel route exists; report that
capability gap without broadening this document's acceptance.

## 11. Decision and implementation changes

**Accept this proposed decision for implementation.** The native interfaces support the selected
routes, the metadata and storage limitations have explicit enforcement boundaries, and the two
encoding mechanisms are now specified. This review does not accept unimplemented runtime behavior.

| Priority | Change | Principle IDs | Acceptance evidence | Regression protection |
|---|---|---|---|---|
| Correctness first | Record §4's encoding choice with semantic ownership, conditional requiredness, analyzer and fresh-epoch decisions | DM-02, DM-07, DM-24, DM-42 | ADR/design update; actual SC03/SC04/SC07 | Field/domain/route tests and encoded-allocation/frame tests |
| Semantic leverage | Land one real contract path through generated union/reference, Delta/read, identity and wire; delete its old owners | DM-16, DM-52, DM-56 | SC01/SC02/SC05/SC06 | Positive extension oracle plus scoped source deletion checks |
| Complete invalidation | Bind semantic/mapping/function/codec witnesses to actual consumers | DM-31, DM-32, DM-51 | SC08 and later FP11 integration | Meaningful-change tests against clean recomputation |
| Cost after correctness | Measure the chosen preflight/native encoding path and full transport | DM-39 | Q11/SC07 actual resource and frame evidence | Representative oversized/nested result regression |

The supported claim is a decidable, interface-checked architecture proposal. Product completion
still requires Plan 17's implementation, deletion and terminal acceptance evidence.
