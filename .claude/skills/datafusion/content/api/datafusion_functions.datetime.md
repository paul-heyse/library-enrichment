# `datafusion_functions::datetime`

Crate `datafusion-functions` · 20 public items · structured records in [`model/datafusion_functions.datetime.json`](../model/datafusion_functions.datetime.json)

## current_date

`function` · `datafusion_functions::datetime::current_date`

```rust
fn current_date() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.datetime.current_date.md).


Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of current_date

---

## current_time

`function` · `datafusion_functions::datetime::current_time`

```rust
fn current_time() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.datetime.current_time.md).


Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of current_time

---

## date_bin

`function` · `datafusion_functions::datetime::date_bin`

```rust
fn date_bin() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.datetime.date_bin.md).


Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of date_bin

---

## date_part

`function` · `datafusion_functions::datetime::date_part`

```rust
fn date_part() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.datetime.date_part.md).


Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of date_part

---

## date_trunc

`function` · `datafusion_functions::datetime::date_trunc`

```rust
fn date_trunc() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.datetime.date_trunc.md).


Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of date_trunc

---

## from_unixtime

`function` · `datafusion_functions::datetime::from_unixtime`

```rust
fn from_unixtime() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.datetime.from_unixtime.md).


Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of from_unixtime

---

## functions

`function` · `datafusion_functions::datetime::functions`

```rust
fn functions() -> Vec<std::sync::Arc<datafusion_expr::ScalarUDF>>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.datetime.functions.md).


Returns all DataFusion functions defined in this package

---

## make_date

`function` · `datafusion_functions::datetime::make_date`

```rust
fn make_date() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.datetime.make_date.md).


Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of make_date

---

## make_time

`function` · `datafusion_functions::datetime::make_time`

```rust
fn make_time() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.datetime.make_time.md).


Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of make_time

---

## now

`function` · `datafusion_functions::datetime::now`

```rust
fn now(config: &datafusion_common::config::ConfigOptions) -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.datetime.now.md).


Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of now

---

## to_char

`function` · `datafusion_functions::datetime::to_char`

```rust
fn to_char() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.datetime.to_char.md).


Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of to_char

---

## to_date

`function` · `datafusion_functions::datetime::to_date`

```rust
fn to_date() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.datetime.to_date.md).


Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of to_date

---

## to_local_time

`function` · `datafusion_functions::datetime::to_local_time`

```rust
fn to_local_time() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.datetime.to_local_time.md).


Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of to_local_time

---

## to_time

`function` · `datafusion_functions::datetime::to_time`

```rust
fn to_time() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.datetime.to_time.md).


Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of to_time

---

## to_timestamp

`function` · `datafusion_functions::datetime::to_timestamp`

```rust
fn to_timestamp(config: &datafusion_common::config::ConfigOptions) -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.datetime.to_timestamp.md).


Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of to_timestamp

---

## to_timestamp_micros

`function` · `datafusion_functions::datetime::to_timestamp_micros`

```rust
fn to_timestamp_micros(config: &datafusion_common::config::ConfigOptions) -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.datetime.to_timestamp_micros.md).


Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of to_timestamp_micros

---

## to_timestamp_millis

`function` · `datafusion_functions::datetime::to_timestamp_millis`

```rust
fn to_timestamp_millis(config: &datafusion_common::config::ConfigOptions) -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.datetime.to_timestamp_millis.md).


Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of to_timestamp_millis

---

## to_timestamp_nanos

`function` · `datafusion_functions::datetime::to_timestamp_nanos`

```rust
fn to_timestamp_nanos(config: &datafusion_common::config::ConfigOptions) -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.datetime.to_timestamp_nanos.md).


Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of to_timestamp_nanos

---

## to_timestamp_seconds

`function` · `datafusion_functions::datetime::to_timestamp_seconds`

```rust
fn to_timestamp_seconds(config: &datafusion_common::config::ConfigOptions) -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.datetime.to_timestamp_seconds.md).


Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of to_timestamp_seconds

---

## to_unixtime

`function` · `datafusion_functions::datetime::to_unixtime`

```rust
fn to_unixtime() -> std::sync::Arc<datafusion_expr::ScalarUDF>
```

[Full member, field, variant and typed contracts](../operations/datafusion_functions.datetime.to_unixtime.md).


Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of to_unixtime

---
