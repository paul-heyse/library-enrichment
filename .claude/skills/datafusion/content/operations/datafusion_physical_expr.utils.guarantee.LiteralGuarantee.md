# `datafusion_physical_expr::utils::guarantee::LiteralGuarantee`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.utils.guarantee.LiteralGuarantee.json).

<a id="op-c6447f3f77c11eaea6c2ddd0"></a>
## LiteralGuarantee

`struct` · `datafusion_physical_expr::utils::guarantee::LiteralGuarantee` · datafusion-physical-expr 55.1.0

```rust
struct LiteralGuarantee
```

Source: `src/utils/guarantee.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Represents a guarantee that must be true for a boolean expression to
evaluate to `true`.

The guarantee takes the form of a column and a set of literal (constant)
[`ScalarValue`](../operations/datafusion_common.scalar.ScalarValue.md#op-360b267c379d0a7a93dce87b)s. For the expression to evaluate to `true`, the column *must
satisfy* the guarantee(s).

To satisfy the guarantee, depending on [`Guarantee`](../operations/datafusion_physical_expr.utils.guarantee.Guarantee.md#op-2f7eb0649ae818e53b368cc3), the values in the
column must either:

1. be ONLY one of that set
2. NOT be ANY of that set

# Uses `LiteralGuarantee`s

`LiteralGuarantee`s can be used to simplify filter expressions and skip data
files (e.g. row groups in parquet files) by proving expressions can not
possibly evaluate to `true`. For example, if we have a guarantee that `a`
must be in (`1`) for a filter to evaluate to `true`, then we can skip any
partition where we know that `a` never has the value of `1`.

**Important**: If a `LiteralGuarantee` is not satisfied, the relevant
expression is *guaranteed* to evaluate to `false` or `null`. **However**,
the opposite does not hold. Even if all `LiteralGuarantee`s are satisfied,
that does **not** guarantee that the predicate will actually evaluate to
`true`: it may still evaluate to `true`, `false` or `null`.

# Creating `LiteralGuarantee`s

Use [`LiteralGuarantee::analyze`](../operations/datafusion_physical_expr.utils.guarantee.LiteralGuarantee.md#op-91c0e8e784ae63d2e6f306e6) to extract literal guarantees from a
filter predicate.

# Details
A guarantee can be one of two forms:

1. The column must be one the values for the predicate to be `true`. If the
   column takes on any other value, the predicate can not evaluate to `true`.
   For example,
   `(a = 1)`, `(a = 1 OR a = 2)` or `a IN (1, 2, 3)`

2. The column must NOT be one of the values for the predicate to be `true`.
   If the column can ONLY take one of these values, the predicate can not
   evaluate to `true`. For example,
   `(a != 1)`, `(a != 1 AND a != 2)` or `a NOT IN (1, 2, 3)`

<a id="op-91c0e8e784ae63d2e6f306e6"></a>
## analyze

`function` · `datafusion_physical_expr::utils::guarantee::LiteralGuarantee::analyze` · datafusion-physical-expr 55.1.0

```rust
fn analyze(expr: &Arc<dyn PhysicalExpr>) -> Vec<LiteralGuarantee>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::utils::guarantee::LiteralGuarantee", "path": "LiteralGuarantee"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [246, 2], "filename": "src/utils/guarantee.rs"}, "trait": null, "trait_path": null}`

Source: `src/utils/guarantee.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Return a list of [`LiteralGuarantee`](../operations/datafusion_physical_expr.utils.guarantee.LiteralGuarantee.md#op-c6447f3f77c11eaea6c2ddd0)s that must be satisfied for `expr`
to evaluate to `true`.

If more than one `LiteralGuarantee` is returned, they must **all** hold
for the expression to possibly be `true`. If any is not satisfied, the
expression is guaranteed to be `null` or `false`.

# Notes:
1. `expr` must be a boolean expression or inlist expression.
2. `expr` is not simplified prior to analysis.

<a id="op-973ac25cf864c7e98068cf2c"></a>
## clone

`function` · `datafusion_physical_expr::utils::guarantee::LiteralGuarantee::clone` · datafusion-physical-expr 55.1.0

```rust
fn clone(&self) -> LiteralGuarantee
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::utils::guarantee::LiteralGuarantee", "path": "LiteralGuarantee"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 17], "end": [73, 22], "filename": "src/utils/guarantee.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/utils/guarantee.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9398e0966b256d8b7803288f"></a>
## column

`struct_field` · `datafusion_physical_expr::utils::guarantee::LiteralGuarantee::column` · datafusion-physical-expr 55.1.0

```rust
column: datafusion_common::Column
```

Source: `src/utils/guarantee.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2ac6c647164792b447433d04"></a>
## eq

`function` · `datafusion_physical_expr::utils::guarantee::LiteralGuarantee::eq` · datafusion-physical-expr 55.1.0

```rust
fn eq(&self, other: &LiteralGuarantee) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::utils::guarantee::LiteralGuarantee", "path": "LiteralGuarantee"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 24], "end": [73, 33], "filename": "src/utils/guarantee.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/utils/guarantee.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-040500df3911321f1213565c"></a>
## fmt

`function` · `datafusion_physical_expr::utils::guarantee::LiteralGuarantee::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::utils::guarantee::LiteralGuarantee", "path": "LiteralGuarantee"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 10], "end": [73, 15], "filename": "src/utils/guarantee.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/utils/guarantee.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-975a12cb7e869bc93fde81ea"></a>
## fmt

`function` · `datafusion_physical_expr::utils::guarantee::LiteralGuarantee::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::utils::guarantee::LiteralGuarantee", "path": "LiteralGuarantee"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [248, 1], "end": [268, 2], "filename": "src/utils/guarantee.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/utils/guarantee.rs:249`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-450e8f53ce4e33d4c93a673e"></a>
## guarantee

`struct_field` · `datafusion_physical_expr::utils::guarantee::LiteralGuarantee::guarantee` · datafusion-physical-expr 55.1.0

```rust
guarantee: Guarantee
```

Source: `src/utils/guarantee.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-945c0e7267a61d5f3372a63d"></a>
## literals

`struct_field` · `datafusion_physical_expr::utils::guarantee::LiteralGuarantee::literals` · datafusion-physical-expr 55.1.0

```rust
literals: std::collections::HashSet<datafusion_common::ScalarValue>
```

Source: `src/utils/guarantee.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
