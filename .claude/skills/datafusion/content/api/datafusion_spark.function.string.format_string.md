# `datafusion_spark::function::string::format_string`

Crate `datafusion-spark` · 7 public items · structured records in [`model/datafusion_spark.function.string.format_string.json`](../model/datafusion_spark.function.string.format_string.json)

## ConversionType

`enum` · `datafusion_spark::function::string::format_string::ConversionType`

```rust
enum ConversionType
```

**Variants**: `BooleanUpper`, `BooleanLower`, `HexHashLower`, `HexHashUpper`, `DecInt`, `OctInt`, `HexIntLower`, `HexIntUpper`, `SciFloatLower`, `SciFloatUpper`, `DecFloatLower`, `CompactFloatLower`, `CompactFloatUpper`, `HexFloatLower`, `HexFloatUpper`, `TimeLower`, `TimeUpper`, `CharLower`, `CharUpper`, `StringLower`, `StringUpper`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn validate(&self, arg_type: &DataType) -> Result<()>
```

Printf data type

---

## FormatElement

`enum` · `datafusion_spark::function::string::format_string::FormatElement`

```rust
enum FormatElement<'a>
```

**Variants**: `Verbatim`, `Format`

**Derives**: Debug

---

## NumericParam

`enum` · `datafusion_spark::function::string::format_string::NumericParam`

```rust
enum NumericParam
```

**Variants**: `Literal`, `FromArgument`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

Width / precision parameter

---

## TimeFormat

`enum` · `datafusion_spark::function::string::format_string::TimeFormat`

```rust
enum TimeFormat
```

**Variants**: `HUpper`, `IUpper`, `KLower`, `LLower`, `MUpper`, `SUpper`, `LUpper`, `NUpper`, `PLower`, `ZLower`, `ZUpper`, `SLower`, `QUpper`, `BUpper`, `BLower`, `AUpper`, `ALower`, `CUpper`, `YUpper`, `YLower`, `JLower`, `MLower`, `DLower`, `ELower`, `RUpper`, `TUpper`, `RLower`, `DUpper`, `FUpper`, `CLower`

**Implements**: `core::convert::TryFrom`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**via `core::convert::TryFrom`**

```rust
fn try_from(value: char) -> Result<Self, Self::Error>
```

---

## ConversionSpecifier

`struct` · `datafusion_spark::function::string::format_string::ConversionSpecifier`

```rust
struct ConversionSpecifier
```

**Fields**: `argument_index`, `alt_form`, `zero_pad`, `left_adj`, `space_sign`, `force_sign`, `grouping_separator`, `negative_in_parentheses`, `width`, `precision`, `conversion_type`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn format(&self, string: &mut String, value: &ScalarValue) -> Result<()>
```

Parsed printf conversion specifier

---

## FormatStringFunc

`struct` · `datafusion_spark::function::string::format_string::FormatStringFunc`

```rust
struct FormatStringFunc
```

**Implements**: `datafusion_expr::udf::ScalarUDFImpl`

**Derives**: Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn new() -> Self
```

**via `datafusion_expr::udf::ScalarUDFImpl`**

```rust
fn aliases(&self) -> &[String]
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
fn name(&self) -> &str
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
fn signature(&self) -> &Signature
```

Spark-compatible `format_string` expression
<https://spark.apache.org/docs/latest/api/sql/index.html#format_string>

---

## Formatter

`struct` · `datafusion_spark::function::string::format_string::Formatter`

```rust
struct Formatter<'a>
```

**Fields**: `elements`, `arg_num`

**Derives**: Debug

**Methods** (3)

```rust
fn format(&self, args: &[ScalarValue]) -> Result<String>
fn new(elements: Vec<FormatElement<'a>>) -> Self
fn parse(fmt: &'a str, arg_types: &[DataType]) -> Result<Self>
```

Compatible with `java.util.Formatter`

---
