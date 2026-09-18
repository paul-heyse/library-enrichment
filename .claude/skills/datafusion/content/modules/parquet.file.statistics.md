# `parquet::file::statistics`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.statistics.json).

<a id="op-50c431aef6bd8f8fb6fb3a3a"></a>
## statistics

`module` · `parquet::file::statistics` · parquet 59.3.0

```rust
mod statistics
```

Source: `src/file/statistics.rs:18`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Contains definitions for working with Parquet statistics.

Though some common methods are available on enum, use pattern match to extract
actual min and max values from statistics, see below:

# Examples
```rust
use parquet::file::statistics::Statistics;

let stats = Statistics::int32(Some(1), Some(10), None, Some(3), true);
assert_eq!(stats.null_count_opt(), Some(3));
assert!(stats.is_min_max_deprecated());
assert!(stats.min_is_exact());
assert!(stats.max_is_exact());

match stats {
    Statistics::Int32(ref typed) => {
        assert_eq!(typed.min_opt(), Some(&1));
        assert_eq!(typed.max_opt(), Some(&10));
    }
    _ => {}
}
```
