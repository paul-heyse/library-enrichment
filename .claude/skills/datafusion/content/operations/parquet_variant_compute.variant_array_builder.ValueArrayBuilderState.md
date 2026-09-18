# `parquet_variant_compute::variant_array_builder::ValueArrayBuilderState`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant_compute.variant_array_builder.ValueArrayBuilderState.json).

<a id="op-30060a68ad22a394ea7915c7"></a>
## ValueArrayBuilderState

`struct` · `parquet_variant_compute::variant_array_builder::ValueArrayBuilderState` · parquet-variant-compute 59.3.0

```rust
struct ValueArrayBuilderState<'a>
```

Source: `src/variant_array_builder.rs:397`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Builder-specific state for array building that manages array-level offsets and nulls. See
[`VariantBuilderExt`] for details.
