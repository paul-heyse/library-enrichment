# `parquet::basic::TimestampType`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.basic.TimestampType.json).

<a id="op-61b4081ec12fd8b8171e8417"></a>
## TimestampType

`struct` · `parquet::basic::TimestampType` · parquet 59.3.0

```rust
struct TimestampType
```

Source: `src/basic.rs:198`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9fa346752eeb076b531dd4ab"></a>
## clone

`function` · `parquet::basic::TimestampType::clone` · parquet 59.3.0

```rust
fn clone(&self) -> TimestampType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::TimestampType", "path": "TimestampType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [198, 1], "end": [205, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/basic.rs:198`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc0222ae710df534851014dc"></a>
## eq

`function` · `parquet::basic::TimestampType::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &TimestampType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::TimestampType", "path": "TimestampType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [198, 1], "end": [205, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/basic.rs:198`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86a3cb774756ee4133292e6a"></a>
## fmt

`function` · `parquet::basic::TimestampType::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::basic::TimestampType", "path": "TimestampType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [198, 1], "end": [205, 2], "filename": "src/basic.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/basic.rs:198`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-508521fd25e47b90b033c933"></a>
## is_adjusted_to_u_t_c

`struct_field` · `parquet::basic::TimestampType::is_adjusted_to_u_t_c` · parquet 59.3.0

```rust
is_adjusted_to_u_t_c: bool
```

Source: `src/basic.rs:198`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Whether the timestamp is adjusted to UTC.

<a id="op-9f7d9a943d2e13df7347bb14"></a>
## unit

`struct_field` · `parquet::basic::TimestampType::unit` · parquet 59.3.0

```rust
unit: TimeUnit
```

Source: `src/basic.rs:198`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

The unit of time.
