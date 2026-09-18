# `datafusion_functions::datetime::to_timestamp`

Crate `datafusion-functions` · 5 public items · structured records in [`model/datafusion_functions.datetime.to_timestamp.json`](../model/datafusion_functions.datetime.to_timestamp.json)

## ToTimestampFunc

`struct` · `datafusion_functions::datetime::to_timestamp::ToTimestampFunc`

```rust
struct ToTimestampFunc
```

**Implements**: `datafusion_expr::udf::ScalarUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn new() -> Self
fn new_with_config(config: &ConfigOptions) -> Self
```

**via `datafusion_expr::udf::ScalarUDFImpl`**

```rust
fn documentation(&self) -> Option<&Documentation>
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
fn with_updated_config(&self, config: &ConfigOptions) -> Option<ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.datetime.to_timestamp.ToTimestampFunc.md).


---

## ToTimestampMicrosFunc

`struct` · `datafusion_functions::datetime::to_timestamp::ToTimestampMicrosFunc`

```rust
struct ToTimestampMicrosFunc
```

**Implements**: `datafusion_expr::udf::ScalarUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn new() -> Self
fn new_with_config(config: &ConfigOptions) -> Self
```

**via `datafusion_expr::udf::ScalarUDFImpl`**

```rust
fn documentation(&self) -> Option<&Documentation>
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
fn with_updated_config(&self, config: &ConfigOptions) -> Option<ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.datetime.to_timestamp.ToTimestampMicrosFunc.md).


---

## ToTimestampMillisFunc

`struct` · `datafusion_functions::datetime::to_timestamp::ToTimestampMillisFunc`

```rust
struct ToTimestampMillisFunc
```

**Implements**: `datafusion_expr::udf::ScalarUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn new() -> Self
fn new_with_config(config: &ConfigOptions) -> Self
```

**via `datafusion_expr::udf::ScalarUDFImpl`**

```rust
fn documentation(&self) -> Option<&Documentation>
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
fn with_updated_config(&self, config: &ConfigOptions) -> Option<ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.datetime.to_timestamp.ToTimestampMillisFunc.md).


---

## ToTimestampNanosFunc

`struct` · `datafusion_functions::datetime::to_timestamp::ToTimestampNanosFunc`

```rust
struct ToTimestampNanosFunc
```

**Implements**: `datafusion_expr::udf::ScalarUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn new() -> Self
fn new_with_config(config: &ConfigOptions) -> Self
```

**via `datafusion_expr::udf::ScalarUDFImpl`**

```rust
fn documentation(&self) -> Option<&Documentation>
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
fn with_updated_config(&self, config: &ConfigOptions) -> Option<ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.datetime.to_timestamp.ToTimestampNanosFunc.md).


---

## ToTimestampSecondsFunc

`struct` · `datafusion_functions::datetime::to_timestamp::ToTimestampSecondsFunc`

```rust
struct ToTimestampSecondsFunc
```

**Implements**: `datafusion_expr::udf::ScalarUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn new() -> Self
fn new_with_config(config: &ConfigOptions) -> Self
```

**via `datafusion_expr::udf::ScalarUDFImpl`**

```rust
fn documentation(&self) -> Option<&Documentation>
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
fn with_updated_config(&self, config: &ConfigOptions) -> Option<ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.datetime.to_timestamp.ToTimestampSecondsFunc.md).


---
