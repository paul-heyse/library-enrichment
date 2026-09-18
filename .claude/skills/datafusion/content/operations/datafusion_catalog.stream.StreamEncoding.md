# `datafusion_catalog::stream::StreamEncoding`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_catalog.stream.StreamEncoding.json).

<a id="op-3972d33b1c69144158ddc093"></a>
## StreamEncoding

`enum` · `datafusion_catalog::stream::StreamEncoding` · datafusion-catalog 55.1.0

```rust
enum StreamEncoding
```

Source: `src/stream.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

The data encoding for [`StreamTable`](../operations/datafusion_catalog.stream.StreamTable.md#op-4f9e242f2c18b302a51a1b01)

<a id="op-4887e84e26047f5b1855bdb7"></a>
## Csv

`variant` · `datafusion_catalog::stream::StreamEncoding::Csv` · datafusion-catalog 55.1.0

```rust
Csv
```

Source: `src/stream.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

CSV records

<a id="op-46d018cdee73cbe88e4df324"></a>
## Err

`assoc_type` · `datafusion_catalog::stream::StreamEncoding::Err` · datafusion-catalog 55.1.0

```rust
Err
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::stream::StreamEncoding", "path": "StreamEncoding"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [111, 2], "filename": "src/stream.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/stream.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e86e23f5c508257a725326a"></a>
## Json

`variant` · `datafusion_catalog::stream::StreamEncoding::Json` · datafusion-catalog 55.1.0

```rust
Json
```

Source: `src/stream.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

Newline-delimited JSON records

<a id="op-e5067576b1c107ee31174afa"></a>
## clone

`function` · `datafusion_catalog::stream::StreamEncoding::clone` · datafusion-catalog 55.1.0

```rust
fn clone(&self) -> StreamEncoding
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::stream::StreamEncoding", "path": "StreamEncoding"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 17], "end": [93, 22], "filename": "src/stream.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/stream.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8f3e85ed93f8496bcc7ac68"></a>
## fmt

`function` · `datafusion_catalog::stream::StreamEncoding::fmt` · datafusion-catalog 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::stream::StreamEncoding", "path": "StreamEncoding"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 10], "end": [93, 15], "filename": "src/stream.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/stream.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b23756a748dbc7b86c56ab4"></a>
## from_str

`function` · `datafusion_catalog::stream::StreamEncoding::from_str` · datafusion-catalog 55.1.0

```rust
fn from_str(s: &str) -> std::result::Result<Self, Self::Err>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_catalog::stream::StreamEncoding", "path": "StreamEncoding"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [111, 2], "filename": "src/stream.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/stream.rs:104`. [Exact documentation build](https://docs.rs/crate/datafusion-catalog/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
