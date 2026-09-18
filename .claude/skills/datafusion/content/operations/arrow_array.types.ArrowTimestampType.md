# `arrow_array::types::ArrowTimestampType`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.types.ArrowTimestampType.json).

<a id="op-21b45b590193d189ee0fc67b"></a>
## ArrowTimestampType

`trait` · `arrow_array::types::ArrowTimestampType` · arrow-array 59.3.0

```rust
trait ArrowTimestampType: ArrowTemporalType<Native = i64>
```

Source: `src/types.rs:320`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A timestamp type allows us to create array builders that take a timestamp.

<a id="op-834546a4ea2fa686f1512a1b"></a>
## UNIT

`assoc_const` · `arrow_array::types::ArrowTimestampType::UNIT` · arrow-array 59.3.0

```rust
UNIT
```

Source: `src/types.rs:322`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

The [`TimeUnit`](../operations/arrow_schema.datatype.TimeUnit.md#op-4e6bd6e27e392e178d8a5b2e) of this timestamp.

<a id="op-44fe19b80f6c6e3ab98c0913"></a>
## from_datetime

`function` · `arrow_array::types::ArrowTimestampType::from_datetime` · arrow-array 59.3.0

```rust
fn from_datetime<Tz: TimeZone>(datetime: DateTime<Tz>) -> Option<i64>
```

Source: `src/types.rs:338`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a timestamp value from a [`DateTime`] in any timezone.

Returns `None` if the timestamp value would overflow the i64 range
(e.g., for nanosecond precision with extreme datetime values).

# Arguments

* `datetime` - The datetime to convert

Unresolved upstream links (retained, not inferred): ``DateTime``.

<a id="op-4cd9d15728b404bf22f1c3a1"></a>
## from_naive_datetime

`function` · `arrow_array::types::ArrowTimestampType::from_naive_datetime` · arrow-array 59.3.0

```rust
fn from_naive_datetime(naive: NaiveDateTime, tz: Option<&Tz>) -> Option<i64>
```

Source: `src/types.rs:347`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a timestamp value from a [`NaiveDateTime`] interpreted in the given timezone.

# Arguments

* `naive` - The local datetime to convert
* `tz` - Optional timezone. If `None`, interprets as UTC
  (equivalent to calling [`Self::make_value`](../operations/arrow_array.types.ArrowTimestampType.md#op-ee967b99832b3c03fedae3bc)).

Unresolved upstream links (retained, not inferred): ``NaiveDateTime``.

<a id="op-ee967b99832b3c03fedae3bc"></a>
## make_value

`function` · `arrow_array::types::ArrowTimestampType::make_value` · arrow-array 59.3.0

```rust
fn make_value(naive: NaiveDateTime) -> Option<i64>
```

Source: `src/types.rs:328`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a ArrowTimestampType::Native from the provided [`NaiveDateTime`]

See [`DataType::Timestamp`](../operations/arrow_schema.datatype.DataType.md#op-4311beb64d86f8afbc59cba2) for more information on timezone handling

Unresolved upstream links (retained, not inferred): ``NaiveDateTime``.
