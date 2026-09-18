# `datafusion_optimizer::unions_to_filter::UnionsToFilter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.unions_to_filter.UnionsToFilter.json).

<a id="op-c5264557fcf52859532c7efa"></a>
## UnionsToFilter

`struct` · `datafusion_optimizer::unions_to_filter::UnionsToFilter` · datafusion-optimizer 55.1.0

```rust
struct UnionsToFilter
```

Source: `src/unions_to_filter.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e57f7d3fede1460b739932b"></a>
## default

`function` · `datafusion_optimizer::unions_to_filter::UnionsToFilter::default` · datafusion-optimizer 55.1.0

```rust
fn default() -> UnionsToFilter
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::unions_to_filter::UnionsToFilter", "path": "UnionsToFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 10], "end": [33, 17], "filename": "src/unions_to_filter.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/unions_to_filter.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-08787352ec55b5a2221f27d0"></a>
## fmt

`function` · `datafusion_optimizer::unions_to_filter::UnionsToFilter::fmt` · datafusion-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::unions_to_filter::UnionsToFilter", "path": "UnionsToFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 19], "end": [33, 24], "filename": "src/unions_to_filter.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/unions_to_filter.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ece1b942ad46fb4deafbf05"></a>
## name

`function` · `datafusion_optimizer::unions_to_filter::UnionsToFilter::name` · datafusion-optimizer 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::unions_to_filter::UnionsToFilter", "path": "UnionsToFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [71, 2], "filename": "src/unions_to_filter.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/unions_to_filter.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b46a59df71e1fe87f9072bf7"></a>
## new

`function` · `datafusion_optimizer::unions_to_filter::UnionsToFilter::new` · datafusion-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::unions_to_filter::UnionsToFilter", "path": "UnionsToFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [41, 2], "filename": "src/unions_to_filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/unions_to_filter.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b52017d24e141a624fcd669"></a>
## rewrite

`function` · `datafusion_optimizer::unions_to_filter::UnionsToFilter::rewrite` · datafusion-optimizer 55.1.0

```rust
fn rewrite(&self, plan: LogicalPlan, config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::unions_to_filter::UnionsToFilter", "path": "UnionsToFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [71, 2], "filename": "src/unions_to_filter.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/unions_to_filter.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2844f857e279e4f9d7b291ba"></a>
## supports_rewrite

`function` · `datafusion_optimizer::unions_to_filter::UnionsToFilter::supports_rewrite` · datafusion-optimizer 55.1.0

```rust
fn supports_rewrite(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::unions_to_filter::UnionsToFilter", "path": "UnionsToFilter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 1], "end": [71, 2], "filename": "src/unions_to_filter.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerRule", "path": "OptimizerRule"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerRule"}`

Source: `src/unions_to_filter.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
