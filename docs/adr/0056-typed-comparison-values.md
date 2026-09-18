---
id: ADR-0056
title: Keep comparison alternatives and callable facts in declared native variants
status: proposed
date: 2026-09-17
deciders: [paul-heyse]
level: decision
principles: [DM-02, DM-06, DM-08, DM-15, DM-23, DM-51, DM-52, DM-53]
design: [§6.3, §7.2]
review: docs/design_review/reviews/design_review_typed-comparison-values_2026-09-17.md
evidence: Implemented
supersedes: []
superseded-by: null
revisit: A comparison scope requires an additional value variant or the pinned native set operator changes its nested-value contract.
verification: typed_comparison_ pure Arrow tests; just schema-conformance; native comparison preparation contract
---

# ADR-0056: Keep comparison alternatives and callable facts in declared native variants

## Context

Plan 19 CP06 requires typed before/after values and callable differences. Comparison previously
used native values for equality but converted the selected inline value through a generic JSON
tree. That erased the closed value contract at the result boundary and allowed it to differ from
the actual API payload declarations. Intermediate built-in `named_struct` projections also lost
immediate child metadata.

## Scope

Amend design §6.3 and §7.2 for comparison value representation, native equality and generated
transport. This decision does not certify producer completeness, installed comparison journeys,
durable field-explanation qualification, recovery or the rest of Plan 19.

## Drivers

- Preserve nested callable order, optional observations and qualified source independently.
- Make native preparation and transport consume the same finite declaration.
- Remove format round trips and obsolete readers during the authorized hard cutover.

## Options

1. Keep generic JSON inline values: rejected because it supplies no closed domain contract.
2. Use native Arrow UnionArray: rejected at this pin because DataFusion excludes Union from hashable types.
3. Generate a tagged Struct from the existing native-union declaration: selected; already shared by other evidence and wire variants.
4. Duplicate a comparison-specific callable model: rejected because it creates a second owner for producer facts.

## Decision

`ComparisonValue` declares API, fragment, Rust documentation configuration, Python header and
relationship variants. API observations reuse `ApiPayload`; documentation fields are null on
the API axis because documentation is compared independently. Source remains a separate
qualified `FactSource`; capture metadata does not define value equality. Null observation differs
from a missing public binding. Sequence fields retain order and duplicates.

Native record expressions construct complete declared fields directly from source columns.
`EXCEPT DISTINCT` compares typed value sets. Inline delivery decodes the generated native union;
artifact delivery streams its declared JSON wire format without a generic JSON value tree.
`ComparisonFieldPath` retains named fields and declared sequence ordinals. An immutable bounded
Arrow UDF lowers values into field paths and exact canonical typed bytes; DataFusion owns set
difference and ordering. Whole-value/parent facts preserve alternative correlation. The rejected
per-field native-join and wide-expression approaches exceeded the 30-second fixture deadline;
this is an application format-lowering kernel, not a second equality algorithm.

Native prefix selection chooses delivery mode and measures exact encoded bytes before the sink.
The sink verifies that measurement.

The current source wire becomes **7.0**, state **14**, comparison artifact format
`service:comparison-value/4`; snapshot stays **11.0**, executor **7**. Regenerate schemas, DTOs,
packaged guidance and fixtures. Refuse incompatible state; delete obsolete installations/state
under the Plan 19 CP11 barrier. No legacy reader, migration or dual mode is introduced.

Seal the wire-7 product guidance in `docs/provenance/bundle-2026-09-17-typed-comparison`.
Recover the predecessor guide's exact sealed bytes from Git and verify its original hash before
relocating that provenance copy. A separately sealed map joins the unmodified wire-5 manifest
to those bytes. All prior manifests, maps and frozen specifications remain unchanged. This is
document provenance, not an older runtime contract or an accepted implementation qualification.

### Consequences

Consumers receive discoverable typed variants instead of unstructured JSON. New axes require an
explicit declaration and regeneration. Complete-field validation remains application-owned;
DataFusion's structural schema check alone does not enforce semantic metadata. This is a breaking
wire contract and needs the matching adapter and fresh state before activation.

### Compensating controls

The comparison preparation family requires the generated value field and qualified source field.
Native constructor argument checks preserve metadata; generated union validation rejects invalid
variants. Pure Arrow tests cover all eight axes, null observations, callable order and set equality.
Schema conformance covers generated and packaged formats. CP12 owns actual durable/installed tests.

## Evidence

Exact versions and sources inspected 2026-09-17; bounded independent verification is recorded in
`.dev-state/plan19/execution/typed-comparison-upstream.md`.

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| DISTINCT set difference has a native logical-plan lowering | [DataFusion 55.1.0 logical-plan builder](https://docs.rs/datafusion-expr/55.1.0/src/datafusion_expr/logical_plan/builder.rs.html#1375-1448) | 2026-09-17 | “Process except set operator” |
| Built-in named_struct creates new immediate fields without their metadata | [DataFusion 55.1.0 named_struct](https://docs.rs/datafusion-functions/55.1.0/src/datafusion_functions/core/named_struct.rs.html#131-148) | 2026-09-17 | `Field::new(name, data_type.to_owned(), true)` |
| Arrow JSON has a field-aware encoder extension | [Arrow JSON 59.3.0 encoder](https://docs.rs/arrow-json/59.3.0/src/arrow_json/writer/encoder.rs.html#304-311) | 2026-09-17 | “Creates an encoder for the given array and field.” |

Local `datafusion-expr-55.1.0/src/utils.rs:963–1009` recursively permits Struct/List hashing and
excludes Union. Native set equality does not supply output order, domain validity, memory bounds
or completeness. The application supplies those contracts and checks.

## Verification

- `cargo test -p enrichment-store --lib typed_comparison_ --locked` — pure Arrow transformations;
  receipt `.dev-state/plan19/execution/typed-comparison-unit.log`.
- `just schemas-generate` includes generated/packaged schema conformance; receipt
  `.dev-state/plan19/execution/typed-comparison-schemas.log`.
- `cargo check --workspace --all-targets --locked` compiles dormant durable fixtures without
  running them; receipt `.dev-state/plan19/execution/typed-comparison-compile.log`.
- Full comparison publication, artifact/recovery and installed MCP qualification remain **not_run**
  until Plan 19 CP11 closes. Pure Arrow checks cannot certify these.

## Boundaries preserved

§B1–§B13 remain intact: Rust owns policy and semantics; Python is generated transport; identities,
qualified evidence and epistemic classes remain distinct; DataFusion executes the transformation;
Delta publication is unchanged. §B10 end-to-end completion is still required at CP12.

## More information

[Plan 19 CP06](../plans/19-unified-runtime-and-delta-cache-completion.md#cp06--native-research-complete-delivery-and-mcp-consumers),
[scoped review](../design_review/reviews/design_review_typed-comparison-values_2026-09-17.md),
[register R-53](register.md).

## Status history

- 2026-09-17 — proposed; native implementation and generated contract landed, full qualification remains open.
