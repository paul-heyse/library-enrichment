# `parquet::record::api::Map`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.record.api.Map.json).

<a id="op-deba2e0676177032d2a6ae3a"></a>
## Map

`struct` · `parquet::record::api::Map` · parquet 59.3.0

```rust
struct Map
```

Source: `src/record/api.rs:473`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

`Map` represents a map which contains a list of key->value pairs.

<a id="op-b8b0fb76805e67ef699a9c50"></a>
## clone

`function` · `parquet::record::api::Map::clone` · parquet 59.3.0

```rust
fn clone(&self) -> Map
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Map", "path": "Map"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [472, 10], "end": [472, 15], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/record/api.rs:472`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9bc1e7399eb7c3281771be9"></a>
## entries

`function` · `parquet::record::api::Map::entries` · parquet 59.3.0

```rust
fn entries(&self) -> &[(Field, Field)]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Map", "path": "Map"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [478, 1], "end": [488, 2], "filename": "src/record/api.rs"}, "trait": null, "trait_path": null}`

Source: `src/record/api.rs:485`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Get the reference to the key-value pairs in this map

<a id="op-6db5760fa2c29fa0263399d5"></a>
## eq

`function` · `parquet::record::api::Map::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &Map) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Map", "path": "Map"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [472, 24], "end": [472, 33], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/record/api.rs:472`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3a27884cb47fa6c0033855b"></a>
## fmt

`function` · `parquet::record::api::Map::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Map", "path": "Map"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [472, 17], "end": [472, 22], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/record/api.rs:472`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72e273c1e067cbc2e6e699f6"></a>
## get_keys

`function` · `parquet::record::api::Map::get_keys` · parquet 59.3.0

```rust
fn get_keys<'a>(&'a self) -> Box<dyn ListAccessor + 'a>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Map", "path": "Map"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [567, 1], "end": [581, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::MapAccessor", "path": "MapAccessor"}, "trait_path": "parquet::record::api::MapAccessor"}`

Source: `src/record/api.rs:568`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-842f0abe075d389bb2626ccc"></a>
## get_values

`function` · `parquet::record::api::Map::get_values` · parquet 59.3.0

```rust
fn get_values<'a>(&'a self) -> Box<dyn ListAccessor + 'a>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Map", "path": "Map"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [567, 1], "end": [581, 2], "filename": "src/record/api.rs"}, "trait": {"args": null, "id": "parquet::record::api::MapAccessor", "path": "MapAccessor"}, "trait_path": "parquet::record::api::MapAccessor"}`

Source: `src/record/api.rs:575`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c85732a0ef07e60e4ef7d8bd"></a>
## len

`function` · `parquet::record::api::Map::len` · parquet 59.3.0

```rust
fn len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::record::api::Map", "path": "Map"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [478, 1], "end": [488, 2], "filename": "src/record/api.rs"}, "trait": null, "trait_path": null}`

Source: `src/record/api.rs:480`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Get the number of fields in this row
