# `parquet_variant_compute::variant_array_builder::ArrayBuilderState`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant_compute.variant_array_builder.ArrayBuilderState.json).

<a id="op-dfbbb2a5b59044af2af127e3"></a>
## ArrayBuilderState

`struct` · `parquet_variant_compute::variant_array_builder::ArrayBuilderState` · parquet-variant-compute 59.3.0

```rust
struct ArrayBuilderState<'a>
```

Source: `src/variant_array_builder.rs:210`. [Exact documentation build](https://docs.rs/crate/parquet-variant-compute/59.3.0/json).

Builder-specific state for array building that manages array-level offsets and nulls. See
[`VariantBuilderExt`] for details.
