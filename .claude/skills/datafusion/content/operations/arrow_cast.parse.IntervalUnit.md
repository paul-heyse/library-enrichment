# `arrow_cast::parse::IntervalUnit`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.parse.IntervalUnit.json).

<a id="op-4ab4a208cc8baaea80314c32"></a>
## IntervalUnit

`enum` · `arrow_cast::parse::IntervalUnit` · arrow-cast 59.3.0

```rust
enum IntervalUnit
```

Source: `src/parse.rs:1122`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Represents the units of an interval, with each variant
corresponding to a bit in the interval's bitfield representation

<a id="op-f219babbd644bda338b2d644"></a>
## Century

`variant` · `arrow_cast::parse::IntervalUnit::Century` · arrow-cast 59.3.0

```rust
Century
```

Source: `src/parse.rs:1124`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

A Century

<a id="op-94fbae9be0c1e74858cedd4c"></a>
## Day

`variant` · `arrow_cast::parse::IntervalUnit::Day` · arrow-cast 59.3.0

```rust
Day
```

Source: `src/parse.rs:1134`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

A Day

<a id="op-ebff359103def2dcce9f34e4"></a>
## Decade

`variant` · `arrow_cast::parse::IntervalUnit::Decade` · arrow-cast 59.3.0

```rust
Decade
```

Source: `src/parse.rs:1126`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

A Decade

<a id="op-96182230109a0de6f74391f8"></a>
## Err

`assoc_type` · `arrow_cast::parse::IntervalUnit::Err` · arrow-cast 59.3.0

```rust
Err
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_cast::parse::IntervalUnit", "path": "IntervalUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1153, 1], "end": [1179, 2], "filename": "src/parse.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/parse.rs:1154`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb5bab96389f692a5b0e664e"></a>
## Hour

`variant` · `arrow_cast::parse::IntervalUnit::Hour` · arrow-cast 59.3.0

```rust
Hour
```

Source: `src/parse.rs:1136`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

An Hour

<a id="op-e4e29ff440c3b78e976e8e4d"></a>
## Microsecond

`variant` · `arrow_cast::parse::IntervalUnit::Microsecond` · arrow-cast 59.3.0

```rust
Microsecond
```

Source: `src/parse.rs:1144`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

A Microsecond

<a id="op-830b28a3e921c353bd7a505e"></a>
## Millisecond

`variant` · `arrow_cast::parse::IntervalUnit::Millisecond` · arrow-cast 59.3.0

```rust
Millisecond
```

Source: `src/parse.rs:1142`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

A Millisecond

<a id="op-cc7ca9f56a5559958abf9f3c"></a>
## Minute

`variant` · `arrow_cast::parse::IntervalUnit::Minute` · arrow-cast 59.3.0

```rust
Minute
```

Source: `src/parse.rs:1138`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

A Minute

<a id="op-40b259c63c6cc93b2c28d482"></a>
## Month

`variant` · `arrow_cast::parse::IntervalUnit::Month` · arrow-cast 59.3.0

```rust
Month
```

Source: `src/parse.rs:1130`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

A Month

<a id="op-127b2b6c80a8b63cc5379636"></a>
## Nanosecond

`variant` · `arrow_cast::parse::IntervalUnit::Nanosecond` · arrow-cast 59.3.0

```rust
Nanosecond
```

Source: `src/parse.rs:1146`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

A Nanosecond

<a id="op-99d56c4862f876ae9d09867e"></a>
## Second

`variant` · `arrow_cast::parse::IntervalUnit::Second` · arrow-cast 59.3.0

```rust
Second
```

Source: `src/parse.rs:1140`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

A Second

<a id="op-8ccbe06c568182baf2f5dcbf"></a>
## Week

`variant` · `arrow_cast::parse::IntervalUnit::Week` · arrow-cast 59.3.0

```rust
Week
```

Source: `src/parse.rs:1132`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

A Week

<a id="op-c3d3c3d83ddeb9951b61c329"></a>
## Year

`variant` · `arrow_cast::parse::IntervalUnit::Year` · arrow-cast 59.3.0

```rust
Year
```

Source: `src/parse.rs:1128`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

A Year

<a id="op-9f2eab09e0ed640bd5c084a9"></a>
## clone

`function` · `arrow_cast::parse::IntervalUnit::clone` · arrow-cast 59.3.0

```rust
fn clone(&self) -> IntervalUnit
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_cast::parse::IntervalUnit", "path": "IntervalUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1118, 17], "end": [1118, 22], "filename": "src/parse.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/parse.rs:1118`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7c487f997568aa86e43dac9"></a>
## fmt

`function` · `arrow_cast::parse::IntervalUnit::fmt` · arrow-cast 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_cast::parse::IntervalUnit", "path": "IntervalUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1118, 10], "end": [1118, 15], "filename": "src/parse.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/parse.rs:1118`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf4cfa694fea482630c563f6"></a>
## from_str

`function` · `arrow_cast::parse::IntervalUnit::from_str` · arrow-cast 59.3.0

```rust
fn from_str(s: &str) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_cast::parse::IntervalUnit", "path": "IntervalUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1153, 1], "end": [1179, 2], "filename": "src/parse.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/parse.rs:1156`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
