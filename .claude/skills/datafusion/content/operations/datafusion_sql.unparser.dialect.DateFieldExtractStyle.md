# `datafusion_sql::unparser::dialect::DateFieldExtractStyle`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.unparser.dialect.DateFieldExtractStyle.json).

<a id="op-6675c5cf61351be5b4e0584e"></a>
## DateFieldExtractStyle

`enum` · `datafusion_sql::unparser::dialect::DateFieldExtractStyle` · datafusion-sql 55.1.0

```rust
enum DateFieldExtractStyle
```

Source: `src/unparser/dialect.rs:324`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Datetime subfield extraction style for unparsing

`<https://www.postgresql.org/docs/current/functions-datetime.html#FUNCTIONS-DATETIME-EXTRACT>`
Different DBMSs follow different standards; popular ones are:
date_part('YEAR', date '2001-02-16')
EXTRACT(YEAR from date '2001-02-16')
Some DBMSs, like Postgres, support both, whereas others like MySQL require EXTRACT.

<a id="op-c76f55d13258ebb01274bfde"></a>
## DatePart

`variant` · `datafusion_sql::unparser::dialect::DateFieldExtractStyle::DatePart` · datafusion-sql 55.1.0

```rust
DatePart
```

Source: `src/unparser/dialect.rs:325`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c025ad88127c171a789b1ebe"></a>
## Extract

`variant` · `datafusion_sql::unparser::dialect::DateFieldExtractStyle::Extract` · datafusion-sql 55.1.0

```rust
Extract
```

Source: `src/unparser/dialect.rs:326`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-876af4c1b20a364057bd6f2b"></a>
## Strftime

`variant` · `datafusion_sql::unparser::dialect::DateFieldExtractStyle::Strftime` · datafusion-sql 55.1.0

```rust
Strftime
```

Source: `src/unparser/dialect.rs:327`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af414e3a75bdc22e8c8fee37"></a>
## clone

`function` · `datafusion_sql::unparser::dialect::DateFieldExtractStyle::clone` · datafusion-sql 55.1.0

```rust
fn clone(&self) -> DateFieldExtractStyle
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::DateFieldExtractStyle", "path": "DateFieldExtractStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [323, 10], "end": [323, 15], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/unparser/dialect.rs:323`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-998b44c5e211cc96166051e1"></a>
## eq

`function` · `datafusion_sql::unparser::dialect::DateFieldExtractStyle::eq` · datafusion-sql 55.1.0

```rust
fn eq(&self, other: &DateFieldExtractStyle) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::dialect::DateFieldExtractStyle", "path": "DateFieldExtractStyle"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [323, 23], "end": [323, 32], "filename": "src/unparser/dialect.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/unparser/dialect.rs:323`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
