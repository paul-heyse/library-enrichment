# Evidence 03 — Arrow type system, schema metadata, statistics and `Precision`

**Source:** the `datafusion` capability repository (the Arrow family is indexed alongside
DataFusion), read 2026-09-16. Covers design goal **(1)**: complex schemas, typed data, metadata
and predicates that remove discrete validation steps.

---

## 1. The type surface available for the canonical schema

```rust
enum DataType {
    Null, Boolean,
    Int8, Int16, Int32, Int64, UInt8, UInt16, UInt32, UInt64, Float16, Float32, Float64,
    Timestamp, Date32, Date64, Time32, Time64, Duration, Interval,
    Binary, FixedSizeBinary, LargeBinary, BinaryView,
    Utf8, LargeUtf8, Utf8View,
    List, ListView, FixedSizeList, LargeList, LargeListView,
    Struct, Union, Dictionary,
    Decimal32, Decimal64, Decimal128, Decimal256,
    Map, RunEndEncoded,
}
```

Choices this makes available, in the terms the proposal's §8.2 records need:

| Canonical record need | Arrow type | Why |
|---|---|---|
| `content_hash`, `entity_id` | `FixedSizeBinary(32)` under an extension type | fixed width, comparable, no per-row length prefix |
| `kind`, `origin`, `precision`, `mapping_status`, enum-like facets | `Dictionary(Int32, Utf8)` | low-cardinality vocabularies stored once; equality is an integer compare |
| `native_range`, `normalized_byte_range` | `Struct<start: UInt32, end: UInt32>` | one column, ordered comparison per field |
| ordered child records (generic args, projections, syntax children, `input_fact_ids`) | `List<Struct<…>>` | order is intrinsic to the type; no `ordinal` column and no side table |
| `assumptions`, free-form extractor detail | `Map<Utf8, Utf8>` | queryable with `map_keys` / `map_extract`, unlike a JSON string |
| `applicability_condition` on a `BehaviorAssertion` | `Struct` of typed facets | so it is a predicate, not prose |
| large source text held for audit | `Utf8View` / `BinaryView` | view types avoid copying on projection |
| `RunEndEncoded` | run-length columns such as `snapshot_id` repeated across millions of rows |

`RunEndEncoded` deserves a note: every row in a snapshot shares one `snapshot_id`, `run_id` is
constant within an extraction run, and `artifact_id` repeats per file. Run-end encoding makes
those columns nearly free while remaining ordinary typed columns to every query.

---

## 2. Arrow extension types — typed identity with validation

```rust
trait ExtensionType {
    // required
    fn try_new(data_type: &DataType, metadata: Self::Metadata) -> Result<Self, ArrowError>
    fn metadata(&self) -> &Self::Metadata
    fn serialize_metadata(&self) -> Option<String>
    fn deserialize_metadata(metadata: Option<&str>) -> Result<Self::Metadata, ArrowError>
    fn supports_data_type(&self, data_type: &DataType) -> Result<(), ArrowError>
    // provided
    fn try_new_from_field_metadata(data_type: &DataType, metadata: &HashMap<String, String>) -> Result<Self, ArrowError>
    fn validate(data_type: &DataType, metadata: Self::Metadata) -> Result<(), ArrowError>
}

enum CanonicalExtensionType {
    FixedShapeTensor, VariableShapeTensor, Json, Uuid, Opaque, Bool8, TimestampWithOffset,
}
```

**`supports_data_type` and `validate` are validation carried by the type itself.** That is goal
(1) in its strongest form: an `EntityId` column cannot be silently populated with the wrong
storage type, and the check is not a separate pass someone has to remember to run.

Registered into the session through `ExtensionTypeRegistry` (evidence 01 §4), extension types are
also visible to the planner, so a UDF can require `EntityId` rather than `Utf8` in its
`Signature` — which makes "this argument is an entity id, not a string that looks like one" a
type error at plan time.

Proposed extension types for the canonical model:

```
codesearch.entity_id       over FixedSizeBinary(32)
codesearch.content_hash    over FixedSizeBinary(32)
codesearch.mechanism_id    over Utf8         (stable, human-readable, per proposal §2)
codesearch.source_anchor   over Struct<artifact_id, start, end>
codesearch.native_handle   over Struct<run_id, namespace, handle>   -- run-scoped by construction
```

The last one enforces the proposal's rule directly: *"Native handles must be run-scoped. Rustdoc
IDs, HIR handles, compiler `DefId`s, and MIR local numbers do not share a namespace."* Making the
run part of the type means an unscoped handle cannot be represented at all.

`CanonicalExtensionType::Uuid` and `::Json` are available if a retained native artifact genuinely
has no better shape; `::Opaque` documents a payload the model deliberately does not interpret,
which is a better record than an untyped blob.

---

## 3. Schema and field metadata

```rust
// DFSchema — Arrow schema plus relation qualifiers
fn metadata(&self) -> &HashMap<String, String>
fn functional_dependencies(&self) -> &FunctionalDependencies
fn from_unqualified_fields(fields: Fields, metadata: HashMap<String, String>) -> Result<Self>
fn from_field_specific_qualified_schema(qualifiers: Vec<Option<TableReference>>, schema: &SchemaRef) -> Result<Self>
fn field_with_qualified_name(&self, qualifier: &TableReference, name: &str) -> Result<&FieldRef>
fn fields_with_qualified(&self, qualifier: &TableReference) -> Vec<&FieldRef>
fn index_of_column_by_name(&self, qualifier: Option<&TableReference>, name: &str) -> Option<usize>
// (+ unqualified variants, has_column_with_*, columns_with_unqualified_name)

// FieldMetadata — literal/field metadata, mergeable
fn new(inner: BTreeMap<String, String>) -> Self
fn new_from_field(field: &Field) -> Self
fn add_to_field(&self, field: Field) -> Field
fn add_to_field_ref(&self, field_ref: FieldRef) -> FieldRef
fn merge_options(m: Option<&FieldMetadata>, n: Option<&FieldMetadata>) -> Option<FieldMetadata>
fn extend(&mut self, other: Self)
fn inner(&self) -> &BTreeMap<String, String>
```

Metadata is `BTreeMap`, i.e. **ordered** — so serialised schema metadata is deterministic, which
matters for a build that must be byte-reproducible.

Design use: every column in the canonical schema carries field metadata naming the extraction
family that may write it, the proposal record type it belongs to, and its evidence requirement.
`merge_options` then gives a defined answer when a projection combines two provenances, instead
of one silently winning.

Schema-level metadata carries the snapshot manifest the proposal's §1 requires — source
revisions, executable fingerprints, grammar revisions — so the manifest travels *with* the data
rather than beside it.

---

## 4. `Precision` is the proposal's epistemic vocabulary, already implemented

```rust
enum Precision { Exact, Inexact, Absent }

// and it is arithmetic, not a label
fn add(…)  fn sub(…)  fn multiply(…)  fn min(…)  fn max(…)
fn add_for_sum(…)  fn cast_to(…)  fn cast_to_sum_type(…)
fn get_value(…)  fn is_exact(…)  fn map(…)
fn to_inexact(self) -> Self
fn with_estimated_selectivity(…)
```

The proposal (§1) asks every mechanism's support for a request to be classified as:

```
exact_under_stated_preconditions
candidate_generation_or_approximation
not_supported_through_this_surface
uncharacterized
```

Three of those four are structurally `Precision`:

| Proposal class | `Precision` | Meaning carried |
|---|---|---|
| `exact_under_stated_preconditions` | `Exact` | the answer is the answer |
| `candidate_generation_or_approximation` | `Inexact` | recall/precision not guaranteed |
| `uncharacterized` | `Absent` | nothing is claimed |
| `not_supported_through_this_surface` | — | a distinct fact; belongs in the contract, not the statistic |

**The important part is that `Precision` propagates.** `to_inexact` and the arithmetic mean that
composing a mechanism whose recall is `Exact` with one that is `Inexact` yields `Inexact` without
anyone writing that rule. The proposal's §10 prefilter condition —
`final_match(file) ⇒ prefilter_selects(file)`, and "when that implication is merely plausible,
mark the prefilter heuristic" — becomes a property the engine already maintains, provided the
plan models recall as a statistic rather than as prose.

This is also the reason the plan uses `Precision`-shaped columns for contract facets: the same
three-valued logic then appears in `TableProviderFilterPushDown`
(`Exact | Inexact | Unsupported`, evidence 01 §2) and in `ColumnStatistics`, so one vocabulary
runs from the contract catalog down to the scan.

---

## 5. Statistics

```rust
struct Statistics {
    fields: num_rows, total_byte_size, column_statistics
    fn add_column_statistics(self, column_stats: ColumnStatistics) -> Self
    fn calculate_total_byte_size(&mut self, schema: &Schema)
}

struct ColumnStatistics {
    fields: null_count, max_value, min_value, sum_value, distinct_count, byte_size
    fn with_min_value(self, min_value: Precision<ScalarValue>) -> Self
    fn with_max_value(…)  fn with_null_count(…)  fn with_distinct_count(…)
    fn with_sum_value(…)  fn with_byte_size(…)
    fn is_singleton(&self) -> bool
    fn to_inexact(self) -> Self
    fn new_unknown() -> Self
}
```

Every statistic is a `Precision<…>`, so "we do not know" is representable rather than defaulting
to a wrong zero. `new_unknown()` is the honest default the plan uses wherever an extraction
family cannot supply counts.

Chained through `StatisticsRegistry` (`compute`, `compute_base`, `register`,
`default_with_builtin_providers`, `with_providers`) which `SessionStateBuilder::with_statistics_registry`
installs — so a Delta-backed provider can answer row counts and min/max from the transaction log.

## 6. Open questions for round 2

1. ~~Which `DataType`s delta-rs can actually persist.~~ **ANSWERED by probe PB03** (evidence
   10). Almost all of them — and several narrow silently, which is the more dangerous outcome.
   The rule is now: choose each type so it is a **fixed point** of the Delta conversion.
2. ~~Whether Arrow extension-type metadata survives a Delta write/read cycle.~~ **ANSWERED —
   it survives** (probe PB01, evidence 10). Extension identity is a storage property; the
   re-attachment step this dossier proposed is unnecessary.
3. ~~Whether `Dictionary(Int32, Utf8)` survives a Delta round trip.~~ **ANSWERED — it is
   flattened to `Utf8`** (probe PB03, evidence 10), and accepted rather than rejected.
