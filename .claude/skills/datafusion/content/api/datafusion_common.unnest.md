# `datafusion_common::unnest`

Crate `datafusion-common` · 3 public items · structured records in [`model/datafusion_common.unnest.json`](../model/datafusion_common.unnest.json)

## NullHandling

`enum` · `datafusion_common::unnest::NullHandling`

Also reachable as `datafusion::common::NullHandling`, `datafusion_common::NullHandling`

```rust
enum NullHandling
```

**Variants**: `Drop`, `Preserve`, `PreserveAndExpandEmpty`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

How [`UnnestOptions`] handles `NULL` and empty list values in the input column.

The variants enumerate the three observable behaviors so that callers do
not have to compose multiple boolean flags to express what they want.

---

## RecursionUnnestOption

`struct` · `datafusion_common::unnest::RecursionUnnestOption`

Also reachable as `datafusion::common::RecursionUnnestOption`, `datafusion_common::RecursionUnnestOption`

```rust
struct RecursionUnnestOption
```

**Fields**: `input_column`, `output_column`, `depth`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

Instruction on how to unnest a column (mostly with a list type)
such as how to name the output, and how many level it should be unnested

---

## UnnestOptions

`struct` · `datafusion_common::unnest::UnnestOptions`

Also reachable as `datafusion::common::UnnestOptions`, `datafusion_common::UnnestOptions`

```rust
struct UnnestOptions
```

**Fields**: `null_handling`, `recursions`

**Implements**: `core::convert::From`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (6)

```rust
fn expand_empty_as_null(&self) -> bool
fn new() -> Self
fn preserve_nulls(&self) -> bool
fn with_null_handling(self, null_handling: NullHandling) -> Self
fn with_preserve_nulls(self, preserve_nulls: bool) -> Self
fn with_recursions(self, recursion: RecursionUnnestOption) -> Self
```

Options for unnesting a column that contains a list type,
replicating values in the other, non nested rows.

Conceptually this operation is like joining each row with all the
values in the list column.

The behavior with `NULL` and empty input lists is controlled by
[`NullHandling`]. See its variants for full details.

# Examples

## `Unnest(c1)`, null_handling: NullHandling::Drop
```text
     ┌─────────┐ ┌─────┐                ┌─────────┐ ┌─────┐
     │ {1, 2}  │ │  A  │   Unnest       │    1    │ │  A  │
     ├─────────┤ ├─────┤                ├─────────┤ ├─────┤
     │  null   │ │  B  │                │    2    │ │  A  │
     ├─────────┤ ├─────┤ ────────────▶  ├─────────┤ ├─────┤
     │   {}    │ │  D  │                │    3    │ │  E  │
     ├─────────┤ ├─────┤                └─────────┘ └─────┘
     │   {3}   │ │  E  │                    c1        c2
     └─────────┘ └─────┘
       c1         c2
```

## `Unnest(c1)`, null_handling: NullHandling::Preserve
```text
     ┌─────────┐ ┌─────┐                ┌─────────┐ ┌─────┐
     │ {1, 2}  │ │  A  │   Unnest       │    1    │ │  A  │
     ├─────────┤ ├─────┤                ├─────────┤ ├─────┤
     │  null   │ │  B  │                │    2    │ │  A  │
     ├─────────┤ ├─────┤ ────────────▶  ├─────────┤ ├─────┤
     │   {}    │ │  D  │                │  null   │ │  B  │
     ├─────────┤ ├─────┤                ├─────────┤ ├─────┤
     │   {3}   │ │  E  │                │    3    │ │  E  │
     └─────────┘ └─────┘                └─────────┘ └─────┘
       c1         c2                        c1        c2
```

## `Unnest(c1)`, null_handling: NullHandling::PreserveAndExpandEmpty
```text
     ┌─────────┐ ┌─────┐                ┌─────────┐ ┌─────┐
     │ {1, 2}  │ │  A  │   Unnest       │    1    │ │  A  │
     ├─────────┤ ├─────┤                ├─────────┤ ├─────┤
     │  null   │ │  B  │                │    2    │ │  A  │
     ├─────────┤ ├─────┤ ────────────▶  ├─────────┤ ├─────┤
     │   {}    │ │  D  │                │  null   │ │  B  │
     ├─────────┤ ├─────┤                ├─────────┤ ├─────┤
     │   {3}   │ │  E  │                │  null   │ │  D  │
     └─────────┘ └─────┘                ├─────────┤ ├─────┤
       c1         c2                    │    3    │ │  E  │
                                        └─────────┘ └─────┘
                                            c1        c2
```

`recursions` instruct how a column should be unnested (e.g unnesting a column multiple
time, with depth = 1 and depth = 2). Any unnested column not being mentioned inside this
options is inferred to be unnested with depth = 1

---
