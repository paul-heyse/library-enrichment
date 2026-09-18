# `datafusion_optimizer::optimizer::OptimizerContext`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_optimizer.optimizer.OptimizerContext.json).

<a id="op-0f77aeae46bd0c79cea0807f"></a>
## OptimizerContext

`struct` · `datafusion_optimizer::optimizer::OptimizerContext` · datafusion-optimizer 55.1.0

```rust
struct OptimizerContext
```

Source: `src/optimizer.rs:164`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

A standalone [`OptimizerConfig`](../operations/datafusion_optimizer.optimizer.OptimizerConfig.md#op-c99ba442ba62b2c8360b9201) that can be used independently
of DataFusion's config management

<a id="op-9f2f49614634a8f6058d675f"></a>
## alias_generator

`function` · `datafusion_optimizer::optimizer::OptimizerContext::alias_generator` · datafusion-optimizer 55.1.0

```rust
fn alias_generator(&self) -> &Arc<AliasGenerator>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerContext", "path": "OptimizerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [239, 1], "end": [251, 2], "filename": "src/optimizer.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerConfig", "path": "OptimizerConfig"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerConfig"}`

Source: `src/optimizer.rs:244`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-812ac468064d2f4f2b4cfaeb"></a>
## default

`function` · `datafusion_optimizer::optimizer::OptimizerContext::default` · datafusion-optimizer 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerContext", "path": "OptimizerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [232, 1], "end": [237, 2], "filename": "src/optimizer.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/optimizer.rs:234`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Create optimizer config

<a id="op-16e0b591af8dfdeddfa40363"></a>
## filter_null_keys

`function` · `datafusion_optimizer::optimizer::OptimizerContext::filter_null_keys` · datafusion-optimizer 55.1.0

```rust
fn filter_null_keys(self, filter_null_keys: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerContext", "path": "OptimizerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [176, 1], "end": [230, 2], "filename": "src/optimizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/optimizer.rs:195`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Specify whether to enable the filter_null_keys rule

<a id="op-5ee83486f3077504b2e6af79"></a>
## fmt

`function` · `datafusion_optimizer::optimizer::OptimizerContext::fmt` · datafusion-optimizer 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerContext", "path": "OptimizerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [163, 10], "end": [163, 15], "filename": "src/optimizer.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/optimizer.rs:163`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c1cd3bdaaa97518f071c10b4"></a>
## new

`function` · `datafusion_optimizer::optimizer::OptimizerContext::new` · datafusion-optimizer 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerContext", "path": "OptimizerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [176, 1], "end": [230, 2], "filename": "src/optimizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/optimizer.rs:178`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Create optimizer config

<a id="op-488e01fb372fb6829086f609"></a>
## new_with_config_options

`function` · `datafusion_optimizer::optimizer::OptimizerContext::new_with_config_options` · datafusion-optimizer 55.1.0

```rust
fn new_with_config_options(options: Arc<ConfigOptions>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerContext", "path": "OptimizerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [176, 1], "end": [230, 2], "filename": "src/optimizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/optimizer.rs:186`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Create a optimizer config with provided [ConfigOptions](../operations/datafusion_common.config.ConfigOptions.md#op-0fede8afa640e38e337e0cc4).

<a id="op-a30839c545dd9fc870a46543"></a>
## options

`function` · `datafusion_optimizer::optimizer::OptimizerContext::options` · datafusion-optimizer 55.1.0

```rust
fn options(&self) -> Arc<ConfigOptions>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerContext", "path": "OptimizerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [239, 1], "end": [251, 2], "filename": "src/optimizer.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerConfig", "path": "OptimizerConfig"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerConfig"}`

Source: `src/optimizer.rs:248`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9186bd55e4e99108a30ac86"></a>
## query_execution_start_time

`function` · `datafusion_optimizer::optimizer::OptimizerContext::query_execution_start_time` · datafusion-optimizer 55.1.0

```rust
fn query_execution_start_time(&self) -> Option<DateTime<Utc>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerContext", "path": "OptimizerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [239, 1], "end": [251, 2], "filename": "src/optimizer.rs"}, "trait": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerConfig", "path": "OptimizerConfig"}, "trait_path": "datafusion_optimizer::optimizer::OptimizerConfig"}`

Source: `src/optimizer.rs:240`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d81fdcf70425df438394aeb"></a>
## with_max_passes

`function` · `datafusion_optimizer::optimizer::OptimizerContext::with_max_passes` · datafusion-optimizer 55.1.0

```rust
fn with_max_passes(self, v: u8) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerContext", "path": "OptimizerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [176, 1], "end": [230, 2], "filename": "src/optimizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/optimizer.rs:226`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Specify how many times to attempt to optimize the plan

<a id="op-6d857b27f470722e34ad53be"></a>
## with_query_execution_start_time

`function` · `datafusion_optimizer::optimizer::OptimizerContext::with_query_execution_start_time` · datafusion-optimizer 55.1.0

```rust
fn with_query_execution_start_time(self, query_execution_start_time: DateTime<Utc>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerContext", "path": "OptimizerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [176, 1], "end": [230, 2], "filename": "src/optimizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/optimizer.rs:203`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Set the query execution start time

<a id="op-6efee3477f096916240fc583"></a>
## with_skip_failing_rules

`function` · `datafusion_optimizer::optimizer::OptimizerContext::with_skip_failing_rules` · datafusion-optimizer 55.1.0

```rust
fn with_skip_failing_rules(self, b: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerContext", "path": "OptimizerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [176, 1], "end": [230, 2], "filename": "src/optimizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/optimizer.rs:220`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Specify whether the optimizer should skip rules that produce
errors, or fail the query

<a id="op-e89e36d0ae666b3129172d5a"></a>
## without_query_execution_start_time

`function` · `datafusion_optimizer::optimizer::OptimizerContext::without_query_execution_start_time` · datafusion-optimizer 55.1.0

```rust
fn without_query_execution_start_time(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_optimizer::optimizer::OptimizerContext", "path": "OptimizerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [176, 1], "end": [230, 2], "filename": "src/optimizer.rs"}, "trait": null, "trait_path": null}`

Source: `src/optimizer.rs:213`. [Exact documentation build](https://docs.rs/crate/datafusion-optimizer/55.1.0/json).

Clear the query execution start time. When `None`, time-dependent
functions like `now()` will not be simplified during optimization.
