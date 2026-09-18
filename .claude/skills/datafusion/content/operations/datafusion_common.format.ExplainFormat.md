# `datafusion_common::format::ExplainFormat`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.format.ExplainFormat.json).

<a id="op-76c1d500e6c018743f6cefab"></a>
## ExplainFormat

`enum` · `datafusion_common::format::ExplainFormat` · datafusion-common 55.1.0

```rust
enum ExplainFormat
```

Source: `src/format.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Output formats for controlling for Explain plans

<a id="op-96697fd59bb0719020e7b8ed"></a>
## Err

`assoc_type` · `datafusion_common::format::ExplainFormat::Err` · datafusion-common 55.1.0

```rust
Err
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::ExplainFormat", "path": "ExplainFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [172, 1], "end": [186, 2], "filename": "src/format.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/format.rs:173`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ddc67cdbb8983898f071f444"></a>
## Graphviz

`variant` · `datafusion_common::format::ExplainFormat::Graphviz` · datafusion-common 55.1.0

```rust
Graphviz
```

Source: `src/format.rs:168`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Graphviz mode

Example:
```text
> explain format graphviz select x from values (1) t(x);
+--------------+------------------------------------------------------------------------+
| plan_type    | plan                                                                   |
+--------------+------------------------------------------------------------------------+
| logical_plan |                                                                        |
|              | // Begin DataFusion GraphViz Plan,                                     |
|              | // display it online here: https://dreampuf.github.io/GraphvizOnline   |
|              |                                                                        |
|              | digraph {                                                              |
|              |   subgraph cluster_1                                                   |
|              |   {                                                                    |
|              |     graph[label="LogicalPlan"]                                         |
|              |     2[shape=box label="SubqueryAlias: t"]                              |
|              |     3[shape=box label="Projection: column1 AS x"]                      |
|              |     2 -> 3 [arrowhead=none, arrowtail=normal, dir=back]                |
|              |     4[shape=box label="Values: (Int64(1))"]                            |
|              |     3 -> 4 [arrowhead=none, arrowtail=normal, dir=back]                |
|              |   }                                                                    |
|              |   subgraph cluster_5                                                   |
|              |   {                                                                    |
|              |     graph[label="Detailed LogicalPlan"]                                |
|              |     6[shape=box label="SubqueryAlias: t\nSchema: [x:Int64;N]"]         |
|              |     7[shape=box label="Projection: column1 AS x\nSchema: [x:Int64;N]"] |
|              |     6 -> 7 [arrowhead=none, arrowtail=normal, dir=back]                |
|              |     8[shape=box label="Values: (Int64(1))\nSchema: [column1:Int64;N]"] |
|              |     7 -> 8 [arrowhead=none, arrowtail=normal, dir=back]                |
|              |   }                                                                    |
|              | }                                                                      |
|              | // End DataFusion GraphViz Plan                                        |
|              |                                                                        |
+--------------+------------------------------------------------------------------------+
```

<a id="op-6d04e6ce0b797d4437b38ed2"></a>
## Indent

`variant` · `datafusion_common::format::ExplainFormat::Indent` · datafusion-common 55.1.0

```rust
Indent
```

Source: `src/format.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Indent mode

Example:
```text
> explain format indent select x from values (1) t(x);
+---------------+-----------------------------------------------------+
| plan_type     | plan                                                |
+---------------+-----------------------------------------------------+
| logical_plan  | SubqueryAlias: t                                    |
|               |   Projection: column1 AS x                          |
|               |     Values: (Int64(1))                              |
| physical_plan | ProjectionExec: expr=[column1@0 as x]               |
|               |   DataSourceExec: partitions=1, partition_sizes=[1] |
|               |                                                     |
+---------------+-----------------------------------------------------+
```

<a id="op-cf9b638ced5156d95419792b"></a>
## PostgresJSON

`variant` · `datafusion_common::format::ExplainFormat::PostgresJSON` · datafusion-common 55.1.0

```rust
PostgresJSON
```

Source: `src/format.rs:131`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Postgres Json mode

A displayable structure that produces plan in postgresql JSON format.

Users can use this format to visualize the plan in existing plan
visualization tools, for example [dalibo](https://explain.dalibo.com/)

Example:
```text
> explain format pgjson select x from values (1) t(x);
+--------------+--------------------------------------+
| plan_type    | plan                                 |
+--------------+--------------------------------------+
| logical_plan | [                                    |
|              |   {                                  |
|              |     "Plan": {                        |
|              |       "Alias": "t",                  |
|              |       "Node Type": "Subquery",       |
|              |       "Output": [                    |
|              |         "x"                          |
|              |       ],                             |
|              |       "Plans": [                     |
|              |         {                            |
|              |           "Expressions": [           |
|              |             "column1 AS x"           |
|              |           ],                         |
|              |           "Node Type": "Projection", |
|              |           "Output": [                |
|              |             "x"                      |
|              |           ],                         |
|              |           "Plans": [                 |
|              |             {                        |
|              |               "Node Type": "Values", |
|              |               "Output": [            |
|              |                 "column1"            |
|              |               ],                     |
|              |               "Plans": [],           |
|              |               "Values": "(Int64(1))" |
|              |             }                        |
|              |           ]                          |
|              |         }                            |
|              |       ]                              |
|              |     }                                |
|              |   }                                  |
|              | ]                                    |
+--------------+--------------------------------------+
```

<a id="op-f03072aa4f7f87c25352db0d"></a>
## Tree

`variant` · `datafusion_common::format::ExplainFormat::Tree` · datafusion-common 55.1.0

```rust
Tree
```

Source: `src/format.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Tree mode

Example:
```text
> explain format tree select x from values (1) t(x);
+---------------+-------------------------------+
| plan_type     | plan                          |
+---------------+-------------------------------+
| physical_plan | ┌───────────────────────────┐ |
|               | │       ProjectionExec      │ |
|               | │    --------------------   │ |
|               | │        x: column1@0       │ |
|               | └─────────────┬─────────────┘ |
|               | ┌─────────────┴─────────────┐ |
|               | │       DataSourceExec      │ |
|               | │    --------------------   │ |
|               | │         bytes: 128        │ |
|               | │       format: memory      │ |
|               | │          rows: 1          │ |
|               | └───────────────────────────┘ |
|               |                               |
+---------------+-------------------------------+
```

<a id="op-2a26f1f6a89fb378eb6a0b47"></a>
## clone

`function` · `datafusion_common::format::ExplainFormat::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> ExplainFormat
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::ExplainFormat", "path": "ExplainFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 17], "end": [41, 22], "filename": "src/format.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/format.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be2b75d3627ec6702fbb86b1"></a>
## eq

`function` · `datafusion_common::format::ExplainFormat::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &ExplainFormat) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::ExplainFormat", "path": "ExplainFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 24], "end": [41, 33], "filename": "src/format.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/format.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13c92d362bfc343994d1fa13"></a>
## fmt

`function` · `datafusion_common::format::ExplainFormat::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::ExplainFormat", "path": "ExplainFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 10], "end": [41, 15], "filename": "src/format.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/format.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68add35425e693f44d1d0314"></a>
## fmt

`function` · `datafusion_common::format::ExplainFormat::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::ExplainFormat", "path": "ExplainFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [198, 2], "filename": "src/format.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/format.rs:189`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-607acda45b9e32e923cb117e"></a>
## from_str

`function` · `datafusion_common::format::ExplainFormat::from_str` · datafusion-common 55.1.0

```rust
fn from_str(format: &str) -> Result<Self, Self::Err>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::ExplainFormat", "path": "ExplainFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [172, 1], "end": [186, 2], "filename": "src/format.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/format.rs:175`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa7e43069fc4a7ba49cdeb62"></a>
## hash

`function` · `datafusion_common::format::ExplainFormat::hash` · datafusion-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::ExplainFormat", "path": "ExplainFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 39], "end": [41, 43], "filename": "src/format.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/format.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a6170d7ae622f2b88ec2149a"></a>
## set

`function` · `datafusion_common::format::ExplainFormat::set` · datafusion-common 55.1.0

```rust
fn set(&mut self, _: &str, value: &str) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::ExplainFormat", "path": "ExplainFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [200, 1], "end": [209, 2], "filename": "src/format.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/format.rs:205`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-362176ab3a4808d5b3031dbc"></a>
## visit

`function` · `datafusion_common::format::ExplainFormat::visit` · datafusion-common 55.1.0

```rust
fn visit<V: Visit>(&self, v: &mut V, key: &str, description: &'static str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::format::ExplainFormat", "path": "ExplainFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [200, 1], "end": [209, 2], "filename": "src/format.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/format.rs:201`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
