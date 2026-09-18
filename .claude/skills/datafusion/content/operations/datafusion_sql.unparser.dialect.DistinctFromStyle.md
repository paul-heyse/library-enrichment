# `datafusion_sql::unparser::dialect::DistinctFromStyle`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.unparser.dialect.DistinctFromStyle.json).

<a id="op-a707851fbf636e9c4ee2695b"></a>
## DistinctFromStyle

`enum` · `datafusion_sql::unparser::dialect::DistinctFromStyle` · datafusion-sql 55.1.0

```rust
enum DistinctFromStyle
```

Source: `src/unparser/dialect.rs:343`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

`DistinctFromStyle` to use for unparsing `IsDistinctFrom` and `IsNotDistinctFrom` operators

<a id="op-e070741a7bbb951d770f5e71"></a>
## FullText

`variant` · `datafusion_sql::unparser::dialect::DistinctFromStyle::FullText` · datafusion-sql 55.1.0

```rust
FullText
```

Source: `src/unparser/dialect.rs:345`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

DBMS supports `IS (NOT) DISTINCT FROM`

<a id="op-2cf31874ed299baf3510233b"></a>
## Spaceship

`variant` · `datafusion_sql::unparser::dialect::DistinctFromStyle::Spaceship` · datafusion-sql 55.1.0

```rust
Spaceship
```

Source: `src/unparser/dialect.rs:347`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

DBMS supports equivalent operations via `<=>` and `NOT <=>`

<a id="op-d710805e3cb711f23de951f3"></a>
## clone

`function` · `datafusion_sql::unparser::dialect::DistinctFromStyle::clone` · datafusion-sql 55.1.0

```rust
fn clone(&self) -> DistinctFromStyle
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::DistinctFromStyle", "path": "DistinctFromStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [342, 10], "end": [342, 15], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/unparser/dialect.rs:342`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7aa29077a666859507976ee"></a>
## eq

`function` · `datafusion_sql::unparser::dialect::DistinctFromStyle::eq` · datafusion-sql 55.1.0

```rust
fn eq(&self, other: &DistinctFromStyle) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::DistinctFromStyle", "path": "DistinctFromStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [342, 23], "end": [342, 32], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/unparser/dialect.rs:342`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
