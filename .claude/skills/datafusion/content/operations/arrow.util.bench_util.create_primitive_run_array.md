# `arrow::util::bench_util::create_primitive_run_array`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.bench_util.create_primitive_run_array.json).

<a id="op-d90ef989d8136870efedd655"></a>
## create_primitive_run_array

`function` · `arrow::util::bench_util::create_primitive_run_array` · arrow 59.3.0

```rust
fn create_primitive_run_array<R: RunEndIndexType, V: ArrowPrimitiveType>(logical_array_len: usize, physical_array_len: usize) -> RunArray<R>
```

Source: `src/util/bench_util.rs:596`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Create primitive run array for given logical and physical array lengths
