# `datafusion::test::table_with_sequence`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.test.table_with_sequence.json).

<a id="op-4e8a9dab8abb9076c0adc2ca"></a>
## table_with_sequence

`function` · `datafusion::test::table_with_sequence` · datafusion 55.1.0

```rust
fn table_with_sequence(seq_start: i32, seq_end: i32) -> error::Result<std::sync::Arc<dyn TableProvider>>
```

Source: `src/test/mod.rs:221`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return a new table provider that has a single Int32 column with
values between `seq_start` and `seq_end`
