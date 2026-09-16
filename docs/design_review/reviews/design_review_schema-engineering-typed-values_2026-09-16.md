# Design review: schema engineering for typed values across the DataFusion/Delta runtime

## 1. Decision and scope

**Decision: Revise.** The structural program in the
[typed struct schema essay](../capability-maps/struct_schema_review.md) — coupled values as
`Struct`, owned repeated records as `List<Struct>`, optional aggregates as nullable structs,
tagged unions as discriminated nullable payloads, one contract registry, planning-time
compatibility checks — is **already implemented for the evidence relations and the control
table**. The essay's flat-columns-versus-structs framing therefore does not describe this
codebase's largest remaining opportunity. What remains is about *where meaning lives*: the
semantic layer above the Arrow datatypes is carried by string conventions (role strings matched
by prefix, hex text for digests and identities, RFC 3339 text for clocks, a `kind` column whose
sibling `start` changes unit by row), the shape of every tagged union is maintained in three
places by hand, typed values degrade to labels and JSON objects at the wire boundary, and the
five product UDFs declare only a return datatype, so the role vocabulary is never checked at
plan time. Two gates fail on in-scope behavior (G1 authority, G2 fidelity); the correction is a
consolidation of the contract layer, not a re-architecture, and most of what exists should be
preserved.

**Proposal reviewed:** the essay (a Proposed design document) applied to the active codebase in
the dirty Plan 15 working tree on 2026-09-16 (HEAD `9c154de8b2ead1d082fd253399bb1694627395c1`
plus uncommitted changes). Code scope: the Arrow evidence contracts
(`crates/enrichment-core/src/evidence/arrow_model/`), the identity and operation contracts
(`native_key.rs`, `native_identity.rs`, `operation.rs`, `telemetry.rs`), the wire types
(`wire/`), and the store's schema-bearing modules (`native_delta.rs`, `delta_cohort.rs`,
`delta_evidence.rs`, `control.rs`, `control_jobs.rs`, `arrow_contract.rs`, `preparation.rs`,
`admission.rs`, `native_catalog.rs`, `projection/`, `views.rs`, both normalizers).

**Reviewer:** Claude (Fable 5.1). **Date:** 2026-09-16. **Depth:** deep.
**Status of claims:** code behavior is *Implemented* where cited by `file:line`; *Tested* only
where a named test or a `.dev-state/plan15/` receipt is cited; *Measured* for exactly one
number (the documentation-column projection); library capabilities are *Interface-checked*
against the pinned DataFusion and delta-rs skill indexes; every recommendation is *Proposed*.
No new benchmark or probe was run for this review.

**Observable outcome sought:** fewer independently maintained semantic decisions per record
family (one declaration per tagged union, per vocabulary, per identity domain), errors that
today surface only as empty query results or as a failed decode surfacing at plan time or at
Delta write time instead, and a wire contract that is derived from the same declaration as the
storage contract rather than mapped to it by hand.

**Baseline — what is already true and must be preserved.** The ten evidence relations are
typed Arrow records with struct-valued `subject`, `target`, `locator`, `source` and `payload`
columns (`crates/enrichment-core/src/evidence/arrow_model/encode.rs:142-225,227-481,483-550,552-712`),
`List<Struct>` for gaps, inputs, files, headers and inventory
(`provenance.rs:18-84,139-160`; `metadata.rs:124-214`), nullable per-variant structs for the
execution payload and release metadata (`execution.rs:458-558`; `metadata.rs:33-333`), a
`record_kind` tag plus fifteen nullable structs for the control union with a generated Delta
CHECK selecting the live payload (`crates/enrichment-store/src/control.rs:1440-1481`), a
semantic/storage schema split with a digest-keyed contract registry
(`native_delta.rs:35-98,680-685`), a directional field-compatibility checker
(`preparation.rs:466-495`), admission plans compiled from the schema's role metadata
(`arrow_model/checks.rs:125-538`), native struct-valued outputs for ranking and comparison
(`projection/score.rs:11-36`; `comparison.rs:44-50,111-114`), and the only Arrow extension
type in the workspace, `arrow.json`, on the producer-extension locator (`encode.rs:244`). The
essay's §1 table is, for this codebase, mostly a description of the present.

**Supported scope and non-goals.** Not reviewed here: the claim/publication protocol and Delta
commit semantics (reviewed 2026-09-15), execution effects and containment, the Python adapter,
MCP tool behavior beyond the evidence-bearing wire shapes, and performance beyond the one
measured receipt. Per the owner's instruction the existing policies and rules were not treated
as constraints on the recommendations; §10 names the recommendations that would touch a
recorded decision so that step can follow separately.

**Constraints and uncertainty.** Pins: DataFusion 55.1.0, Arrow/Parquet 59.3.0, object_store
0.13.2, delta-rs `58f07cd6` on the Buoyant kernel `8ba063f8f84fec222000f66d40d70911d7c79675`. The deltalake skill's corpus
contains examples, guides, tests and the protocol but not `crates/core/src`, so two facts that
matter here — whether the pinned writer computes file statistics for nested leaves, and whether
it enforces `NOT NULL` on a nested struct child at write — are **index-silent** and are labeled
Proposed with a probe named in §9. The DataFusion skill's corpus likewise has guides and
examples but not crate source.

### Method and coverage

Read in full: the essay; `STATUS.md`; Plan 15 §3 and Plan 16 §1–2; DESIGN.md §6 and §17; the
charter, addendum, template and reference; every file under `arrow_model/`; `native_key.rs`,
`native_identity.rs`, `operation.rs` (schema sections), `telemetry.rs`, `wire/evidence.rs`,
`wire/data.rs` (DTO sections), `evidence/{model,relational,snapshot,execution,path}.rs`;
`native_delta.rs`, `delta_cohort.rs`, `delta_evidence.rs` (head), `arrow_contract.rs`,
`control.rs` (families, union, rules), `control_jobs.rs` (schemas, encode), `preparation.rs`
(compatibility), `admission.rs` (relations, rules), `native_catalog.rs` (constraints,
inventory), `projection/{publication,score,search,render,mod}.rs`, `search_projection.rs`,
`views.rs`, `coverage_plan.rs`, `registry.rs` (row decode), and the relevant parts of
`python_normalize.rs`, `comparison.rs`, `http_cache.rs`, `result_catalog.rs`,
`execution_policy.rs`, `coverage.rs`, `query.rs`. Tests read: `tests/typed_arrow.rs`
(round-trip and metadata assertions), `tests/views.rs` (projection measurement). Receipts read:
`.dev-state/plan15/native-column-projection-tests.log` and the STATUS entries for the
nullability, presence and nested read-layout receipts.

Delegated: one read-only survey of flattening patterns across the three crates and one
capability inventory over the two skills. Every survey lead cited below was re-read at the
cited line before use; the capability facts below were looked up in
`content/index/*.tsv`, `content/api/*.md`, `content/catalogs/*.md` and the corpus directly
(the queries are recorded under `evidence/schema-engineering-typed-values-2026-09-16/`).
Context7 was not used.

Not inspected: the daemon's `ops/*.rs` beyond `delivery.rs:115-175`, `bundle.rs`, `blob.rs`,
`repository.rs` beyond its decode sites, the Python worker, the Rust worker's fact extraction
internals, and every test other than the two named. Guarantees **not attacked**: concurrency of
contract registration, CDF semantics, and the claim that the `ArrowContract` physical cast
restores nested metadata after every optimizer rewrite (asserted in DESIGN.md §17; one
Parquet/DataFusion round trip is tested, no UNION or join case is). Guarantees attacked: struct
child order through `project()` (§5, finding S07), cross-domain equality joins (S05), the
three-way variant table (S01).

## 2. Authority and lifecycle map

Reconstructed from the code. An "authority" is a place where the fact can be edited
independently; two rows marked ✗ are the G1 evidence.

| Fact | Semantic type and identity | Authority / owner | Derived representations | One authority? |
|---|---|---|---|---|
| Physical shape of an evidence relation | Arrow `Schema` returned by an empty encode (`Relation::schema`, `admission.rs:69-84`) | The encoder function in `arrow_model/*` | `StorageContract.storage` (`native_delta.rs:148-232`), Delta `StructType`, `Key::schema` subsets (`native_key.rs:140-161`), search-surface contracts (`search_projection.rs:161`) | ✓ (derived) |
| Rust value shape of the same relation | serde/schemars struct in `evidence/relational.rs` | The struct | JSON Schema, Pydantic model, wire DTOs (`wire/data.rs:253-263` embeds the same types) | ✗ — the struct and the encoder/decoder are edited independently; agreement is checked by one fixture round trip (`tests/typed_arrow.rs:158-222`) |
| Tagged-union variant → member fields (subject, target, locator, execution target) | Which children may be non-null under each `kind` | (a) the enum in `relational.rs:18-153` / `execution.rs:119-137`; (b) the variant table in `checks.rs:343-401`; (c) the `variant(&[...])` allowlists in `decode.rs:76-232`, `relations.rs:74-93`, `execution.rs:158-173` | Native admission predicates (`checks.rs:412-425`); decode rejection (`cells.rs:219-229`) | ✗ — three hand-kept copies, no derivation, no equality test |
| Closed vocabularies (`kind`, `origin`, `outcome`, …) | Finite string sets, versioned in the role (`vocabulary:subject/1`) | The Rust enum, read through schemars (`checks.rs:13-35`) | `in_list` predicates (`checks.rs:257-266`); frozen `enums.json` conformance | ✓ (derived) |
| Field semantics (role, unit, base, domain) | `enrichment.role` string on the `Field` (`cells.rs:343-352`) | The encoder call site; interpreted by prefix in `checks.rs:257-282`, by equality in `preparation.rs:466-477`, displayed by `native_catalog.rs:615` | Delta stores none of it (`storage_field` drops metadata, `native_delta.rs:148-154`); restored on read via `DeltaScanConfig::with_schema` (`:556`) and the IPC registry | ✓ in count, but the *meaning* of a role string exists only in the consumers' `match` arms |
| Required-ness of nested children | Physical nullability, then `enrichment.null=forbidden` after `native_read_field` (`cells.rs:397-419`) | The encoder's `nullable` argument | `checks::required` (`checks.rs:114-120`); Delta stores the **nullable** layout (`batch()` at `cells.rs:370-395` runs before `StorageContract::new`) | ✓, but not enforced by storage (S04) |
| Content identity | `prefix_` + 64 hex over canonical bytes of the declared fields (`native_key.rs:230-266`) | `Key::schema` + `CanonicalBytes` (`native_identity.rs:60-152`) | Same graph constant-folded for DTO ingress (`native_key.rs:269-306`) | ✓ |
| Semantic contract identity | SHA-256 of the JSON of the semantic `Schema`, including role metadata (`native_delta.rs:55-56`) | `StorageContract::new` | Delta table property `enrichment.arrowContract`; `semantic_contracts` row with IPC bytes (`:680-685`) | ✓ |
| Control record families | `record_kind` + one nullable struct per family (`control.rs:1440-1454`) | `Table::ALL` + each family's schema function | `tag_rule()` / `command_rule()` CHECK constraints (`:1456-1481`), `pack_plan` / `pack_rows` (`:1484-1558`) | ✓ (generated) |
| Cross-relation references | `ref:<relation>` role on the field | The encoder call site | **Not** derived: 5 + 14 hand-written `SqlRule` anti-joins (`admission.rs:261-285`; `control.rs:40-107`); `validated_constraints` derives only the primary key from `key()` (`native_catalog.rs:30-48`) | ✗ (role says "reference", rule says which one) |
| Wire shape of an evidence entry | `wire::Evidence { subject: String, locator: JsonObject, … }` (`wire/evidence.rs:151-173`) | The DTO | Rendered by hand from the typed row (`render.rs:85-114`) | ✗ — a second, lossy authority for subject and locator |

**Deliberately opaque behavior.** Canonical-byte framing (`native_identity.rs:118-244`),
SemVer/PEP 440 precedence keys (`native_version.rs`), URL parsing (`native_url.rs`) and the
`ArrowContract` physical cast (`arrow_contract.rs:85-127`) are bounded kernels behind declared
signatures. That placement is correct (charter §F) and nothing below proposes moving them.

**Identity behavior.** A role string is part of the contract digest, so renaming a role,
changing a unit annotation or adding a nested child changes `contract_id`, which is what keys
the search-surface tables (`search_projection.rs:161`) and what `verify_contract` refuses on
an existing evidence table (`native_delta.rs:628-659`). There is no in-place schema evolution
route; evolution is a new epoch. That is a legitimate DM-51 choice and is recorded in §10.

## 3. Semantic contracts and invariants

| Contract or invariant | Representation today | Enforcement boundary | Failure behavior | Evidence |
|---|---|---|---|---|
| A tagged value populates exactly its variant's members | Flattened shared nullable columns + `kind` (subject/target/locator); per-variant nullable structs (execution payload, release metadata, control, command arguments) | Admission predicates (`checks.rs:328-427`); decode (`cells.rs:219-229`); Delta CHECK for the control union only (`control.rs:1456-1481`) | Candidate cohort not selected; decode error; Delta write rejected (control) | Tested: `native-presence-contract-tests.log`, `native-field-contract-tests.log`; the variant table itself is untested against the enum (S01) |
| Nested required child is present when its parent is | `enrichment.null=forbidden` metadata after nullable read layout | Admission predicate `present AND value IS NULL` (`checks.rs:248-250`) | Cohort not selected | Tested for one case (`native-presence-contract-tests.log`); **not** a storage guarantee (S04) |
| Coordinates are well-formed for their variant | `coordinate_checks` (`checks.rs:440-538`), including the `u32` domain of `start` for line variants (`:517-534`) | Admission | Cohort not selected | Implemented; the unit itself is not typed (S03) |
| Vocabulary membership | `in_list` over schemars-derived values (`checks.rs:257-266`) | Admission | Cohort not selected | Implemented; frozen-enum conformance tested (`tests/wire_conformance.rs`) |
| Digest shape | Regex `^[0-9a-f]{64}$` (`checks.rs:277-281`); Delta CHECK `regexp_like(body_digest, …)` (`http_cache.rs:112`); Rust byte loop (`acquisitions.rs:128-139`) | Admission / write / decode | Rejected | Implemented; the same rule in three notations (S03) |
| Clock values are timestamps | Utf8 in evidence and publication relations; `try_cast(retrieved_at AS TIMESTAMP) IS NOT NULL` CHECK (`http_cache.rs:117-119`); `Timestamp(µs, UTC)` in control families (`control_jobs.rs:14-20`) and telemetry (`telemetry.rs:320-324`) | Write (HTTP) / none (evidence) | Rejected (HTTP); silently accepted (evidence) | Implemented; inconsistent (S03) |
| A field's meaning survives a plan | `enrichment.role` on the output field compared to the expected field (`preparation.rs:445-477`) | After planning, for declared query families only | `InvariantFailure::contract` | Implemented; intermediate expressions are unchecked (S05) |
| Two struct values with the same child types are the same type | `equals_datatype` passthrough in `project()` (`native_delta.rs:114`) followed by the `ArrowContract` physical cast (`arrow_contract.rs:105-124`) | Delta write projection | None — a struct whose children match by type but not by name passes the logical check; what the physical cast then does depends on which cast runs (S07) | Unattacked until this review (S07) |
| Storage mapping is lossless | `storage_type` (`native_delta.rs:156-232`): `UInt64→Decimal128(20,0)`, views→`Utf8`, `Dictionary→value`, fixed-size and non-µs temporal types rejected | Contract construction | Explicit `unsupported Arrow/Delta storage contract` | Tested for `u64::MAX` (`native-scan-schema-tests.log`) |
| Field metadata survives storage | Dropped at `storage_field`; restored from the IPC registry through `with_schema` | Read | Contract mismatch is refused | Tested for one nested Parquet round trip (`tests/typed_arrow.rs:199-206`); not for a Delta UNION/join |

**Absence and uncertainty.** The code distinguishes: a null struct (variant inactive), a struct
with null children (present but unobserved, `enrichment.null=unobserved`), an empty list, a
null list, and a `*_known` flag beside a list. The last is where the design has *not* used the
type: `Option<Vec<String>>` is encoded as `targets_known: Boolean` + `targets: List<Utf8>`
(`metadata.rs:80-91`, consistency re-checked at `:336-340`) and as `features_known: Boolean` +
`features: List<Utf8>` (`native_key.rs:176-177`; `identity/mod.rs:277-279`). A nullable
`List` expresses exactly "unknown versus empty" and both `storage_type` and the identity
kernel (`native_identity.rs:154-158`, null tag 0 versus length 0) already handle it (S08).

**Equivalence requirements.** Contract identity is byte equality of the JSON-encoded semantic
schema — so it is sensitive to metadata key order only insofar as `serde_json` orders map keys,
which it does by default. Storage-schema equality is exact `Field` equality after the mapping
(`native_delta.rs:641-658`). Neither uses `DataType::equals_datatype`; only `project()` does,
by design, for passthrough (S07).

## 4. Derivation and execution design

### 4.1 The path a typed value takes today

```text
Rust value (relational.rs)
  -> encoder: struct/list columns + enrichment.role/null/contract metadata   encode.rs, cells.rs:343-352
  -> batch(): nullable nested read layout, relation metadata                  cells.rs:370-419
  -> StorageContract::new: storage schema (metadata dropped, unsigned mapped) native_delta.rs:35-98,148-232
     contract_id = sha256(json(semantic schema)); IPC bytes registered        :55-56, :477-524
  -> DeltaStore::append: WriteBuilder with_input_plan, CHECK rules, app txn  :374-415
  -> DeltaScanNext + DeltaScanConfig::with_schema(semantic)                   :541-562
  -> native views (binding_sql / surface_sql) and admission plans             views.rs:212-224; checks.rs
  -> preparation::result: output-field role compatibility                    preparation.rs:445-495
  -> render / RowSet decode / arrow::json round trip                          render.rs; registry.rs:166-182
  -> wire DTO (schemars) -> JSON Schema -> Pydantic                           scripts/schemas-generate.sh
```

Two features of this path decide most of the findings. First, **meaning is attached at the
encoder call site as a string** and recovered by four consumers that each pattern-match the
string. Second, **the same shape is declared twice** (Rust struct; encoder) and, for tagged
unions, three times.

### 4.2 What the pinned libraries offer for the semantic layer (Interface-checked)

| Capability | Canonical path (file read) | Present at pin | Used today | Disposition |
|---|---|---|---|---|
| Full-field UDF planning | `datafusion_expr::udf::ScalarUDFImpl::return_field_from_args(ReturnFieldArgs{arg_fields, scalar_arguments}) -> FieldRef` (`content/index/methods.tsv`; `api/datafusion_expr.udf.md`) | yes | no — all five product UDFs implement only `return_type` (`native_identity.rs:73`, `native_url.rs:40`, `native_version.rs:32,126,215`) | Adopt (S05) |
| Execution-time field access | `ScalarFunctionArgs{args, arg_fields, number_rows, return_field, config_options}` (`api/datafusion_expr.udf.md`) | yes | `args`, `number_rows` only | Adopt with S05 |
| Optimizer-facing UDF hooks | `simplify`, `coerce_types`, `is_nullable`, `output_ordering`, `preserves_lex_ordering`, `struct_field_mapping(literal_args) -> Option<StructFieldMapping{field_accessor, fields}>`, `short_circuits`, `preimage`, `evaluate_bounds` (`api/datafusion_expr.udf.md`) | yes | none | `struct_field_mapping` for `url_parts_v1` and `pep440_value_v1` (struct-returning); others conditional |
| Expression field derivation | `ExprSchemable::{to_field, metadata, data_type_and_nullable}`; `datafusion_common::metadata::FieldMetadata{add_to_field, merge_options, new_from_field}` | yes | `get_type`, `cast_to` only (`native_delta.rs:110,120`) | Adopt (S05) |
| Metadata-carrying expressions | `datafusion_expr::expr::Alias::with_metadata(Option<FieldMetadata>)`; `datafusion_expr::literal::lit_with_metadata`; `datafusion_functions::core::expr_fn::with_metadata`; `Cast::new_from_field` | yes | `Cast::new_from_field` in `scoring.rs:38-40` | Adopt |
| Arrow extension types | `arrow_schema::extension::ExtensionType{NAME, try_new, validate, serialize_metadata, deserialize_metadata, supports_data_type}`; `Field::{with_extension_type, try_extension_type, extension_type_name, try_canonical_extension_type}`; canonical `Json`, `Uuid`, `Opaque`, `Bool8`, `TimestampWithOffset`, tensors | yes; Parquet embeds field metadata on write (`datafusion.execution.parquet.skip_arrow_metadata` = false) but the reader skips it by default (`datafusion.execution.parquet.skip_metadata` = true, `catalogs/config-options.md`) | `Json` only (`encode.rs:244`); the Delta read path restores metadata from the IPC registry through `with_schema`, not from the file | Adopt (S02) |
| DataFusion extension-type registry | `datafusion_common::types::extension::DFExtensionType{storage_type, serialize_metadata, create_array_formatter}` with seven canonical impls; `datafusion_expr::registry::{ExtensionTypeRegistry, MemoryExtensionTypeRegistry, ExtensionTypeRegistration}`; `SessionStateBuilder::with_extension_type_registry`; example `corpus/examples/extension_types/temperature.rs` | yes | no | Adopt (S02) |
| Plan-wide semantic rules | `SessionStateBuilder::{with_analyzer_rule(s), with_expr_planners, with_type_planner}` | yes | no rules; `with_query_planner` only (`runtime.rs:639-641`) | One analyzer rule (S05); no expr/type planner |
| Struct coercion and casting | name-based across UNION, arrays, joins, aggregates; explicit `CAST(... AS STRUCT(...))` for order (`corpus/guides/user-guide/sql/struct_coercion.md`); `datafusion_common::nested_struct::{cast_column, requires_nested_struct_cast, has_one_of_more_common_fields}` — struct-to-struct casts match children by name, null-fill missing target children and require at least one common name (`api/datafusion_common.nested_struct.md`); arrow-cast 59.3 documents casts "to or from `StructArray`" as unsupported (`api/arrow_cast.cast.md`) | yes | `project()` decides passthrough with `equals_datatype` and leaves the physical cast to choose | Fix (S07) |
| Unnest | `DataFrame::unnest_columns_with_options`; `UnnestOptions::{with_preserve_nulls, with_recursions, with_null_handling}` | yes | `with_preserve_nulls(false)` (`checks.rs:193,304`; `python_normalize.rs:539,577`) | Preserve; remove the parallel-list use (S08) |
| Union datatype | `datafusion_functions::core::expr_fn::{union_extract, union_tag}`; `arrow_array::UnionArray` | yes in DataFusion; **no union in the kernel `DataType`** (`buoyant_kernel::schema::DataType`: struct/array/map/primitive) | rejected by `storage_type` | Do not use for stored relations |
| Parquet nested predicate pushdown | `datafusion_datasource_parquet::row_filter::can_expr_be_pushed_down_with_schemas`: struct leaves via `get_field` with primitive type are pushable; whole-struct references are not (`api/datafusion_datasource_parquet.row_filter.md`) | yes | `decoder_filter`/`reorder_filters` are policy fields (`operation.rs:201-203`) | Preserve; prefer leaf predicates |
| Nested leaf projection | `parquet::arrow::ProjectionMask::{leaves, roots, columns}` exists; `TableProvider::scan(projection: Option<&Vec<usize>>)` and `DeltaScan::scan` are top-level (`api/deltalake_core.delta_datafusion.table_provider.next.md:123`) | leaves at the Parquet layer only | **Measured:** un-nesting `docs` cut an inspection read from 1,018,844 to 20,714 bytes (`tests/views.rs:25-28`; `native-column-projection-tests.log`) | Design rule: large text leaves stay top-level |
| Nested schema adaptation at scan | `datafusion_physical_expr_adapter::schema_rewriter::{PhysicalExprAdapterFactory, BatchAdapterFactory}` — column reordering, casting, null-fill of missing nullable columns, "nested struct fields are recursively adapted" (`api/datafusion_physical_expr_adapter.schema_rewriter.md:86-94`); `SchemaAdapter`/`DefaultSchemaAdapterFactory` deprecated since 52.0.0 (`api/datafusion_datasource.schema_adapter.md`) | yes | no (evolution is a new epoch) | Record as the alternative to epoch evolution (§10) |
| Delta field metadata | `buoyant_kernel::schema::StructField::{with_metadata, add_metadata, get_config_value, metadata_with_string_values}`; `ColumnMetadataKey::{ColumnMappingId, ColumnMappingPhysicalName, ColumnMappingNestedIds, GenerationExpression, Invariants, CurrentDefault, Identity*, ParquetFieldId, ParquetFieldNestedIds, InternalColumn, MetadataSpec}` | yes | none (storage fields carry no metadata) | Optional (§10) |
| Delta nested nullability | `StructField::{not_null, nullable, is_nullable}`; kernel `DataCheck` implementors `Invariant`, `Constraint`, `GeneratedColumn`; `StructTypeExt::{get_invariants, get_generated_columns}` | declared; the kernel ships recursive `SchemaTransform` checkers `NonNullFieldChecker` and `InvariantChecker` (`api/buoyant_kernel.transforms.schema.md:59-61`), but **whether delta-rs's write path runs them on nested children is index-silent**; a missing top-level non-nullable column fails a schema-merge write with "Invalid data found" (`corpus/tests/it_datafusion/integration_datafusion.rs:2017-2044`) | top-level `NOT NULL` Tested (`native-write-nullability-tests.log`) | Probe, then adopt (S04) |
| Delta CHECK over nested paths | `TableProperty` + `delta.constraints.<name>` configuration; `command_rule` uses `commands.arguments.<kind>` | yes | Tested by control-table creation receipts | Generate for evidence unions (S01) |
| Delta statistics selection | `TableProperty::{DataSkippingNumIndexedCols, DataSkippingStatsColumns, CheckpointWriteStatsAsStruct}`; `WriterStatsConfig{num_indexed_cols, stats_columns}`; kernel `StatsColumnVerifier::new(Vec<(ColumnName, DataType)>)`; `ColumnName::{new, from_naive_str_split, parent, join}` are name paths | yes for selection; **nested-leaf statistics do not reach DataFusion at this pin**: delta-rs's own nested-struct statistics test is disabled pending delta-kernel-rs#1075 (`corpus/tests/it_datafusion/integration_datafusion.rs:763-775`); DataFusion's `datafusion_common::pruning::PruningStatistics::{min_values, max_values, null_counts}` take a flat `&Column` (`Column::new(relation, name)`, no path segments); kernel-side nested data skipping is index-silent | none set; Bloom on `observation_id` only (`native_policy.rs:184-186`) | Probe, then set (S09) |
| Delta data-skipping predicates | `deltalake_core::delta_datafusion::expr::parse_sql_predicate_to_kernel` — "only constructs the kernel can evaluate against file-level metadata are accepted" (`api/deltalake_core.delta_datafusion.expr.md`) | yes | cohort filter `cohort_id = ?` (`delta_cohort.rs:87-99`) | Measure with S09 |
| Partitioning / clustering | protocol: `partitionColumns: Array[String]` are top-level names; `clusteringColumns` entries are name paths that may be nested (`corpus/protocol/PROTOCOL.md:511,1853`) | yes; `OptimizeType::ZOrder` accepts nested dotted paths and rejects unknown ones (`corpus/tests/it_datafusion/command_optimize.rs:2039-2140`) | no table is partitioned; no Z-order | Hypothesis only (§9) |
| Column mapping for nested renames | protocol §column mapping with `delta.columnMapping.nested.ids` (`PROTOCOL.md:968,1524,1559`); `TableProperty::ColumnMappingMode` | yes | no; evolution is a new contract epoch | Record as the chosen alternative (§10) |

### 4.3 Where the semantic layer is not typed

The essay's "domain field = datatype + nullability + semantic metadata + identity/version +
compatibility rules" is present here as `Field{datatype, nullable} + enrichment.role +
enrichment.null + enrichment.contract`. The role carries the missing three in a string:

| Role string (`encode.rs`, `provenance.rs`, `acquisitions.rs`, `execution.rs`) | What it means | Who interprets it |
|---|---|---|
| `key:definition`, `ref:symbol`, `ref:artifact` | identity domain, foreign relation | `checks.rs:267-276` (only "non-empty, no control chars"); `native_catalog.rs:30-48` (primary key by name, not by role) |
| `vocabulary:locator/1` | closed set + version | `checks.rs:257-266` (`strip_prefix`) |
| `sha256`, `content-digest:sha256` | algorithm | `checks.rs:277-281` (regex) — two spellings for one meaning (`acquisitions.rs:39` vs `provenance.rs:320`) |
| `zero-based-line`, `utf8-byte-boundary` | base and unit | nobody; `Symbol.span_line` is documented 1-based (`model.rs:151`) and `Utf8Position.line` is 0-based |
| `coordinate-start`, `coordinate-end` | a number whose unit is line, byte or ordinal depending on the sibling `kind` (`encode.rs:326-345`) | `coordinate_checks` by variant (`checks.rs:465-534`) |
| `acquisition-clock`, `attempt-start-time`, `observed-timestamp` | RFC 3339 instant | nobody at the type level; `try_cast` at use sites (`http_cache.rs:117-119`; `execution_policy.rs:261`) |
| `ordered:path-components`, `ordered:overloads` | ordered list | the identity kernel treats every list as ordered anyway |

This is the essay's §6 point in its sharpest form: the annotations are portable declarations,
but the "authoritative interpretation, compatibility rules and validation" are spread across
four consumers with no shared parser and no type for the role.

## 5. Representative journeys

### Ordinary extension: add a `Locator` variant

Adding, say, `Locator::NotebookCell { file, cell, line }` requires: the enum arm
(`relational.rs:89-153`) and its `validate` arm; three or four new nullable columns or reuse of
`file`/`start` in the encoder (`encode.rs:227-481`); a `variant(&[...])` allowlist in
`decode.rs`; a row in the `tagged_checks` variant table and, if the unit differs, a
`coordinate_checks` arm (`checks.rs:358-372,465-534`); the expression re-encoders in both
normalizers (`python_normalize.rs:357-367`; `rust_normalize.rs:356-364`) if a producer emits
it. Nothing ties these together at compile time; the round-trip fixture catches a mismatch
only if the fixture contains the new variant. Multiple files is not the defect — the same
variant→members mapping written three times is (DM-56, S01).

### Meaningful change: give coordinates a typed unit

Changing `locator.start` from a shared `UInt64` into per-variant typed children changes the
semantic schema, hence `contract_id`, hence every evidence table's `enrichment.arrowContract`
property and the search-surface table names. `verify_contract` will refuse the old tables; the
change is an epoch (Plan 15 §1.1 forbids migration). The semantic diff is expressible: the
old role `coordinate-start` disappears and `lines.start`/`bytes.start` appear with distinct
roles. Identity of observations does **not** change unless the `Key` schema's fields change,
because `native_identity::type_bytes` frames field names and types (`:120-152`) — reordering a
struct's children *does* change a key. That coupling is correct and should be stated in the
contract-change procedure.

### Boundary: Rust → Arrow → Delta → DataFusion → MCP

Preserved: names, datatypes, nested structure, nullability of top-level columns, role metadata
(via the registry and `with_schema`), the `arrow.json` extension on `locator.extension`.
Deliberately transformed: `UInt64→Decimal128(20,0)`, view strings →`Utf8`, nested children →
nullable. Lost at the wire: `SubjectRef` becomes a label string in `wire::Evidence.subject`
(`render.rs:101`; `wire/evidence.rs:157`) so a feature named `x` and a symbol path `x` are
indistinguishable in the `evidence` array; `Locator` becomes an untyped `JsonObject`
(`render.rs:92-96`; `wire/evidence.rs:164`) whose schema the generated `tool-data.schema.json`
cannot express; the legacy wire `Symbol` is filled with `None`/`0` for seven fields that no
longer have a column (`render.rs:22-45`). The `data` sections do carry typed `SubjectRef`,
`ApiPayload` and `FactSource` (`wire/data.rs:253-263`), so the loss is confined to the
`evidence` array and `InspectData.symbol`.

### Interruption or invalid input: a reordered struct reaches `project()`

`project()` (`native_delta.rs:100-146`) first requires `arrow::compute::can_cast_types` (`:111`)
and then passes a column through uncast when `actual.equals_datatype(declared)` holds (`:114`).
arrow-rs documents `equals_datatype` as comparing datatypes "ignoring nested field names and
metadata" (the method exists at 59.3 in `content/index/methods.tsv`; the doc text is quoted from
the essay because the skill's prose page carries no doc string). So a struct whose children
match the declared types but not the declared names passes the logical projection untouched,
and what happens next depends on which cast the `ArrowContract` planner's
`physical_expr::expressions::cast` (`arrow_contract.rs:105-124`) selects. At this pin two
candidates exist with different semantics: arrow-cast documents casts "to or from
`StructArray`" as unsupported (`api/arrow_cast.cast.md`), which would make the physical cast
fail loudly, while DataFusion 55.1's `datafusion_common::nested_struct::cast_column` matches
struct children **by name**, null-fills a missing target child and requires only one common
name (`api/datafusion_common.nested_struct.md`); `requires_nested_struct_cast` is documented as
the selector "at both planning time … and execution time". Which one `CastExpr` takes is
index-silent. Under the name-based path a *renamed* child (`{a, c}` against `{a, b}`) is
accepted, `b` becomes NULL, and because nested children are nullable in storage (S04) the row
is written without error; under a positional path values are swapped. Today every producer of
such a frame goes through `record()` (`expressions.rs:27-50`), which emits children in declared
order and refuses unknown names, so neither path is reached; it is an unguarded boundary, not
a live bug (S07).

## 6. Acceptance gates

| Gate | Verdict | Evidence or scope rationale | Required action |
|---|---|---|---|
| G1 — Authority | **Fail** | The variant→members mapping of every flattened tagged union is independently editable in three places (`relational.rs` enums; `checks.rs:343-401`; `decode.rs:76-232` and `relations.rs:74-93`, `execution.rs:158-173`) with no derivation and no equality test. Each evidence relation's shape is editable as a Rust struct and as an encoder with one fixture round trip as the only link. The wire `Evidence` re-declares subject and locator lossily. Reference targets live in role strings and separately in 19 SQL rules | S01, S06, S10 |
| G2 — Semantic fidelity | **Fail (narrow)** | Subject kind is dropped in `wire::Evidence.subject` (`render.rs:101`). One column-wide role (`coordinate-start`) covers a value whose unit changes by row (`encode.rs:326-345`); two line bases coexist without a type (`model.rs:151` vs `execution.rs:26-40`). Clocks are text in evidence relations and timestamps in control relations. None of these is a sentinel collision, and the `data` sections keep the typed forms, so the failure is narrow | S02, S03, S06 |
| G3 — Validity | **Pass, qualified** | Every candidate cohort passes schema-compiled admission before a control commit can select it; the control union has a Delta CHECK; the HTTP table has CHECK constraints. Qualification: nested required-ness is not a storage guarantee (S04) and a positional struct cast is reachable in principle (S07) | S04, S07 as strengthening |
| G4 — Hidden behavior | **Pass** | No inspection or planning path in scope mutates state or reads ambient input; the `equals_datatype` passthrough is declared in a comment (`native_delta.rs:101-105`) | — |
| G5 — Consistency and recovery | **Not applicable** | Publication and recovery were reviewed on 2026-09-15 and are outside this scope | — |
| G6 — Transformation and reuse | **Pass, qualified** | Contract identity includes role metadata, so any semantic change invalidates derived tables (DM-32 satisfied). Qualification: field metadata across a physical UNION is asserted, not tested (DESIGN.md §17; the DataFusion skill flags 55.1 as differing between logical and physical UNION on metadata), and S07 is a projection whose outcome for a same-typed, differently named struct depends on which cast runs rather than on the contract | S07; test in §9 |
| G7 — Truthful capability claims | **Pass** | The essay labels its performance claims as unmeasured; STATUS records the one measured projection number with its conditions; nothing in scope advertises a nested-pruning or nested-stats capability it does not have | Keep the index-silent items labeled Proposed |

## 7. Principle findings

Ordered by consequence. "Verification" names the oracle tier; where none exists that is stated.

| # | Finding | Principle IDs | Concrete evidence or gap | Consequence | Proposed correction | Verification |
|---|---|---|---|---|---|---|
| **S01 · P0** | Tagged-union shape has three hand-kept authorities, and two different encodings coexist | DM-02, DM-06, DM-17, DM-56 · G1 | Enum arms `relational.rs:18-45,70-85,89-153`, `execution.rs:119-137`; variant table `checks.rs:343-401`; allowlists `decode.rs:76-232`, `relations.rs:74-93`, `execution.rs:158-173`. Subject/target/locator flatten variants into shared nullable columns; execution payload (`execution.rs:493-556`), release metadata (`metadata.rs:33-333`), control records (`control.rs:1440-1454`) and command arguments (`operation.rs:139-146`) use one nullable struct per variant | Adding or changing a variant is four edits with no compile-time or test-time link; a variant table drift admits a malformed row (predicate too weak) or rejects a valid one (decode too strict). The shared-column encoding is also why `start` has a row-dependent unit (S03) | Choose the per-variant-struct encoding everywhere (`Struct<kind, lines: Struct<file?, start, end>?, bytes: Struct<start, end>?, …>`); derive the variant table once from the enum through schemars (the `enum_values` bridge at `checks.rs:13-35` already parses `oneOf`/`properties.kind`), and let encode, `tagged_checks`, decode and a generated Delta CHECK per union all read that one table (the `tag_rule()` generator at `control.rs:1456-1475` is the pattern). Surface area: `checks.rs`, `decode.rs`, the four encoders, both normalizers' `record()` call sites | **Test** (missing): assert the schemars-derived variant table equals `tagged_checks`'s table and every `variant(&[...])` list. **Rule** (missing): an `ast-grep` rule over `arrow_model/` forbidding a literal `variant(&[` allowlist outside the generated table |
| **S02 · P0** | Field semantics are role strings interpreted by prefix in four consumers | DM-06, DM-01, DM-04, DM-53 · G2 | `cells.rs:343-352` attaches `enrichment.role`; `checks.rs:257-282` (`strip_prefix("vocabulary:")`, `starts_with("key:")`, `== "sha256"`), `preparation.rs:466-477` (string equality), `native_catalog.rs:615` (display), `provider.rs:8-30` (rewrites the role); two spellings for one digest role (`acquisitions.rs:39` vs `provenance.rs:320`); Parquet-embedded field metadata is skipped on read by default (`datafusion.execution.parquet.skip_metadata` = true), so the IPC registry, not the file, is the carrier | A role string is data to five functions and a type to none; a misspelled role is silently unchecked; no consumer can tell `ref:symbol` from `key:symbol` at plan time; the meaning of "coordinate" is undiscoverable from the schema | Replace the free-form role with a small closed set of **application extension types** on `arrow_schema::extension::ExtensionType` (`enrichment.id{domain}`, `enrichment.ref{relation}`, `enrichment.vocabulary{name, version}`, `enrichment.digest{algorithm}`, `enrichment.coordinate{unit, base}`, `enrichment.ordered_set`), registered with `DFExtensionType` in a `MemoryExtensionTypeRegistry` on the session. Keep `enrichment.role` as the *display* label. Every consumer then calls `Field::try_extension_type::<T>()` instead of parsing. Delta still stores no metadata; the existing IPC registry restores it unchanged | **Test** (missing): every field in every relation schema carries a registered extension type or a documented plain role. **Rule** (missing): forbid `metadata().get("enrichment.role")` outside the extension-type module |
| **S03 · P1** | Clocks, digests and identities are text; coordinate units are row-dependent | DM-06, DM-08, DM-40 · G2 | Text clocks: `provenance.rs:196-206`, `acquisitions.rs:74-90`, `publication.rs:96`, `snapshot.rs:107`, `catalog.rs:204-206`; typed clocks: `control_jobs.rs:14-20`, `telemetry.rs:320-324`; text digests with regex checks `checks.rs:277-281`, `http_cache.rs:112`, byte loop `acquisitions.rs:128-139`; `prefix_` + hex identities `native_key.rs:262-266`; `locator.start` as line/byte/ordinal by variant `encode.rs:326-345` with a separate `u32` domain rule `checks.rs:517-534`; 1-based `model.rs:151` vs 0-based `execution.rs:26-40` | Temporal predicates need `try_cast` at every use site; digest well-formedness is re-stated in three notations; identities from different domains share one physical type; a coordinate's unit is not discoverable without reading `kind` | `Timestamp(µs, UTC)` for every clock (accepted by `storage_type` `native_delta.rs:181-185`); `FixedSizeBinary(32)` under `enrichment.digest` for digests (mapped to Delta `Binary` at `:174-176`); identities as digest + `enrichment.id{domain}` with the prefixed hex kept as a derived wire rendering; per-variant coordinate structs typed `UInt32` for lines and `UInt64` for bytes with `enrichment.coordinate{unit, base}`. `native_identity::type_bytes` needs two new tags (`FixedSizeBinary`, `Timestamp`) and the identity contract version bumps | **Test** (missing): a relation schema walk asserting no Utf8 field carries a clock or digest role. **Rule**: forbid `text(` with a `*-time`/`*-clock` role in `arrow_model/` |
| **S04 · P1** | Nested required-ness is enforced by admission, not by storage | DM-07, DM-22, DM-43 · G3 | `batch()` applies `native_read_field` before the schema reaches `StorageContract::new` (`cells.rs:370-395` → `admission.rs:69-84` → `delta_cohort.rs:12-19`), so the Delta `StructType` declares every nested child nullable; `enrichment.null=forbidden` is read only by `checks::required` (`checks.rs:114-120`). Top-level `NOT NULL` is Delta-enforced and Tested (`native-write-nullability-tests.log`); the kernel ships recursive `NonNullFieldChecker`/`InvariantChecker` schema transforms (`api/buoyant_kernel.transforms.schema.md:59-61`) whose use by the delta-rs writer on nested children is index-silent | Any native plan that writes a cohort directly (`publish_native`, both normalizers) can persist a NULL required nested child; it is unpublishable only because admission runs later. The storage layer permits what the ingress forbids, which is a second, weaker authority on the same fact | Give `StorageContract` a third schema: the **declared physical nullability** (semantic nullability, nested children `NOT NULL` where the contract says forbidden), used for `CreateBuilder::with_columns`, while the nullable read layout remains what `with_schema` presents. Index-silent whether the pinned writer enforces nested `NOT NULL`; if it does not, add a generated CHECK per required nested leaf (`payload IS NULL OR payload.declared_kind IS NOT NULL`) | **Probe** (§9): write a NULL nested child against a `NOT NULL` nested `StructField` through `WriteBuilder` and observe rejection. **Test**: extend `native_writer_checks_actual_nulls_in_conservatively_typed_plans` (`native_delta.rs:695-737`) to a nested field |
| **S05 · P1** | The role vocabulary is never checked at plan time; UDFs declare only datatypes | DM-22, DM-24, DM-26, DM-53 · G3, G6 | Five `ScalarUDFImpl`s implement `return_type` only (`native_identity.rs:73`; `native_url.rs:40`; `native_version.rs:32,126,215`); `Key::expression_for` binds inputs by position (`native_key.rs:230-266`); `preparation::result` checks only final output fields for declared families (`preparation.rs:445-495`); no `AnalyzerRule` is registered (`runtime.rs:639-641`) | A join `o.subject.definition_id = s.symbol_id` or a canonical-bytes call with arguments in the wrong order plans, runs and returns empty — and an empty result "is not proof of absence" (AGENTS.md), so the mistake reads as a fact. The essay's "once-per-plan analysis" is available and unused | Implement `return_field_from_args` on the five UDFs (reject an `arg_field` whose extension type is not the declared one; return a typed `FieldRef`); make `Key::bind` validate roles, not arity; add **one** `AnalyzerRule` that walks `BinaryExpr`/`Join` predicates and rejects equality between two `enrichment.id`/`enrichment.ref` fields of different domains unless wrapped in a declared conversion. Prefer `struct_field_mapping` on `url_parts_v1`/`pep440_value_v1` so ordering propagates through their struct outputs | **Test** (missing): a cross-domain join on admitted providers fails at `create_physical_plan`. **Rule** (missing): every `impl ScalarUDFImpl` in `crates/` must define `return_field_from_args` (an `ast-grep` `kind`-anchored rule) |
| **S06 · P1** | The wire re-declares evidence shapes lossily and by hand | DM-02, DM-42, DM-52 · G1, G2 | `wire::Evidence{subject: String, locator: JsonObject}` (`wire/evidence.rs:151-173`); `render.rs:85-114` builds it from the typed row; legacy `Symbol` filled with `None`/`0` (`render.rs:22-45`; `model.rs:127-166`); `result.rs:132-137` maps section aliases from JSON field names; `delivery.rs:129-175` clears and reconstructs fields; JSON Schemas are generated from the DTOs, not from the Arrow contract (`scripts/schemas-generate.sh:20-24`) | The calling agent cannot distinguish subject kinds in `evidence[]`; `locator` has no schema on the wire; a new nested field reaches MCP only after a DTO edit, a render edit and a regeneration | Derive the evidence-bearing JSON Schemas from the Arrow contract (Struct→object, List→array, `enrichment.vocabulary`→enum, `enrichment.ref`→string with a declared target, extension types → `format`), and serialize evidence rows from batches (`arrow::json::ArrayWriter` is already the decode bridge at `registry.rs:166-182`). Keep hand-authored DTOs for request, envelope and delivery. Replace `Evidence.subject: String` with `SubjectRef` and `locator: JsonObject` with the typed `Locator` (both already `JsonSchema`); delete the `Symbol` legacy fill | `just schema-conformance` extended with a fixture comparing the Arrow-derived and DTO-derived schemas for `SubjectRef`, `Locator`, `FactSource`, `ApiPayload` until the DTO copies are removed |
| **S07 · P1** | A struct whose children match by type but not by name passes the logical projection; the physical outcome depends on which cast runs | DM-24, DM-40, DM-53 · G6 | `native_delta.rs:111,114` (`can_cast_types`, then `equals_datatype` passthrough), `arrow_contract.rs:105-124` (physical `cast` to the declared type); arrow-cast documents struct casts as unsupported while DataFusion's `nested_struct::cast_column` is name-based with null-fill (`api/arrow_cast.cast.md`; `api/datafusion_common.nested_struct.md`) | A renamed child is either refused, null-filled and then persisted because nested storage is nullable (S04), or, under a positional cast, swapped; the contract does not decide which (see §5) | Replace the passthrough test with exact `Field` equality (`DataType::Struct` compares child names, types and nullability), and add an explicit name-keyed `CAST(... AS STRUCT(...))` where a reorder is intended | **Test** (missing): `project()` given `{b, a}` and `{a, c}` against declared `{a, b}` must refuse or reorder by name; the test also pins which cast `CastExpr` selects at this pin |
| **S08 · P2** | Parallel lists and bool+list encodings where the type expresses the relation | DM-06, DM-08, DM-10 | `python_normalize.rs:573-577` zips `texts` and `ordinals` by positional multi-column unnest; `execution_policy.rs:303-304` pairs two `array_agg(... ORDER BY ordinal)`; `coverage.rs:195-206` zips four independently decoded `Vec`s; `query.rs:384-386` zips two; `metadata.rs:80-91,336-340` and `native_key.rs:176-177` encode `Option<Vec>` as flag + list | Each site carries an alignment invariant the schema cannot state; the flag+list form needs a consistency check on every decode and an extra identity field | `List<Struct<ordinal, text>>` (one `array_agg(named_struct(...) ORDER BY ...)`, as `dependency_plan.rs:232` already does); decode `List<Struct>` rows with `Row::records` instead of parallel `strings()`; nullable `List` for `Option<Vec>` | **Rule** (missing): forbid `unnest_columns_with_options(&[a, b])` with two columns in `crates/`; **Test**: identity of `Environment{features: None}` versus `Some(vec![])` after the change |
| **S09 · P2** | No statistics or partitioning policy for nested tables | DM-36, DM-39 | No `dataSkippingStatsColumns`, no `num_indexed_cols`, no partition or Z-order; Bloom only on `observation_id` (`native_policy.rs:184-186`); cohort selection is a row filter on `cohort_id` (`delta_cohort.rs:87-99`); upstream's nested-struct statistics test is disabled pending delta-kernel-rs#1075 (`integration_datafusion.rs:763-775`) and DataFusion's `PruningStatistics` addresses columns by a flat `Column` name (`content/index/methods.tsv`) | With nested leaves counted toward the first-32-column default, statistics budget may be spent on locator leaves and not on keys, and nested-leaf statistics do not reach DataFusion's pruning at this pin; unmeasured either way | Set `DataSkippingStatsColumns` to the top-level keys, tags and `cohort_id` now (top-level columns are unaffected by the nested gap); treat partitioning by `cohort_id` and nested-path Z-order (`OptimizeType::ZOrder(["meta.field_a"])` is Tested upstream, `command_optimize.rs:2039-2140`) as hypotheses to measure, not to adopt | **Probe + benchmark** (§9); no rule |
| **S10 · P2** | Reference integrity rules are hand-written SQL beside `ref:` roles | DM-09, DM-16, DM-17 · G1 | 5 + 14 `SqlRule` literals (`admission.rs:261-285`; `control.rs:40-107`); `validated_constraints` derives only the primary key (`native_catalog.rs:30-48`); the `ref:` role triggers only an emptiness check (`checks.rs:267-276`) | A new reference field admits dangling references until someone writes its rule | With `enrichment.ref{relation}` (S02) generate the anti-join per reference field with the same disjunction builder as `checks.rs`; keep hand-written SQL only for rules that are not references (log presence, profile agreement) | **Test** (missing): every `enrichment.ref` field has a generated or listed rule |
| **S11 · P2** | Two decoding disciplines: typed `RowSet` and Arrow→JSON→serde | DM-41, DM-52 | `registry.rs:166-182`, `search_projection.rs:64-93`, `publication.rs:135-148`, `control_jobs.rs:108-119,367-370`, `repository.rs:574-577` route typed Arrow through JSON; `Command.submitted_at: String` (`control_jobs.rs:137`) beside a `Timestamp` column (`:33`) | Timestamps, `Decimal128`-mapped unsigned values and nested nullability pass through JSON's weaker type system; each family has a second representation | Fold into S06's generator: derive the serde ingress/egress from the contract, or keep the bridge and add the `operation.rs:385-399` `complete()` field-set test to every family that uses it | **Test**: the existing `complete()` pattern applied per family |
| **S12 · observation** | The essay's "nested I/O savings" is contradicted at this pin for large leaves | DM-39 | `tests/views.rs:25-28`; `native-column-projection-tests.log` (20,714 vs 1,018,844 bytes); `TableProvider::scan` projection is top-level | Nesting a large text leaf inside a frequently read struct costs a full-struct read | Design rule: nest small coherent values; keep large text leaves top-level; measure before nesting anything over a few hundred bytes per row | The existing test is the oracle; keep it |

**Applicability.** Groups 1, 2, 3, 5, 9 and 11 carry the findings (authority, semantic types,
identity, derivation contracts, boundaries, verification). Group 4 bears positively:
`checks.rs` is a declaration-compiled validator and should be kept. Group 8 bears through
S09/S12 only. Group 10 (provenance) is satisfied in scope: every evidence row carries a
`source` struct with producer binding, artifact and locator, and identity excludes attempt
clocks. Groups 6 and 7 (effects, concurrency) do not bear on a schema review and were reviewed
on 2026-09-15. Group 12: the proportionality question is answered in §8 — the registry, the
extension types and the generator each have five or more demonstrated consumers today.

**Principle verdicts (applicable groups only).** Satisfied: DM-01 (struct-valued domain
records), DM-03 (semantic/storage split), DM-07 (admission boundary), DM-14/DM-15 (contract
digest; canonical bytes), DM-18 (structs preserved through views until `surface_sql`),
DM-20, DM-23 (Delta providers are read-only views), DM-32 (contract in the key), DM-46. Violated:
DM-02 (S01, S06, S10), DM-06 (S02, S03), DM-24 (S07), DM-52 (S06). Unresolved: DM-51 (evolution
by epoch is chosen but not written as a procedure), DM-36/DM-39 (S09), DM-43 for nested
`NOT NULL` (S04 until probed).

## 8. Alternatives and architectural leverage

| Alternative | Semantic duplication and extension locality | Correctness and operational risks | Cost | Performance evidence | Why selected or rejected |
|---|---|---|---|---|---|
| **Current baseline** | Three copies per tagged union; two per relation; role strings parsed in four places; 19 SQL reference rules; lossy wire DTOs | Silent empty results for domain mistakes; storage permits nested NULLs; positional struct cast reachable | Already paid | One measured projection number | Rejected as the end state; preserved as the substrate |
| **Proposed: one contract layer** (S01, S02, S03, S05, S06) — per-variant unions generated from the enum, a closed extension-type set with a session registry, typed clocks/digests/ids, full-field UDFs plus one analyzer rule, wire schemas derived from the Arrow contract | One declaration per union, vocabulary and domain; a new variant is an enum arm plus a generated table entry; a new reference is one extension annotation | New machinery: extension-type registration, a schema-to-JSON-Schema emitter, one analyzer rule; each has ≥5 existing consumers. Risk: metadata propagation through UNION/join at 55.1 must be tested before relying on it at plan time | Medium: `checks.rs`, `decode.rs`, encoders, `native_key.rs`, `native_identity.rs`, `render.rs`, `emit-schemas.rs`, five UDF impls; an epoch change | None claimed; S12 is the only measured constraint | Selected: it removes the duplicated meaning the essay targets, using capability that exists at the pin |
| **Simpler viable alternative** — keep strings and hand-written codecs; add (a) a test that derives the variant table from schemars and asserts equality with `checks.rs` and every allowlist, (b) typed clocks and digests only, (c) `SubjectRef`/`Locator` on the wire `Evidence`, (d) exact-`Field` passthrough in `project()` | Duplication remains but is caught; wire loss fixed | Domain mistakes still plan and return empty (S05 unaddressed); roles still parsed by prefix | Low: four tests, three type changes, one comparison change | — | Rejected as the end state, **adopted as the first slice**: it is a strict subset of the proposal and removes G1/G2 failures on the wire and the union table before any new machinery |
| **Structured signatures** — `List<Struct<name, kind, annotation, default>>` + `returns` instead of `signature: Utf8`, `overloads: List<Utf8>` | Would move parameter semantics from text into the model; lets comparisons diff parameters and `verify_usage` check arguments natively | A product-model change; Griffe and rustdoc both have structured parameter data but different shapes; a new epistemic boundary (rendered text is what the producer said; a structured form is an interpretation) | High | — | Not selected here: no current MCP consumer asks parameter-level questions; recorded in §10 as the owner's decision |

**Abstractions justified by current needs.** The extension-type set (five consumers today),
the variant-table derivation (four consumers per union), the contract-to-JSON-Schema emitter
(three generated schemas and one Pydantic package already exist downstream). Not justified: a
general schema DSL, a second expression language, a generic "domain contract engine", Arrow
`Union` for stored relations (the kernel has no union type), `Dictionary` as a storage type
(stripped by `storage_type` at `native_delta.rs:177`), the kernel's Variant type
(`DataType::unshredded_variant` exists at this pin; a fixed schema has no use for it), and
column mapping or scan-time nested schema adaptation (`BatchAdapterFactory`) until an in-place
evolution requirement exists.

**What remains ordinary code.** Canonical byte framing, version precedence kernels, URL
parsing, the physical `ArrowContract` cast, the two producers' fact extractors, and the
non-reference SQL rules (log presence, profile agreement, artifact closure).

## 9. Verification and measurement plan

| Claim or risk | Evidence label today | Test / analysis / probe | Conditions and expected result | Current result or gap |
|---|---|---|---|---|
| Variant table agrees with the enum and the decode allowlists (S01) | Implemented, untested | Unit test deriving the table from `schemars::schema_for!` and comparing to `tagged_checks` and each `variant(&[...])` | Equality for subject, target, locator, execution target, execution payload, release metadata | Missing |
| Extension types survive the plan (S02, S05) | Proposed | DataFrame test: projection, alias, cast, UNION, join, aggregate; assert `extension_type_name()` on every output field | Metadata present after each operator at 55.1; document any operator that drops it | Missing; the skill flags UNION |
| Extension types survive Delta (S02) | Interface-checked | Write, reopen through `provider()`, read schema | Names and metadata restored by `with_schema` | One nested Parquet case tested (`tests/typed_arrow.rs:199-206`); Delta path untested for extension names |
| Nested `NOT NULL` enforced at write (S04) | **Index-silent** | Probe: `CreateBuilder` with a `NOT NULL` nested `StructField`; `WriteBuilder::with_input_plan` with a NULL child | Either rejection (adopt) or acceptance (generate CHECK constraints instead) | Missing |
| Nested statistics emitted and used (S09) | **Interface-checked as unsupported** for DataFusion statistics (upstream test disabled pending delta-kernel-rs#1075); kernel-side nested skipping index-silent | Probe: write a table with `DataSkippingStatsColumns = subject.kind, source.artifact_id`; inspect `add.stats`; `EXPLAIN` a filtered scan and count files | Stats present for nested leaves in the log; whether the file count drops with a selective nested predicate is the open question | Missing |
| Cross-domain join rejected at planning (S05) | Proposed | Test: `symbols.symbol_id = definitions.definition_id` under the analyzer rule | `create_physical_plan` fails with a typed error | Missing |
| Reordered or renamed struct through `project()` (S07) | Proposed | Test with `named_struct('b', …, 'a', …)` and `named_struct('a', …, 'c', …)` against `{a, b}` | Refusal or name-keyed reorder; never a silent null-fill or a positional swap; the test records which cast `CastExpr` selected | Missing |
| Wire schema derived from the contract equals the DTO schema (S06) | Proposed | Extend `just schema-conformance` with an Arrow-derived fixture for `SubjectRef`, `Locator`, `FactSource`, `ApiPayload` | Structural equality until the DTO copies are deleted | Missing |
| Typed clocks and digests round-trip (S03) | Proposed | Extend `full_unsigned_domain_survives_delta_and_read_view_rejects_dml` (`native_delta.rs:738-805`) to `Timestamp(µs, UTC)` and `FixedSizeBinary(32)` | Equality after Delta write/read; `verify_contract` accepts | `u64::MAX` case Tested; others missing |
| Large leaves stay top-level (S12) | **Measured** | `tests/views.rs:25-28` | Selected bytes ≪ full bytes | 20,714 vs 1,018,844 bytes, 2026-09-16 |
| Cohort partitioning or Z-order helps (S09) | Hypothesis | Benchmark file counts and bytes read for `cohort_id` selection at realistic cohort counts | Only adopt on a measured reduction | Missing |

**Cost accounting.** Construction: unchanged (encoders already build structs). Validation:
S01 removes one of three copies; S05 adds one analyzer pass per plan, replacing per-row regex
checks for digests and per-site `try_cast` for clocks. Storage: `FixedSizeBinary(32)` halves
digest bytes versus hex text; `Timestamp` is 8 bytes versus ~27. Inspection: extension names
make `native_catalog`'s inventory (`:590-660`) self-describing. Recovery: unchanged.

## 10. Exceptions and unresolved decisions

**Evolution by epoch, not migration.** Principle IDs: DM-51. Scope: every contract change
above. Reason: Plan 15 §1.1 forbids migration and `verify_contract` refuses a changed schema.
Consequence: S01–S03 together are one epoch change and should be sequenced as one. Compensating
control: the contract digest and the search-surface naming already make a stale table
unreachable. Owner: the project owner. Revisit trigger: the first requirement to read an old
epoch's evidence without re-acquisition; at that point Delta column mapping
(`TableProperty::ColumnMappingMode`, nested ids per protocol §column mapping) becomes the
alternative to compare.

**Structured signatures.** Principle IDs: DM-06, DM-58. Scope: `ApiPayload.signature`,
`overloads`, `bases`, `rust_signatures.text`. Reason not to act: no current consumer asks a
parameter-level question, and the structured form is an interpretation of producer text that
would need its own epistemic label. Revisit trigger: `compare_releases` or `verify_usage`
needing parameter granularity.

**Delta field metadata as a second carrier.** `StructField::with_metadata` could store the
extension name in the Delta schema itself, making tables self-describing without the IPC
registry. Not recommended now: it would create a second copy of the contract (property digest
plus per-field metadata) and whether `TryIntoArrow` restores it is index-silent. Revisit if an
external reader of the Delta tables appears.

**Recommendations that touch a recorded decision.** S02 and S05 add an extension-type registry
and an analyzer rule to the session; the 2026-09-15 capability review deferred custom
`AnalyzerRule`s "until a repeated plan-wide semantic check has a demonstrated consumer" — S05
is that consumer. S04 changes the physical Delta schema policy in Plan 15 §3.5. S06 changes
the source of generated wire schemas named in DESIGN.md §6.3 and ADR-0037. Each needs its
decision record before implementation; none changes a §B binding boundary.

## 11. Decision and implementation changes

**Decision: Revise.** The evidence model already realizes the essay's structural program; the
gates fail where meaning is still carried by convention and copied by hand. The correction is
to type the semantic layer with capability that exists at the pin and to derive the copies
from one declaration, sequenced as one contract epoch.

| Priority | Change | Principle IDs | Acceptance evidence | Regression protection |
|---|---|---|---|---|
| 1 (correctness) | First slice, no new machinery: variant-table equality test; `SubjectRef`/`Locator` on `wire::Evidence` and deletion of the legacy `Symbol` fill; exact-`Field` passthrough in `project()`; `Timestamp(µs, UTC)` and `FixedSizeBinary(32)` for clocks and digests | DM-02, DM-06, DM-24, DM-42 | Tests named in §9 rows 1, 7, 8, 9 pass; `just schema-conformance` passes | The new tests; a rule forbidding text clocks in `arrow_model/` |
| 2 (correctness) | Per-variant union encoding for subject, target, locator and execution target, generated from the enum, with a generated Delta CHECK per union | DM-02, DM-06, DM-17 | Round trip and admission tests for every variant; control-style CHECK receipts for evidence tables | The equality test from priority 1; a rule forbidding literal allowlists |
| 3 (semantic leverage) | Closed extension-type set with a session registry replacing role-string parsing; `enrichment.ref` generating reference rules | DM-06, DM-09, DM-16 | Every relation field typed or documented; generated anti-joins equal the hand-written set for existing references | A rule forbidding `metadata().get("enrichment.role")` outside the extension module |
| 4 (semantic leverage) | `return_field_from_args` on the five UDFs; role-aware `Key::bind`; one analyzer rule for cross-domain equality | DM-22, DM-24, DM-26 | Cross-domain join rejected at planning; metadata-propagation test across UNION/join | An `ast-grep` rule requiring `return_field_from_args` on every `impl ScalarUDFImpl` |
| 5 (semantic leverage) | Wire JSON Schemas for evidence sections derived from the Arrow contract; serialize rows from batches; retire the Arrow→JSON→serde bridges or guard each with the `complete()` pattern | DM-52, DM-41 | Conformance fixtures equal; Python contract tests pass | `just schema-conformance` |
| 6 (storage guarantee) | Declared physical nullability for nested children after the probe; nested `NOT NULL` or generated CHECK constraints | DM-07, DM-43 | Probe receipt; nested write-rejection test | The extended nullability test |
| 7 (measured) | `DataSkippingStatsColumns` on keys, refs, tags, `cohort_id` after the nested-stats probe; partitioning and Z-order only on a measured reduction | DM-36, DM-39 | Probe receipt; file-count and bytes-read benchmark | The benchmark, recorded with conditions |

**Final check.** The claims above match the evidence cited; the two index-silent facts are
labeled Proposed with probes; the supported scope excludes the publication protocol and
performance beyond one measurement; and later extensions — a new variant, a new reference, a
new vocabulary — each become one declaration plus generated artifacts once priorities 2–3 land.
