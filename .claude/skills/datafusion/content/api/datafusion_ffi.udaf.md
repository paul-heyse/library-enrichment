# `datafusion_ffi::udaf`

Crate `datafusion-ffi` · 4 public items · structured records in [`model/datafusion_ffi.udaf.json`](../model/datafusion_ffi.udaf.json)

## FFI_AggregateOrderSensitivity

`enum` · `datafusion_ffi::udaf::FFI_AggregateOrderSensitivity`

```rust
enum FFI_AggregateOrderSensitivity
```

**Variants**: `Insensitive`, `HardRequirement`, `SoftRequirement`, `Beneficial`

**Implements**: `core::convert::From`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(value: AggregateOrderSensitivity) -> Self
```

---

## AggregateUDFPrivateData

`struct` · `datafusion_ffi::udaf::AggregateUDFPrivateData`

```rust
struct AggregateUDFPrivateData
```

**Fields**: `udaf`

---

## FFI_AggregateUDF

`struct` · `datafusion_ffi::udaf::FFI_AggregateUDF`

```rust
struct FFI_AggregateUDF
```

**Fields**: `name`, `aliases`, `volatility`, `return_field`, `is_nullable`, `groups_accumulator_supported`, `accumulator`, `create_sliding_accumulator`, `state_fields`, `create_groups_accumulator`, `with_beneficial_ordering`, `order_sensitivity`, `coerce_types`, `clone`, `release`, `private_data`, `library_marker_id`, `supports_null_handling_clause`

**Implements**: `core::convert::From`, `core::ops::drop::Drop`

**Derives**: Clone, Debug, Send, Sync

**via `core::convert::From`**

```rust
fn from(udaf: Arc<AggregateUDF>) -> Self
```

**via `core::ops::drop::Drop`**

```rust
fn drop(&mut self)
```

A stable struct for sharing a [`AggregateUDF`] across FFI boundaries.

---

## ForeignAggregateUDF

`struct` · `datafusion_ffi::udaf::ForeignAggregateUDF`

```rust
struct ForeignAggregateUDF
```

**Implements**: `datafusion_expr::udaf::AggregateUDFImpl`

**Derives**: Debug, Eq, Hash, PartialEq, Send, Sync

**via `datafusion_expr::udaf::AggregateUDFImpl`**

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
fn aliases(&self) -> &[String]
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
fn create_groups_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
fn create_sliding_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
fn groups_accumulator_supported(&self, args: AccumulatorArgs<'_>) -> bool
fn is_nullable(&self) -> bool
fn name(&self) -> &str
fn order_sensitivity(&self) -> AggregateOrderSensitivity
fn return_field(&self, arg_fields: &[FieldRef]) -> Result<FieldRef>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
fn simplify(&self) -> Option<AggregateFunctionSimplification>
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
fn supports_null_handling_clause(&self) -> bool
fn with_beneficial_ordering(Arc<self>, beneficial_ordering: bool) -> Result<Option<Arc<dyn AggregateUDFImpl>>>
```

This struct is used to access an UDF provided by a foreign
library across a FFI boundary.

The ForeignAggregateUDF is to be used by the caller of the UDF, so it has
no knowledge or access to the private data. All interaction with the UDF
must occur through the functions defined in FFI_AggregateUDF.

---
