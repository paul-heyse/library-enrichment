# `datafusion_spark::function::string::concat_ws`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.concat_ws.json).

<a id="op-7c49e66ed767046dc5a4715d"></a>
## concat_ws

`module` · `datafusion_spark::function::string::concat_ws` · datafusion-spark 55.1.0

```rust
mod concat_ws
```

Source: `src/function/string/concat_ws.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Spark-compatible `concat_ws`: joins strings (and array elements) with a separator.

Null scalar args and null array elements are skipped; a null separator yields a
null row. Non-string args are coerced to STRING; list args (`List`, `LargeList`,
`ListView`, `LargeListView`, `FixedSizeList`) expand their elements.

Differences with DataFusion core `concat_ws`:
- Accepts list arguments and expands their elements
- Always returns Utf8 (Spark's `STRING` type)
- Coerces non-string scalars (numbers, booleans, dates, ...) to Utf8
