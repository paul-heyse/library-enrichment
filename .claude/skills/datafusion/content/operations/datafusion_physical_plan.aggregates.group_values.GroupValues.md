# `datafusion_physical_plan::aggregates::group_values::GroupValues`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.aggregates.group_values.GroupValues.json).

<a id="op-45b0dbc634161f3761d56030"></a>
## GroupValues

`trait` · `datafusion_physical_plan::aggregates::group_values::GroupValues` · datafusion-physical-plan 55.1.0

```rust
trait GroupValues: Send
```

Source: `src/aggregates/group_values/mod.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Stores the group values during hash aggregation.

# Background

In a query such as `SELECT a, b, count(*) FROM t GROUP BY a, b`, the group values
identify each group, and correspond to all the distinct values of `(a,b)`.

```sql
-- Input has 4 rows with 3 distinct combinations of (a,b) ("groups")
create table t(a int, b varchar)
as values (1, 'a'), (2, 'b'), (1, 'a'), (3, 'c');

select a, b, count(*) from t group by a, b;
----
1 a 2
2 b 1
3 c 1
```

# Design

Managing group values is a performance critical operation in hash
aggregation. The major operations are:

1. Intern: Quickly finding existing and adding new group values
2. Emit: Returning the group values as an array

There are multiple specialized implementations of this trait optimized for
different data types and number of columns, optimized for these operations.
See [`new_group_values`](../operations/datafusion_physical_plan.aggregates.group_values.new_group_values.md#op-eb5ab4eed709f30c47b0b677) for details.

# Group Ids

Each distinct group in a hash aggregation is identified by a unique group id
(usize) which is assigned by instances of this trait. Group ids are
continuous without gaps, starting from 0.

<a id="op-6b0d493a3cee50fb706ed5ba"></a>
## clear_shrink

`function` · `datafusion_physical_plan::aggregates::group_values::GroupValues::clear_shrink` · datafusion-physical-plan 55.1.0

```rust
fn clear_shrink(&mut self, num_rows: usize)
```

Source: `src/aggregates/group_values/mod.rs:117`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Clear the contents and shrink the capacity to the size of the batch (free up memory usage)

<a id="op-37ad1e4dd099c17f117ae028"></a>
## emit

`function` · `datafusion_physical_plan::aggregates::group_values::GroupValues::emit` · datafusion-physical-plan 55.1.0

```rust
fn emit(&mut self, emit_to: EmitTo) -> Result<Vec<ArrayRef>>
```

Source: `src/aggregates/group_values/mod.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Emits the group values

<a id="op-3448f185758642d0db2ae484"></a>
## intern

`function` · `datafusion_physical_plan::aggregates::group_values::GroupValues::intern` · datafusion-physical-plan 55.1.0

```rust
fn intern(&mut self, cols: &[ArrayRef], groups: &mut Vec<usize>) -> Result<()>
```

Source: `src/aggregates/group_values/mod.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Calculates the group id for each input row of `cols`, assigning new
group ids as necessary.

When the function returns, `groups`  must contain the group id for each
row in `cols`.

If a row has the same value as a previous row, the same group id is
assigned. If a row has a new value, the next available group id is
assigned.

<a id="op-bb98ed974e89ad313c53f597"></a>
## is_empty

`function` · `datafusion_physical_plan::aggregates::group_values::GroupValues::is_empty` · datafusion-physical-plan 55.1.0

```rust
fn is_empty(&self) -> bool
```

Source: `src/aggregates/group_values/mod.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns true if this [`GroupValues`](../operations/datafusion_physical_plan.aggregates.group_values.GroupValues.md#op-45b0dbc634161f3761d56030) is empty

<a id="op-982cd53a78fad80712a76b62"></a>
## len

`function` · `datafusion_physical_plan::aggregates::group_values::GroupValues::len` · datafusion-physical-plan 55.1.0

```rust
fn len(&self) -> usize
```

Source: `src/aggregates/group_values/mod.rs:111`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The number of values (distinct group values) stored in this [`GroupValues`](../operations/datafusion_physical_plan.aggregates.group_values.GroupValues.md#op-45b0dbc634161f3761d56030)

<a id="op-ea053d78d01a737768c3b51d"></a>
## size

`function` · `datafusion_physical_plan::aggregates::group_values::GroupValues::size` · datafusion-physical-plan 55.1.0

```rust
fn size(&self) -> usize
```

Source: `src/aggregates/group_values/mod.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns the number of bytes of memory used by this [`GroupValues`](../operations/datafusion_physical_plan.aggregates.group_values.GroupValues.md#op-45b0dbc634161f3761d56030).

May be expensive; check the implementation before calling on hot paths.
