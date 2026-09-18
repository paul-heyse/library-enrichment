# `arrow_array::types::RunEndIndexType`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.types.RunEndIndexType.json).

<a id="op-b20dc51c212ffcdcff0708b7"></a>
## RunEndIndexType

`trait` · `arrow_array::types::RunEndIndexType` · arrow-array 59.3.0

```rust
trait RunEndIndexType: ArrowPrimitiveType
```

Source: `src/types.rs:290`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A subtype of primitive type that is used as run-ends index
in `RunArray`.
See <https://arrow.apache.org/docs/format/Columnar.html>
