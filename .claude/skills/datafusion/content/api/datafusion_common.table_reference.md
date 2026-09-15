# `datafusion_common::table_reference`

Crate `datafusion-common` · 2 public items · structured records in [`model/datafusion_common.table_reference.json`](../model/datafusion_common.table_reference.json)

## TableReference

`enum` · `datafusion_common::table_reference::TableReference`

Also reachable as `datafusion::common::TableReference`, `datafusion_common::TableReference`

```rust
enum TableReference
```

**Variants**: `Bare`, `Partial`, `Full`

**Implements**: `core::convert::From`, `core::convert::TryFrom`, `core::fmt::Display`, `datafusion_common::heap_size::DFHeapSize`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (13)

```rust
fn bare(table: impl Into<Arc<str>>) -> TableReference
fn catalog(&self) -> Option<&str>
fn full(catalog: impl Into<Arc<str>>, schema: impl Into<Arc<str>>, table: impl Into<Arc<str>>) -> TableReference
fn none() -> Option<TableReference>
fn parse_str(s: &str) -> Self
fn parse_str_normalized(s: &str, ignore_case: bool) -> Self
fn partial(schema: impl Into<Arc<str>>, table: impl Into<Arc<str>>) -> TableReference
fn resolve(self, default_catalog: &str, default_schema: &str) -> ResolvedTableReference
fn resolved_eq(&self, other: &Self) -> bool
fn schema(&self) -> Option<&str>
fn table(&self) -> &str
fn to_quoted_string(&self) -> String
fn to_vec(&self) -> Vec<String>
```

**via `core::convert::From`**

```rust
fn from(s: String) -> Self
fn from(resolved: ResolvedTableReference) -> Self
fn from(s: &'a str) -> Self
fn from(s: &'a String) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `datafusion_common::heap_size::DFHeapSize`**

```rust
fn heap_size(&self, ctx: &mut DFHeapSizeCtx) -> usize
```

A multi part identifier (path) to a table that may require further
resolution (e.g. `foo.bar`).

[`TableReference`]s are cheap to `clone()` as they are implemented with
`Arc`.

See [`ResolvedTableReference`] for a fully resolved table reference.

# Creating [`TableReference`]

When converting strings to [`TableReference`]s, the string is parsed as
though it were a SQL identifier, normalizing (convert to lowercase) any
unquoted identifiers.  [`TableReference::bare`] creates references without
applying normalization semantics.

# Examples
```
# use datafusion_common::TableReference;
// Get a table reference to 'mytable'
let table_reference = TableReference::from("mytable");
assert_eq!(table_reference, TableReference::bare("mytable"));

// Get a table reference to 'mytable' (note the capitalization)
let table_reference = TableReference::from("MyTable");
assert_eq!(table_reference, TableReference::bare("mytable"));

// Get a table reference to 'MyTable' (note the capitalization) using double quotes
// (programmatically it is better to use `TableReference::bare` for this)
let table_reference = TableReference::from(r#""MyTable""#);
assert_eq!(table_reference, TableReference::bare("MyTable"));

// Get a table reference to 'myschema.mytable' (note the capitalization)
let table_reference = TableReference::from("MySchema.MyTable");
assert_eq!(
    table_reference,
    TableReference::partial("myschema", "mytable")
);
```

---

## ResolvedTableReference

`struct` · `datafusion_common::table_reference::ResolvedTableReference`

Also reachable as `datafusion::common::ResolvedTableReference`, `datafusion_common::ResolvedTableReference`

```rust
struct ResolvedTableReference
```

**Fields**: `catalog`, `schema`, `table`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

A fully resolved path to a table of the form "catalog.schema.table"

---
