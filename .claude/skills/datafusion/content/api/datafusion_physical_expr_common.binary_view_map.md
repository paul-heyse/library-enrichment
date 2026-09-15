# `datafusion_physical_expr_common::binary_view_map`

Crate `datafusion-physical-expr-common` · 2 public items · structured records in [`model/datafusion_physical_expr_common.binary_view_map.json`](../model/datafusion_physical_expr_common.binary_view_map.json)

## ArrowBytesViewMap

`struct` · `datafusion_physical_expr_common::binary_view_map::ArrowBytesViewMap`

```rust
struct ArrowBytesViewMap<V> where V: Debug + PartialEq + Eq + Clone + Copy + Default
```

**Derives**: Debug

**Methods** (8)

```rust
fn insert_if_new<MP, OP>(&mut self, values: &ArrayRef, make_payload_fn: MP, observe_payload_fn: OP) where MP: FnMut(Option<&[u8]>) -> V, OP: FnMut(V)
fn into_state(self) -> ArrayRef
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn new(output_type: OutputType) -> Self
fn non_null_len(&self) -> usize
fn size(&self) -> usize
fn take(&mut self) -> Self
```

---

## ArrowBytesViewSet

`struct` · `datafusion_physical_expr_common::binary_view_map::ArrowBytesViewSet`

```rust
struct ArrowBytesViewSet
```

**Derives**: Debug

**Methods** (8)

```rust
fn insert(&mut self, values: &ArrayRef)
fn into_state(self) -> ArrayRef
fn is_empty(&self) -> bool
fn len(&self) -> usize
fn new(output_type: OutputType) -> Self
fn non_null_len(&self) -> usize
fn size(&self) -> usize
fn take(&mut self) -> Self
```

HashSet optimized for storing string or binary values that can produce that
the final set as a `GenericBinaryViewArray` with minimal copies.

---
