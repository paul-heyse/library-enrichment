# `datafusion_common::config::ExplainOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config.ExplainOptions.json).

<a id="op-2ae1c989543f41b4e318de4d"></a>
## ExplainOptions

`struct` · `datafusion_common::config::ExplainOptions` · datafusion-common 55.1.0

```rust
struct ExplainOptions
```

Source: `src/config.rs:1771`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Options controlling explain output

See also: [`SessionConfig`]

[`SessionConfig`]: https://docs.rs/datafusion/latest/datafusion/prelude/struct.SessionConfig.html

<a id="op-41416495862c7c7b7a0267db"></a>
## analyze_categories

`struct_field` · `datafusion_common::config::ExplainOptions::analyze_categories` · datafusion-common 55.1.0

```rust
analyze_categories: format::ExplainAnalyzeCategories
```

Source: `src/config.rs:1771`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Which metric categories to include in "EXPLAIN ANALYZE" output.
Comma-separated list of: "rows", "bytes", "timing", "uncategorized".
Use "none" to show plan structure only, or "all" (default) to show everything.
Metrics without a declared category are treated as "uncategorized".

<a id="op-9ad9eb2c5300b140fc7432a0"></a>
## analyze_level

`struct_field` · `datafusion_common::config::ExplainOptions::analyze_level` · datafusion-common 55.1.0

```rust
analyze_level: format::MetricType
```

Source: `src/config.rs:1771`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Verbosity level for "EXPLAIN ANALYZE". Default is "dev"
"summary" shows common metrics for high-level insights.
"dev" provides deep operator-level introspection for developers.

<a id="op-f5da8db88049953c2c4133af"></a>
## clone

`function` · `datafusion_common::config::ExplainOptions::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> ExplainOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ExplainOptions", "path": "ExplainOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1771, 1], "end": [1813, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/config.rs:1771`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bad5f58596102bd07330f320"></a>
## default

`function` · `datafusion_common::config::ExplainOptions::default` · datafusion-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ExplainOptions", "path": "ExplainOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1771, 1], "end": [1813, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/config.rs:1771`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-58fe5f3c1e476ae2c72cb29d"></a>
## eq

`function` · `datafusion_common::config::ExplainOptions::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &ExplainOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ExplainOptions", "path": "ExplainOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1771, 1], "end": [1813, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/config.rs:1771`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de8dcd09c3cbeb954d801167"></a>
## fmt

`function` · `datafusion_common::config::ExplainOptions::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ExplainOptions", "path": "ExplainOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1771, 1], "end": [1813, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/config.rs:1771`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a92b2d010a0ed55270e896e8"></a>
## format

`struct_field` · `datafusion_common::config::ExplainOptions::format` · datafusion-common 55.1.0

```rust
format: format::ExplainFormat
```

Source: `src/config.rs:1771`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Display format of explain. Default is "indent".
When set to "tree", it will print the plan in a tree-rendered format.

<a id="op-191d53876f2005959a21bf6c"></a>
## logical_plan_only

`struct_field` · `datafusion_common::config::ExplainOptions::logical_plan_only` · datafusion-common 55.1.0

```rust
logical_plan_only: bool
```

Source: `src/config.rs:1771`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When set to true, the explain statement will only print logical plans

<a id="op-732bd54aa675aa3f93531760"></a>
## physical_plan_only

`struct_field` · `datafusion_common::config::ExplainOptions::physical_plan_only` · datafusion-common 55.1.0

```rust
physical_plan_only: bool
```

Source: `src/config.rs:1771`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When set to true, the explain statement will only print physical plans

<a id="op-8a2a46b4febe7f2f4645a69a"></a>
## reset

`function` · `datafusion_common::config::ExplainOptions::reset` · datafusion-common 55.1.0

```rust
fn reset(&mut self, key: &str) -> error::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ExplainOptions", "path": "ExplainOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1771, 1], "end": [1813, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:1771`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b4e6ef63ac5e689d5e2b2c5"></a>
## set

`function` · `datafusion_common::config::ExplainOptions::set` · datafusion-common 55.1.0

```rust
fn set(&mut self, key: &str, value: &str) -> error::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ExplainOptions", "path": "ExplainOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1771, 1], "end": [1813, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:1771`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-29cc039a71fc133502638f32"></a>
## show_schema

`struct_field` · `datafusion_common::config::ExplainOptions::show_schema` · datafusion-common 55.1.0

```rust
show_schema: bool
```

Source: `src/config.rs:1771`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When set to true, the explain statement will print schema information

<a id="op-d09db5078cd9fe79d04113e5"></a>
## show_sizes

`struct_field` · `datafusion_common::config::ExplainOptions::show_sizes` · datafusion-common 55.1.0

```rust
show_sizes: bool
```

Source: `src/config.rs:1771`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When set to true, the explain statement will print the partition sizes

<a id="op-ae22b45989ce722a5964b877"></a>
## show_statistics

`struct_field` · `datafusion_common::config::ExplainOptions::show_statistics` · datafusion-common 55.1.0

```rust
show_statistics: bool
```

Source: `src/config.rs:1771`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

When set to true, the explain statement will print operator statistics
for physical plans

<a id="op-98a1b4af6553867b7dbb6d79"></a>
## tree_maximum_render_width

`struct_field` · `datafusion_common::config::ExplainOptions::tree_maximum_render_width` · datafusion-common 55.1.0

```rust
tree_maximum_render_width: usize
```

Source: `src/config.rs:1771`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

(format=tree only) Maximum total width of the rendered tree.
When set to 0, the tree will have no width limit.

<a id="op-fa4ade92b6e1a9338a258da7"></a>
## visit

`function` · `datafusion_common::config::ExplainOptions::visit` · datafusion-common 55.1.0

```rust
fn visit<V: config::Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ExplainOptions", "path": "ExplainOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1771, 1], "end": [1813, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:1771`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
