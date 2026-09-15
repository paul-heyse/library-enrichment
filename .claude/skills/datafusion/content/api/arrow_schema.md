# `arrow_schema`

Crate `arrow-schema` · 1 public items · structured records in [`model/arrow_schema.json`](../model/arrow_schema.json)

## SortOptions

`struct` · `arrow_schema::SortOptions`

Also reachable as `arrow::compute::SortOptions`, `arrow::compute::kernels::sort::SortOptions`, `arrow_ord::sort::SortOptions`

```rust
struct SortOptions
```

**Fields**: `descending`, `nulls_first`

**Implements**: `core::convert::From`, `core::fmt::Display`, `core::ops::bit::Not`

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (7)

```rust
fn asc(self) -> Self
fn desc(self) -> Self
fn new(descending: bool, nulls_first: bool) -> Self
fn nulls_first(self) -> Self
fn nulls_last(self) -> Self
fn with_descending(self, descending: bool) -> Self
fn with_nulls_first(self, nulls_first: bool) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `core::ops::bit::Not`**

```rust
fn not(self) -> SortOptions
```

Options that define the sort order of a given column

The default sorts equivalently to of `ASC NULLS FIRST` in SQL (i.e.
ascending order with nulls sorting before any other values).

# Example creation
```
# use arrow_schema::SortOptions;
// configure using explicit initialization
let options = SortOptions {
  descending: false,
  nulls_first: true,
};
// Default is ASC NULLs First
assert_eq!(options, SortOptions::default());
assert_eq!(options.to_string(), "ASC NULLS FIRST");

// Configure using builder APIs
let options = SortOptions::default()
 .desc()
 .nulls_first();
assert_eq!(options.to_string(), "DESC NULLS FIRST");

// configure using explicit field values
let options = SortOptions::default()
 .with_descending(false)
 .with_nulls_first(false);
assert_eq!(options.to_string(), "ASC NULLS LAST");
```

# Example operations
It is also possible to negate the sort options using the `!` operator.
```
use arrow_schema::SortOptions;
let options = !SortOptions::default();
assert_eq!(options.to_string(), "DESC NULLS LAST");
```

---
