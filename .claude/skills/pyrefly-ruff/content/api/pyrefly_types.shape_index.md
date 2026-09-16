# `pyrefly_types::shape_index`

Crate `pyrefly_types` · 9 public items · structured records in [`model/pyrefly_types.shape_index.json`](../model/pyrefly_types.shape_index.json)

## IndexComponent

`enum` · `pyrefly_types::shape_index::IndexComponent`

```rust
enum IndexComponent
```

**Variants**: `Operation`, `Ellipsis`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

One component of a lowered indexing operation.

---

## IndexOp

`enum` · `pyrefly_types::shape_index::IndexOp`

```rust
enum IndexOp
```

**Variants**: `Int`, `Slice`, `ShapedArrayIndex`, `Fancy`, `NewAxis`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

A single index operation, pre-classified by the type checker.
The type checker resolves Expr nodes into these before calling shape functions.

---

## IndexTypeLowering

`enum` · `pyrefly_types::shape_index::IndexTypeLowering`

```rust
enum IndexTypeLowering
```

**Variants**: `Precise`, `Gradual`, `Invalid`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn is_valid(&self) -> bool
```

The syntax-free result of interpreting an ordinary type as an index value.

`Index`-restricted type parameters retain their ordinary `Type`; this result is created only
while validating a specialization or evaluating an indexing operation.

---

## index_shape_from_type

`function` · `pyrefly_types::shape_index::index_shape_from_type`

```rust
fn index_shape_from_type(shape: &shaped_array::IntTuple, index: &types::Type) -> Result<shaped_array::IntTuple, dimension::ShapeError>
```

Apply an ordinary retained index type to a shape.

The `Index` restriction validates this same lowering before specialization. A gradual lowering
produces a gradual shape, while algebra failures retain their indexing-specific diagnostic.

---

## index_shape_int

`function` · `pyrefly_types::shape_index::index_shape_int`

```rust
fn index_shape_int(shape: &shaped_array::IntTuple) -> Result<shaped_array::IntTuple, dimension::ShapeError>
```

Apply a single integer index — removes first dimension.
E.g. `Tensor[10, 20][i]` -> `Tensor[20]`

---

## index_shape_multi

`function` · `pyrefly_types::shape_index::index_shape_multi`

```rust
fn index_shape_multi(shape: &shaped_array::IntTuple, pre_ops: &[IndexOp], post_ops: &[IndexOp], has_ellipsis: bool) -> Result<shaped_array::IntTuple, dimension::ShapeError>
```

Apply multi-axis indexing with optional ellipsis.
`pre_ops` are applied left-to-right from dim 0.
`post_ops` are applied from the end (only when `has_ellipsis` is true).
Dims between pre and post (the ellipsis range) are preserved.

---

## index_shape_slice

`function` · `pyrefly_types::shape_index::index_shape_slice`

```rust
fn index_shape_slice(shape: &shaped_array::IntTuple, start: Option<dimension::Int>, stop: Option<dimension::Int>, step: Option<dimension::Int>) -> Result<shaped_array::IntTuple, dimension::ShapeError>
```

Apply a single slice to first dimension.
E.g. `Tensor[10, 20][2:5]` -> `Tensor[3, 20]`
With step: `Tensor[100][::2]` -> `Tensor[50]` (ceil_div(100, 2))

---

## index_shape_tensor

`function` · `pyrefly_types::shape_index::index_shape_tensor`

```rust
fn index_shape_tensor(shape: &shaped_array::IntTuple, idx_dims: &[dimension::Int]) -> Result<shaped_array::IntTuple, dimension::ShapeError>
```

Apply tensor-as-index — replaces first dim with index tensor's dims.
E.g. `Tensor[B, D1, D2][Tensor[T]]` -> `Tensor[T, D1, D2]`

---

## lower_index_type

`function` · `pyrefly_types::shape_index::lower_index_type`

```rust
fn lower_index_type(ty: &types::Type) -> IndexTypeLowering
```

Interpret an ordinary runtime type as the supported shape-index grammar.

A tuple at the root denotes several axis operations, while a tuple nested inside that root is
one integer-sequence operand. Unknown structure is accepted as gradual so typed fallback
overloads do not need a parallel representation for imprecise-but-valid indices.

---
