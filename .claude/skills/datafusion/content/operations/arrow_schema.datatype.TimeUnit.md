# `arrow_schema::datatype::TimeUnit`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_schema.datatype.TimeUnit.json).

<a id="op-4e6bd6e27e392e178d8a5b2e"></a>
## TimeUnit

`enum` · `arrow_schema::datatype::TimeUnit` · arrow-schema 59.3.0

```rust
enum TimeUnit
```

Source: `src/datatype.rs:436`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

An absolute length of time in seconds, milliseconds, microseconds or nanoseconds.

<a id="op-4dedd42e59ab58d81ba92eb9"></a>
## Microsecond

`variant` · `arrow_schema::datatype::TimeUnit::Microsecond` · arrow-schema 59.3.0

```rust
Microsecond
```

Source: `src/datatype.rs:442`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Time in microseconds.

<a id="op-cbff9c8e3bfcdc059d5abccf"></a>
## Millisecond

`variant` · `arrow_schema::datatype::TimeUnit::Millisecond` · arrow-schema 59.3.0

```rust
Millisecond
```

Source: `src/datatype.rs:440`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Time in milliseconds.

<a id="op-3b298922bb6810a83bc2cbda"></a>
## Nanosecond

`variant` · `arrow_schema::datatype::TimeUnit::Nanosecond` · arrow-schema 59.3.0

```rust
Nanosecond
```

Source: `src/datatype.rs:444`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Time in nanoseconds.

<a id="op-c42d7dabe782adc7b22bb568"></a>
## Second

`variant` · `arrow_schema::datatype::TimeUnit::Second` · arrow-schema 59.3.0

```rust
Second
```

Source: `src/datatype.rs:438`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

Time in seconds.

<a id="op-77cbf4743f110930fa416a57"></a>
## clone

`function` · `arrow_schema::datatype::TimeUnit::clone` · arrow-schema 59.3.0

```rust
fn clone(&self) -> TimeUnit
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::TimeUnit", "path": "TimeUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [434, 17], "end": [434, 22], "filename": "src/datatype.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/datatype.rs:434`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-779e74a4fe2ec9ae8eeac0dc"></a>
## cmp

`function` · `arrow_schema::datatype::TimeUnit::cmp` · arrow-schema 59.3.0

```rust
fn cmp(&self, other: &TimeUnit) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::TimeUnit", "path": "TimeUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [434, 63], "end": [434, 66], "filename": "src/datatype.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/datatype.rs:434`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc82999344d971cacde8e96c"></a>
## deserialize

`function` · `arrow_schema::datatype::TimeUnit::deserialize` · arrow-schema 59.3.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::TimeUnit", "path": "TimeUnit"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [435, 56], "end": [435, 74], "filename": "src/datatype.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/datatype.rs:435`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-778da97b97e9ac425dd12078"></a>
## eq

`function` · `arrow_schema::datatype::TimeUnit::eq` · arrow-schema 59.3.0

```rust
fn eq(&self, other: &TimeUnit) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::TimeUnit", "path": "TimeUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [434, 30], "end": [434, 39], "filename": "src/datatype.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/datatype.rs:434`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-07cdcfaec210aaa886a3d8e4"></a>
## fmt

`function` · `arrow_schema::datatype::TimeUnit::fmt` · arrow-schema 59.3.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::TimeUnit", "path": "TimeUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [447, 1], "end": [456, 2], "filename": "src/datatype.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/datatype.rs:448`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da613945255c0b68d1a4b107"></a>
## fmt

`function` · `arrow_schema::datatype::TimeUnit::fmt` · arrow-schema 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::TimeUnit", "path": "TimeUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [434, 10], "end": [434, 15], "filename": "src/datatype.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/datatype.rs:434`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8372790054e7704fadc78d8b"></a>
## hash

`function` · `arrow_schema::datatype::TimeUnit::hash` · arrow-schema 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::TimeUnit", "path": "TimeUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [434, 45], "end": [434, 49], "filename": "src/datatype.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/datatype.rs:434`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8103df92bc249f1cd312d161"></a>
## partial_cmp

`function` · `arrow_schema::datatype::TimeUnit::partial_cmp` · arrow-schema 59.3.0

```rust
fn partial_cmp(&self, other: &TimeUnit) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::TimeUnit", "path": "TimeUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [434, 51], "end": [434, 61], "filename": "src/datatype.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/datatype.rs:434`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-67b9dcdb2ab0a267ebaf8cd0"></a>
## serialize

`function` · `arrow_schema::datatype::TimeUnit::serialize` · arrow-schema 59.3.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_schema::datatype::TimeUnit", "path": "TimeUnit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [435, 38], "end": [435, 54], "filename": "src/datatype.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/datatype.rs:435`. [Exact documentation build](https://docs.rs/crate/arrow-schema/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
