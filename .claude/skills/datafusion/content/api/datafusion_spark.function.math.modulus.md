# `datafusion_spark::function::math::modulus`

Crate `datafusion-spark` · 4 public items · structured records in [`model/datafusion_spark.function.math.modulus.json`](../model/datafusion_spark.function.math.modulus.json)

## spark_mod

`function` · `datafusion_spark::function::math::modulus::spark_mod`

```rust
fn spark_mod(args: &[datafusion_expr::ColumnarValue], enable_ansi_mode: bool) -> datafusion_common::Result<datafusion_expr::ColumnarValue>
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.math.modulus.spark_mod.md).


Spark-compatible `mod` function
In ANSI mode, division by zero throws an error.
In legacy mode, division by zero returns NULL (Spark behavior).

---

## spark_pmod

`function` · `datafusion_spark::function::math::modulus::spark_pmod`

```rust
fn spark_pmod(args: &[datafusion_expr::ColumnarValue], enable_ansi_mode: bool) -> datafusion_common::Result<datafusion_expr::ColumnarValue>
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.math.modulus.spark_pmod.md).


Spark-compatible `pmod` function
In ANSI mode, division by zero throws an error.
In legacy mode, division by zero returns NULL (Spark behavior).

---

## SparkMod

`struct` · `datafusion_spark::function::math::modulus::SparkMod`

```rust
struct SparkMod
```

**Implements**: `datafusion_expr::udf::ScalarUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr::udf::ScalarUDFImpl`**

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.math.modulus.SparkMod.md).


SparkMod implements the Spark-compatible modulo function

---

## SparkPmod

`struct` · `datafusion_spark::function::math::modulus::SparkPmod`

```rust
struct SparkPmod
```

**Implements**: `datafusion_expr::udf::ScalarUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr::udf::ScalarUDFImpl`**

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.function.math.modulus.SparkPmod.md).


SparkMod implements the Spark-compatible modulo function

---
