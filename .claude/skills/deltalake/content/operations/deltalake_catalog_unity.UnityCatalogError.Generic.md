# `deltalake_catalog_unity::UnityCatalogError::Generic`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_unity.UnityCatalogError.Generic.json).

<a id="op-40ddd6609974aa8a0e69de03"></a>
## source

`struct_field` · `deltalake_catalog_unity::UnityCatalogError::Generic::source` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
source: Box<dyn std::error::Error + Send + Sync + 'static>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L132).

Source: `crates/catalog-unity/src/lib.rs:132`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error message
