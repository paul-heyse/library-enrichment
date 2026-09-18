# `datafusion_sql::parser::ResetStatement`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.parser.ResetStatement.json).

<a id="op-a5045b5ec802636203325692"></a>
## ResetStatement

`enum` · `datafusion_sql::parser::ResetStatement` · datafusion-sql 55.1.0

```rust
enum ResetStatement
```

Source: `src/parser.rs:314`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

DataFusion extension for `RESET`

<a id="op-01d409d3e5970fe48d116c28"></a>
## Variable

`variant` · `datafusion_sql::parser::ResetStatement::Variable` · datafusion-sql 55.1.0

```rust
Variable
```

Source: `src/parser.rs:316`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Reset a single configuration variable (stored as provided)

<a id="op-afb0f6917401b38c24c3bd37"></a>
## clone

`function` · `datafusion_sql::parser::ResetStatement::clone` · datafusion-sql 55.1.0

```rust
fn clone(&self) -> ResetStatement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::parser::ResetStatement", "path": "ResetStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 17], "end": [313, 22], "filename": "src/parser.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/parser.rs:313`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b58aaae00d522655f0547354"></a>
## eq

`function` · `datafusion_sql::parser::ResetStatement::eq` · datafusion-sql 55.1.0

```rust
fn eq(&self, other: &ResetStatement) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::parser::ResetStatement", "path": "ResetStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 24], "end": [313, 33], "filename": "src/parser.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/parser.rs:313`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-954ffa725ee1f48b16de72b7"></a>
## fmt

`function` · `datafusion_sql::parser::ResetStatement::fmt` · datafusion-sql 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::parser::ResetStatement", "path": "ResetStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 10], "end": [313, 15], "filename": "src/parser.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/parser.rs:313`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c86be4d136c72f80db958fc"></a>
## fmt

`function` · `datafusion_sql::parser::ResetStatement::fmt` · datafusion-sql 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::parser::ResetStatement", "path": "ResetStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [319, 1], "end": [325, 2], "filename": "src/parser.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/parser.rs:320`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
