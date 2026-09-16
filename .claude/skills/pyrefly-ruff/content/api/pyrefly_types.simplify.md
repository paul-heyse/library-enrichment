# `pyrefly_types::simplify`

Crate `pyrefly_types` · 5 public items · structured records in [`model/pyrefly_types.simplify.json`](../model/pyrefly_types.simplify.json)

## intersect

`function` · `pyrefly_types::simplify::intersect`

```rust
fn intersect(ts: Vec<types::Type>, fallback: types::Type, heap: &heap::TypeHeap) -> types::Type
```

---

## simplify_tuples

`function` · `pyrefly_types::simplify::simplify_tuples`

```rust
fn simplify_tuples(tuple: tuple::Tuple, _heap: &heap::TypeHeap) -> tuple::Tuple
```

---

## simplify_tuples_and_distribute_unpacking

`function` · `pyrefly_types::simplify::simplify_tuples_and_distribute_unpacking`

```rust
fn simplify_tuples_and_distribute_unpacking(tuple: tuple::Tuple, heap: &heap::TypeHeap) -> types::Type
```

Simplify a tuple, distributing an unpacked union into a union of tuple types.

---

## unions

`function` · `pyrefly_types::simplify::unions`

```rust
fn unions(xs: Vec<types::Type>, heap: &heap::TypeHeap) -> types::Type
```

Union a set of types together, simplifying as much as you can.

---

## unions_with_literals

`function` · `pyrefly_types::simplify::unions_with_literals`

```rust
fn unions_with_literals(xs: Vec<types::Type>, stdlib: &stdlib::Stdlib, enum_members: &dyn Fn(&class::Class) -> Option<usize>, heap: &heap::TypeHeap) -> types::Type
```

Like `unions`, but also simplify away things regarding literals if you can,
e.g. `Literal[True, False] ==> bool`.

---
