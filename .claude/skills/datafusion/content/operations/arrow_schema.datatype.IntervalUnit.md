# `arrow_schema::datatype::IntervalUnit`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_schema.datatype.IntervalUnit.json).

<a id="op-5f824e96bbdb1d739f87fdc2"></a>
## IntervalUnit

`enum` · `arrow_schema::datatype::IntervalUnit` · arrow-schema 59.3.0

```rust
enum IntervalUnit
```

Source: `src/datatype.rs:461`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

YEAR_MONTH, DAY_TIME, MONTH_DAY_NANO interval in SQL style.

<a id="op-b476388771e449f28a2e80e6"></a>
## DayTime

`variant` · `arrow_schema::datatype::IntervalUnit::DayTime` · arrow-schema 59.3.0

```rust
DayTime
```

Source: `src/datatype.rs:466`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Indicates the number of elapsed days and milliseconds,
stored as 2 contiguous 32-bit integers (days, milliseconds) (8-bytes in total).

<a id="op-3abb6fc3820cf54f31ac7939"></a>
## MonthDayNano

`variant` · `arrow_schema::datatype::IntervalUnit::MonthDayNano` · arrow-schema 59.3.0

```rust
MonthDayNano
```

Source: `src/datatype.rs:474`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

A triple of the number of elapsed months, days, and nanoseconds.
The values are stored contiguously in 16 byte blocks. Months and
days are encoded as 32 bit integers and nanoseconds is encoded as a
64 bit integer. All integers are signed. Each field is independent
(e.g. there is no constraint that nanoseconds have the same sign
as days or that the quantity of nanoseconds represents less
than a day's worth of time).

<a id="op-0fcbc8eab5d1c4df80148a43"></a>
## YearMonth

`variant` · `arrow_schema::datatype::IntervalUnit::YearMonth` · arrow-schema 59.3.0

```rust
YearMonth
```

Source: `src/datatype.rs:463`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Indicates the number of elapsed whole months, stored as 4-byte integers.

<a id="op-f26d28f809a5d0c037ea907b"></a>
## clone

`function` · `arrow_schema::datatype::IntervalUnit::clone` · arrow-schema 59.3.0

```rust
fn clone(&self) -> IntervalUnit
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::IntervalUnit", "path": "IntervalUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [459, 17], "end": [459, 22], "filename": "src/datatype.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/datatype.rs:459`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dd9ce1d212eaed9a366bddf4"></a>
## cmp

`function` · `arrow_schema::datatype::IntervalUnit::cmp` · arrow-schema 59.3.0

```rust
fn cmp(&self, other: &IntervalUnit) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::IntervalUnit", "path": "IntervalUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [459, 63], "end": [459, 66], "filename": "src/datatype.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/datatype.rs:459`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff8d933dd8c6075e6a6e57df"></a>
## deserialize

`function` · `arrow_schema::datatype::IntervalUnit::deserialize` · arrow-schema 59.3.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::IntervalUnit", "path": "IntervalUnit"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [460, 56], "end": [460, 74], "filename": "src/datatype.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/datatype.rs:460`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f90abea227ce29665fdaf763"></a>
## eq

`function` · `arrow_schema::datatype::IntervalUnit::eq` · arrow-schema 59.3.0

```rust
fn eq(&self, other: &IntervalUnit) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::IntervalUnit", "path": "IntervalUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [459, 30], "end": [459, 39], "filename": "src/datatype.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/datatype.rs:459`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e662f3b55845e7c5085f136d"></a>
## fmt

`function` · `arrow_schema::datatype::IntervalUnit::fmt` · arrow-schema 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::IntervalUnit", "path": "IntervalUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [459, 10], "end": [459, 15], "filename": "src/datatype.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/datatype.rs:459`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ceb3432f9f52cd992d73447"></a>
## hash

`function` · `arrow_schema::datatype::IntervalUnit::hash` · arrow-schema 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::IntervalUnit", "path": "IntervalUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [459, 45], "end": [459, 49], "filename": "src/datatype.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/datatype.rs:459`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-213daa204a7084e45dcf606d"></a>
## partial_cmp

`function` · `arrow_schema::datatype::IntervalUnit::partial_cmp` · arrow-schema 59.3.0

```rust
fn partial_cmp(&self, other: &IntervalUnit) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::IntervalUnit", "path": "IntervalUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [459, 51], "end": [459, 61], "filename": "src/datatype.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/datatype.rs:459`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e988644e93169101b0d1cb70"></a>
## serialize

`function` · `arrow_schema::datatype::IntervalUnit::serialize` · arrow-schema 59.3.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::IntervalUnit", "path": "IntervalUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [460, 38], "end": [460, 54], "filename": "src/datatype.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/datatype.rs:460`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
