# `parquet::basic::IntType`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.basic.IntType.json).

<a id="op-b55eb592d4e82ae4ea85c85c"></a>
## IntType

`struct` · `parquet::basic::IntType` · parquet 59.3.0

```rust
struct IntType
```

Source: `src/basic.rs:210`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ebaa4224e5a854a9f221ecd"></a>
## bit_width

`struct_field` · `parquet::basic::IntType::bit_width` · parquet 59.3.0

```rust
bit_width: i8
```

Source: `src/basic.rs:210`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The number of bits in the integer.

<a id="op-b558f609da25bbb38b5907f0"></a>
## clone

`function` · `parquet::basic::IntType::clone` · parquet 59.3.0

```rust
fn clone(&self) -> IntType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::IntType", "path": "IntType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [210, 1], "end": [217, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/basic.rs:210`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5498579fa2b501fc903826fe"></a>
## eq

`function` · `parquet::basic::IntType::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &IntType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::IntType", "path": "IntType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [210, 1], "end": [217, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/basic.rs:210`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-097fbc682db65213670909bc"></a>
## fmt

`function` · `parquet::basic::IntType::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::IntType", "path": "IntType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [210, 1], "end": [217, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/basic.rs:210`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b7e6f1a9dd639f783a4c855"></a>
## is_signed

`struct_field` · `parquet::basic::IntType::is_signed` · parquet 59.3.0

```rust
is_signed: bool
```

Source: `src/basic.rs:210`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Whether the integer is signed.
