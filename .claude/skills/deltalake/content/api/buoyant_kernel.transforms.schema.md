# `buoyant_kernel::transforms::schema`

Crate `buoyant_kernel` · 2 public items · structured records in [`model/buoyant_kernel.transforms.schema.json`](../model/buoyant_kernel.transforms.schema.json)

## SchemaDepthChecker

`struct` · `buoyant_kernel::transforms::schema::SchemaDepthChecker`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.transforms.schema.SchemaDepthChecker.md)

Also reachable as `buoyant_kernel::transforms::SchemaDepthChecker`, `delta_kernel::transforms::schema::SchemaDepthChecker`

```rust
struct SchemaDepthChecker
```

**Implements**: `buoyant_kernel::transforms::schema::SchemaTransform`

**Methods** (1)

```rust
fn check(data_type: &DataType, depth_limit: usize) -> usize
```

**via `buoyant_kernel::transforms::schema::SchemaTransform`**

```rust
fn transform_array(&mut self, atype: &'a ArrayType) -> Result<(), ()>
fn transform_map(&mut self, mtype: &'a MapType) -> Result<(), ()>
fn transform_struct(&mut self, stype: &'a StructType) -> Result<(), ()>
fn transform_struct_field(&mut self, field: &'a StructField) -> Result<(), ()>
```

A schema "transform" that doesn't actually change the schema at all. Instead, it measures the
maximum depth of a schema, with a depth limit to prevent stack overflow. Useful for verifying
that a schema has reasonable depth before attempting to work with it.

---

## SchemaTransform

`trait` · `buoyant_kernel::transforms::schema::SchemaTransform`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.transforms.schema.SchemaTransform.md)

Also reachable as `buoyant_kernel::transforms::SchemaTransform`, `delta_kernel::transforms::schema::SchemaTransform`

```rust
trait SchemaTransform<'a>
```

**Implementors** (25)

- `buoyant_kernel::engine::arrow_utils::StringifyFailureProneLeaves`
- `buoyant_kernel::expressions::literal_expression_transform::LiteralExpressionTransform`
- `buoyant_kernel::scan::GetReferencedFields`
- `buoyant_kernel::scan::data_skipping::stats_schema::BaseStatsTransform`
- `buoyant_kernel::scan::data_skipping::stats_schema::MinMaxStatsTransform`
- `buoyant_kernel::scan::data_skipping::stats_schema::NullCountStatsTransform`
- `buoyant_kernel::scan::data_skipping::stats_schema::NullableStatsTransform`
- `buoyant_kernel::scan::data_skipping::stats_schema::StripFieldMetadataTransform`
- `buoyant_kernel::schema::GetSchemaLeaves`
- `buoyant_kernel::schema::InvariantChecker`
- `buoyant_kernel::schema::MakePhysical`
- `buoyant_kernel::schema::NonNullFieldChecker`
- `buoyant_kernel::schema::validation::SchemaValidator`
- `buoyant_kernel::schema::variant_utils::UsesVariant`
- `buoyant_kernel::schema::void_utils::StripVoidFields`
- `buoyant_kernel::schema::void_utils::ValidateForWrite`
- `buoyant_kernel::table_features::column_mapping::MaxColumnId`
- `buoyant_kernel::table_features::iceberg_compat::LegacyNestedIdsVisitor`
- `buoyant_kernel::table_features::iceberg_compat::TypeAllowListVisitor`
- `buoyant_kernel::table_features::timestamp_nanos::UsesTimestampNanos`
- `buoyant_kernel::table_features::timestamp_ntz::UsesTimestampNtz`
- `buoyant_kernel::transforms::schema::SchemaDepthChecker`
- `deltalake_core::kernel::arrow::engine_ext::BaseStatsTransform`
- `deltalake_core::kernel::arrow::engine_ext::MinMaxStatsTransform`
- `deltalake_core::kernel::arrow::engine_ext::NullCountStatsTransform`

**Methods** (14)

```rust
fn recurse_into_array(&mut self, atype: &'a ArrayType) -> Self::Output<ArrayType>
fn recurse_into_map(&mut self, mtype: &'a MapType) -> Self::Output<MapType>
fn recurse_into_struct(&mut self, stype: &'a StructType) -> Self::Output<StructType>
fn recurse_into_struct_field(&mut self, field: &'a StructField) -> Self::Output<StructField>
fn transform(&mut self, data_type: &'a DataType) -> Self::Output<DataType>
fn transform_array(&mut self, atype: &'a ArrayType) -> Self::Output<ArrayType>
fn transform_array_element(&mut self, etype: &'a DataType) -> Self::Output<DataType>
fn transform_map(&mut self, mtype: &'a MapType) -> Self::Output<MapType>
fn transform_map_key(&mut self, etype: &'a DataType) -> Self::Output<DataType>
fn transform_map_value(&mut self, etype: &'a DataType) -> Self::Output<DataType>
fn transform_primitive(&mut self, ptype: &'a PrimitiveType) -> Self::Output<PrimitiveType>
fn transform_struct(&mut self, stype: &'a StructType) -> Self::Output<StructType>
fn transform_struct_field(&mut self, field: &'a StructField) -> Self::Output<StructField>
fn transform_variant(&mut self, stype: &'a StructType) -> Self::Output<StructType>
```

Generic framework for describing recursive bottom-up schema transforms.

The transform can start from whatever schema element is available
(e.g. [`Self::transform_struct`] to start with [`StructType`]), or it can start from the generic
[`Self::transform`].

The provided `transform_xxx` methods all default to no-op (usually by invoking the corresponding
recursive helper method), and implementations should selectively override specific
`transform_xxx` methods as needed for the task at hand.

# Recursive helper methods

The provided `recurse_into_xxx` methods encapsulate the boilerplate work of recursing into the
child schema elements of each schema element. Except as specifically noted otherwise, these
recursive helpers all behave uniformly, based on the number of children the schema element has:

* Leaf (no children) - Leaf `transform_xxx` methods simply return their argument unchanged, and
  no corresponding `recurse_into_xxx` method is provided.

* Unary (single child) - If the child was filtered out, filter out the parent. If the child
  changed, build a new parent around it. Otherwise, return the parent unchanged.

* Binary (two children) - If either child was filtered out, filter out the parent. If at least
  one child changed, build a new parent around them. Otherwise, return the parent unchanged.

* Variadic (0+ children) - If no children remain (all filtered out), filter out the parent.
  Otherwise, if at least one child changed or was filtered out, build a new parent around the
  children. Otherwise, return the parent unchanged.

Implementations can call these as needed, but will generally not need to override them.

# Transform carrier selection

Implementations choose an output [`Carrier`] instance based on the operation to be
performed. That carrier determines the return type of each transform method.

For example, a simple read-only visitor would use `()` as a carrier, while a validity checker
could use `DeltaResult<()>` instead. A mutating transform uses `Cow<_>`, returning `Cow::Owned`
for changed/replaced nodes, and a filtering transform uses `Option<Cow<_>>`, where `None`
indicates the node should be dropped rather than replaced. `DeltaResult<Cow<_>>` and
`Result<Option<Cow<_>>, E>` round out the set as fallible mutating and fitering transforms that
short circuit immediately upon `Err`.

---
