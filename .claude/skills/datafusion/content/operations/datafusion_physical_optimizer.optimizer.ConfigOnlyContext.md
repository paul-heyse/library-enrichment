# `datafusion_physical_optimizer::optimizer::ConfigOnlyContext`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_optimizer.optimizer.ConfigOnlyContext.json).

<a id="op-92274dde692828eacdcaa83f"></a>
## ConfigOnlyContext

`struct` · `datafusion_physical_optimizer::optimizer::ConfigOnlyContext` · datafusion-physical-optimizer 55.1.0

```rust
struct ConfigOnlyContext<'a>
```

Source: `src/optimizer.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Simple context wrapping [`ConfigOptions`](../operations/datafusion_common.config.ConfigOptions.md#op-0fede8afa640e38e337e0cc4) for backward compatibility.

This struct provides a minimal implementation of [`PhysicalOptimizerContext`](../operations/datafusion_session.physical_optimizer.PhysicalOptimizerContext.md#op-3296df92ae1d4db86475371d)
that only supplies configuration options. Used when no statistics registry
is available or needed.

<a id="op-a1eb70b97dd211e4ec92b358"></a>
## config_options

`function` · `datafusion_physical_optimizer::optimizer::ConfigOnlyContext::config_options` · datafusion-physical-optimizer 55.1.0

```rust
fn config_options(&self) -> &ConfigOptions
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "datafusion_physical_optimizer::optimizer::ConfigOnlyContext", "path": "ConfigOnlyContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [67, 2], "filename": "src/optimizer.rs"}, "trait": {"args": null, "id": "datafusion_session::physical_optimizer::PhysicalOptimizerContext", "path": "PhysicalOptimizerContext"}, "trait_path": "datafusion_session::physical_optimizer::PhysicalOptimizerContext"}`

Source: `src/optimizer.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62a95dbc1b284b9036acf6a4"></a>
## new

`function` · `datafusion_physical_optimizer::optimizer::ConfigOnlyContext::new` · datafusion-physical-optimizer 55.1.0

```rust
fn new(config: &'a ConfigOptions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_optimizer::optimizer::ConfigOnlyContext", "path": "ConfigOnlyContext"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [61, 2], "filename": "src/optimizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/optimizer.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-optimizer/55.1.0/json).

Create a new context wrapping the given config options.
