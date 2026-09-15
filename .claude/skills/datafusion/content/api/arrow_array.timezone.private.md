# `arrow_array::timezone::private`

Crate `arrow-array` · 2 public items · structured records in [`model/arrow_array.timezone.private.json`](../model/arrow_array.timezone.private.json)

## Tz

`struct` · `arrow_array::timezone::private::Tz`

Also reachable as `arrow_array::timezone::Tz`

```rust
struct Tz
```

**Implements**: `chrono::offset::TimeZone`, `core::fmt::Display`, `core::str::traits::FromStr`

**Derives**: Clone, Copy, Debug

**via `chrono::offset::TimeZone`**

```rust
fn from_offset(offset: &Self::Offset) -> Self
fn offset_from_local_date(&self, local: &NaiveDate) -> LocalResult<Self::Offset>
fn offset_from_local_datetime(&self, local: &NaiveDateTime) -> LocalResult<Self::Offset>
fn offset_from_utc_date(&self, utc: &NaiveDate) -> Self::Offset
fn offset_from_utc_datetime(&self, utc: &NaiveDateTime) -> Self::Offset
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(tz: &str) -> Result<Self, Self::Err>
```

An Arrow [`TimeZone`]

---

## TzOffset

`struct` · `arrow_array::timezone::private::TzOffset`

Also reachable as `arrow_array::timezone::TzOffset`

```rust
struct TzOffset
```

**Implements**: `chrono::offset::Offset`, `core::fmt::Display`

**Derives**: Clone, Copy, Debug

**via `chrono::offset::Offset`**

```rust
fn fix(&self) -> FixedOffset
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

An [`Offset`] for [`Tz`]

---
