# `arrow::util::bench_util::create_string_array_for_runs`

Full upstream contracts; raw type trees and source locators in [structured records](arrow.util.bench_util.create_string_array_for_runs.json).

<a id="op-8162071c0ce421a7197e0b7a"></a>
## create_string_array_for_runs

`function` · `arrow::util::bench_util::create_string_array_for_runs` · arrow 59.3.0

```rust
fn create_string_array_for_runs(physical_array_len: usize, logical_array_len: usize, string_len: usize) -> Vec<String>
```

Source: `src/util/bench_util.rs:630`. [Exact documentation build](https://docs.rs/crate/arrow/59.3.0/json).

Create string array to be used by run array builder. The string array
will result in run array with physical length of `physical_array_len`
and logical length of `logical_array_len`
