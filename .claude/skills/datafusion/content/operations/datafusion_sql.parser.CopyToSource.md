# `datafusion_sql::parser::CopyToSource`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.parser.CopyToSource.json).

<a id="op-8ac74088862aa5414429cec7"></a>
## CopyToSource

`enum` · `datafusion_sql::parser::CopyToSource` · datafusion-sql 55.1.0

```rust
enum CopyToSource
```

Source: `src/parser.rs:201`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-113f319bc519f7650a69bb84"></a>
## Query

`variant` · `datafusion_sql::parser::CopyToSource::Query` · datafusion-sql 55.1.0

```rust
Query
```

Source: `src/parser.rs:205`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

COPY (...query...) TO ...

<a id="op-21fa806b2e241e7fa9860143"></a>
## Relation

`variant` · `datafusion_sql::parser::CopyToSource::Relation` · datafusion-sql 55.1.0

```rust
Relation
```

Source: `src/parser.rs:203`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

`COPY <table> TO ...`

<a id="op-79e410f9daeb9e8bf1c24c7c"></a>
## clone

`function` · `datafusion_sql::parser::CopyToSource::clone` · datafusion-sql 55.1.0

```rust
fn clone(&self) -> CopyToSource
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::parser::CopyToSource", "path": "CopyToSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [200, 17], "end": [200, 22], "filename": "src/parser.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/parser.rs:200`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81a993c09923475c246e6207"></a>
## eq

`function` · `datafusion_sql::parser::CopyToSource::eq` · datafusion-sql 55.1.0

```rust
fn eq(&self, other: &CopyToSource) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::parser::CopyToSource", "path": "CopyToSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [200, 24], "end": [200, 33], "filename": "src/parser.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/parser.rs:200`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-460751c24c3ba888f7e9cc19"></a>
## fmt

`function` · `datafusion_sql::parser::CopyToSource::fmt` · datafusion-sql 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::parser::CopyToSource", "path": "CopyToSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [200, 10], "end": [200, 15], "filename": "src/parser.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/parser.rs:200`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5862a7fc63de250614e6ca7"></a>
## fmt

`function` · `datafusion_sql::parser::CopyToSource::fmt` · datafusion-sql 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::parser::CopyToSource", "path": "CopyToSource"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [208, 1], "end": [215, 2], "filename": "src/parser.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/parser.rs:209`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
