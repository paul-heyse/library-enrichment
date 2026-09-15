# `datafusion_common::functional_dependencies`

Crate `datafusion-common` · 9 public items · structured records in [`model/datafusion_common.functional_dependencies.json`](../model/datafusion_common.functional_dependencies.json)

## Constraint

`enum` · `datafusion_common::functional_dependencies::Constraint`

Also reachable as `datafusion::common::Constraint`, `datafusion_common::Constraint`

```rust
enum Constraint
```

**Variants**: `PrimaryKey`, `Unique`

**Implements**: `core::convert::From`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

This object defines a constraint on a table.

---

## Dependency

`enum` · `datafusion_common::functional_dependencies::Dependency`

Also reachable as `datafusion::common::Dependency`, `datafusion_common::Dependency`

```rust
enum Dependency
```

**Variants**: `Single`, `Multi`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

Describes functional dependency mode.

---

## aggregate_functional_dependencies

`function` · `datafusion_common::functional_dependencies::aggregate_functional_dependencies`

Also reachable as `datafusion::common::aggregate_functional_dependencies`, `datafusion_common::aggregate_functional_dependencies`

```rust
fn aggregate_functional_dependencies(aggr_input_schema: &DFSchema, group_by_expr_names: &[String], aggr_schema: &DFSchema) -> FunctionalDependencies
```

Calculates functional dependencies for aggregate output, when there is a GROUP BY expression.

---

## get_required_group_by_exprs_indices

`function` · `datafusion_common::functional_dependencies::get_required_group_by_exprs_indices`

Also reachable as `datafusion::common::get_required_group_by_exprs_indices`, `datafusion_common::get_required_group_by_exprs_indices`

```rust
fn get_required_group_by_exprs_indices(schema: &DFSchema, group_by_expr_names: &[String]) -> Option<Vec<usize>>
```

Returns indices for the minimal subset of GROUP BY expressions that are
functionally equivalent to the original set of GROUP BY expressions.

---

## get_required_sort_exprs_indices

`function` · `datafusion_common::functional_dependencies::get_required_sort_exprs_indices`

Also reachable as `datafusion::common::get_required_sort_exprs_indices`, `datafusion_common::get_required_sort_exprs_indices`

```rust
fn get_required_sort_exprs_indices(schema: &DFSchema, sort_expr_names: &[String]) -> Vec<usize>
```

Returns indices for the minimal subset of ORDER BY expressions that are
functionally equivalent to the original set of ORDER BY expressions.

---

## get_target_functional_dependencies

`function` · `datafusion_common::functional_dependencies::get_target_functional_dependencies`

Also reachable as `datafusion::common::get_target_functional_dependencies`, `datafusion_common::get_target_functional_dependencies`

```rust
fn get_target_functional_dependencies(schema: &DFSchema, group_by_expr_names: &[String]) -> Option<Vec<usize>>
```

Returns target indices, for the determinant keys that are inside
group by expressions.

---

## Constraints

`struct` · `datafusion_common::functional_dependencies::Constraints`

Also reachable as `datafusion::common::Constraints`, `datafusion_common::Constraints`

```rust
struct Constraints
```

**Implements**: `core::convert::From`, `core::convert::TryFrom`, `core::fmt::Display`, `core::iter::traits::collect::IntoIterator`, `core::ops::deref::Deref`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (3)

```rust
fn extend(&mut self, other: Constraints)
fn new_unverified(constraints: Vec<Constraint>) -> Self
fn project(&self, proj_indices: &[usize]) -> Option<Self>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

**via `core::iter::traits::collect::IntoIterator`**

```rust
fn into_iter(self) -> Self::IntoIter
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

This object encapsulates a list of functional constraints:

---

## FunctionalDependence

`struct` · `datafusion_common::functional_dependencies::FunctionalDependence`

Also reachable as `datafusion::common::FunctionalDependence`, `datafusion_common::FunctionalDependence`

```rust
struct FunctionalDependence
```

**Fields**: `source_indices`, `target_indices`, `nullable`, `mode`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn new(source_indices: Vec<usize>, target_indices: Vec<usize>, nullable: bool) -> Self
fn with_mode(self, mode: Dependency) -> Self
```

This object defines a functional dependence in the schema. A functional
dependence defines a relationship between determinant keys and dependent
columns. A determinant key is a column, or a set of columns, whose value
uniquely determines values of some other (dependent) columns. If two rows
have the same determinant key, dependent columns in these rows are
necessarily the same. If the determinant key is unique, the set of
dependent columns is equal to the entire schema and the determinant key can
serve as a primary key. Note that a primary key may "downgrade" into a
determinant key due to an operation such as a join, and this object is
used to track dependence relationships in such cases. For more information
on functional dependencies, see:
<https://www.scaler.com/topics/dbms/functional-dependency-in-dbms/>

---

## FunctionalDependencies

`struct` · `datafusion_common::functional_dependencies::FunctionalDependencies`

Also reachable as `datafusion::common::FunctionalDependencies`, `datafusion_common::FunctionalDependencies`

```rust
struct FunctionalDependencies
```

**Implements**: `core::ops::deref::Deref`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (10)

```rust
fn add_offset(&mut self, offset: usize)
fn empty() -> Self
fn extend(&mut self, other: FunctionalDependencies)
fn extend_target_indices(&mut self, n_out: usize)
fn is_valid(&self, n_field: usize) -> bool
fn join(&self, other: &FunctionalDependencies, join_type: &JoinType, left_cols_len: usize) -> FunctionalDependencies
fn new(dependencies: Vec<FunctionalDependence>) -> Self
fn new_from_constraints(constraints: Option<&Constraints>, n_field: usize) -> Self
fn project_functional_dependencies(&self, proj_indices: &[usize], n_out: usize) -> FunctionalDependencies
fn with_dependency(self, mode: Dependency) -> Self
```

**via `core::ops::deref::Deref`**

```rust
fn deref(&self) -> &Self::Target
```

This object encapsulates all functional dependencies in a given relation.

---
