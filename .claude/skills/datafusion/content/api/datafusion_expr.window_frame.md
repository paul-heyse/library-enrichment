# `datafusion_expr::window_frame`

Crate `datafusion-expr` · 3 public items · structured records in [`model/datafusion_expr.window_frame.json`](../model/datafusion_expr.window_frame.json)

## WindowFrameBound

`enum` · `datafusion_expr::window_frame::WindowFrameBound`

Also reachable as `datafusion::logical_expr::WindowFrameBound`, `datafusion_expr::WindowFrameBound`

```rust
enum WindowFrameBound
```

**Variants**: `Preceding`, `CurrentRow`, `Following`

**Implements**: `core::convert::TryFrom`, `core::fmt::Display`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (1)

```rust
fn is_unbounded(&self) -> bool
```

**via `core::convert::TryFrom`**

```rust
fn try_from(bound: protobuf::WindowFrameBound) -> Result<Self, Self::Error>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.window_frame.WindowFrameBound.md).


There are five ways to describe starting and ending frame boundaries:

1. UNBOUNDED PRECEDING
2. `<expr>` PRECEDING
3. CURRENT ROW
4. `<expr>` FOLLOWING
5. UNBOUNDED FOLLOWING

---

## WindowFrameUnits

`enum` · `datafusion_expr::window_frame::WindowFrameUnits`

Also reachable as `datafusion::logical_expr::WindowFrameUnits`, `datafusion_expr::WindowFrameUnits`

```rust
enum WindowFrameUnits
```

**Variants**: `Rows`, `Range`, `Groups`

**Implements**: `core::convert::From`, `core::fmt::Display`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**via `core::convert::From`**

```rust
fn from(value: ast::WindowFrameUnits) -> Self
fn from(units: protobuf::WindowFrameUnits) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.window_frame.WindowFrameUnits.md).


There are three frame types: ROWS, GROUPS, and RANGE. The frame type determines how the
starting and ending boundaries of the frame are measured.

---

## WindowFrame

`struct` · `datafusion_expr::window_frame::WindowFrame`

Also reachable as `datafusion::logical_expr::WindowFrame`, `datafusion_expr::WindowFrame`

```rust
struct WindowFrame
```

**Fields**: `units`, `start_bound`, `end_bound`

**Implements**: `core::convert::TryFrom`, `core::fmt::Display`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (7)

```rust
fn can_accept_multi_orderby(&self) -> bool
fn is_causal(&self) -> bool
fn is_ever_expanding(&self) -> bool
fn new(order_by: Option<bool>) -> Self
fn new_bounds(units: WindowFrameUnits, start_bound: WindowFrameBound, end_bound: WindowFrameBound) -> Self
fn regularize_order_bys(&self, order_by: &mut Vec<Sort>) -> Result<()>
fn reverse(&self) -> Self
```

**via `core::convert::TryFrom`**

```rust
fn try_from(value: ast::WindowFrame) -> Result<Self>
fn try_from(window: protobuf::WindowFrame) -> Result<Self, Self::Error>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

[Full member, field, variant and typed contracts](../operations/datafusion_expr.window_frame.WindowFrame.md).


The frame specification determines which output rows are read by an aggregate
window function. The ending frame boundary can be omitted if the `BETWEEN`
and `AND` keywords that surround the starting frame boundary are also omitted,
in which case the ending frame boundary defaults to `CURRENT ROW`.

---
