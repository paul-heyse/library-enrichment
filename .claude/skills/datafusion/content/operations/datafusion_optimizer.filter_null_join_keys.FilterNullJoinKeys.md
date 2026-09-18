# `datafusion_optimizer::filter_null_join_keys::FilterNullJoinKeys`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.filter_null_join_keys.FilterNullJoinKeys.json).

<a id="op-e37e7f05e219d9a309f827b9"></a>
## FilterNullJoinKeys

`struct` · `datafusion_optimizer::filter_null_join_keys::FilterNullJoinKeys` · datafusion-optimizer 55.1.0

```rust
struct FilterNullJoinKeys
```

Source: `src/filter_null_join_keys.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

The FilterNullJoinKeys rule will identify joins with equi-join conditions
where the join key is nullable and then insert an `IsNotNull` filter on the nullable side since null values
can never match.

<a id="op-20c2f2f1cfffc4d2b53ec340"></a>
## apply_order

`function` · `datafusion_optimizer::filter_null_join_keys::FilterNullJoinKeys::apply_order` · datafusion-optimizer 55.1.0

```rust
fn apply_order(&self) -> Option<ApplyOrder>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::filter_null_join_keys::FilterNullJoinKeys", "path": "FilterNullJoinKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [97, 2], "filename": "src/filter_null_join_keys.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/filter_null_join_keys.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f234442895d89659e955d856"></a>
## default

`function` · `datafusion_optimizer::filter_null_join_keys::FilterNullJoinKeys::default` · datafusion-optimizer 55.1.0

```rust
fn default() -> FilterNullJoinKeys
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::filter_null_join_keys::FilterNullJoinKeys", "path": "FilterNullJoinKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 10], "end": [32, 17], "filename": "src/filter_null_join_keys.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/filter_null_join_keys.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-982a093a2bfff8fc6054cc0e"></a>
## fmt

`function` · `datafusion_optimizer::filter_null_join_keys::FilterNullJoinKeys::fmt` · datafusion-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::filter_null_join_keys::FilterNullJoinKeys", "path": "FilterNullJoinKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [32, 19], "end": [32, 24], "filename": "src/filter_null_join_keys.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/filter_null_join_keys.rs:32`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-208526f60040ae59edb85c9d"></a>
## name

`function` · `datafusion_optimizer::filter_null_join_keys::FilterNullJoinKeys::name` · datafusion-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::filter_null_join_keys::FilterNullJoinKeys", "path": "FilterNullJoinKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [97, 2], "filename": "src/filter_null_join_keys.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/filter_null_join_keys.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-156ea26fcb3412383aaa185b"></a>
## rewrite

`function` · `datafusion_optimizer::filter_null_join_keys::FilterNullJoinKeys::rewrite` · datafusion-optimizer 55.1.0

```rust
fn rewrite(&self, plan: LogicalPlan, config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::filter_null_join_keys::FilterNullJoinKeys", "path": "FilterNullJoinKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [97, 2], "filename": "src/filter_null_join_keys.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/filter_null_join_keys.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e42632d3058d52b5625f00be"></a>
## supports_rewrite

`function` · `datafusion_optimizer::filter_null_join_keys::FilterNullJoinKeys::supports_rewrite` · datafusion-optimizer 55.1.0

```rust
fn supports_rewrite(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::filter_null_join_keys::FilterNullJoinKeys", "path": "FilterNullJoinKeys"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 1], "end": [97, 2], "filename": "src/filter_null_join_keys.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/filter_null_join_keys.rs:36`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
