# `datafusion_expr::logical_plan::statement::ResetVariable`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.statement.ResetVariable.json).

<a id="op-a72c5a9571e867e627410c20"></a>
## ResetVariable

`struct` · `datafusion_expr::logical_plan::statement::ResetVariable` · datafusion-expr 55.1.0

```rust
struct ResetVariable
```

Source: `src/logical_plan/statement.rs:202`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Reset a configuration variable to its default

<a id="op-0fb24f8c2a31b72aa7e4f91c"></a>
## clone

`function` · `datafusion_expr::logical_plan::statement::ResetVariable::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> ResetVariable
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::ResetVariable", "path": "ResetVariable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 17], "end": [201, 22], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/statement.rs:201`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bcb841c700fd0fa5f3f4f016"></a>
## eq

`function` · `datafusion_expr::logical_plan::statement::ResetVariable::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &ResetVariable) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::ResetVariable", "path": "ResetVariable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 24], "end": [201, 33], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/statement.rs:201`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d4962c66865578a6a191cc1"></a>
## fmt

`function` · `datafusion_expr::logical_plan::statement::ResetVariable::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::ResetVariable", "path": "ResetVariable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 10], "end": [201, 15], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/statement.rs:201`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f90af04d5b0b496798e3f1a"></a>
## hash

`function` · `datafusion_expr::logical_plan::statement::ResetVariable::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::ResetVariable", "path": "ResetVariable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 51], "end": [201, 55], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/statement.rs:201`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8fcb9c4ec3288f5fc857f40"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::statement::ResetVariable::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &ResetVariable) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::ResetVariable", "path": "ResetVariable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 39], "end": [201, 49], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/statement.rs:201`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea420e3c5fbd9809be30dbb8"></a>
## variable

`struct_field` · `datafusion_expr::logical_plan::statement::ResetVariable::variable` · datafusion-expr 55.1.0

```rust
variable: String
```

Source: `src/logical_plan/statement.rs:204`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The variable name
