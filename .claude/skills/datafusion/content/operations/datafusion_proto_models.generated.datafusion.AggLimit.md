# `datafusion_proto_models::generated::datafusion::AggLimit`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_proto_models.generated.datafusion.AggLimit.json).

<a id="op-a61b813af24f3e90ce29ae2e"></a>
## AggLimit

`struct` · `datafusion_proto_models::generated::datafusion::AggLimit` · datafusion-proto-models 55.1.0

```rust
struct AggLimit
```

Source: `src/generated/prost.rs:2206`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8687d278ca440d8822cfaac"></a>
## clear

`function` · `datafusion_proto_models::generated::datafusion::AggLimit::clear` · datafusion-proto-models 55.1.0

```rust
fn clear(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AggLimit", "path": "AggLimit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2205, 44], "end": [2205, 60], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:2205`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb925b1f0271386526fc4306"></a>
## clone

`function` · `datafusion_proto_models::generated::datafusion::AggLimit::clone` · datafusion-proto-models 55.1.0

```rust
fn clone(&self) -> AggLimit
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AggLimit", "path": "AggLimit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2205, 10], "end": [2205, 15], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/generated/prost.rs:2205`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bab8caadaf5887ea6d4af7c8"></a>
## default

`function` · `datafusion_proto_models::generated::datafusion::AggLimit::default` · datafusion-proto-models 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AggLimit", "path": "AggLimit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2205, 44], "end": [2205, 60], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/generated/prost.rs:2205`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9340e25cae1d3f77eb95e221"></a>
## descending

`function` · `datafusion_proto_models::generated::datafusion::AggLimit::descending` · datafusion-proto-models 55.1.0

```rust
fn descending(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AggLimit", "path": "AggLimit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2205, 44], "end": [2205, 60], "filename": "src/generated/prost.rs"}, "trait": null, "trait_path": null}`

Source: `src/generated/prost.rs:2205`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Returns the value of `descending`, or the default value if `descending` is unset.

<a id="op-c641d581b1894a12f8a749e6"></a>
## descending

`struct_field` · `datafusion_proto_models::generated::datafusion::AggLimit::descending` · datafusion-proto-models 55.1.0

```rust
descending: ::core::option::Option<bool>
```

Source: `src/generated/prost.rs:2212`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

Optional ordering direction for TopK aggregation (true = descending, false = ascending)

<a id="op-ed8924d76df0a870527bbab0"></a>
## deserialize

`function` · `datafusion_proto_models::generated::datafusion::AggLimit::deserialize` · datafusion-proto-models 55.1.0

```rust
fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AggLimit", "path": "AggLimit"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [27, 1], "end": [112, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/generated/pbjson.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-312b87a8ed3ffedea7c6da56"></a>
## encoded_len

`function` · `datafusion_proto_models::generated::datafusion::AggLimit::encoded_len` · datafusion-proto-models 55.1.0

```rust
fn encoded_len(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AggLimit", "path": "AggLimit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2205, 44], "end": [2205, 60], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "prost::message::Message", "path": "Message"}, "trait_path": "prost::message::Message"}`

Source: `src/generated/prost.rs:2205`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c4d26bb8c9762ef9e94ebda"></a>
## eq

`function` · `datafusion_proto_models::generated::datafusion::AggLimit::eq` · datafusion-proto-models 55.1.0

```rust
fn eq(&self, other: &AggLimit) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AggLimit", "path": "AggLimit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2205, 23], "end": [2205, 32], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/generated/prost.rs:2205`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06d18cf53b8b30c9adca5cee"></a>
## fmt

`function` · `datafusion_proto_models::generated::datafusion::AggLimit::fmt` · datafusion-proto-models 55.1.0

```rust
fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AggLimit", "path": "AggLimit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2205, 44], "end": [2205, 60], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/generated/prost.rs:2205`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-135ef768007a0eedd2de659d"></a>
## hash

`function` · `datafusion_proto_models::generated::datafusion::AggLimit::hash` · datafusion-proto-models 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AggLimit", "path": "AggLimit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2205, 38], "end": [2205, 42], "filename": "src/generated/prost.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/generated/prost.rs:2205`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b83c4197443bcedc056131a"></a>
## limit

`struct_field` · `datafusion_proto_models::generated::datafusion::AggLimit::limit` · datafusion-proto-models 55.1.0

```rust
limit: u64
```

Source: `src/generated/prost.rs:2209`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

wrap into a message to make it optional

<a id="op-85bfe08f1f94b10819aa72b4"></a>
## serialize

`function` · `datafusion_proto_models::generated::datafusion::AggLimit::serialize` · datafusion-proto-models 55.1.0

```rust
fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_proto_models::generated::datafusion::AggLimit", "path": "AggLimit"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1, 1], "end": [26, 2], "filename": "src/generated/pbjson.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/generated/pbjson.rs:3`. [Exact documentation build](https://docs.rs/crate/datafusion-proto-models/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
