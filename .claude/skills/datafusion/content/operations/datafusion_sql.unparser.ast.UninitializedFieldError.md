# `datafusion_sql::unparser::ast::UninitializedFieldError`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.unparser.ast.UninitializedFieldError.json).

<a id="op-ac48722e33b392533d0ac325"></a>
## UninitializedFieldError

`struct` · `datafusion_sql::unparser::ast::UninitializedFieldError` · datafusion-sql 55.1.0

```rust
struct UninitializedFieldError
```

Source: `src/unparser/ast.rs:845`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Runtime error when a `build()` method is called and one or more required fields
do not have a value.

<a id="op-916bc833ea72bdd563a34251"></a>
## clone

`function` · `datafusion_sql::unparser::ast::UninitializedFieldError::clone` · datafusion-sql 55.1.0

```rust
fn clone(&self) -> UninitializedFieldError
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::UninitializedFieldError", "path": "UninitializedFieldError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [844, 17], "end": [844, 22], "filename": "src/unparser/ast.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/unparser/ast.rs:844`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc6160b6eb484f9755ba7b00"></a>
## field_name

`function` · `datafusion_sql::unparser::ast::UninitializedFieldError::field_name` · datafusion-sql 55.1.0

```rust
fn field_name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::UninitializedFieldError", "path": "UninitializedFieldError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [847, 1], "end": [857, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:854`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Get the name of the first-declared field that wasn't initialized

<a id="op-c6f6f42e4a46edfd8bd2591a"></a>
## fmt

`function` · `datafusion_sql::unparser::ast::UninitializedFieldError::fmt` · datafusion-sql 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::UninitializedFieldError", "path": "UninitializedFieldError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [844, 10], "end": [844, 15], "filename": "src/unparser/ast.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/unparser/ast.rs:844`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-caba2e499f6d87d1bd957a21"></a>
## fmt

`function` · `datafusion_sql::unparser::ast::UninitializedFieldError::fmt` · datafusion-sql 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::UninitializedFieldError", "path": "UninitializedFieldError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [859, 1], "end": [863, 2], "filename": "src/unparser/ast.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/unparser/ast.rs:860`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad6970a87a531cdc640d2e2a"></a>
## from

`function` · `datafusion_sql::unparser::ast::UninitializedFieldError::from` · datafusion-sql 55.1.0

```rust
fn from(field_name: &'static str) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::UninitializedFieldError", "path": "UninitializedFieldError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [865, 1], "end": [869, 2], "filename": "src/unparser/ast.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'static", "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/unparser/ast.rs:866`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ffbbf250bd15a3e1b160f782"></a>
## new

`function` · `datafusion_sql::unparser::ast::UninitializedFieldError::new` · datafusion-sql 55.1.0

```rust
fn new(field_name: &'static str) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::UninitializedFieldError", "path": "UninitializedFieldError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [847, 1], "end": [857, 2], "filename": "src/unparser/ast.rs"}, "trait": null, "trait_path": null}`

Source: `src/unparser/ast.rs:849`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Create a new `UninitializedFieldError` for the specified field name.
