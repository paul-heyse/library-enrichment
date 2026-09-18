# `datafusion_sql::unparser::ast::BuilderError`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.unparser.ast.BuilderError.json).

<a id="op-5ea9e5a38f63f1432fb486af"></a>
## BuilderError

`enum` · `datafusion_sql::unparser::ast::BuilderError` · datafusion-sql 55.1.0

```rust
enum BuilderError
```

Source: `src/unparser/ast.rs:873`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-828ee891302b615b821a9540"></a>
## UninitializedField

`variant` · `datafusion_sql::unparser::ast::BuilderError::UninitializedField` · datafusion-sql 55.1.0

```rust
UninitializedField
```

Source: `src/unparser/ast.rs:874`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-723f3bf8614886c4faf20457"></a>
## ValidationError

`variant` · `datafusion_sql::unparser::ast::BuilderError::ValidationError` · datafusion-sql 55.1.0

```rust
ValidationError
```

Source: `src/unparser/ast.rs:875`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-823eb490415d3304cc272fa1"></a>
## fmt

`function` · `datafusion_sql::unparser::ast::BuilderError::fmt` · datafusion-sql 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::BuilderError", "path": "BuilderError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [872, 10], "end": [872, 15], "filename": "src/unparser/ast.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/unparser/ast.rs:872`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a23aab7154f753cbbc6adb51"></a>
## fmt

`function` · `datafusion_sql::unparser::ast::BuilderError::fmt` · datafusion-sql 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::BuilderError", "path": "BuilderError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [887, 1], "end": [896, 2], "filename": "src/unparser/ast.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/unparser/ast.rs:888`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c95c8a873db7b3f08a573d0"></a>
## from

`function` · `datafusion_sql::unparser::ast::BuilderError::from` · datafusion-sql 55.1.0

```rust
fn from(s: UninitializedFieldError) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::BuilderError", "path": "BuilderError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [877, 1], "end": [881, 2], "filename": "src/unparser/ast.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::UninitializedFieldError", "path": "UninitializedFieldError"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/unparser/ast.rs:878`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd9f22ab90b3a3405c3b0d5f"></a>
## from

`function` · `datafusion_sql::unparser::ast::BuilderError::from` · datafusion-sql 55.1.0

```rust
fn from(s: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::unparser::ast::BuilderError", "path": "BuilderError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [882, 1], "end": [886, 2], "filename": "src/unparser/ast.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/unparser/ast.rs:883`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
