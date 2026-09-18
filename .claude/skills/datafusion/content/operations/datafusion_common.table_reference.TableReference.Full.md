# `datafusion_common::table_reference::TableReference::Full`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.table_reference.TableReference.Full.json).

<a id="op-368fc8cb3e96bb99678b4993"></a>
## catalog

`struct_field` · `datafusion_common::table_reference::TableReference::Full::catalog` · datafusion-common 55.1.0

```rust
catalog: std::sync::Arc<str>
```

Source: `src/table_reference.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The catalog (aka database) containing the table

<a id="op-0a8b35576833dec4d719131e"></a>
## schema

`struct_field` · `datafusion_common::table_reference::TableReference::Full::schema` · datafusion-common 55.1.0

```rust
schema: std::sync::Arc<str>
```

Source: `src/table_reference.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The schema containing the table

<a id="op-aaaa22c2a49afeb9a278af2f"></a>
## table

`struct_field` · `datafusion_common::table_reference::TableReference::Full::table` · datafusion-common 55.1.0

```rust
table: std::sync::Arc<str>
```

Source: `src/table_reference.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The table name
