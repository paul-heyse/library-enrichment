# `datafusion_common::display::graphviz`

Crate `datafusion-common` · 1 public items · structured records in [`model/datafusion_common.display.graphviz.json`](../model/datafusion_common.display.graphviz.json)

## GraphvizBuilder

`struct` · `datafusion_common::display::graphviz::GraphvizBuilder`

```rust
struct GraphvizBuilder
```

**Derives**: Default

**Methods** (8)

```rust
fn add_edge(&self, f: &mut fmt::Formatter<'_>, from_id: usize, to_id: usize) -> fmt::Result
fn add_node(&self, f: &mut fmt::Formatter<'_>, id: usize, label: &str, tooltip: Option<&str>) -> fmt::Result
fn end_cluster(&mut self, f: &mut fmt::Formatter<'_>) -> fmt::Result
fn end_graph(&mut self, f: &mut fmt::Formatter<'_>) -> fmt::Result
fn next_id(&mut self) -> usize
fn quoted(label: &str) -> String
fn start_cluster(&mut self, f: &mut fmt::Formatter<'_>, title: &str) -> fmt::Result
fn start_graph(&mut self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

---
