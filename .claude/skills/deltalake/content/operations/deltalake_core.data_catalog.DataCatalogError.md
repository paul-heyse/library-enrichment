# `deltalake_core::data_catalog::DataCatalogError`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.data_catalog.DataCatalogError.json).

<a id="op-4fe4aaefb52ab80fd7698e8c"></a>
## DataCatalogError

`enum` · `deltalake_core::data_catalog::DataCatalogError` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum DataCatalogError
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/data_catalog/mod.rs#L13).

Source: `crates/core/src/data_catalog/mod.rs:13`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error enum that represents a CatalogError.

<a id="op-d5d3c04a49f76499c1160463"></a>
## Generic

`variant` · `deltalake_core::data_catalog::DataCatalogError::Generic` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Generic
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/data_catalog/mod.rs#L16).

Source: `crates/core/src/data_catalog/mod.rs:16`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A generic error qualified in the message

<a id="op-465bed0f0fbdb85dbb39fd22"></a>
## InvalidDataCatalog

`variant` · `deltalake_core::data_catalog::DataCatalogError::InvalidDataCatalog` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
InvalidDataCatalog
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/data_catalog/mod.rs#L25).

Source: `crates/core/src/data_catalog/mod.rs:25`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Error representing an invalid Data Catalog.

<a id="op-ed91272eabc852bd67e61024"></a>
## RequestError

`variant` · `deltalake_core::data_catalog::DataCatalogError::RequestError` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
RequestError
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/data_catalog/mod.rs#L42).

Source: `crates/core/src/data_catalog/mod.rs:42`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A request to the underlying catalog service failed.

<a id="op-7690b1dd17b2c308b1492dd7"></a>
## UnknownConfigKey

`variant` · `deltalake_core::data_catalog::DataCatalogError::UnknownConfigKey` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
UnknownConfigKey
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/data_catalog/mod.rs#L32).

Source: `crates/core/src/data_catalog/mod.rs:32`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Unknown configuration key

<a id="op-30f3caebe6c9516e8cb4b29a"></a>
## fmt

`function` · `deltalake_core::data_catalog::DataCatalogError::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/data_catalog/mod.rs#L12).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::data_catalog::DataCatalogError", "path": "DataCatalogError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12, 28], "end": [12, 33], "filename": "crates/core/src/data_catalog/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/data_catalog/mod.rs:12`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9412c28203d51401900602f"></a>
## fmt

`function` · `deltalake_core::data_catalog::DataCatalogError::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/data_catalog/mod.rs#L12).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::data_catalog::DataCatalogError", "path": "DataCatalogError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12, 10], "end": [12, 26], "filename": "crates/core/src/data_catalog/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `crates/core/src/data_catalog/mod.rs:12`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-518089e02c4316a64a65d919"></a>
## source

`function` · `deltalake_core::data_catalog::DataCatalogError::source` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private20::Error + 'static>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/data_catalog/mod.rs#L12).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::data_catalog::DataCatalogError", "path": "DataCatalogError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [12, 10], "end": [12, 26], "filename": "crates/core/src/data_catalog/mod.rs"}, "trait": {"args": null, "id": "core::error::Error", "path": "Error"}, "trait_path": "core::error::Error"}`

Source: `crates/core/src/data_catalog/mod.rs:12`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
