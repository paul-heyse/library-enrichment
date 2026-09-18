# `deltalake_core::errors::DeltaTableError::Arrow`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.errors.DeltaTableError.Arrow.json).

<a id="op-d6d7e6a23198ca8c5a832d41"></a>
## source

`struct_field` · `deltalake_core::errors::DeltaTableError::Arrow::source` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
source: arrow::error::ArrowError
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L57).

Source: `crates/core/src/errors.rs:57`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Arrow error details returned when converting the schema in Arrow format failed
