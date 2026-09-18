# `deltalake_catalog_unity::models::ListTableSummariesResponse::Success`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_unity.models.ListTableSummariesResponse.Success.json).

<a id="op-848111a1c9c11cd4eb768696"></a>
## next_page_token

`struct_field` · `deltalake_catalog_unity::models::ListTableSummariesResponse::Success::next_page_token` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
next_page_token: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L117).

Source: `crates/catalog-unity/src/models.rs:117`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Continuation token

<a id="op-423fd1b094d4b31e2d228bf4"></a>
## tables

`struct_field` · `deltalake_catalog_unity::models::ListTableSummariesResponse::Success::tables` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
tables: Vec<TableSummary>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L115).

Source: `crates/catalog-unity/src/models.rs:115`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Basic table infos
