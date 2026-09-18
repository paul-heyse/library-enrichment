# Typed comparison values and native field differences

## 1. Decision and scope

**Proposal:** implement Plan 19 CP06 comparison values using one closed native declaration.
**Reviewer:** Codex, 2026-09-17. **Revision:** wire 7.0, state 14, snapshot 11.0, executor 7.
Representation/projection and field explanations are **Tested** in isolated Arrow fixtures. Installed behavior is **not_run**.

The prior generic inline JSON value is replaced by `ComparisonValue`; the declared API payload
supplies callable fields. Native comparisons exclude source clocks, preserve unknown observations,
and produce typed values at the boundary. This is a breaking hard pivot, with no legacy decoder.

**Method and coverage:** read the native declarations, comparison axis plans, preparation contract,
record constructor, final sink, schema outputs and exact DataFusion 55.1.0/Arrow 59.3.0 sources.
Read the independent upstream verifier's local source pointers and verified the pertinent native
set/hash/field behavior directly. Inspected pure Arrow execution receipts. No Delta publication,
artifact storage, restart, real producer, MCP or installed journey was run because CP11 is open.
Full CP06, cache lifecycle and Plan 19 completion are outside this scoped representation decision.

## 2. Authority and lifecycle map

| Concept | Owner | Boundary | Derived forms |
|---|---|---|---|
| Value shape | `core/compare.rs::ComparisonValue` | Native declaration / wire 7.0 | Arrow fields, Rust enum, schemas and Python DTOs |
| Callable facts | `evidence/relational.rs::ApiPayload` and `evidence/declarations.rs` | Exact admitted snapshot | API comparison payload |
| Difference | `store/comparison.rs` and `comparison/fields.rs` | Exact before/after values | Native set differences and typed field paths |
| Provenance | Qualified `FactSource` | Snapshot/environment/producer/artifact | Separate alternative source |
| Delivery | Native prefix selection; bounded sink | Selected exact bytes | Inline union or immutable JSON artifact |

Opaque code is limited to generated decoding, bounded encoding/I/O and a schema-derived Arrow
field-fact UDF. The UDF emits typed paths and exact canonical value bytes; it makes no equality
decisions. Native set operators compare those facts.

## 3. Semantic contracts and invariants

API documentation is null on that axis and compared through documentation scope. An absent API
observation remains different from an absent binding. Python header duplicates and callable
parameter sequence remain meaningful. Parent field differences preserve correlations between
alternatives; independently equal leaf sets are not a proof of equal complete values.

The preparation family requires complete generated value/source fields. `native_record` checks
full arguments; native unions check discriminator/payload structure. Native set operators supply
equality, not completeness or compatibility verdicts. Observed changes retain coverage warnings.

## 4. Architecture and transformations

Admitted native columns → metadata-preserving record expressions → DISTINCT set reconciliation →
selected keys and native field differences → native ordered alternative prefix → typed inline
codec or bounded streaming artifact codec. Generic JSON is absent from semantic equality and inline
delivery. Final JSON remains a protocol representation.

## 5. Change and adversarial journeys

- New variant: add one declaration, its axis projection and focused oracle; regenerate transport.
- Changed callable: native nested fields and ordered parameters participate in equality. Field
  paths identify declared properties and sequence ordinals; before/after alternatives retain values.
- Missing observation: retain null observation and qualified binding instead of inventing defaults.
- Output capacity: native selection refuses an unfit first alternative; the sink verifies bytes.
- Old state/client: source epoch rejects obsolete contracts. Physical deletion and fresh activation
  still belong to CP11/CP13; no claim of installed compatibility is made.

## 6. Acceptance gates

| Gate | Verdict | Evidence and scope | Required action |
|---|---|---|---|
| G1 Authority | Pass | One value declaration; original callable owner reused | Keep schema generation authoritative |
| G2 Semantic fidelity | Pass | Pure Arrow null/order/metadata/set/codec fixture; independent documentation axis | CP12 actual producer and complete comparison journey |
| G3 Validity | Pass within representation scope | Complete preparation fields and generated union decoder; incompatible fields fail construction | Complete all-family negative matrix in CP02/CP12 |
| G4 Hidden behavior | Pass | Projection, equality and prefix policy are native; sink only encodes selected values | No new effect authority in serializers |
| G5 Consistency and recovery | Not applicable to scoped representation acceptance | Exact immutable input vectors supplied by existing comparison owner; durable recovery not qualified here | CP07/CP08/CP12 remain open |
| G6 Transformation and reuse | Pass within representation scope | Struct/List set operators; no JSON equality or inline round trip; full field check | Qualify full workload cost at CP12 |
| G7 Truthful capability claims | Pass | No source-compatibility inference, no timing/installed guarantee; upstream limits explicit | Keep qualification gaps visible |

## 7. Principle findings

| Finding | IDs / verdict | Evidence | Consequence | Correction | Oracle |
|---|---|---|---|---|---|
| F1 Closed values reuse the original callable owner | DM-02/06/23/52 Satisfied | `compare.rs::ComparisonValue`, `ApiComparisonObservation` | Wire cannot independently invent API payload shape | Land regenerated current artifacts | `typed_comparison_`; schema-conformance |
| F2 Missing observation and ordered fields retain meaning | DM-08/15/53 Satisfied within tested scope | `comparison/typed_tests.rs` | Nulls and sequence changes affect equality as declared | Keep before/after source distinct | Pure null/order/set/codec fixture |
| F3 Built-in struct construction loses immediate metadata | DM-07/23 Satisfied by application wrapper | Pinned `named_struct.rs:131–148`; `native_record.rs` | Native schema compatibility alone is insufficient | Direct declared record projections replace raw named_struct | Full child-field assertions and native preparation |
| F4 Durable and installed cutover is unqualified | DM-51/54 Unresolved for whole service | CP11/CP12 not run | Cannot claim end-to-end pivot completion | Complete removal and final journeys in their required order | CP11 receipt and CP12 comparison/client qualification |

**Applicability:** authority, semantic types, equality, transformations, boundaries, generated
contracts and verification bear directly on this slice (groups 1–3, 5, 9–11). Preparation/effects,
retention and evidence groups apply only to the stated sink/provenance boundaries; their complete
lifecycle guarantees are not assessed. Graphs, numerical model behavior and generic extension
platforms are not introduced and do not warrant additional machinery.

## 8. Alternatives and architectural leverage

| Alternative | Semantics / extension cost | Risks | Performance evidence | Decision |
|---|---|---|---|---|
| Generic JSON | Separate untyped boundary | Shape and metadata drift | No comparative measurement | Remove |
| Arrow UnionArray | Compact native union | Not hashable in pinned DataFusion set path | Not measured | Reject at this pin |
| Generated tagged Struct | Shared finite model and codec | Full field validation still required | Focused execution only | Select |
| Duplicate comparison payloads | Initially smaller fields | Repeated callable authority | Not measured | Reject |

The existing declaration generator and native record wrapper suffice. No new generic diff engine
or schema language is needed. The final field relation uses normal DataFusion set operators.
Per-field joins and a wide nested expression plan both exceeded the 30-second focused fixture
deadline; the bounded native lowering UDF passed the fixture in 1.55 seconds together with the
all-axis case. This is fixture evidence only, not a whole-service performance claim.

## 9. Verification and measurement plan

| Claim | Evidence | Oracle / conditions | Result |
|---|---|---|---|
| All eight axes use the same value contract | Tested | `typed_comparison_all_axes_use_one_closed_value_contract`, pure Arrow | Passed in `typed-comparison-unit.log` |
| Null/order/metadata/native set/final codec fidelity | Tested | `typed_comparison_preserves_absence_order_metadata_and_set_equality`, pure Arrow | Passed including field paths in `typed-comparison-fields-unit.log` |
| Generated wire is current | Tested | `typed-comparison-schemas.log` | Four valid / eight negative fixtures; current field-path schema receipt is `typed-research-schemas.log` |
| Field explanation includes meaningful differences | Tested | `typed-comparison-fields-unit.log` | Two pure fixtures passed, including declared parameter ordinal and complete-value paths (1.55 s) |
| Durability and installed behavior | Proposed qualification | Plan 19 CP12 | not_run |

No performance advantage is asserted. Field plan construction, alternative fanout and output
cost require workload measurements after the deletion barrier; hard result bounds still apply.

## 10. Exceptions and unresolved decisions

No charter exception requested. The user explicitly authorizes incompatible design-phase cutover.
Deletion and activation are mandatory, not implied by epoch changes. New variants or upstream
set/field changes trigger ADR-0056 register R-53 review. Whole-service qualification remains open.

## 11. Decision and implementation changes

**Decision: Accept-scoped** — closed representation and its declared native projection/encoding
contract. This does not accept durable field-explanation qualification, full CP06 or Plan 19 completion.
ADR-0056 remains proposed while the implemented scope and final verification are being completed.

| Priority | Change | Principles | Acceptance evidence | Regression protection |
|---|---|---|---|---|
| 1 | Remove generic comparison JSON authority | DM-02/06/23 | Native values plus generated transport | Preparation and pure typed fixture |
| 2 | Complete typed field explanation and negative cases | DM-15/53/54 | Exact pure Arrow differences | Field-path fixture |
| 3 | Retire installations/state, then qualify actual journeys | DM-51/54 | CP11/CP12 receipts | Matching candidate tests |
