# `datafusion_substrait::variation_const`

Crate `datafusion-substrait` · 25 public items · structured records in [`model/datafusion_substrait.variation_const.json`](../model/datafusion_substrait.variation_const.json)

## DATE_32_TYPE_VARIATION_REF

`constant` · `datafusion_substrait::variation_const::DATE_32_TYPE_VARIATION_REF`

```rust
const DATE_32_TYPE_VARIATION_REF: u32 = 0
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.variation_const.DATE_32_TYPE_VARIATION_REF.md).


---

## DATE_64_TYPE_VARIATION_REF

`constant` · `datafusion_substrait::variation_const::DATE_64_TYPE_VARIATION_REF`

```rust
const DATE_64_TYPE_VARIATION_REF: u32 = 1
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.variation_const.DATE_64_TYPE_VARIATION_REF.md).


---

## DECIMAL_128_TYPE_VARIATION_REF

`constant` · `datafusion_substrait::variation_const::DECIMAL_128_TYPE_VARIATION_REF`

```rust
const DECIMAL_128_TYPE_VARIATION_REF: u32 = 0
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.variation_const.DECIMAL_128_TYPE_VARIATION_REF.md).


---

## DECIMAL_256_TYPE_VARIATION_REF

`constant` · `datafusion_substrait::variation_const::DECIMAL_256_TYPE_VARIATION_REF`

```rust
const DECIMAL_256_TYPE_VARIATION_REF: u32 = 1
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.variation_const.DECIMAL_256_TYPE_VARIATION_REF.md).


---

## DEFAULT_CONTAINER_TYPE_VARIATION_REF

`constant` · `datafusion_substrait::variation_const::DEFAULT_CONTAINER_TYPE_VARIATION_REF`

```rust
const DEFAULT_CONTAINER_TYPE_VARIATION_REF: u32 = 0
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.variation_const.DEFAULT_CONTAINER_TYPE_VARIATION_REF.md).


---

## DEFAULT_INTERVAL_DAY_TYPE_VARIATION_REF

`constant` · `datafusion_substrait::variation_const::DEFAULT_INTERVAL_DAY_TYPE_VARIATION_REF`

```rust
const DEFAULT_INTERVAL_DAY_TYPE_VARIATION_REF: u32 = 0
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.variation_const.DEFAULT_INTERVAL_DAY_TYPE_VARIATION_REF.md).


Used for the arrow type [`DataType::Interval`] with [`IntervalUnit::DayTime`].

[`DataType::Interval`]: datafusion::arrow::datatypes::DataType::Interval
[`IntervalUnit::DayTime`]: datafusion::arrow::datatypes::IntervalUnit::DayTime

---

## DEFAULT_MAP_TYPE_VARIATION_REF

`constant` · `datafusion_substrait::variation_const::DEFAULT_MAP_TYPE_VARIATION_REF`

```rust
const DEFAULT_MAP_TYPE_VARIATION_REF: u32 = 0
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.variation_const.DEFAULT_MAP_TYPE_VARIATION_REF.md).


---

## DEFAULT_TYPE_VARIATION_REF

`constant` · `datafusion_substrait::variation_const::DEFAULT_TYPE_VARIATION_REF`

```rust
const DEFAULT_TYPE_VARIATION_REF: u32 = 0
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.variation_const.DEFAULT_TYPE_VARIATION_REF.md).


The "system-preferred" variation (i.e., no variation).

---

## DICTIONARY_MAP_TYPE_VARIATION_REF

`constant` · `datafusion_substrait::variation_const::DICTIONARY_MAP_TYPE_VARIATION_REF`

```rust
const DICTIONARY_MAP_TYPE_VARIATION_REF: u32 = 1
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.variation_const.DICTIONARY_MAP_TYPE_VARIATION_REF.md).


---

## DURATION_INTERVAL_DAY_TYPE_VARIATION_REF

`constant` · `datafusion_substrait::variation_const::DURATION_INTERVAL_DAY_TYPE_VARIATION_REF`

```rust
const DURATION_INTERVAL_DAY_TYPE_VARIATION_REF: u32 = 1
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.variation_const.DURATION_INTERVAL_DAY_TYPE_VARIATION_REF.md).


Used for the arrow type [`DataType::Duration`].

[`DataType::Duration`]: datafusion::arrow::datatypes::DataType::Duration

---

## FLOAT_16_TYPE_NAME

`constant` · `datafusion_substrait::variation_const::FLOAT_16_TYPE_NAME`

```rust
const FLOAT_16_TYPE_NAME: &str = "fp16"
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.variation_const.FLOAT_16_TYPE_NAME.md).


Defined in <https://github.com/apache/arrow/blame/main/format/substrait/extension_types.yaml>

---

## INTERVAL_DAY_TIME_TYPE_REF

`constant` · `datafusion_substrait::variation_const::INTERVAL_DAY_TIME_TYPE_REF`

> **Deprecated** — since 41.0.0: Use Substrait `IntervalDay` type instead

```rust
const INTERVAL_DAY_TIME_TYPE_REF: u32 = 2
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.variation_const.INTERVAL_DAY_TIME_TYPE_REF.md).


For [`DataType::Interval`] with [`IntervalUnit::DayTime`].

An `i64` as:
- days: `i32`
- milliseconds: `i32`

See also [`ScalarValue::IntervalDayTime`] for the literal definition in DataFusion.

[`DataType::Interval`]: datafusion::arrow::datatypes::DataType::Interval
[`IntervalUnit::DayTime`]: datafusion::arrow::datatypes::IntervalUnit::DayTime
[`ScalarValue::IntervalDayTime`]: datafusion::common::ScalarValue::IntervalDayTime

---

## INTERVAL_MONTH_DAY_NANO_TYPE_NAME

`constant` · `datafusion_substrait::variation_const::INTERVAL_MONTH_DAY_NANO_TYPE_NAME`

> **Deprecated** — since 43.0.0: Use Substrait `IntervalCompound` type instead

```rust
const INTERVAL_MONTH_DAY_NANO_TYPE_NAME: &str = "interval-month-day-nano"
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.variation_const.INTERVAL_MONTH_DAY_NANO_TYPE_NAME.md).


For [`DataType::Interval`] with [`IntervalUnit::MonthDayNano`].

[`DataType::Interval`]: datafusion::arrow::datatypes::DataType::Interval
[`IntervalUnit::MonthDayNano`]: datafusion::arrow::datatypes::IntervalUnit::MonthDayNano

---

## INTERVAL_MONTH_DAY_NANO_TYPE_REF

`constant` · `datafusion_substrait::variation_const::INTERVAL_MONTH_DAY_NANO_TYPE_REF`

> **Deprecated** — since 41.0.0: Use Substrait `IntervalCompound` type instead

```rust
const INTERVAL_MONTH_DAY_NANO_TYPE_REF: u32 = 3
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.variation_const.INTERVAL_MONTH_DAY_NANO_TYPE_REF.md).


For [`DataType::Interval`] with [`IntervalUnit::MonthDayNano`].

An `i128` as:
- months: `i32`
- days: `i32`
- nanoseconds: `i64`

See also [`ScalarValue::IntervalMonthDayNano`] for the literal definition in DataFusion.

[`DataType::Interval`]: datafusion::arrow::datatypes::DataType::Interval
[`IntervalUnit::MonthDayNano`]: datafusion::arrow::datatypes::IntervalUnit::MonthDayNano
[`ScalarValue::IntervalMonthDayNano`]: datafusion::common::ScalarValue::IntervalMonthDayNano

---

## INTERVAL_YEAR_MONTH_TYPE_REF

`constant` · `datafusion_substrait::variation_const::INTERVAL_YEAR_MONTH_TYPE_REF`

> **Deprecated** — since 41.0.0: Use Substrait `IntervalYear` type instead

```rust
const INTERVAL_YEAR_MONTH_TYPE_REF: u32 = 1
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.variation_const.INTERVAL_YEAR_MONTH_TYPE_REF.md).


For [`DataType::Interval`] with [`IntervalUnit::YearMonth`].

An `i32` for elapsed whole months. See also [`ScalarValue::IntervalYearMonth`]
for the literal definition in DataFusion.

[`DataType::Interval`]: datafusion::arrow::datatypes::DataType::Interval
[`IntervalUnit::YearMonth`]: datafusion::arrow::datatypes::IntervalUnit::YearMonth
[`ScalarValue::IntervalYearMonth`]: datafusion::common::ScalarValue::IntervalYearMonth

---

## LARGE_CONTAINER_TYPE_VARIATION_REF

`constant` · `datafusion_substrait::variation_const::LARGE_CONTAINER_TYPE_VARIATION_REF`

```rust
const LARGE_CONTAINER_TYPE_VARIATION_REF: u32 = 1
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.variation_const.LARGE_CONTAINER_TYPE_VARIATION_REF.md).


---

## NULL_TYPE_NAME

`constant` · `datafusion_substrait::variation_const::NULL_TYPE_NAME`

```rust
const NULL_TYPE_NAME: &str = "null"
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.variation_const.NULL_TYPE_NAME.md).


For [`DataType::Null`]

[`DataType::Null`]: datafusion::arrow::datatypes::DataType::Null

---

## TIMESTAMP_MICRO_TYPE_VARIATION_REF

`constant` · `datafusion_substrait::variation_const::TIMESTAMP_MICRO_TYPE_VARIATION_REF`

> **Deprecated** — since 42.0.0: Use `PrecisionTimestamp(Tz)` type instead

```rust
const TIMESTAMP_MICRO_TYPE_VARIATION_REF: u32 = 2
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.variation_const.TIMESTAMP_MICRO_TYPE_VARIATION_REF.md).


---

## TIMESTAMP_MILLI_TYPE_VARIATION_REF

`constant` · `datafusion_substrait::variation_const::TIMESTAMP_MILLI_TYPE_VARIATION_REF`

> **Deprecated** — since 42.0.0: Use `PrecisionTimestamp(Tz)` type instead

```rust
const TIMESTAMP_MILLI_TYPE_VARIATION_REF: u32 = 1
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.variation_const.TIMESTAMP_MILLI_TYPE_VARIATION_REF.md).


---

## TIMESTAMP_NANO_TYPE_VARIATION_REF

`constant` · `datafusion_substrait::variation_const::TIMESTAMP_NANO_TYPE_VARIATION_REF`

> **Deprecated** — since 42.0.0: Use `PrecisionTimestamp(Tz)` type instead

```rust
const TIMESTAMP_NANO_TYPE_VARIATION_REF: u32 = 3
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.variation_const.TIMESTAMP_NANO_TYPE_VARIATION_REF.md).


---

## TIMESTAMP_SECOND_TYPE_VARIATION_REF

`constant` · `datafusion_substrait::variation_const::TIMESTAMP_SECOND_TYPE_VARIATION_REF`

> **Deprecated** — since 42.0.0: Use `PrecisionTimestamp(Tz)` type instead

```rust
const TIMESTAMP_SECOND_TYPE_VARIATION_REF: u32 = 0
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.variation_const.TIMESTAMP_SECOND_TYPE_VARIATION_REF.md).


---

## TIME_32_TYPE_VARIATION_REF

`constant` · `datafusion_substrait::variation_const::TIME_32_TYPE_VARIATION_REF`

```rust
const TIME_32_TYPE_VARIATION_REF: u32 = 0
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.variation_const.TIME_32_TYPE_VARIATION_REF.md).


---

## TIME_64_TYPE_VARIATION_REF

`constant` · `datafusion_substrait::variation_const::TIME_64_TYPE_VARIATION_REF`

```rust
const TIME_64_TYPE_VARIATION_REF: u32 = 1
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.variation_const.TIME_64_TYPE_VARIATION_REF.md).


---

## UNSIGNED_INTEGER_TYPE_VARIATION_REF

`constant` · `datafusion_substrait::variation_const::UNSIGNED_INTEGER_TYPE_VARIATION_REF`

```rust
const UNSIGNED_INTEGER_TYPE_VARIATION_REF: u32 = 1
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.variation_const.UNSIGNED_INTEGER_TYPE_VARIATION_REF.md).


---

## VIEW_CONTAINER_TYPE_VARIATION_REF

`constant` · `datafusion_substrait::variation_const::VIEW_CONTAINER_TYPE_VARIATION_REF`

```rust
const VIEW_CONTAINER_TYPE_VARIATION_REF: u32 = 2
```

[Full member, field, variant and typed contracts](../operations/datafusion_substrait.variation_const.VIEW_CONTAINER_TYPE_VARIATION_REF.md).


---
