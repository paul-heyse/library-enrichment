# `buoyant_kernel::engine::arrow_utils::parse_json`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.arrow_utils.parse_json.json).

<a id="op-11224bb8895392a8751f2e64"></a>
## parse_json

`function` · `buoyant_kernel::engine::arrow_utils::parse_json` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn parse_json(json_strings: Box<dyn EngineData>, schema: schema::SchemaRef) -> DeltaResult<Box<dyn EngineData>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_utils/mod.rs#L1157).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_utils/mod.rs:1157`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Parse a column of JSON strings into a typed `RecordBatch` matching `schema`. N input
rows produce N output rows.

Arrow lacks the functionality to json-parse a string column into a struct column, so we
implement it here.

Failure-prone primitive leaves (`Timestamp`, `TimestampNtz`, `Date`, `Decimal`) produce
per-cell NULL when the typed decoder rejects a value (extended-year timestamps,
decimals that overflow the declared precision, etc.). Other leaf type mismatches still
surface as batch-level errors.
