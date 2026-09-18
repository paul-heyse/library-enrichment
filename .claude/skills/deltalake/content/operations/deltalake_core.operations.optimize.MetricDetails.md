# `deltalake_core::operations::optimize::MetricDetails`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.optimize.MetricDetails.json).

<a id="op-bf30fca77b17638b3b61ab13"></a>
## MetricDetails

`struct` · `deltalake_core::operations::optimize::MetricDetails` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct MetricDetails
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L191).

Source: `crates/core/src/operations/optimize.rs:191`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Statistics on files for a particular operation
Operation can be remove or add

<a id="op-356a3b02195dd312fafa3f49"></a>
## add

`function` · `deltalake_core::operations::optimize::MetricDetails::add` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn add(&mut self, partial: &MetricDetails)
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L206).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::optimize::MetricDetails", "path": "MetricDetails"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 1], "end": [213, 2], "filename": "crates/core/src/operations/optimize.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/optimize.rs:206`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Add a partial metric to the metrics

<a id="op-43af6760edf3bba704f866f1"></a>
## avg

`struct_field` · `deltalake_core::operations::optimize::MetricDetails::avg` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
avg: f64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L193).

Source: `crates/core/src/operations/optimize.rs:193`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Average file size of a operation

<a id="op-08ec6c95381f2d243e3167de"></a>
## clone

`function` · `deltalake_core::operations::optimize::MetricDetails::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> MetricDetails
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L189).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::optimize::MetricDetails", "path": "MetricDetails"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [189, 28], "end": [189, 33], "filename": "crates/core/src/operations/optimize.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/operations/optimize.rs:189`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95003f35c90f2c2548766bce"></a>
## default

`function` · `deltalake_core::operations::optimize::MetricDetails::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L256).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::optimize::MetricDetails", "path": "MetricDetails"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [265, 2], "filename": "crates/core/src/operations/optimize.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/operations/optimize.rs:256`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71df46ee3f4cfa2d1261f3cd"></a>
## deserialize

`function` · `deltalake_core::operations::optimize::MetricDetails::deserialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L189).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::optimize::MetricDetails", "path": "MetricDetails"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [189, 46], "end": [189, 57], "filename": "crates/core/src/operations/optimize.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/core/src/operations/optimize.rs:189`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ecdd4f48a1dfea6a35f89b1"></a>
## eq

`function` · `deltalake_core::operations::optimize::MetricDetails::eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &MetricDetails) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L189).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::optimize::MetricDetails", "path": "MetricDetails"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [189, 17], "end": [189, 26], "filename": "crates/core/src/operations/optimize.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/core/src/operations/optimize.rs:189`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aeb1183584d93c73959b619d"></a>
## fmt

`function` · `deltalake_core::operations::optimize::MetricDetails::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L189).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::optimize::MetricDetails", "path": "MetricDetails"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [189, 10], "end": [189, 15], "filename": "crates/core/src/operations/optimize.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/operations/optimize.rs:189`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f1c4e15cab901774debe574a"></a>
## fmt

`function` · `deltalake_core::operations::optimize::MetricDetails::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L217).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::optimize::MetricDetails", "path": "MetricDetails"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [215, 1], "end": [220, 2], "filename": "crates/core/src/operations/optimize.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `crates/core/src/operations/optimize.rs:217`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Display the metric details using serde serialization

<a id="op-763493c383eb39507c856c1e"></a>
## max

`struct_field` · `deltalake_core::operations::optimize::MetricDetails::max` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
max: i64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L195).

Source: `crates/core/src/operations/optimize.rs:195`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Maximum file size of a operation

<a id="op-39f59cea9600aa751d27e784"></a>
## min

`struct_field` · `deltalake_core::operations::optimize::MetricDetails::min` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
min: i64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L197).

Source: `crates/core/src/operations/optimize.rs:197`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Minimum file size of a operation

<a id="op-a2dbd4cf0a54d01d59878835"></a>
## serialize

`function` · `deltalake_core::operations::optimize::MetricDetails::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L189).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::optimize::MetricDetails", "path": "MetricDetails"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [189, 35], "end": [189, 44], "filename": "crates/core/src/operations/optimize.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/operations/optimize.rs:189`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d5ff2e489336413fda3e2ec"></a>
## total_files

`struct_field` · `deltalake_core::operations::optimize::MetricDetails::total_files` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
total_files: usize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L199).

Source: `crates/core/src/operations/optimize.rs:199`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Number of files encountered during operation

<a id="op-8c8c0290b1cb295bc2b34371"></a>
## total_size

`struct_field` · `deltalake_core::operations::optimize::MetricDetails::total_size` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
total_size: i64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L201).

Source: `crates/core/src/operations/optimize.rs:201`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Sum of file sizes of a operation
