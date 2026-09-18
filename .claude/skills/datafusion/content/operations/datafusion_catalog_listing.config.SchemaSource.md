# `datafusion_catalog_listing::config::SchemaSource`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog_listing.config.SchemaSource.json).

<a id="op-f5706baca32abfe2542eddab"></a>
## SchemaSource

`enum` · `datafusion_catalog_listing::config::SchemaSource` · datafusion-catalog-listing 55.1.0

```rust
enum SchemaSource
```

Source: `src/config.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Indicates the source of the schema for a [`crate::ListingTable`](../operations/datafusion_catalog_listing.table.ListingTable.md#op-00feae5c1ac67d3f887e695a)

<a id="op-769bbb8a20877012d956f0cc"></a>
## Inferred

`variant` · `datafusion_catalog_listing::config::SchemaSource::Inferred` · datafusion-catalog-listing 55.1.0

```rust
Inferred
```

Source: `src/config.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Schema was inferred from first table_path

<a id="op-5c22d3cc6801bf0571ef030c"></a>
## Specified

`variant` · `datafusion_catalog_listing::config::SchemaSource::Specified` · datafusion-catalog-listing 55.1.0

```rust
Specified
```

Source: `src/config.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Schema was specified explicitly via with_schema

<a id="op-365c35824fc26d48863322aa"></a>
## Unset

`variant` · `datafusion_catalog_listing::config::SchemaSource::Unset` · datafusion-catalog-listing 55.1.0

```rust
Unset
```

Source: `src/config.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

Schema is not yet set (initial state)

<a id="op-ac46b74284c9cb3f0951d4fe"></a>
## clone

`function` · `datafusion_catalog_listing::config::SchemaSource::clone` · datafusion-catalog-listing 55.1.0

```rust
fn clone(&self) -> SchemaSource
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::config::SchemaSource", "path": "SchemaSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 17], "end": [32, 22], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/config.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd9a408226278c4646921adf"></a>
## default

`function` · `datafusion_catalog_listing::config::SchemaSource::default` · datafusion-catalog-listing 55.1.0

```rust
fn default() -> SchemaSource
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::config::SchemaSource", "path": "SchemaSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 41], "end": [32, 48], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/config.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99d0ca90bf415118be1186b3"></a>
## eq

`function` · `datafusion_catalog_listing::config::SchemaSource::eq` · datafusion-catalog-listing 55.1.0

```rust
fn eq(&self, other: &SchemaSource) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::config::SchemaSource", "path": "SchemaSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 30], "end": [32, 39], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/config.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20fe21aabcff3ec9fdae5ab5"></a>
## fmt

`function` · `datafusion_catalog_listing::config::SchemaSource::fmt` · datafusion-catalog-listing 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog_listing::config::SchemaSource", "path": "SchemaSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 10], "end": [32, 15], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/config.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog-listing/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
