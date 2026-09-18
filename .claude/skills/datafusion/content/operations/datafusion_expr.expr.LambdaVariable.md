# `datafusion_expr::expr::LambdaVariable`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr.LambdaVariable.json).

<a id="op-b6c77ba2103aa239605d5e16"></a>
## LambdaVariable

`struct` · `datafusion_expr::expr::LambdaVariable` · datafusion-expr 55.1.0

```rust
struct LambdaVariable
```

Source: `src/expr.rs:519`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A named reference to a lambda parameter which includes it's own [`FieldRef`](../operations/arrow_schema.field.FieldRef.md#op-a0fdaf7a91a3563923566542),
which is used to implement [`ExprSchemable`](../operations/datafusion_expr.expr_schema.ExprSchemable.md#op-7ac274f9ddd864e7c1033522), for example. It is an option only to make
easier for `expr_api` users to construct lambda variables, but any expression
tree or [`LogicalPlan`] containing unresolved variables must be resolved before
usage with either [`Expr::resolve_lambda_variables`](../operations/datafusion_expr.expr.Expr.md#op-a84a19ab1b8cbaf85e0d80f0) or
[`LogicalPlan::resolve_lambda_variables`]. The default SQL planner produces
already resolved variables and no further resolving is required.

After resolving, if any argument from the lambda function which this
variables originates from have it's field changed (type, nullability,
metadata, etc), the resolved variable may became outdated and must be
resolved again.

[`LogicalPlan`]: crate::LogicalPlan
[`LogicalPlan::resolve_lambda_variables`]: crate::LogicalPlan::resolve_lambda_variables

<a id="op-4b701a64603c5fc8977c28a6"></a>
## clone

`function` · `datafusion_expr::expr::LambdaVariable::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> LambdaVariable
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::LambdaVariable", "path": "LambdaVariable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [518, 10], "end": [518, 15], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expr.rs:518`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5bcb9b02d1680fe1433321e"></a>
## eq

`function` · `datafusion_expr::expr::LambdaVariable::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &LambdaVariable) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::LambdaVariable", "path": "LambdaVariable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [518, 17], "end": [518, 26], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expr.rs:518`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88f713fda17d7d0c7186cfc6"></a>
## field

`struct_field` · `datafusion_expr::expr::LambdaVariable::field` · datafusion-expr 55.1.0

```rust
field: Option<arrow::datatypes::FieldRef>
```

Source: `src/expr.rs:521`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-19fb9ed9973b91b2b9ca8e8a"></a>
## fmt

`function` · `datafusion_expr::expr::LambdaVariable::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::LambdaVariable", "path": "LambdaVariable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [518, 44], "end": [518, 49], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr.rs:518`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ab12dbf69180edfea090a2c"></a>
## hash

`function` · `datafusion_expr::expr::LambdaVariable::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::LambdaVariable", "path": "LambdaVariable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [518, 51], "end": [518, 55], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expr.rs:518`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d96cb0caa4882192e52f6ffc"></a>
## name

`struct_field` · `datafusion_expr::expr::LambdaVariable::name` · datafusion-expr 55.1.0

```rust
name: String
```

Source: `src/expr.rs:520`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2db6db0f6805fe9be714c8c8"></a>
## new

`function` · `datafusion_expr::expr::LambdaVariable::new` · datafusion-expr 55.1.0

```rust
fn new(name: String, field: Option<FieldRef>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::LambdaVariable", "path": "LambdaVariable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [525, 1], "end": [543, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:532`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a lambda variable from a name and an optional field.
If the field is none, the expression tree or LogicalPlan which
owns this variable must be resolved before usage with either
[`Expr::resolve_lambda_variables`](../operations/datafusion_expr.expr.Expr.md#op-a84a19ab1b8cbaf85e0d80f0) or [`LogicalPlan::resolve_lambda_variables`].

[`LogicalPlan::resolve_lambda_variables`]: crate::LogicalPlan::resolve_lambda_variables

<a id="op-e6d32d4e061fc5577a7b714e"></a>
## partial_cmp

`function` · `datafusion_expr::expr::LambdaVariable::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &LambdaVariable) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::LambdaVariable", "path": "LambdaVariable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [518, 28], "end": [518, 38], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/expr.rs:518`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7160b828ef888afff98c2a0"></a>
## spans

`struct_field` · `datafusion_expr::expr::LambdaVariable::spans` · datafusion-expr 55.1.0

```rust
spans: datafusion_common::Spans
```

Source: `src/expr.rs:522`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e401213f7d1b8c40c49b224"></a>
## spans_mut

`function` · `datafusion_expr::expr::LambdaVariable::spans_mut` · datafusion-expr 55.1.0

```rust
fn spans_mut(&mut self) -> &mut Spans
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::LambdaVariable", "path": "LambdaVariable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [525, 1], "end": [543, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:540`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
