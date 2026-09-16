# `ruff_python_semantic::cfg::visualize`

Crate `ruff_python_semantic` · 6 public items · structured records in [`model/ruff_python_semantic.cfg.visualize.json`](../model/ruff_python_semantic.cfg.visualize.json)

## MermaidEdgeKind

`enum` · `ruff_python_semantic::cfg::visualize::MermaidEdgeKind`

```rust
enum MermaidEdgeKind
```

**Variants**: `Arrow`, `DottedArrow`, `ThickArrow`, `BidirectionalArrow`

**Implements**: `core::fmt::Display`

**Derives**: Debug, Default

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

---

## MermaidNodeShape

`enum` · `ruff_python_semantic::cfg::visualize::MermaidNodeShape`

```rust
enum MermaidNodeShape
```

**Variants**: `Rectangle`, `DoubleRectangle`, `RoundedRectangle`, `Stadium`, `Circle`, `DoubleCircle`, `Asymmetric`, `Rhombus`, `Hexagon`, `Parallelogram`, `Trapezoid`

**Derives**: Debug, Default

---

## draw_cfg

`function` · `ruff_python_semantic::cfg::visualize::draw_cfg`

```rust
fn draw_cfg(graph: cfg::graph::ControlFlowGraph<'_>, source: &str) -> String
```

Returns control flow graph in Mermaid syntax.

---

## MermaidEdge

`struct` · `ruff_python_semantic::cfg::visualize::MermaidEdge`

```rust
struct MermaidEdge
```

**Implements**: `core::fmt::Display`

**Derives**: Debug, Default

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

---

## MermaidNode

`struct` · `ruff_python_semantic::cfg::visualize::MermaidNode`

```rust
struct MermaidNode
```

**Implements**: `core::fmt::Display`

**Methods** (1)

```rust
fn with_content(content: String) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

---

## DirectedGraph

`trait` · `ruff_python_semantic::cfg::visualize::DirectedGraph`

```rust
trait DirectedGraph<'a>
```

**Methods** (3)

```rust
fn num_nodes(&self) -> usize
fn start_node(&self) -> Self::Node
fn successors(&self, node: Self::Node) -> impl ExactSizeIterator<Item = Self::Node> + '_
```

---
