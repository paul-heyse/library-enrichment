# `datafusion_physical_expr::aggregate::LoweredAggregate`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.aggregate.LoweredAggregate.json).

<a id="op-884fe9ff4d1c445cb63e6402"></a>
## LoweredAggregate

`struct` · `datafusion_physical_expr::aggregate::LoweredAggregate` · datafusion-physical-expr 55.1.0

```rust
struct LoweredAggregate
```

Source: `src/aggregate.rs:401`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Result of lowering a logical aggregate expression into physical aggregate
planning pieces.

<a id="op-6fd33e78c004190d20d4d89d"></a>
## aggregate

`struct_field` · `datafusion_physical_expr::aggregate::LoweredAggregate::aggregate` · datafusion-physical-expr 55.1.0

```rust
aggregate: std::sync::Arc<AggregateFunctionExpr>
```

Source: `src/aggregate.rs:404`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Physical aggregate expression that can be used by an aggregate execution
plan.

<a id="op-25e1c9ddb8ebbbc989028dcd"></a>
## clone

`function` · `datafusion_physical_expr::aggregate::LoweredAggregate::clone` · datafusion-physical-expr 55.1.0

```rust
fn clone(&self) -> LoweredAggregate
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::LoweredAggregate", "path": "LoweredAggregate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [400, 17], "end": [400, 22], "filename": "src/aggregate.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/aggregate.rs:400`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf3c935647b47ffaeb5f07f9"></a>
## filter

`struct_field` · `datafusion_physical_expr::aggregate::LoweredAggregate::filter` · datafusion-physical-expr 55.1.0

```rust
filter: Option<std::sync::Arc<dyn PhysicalExpr>>
```

Source: `src/aggregate.rs:406`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Optional physical filter expression for `FILTER (WHERE ...)`.

<a id="op-117a993c523d0f2bfbd1b5c9"></a>
## fmt

`function` · `datafusion_physical_expr::aggregate::LoweredAggregate::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::LoweredAggregate", "path": "LoweredAggregate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [400, 10], "end": [400, 15], "filename": "src/aggregate.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/aggregate.rs:400`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-19434134766c8e77a575aa62"></a>
## order_bys

`struct_field` · `datafusion_physical_expr::aggregate::LoweredAggregate::order_bys` · datafusion-physical-expr 55.1.0

```rust
order_bys: Vec<datafusion_physical_expr_common::sort_expr::PhysicalSortExpr>
```

Source: `src/aggregate.rs:408`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Physical ordering expressions from aggregate `ORDER BY`.
