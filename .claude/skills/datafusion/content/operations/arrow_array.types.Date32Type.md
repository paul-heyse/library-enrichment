# `arrow_array::types::Date32Type`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.types.Date32Type.json).

<a id="op-7798afce101ac085c553ac62"></a>
## Date32Type

`struct` · `arrow_array::types::Date32Type` · arrow-array 59.3.0

```rust
struct Date32Type
```

Source: `src/types.rs:186`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

32-bit date type: the elapsed time since UNIX epoch in days (32 bits).

<a id="op-e05929929a4c753c5a806d0f"></a>
## DATA_TYPE

`assoc_const` · `arrow_array::types::Date32Type::DATA_TYPE` · arrow-array 59.3.0

```rust
DATA_TYPE
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::Date32Type", "path": "Date32Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [191, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}, "trait_path": "arrow_array::types::ArrowPrimitiveType"}`

Source: `src/types.rs:186`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1aa55109020ce288fdab0655"></a>
## Native

`assoc_type` · `arrow_array::types::Date32Type::Native` · arrow-array 59.3.0

```rust
Native
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::Date32Type", "path": "Date32Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [191, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "arrow_array::types::ArrowPrimitiveType", "path": "ArrowPrimitiveType"}, "trait_path": "arrow_array::types::ArrowPrimitiveType"}`

Source: `src/types.rs:186`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c18ce6ffd3d7c380efcb35f5"></a>
## add_day_time

`function` · `arrow_array::types::Date32Type::add_day_time` · arrow-array 59.3.0

```rust
fn add_day_time(date: <Date32Type as ArrowPrimitiveType>::Native, delta: <IntervalDayTimeType as ArrowPrimitiveType>::Native) -> <Date32Type as ArrowPrimitiveType>::Native
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::Date32Type", "path": "Date32Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [942, 1], "end": [1205, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:1023`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Adds the given IntervalDayTimeType to an arrow Date32Type

# Arguments

* `date` - The date on which to perform the operation
* `delta` - The interval to add

<a id="op-56a91f566e0359e8dcf94126"></a>
## add_day_time_opt

`function` · `arrow_array::types::Date32Type::add_day_time_opt` · arrow-array 59.3.0

```rust
fn add_day_time_opt(date: <Date32Type as ArrowPrimitiveType>::Native, delta: <IntervalDayTimeType as ArrowPrimitiveType>::Native) -> Option<<Date32Type as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::Date32Type", "path": "Date32Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [942, 1], "end": [1205, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:1040`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Adds the given IntervalDayTimeType to an arrow Date32Type

# Arguments

* `date` - The date on which to perform the operation
* `delta` - The interval to add

Returns `Some(Date32Type)` if it fits, `None` otherwise.

<a id="op-2979619dd0a31af7e2a773e0"></a>
## add_month_day_nano

`function` · `arrow_array::types::Date32Type::add_month_day_nano` · arrow-array 59.3.0

```rust
fn add_month_day_nano(date: <Date32Type as ArrowPrimitiveType>::Native, delta: <IntervalMonthDayNanoType as ArrowPrimitiveType>::Native) -> <Date32Type as ArrowPrimitiveType>::Native
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::Date32Type", "path": "Date32Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [942, 1], "end": [1205, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:1061`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Adds the given IntervalMonthDayNanoType to an arrow Date32Type

# Arguments

* `date` - The date on which to perform the operation
* `delta` - The interval to add

<a id="op-365fc3f77d86af1e7e35b185"></a>
## add_month_day_nano_opt

`function` · `arrow_array::types::Date32Type::add_month_day_nano_opt` · arrow-array 59.3.0

```rust
fn add_month_day_nano_opt(date: <Date32Type as ArrowPrimitiveType>::Native, delta: <IntervalMonthDayNanoType as ArrowPrimitiveType>::Native) -> Option<<Date32Type as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::Date32Type", "path": "Date32Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [942, 1], "end": [1205, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:1078`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Adds the given IntervalMonthDayNanoType to an arrow Date32Type

# Arguments

* `date` - The date on which to perform the operation
* `delta` - The interval to add

Returns `Some(Date32Type)` if it fits, `None` otherwise.

<a id="op-3d92af059a87e659dc95c5b3"></a>
## add_year_months

`function` · `arrow_array::types::Date32Type::add_year_months` · arrow-array 59.3.0

```rust
fn add_year_months(date: <Date32Type as ArrowPrimitiveType>::Native, delta: <IntervalYearMonthType as ArrowPrimitiveType>::Native) -> <Date32Type as ArrowPrimitiveType>::Native
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::Date32Type", "path": "Date32Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [942, 1], "end": [1205, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:986`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Adds the given IntervalYearMonthType to an arrow Date32Type

# Arguments

* `date` - The date on which to perform the operation
* `delta` - The interval to add

<a id="op-16c710fa6c060bd376245693"></a>
## add_year_months_opt

`function` · `arrow_array::types::Date32Type::add_year_months_opt` · arrow-array 59.3.0

```rust
fn add_year_months_opt(date: <Date32Type as ArrowPrimitiveType>::Native, delta: <IntervalYearMonthType as ArrowPrimitiveType>::Native) -> Option<<Date32Type as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::Date32Type", "path": "Date32Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [942, 1], "end": [1205, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:1003`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Adds the given IntervalYearMonthType to an arrow Date32Type

# Arguments

* `date` - The date on which to perform the operation
* `delta` - The interval to add

Returns `Some(Date32Type)` if it fits, `None` otherwise.

<a id="op-953e70a77dd9583df1acf2c9"></a>
## fmt

`function` · `arrow_array::types::Date32Type::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::Date32Type", "path": "Date32Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [186, 1], "end": [191, 2], "filename": "src/types.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/types.rs:186`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-54bd3e9178463de04173837b"></a>
## from_naive_date

`function` · `arrow_array::types::Date32Type::from_naive_date` · arrow-array 59.3.0

```rust
fn from_naive_date(d: NaiveDate) -> <Date32Type as ArrowPrimitiveType>::Native
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::Date32Type", "path": "Date32Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [942, 1], "end": [1205, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:971`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Converts a chrono::NaiveDate into an arrow Date32Type

# Arguments

* `d` - The NaiveDate to convert

<a id="op-1caed0f78301cb07c89d89a3"></a>
## subtract_day_time

`function` · `arrow_array::types::Date32Type::subtract_day_time` · arrow-array 59.3.0

```rust
fn subtract_day_time(date: <Date32Type as ArrowPrimitiveType>::Native, delta: <IntervalDayTimeType as ArrowPrimitiveType>::Native) -> <Date32Type as ArrowPrimitiveType>::Native
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::Date32Type", "path": "Date32Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [942, 1], "end": [1205, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:1137`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Subtract the given IntervalDayTimeType to an arrow Date32Type

# Arguments

* `date` - The date on which to perform the operation
* `delta` - The interval to subtract

<a id="op-2d2273cca6267d662b7552f4"></a>
## subtract_day_time_opt

`function` · `arrow_array::types::Date32Type::subtract_day_time_opt` · arrow-array 59.3.0

```rust
fn subtract_day_time_opt(date: <Date32Type as ArrowPrimitiveType>::Native, delta: <IntervalDayTimeType as ArrowPrimitiveType>::Native) -> Option<<Date32Type as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::Date32Type", "path": "Date32Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [942, 1], "end": [1205, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:1154`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Subtract the given IntervalDayTimeType to an arrow Date32Type

# Arguments

* `date` - The date on which to perform the operation
* `delta` - The interval to subtract

Returns `Some(Date32Type)` if it fits, `None` otherwise.

<a id="op-ea5faf957c8511fdc856e71e"></a>
## subtract_month_day_nano

`function` · `arrow_array::types::Date32Type::subtract_month_day_nano` · arrow-array 59.3.0

```rust
fn subtract_month_day_nano(date: <Date32Type as ArrowPrimitiveType>::Native, delta: <IntervalMonthDayNanoType as ArrowPrimitiveType>::Native) -> <Date32Type as ArrowPrimitiveType>::Native
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::Date32Type", "path": "Date32Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [942, 1], "end": [1205, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:1175`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Subtract the given IntervalMonthDayNanoType to an arrow Date32Type

# Arguments

* `date` - The date on which to perform the operation
* `delta` - The interval to subtract

<a id="op-9c5134cb761e1ceb4f9665bb"></a>
## subtract_month_day_nano_opt

`function` · `arrow_array::types::Date32Type::subtract_month_day_nano_opt` · arrow-array 59.3.0

```rust
fn subtract_month_day_nano_opt(date: <Date32Type as ArrowPrimitiveType>::Native, delta: <IntervalMonthDayNanoType as ArrowPrimitiveType>::Native) -> Option<<Date32Type as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::Date32Type", "path": "Date32Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [942, 1], "end": [1205, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:1194`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Subtract the given IntervalMonthDayNanoType to an arrow Date32Type

# Arguments

* `date` - The date on which to perform the operation
* `delta` - The interval to subtract

Returns `Some(Date32Type)` if it fits, `None` otherwise.

<a id="op-1c0639221341ab4f86aac0d5"></a>
## subtract_year_months

`function` · `arrow_array::types::Date32Type::subtract_year_months` · arrow-array 59.3.0

```rust
fn subtract_year_months(date: <Date32Type as ArrowPrimitiveType>::Native, delta: <IntervalYearMonthType as ArrowPrimitiveType>::Native) -> <Date32Type as ArrowPrimitiveType>::Native
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::Date32Type", "path": "Date32Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [942, 1], "end": [1205, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:1100`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Subtract the given IntervalYearMonthType to an arrow Date32Type

# Arguments

* `date` - The date on which to perform the operation
* `delta` - The interval to subtract

<a id="op-0f8c6022fc259dfe2064d979"></a>
## subtract_year_months_opt

`function` · `arrow_array::types::Date32Type::subtract_year_months_opt` · arrow-array 59.3.0

```rust
fn subtract_year_months_opt(date: <Date32Type as ArrowPrimitiveType>::Native, delta: <IntervalYearMonthType as ArrowPrimitiveType>::Native) -> Option<<Date32Type as ArrowPrimitiveType>::Native>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::Date32Type", "path": "Date32Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [942, 1], "end": [1205, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:1117`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Subtract the given IntervalYearMonthType to an arrow Date32Type

# Arguments

* `date` - The date on which to perform the operation
* `delta` - The interval to subtract

Returns `Some(Date32Type)` if it fits, `None` otherwise.

<a id="op-ebe4f3c37aeee03dd05f5823"></a>
## to_naive_date

`function` · `arrow_array::types::Date32Type::to_naive_date` · arrow-array 59.3.0

```rust
fn to_naive_date(i: <Date32Type as ArrowPrimitiveType>::Native) -> NaiveDate
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::Date32Type", "path": "Date32Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [942, 1], "end": [1205, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:949`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Converts an arrow Date32Type into a chrono::NaiveDate

# Arguments

* `i` - The Date32Type to convert

<a id="op-0eff1a445796c8580a070c6c"></a>
## to_naive_date_opt

`function` · `arrow_array::types::Date32Type::to_naive_date_opt` · arrow-array 59.3.0

```rust
fn to_naive_date_opt(i: <Date32Type as ArrowPrimitiveType>::Native) -> Option<NaiveDate>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::types::Date32Type", "path": "Date32Type"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [942, 1], "end": [1205, 2], "filename": "src/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/types.rs:961`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Converts an arrow Date32Type into a chrono::NaiveDate

# Arguments

* `i` - The Date32Type to convert

Returns `Some(NaiveDate)` if it fits, `None` otherwise.
