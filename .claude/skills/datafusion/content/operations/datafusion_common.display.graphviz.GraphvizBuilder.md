# `datafusion_common::display::graphviz::GraphvizBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.display.graphviz.GraphvizBuilder.json).

<a id="op-6c9cd3bc480f44b154c3c2cc"></a>
## GraphvizBuilder

`struct` · `datafusion_common::display::graphviz::GraphvizBuilder` · datafusion-common 55.1.0

```rust
struct GraphvizBuilder
```

Source: `src/display/graphviz.rs:23`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d936394e16a3438abcdc1099"></a>
## add_edge

`function` · `datafusion_common::display::graphviz::GraphvizBuilder::add_edge` · datafusion-common 55.1.0

```rust
fn add_edge(&self, f: &mut fmt::Formatter<'_>, from_id: usize, to_id: usize) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::display::graphviz::GraphvizBuilder", "path": "GraphvizBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [105, 2], "filename": "src/display/graphviz.rs"}, "trait": null, "trait_path": null}`

Source: `src/display/graphviz.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aabe386ff3a75e1a36ee9934"></a>
## add_node

`function` · `datafusion_common::display::graphviz::GraphvizBuilder::add_node` · datafusion-common 55.1.0

```rust
fn add_node(&self, f: &mut fmt::Formatter<'_>, id: usize, label: &str, tooltip: Option<&str>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::display::graphviz::GraphvizBuilder", "path": "GraphvizBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [105, 2], "filename": "src/display/graphviz.rs"}, "trait": null, "trait_path": null}`

Source: `src/display/graphviz.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bf37a3dc494cef471a3d3a79"></a>
## default

`function` · `datafusion_common::display::graphviz::GraphvizBuilder::default` · datafusion-common 55.1.0

```rust
fn default() -> GraphvizBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::display::graphviz::GraphvizBuilder", "path": "GraphvizBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 10], "end": [22, 17], "filename": "src/display/graphviz.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/display/graphviz.rs:22`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-341dd0790874a4b7e731f996"></a>
## end_cluster

`function` · `datafusion_common::display::graphviz::GraphvizBuilder::end_cluster` · datafusion-common 55.1.0

```rust
fn end_cluster(&mut self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::display::graphviz::GraphvizBuilder", "path": "GraphvizBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [105, 2], "filename": "src/display/graphviz.rs"}, "trait": null, "trait_path": null}`

Source: `src/display/graphviz.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b08b4b9124c98fbe4a389377"></a>
## end_graph

`function` · `datafusion_common::display::graphviz::GraphvizBuilder::end_graph` · datafusion-common 55.1.0

```rust
fn end_graph(&mut self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::display::graphviz::GraphvizBuilder", "path": "GraphvizBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [105, 2], "filename": "src/display/graphviz.rs"}, "trait": null, "trait_path": null}`

Source: `src/display/graphviz.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80f1022e5a59402caab8deb2"></a>
## next_id

`function` · `datafusion_common::display::graphviz::GraphvizBuilder::next_id` · datafusion-common 55.1.0

```rust
fn next_id(&mut self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::display::graphviz::GraphvizBuilder", "path": "GraphvizBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [105, 2], "filename": "src/display/graphviz.rs"}, "trait": null, "trait_path": null}`

Source: `src/display/graphviz.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48748c226483dedcc966e870"></a>
## quoted

`function` · `datafusion_common::display::graphviz::GraphvizBuilder::quoted` · datafusion-common 55.1.0

```rust
fn quoted(label: &str) -> String
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::display::graphviz::GraphvizBuilder", "path": "GraphvizBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [105, 2], "filename": "src/display/graphviz.rs"}, "trait": null, "trait_path": null}`

Source: `src/display/graphviz.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

makes a quoted string suitable for inclusion in a graphviz chart

<a id="op-f56427b61f8b885d3c514a28"></a>
## start_cluster

`function` · `datafusion_common::display::graphviz::GraphvizBuilder::start_cluster` · datafusion-common 55.1.0

```rust
fn start_cluster(&mut self, f: &mut fmt::Formatter<'_>, title: &str) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::display::graphviz::GraphvizBuilder", "path": "GraphvizBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [105, 2], "filename": "src/display/graphviz.rs"}, "trait": null, "trait_path": null}`

Source: `src/display/graphviz.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d54a490bdcf8fed6f6665a0"></a>
## start_graph

`function` · `datafusion_common::display::graphviz::GraphvizBuilder::start_graph` · datafusion-common 55.1.0

```rust
fn start_graph(&mut self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::display::graphviz::GraphvizBuilder", "path": "GraphvizBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [105, 2], "filename": "src/display/graphviz.rs"}, "trait": null, "trait_path": null}`

Source: `src/display/graphviz.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
