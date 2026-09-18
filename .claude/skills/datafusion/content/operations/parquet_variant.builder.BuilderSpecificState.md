# `parquet_variant::builder::BuilderSpecificState`

Full upstream contracts; raw type trees and source locators in [structured records](parquet_variant.builder.BuilderSpecificState.json).

<a id="op-edb0d41012c518a778fdb47d"></a>
## BuilderSpecificState

`trait` · `parquet_variant::builder::BuilderSpecificState` · parquet-variant 59.3.0

```rust
trait BuilderSpecificState: std::fmt::Debug
```

Source: `src/builder.rs:383`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

A trait for managing state specific to different builder types.

<a id="op-e322daafcfe5797d960e1f3d"></a>
## finish

`function` · `parquet_variant::builder::BuilderSpecificState::finish` · parquet-variant 59.3.0

```rust
fn finish(&mut self, _metadata_builder: &mut dyn MetadataBuilder, _value_builder: &mut ValueBuilder)
```

Source: `src/builder.rs:391`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Called by [`ParentState::finish`](../operations/parquet_variant.builder.ParentState.md#op-9d5239e461de79715bbbe618) to apply any pending builder-specific changes.

The provided implementation does nothing by default.

Parameters:
- `metadata_builder`: The metadata builder that was used
- `value_builder`: The value builder that was used

<a id="op-46b4f057e444d669499ecd3f"></a>
## rollback

`function` · `parquet_variant::builder::BuilderSpecificState::rollback` · parquet-variant 59.3.0

```rust
fn rollback(&mut self)
```

Source: `src/builder.rs:405`. [Exact documentation build](https://docs.rs/crate/parquet-variant/59.3.0/json).

Called by [`ParentState::drop`](../operations/parquet_variant.builder.ParentState.md#op-5c7f5f4bd7f6b26be1856bab) to revert any changes that were eagerly applied, if
[`ParentState::finish`](../operations/parquet_variant.builder.ParentState.md#op-9d5239e461de79715bbbe618) was never invoked.

The provided implementation does nothing by default.

The base [`ParentState`](../operations/parquet_variant.builder.ParentState.md#op-3c8a64214e3c1dca4791d5f0) will handle rolling back the value and metadata builders,
but builder-specific state may need to revert its own changes.
