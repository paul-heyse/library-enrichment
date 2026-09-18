# `datafusion_physical_expr::equivalence::class::ConstExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.equivalence.class.ConstExpr.json).

<a id="op-2a7536bfbd272811a206c52c"></a>
## ConstExpr

`struct` · `datafusion_physical_expr::equivalence::class::ConstExpr` · datafusion-physical-expr 55.1.0

```rust
struct ConstExpr
```

Source: `src/equivalence/class.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

A structure representing a expression known to be constant in a physical
execution plan.

The `ConstExpr` struct encapsulates an expression that is constant during
the execution of a query. For example if a filter like `A = 5` appears
earlier in the plan, `A` would become a constant in subsequent operations.

# Fields

- `expr`: Constant expression for a node in the physical plan.
- `across_partitions`: A boolean flag indicating whether the constant
  expression is the same across partitions. If set to `true`, the constant
  expression has same value for all partitions. If set to `false`, the
  constant expression may have different values for different partitions.

# Example

```rust
# use datafusion_physical_expr::ConstExpr;
# use datafusion_physical_expr::expressions::lit;
let col = lit(5);
// Create a constant expression from a physical expression:
let const_expr = ConstExpr::from(col);
```

<a id="op-35a56aaa83fe0d0927f4b664"></a>
## across_partitions

`struct_field` · `datafusion_physical_expr::equivalence::class::ConstExpr::across_partitions` · datafusion-physical-expr 55.1.0

```rust
across_partitions: AcrossPartitions
```

Source: `src/equivalence/class.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Indicates whether the constant have the same value across all partitions.

<a id="op-8ddb18bd8f233cba24b92316"></a>
## clone

`function` · `datafusion_physical_expr::equivalence::class::ConstExpr::clone` · datafusion-physical-expr 55.1.0

```rust
fn clone(&self) -> ConstExpr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::ConstExpr", "path": "ConstExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 10], "end": [88, 15], "filename": "src/equivalence/class.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/equivalence/class.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e7a0482ad3d312397c5996d"></a>
## collect_predicate_constants

`function` · `datafusion_physical_expr::equivalence::class::ConstExpr::collect_predicate_constants` · datafusion-physical-expr 55.1.0

```rust
fn collect_predicate_constants(input_eqs: &EquivalenceProperties, predicate: &Arc<dyn PhysicalExpr>) -> Vec<ConstExpr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::ConstExpr", "path": "crate::ConstExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [106, 2], "filename": "src/utils/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/utils/mod.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Collects predicate-derived constants from equality conjunctions.

For each equality predicate of the form `lhs = rhs`, if either side is
already known constant according to `input_eqs`, or is a literal, then
the other side is also constant and will be returned as a [`ConstExpr`](../operations/datafusion_physical_expr.equivalence.class.ConstExpr.md#op-2a7536bfbd272811a206c52c).

Literals are treated as uniform constants across partitions, so
`col = literal` produces a constant for `col` with the literal value.

For example, given predicate `a = 5 AND b = c` where `c` is already
known constant, this returns constants for both `a` (Uniform with value
5) and `b` (propagating `c`'s across-partitions value).

<a id="op-33d757070c3a2f06671a2f87"></a>
## eq

`function` · `datafusion_physical_expr::equivalence::class::ConstExpr::eq` · datafusion-physical-expr 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::ConstExpr", "path": "ConstExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [139, 1], "end": [143, 2], "filename": "src/equivalence/class.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/equivalence/class.rs:140`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28eb09333570bd9c7095d853"></a>
## expr

`struct_field` · `datafusion_physical_expr::equivalence::class::ConstExpr::expr` · datafusion-physical-expr 55.1.0

```rust
expr: std::sync::Arc<dyn PhysicalExpr>
```

Source: `src/equivalence/class.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

The expression that is known to be constant (e.g. a `Column`).

<a id="op-05b67e32db9fa910e12a1ff7"></a>
## fmt

`function` · `datafusion_physical_expr::equivalence::class::ConstExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::ConstExpr", "path": "ConstExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 17], "end": [88, 22], "filename": "src/equivalence/class.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/equivalence/class.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-75c6554d00a3bcc5752d5290"></a>
## fmt

`function` · `datafusion_physical_expr::equivalence::class::ConstExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::ConstExpr", "path": "ConstExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [145, 1], "end": [150, 2], "filename": "src/equivalence/class.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/equivalence/class.rs:146`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-abf66dab2a14fe1a34151acf"></a>
## format_list

`function` · `datafusion_physical_expr::equivalence::class::ConstExpr::format_list` · datafusion-physical-expr 55.1.0

```rust
fn format_list(input: &[ConstExpr]) -> impl Display + '_
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::ConstExpr", "path": "ConstExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 1], "end": [137, 2], "filename": "src/equivalence/class.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/class.rs:119`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Returns a [`Display`]able list of `ConstExpr`.

Unresolved upstream links (retained, not inferred): ``Display``.

<a id="op-b9c9e48a814aab50e58896cb"></a>
## from

`function` · `datafusion_physical_expr::equivalence::class::ConstExpr::from` · datafusion-physical-expr 55.1.0

```rust
fn from(expr: Arc<dyn PhysicalExpr>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::ConstExpr", "path": "ConstExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [152, 1], "end": [167, 2], "filename": "src/equivalence/class.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"dyn_trait": {"lifetime": null, "traits": [{"generic_params": [], "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}}]}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/equivalence/class.rs:153`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d24406069bbcd427d3d63b0"></a>
## new

`function` · `datafusion_physical_expr::equivalence::class::ConstExpr::new` · datafusion-physical-expr 55.1.0

```rust
fn new(expr: Arc<dyn PhysicalExpr>, across_partitions: AcrossPartitions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::ConstExpr", "path": "ConstExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 1], "end": [137, 2], "filename": "src/equivalence/class.rs"}, "trait": null, "trait_path": null}`

Source: `src/equivalence/class.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a new constant expression from a physical expression, specifying
whether the constant expression is the same across partitions.

Note that you can also use `ConstExpr::from` to create a constant
expression from just a physical expression, with the *safe* assumption
of heterogenous values across partitions unless the expression is a
literal.
