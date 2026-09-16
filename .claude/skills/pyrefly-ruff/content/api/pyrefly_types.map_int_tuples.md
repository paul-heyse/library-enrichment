# `pyrefly_types::map_int_tuples`

Crate `pyrefly_types` · 4 public items · structured records in [`model/pyrefly_types.map_int_tuples.json`](../model/pyrefly_types.map_int_tuples.json)

## MapIntTuplesInterpretation

`enum` · `pyrefly_types::map_int_tuples::MapIntTuplesInterpretation`

```rust
enum MapIntTuplesInterpretation
```

**Variants**: `Forward`, `ParameterPattern`

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

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

Whether a deferred `MapIntTuples` computes forward from its source or describes a parameter.

The same syntax has opposite information flow at a function boundary. In
`def make[S: IntTuples](s: S) -> MapIntTuples[lambda T: Box[T], S]`, the source `S` is known
and the map computes the return type. In
`def consume[S: IntTuples](xs: MapIntTuples[lambda T: Box[T], S]) -> S`, the argument is known;
the parameter pattern derives an ordinary `Sequence[Box[IntTuple]]` view from the mapped member
stored here, and parameter inference recovers `S` from the argument's elements.

---

## map_int_tuples_mapper_binder

`function` · `pyrefly_types::map_int_tuples::map_int_tuples_mapper_binder`

```rust
fn map_int_tuples_mapper_binder(module: pyrefly_python::module_name::ModuleName, parameter: &ruff_python_ast::Identifier) -> quantified::Quantified
```

Builds the binder denoted by an `IntTuples` mapper parameter.

The binder depends only on the parameter's module, name, and source location. Binding and
solving can therefore reconstruct the same binder independently, regardless of solve order.
Mapper parsing and standalone parameter resolution must both call this constructor so their
`QuantifiedIdentity` values compare equal during substitution.

---

## MapIntTuples

`struct` · `pyrefly_types::map_int_tuples::MapIntTuples`

```rust
struct MapIntTuples
```

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn make_parameter_pattern(&mut self, mapped_member: Type)
fn parts(&self) -> (&TypeLambda, &MapIntTuplesInterpretation, &Type)
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

A possibly deferred `MapIntTuples[<mapper>, <source>]` operation.

---

## TypeLambda

`struct` · `pyrefly_types::map_int_tuples::TypeLambda`

```rust
struct TypeLambda
```

**Implements**: `pyrefly_types::equality::TypeEq`, `pyrefly_util::visit::Visit`, `pyrefly_util::visit::VisitMut`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd

**Methods** (5)

```rust
fn apply(&self, argument: Type) -> Type
fn apply_to_tuple(&self, tuple: Tuple) -> Result<Type, ShapeError>
fn body(&self) -> &Type
fn new(parameter: Quantified, body: Type) -> Self
fn parameter(&self) -> &Quantified
```

**via `pyrefly_types::equality::TypeEq`**

```rust
fn type_eq(&self, other: &Self, ctx: &mut TypeEqCtx) -> bool
```

**via `pyrefly_util::visit::Visit`**

```rust
fn recurse<'a>(&'a self, f: &mut dyn FnMut(&'a Type))
```

**via `pyrefly_util::visit::VisitMut`**

```rust
fn recurse_mut(&mut self, f: &mut dyn FnMut(&mut Type))
```

The unary type-level function accepted by experimental `shape_extensions.MapIntTuples`.

Applying the function substitutes its quantified parameter in the body type. It is stored only
inside the experimental map representation and is not a general Python type-system construct.

---
