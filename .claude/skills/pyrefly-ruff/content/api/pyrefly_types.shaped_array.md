# `pyrefly_types::shaped_array`

Crate `pyrefly_types` · 11 public items · structured records in [`model/pyrefly_types.shaped_array.json`](../model/pyrefly_types.shaped_array.json)

## IntTupleView

`enum` · `pyrefly_types::shaped_array::IntTupleView`

```rust
enum IntTupleView<'a>
```

**Variants**: `Concrete`, `Gradual`, `Unpacked`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

---

## ShapedArraySyntax

`enum` · `pyrefly_types::shaped_array::ShapedArraySyntax`

```rust
enum ShapedArraySyntax
```

**Variants**: `Native`, `Jaxtyping`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, f: &mut dyn FnMut(&'a To))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut To))
```

Whether a shaped-array type was constructed using native (`Tensor[N, M]`) or
jaxtyping (`Float[Tensor, "N M"]`) syntax. Controls display rendering and
enables diagnostic checks (e.g., mixing both syntaxes in one function).

---

## broadcast_shapes

`function` · `pyrefly_types::shaped_array::broadcast_shapes`

```rust
fn broadcast_shapes(a: &IntTuple, b: &IntTuple) -> Result<IntTuple, dimension::ShapeError>
```

Compute the broadcasted shape of two tensor shapes following NumPy/PyTorch broadcasting rules:
- Dimensions are aligned from right to left
- Each dimension must either match or one of them must be 1
- Missing dimensions are treated as 1

For shapes with variadic middles (Unpacked), the algorithm:
1. Consume concrete suffix dims from both sides, right-to-left, broadcasting each pair.
   Stop when either side runs out of concrete dims (hits a middle or exhausts its dims).
2. Analyze what remains after suffix consumption:
   - empty + anything → result is the other side
   - concrete + unpacked(p, m, []) → shapeless if m is gradual; error if m is TypeVarTuple
   - unpacked + unpacked → if same TypeVarTuple with no extra suffix, broadcast prefixes;
     if either is gradual, shapeless; otherwise error
3. Assemble result from step 2 output + broadcast suffix.

---

## is_int_tuples_type

`function` · `pyrefly_types::shaped_array::is_int_tuples_type`

```rust
fn is_int_tuples_type(ty: &types::Type) -> bool
```

Whether `ty` is a structural `IntTuples` value.

Fixed and unbounded tuples contain `IntTuple` values directly. An unpacked tuple is valid when
its fixed members are `IntTuple` values and its middle is itself an `IntTuples` value; unions
are valid when every alternative has that structure.

---

## is_tuple_carrier_shape_middle

`function` · `pyrefly_types::shaped_array::is_tuple_carrier_shape_middle`

```rust
fn is_tuple_carrier_shape_middle(ty: &types::Type) -> bool
```

Detects a tuple-carrier shape variable occupying the variadic middle of an
unpacked shape.

---

## shape_to_tuple_carrier

`function` · `pyrefly_types::shaped_array::shape_to_tuple_carrier`

```rust
fn shape_to_tuple_carrier(shape: &IntTuple) -> types::Type
```

Convert an `IntTuple` into the equivalent tuple-carrier `Type`.

---

## shape_to_tuple_carrier_arg

`function` · `pyrefly_types::shaped_array::shape_to_tuple_carrier_arg`

```rust
fn shape_to_tuple_carrier_arg(shape: &IntTuple) -> types::Type
```

Convert a projected tuple-carrier shape back to the class type argument.

A tuple-carrier `TypeVar` represents the whole shape tuple, so `ndarray[S,
DType]` projects to `Unpacked([], S, [])` but must round-trip back to `S`,
not `tuple[*S]`.

---

## tuple_carrier_to_shape

`function` · `pyrefly_types::shaped_array::tuple_carrier_to_shape`

```rust
fn tuple_carrier_to_shape(carrier: &types::Type) -> Option<IntTuple>
```

Convert a tuple-carrier `Type` into an `IntTuple`.

Returns `None` when the carrier is not a tuple or contains an element that is
not a valid dimension.

`tuple[T, ...]` (including `tuple[int, ...]` and `tuple[Any, ...]`)
intentionally canonicalizes to the shapeless / unknown-rank shape: an
unbounded carrier conveys no recoverable per-dimension information. Its
element must still be a dimension, so `tuple[str, ...]` is not a carrier.

---

## type_to_dim

`function` · `pyrefly_types::shaped_array::type_to_dim`

```rust
fn type_to_dim(dim: &types::Type) -> Option<dimension::Int>
```

Convert a `Type` into an internal shape dimension, accepting only types that
form a valid `Int` dimension. Returns `None` for anything else, so callers
fail cleanly instead of treating an unrelated type as a dimension.

---

## IntTuple

`struct` · `pyrefly_types::shaped_array::IntTuple`

```rust
struct IntTuple
```

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (26)

```rust
fn all_literal(&self) -> bool
fn as_concrete(&self) -> Option<&[Int]>
fn as_literals(&self) -> Option<Vec<i64>>
fn dims(&self) -> &Vec<Int>
fn dims_mut(&mut self) -> &mut Vec<Int>
fn dims_slice(&self) -> &[Int]
fn fmt_jaxtyping(&self) -> String
fn from_shape_arg_or_tuple_carrier(arg: &Type) -> Option<Self>
fn from_shape_arg_type(arg: &Type) -> Option<Self>
fn from_tuple(tuple: Tuple) -> Self
fn from_types(dims: Vec<Type>) -> Self
fn get_dim(&self, index: usize) -> Type
fn is_empty(&self) -> bool
fn is_shapeless(&self) -> bool
fn materialize(&mut self)
fn new(dims: Vec<Int>) -> Self
fn normalize(&self) -> Self
fn normalize_dim(&self, dim: i64) -> Result<usize, ShapeError>
fn rank(&self) -> usize
fn shapeless() -> Self
fn to_shape_arg_type(&self) -> Type
fn to_tuple(&self) -> Tuple
fn to_tuple_type(&self) -> Type
fn unpacked(prefix: Vec<Int>, middle: Type, suffix: Vec<Int>) -> Self
fn unpacked_from_types(prefix: Vec<Type>, middle: Type, suffix: Vec<Type>) -> Self
fn view(&self) -> IntTupleView<'_>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, f: &mut dyn FnMut(&'a To))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut To))
```

Shape of a shaped array.

The storage is deliberately not a `Tuple`: fixed dimensions are always
canonical `Int`s, while variadic middles carry the original tuple/type
variable shape carrier.

---

## ShapedArrayType

`struct` · `pyrefly_types::shaped_array::ShapedArrayType`

```rust
struct ShapedArrayType
```

**Fields**: `base_class`, `syntax`

**Implements**: `core::fmt::Display`, `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (12)

```rust
fn is_shapeless(&self) -> bool
fn materialize_inline_shape(&mut self)
fn new(base_class: ClassType, shape: IntTuple) -> Self
fn rank(&self) -> Option<usize>
fn set_shape(&mut self, shape: IntTuple)
fn set_tuple_carrier_shape_arg(&mut self, index: usize)
fn shape(&self) -> IntTuple
fn shapeless(base_class: ClassType) -> Self
fn to_type(self) -> Type
fn tuple_carrier_shape_arg_index(&self) -> Option<usize>
fn with_syntax(self, syntax: ShapedArraySyntax) -> Self
fn with_tuple_carrier_shape_arg(self, index: usize) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut equality::TypeEqCtx) -> bool
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, f: &mut dyn FnMut(&'a To))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut To))
```

A class instance with shape information.
Example: Tensor[[2, 3]] represents a 2x3 tensor
Example: Tensor (no brackets) represents a shapeless tensor (`IntTuple`)

---
