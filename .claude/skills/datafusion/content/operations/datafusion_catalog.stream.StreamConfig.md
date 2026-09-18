# `datafusion_catalog::stream::StreamConfig`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog.stream.StreamConfig.json).

<a id="op-5a9c0db207927628f8881b75"></a>
## StreamConfig

`struct` · `datafusion_catalog::stream::StreamConfig` · datafusion-catalog 55.1.0

```rust
struct StreamConfig
```

Source: `src/stream.rs:254`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

The configuration for a [`StreamTable`](../operations/datafusion_catalog.stream.StreamTable.md#op-4f9e242f2c18b302a51a1b01)

<a id="op-32b216702904545972e3538c"></a>
## fmt

`function` · `datafusion_catalog::stream::StreamConfig::fmt` · datafusion-catalog 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::stream::StreamConfig", "path": "StreamConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [253, 10], "end": [253, 15], "filename": "src/stream.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/stream.rs:253`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e48559be7a7431a690a796ac"></a>
## new

`function` · `datafusion_catalog::stream::StreamConfig::new` · datafusion-catalog 55.1.0

```rust
fn new(source: Arc<dyn StreamProvider>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::stream::StreamConfig", "path": "StreamConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [260, 1], "end": [289, 2], "filename": "src/stream.rs"}, "trait": null, "trait_path": null}`

Source: `src/stream.rs:262`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Create a new `StreamConfig` from a `StreamProvider`

<a id="op-cc24e4d8cae7c1b8a45fece2"></a>
## with_constraints

`function` · `datafusion_catalog::stream::StreamConfig::with_constraints` · datafusion-catalog 55.1.0

```rust
fn with_constraints(self, constraints: Constraints) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::stream::StreamConfig", "path": "StreamConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [260, 1], "end": [289, 2], "filename": "src/stream.rs"}, "trait": null, "trait_path": null}`

Source: `src/stream.rs:277`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Assign constraints

<a id="op-125c82eae35ef9bfbba8bcd4"></a>
## with_order

`function` · `datafusion_catalog::stream::StreamConfig::with_order` · datafusion-catalog 55.1.0

```rust
fn with_order(self, order: Vec<Vec<SortExpr>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::stream::StreamConfig", "path": "StreamConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [260, 1], "end": [289, 2], "filename": "src/stream.rs"}, "trait": null, "trait_path": null}`

Source: `src/stream.rs:271`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Specify a sort order for the stream
