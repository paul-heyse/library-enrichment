# `datafusion_expr::logical_plan::statement::Deallocate`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.statement.Deallocate.json).

<a id="op-dce49df8ad849c636d53f897"></a>
## Deallocate

`struct` · `datafusion_expr::logical_plan::statement::Deallocate` · datafusion-expr 55.1.0

```rust
struct Deallocate
```

Source: `src/logical_plan/statement.rs:229`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Deallocate a prepared statement.

<a id="op-aee1bff60aa41386880c187a"></a>
## clone

`function` · `datafusion_expr::logical_plan::statement::Deallocate::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> Deallocate
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::Deallocate", "path": "Deallocate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [228, 17], "end": [228, 22], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/statement.rs:228`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-826012a029fc2d3887463426"></a>
## eq

`function` · `datafusion_expr::logical_plan::statement::Deallocate::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Deallocate) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::Deallocate", "path": "Deallocate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [228, 24], "end": [228, 33], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/statement.rs:228`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f09171ceec4388d612a05f09"></a>
## fmt

`function` · `datafusion_expr::logical_plan::statement::Deallocate::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::Deallocate", "path": "Deallocate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [228, 10], "end": [228, 15], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/statement.rs:228`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1fba2decf528834a8fefed11"></a>
## hash

`function` · `datafusion_expr::logical_plan::statement::Deallocate::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::Deallocate", "path": "Deallocate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [228, 51], "end": [228, 55], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/statement.rs:228`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31c9f6af7de009cd1a12abdf"></a>
## name

`struct_field` · `datafusion_expr::logical_plan::statement::Deallocate::name` · datafusion-expr 55.1.0

```rust
name: String
```

Source: `src/logical_plan/statement.rs:231`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The name of the prepared statement to deallocate

<a id="op-3864f0e799842cea52ac1ce3"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::statement::Deallocate::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Deallocate) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::statement::Deallocate", "path": "Deallocate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [228, 35], "end": [228, 45], "filename": "src/logical_plan/statement.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/statement.rs:228`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
