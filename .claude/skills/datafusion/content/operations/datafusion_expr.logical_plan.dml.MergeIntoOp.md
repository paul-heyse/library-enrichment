# `datafusion_expr::logical_plan::dml::MergeIntoOp`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.dml.MergeIntoOp.json).

<a id="op-a11249b6a092e349b07e7c4e"></a>
## MergeIntoOp

`struct` · `datafusion_expr::logical_plan::dml::MergeIntoOp` · datafusion-expr 55.1.0

```rust
struct MergeIntoOp
```

Source: `src/logical_plan/dml.rs:303`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Describes a MERGE INTO operation's parameters.

<a id="op-098d4d47b88b25b00a3baa75"></a>
## clauses

`struct_field` · `datafusion_expr::logical_plan::dml::MergeIntoOp::clauses` · datafusion-expr 55.1.0

```rust
clauses: Vec<MergeIntoClause>
```

Source: `src/logical_plan/dml.rs:307`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The WHEN clauses, in the order they appeared in the SQL.

<a id="op-68ccb4c249a1351f76212cdb"></a>
## clone

`function` · `datafusion_expr::logical_plan::dml::MergeIntoOp::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> MergeIntoOp
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::MergeIntoOp", "path": "MergeIntoOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [302, 17], "end": [302, 22], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/dml.rs:302`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7dd965f901ccb6b1d5918b84"></a>
## eq

`function` · `datafusion_expr::logical_plan::dml::MergeIntoOp::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &MergeIntoOp) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::MergeIntoOp", "path": "MergeIntoOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [302, 24], "end": [302, 33], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/dml.rs:302`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-304117292646919063f3af12"></a>
## exprs

`function` · `datafusion_expr::logical_plan::dml::MergeIntoOp::exprs` · datafusion-expr 55.1.0

```rust
fn exprs(&self) -> Vec<&Expr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::MergeIntoOp", "path": "MergeIntoOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [310, 1], "end": [408, 2], "filename": "src/logical_plan/dml.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/dml.rs:332`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Top-level [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc)s in stable order: `on`, then per-clause predicate
(if any) and action value expressions.

<a id="op-42810327ca574be29a1d77da"></a>
## fmt

`function` · `datafusion_expr::logical_plan::dml::MergeIntoOp::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::MergeIntoOp", "path": "MergeIntoOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [302, 10], "end": [302, 15], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/dml.rs:302`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7af4e66e912db26fc866870"></a>
## hash

`function` · `datafusion_expr::logical_plan::dml::MergeIntoOp::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::MergeIntoOp", "path": "MergeIntoOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [302, 51], "end": [302, 55], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/dml.rs:302`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd7edf74799413e1fc46be6c"></a>
## on

`struct_field` · `datafusion_expr::logical_plan::dml::MergeIntoOp::on` · datafusion-expr 55.1.0

```rust
on: Expr
```

Source: `src/logical_plan/dml.rs:305`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The join condition from `ON <expr>`.

<a id="op-680b1124448c455a4fb7b51b"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::dml::MergeIntoOp::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &MergeIntoOp) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::MergeIntoOp", "path": "MergeIntoOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [302, 39], "end": [302, 49], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/dml.rs:302`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41287758c8f946e0cecd0709"></a>
## with_new_exprs

`function` · `datafusion_expr::logical_plan::dml::MergeIntoOp::with_new_exprs` · datafusion-expr 55.1.0

```rust
fn with_new_exprs(&self, exprs: Vec<Expr>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::MergeIntoOp", "path": "MergeIntoOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [310, 1], "end": [408, 2], "filename": "src/logical_plan/dml.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/dml.rs:356`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Rebuild this `MergeIntoOp` from a flat vector of new expressions, in
the same order produced by [`Self::exprs`](../operations/datafusion_expr.logical_plan.dml.MergeIntoOp.md#op-304117292646919063f3af12). The clause kinds, action
kinds, column lists, and presence/absence of each predicate are
preserved from `self`.
