# `datafusion_optimizer::single_distinct_to_groupby::SingleDistinctToGroupBy`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.single_distinct_to_groupby.SingleDistinctToGroupBy.json).

<a id="op-924b1ec5018a1724c5f02bed"></a>
## SingleDistinctToGroupBy

`struct` · `datafusion_optimizer::single_distinct_to_groupby::SingleDistinctToGroupBy` · datafusion-optimizer 55.1.0

```rust
struct SingleDistinctToGroupBy
```

Source: `src/single_distinct_to_groupby.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

single distinct to group by optimizer rule
 ```text
   Before:
   SELECT a, count(DISTINCT b), sum(c)
   FROM t
   GROUP BY a

   After:
   SELECT a, count(alias1), sum(alias2)
   FROM (
     SELECT a, b as alias1, sum(c) as alias2
     FROM t
     GROUP BY a, b
   )
   GROUP BY a
 ```

<a id="op-2c825c52cde05ce876dcb616"></a>
## apply_order

`function` · `datafusion_optimizer::single_distinct_to_groupby::SingleDistinctToGroupBy::apply_order` · datafusion-optimizer 55.1.0

```rust
fn apply_order(&self) -> Option<ApplyOrder>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::single_distinct_to_groupby::SingleDistinctToGroupBy", "path": "SingleDistinctToGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [287, 2], "filename": "src/single_distinct_to_groupby.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/single_distinct_to_groupby.rs:112`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a3f960a54bcce3e2f8dfdf1"></a>
## default

`function` · `datafusion_optimizer::single_distinct_to_groupby::SingleDistinctToGroupBy::default` · datafusion-optimizer 55.1.0

```rust
fn default() -> SingleDistinctToGroupBy
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::single_distinct_to_groupby::SingleDistinctToGroupBy", "path": "SingleDistinctToGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 10], "end": [52, 17], "filename": "src/single_distinct_to_groupby.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/single_distinct_to_groupby.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c79ae0b58119d6b216fa4ead"></a>
## fmt

`function` · `datafusion_optimizer::single_distinct_to_groupby::SingleDistinctToGroupBy::fmt` · datafusion-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::single_distinct_to_groupby::SingleDistinctToGroupBy", "path": "SingleDistinctToGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 19], "end": [52, 24], "filename": "src/single_distinct_to_groupby.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/single_distinct_to_groupby.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47afc4bce7ceadcc18e82ab1"></a>
## name

`function` · `datafusion_optimizer::single_distinct_to_groupby::SingleDistinctToGroupBy::name` · datafusion-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::single_distinct_to_groupby::SingleDistinctToGroupBy", "path": "SingleDistinctToGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [287, 2], "filename": "src/single_distinct_to_groupby.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/single_distinct_to_groupby.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3505c2c5058f41b0115754d"></a>
## new

`function` · `datafusion_optimizer::single_distinct_to_groupby::SingleDistinctToGroupBy::new` · datafusion-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::single_distinct_to_groupby::SingleDistinctToGroupBy", "path": "SingleDistinctToGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [62, 2], "filename": "src/single_distinct_to_groupby.rs"}, "trait": null, "trait_path": null}`

Source: `src/single_distinct_to_groupby.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b763528978bc31631960d8c"></a>
## rewrite

`function` · `datafusion_optimizer::single_distinct_to_groupby::SingleDistinctToGroupBy::rewrite` · datafusion-optimizer 55.1.0

```rust
fn rewrite(&self, plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>, DataFusionError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::single_distinct_to_groupby::SingleDistinctToGroupBy", "path": "SingleDistinctToGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [287, 2], "filename": "src/single_distinct_to_groupby.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/single_distinct_to_groupby.rs:120`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e18a51138bfd46bfd15ee3f"></a>
## supports_rewrite

`function` · `datafusion_optimizer::single_distinct_to_groupby::SingleDistinctToGroupBy::supports_rewrite` · datafusion-optimizer 55.1.0

```rust
fn supports_rewrite(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::single_distinct_to_groupby::SingleDistinctToGroupBy", "path": "SingleDistinctToGroupBy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [287, 2], "filename": "src/single_distinct_to_groupby.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/single_distinct_to_groupby.rs:116`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
