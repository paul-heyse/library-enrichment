# `arrow_array::builder::generic_byte_run_builder::LargeStringRunBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.builder.generic_byte_run_builder.LargeStringRunBuilder.json).

<a id="op-1befa0641a65db2196e98d57"></a>
## LargeStringRunBuilder

`type_alias` · `arrow_array::builder::generic_byte_run_builder::LargeStringRunBuilder` · arrow-array 59.3.0

```rust
type LargeStringRunBuilder<K> = GenericByteRunBuilder<K, types::LargeUtf8Type>
```

Source: `src/builder/generic_byte_run_builder.rs:336`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Builder for [`RunArray`](../operations/arrow_array.array.run_array.RunArray.md#op-f0742b2d9e045922a076478f) of [`LargeStringArray`](crate::array::LargeStringArray)
