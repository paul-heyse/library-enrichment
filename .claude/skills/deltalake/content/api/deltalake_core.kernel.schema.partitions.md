# `deltalake_core::kernel::schema::partitions`

Crate `deltalake-core` · 9 public items · structured records in [`model/deltalake_core.kernel.schema.partitions.json`](../model/deltalake_core.kernel.schema.partitions.json)

## NULL_PARTITION_VALUE_DATA_PATH

`constant` · `deltalake_core::kernel::schema::partitions::NULL_PARTITION_VALUE_DATA_PATH`

Also reachable as `deltalake::NULL_PARTITION_VALUE_DATA_PATH`, `deltalake::kernel::partitions::NULL_PARTITION_VALUE_DATA_PATH`, `deltalake::kernel::schema::partitions::NULL_PARTITION_VALUE_DATA_PATH`, `deltalake::partitions::NULL_PARTITION_VALUE_DATA_PATH`, `deltalake_core::NULL_PARTITION_VALUE_DATA_PATH`, `deltalake_core::kernel::partitions::NULL_PARTITION_VALUE_DATA_PATH`, `deltalake_core::partitions::NULL_PARTITION_VALUE_DATA_PATH`

```rust
const NULL_PARTITION_VALUE_DATA_PATH: &str = "__HIVE_DEFAULT_PARTITION__"
```

A special value used in Hive to represent the null partition in partitioned tables

---

## FilterOp

`enum` · `deltalake_core::kernel::schema::partitions::FilterOp`

Also reachable as `deltalake::FilterOp`, `deltalake::kernel::partitions::FilterOp`, `deltalake::kernel::schema::partitions::FilterOp`, `deltalake::partitions::FilterOp`, `deltalake_core::FilterOp`, `deltalake_core::kernel::partitions::FilterOp`, `deltalake_core::partitions::FilterOp`

```rust
enum FilterOp
```

**Variants**: `Eq`, `Ne`, `Lt`, `Le`, `Gt`, `Ge`, `In`, `NotIn`

**Implements**: `core::fmt::Display`, `core::str::traits::FromStr`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn as_str(self) -> &'static str
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

The comparison operator of a `(column, op, value)` filter literal.

---

## FilterValue

`enum` · `deltalake_core::kernel::schema::partitions::FilterValue`

Also reachable as `deltalake::FilterValue`, `deltalake::kernel::partitions::FilterValue`, `deltalake::kernel::schema::partitions::FilterValue`, `deltalake::partitions::FilterValue`, `deltalake_core::FilterValue`, `deltalake_core::kernel::partitions::FilterValue`, `deltalake_core::partitions::FilterValue`

```rust
enum FilterValue<'a>
```

**Variants**: `Scalar`, `Set`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

The value of a `(column, op, value)` filter literal: a single partition-value
encoded string, or a set of them for `in` / `not in`.

---

## conjunction_to_kernel_predicate

`function` · `deltalake_core::kernel::schema::partitions::conjunction_to_kernel_predicate`

Also reachable as `deltalake::conjunction_to_kernel_predicate`, `deltalake::kernel::partitions::conjunction_to_kernel_predicate`, `deltalake::kernel::schema::partitions::conjunction_to_kernel_predicate`, `deltalake::partitions::conjunction_to_kernel_predicate`, `deltalake_core::conjunction_to_kernel_predicate`, `deltalake_core::kernel::partitions::conjunction_to_kernel_predicate`, `deltalake_core::partitions::conjunction_to_kernel_predicate`

```rust
fn conjunction_to_kernel_predicate(literals: &[FilterLiteral<'_>], table_schema: &delta_kernel::schema::StructType) -> errors::DeltaResult<delta_kernel::expressions::Predicate>
```

Translate a conjunction (AND) of filter literals into a kernel [`Predicate`].

Errors on an empty conjunction: an empty AND is vacuously true and would
silently match every file.

---

## dnf_to_kernel_predicate

`function` · `deltalake_core::kernel::schema::partitions::dnf_to_kernel_predicate`

Also reachable as `deltalake::dnf_to_kernel_predicate`, `deltalake::kernel::partitions::dnf_to_kernel_predicate`, `deltalake::kernel::schema::partitions::dnf_to_kernel_predicate`, `deltalake::partitions::dnf_to_kernel_predicate`, `deltalake_core::dnf_to_kernel_predicate`, `deltalake_core::kernel::partitions::dnf_to_kernel_predicate`, `deltalake_core::partitions::dnf_to_kernel_predicate`

```rust
fn dnf_to_kernel_predicate(dnf: &[Vec<FilterLiteral<'_>>], table_schema: &delta_kernel::schema::StructType) -> errors::DeltaResult<delta_kernel::expressions::Predicate>
```

Translate filters in disjunctive normal form -- an OR across conjunctions
(AND groups) of `(column, op, value)` literals -- into a kernel [`Predicate`].

---

## filter_literal

`function` · `deltalake_core::kernel::schema::partitions::filter_literal`

Also reachable as `deltalake::filter_literal`, `deltalake::kernel::partitions::filter_literal`, `deltalake::kernel::schema::partitions::filter_literal`, `deltalake::partitions::filter_literal`, `deltalake_core::filter_literal`, `deltalake_core::kernel::partitions::filter_literal`, `deltalake_core::partitions::filter_literal`

```rust
fn filter_literal<'a>(column: &'a str, op: &str, value: FilterValue<'a>) -> errors::DeltaResult<FilterLiteral<'a>>
```

Validate a raw `(column, op, value)` tuple into a [`FilterLiteral`],
parsing the operator string.

This is the boundary where stringly-typed filters (e.g. from FFI) enter:
an unknown operator, an empty column name, or an operator/value shape
mismatch all yield the pinned `InvalidPartitionFilter` error.

---

## literal_to_kernel_predicate

`function` · `deltalake_core::kernel::schema::partitions::literal_to_kernel_predicate`

Also reachable as `deltalake::kernel::partitions::literal_to_kernel_predicate`, `deltalake::kernel::schema::partitions::literal_to_kernel_predicate`, `deltalake::literal_to_kernel_predicate`, `deltalake::partitions::literal_to_kernel_predicate`, `deltalake_core::kernel::partitions::literal_to_kernel_predicate`, `deltalake_core::literal_to_kernel_predicate`, `deltalake_core::partitions::literal_to_kernel_predicate`

```rust
fn literal_to_kernel_predicate(literal: &FilterLiteral<'_>, table_schema: &delta_kernel::schema::StructType) -> errors::DeltaResult<delta_kernel::expressions::Predicate>
```

Translate a single filter literal into a kernel [`Predicate`].

The raw value is parsed against the schema type of `column`. A null scalar
under `=` / `!=` becomes an IS [NOT] NULL check: in SQL NULL compares equal
to nothing, itself included, but these filters have always allowed equality
against the null partition value.

---

## DeltaTablePartition

`struct` · `deltalake_core::kernel::schema::partitions::DeltaTablePartition`

Also reachable as `deltalake::DeltaTablePartition`, `deltalake::kernel::partitions::DeltaTablePartition`, `deltalake::kernel::schema::partitions::DeltaTablePartition`, `deltalake::partitions::DeltaTablePartition`, `deltalake_core::DeltaTablePartition`, `deltalake_core::kernel::partitions::DeltaTablePartition`, `deltalake_core::partitions::DeltaTablePartition`

```rust
struct DeltaTablePartition
```

**Fields**: `key`, `value`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn from_partition_value(partition_value: (&str, &Scalar)) -> Self
```

**via `core::convert::TryFrom`**

```rust
fn try_from(partition: &str) -> Result<Self, DeltaTableError>
```

A Struct DeltaTablePartition used to represent a partition of a DeltaTable.

---

## FilterLiteral

`type_alias` · `deltalake_core::kernel::schema::partitions::FilterLiteral`

Also reachable as `deltalake::FilterLiteral`, `deltalake::kernel::partitions::FilterLiteral`, `deltalake::kernel::schema::partitions::FilterLiteral`, `deltalake::partitions::FilterLiteral`, `deltalake_core::FilterLiteral`, `deltalake_core::kernel::partitions::FilterLiteral`, `deltalake_core::partitions::FilterLiteral`

```rust
type FilterLiteral<'a> = (&'a str, FilterOp, FilterValue<'a>)
```

A `(column, op, value)` comparison, mirroring the tuple filters accepted by
the Python bindings.

---
