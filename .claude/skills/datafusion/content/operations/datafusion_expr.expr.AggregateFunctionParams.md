# `datafusion_expr::expr::AggregateFunctionParams`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr.AggregateFunctionParams.json).

<a id="op-642f82f908f40807e678f624"></a>
## AggregateFunctionParams

`struct` · `datafusion_expr::expr::AggregateFunctionParams` · datafusion-expr 55.1.0

```rust
struct AggregateFunctionParams
```

Source: `src/expr.rs:1121`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-447f285ae90b0ef225d6ddd2"></a>
## args

`struct_field` · `datafusion_expr::expr::AggregateFunctionParams::args` · datafusion-expr 55.1.0

```rust
args: Vec<Expr>
```

Source: `src/expr.rs:1122`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ad67c20311dfabf5cdeaeb3"></a>
## clone

`function` · `datafusion_expr::expr::AggregateFunctionParams::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> AggregateFunctionParams
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::AggregateFunctionParams", "path": "AggregateFunctionParams"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1120, 10], "end": [1120, 15], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expr.rs:1120`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-74d03d4ba2338ec43823d7b8"></a>
## distinct

`struct_field` · `datafusion_expr::expr::AggregateFunctionParams::distinct` · datafusion-expr 55.1.0

```rust
distinct: bool
```

Source: `src/expr.rs:1124`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Whether this is a DISTINCT aggregation or not

<a id="op-d5d735775c74231b0df4287e"></a>
## eq

`function` · `datafusion_expr::expr::AggregateFunctionParams::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &AggregateFunctionParams) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::AggregateFunctionParams", "path": "AggregateFunctionParams"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1120, 17], "end": [1120, 26], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expr.rs:1120`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eac4db8960dd846c9ce8bb34"></a>
## filter

`struct_field` · `datafusion_expr::expr::AggregateFunctionParams::filter` · datafusion-expr 55.1.0

```rust
filter: Option<Box<Expr>>
```

Source: `src/expr.rs:1126`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Optional filter

<a id="op-d88e54f254067fe9bac8b563"></a>
## fmt

`function` · `datafusion_expr::expr::AggregateFunctionParams::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::AggregateFunctionParams", "path": "AggregateFunctionParams"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1120, 50], "end": [1120, 55], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr.rs:1120`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7316c835a18dd756bcf017b"></a>
## hash

`function` · `datafusion_expr::expr::AggregateFunctionParams::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::AggregateFunctionParams", "path": "AggregateFunctionParams"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1120, 44], "end": [1120, 48], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expr.rs:1120`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-308960d375c11c00f96ae54e"></a>
## null_treatment

`struct_field` · `datafusion_expr::expr::AggregateFunctionParams::null_treatment` · datafusion-expr 55.1.0

```rust
null_treatment: Option<NullTreatment>
```

Source: `src/expr.rs:1129`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dcbec8073fa8f3b6aa907922"></a>
## order_by

`struct_field` · `datafusion_expr::expr::AggregateFunctionParams::order_by` · datafusion-expr 55.1.0

```rust
order_by: Vec<Sort>
```

Source: `src/expr.rs:1128`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Optional ordering

<a id="op-63293eb2a80a2d6537feaedc"></a>
## partial_cmp

`function` · `datafusion_expr::expr::AggregateFunctionParams::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &AggregateFunctionParams) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::AggregateFunctionParams", "path": "AggregateFunctionParams"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1120, 32], "end": [1120, 42], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/expr.rs:1120`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
