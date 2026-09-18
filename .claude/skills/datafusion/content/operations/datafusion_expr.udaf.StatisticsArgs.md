# `datafusion_expr::udaf::StatisticsArgs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.udaf.StatisticsArgs.json).

<a id="op-294fa60e50ab2233c4ab0122"></a>
## StatisticsArgs

`struct` · `datafusion_expr::udaf::StatisticsArgs` · datafusion-expr 55.1.0

```rust
struct StatisticsArgs<'a>
```

Source: `src/udaf.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Arguments passed to [`AggregateUDFImpl::value_from_stats`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-20017ef1e72ea9537862313e)

<a id="op-bca9ccbf53d958dc201e7ed1"></a>
## exprs

`struct_field` · `datafusion_expr::udaf::StatisticsArgs::exprs` · datafusion-expr 55.1.0

```rust
exprs: &'a [std::sync::Arc<dyn PhysicalExpr>]
```

Source: `src/udaf.rs:119`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The physical expression of arguments the aggregate function takes.

<a id="op-f51d24054f2705efb52f9ad1"></a>
## fmt

`function` · `datafusion_expr::udaf::StatisticsArgs::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_expr::udaf::StatisticsArgs", "path": "StatisticsArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [106, 10], "end": [106, 15], "filename": "src/udaf.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/udaf.rs:106`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc47ceb532763e5f1d2e5b98"></a>
## is_distinct

`struct_field` · `datafusion_expr::udaf::StatisticsArgs::is_distinct` · datafusion-expr 55.1.0

```rust
is_distinct: bool
```

Source: `src/udaf.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Whether the aggregate function is distinct.

```sql
SELECT COUNT(DISTINCT column1) FROM t;
```

<a id="op-54dec394da69e7326adf2153"></a>
## return_type

`struct_field` · `datafusion_expr::udaf::StatisticsArgs::return_type` · datafusion-expr 55.1.0

```rust
return_type: &'a arrow::datatypes::DataType
```

Source: `src/udaf.rs:111`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The resolved return type of the aggregate function

<a id="op-2cf216679cd700ec8021b8c5"></a>
## statistics

`struct_field` · `datafusion_expr::udaf::StatisticsArgs::statistics` · datafusion-expr 55.1.0

```rust
statistics: &'a datafusion_common::Statistics
```

Source: `src/udaf.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The statistics of the aggregate input
