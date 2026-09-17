# PB03 companion — the conversion mechanism, read from pinned source

Read from the cached kernel checkout at the rev delta-rs pins:
`~/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/393fbf6/kernel/src/engine/arrow_conversion/mod.rs`

This is the *mechanism*; the executed probe (`delta-arrow-probe`) is the *fact*. They are recorded
separately because a mechanism read from source can be misread, and a fact without a mechanism
cannot be reasoned about.

---

## The finding: the kernel maps, it does not reject

`impl TryFromArrow<&ArrowDataType> for DataType` (line 548) is a match with a wide accepting
surface and a narrow `s => Err(...)` fallthrough. **Evidence 04 §1 derived a rejection list from
the `PrimitiveType` enum. That derivation was wrong** — the enum is the *target* vocabulary, not
the accepted *source* vocabulary.

### Accepted, with the mapping applied

| Arrow type | Kernel type | Lossy? |
|---|---|---|
| `Utf8`, `LargeUtf8`, **`Utf8View`** | `STRING` | no |
| `Binary`, `LargeBinary`, **`BinaryView`**, **`FixedSizeBinary(_)`** | `BINARY` | **yes — the fixed width is discarded** |
| `Int64` | `LONG` | no |
| **`UInt64`** | `LONG` (i64) | **yes — values above `i64::MAX` cannot round-trip** |
| `Int32` | `INTEGER` | no |
| **`UInt32`** | `INTEGER` (i32) | **yes — values above `i32::MAX` cannot round-trip** |
| `Int16` / `UInt16` | `SHORT` | yes for UInt16 |
| `Int8` / `UInt8` | `BYTE` | yes for UInt8 |
| `Null` | `VOID` | no |
| `Float32` / `Float64` | `FLOAT` / `DOUBLE` | no |
| `Boolean` | `BOOLEAN` | no |
| `Decimal128(p, s)` | `Decimal(p, s)` | **negative scale is an error** |
| `Date32`, `Date64` | `DATE` | yes for Date64 |
| `Timestamp(µs, None)` | `TIMESTAMP_NTZ` | no |
| `Timestamp(µs, Some("utc"))` | `TIMESTAMP` | no |
| **`Timestamp(ns, None)`** | `TIMESTAMP_NTZ` | **yes — nanoseconds silently truncated to micros** |
| `Struct` | struct | no |
| `List`, `ListView`, `LargeList`, `LargeListView`, **`FixedSizeList`** | `ArrayType` | the fixed size is discarded |
| `Map` | `MapType` | no |
| **`Dictionary(_, value)`** | the **value type** | the encoding is discarded |

The `Dictionary` arm carries its own comment:

> *"Dictionary types are just an optimized in-memory representation of an array. Schema-wise, they
> are the same as the value type."*

### Rejected — the fallthrough only

```rust
s => Err(ArrowError::SchemaError(format!("Invalid data type for Delta Lake: {s}")))
```

Reaching it: `Float16`, `RunEndEncoded`, `Union`, `Decimal256`, `Decimal32`, `Decimal64`,
`Interval`, `Time32`, `Time64`, `Duration`, and any `Timestamp` whose unit is not microsecond or
nanosecond, or whose timezone is not UTC.

---

## Why this changes the design more than a rejection list would

A rejection is loud: the write fails and you pick another type. **A silent lossy mapping is the
dangerous case**, and it is the one that actually happens here:

1. **`UInt32` → `INTEGER` (i32).** The plan wanted `UInt32` for byte ranges in `SourceAnchor`.
   A file larger than 2 GiB produces offsets above `i32::MAX`. Nothing raises — it is a schema
   conversion, not a value check. **Use `Int64` for all offsets.**
2. **`FixedSizeBinary(32)` → `BINARY`.** The width that made it a hash is gone. Any "this column
   is a 32-byte digest" guarantee has to be re-imposed, which is what the extension type and a
   `CheckConstraints` length predicate are for.
3. **`Dictionary(Int32, Utf8)` → `STRING`.** Harmless — the design already treated dictionary
   encoding as in-memory only (evidence 04 §1) — but for the *opposite reason* to the one
   recorded. It is not rejected; it is flattened.
4. **`Timestamp(ns)` → micros.** Extraction-run ordering at nanosecond resolution is not
   preserved. Use microseconds explicitly rather than discovering the truncation later.

## Correction required

Evidence 04 §1's table must be replaced. The corrected rule is:

> Delta accepts almost every Arrow type the plan wanted, and **silently narrows several of them**.
> The schema must be chosen so that the Arrow type and its Delta image are the *same* type —
> otherwise what is read back is not what was written, and nothing reported it.

## Metadata (PB01's mechanism, same file)

Both directions preserve arbitrary field metadata. The write path
(`kernel_flat_parquet_id_to_arrow_metadata`, line 45) documents itself:

> *"Translate a kernel `StructField`'s flat (non-nested) parquet field id metadata into Arrow
> field metadata: rewrites kernel-side `"parquet.field.id"` to arrow-side `"PARQUET:field_id"`.
> **All other entries pass through unchanged.**"*

and the read path (line 438) maps every Arrow metadata entry into `MetadataValue`, special-casing
only the two parquet-field-id spellings.

**Prediction: `ARROW:extension:name` and `ARROW:extension:metadata` round-trip.** The executed
probe tests it, with an ordinary metadata key as the control.
