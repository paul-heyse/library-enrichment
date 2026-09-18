# `sqlparser::ast::value::DateTimeField`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.value.DateTimeField.json).

<a id="op-e1c76b31457ce16b158c0a10"></a>
## DateTimeField

`enum` · `sqlparser::ast::value::DateTimeField` · sqlparser 0.62.0

```rust
enum DateTimeField
```

Source: `src/ast/value.rs:349`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents the date/time fields used by functions like `EXTRACT`.

Each variant corresponds to a supported date/time part (for example
`YEAR`, `MONTH`, `DAY`, etc.). The `Custom` variant allows arbitrary
identifiers (e.g. dialect-specific abbreviations).

<a id="op-d7cbab4acf39b08387337ba9"></a>
## Century

`variant` · `sqlparser::ast::value::DateTimeField::Century` · sqlparser 0.62.0

```rust
Century
```

Source: `src/ast/value.rs:389`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`CENTURY`

<a id="op-463356e7f50300d10d7ec3f7"></a>
## Custom

`variant` · `sqlparser::ast::value::DateTimeField::Custom` · sqlparser 0.62.0

```rust
Custom
```

Source: `src/ast/value.rs:444`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Arbitrary abbreviation or custom date-time part.

```sql
EXTRACT(q FROM CURRENT_TIMESTAMP)
```
[Snowflake](https://docs.snowflake.com/en/sql-reference/functions-date-time#supported-date-and-time-parts)

<a id="op-e6a091616ba6536cd9a5982d"></a>
## Date

`variant` · `sqlparser::ast::value::DateTimeField::Date` · sqlparser 0.62.0

```rust
Date
```

Source: `src/ast/value.rs:373`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DATE`

<a id="op-ede3ca3b267bf82ff9373d0d"></a>
## Datetime

`variant` · `sqlparser::ast::value::DateTimeField::Datetime` · sqlparser 0.62.0

```rust
Datetime
```

Source: `src/ast/value.rs:375`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DATETIME`

<a id="op-dcaf139b385b915480058fe3"></a>
## Day

`variant` · `sqlparser::ast::value::DateTimeField::Day` · sqlparser 0.62.0

```rust
Day
```

Source: `src/ast/value.rs:365`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DAY`

<a id="op-6e8e26beb68583369b554a51"></a>
## DayOfWeek

`variant` · `sqlparser::ast::value::DateTimeField::DayOfWeek` · sqlparser 0.62.0

```rust
DayOfWeek
```

Source: `src/ast/value.rs:367`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DAYOFWEEK`

<a id="op-8468d888d45de3a7aebf5422"></a>
## DayOfYear

`variant` · `sqlparser::ast::value::DateTimeField::DayOfYear` · sqlparser 0.62.0

```rust
DayOfYear
```

Source: `src/ast/value.rs:369`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DAYOFYEAR`

<a id="op-fd5c2c4cf1c4401b9d3a18c4"></a>
## Days

`variant` · `sqlparser::ast::value::DateTimeField::Days` · sqlparser 0.62.0

```rust
Days
```

Source: `src/ast/value.rs:371`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DAYS` (plural form)

<a id="op-182d440ad9e91d49ce100713"></a>
## Decade

`variant` · `sqlparser::ast::value::DateTimeField::Decade` · sqlparser 0.62.0

```rust
Decade
```

Source: `src/ast/value.rs:391`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DECADE`

<a id="op-922551c49988b4f5da9a77d9"></a>
## Dow

`variant` · `sqlparser::ast::value::DateTimeField::Dow` · sqlparser 0.62.0

```rust
Dow
```

Source: `src/ast/value.rs:393`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DOW` (day of week short form)

<a id="op-5e93278b62c3ba9fb9882509"></a>
## Doy

`variant` · `sqlparser::ast::value::DateTimeField::Doy` · sqlparser 0.62.0

```rust
Doy
```

Source: `src/ast/value.rs:395`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DOY` (day of year short form)

<a id="op-df1f0b70e668131b68c4ef86"></a>
## Epoch

`variant` · `sqlparser::ast::value::DateTimeField::Epoch` · sqlparser 0.62.0

```rust
Epoch
```

Source: `src/ast/value.rs:397`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`EPOCH`

<a id="op-9d570fd11a214eb3e9105ef5"></a>
## Hour

`variant` · `sqlparser::ast::value::DateTimeField::Hour` · sqlparser 0.62.0

```rust
Hour
```

Source: `src/ast/value.rs:377`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`HOUR`

<a id="op-bb107e82740b0b2afe0847a0"></a>
## Hours

`variant` · `sqlparser::ast::value::DateTimeField::Hours` · sqlparser 0.62.0

```rust
Hours
```

Source: `src/ast/value.rs:379`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`HOURS` (plural form)

<a id="op-f022dde7a1533de519fecef2"></a>
## IsoWeek

`variant` · `sqlparser::ast::value::DateTimeField::IsoWeek` · sqlparser 0.62.0

```rust
IsoWeek
```

Source: `src/ast/value.rs:403`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ISOWEEK`

<a id="op-223c6768eb350465176e948e"></a>
## Isodow

`variant` · `sqlparser::ast::value::DateTimeField::Isodow` · sqlparser 0.62.0

```rust
Isodow
```

Source: `src/ast/value.rs:399`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ISODOW`

<a id="op-0128bb0f899facf56cbe097e"></a>
## Isoyear

`variant` · `sqlparser::ast::value::DateTimeField::Isoyear` · sqlparser 0.62.0

```rust
Isoyear
```

Source: `src/ast/value.rs:401`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ISOYEAR`

<a id="op-54d1d416b1601aa8fe00a9d1"></a>
## Julian

`variant` · `sqlparser::ast::value::DateTimeField::Julian` · sqlparser 0.62.0

```rust
Julian
```

Source: `src/ast/value.rs:405`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`JULIAN`

<a id="op-e2f9d43a0fee0f00b3a818da"></a>
## Microsecond

`variant` · `sqlparser::ast::value::DateTimeField::Microsecond` · sqlparser 0.62.0

```rust
Microsecond
```

Source: `src/ast/value.rs:407`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`MICROSECOND`

<a id="op-1154f85d12c06eccf4a742f7"></a>
## Microseconds

`variant` · `sqlparser::ast::value::DateTimeField::Microseconds` · sqlparser 0.62.0

```rust
Microseconds
```

Source: `src/ast/value.rs:409`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`MICROSECONDS` (plural form)

<a id="op-c468b13476e0aba9850d3834"></a>
## Millenium

`variant` · `sqlparser::ast::value::DateTimeField::Millenium` · sqlparser 0.62.0

```rust
Millenium
```

Source: `src/ast/value.rs:411`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`MILLENIUM` (alternate spelling)

<a id="op-9735b107d7fb887e86eaba83"></a>
## Millennium

`variant` · `sqlparser::ast::value::DateTimeField::Millennium` · sqlparser 0.62.0

```rust
Millennium
```

Source: `src/ast/value.rs:413`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`MILLENNIUM` (alternate spelling)

<a id="op-28cbdc1b0551ac8a9ba52797"></a>
## Millisecond

`variant` · `sqlparser::ast::value::DateTimeField::Millisecond` · sqlparser 0.62.0

```rust
Millisecond
```

Source: `src/ast/value.rs:415`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`MILLISECOND`

<a id="op-eac071d43737f482c2906c24"></a>
## Milliseconds

`variant` · `sqlparser::ast::value::DateTimeField::Milliseconds` · sqlparser 0.62.0

```rust
Milliseconds
```

Source: `src/ast/value.rs:417`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`MILLISECONDS` (plural form)

<a id="op-3d742b42faa9374b72850571"></a>
## Minute

`variant` · `sqlparser::ast::value::DateTimeField::Minute` · sqlparser 0.62.0

```rust
Minute
```

Source: `src/ast/value.rs:381`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`MINUTE`

<a id="op-c5e248fad937575648e7e571"></a>
## Minutes

`variant` · `sqlparser::ast::value::DateTimeField::Minutes` · sqlparser 0.62.0

```rust
Minutes
```

Source: `src/ast/value.rs:383`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`MINUTES` (plural form)

<a id="op-13ad08a1eaba6e6b50b195ee"></a>
## Month

`variant` · `sqlparser::ast::value::DateTimeField::Month` · sqlparser 0.62.0

```rust
Month
```

Source: `src/ast/value.rs:355`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`MONTH`

<a id="op-f241a82fcb80e306aa54edff"></a>
## Months

`variant` · `sqlparser::ast::value::DateTimeField::Months` · sqlparser 0.62.0

```rust
Months
```

Source: `src/ast/value.rs:357`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`MONTHS` (plural form)

<a id="op-62892dfc81bad8dfa1cb94d1"></a>
## Nanosecond

`variant` · `sqlparser::ast::value::DateTimeField::Nanosecond` · sqlparser 0.62.0

```rust
Nanosecond
```

Source: `src/ast/value.rs:419`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`NANOSECOND`

<a id="op-95e30ef236a6fa7cd8936317"></a>
## Nanoseconds

`variant` · `sqlparser::ast::value::DateTimeField::Nanoseconds` · sqlparser 0.62.0

```rust
Nanoseconds
```

Source: `src/ast/value.rs:421`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`NANOSECONDS` (plural form)

<a id="op-eaf83eec310b6b37a31f288f"></a>
## NoDateTime

`variant` · `sqlparser::ast::value::DateTimeField::NoDateTime` · sqlparser 0.62.0

```rust
NoDateTime
```

Source: `src/ast/value.rs:437`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`NODATETIME` indicates no date/time part

<a id="op-2d3f204d0e406ea9fd49e174"></a>
## Quarter

`variant` · `sqlparser::ast::value::DateTimeField::Quarter` · sqlparser 0.62.0

```rust
Quarter
```

Source: `src/ast/value.rs:423`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`QUARTER`

<a id="op-172fe3e1e7daaa6f9a2bf2ba"></a>
## Second

`variant` · `sqlparser::ast::value::DateTimeField::Second` · sqlparser 0.62.0

```rust
Second
```

Source: `src/ast/value.rs:385`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SECOND`

<a id="op-c26f8f52338ffa957b856f9f"></a>
## Seconds

`variant` · `sqlparser::ast::value::DateTimeField::Seconds` · sqlparser 0.62.0

```rust
Seconds
```

Source: `src/ast/value.rs:387`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SECONDS` (plural form)

<a id="op-3d8dfc0e5ad2e1d617250858"></a>
## Time

`variant` · `sqlparser::ast::value::DateTimeField::Time` · sqlparser 0.62.0

```rust
Time
```

Source: `src/ast/value.rs:425`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`TIME`

<a id="op-dbea5f56d410a36d3747ffbb"></a>
## Timezone

`variant` · `sqlparser::ast::value::DateTimeField::Timezone` · sqlparser 0.62.0

```rust
Timezone
```

Source: `src/ast/value.rs:427`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`TIMEZONE`

<a id="op-ecb1354ad6d989b50e618615"></a>
## TimezoneAbbr

`variant` · `sqlparser::ast::value::DateTimeField::TimezoneAbbr` · sqlparser 0.62.0

```rust
TimezoneAbbr
```

Source: `src/ast/value.rs:429`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`TIMEZONE_ABBR`

<a id="op-90bd0bbeb80ba71730039710"></a>
## TimezoneHour

`variant` · `sqlparser::ast::value::DateTimeField::TimezoneHour` · sqlparser 0.62.0

```rust
TimezoneHour
```

Source: `src/ast/value.rs:431`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`TIMEZONE_HOUR`

<a id="op-7fbf74526229c2cc7b8662e0"></a>
## TimezoneMinute

`variant` · `sqlparser::ast::value::DateTimeField::TimezoneMinute` · sqlparser 0.62.0

```rust
TimezoneMinute
```

Source: `src/ast/value.rs:433`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`TIMEZONE_MINUTE`

<a id="op-aa1814ac1b76930d27599082"></a>
## TimezoneRegion

`variant` · `sqlparser::ast::value::DateTimeField::TimezoneRegion` · sqlparser 0.62.0

```rust
TimezoneRegion
```

Source: `src/ast/value.rs:435`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`TIMEZONE_REGION`

<a id="op-bb3580d6b37a6dc7eebd18d2"></a>
## Week

`variant` · `sqlparser::ast::value::DateTimeField::Week` · sqlparser 0.62.0

```rust
Week
```

Source: `src/ast/value.rs:361`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`WEEK`, optionally followed by a weekday, e.g. `WEEK(MONDAY)`.

[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/date_functions#extract)

<a id="op-3b0f212c9fbf5fba688cbc9e"></a>
## Weeks

`variant` · `sqlparser::ast::value::DateTimeField::Weeks` · sqlparser 0.62.0

```rust
Weeks
```

Source: `src/ast/value.rs:363`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`WEEKS` (plural form)

<a id="op-7cb493514fa1bb3c161d1487"></a>
## Year

`variant` · `sqlparser::ast::value::DateTimeField::Year` · sqlparser 0.62.0

```rust
Year
```

Source: `src/ast/value.rs:351`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`YEAR`

<a id="op-6fc690000e5a30af406e9359"></a>
## Years

`variant` · `sqlparser::ast::value::DateTimeField::Years` · sqlparser 0.62.0

```rust
Years
```

Source: `src/ast/value.rs:353`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`YEARS` (plural form)

<a id="op-1e962cd5f1782ac1ae6319dd"></a>
## clone

`function` · `sqlparser::ast::value::DateTimeField::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> DateTimeField
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::DateTimeField", "path": "DateTimeField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [346, 17], "end": [346, 22], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/value.rs:346`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3124ee2e1c191f1547e3e00a"></a>
## cmp

`function` · `sqlparser::ast::value::DateTimeField::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &DateTimeField) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::DateTimeField", "path": "DateTimeField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [346, 39], "end": [346, 42], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/value.rs:346`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d465d12e6dac190ec6920a6"></a>
## deserialize

`function` · `sqlparser::ast::value::DateTimeField::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::DateTimeField", "path": "DateTimeField"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [347, 49], "end": [347, 60], "filename": "src/ast/value.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/value.rs:347`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6b936fd7233ffb45b60730c"></a>
## eq

`function` · `sqlparser::ast::value::DateTimeField::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &DateTimeField) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::DateTimeField", "path": "DateTimeField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [346, 24], "end": [346, 33], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/value.rs:346`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1860f801a25eabd412771a75"></a>
## fmt

`function` · `sqlparser::ast::value::DateTimeField::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::DateTimeField", "path": "DateTimeField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [447, 1], "end": [502, 2], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/value.rs:448`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b95073c84a59843eecfabaf2"></a>
## fmt

`function` · `sqlparser::ast::value::DateTimeField::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::DateTimeField", "path": "DateTimeField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [346, 10], "end": [346, 15], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/value.rs:346`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f1d1d2e019a96705c1e71c0c"></a>
## hash

`function` · `sqlparser::ast::value::DateTimeField::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::DateTimeField", "path": "DateTimeField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [346, 56], "end": [346, 60], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/value.rs:346`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dcbd06cbd93487715875b313"></a>
## partial_cmp

`function` · `sqlparser::ast::value::DateTimeField::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &DateTimeField) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::DateTimeField", "path": "DateTimeField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [346, 44], "end": [346, 54], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/value.rs:346`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff8d5a679304638dd29c6f0e"></a>
## serialize

`function` · `sqlparser::ast::value::DateTimeField::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::DateTimeField", "path": "DateTimeField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [347, 38], "end": [347, 47], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/value.rs:347`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94416388192ed9c64ee04dfa"></a>
## visit

`function` · `sqlparser::ast::value::DateTimeField::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::DateTimeField", "path": "DateTimeField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [348, 40], "end": [348, 45], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/value.rs:348`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5ee7b3802b3ce1b095255b1"></a>
## visit

`function` · `sqlparser::ast::value::DateTimeField::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::value::DateTimeField", "path": "DateTimeField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [348, 47], "end": [348, 55], "filename": "src/ast/value.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/value.rs:348`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
