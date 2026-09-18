---
id: ADR-0047
title: Generate semantic, storage and wire contracts from one native declaration
status: accepted
date: 2026-09-16
deciders: [paul-heyse]
level: decision
principles: [DM-02, DM-06, DM-07, DM-16, DM-24, DM-42, DM-52]
design: [§6.2, §6.3, §7.2, §7.3, §7.4, §8.2, §8.3, §17]
review: docs/design_review/reviews/design_review_plan17-schema-contract-decision_2026-09-16.md
evidence: Interface-checked
supersedes: [ADR-0043, ADR-0037]
superseded-by: null
revisit: A required field, operator or mutation cannot preserve the declared domain, presence, scope or bounded wire value through its qualified native route.
verification: Plan 17 SC01-SC10 and Q01-Q13; native schema, mutation, producer and real MCP boundary tests.
---

# ADR-0047: Generate semantic, storage and wire contracts from one native declaration

## Context

The owner explicitly authorized all of Plan 17 on 2026-09-16 as one hard pivot. The schema review
found independently maintained variants, role-string policies and lossy wire projections. Exact
pinned-source probes additionally show metadata erasure, valid optional parents rejected by nested
NOT NULL, and CHECK metadata without the enabling protocol feature. The accepted compact review
specifies the corrected proposal; acceptance of this decision does not certify its implementation.

## Scope

One finite Rust declaration supplies semantic Arrow fields, native expressions and their generated
producer, storage/read and wire projections. This amends schema ownership and the presentation
mechanism while preserving exact evidence, six epistemic classes, outcome/delivery separation,
precommitted complete results, independent aspect pages, typed recovery and export closure.
All Plan 15/17 control, effect, retention and installed qualification obligations remain mandatory.

## Drivers

One semantic owner per field/rule; lossless typed boundaries; native enforcement and optimizer
visibility; bounded real MCP values; complete removal of historical runtime mechanisms.

## Options

- Keep manual schemas/codecs and compare their tables: rejected as the delivered architecture;
  an equality assertion detects duplication without removing it.
- Store metadata and assume the libraries enforce its meaning: rejected by the pinned probes.
- Generate finite projections from Rust/Arrow declarations and enforce rules with native plans:
  selected. No general schema DSL, second query IR or generic semantic interpreter is introduced.

## Decision

Declare tagged variants, ID domains, vocabularies, requiredness, coordinate units, collection
meaning, scoped references and value encoding once. Generate enum/field consumers and per-variant
Struct layouts. Use typed microsecond UTC clocks, fixed-binary digests/domain IDs and exact numeric
representations. The native canonical contract binds every meaningful dependency and encoding
revision; physical table/file changes do not invent semantic identity changes.

Arrow extension metadata and the DataFusion registry are typed carriers, not proof of valid values
or authority. Full-field UDF contracts and one finite pre-coercion semantic analyzer reject domain
mismatches; derived logical/physical/output contracts are checked too. Native PK/Unique metadata
requires actual admitted facts. Scoped foreign-reference enforcement uses native anti/semi-joins.
Do not declare parsed UDF output fields equivalent to whole input arguments.

Derive storage/read layouts from the semantic declaration. Use NOT NULL where sound and generated
parent-aware total-Boolean predicates for conditional requiredness. Install CHECK through complete
Delta constraint operations with the real session and active protocol feature. Collection-child
conditions use generated native relational admission where CHECK cannot express them. All routes
must enforce the same rules; unsupported bypass writes refuse. Missing/renamed struct children are
errors before a native cast could silently fill them with NULL.

Generate evidence/result JSON Schemas, exact value encoders and mechanical Python bindings from
that authority. Python no longer authors independent domain presentation policies or reconstructs
results. Native relations select sections, pages, references and recovery; bounded final encoding
only renders the selection. Retain no old DTO, union allowlist, label/JsonObject replacement,
semantic JSON decode bridge, journal/state reader, old cursor, dual writer or compatibility mode.
Protocol-only envelopes may remain typed mechanical structures using generated semantic fields.

Before encoding a row, compute checked escaped-byte sizes or a proven conservative bound and
obtain the resource reservation. Arrow JSON buffers the row before Write, so a capped downstream
sink alone is insufficient. Use its EncoderFactory only with bounded allocation established in
advance; otherwise use a bounded mechanical field encoder with the same generated rules. Include
all UTF-8 escaping, keys, nesting, envelope and actual MCP framing. Full-range UInt64/Decimal uses
a declared exact string form unless the numeric domain is provably interoperable without loss.

Pass explicit empty or separately bounded content to FastMCP ToolResult alongside structured_content,
preventing implicit text duplication. Intentional recovery previews remain native-derived bounded
presentation; a missing preview never changes evidence or terminal outcome. The complete retained
result and retrieval action survive page/inline limits and are admitted before terminal success.

Activate the fully integrated implementation once on fresh state. Reacquire evidence through target
producers; no old-state import, conversion job or rollback executable is retained. Exact new source
bytes and target Delta history/replay/retention remain required functionality. Frozen specifications
and requested review evidence remain provenance.

### Consequences

Every field/variant change reaches native admission and transport through one declaration. The
implementation must qualify metadata propagation, physical nullability, collection rules and exact
encoding; merely selecting a library API is insufficient. New schema/canonical/wire identities
invalidate old data directly. Producer-specific callable records improve existing inspect/compare;
they do not establish cross-language equivalence or replace actual runtime verification.

### Compensating controls

Exact pinned source, shared session/planner, generated positive/negative field cases, independent
producer facts, invalid-write/refusal tests, scoped reference fixtures, allocation/frame limits and
fresh installed nine-tool/client acceptance. Target runtime gates remain not_run until executed.

## Evidence

Read 2026-09-16 in the pinned skills and exact local source; the compact upstream review and
Plan 17 evidence retain the detailed source references and executed library probes.

| Claim | Primary source | Retrieved | Quote |
|---|---|---|---|
| UDF full fields include nullability | [DataFusion 55.1](https://docs.rs/datafusion-expr/55.1.0/src/datafusion_expr/udf.rs.html) | 2026-09-16 | `fn return_field_from_args(&self, args: ReturnFieldArgs)` |
| Delta gates CHECK by feature | [Pinned validation](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/data_validation.rs) | 2026-09-16 | `is_feature_enabled(&TableFeature::CheckConstraints)` |
| Constraint operation installs protocol and metadata | [Pinned constraint builder](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/constraints.rs) | 2026-09-16 | “Put all the constraint into one commit” |
| Encoder allocates before output write | [Arrow JSON 59.3](https://docs.rs/arrow-json/59.3.0/src/arrow_json/writer/mod.rs.html) | 2026-09-16 | `encoder.encode(idx, &mut buffer)` |
| Omitted MCP content copies the structured value | [FastMCP 4.0.3](https://github.com/PrefectHQ/fastmcp/blob/v4.0.3/src/fastmcp/tools/base.py) | 2026-09-16 | `content = structured_content` |

## Verification

Plan 17 SC01-SC10 define generated variant/reference addition, actual requiredness/CHECK refusal,
field/domain propagation, typed identities, exact wire values, callable observations, invalidation
and layout measurement. Q01-Q13 retain full runtime, deletion, resource, installed/client and
power-loss requirements. The document-stage review accepts this specified architecture only.

## Boundaries preserved

Rust owns core semantics, policy, publication and jobs. Python remains a thin FastMCP 4 adapter
and separate extraction worker. Hosted rustdoc preference, static Griffe, the selected ty and
rust-analyzer producer contracts, isolated effects, exact provenance, truthful coverage and bounded
MCP functionality remain required. Operator ADR-0046 and unrelated harness changes remain intact.
Frozen blueprint/handoff bytes are unchanged; protected enforcement updates use the allowed
operator patch installation boundary.

## Status history

- 2026-09-16 — supersedes ADR-0043 and ADR-0037 mechanisms for the authorized Plan 17 hard pivot; preserved functional and ownership obligations are stated above.

- 2026-09-18 — implementation checkpoint: binary Job/Interest/Attempt domains, full-field
  parameters/collections, typed job/qualification clocks and a shared native diagnostic owner
  advance the accepted declaration contract. Source is state 22/snapshot 12.0/wire 8.0.
  The active tool guide is sealed in `docs/provenance/bundle-2026-09-18-native-identities/`;
  prior sealed bytes remain unchanged. Plan 19 records scoped units and the still-open
  architecture/deletion/qualification boundary; no new implementation acceptance is asserted.

- 2026-09-18 — implementation checkpoint: native binary GrantId and canonical claim admission;
  one generated Rust resource catalog; full-field SQL coalesce; fresh exact Delta history before
  cached table/provider/descriptor reuse. Source is state 23/snapshot 13.0/wire 9.0, executor 8
  and native-rustdoc-arrow/4. The active guide is sealed in
  `docs/provenance/bundle-2026-09-18-native-grants-resources/`; earlier seals are preserved.
  These implement existing decisions; terminal Plan 19 qualification remains open.

- 2026-09-18 — implementation checkpoint: eight typed immutable-definition identities,
  declaration-bound definition storage, and one admitted retention row selection used for
  semantic reads, pending-writer recovery and native Delta binary deletion. Source is state
  24/snapshot 14.0/wire 10.0 and executor 9; decoder remains native-rustdoc-arrow/4. The active
  guide is sealed in `docs/provenance/bundle-2026-09-18-typed-definitions/`. This implements
  existing decisions; no predecessor reader is retained and terminal qualification remains open.

- 2026-09-18 — implementation checkpoint: six typed lifecycle/physical-ownership UUID domains,
  typed inline retention-policy identity, one private-directory dependency union and native
  cleanup/admission joins. Broker names and quarantine leaves are boundary renderings; legacy
  dependency variants and redundant stored path components are removed. Source is state 25 /
  snapshot 15.0 / wire 11.0; executor 9 and native-rustdoc-arrow/4 are unchanged. The active guide
  is sealed in `docs/provenance/bundle-2026-09-18-typed-ownership/`. This implements the accepted
  declaration contract; full storage/restart/client qualification remains deferred behind CP11.

- 2026-09-18 — implementation checkpoint: shared Delta table/version/selection/CDF references,
  binary CohortId and SchemaContractId, native schema-registry records, returned-state capture
  and shared exact-read admission across evidence/results/definitions/replay. Legacy flat
  binding types and string semantic identity authority are removed. Source is state 26 /
  snapshot 16.0 / wire 12.0; executor/decoder/provider codec framing is unchanged. The active
  guide is sealed in `docs/provenance/bundle-2026-09-18-typed-delta-references/`. Sixteen distinct
  scoped units pass; Plan 19 records the initial byte-bound validator failure and repair.
  This implements the accepted contract; terminal qualification remains deferred behind CP11.
