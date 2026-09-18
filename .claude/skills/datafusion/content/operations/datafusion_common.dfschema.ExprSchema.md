# `datafusion_common::dfschema::ExprSchema`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.dfschema.ExprSchema.json).

<a id="op-ffb72162f5de4ff12fd93126"></a>
## ExprSchema

`trait` · `datafusion_common::dfschema::ExprSchema` · datafusion-common 55.1.0

```rust
trait ExprSchema: std::fmt::Debug
```

Source: `src/dfschema.rs:1212`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Provides schema information needed by certain methods of `Expr`
(defined in the datafusion-common crate).

Note that this trait is implemented for &[DFSchema](../operations/datafusion_common.dfschema.DFSchema.md#op-8af5adf56b372aa63b81eb98) which is
widely used in the DataFusion codebase.

<a id="op-5d469f7f6b322e9e852c83d6"></a>
## data_type

`function` · `datafusion_common::dfschema::ExprSchema::data_type` · datafusion-common 55.1.0

```rust
fn data_type(&self, col: &Column) -> Result<&DataType>
```

Source: `src/dfschema.rs:1219`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

What is the datatype of this column?

<a id="op-c2cffe0ed68f13fd26191351"></a>
## data_type_and_nullable

`function` · `datafusion_common::dfschema::ExprSchema::data_type_and_nullable` · datafusion-common 55.1.0

```rust
fn data_type_and_nullable(&self, col: &Column) -> Result<(&DataType, bool)>
```

Source: `src/dfschema.rs:1229`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Return the column's datatype and nullability

<a id="op-68f75b178487048436904818"></a>
## field_from_column

`function` · `datafusion_common::dfschema::ExprSchema::field_from_column` · datafusion-common 55.1.0

```rust
fn field_from_column(&self, col: &Column) -> Result<&FieldRef>
```

Source: `src/dfschema.rs:1235`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a2d32ba49afd00b770204ea"></a>
## metadata

`function` · `datafusion_common::dfschema::ExprSchema::metadata` · datafusion-common 55.1.0

```rust
fn metadata(&self, col: &Column) -> Result<&HashMap<String, String>>
```

Source: `src/dfschema.rs:1224`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns the column's optional metadata.

<a id="op-7b048de7a5b94a7bb3bb6bb4"></a>
## nullable

`function` · `datafusion_common::dfschema::ExprSchema::nullable` · datafusion-common 55.1.0

```rust
fn nullable(&self, col: &Column) -> Result<bool>
```

Source: `src/dfschema.rs:1214`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Is this column reference nullable?
