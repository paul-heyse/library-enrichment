# `deltalake_catalog_unity::UnityCatalogError::RequestError`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_unity.UnityCatalogError.RequestError.json).

<a id="op-6ec9480df597cb17a9caaa66"></a>
## source

`struct_field` · `deltalake_catalog_unity::UnityCatalogError::RequestError::source` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
source: reqwest::Error
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/lib.rs#L62).

Source: `crates/catalog-unity/src/lib.rs:62`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The underlying reqwest_middleware::Error
