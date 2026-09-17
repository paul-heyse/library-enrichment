# Evidence 04 — Delta Lake: the storage contract, type model and table features

**Source:** the `deltalake` capability repository (13 crates, delta-rs + the Delta kernel at a
pinned commit), read 2026-09-16. Covers design goal **(3)**: contain persistence within a Delta
basis, and the part of goal (1) that storage constrains.

---

## 1. The Delta type model is narrower than Arrow — this constrains the schema

```rust
// buoyant_kernel::schema
enum DataType { Primitive, Array, Struct, Map, Variant }

enum PrimitiveType {
    String, Long, Integer, Short, Byte, Float, Double, Boolean, Binary,
    Date, Timestamp, TimestampNtz, TimestampNanos, TimestampNanosNtz,
    Void, IntervalYearMonth, IntervalDayTime, Decimal,
}

fn try_from_arrow(arrow_datatype: &ArrowDataType) -> Result<Self, ArrowError>
fn can_read_as(&self, read_type: &Self) -> Result<(), Error>      // on DataType
fn can_widen_to(&self, target: &Self) -> bool                     // on PrimitiveType
```

> ## ⚠ SUPERSEDED BY EXECUTED PROBE PB03 — see [evidence 10](10-pb01-pb03-executed-results.md)
>
> The table below was **derived** from the `PrimitiveType` enum and is **wrong**. That enum is
> the conversion's *target* vocabulary, not its accepted *source* vocabulary. Executed probe
> PB03 shows the kernel accepts ten of the eleven types listed here as absent, and **silently
> narrows** them instead: `UInt32`→`Int32`, `FixedSizeBinary(32)`→`Binary`,
> `Dictionary(Int32,Utf8)`→`Utf8`, `Utf8View`→`Utf8`, `LargeList`→`List`. Only `Float16` is
> rejected. The corrected rule is: **choose each Arrow type so it is a fixed point of the Delta
> conversion.** The table is kept below as the record of a derivation that did not survive
> measurement.

**This corrects the schema proposed in evidence 03.** Comparing against Arrow's 41 `DataType`
variants, the following are *not* Delta primitives and must not appear in a persisted schema:

| Arrow type evidence 03 wanted | Status in Delta | What to persist instead |
|---|---|---|
| `FixedSizeBinary(32)` for hashes | **absent** | `Binary`, or `String` holding hex |
| `Dictionary(Int32, Utf8)` for enums | **absent** | `String`; dictionary-encode in memory only |
| `UInt8/16/32/64` | **absent — Delta integers are signed only** | `Integer` / `Long` |
| `RunEndEncoded` | **absent** | ordinary columns; rely on Parquet encoding |
| `Utf8View` / `BinaryView` | **absent as distinct types** | `String` / `Binary` |
| `Float16` | **absent** | `Float` |
| `Decimal32` / `Decimal64` | only `Decimal` | `Decimal(p, s)` |
| `Union` | **absent** | a `Struct` of nullable arms, plus a discriminant |
| `List` / `Struct` / `Map` | **present** as `Array` / `Struct` / `Map` | unchanged — the nested design survives |

Two Delta types have no plain Arrow counterpart and are worth using:

- **`Variant`** (`unshredded_variant()`, `variant_type(fields)`) — semi-structured storage with an
  optional shredded layout. This is the right home for a retained native artifact whose shape the
  model deliberately does not interpret, replacing "a JSON string in a `String` column".
- **`TimestampNtz` / `TimestampNanos`** — explicit about timezone and resolution, which matters
  for ordering extraction runs.

`can_widen_to` and `can_read_as` are the protocol's own compatibility predicates. The plan uses
them as the schema-evolution gate rather than writing a comparison.

### The consequence for identity columns

The proposal's identity model wants an `EntityId` that cannot be confused with a string. Delta
cannot store a fixed-width binary or an Arrow extension type natively, so the design is:

```
storage (Delta)      String        -- stable, sortable, greppable in the log
in-memory (Arrow)    extension type codesearch.entity_id over Utf8
boundary             re-attach the extension type on read from field metadata
```

Field metadata is the carrier, and it *is* writable at the Delta level — see §3.

---

## 2. Table features: what the protocol will enforce for us

```rust
enum TableFeature {
    AppendOnly, Invariants, CheckConstraints, ChangeDataFeed, GeneratedColumns,
    IdentityColumns, InCommitTimestamp, RowTracking, DomainMetadata,
    IcebergCompatV1, IcebergCompatV2, IcebergCompatV3, ClusteredTable,
    MaterializePartitionColumns, AllowColumnDefaults, CatalogManaged, CatalogOwnedPreview,
    ColumnMapping, DeletionVectors, TimestampNanos, TimestampWithoutTimezone,
    TypeWidening, TypeWideningPreview, V2Checkpoint, VacuumProtocolCheck,
    VariantType, VariantTypePreview, VariantShredding, VariantShreddingPreview, Unknown,
}
```

The seven that carry design weight:

| Feature | What it buys this design |
|---|---|
| **`CheckConstraints`** | Declarative validation *in the table*. The proposal's invariants — an `Evidence` row must reference a real `ExtractionRun`, a `Derivation` must name a `rule_version` — become named CHECK expressions the writer enforces. This is goal (1)'s "remove discrete validation steps" in its purest form. |
| **`GeneratedColumns`** | Derived columns computed by the writer, not by a caller. Partition keys, bucketing keys and normalised sort keys stop being something an ingestion path must remember to populate. |
| **`RowTracking`** | Stable row IDs that survive file rewrites and `OPTIMIZE`. Directly serves the proposal's §8.1 identity model: a canonical entity keeps its row identity across compaction. |
| **`ChangeDataFeed`** | Read what changed between two versions. This is what makes re-derivation incremental — a `Derivation` can be re-run over only the rows whose inputs changed, instead of over the snapshot. |
| **`DomainMetadata`** | Namespaced metadata attached to the table itself. The proposal's §1 artifact-level manifest (source revisions, executable fingerprints, grammar revisions) lives here, versioned with the data. |
| **`DeletionVectors`** | Soft deletes, so `MERGE` and `DELETE` do not rewrite whole files. Matters because the merge passes touch many small updates. |
| **`ClusteredTable`** | Liquid clustering — multi-dimensional locality without committing to a partition hierarchy. Better than partitioning for high-cardinality keys like `entity_id`. |

`TypeWidening` allows schema evolution without rewriting data, which is what lets the canonical
schema grow as extraction families are added (proposal's staged implementation sequence).

`AllowColumnDefaults` pairs with DataFusion's `TableProvider::get_column_default` (evidence 01) —
the same concept on both sides of the seam.

**Caution:** several features are protocol-versioned and some are marked `Preview`
(`TypeWideningPreview`, `VariantTypePreview`, `VariantShreddingPreview`, `CatalogOwnedPreview`).
`MAX_VALID_READER_VERSION`, `MAX_VALID_WRITER_VERSION`, `TABLE_FEATURES_MIN_READER_VERSION` and
`TABLE_FEATURES_MIN_WRITER_VERSION` are constants in `table_features`; the plan must pin a
protocol version explicitly rather than accepting whatever the writer negotiates.

---

## 3. Column mapping and field metadata

```rust
enum ColumnMappingMode { None, Id, Name }
enum ColumnMappingOperation { /* whether an unsupported column-mapping access was a read or a write */ }
```

Column mapping decouples the logical column name from the physical Parquet name, which is what
makes renames cheap. It is a prerequisite for `TypeWidening` in practice and for keeping the
canonical schema legible while it evolves.

`deltalake_core::operations::update_field_metadata::UpdateFieldMetadataBuilder` exists as a
first-class operation, which confirms the evidence-03 plan: **per-field metadata is writable and
durable**, so the provenance annotations (which extraction family may write a column, which
proposal record type it belongs to) can live on the schema rather than in a side table.

---

## 4. Proposed physical layout

One Delta table per canonical record category, all under one DataFusion catalog (evidence 01 §1):

```
snapshot.artifact            partition: none          cluster: artifact_id
snapshot.extraction_run      partition: family        cluster: run_id
snapshot.entity              partition: kind          cluster: entity_id
snapshot.native_binding      partition: native_namespace  cluster: canonical_entity_id
snapshot.source_anchor       partition: none          cluster: artifact_id
program.*                    partition: record_kind   cluster: owner_id, entity_id
evidence.evidence            partition: observation_kind
evidence.derivation          partition: rule_id
evidence.behavior_assertion  partition: subject_kind
catalog.mechanism            small; no partitioning
catalog.capability           small; no partitioning
```

Rationale: partition on low-cardinality vocabulary columns (`kind`, `family`, `rule_id`) because
those are the columns every decision-packet query filters on; cluster on the high-cardinality
identity columns because those are the join keys. `ClusteredTable` is preferred over a second
partition level for `entity_id`.

## 5. Open questions for round 2

1. ~~Does Arrow extension-type metadata survive a Delta write/read cycle?~~ **ANSWERED — yes.**
   Probe PB01 (evidence 10): all four metadata keys round-tripped byte-for-byte, control
   included. No re-attachment step is needed.
2. Whether `CheckConstraints` expressions may reference other tables (almost certainly not), which
   decides whether referential invariants are constraints or scheduled queries.
3. Which of the `Preview` features the pinned delta-rs will actually write, and at which protocol
   versions.
4. Whether `Variant` is usable through delta-rs today or only present in the kernel schema model.
5. ~~`DeltaOps` — does it exist at this rev?~~ **ANSWERED — no.** Probe PB04 (evidence 10):
   both `deltalake::DeltaOps` and `deltalake::operations::DeltaOps` fail to resolve (E0432). The
   skill's index was correct rather than incomplete, and the individual operation builders are
   the only entry point — which the plan already assumes.
