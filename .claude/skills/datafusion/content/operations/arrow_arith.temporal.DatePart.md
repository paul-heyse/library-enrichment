# `arrow_arith::temporal::DatePart`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_arith.temporal.DatePart.json).

<a id="op-f5e1db29c834d5950775ec3c"></a>
## DatePart

`enum` · `arrow_arith::temporal::DatePart` · arrow-arith 59.3.0

```rust
enum DatePart
```

Source: `src/temporal.rs:46`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Valid parts to extract from date/time/timestamp arrays.

See [`date_part`](../operations/arrow_arith.temporal.date_part.md#op-1c788fff179b94ab22ae9330).

Marked as non-exhaustive as may expand to support more types of
date parts in the future.

<a id="op-f74d19fc164810246ee9c5bc"></a>
## Day

`variant` · `arrow_arith::temporal::DatePart::Day` · arrow-arith 59.3.0

```rust
Day
```

Source: `src/temporal.rs:60`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Day of the month, in range `1..=31`

<a id="op-1fb53ce45666506ff57555a1"></a>
## DayOfWeekMonday0

`variant` · `arrow_arith::temporal::DatePart::DayOfWeekMonday0` · arrow-arith 59.3.0

```rust
DayOfWeekMonday0
```

Source: `src/temporal.rs:64`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Day of the week, in range `0..=6`, where Monday is `0`

<a id="op-38eb31c4c0d683255ded5323"></a>
## DayOfWeekMonday1

`variant` · `arrow_arith::temporal::DatePart::DayOfWeekMonday1` · arrow-arith 59.3.0

```rust
DayOfWeekMonday1
```

Source: `src/temporal.rs:68`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

ISO day of the week, in range `1..=7`, where Monday is `1`

<a id="op-aa1d1e467bed303b7f70a2fa"></a>
## DayOfWeekSunday0

`variant` · `arrow_arith::temporal::DatePart::DayOfWeekSunday0` · arrow-arith 59.3.0

```rust
DayOfWeekSunday0
```

Source: `src/temporal.rs:62`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Day of the week, in range `0..=6`, where Sunday is `0`

<a id="op-59905e81b1f0190f02c58c44"></a>
## DayOfWeekSunday1

`variant` · `arrow_arith::temporal::DatePart::DayOfWeekSunday1` · arrow-arith 59.3.0

```rust
DayOfWeekSunday1
```

Source: `src/temporal.rs:66`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Day of the week, in range `1..=7`, where Sunday is `1`

<a id="op-c5300d72d631f85ff7f1f4de"></a>
## DayOfYear

`variant` · `arrow_arith::temporal::DatePart::DayOfYear` · arrow-arith 59.3.0

```rust
DayOfYear
```

Source: `src/temporal.rs:70`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Day of year, in range `1..=366`

<a id="op-2c3a0224f555df67df42597c"></a>
## Err

`assoc_type` · `arrow_arith::temporal::DatePart::Err` · arrow-arith 59.3.0

```rust
Err
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_arith::temporal::DatePart", "path": "DatePart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [106, 1], "end": [140, 2], "filename": "src/temporal.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/temporal.rs:107`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d08aa30b615998ae70eb4dfa"></a>
## Hour

`variant` · `arrow_arith::temporal::DatePart::Hour` · arrow-arith 59.3.0

```rust
Hour
```

Source: `src/temporal.rs:72`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Hour of the day, in range `0..=23`

<a id="op-79de0dd6bf7b0762f67be1ed"></a>
## Microsecond

`variant` · `arrow_arith::temporal::DatePart::Microsecond` · arrow-arith 59.3.0

```rust
Microsecond
```

Source: `src/temporal.rs:80`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Microsecond of the second

<a id="op-20182eb5aa53e3eb698ca193"></a>
## Millisecond

`variant` · `arrow_arith::temporal::DatePart::Millisecond` · arrow-arith 59.3.0

```rust
Millisecond
```

Source: `src/temporal.rs:78`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Millisecond of the second

<a id="op-f5373beb47bcb4db70b0390b"></a>
## Minute

`variant` · `arrow_arith::temporal::DatePart::Minute` · arrow-arith 59.3.0

```rust
Minute
```

Source: `src/temporal.rs:74`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Minute of the hour, in range `0..=59`

<a id="op-476604c9b799d1b2679e472b"></a>
## Month

`variant` · `arrow_arith::temporal::DatePart::Month` · arrow-arith 59.3.0

```rust
Month
```

Source: `src/temporal.rs:54`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Month in the year, in range `1..=12`

<a id="op-a88ce094437d693922a2fee8"></a>
## Nanosecond

`variant` · `arrow_arith::temporal::DatePart::Nanosecond` · arrow-arith 59.3.0

```rust
Nanosecond
```

Source: `src/temporal.rs:82`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Nanosecond of the second

<a id="op-cf0f1afa1e0fb9e0f248eb93"></a>
## Quarter

`variant` · `arrow_arith::temporal::DatePart::Quarter` · arrow-arith 59.3.0

```rust
Quarter
```

Source: `src/temporal.rs:48`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Quarter of the year, in range `1..=4`

<a id="op-18b25e956fed93406ecacc2e"></a>
## Second

`variant` · `arrow_arith::temporal::DatePart::Second` · arrow-arith 59.3.0

```rust
Second
```

Source: `src/temporal.rs:76`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Second of the minute, in range `0..=59`

<a id="op-cf9b5b5d09d5b655fd53d2aa"></a>
## Week

`variant` · `arrow_arith::temporal::DatePart::Week` · arrow-arith 59.3.0

```rust
Week
```

Source: `src/temporal.rs:56`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

week of the year, in range `1..=53`, computed as per ISO 8601

<a id="op-dc8f58c6d305fc52c2ffe0f3"></a>
## WeekISO

`variant` · `arrow_arith::temporal::DatePart::WeekISO` · arrow-arith 59.3.0

```rust
WeekISO
```

Source: `src/temporal.rs:58`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

ISO week of the year, in range `1..=53`

<a id="op-fa9e8336e1c84e66a37358ac"></a>
## Year

`variant` · `arrow_arith::temporal::DatePart::Year` · arrow-arith 59.3.0

```rust
Year
```

Source: `src/temporal.rs:50`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

Calendar year

<a id="op-67a5fbe31272c41262fa532b"></a>
## YearISO

`variant` · `arrow_arith::temporal::DatePart::YearISO` · arrow-arith 59.3.0

```rust
YearISO
```

Source: `src/temporal.rs:52`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

ISO year, computed as per ISO 8601

<a id="op-363f658c31164ca1d8b63824"></a>
## clone

`function` · `arrow_arith::temporal::DatePart::clone` · arrow-arith 59.3.0

```rust
fn clone(&self) -> DatePart
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_arith::temporal::DatePart", "path": "DatePart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 17], "end": [44, 22], "filename": "src/temporal.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/temporal.rs:44`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03ae7acb2960c837ccd79427"></a>
## eq

`function` · `arrow_arith::temporal::DatePart::eq` · arrow-arith 59.3.0

```rust
fn eq(&self, other: &DatePart) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_arith::temporal::DatePart", "path": "DatePart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 30], "end": [44, 39], "filename": "src/temporal.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/temporal.rs:44`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-29ae4f58f16153837b473fc0"></a>
## fmt

`function` · `arrow_arith::temporal::DatePart::fmt` · arrow-arith 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_arith::temporal::DatePart", "path": "DatePart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 10], "end": [44, 15], "filename": "src/temporal.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/temporal.rs:44`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5dc1b4050fb8200d95edda8"></a>
## fmt

`function` · `arrow_arith::temporal::DatePart::fmt` · arrow-arith 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_arith::temporal::DatePart", "path": "DatePart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [89, 2], "filename": "src/temporal.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/temporal.rs:86`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25fb42314eb83f238eb867d0"></a>
## from_str

`function` · `arrow_arith::temporal::DatePart::from_str` · arrow-arith 59.3.0

```rust
fn from_str(s: &str) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_arith::temporal::DatePart", "path": "DatePart"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [106, 1], "end": [140, 2], "filename": "src/temporal.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/temporal.rs:109`. [Exact documentation build](https://docs.rs/crate/arrow-arith/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
