# `deltalake_core::operations::optimize::PlannerStrategy`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.optimize.PlannerStrategy.json).

<a id="op-3f6d11ac0ced4cf00f298faf"></a>
## PlannerStrategy

`enum` · `deltalake_core::operations::optimize::PlannerStrategy` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum PlannerStrategy
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L74).

Source: `crates/core/src/operations/optimize.rs:74`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Planner used by optimize.

<a id="op-1eb1309accb11f49ed34358b"></a>
## PreserveLocality

`variant` · `deltalake_core::operations::optimize::PlannerStrategy::PreserveLocality` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
PreserveLocality
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L79).

Source: `crates/core/src/operations/optimize.rs:79`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Compact planner.

<a id="op-653d3e873bbb0b646619cb29"></a>
## UnknownLegacy

`variant` · `deltalake_core::operations::optimize::PlannerStrategy::UnknownLegacy` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
UnknownLegacy
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L77).

Source: `crates/core/src/operations/optimize.rs:77`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Older metrics with no planner field.

<a id="op-ebedf4411f080b7b95e64c86"></a>
## ZOrder

`variant` · `deltalake_core::operations::optimize::PlannerStrategy::ZOrder` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
ZOrder
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L81).

Source: `crates/core/src/operations/optimize.rs:81`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Z order planner.

<a id="op-9665aee060dbe254e67458a0"></a>
## clone

`function` · `deltalake_core::operations::optimize::PlannerStrategy::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> PlannerStrategy
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L72).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::optimize::PlannerStrategy", "path": "PlannerStrategy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 17], "end": [72, 22], "filename": "crates/core/src/operations/optimize.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/operations/optimize.rs:72`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d858229873cd399543c186cc"></a>
## default

`function` · `deltalake_core::operations::optimize::PlannerStrategy::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> PlannerStrategy
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L72).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::optimize::PlannerStrategy", "path": "PlannerStrategy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 30], "end": [72, 37], "filename": "crates/core/src/operations/optimize.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/operations/optimize.rs:72`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae5d86eaa15717e2a89c13f9"></a>
## deserialize

`function` · `deltalake_core::operations::optimize::PlannerStrategy::deserialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L72).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::optimize::PlannerStrategy", "path": "PlannerStrategy"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 65], "end": [72, 76], "filename": "crates/core/src/operations/optimize.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/core/src/operations/optimize.rs:72`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf04879a3d0a0eb4f0c90153"></a>
## eq

`function` · `deltalake_core::operations::optimize::PlannerStrategy::eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &PlannerStrategy) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L72).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::optimize::PlannerStrategy", "path": "PlannerStrategy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 39], "end": [72, 48], "filename": "crates/core/src/operations/optimize.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/core/src/operations/optimize.rs:72`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9eb5299d6a7ec6e603591f9d"></a>
## fmt

`function` · `deltalake_core::operations::optimize::PlannerStrategy::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L72).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::optimize::PlannerStrategy", "path": "PlannerStrategy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 10], "end": [72, 15], "filename": "crates/core/src/operations/optimize.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/operations/optimize.rs:72`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-850009eefd264197699fcfa3"></a>
## serialize

`function` · `deltalake_core::operations::optimize::PlannerStrategy::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L72).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::optimize::PlannerStrategy", "path": "PlannerStrategy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 54], "end": [72, 63], "filename": "crates/core/src/operations/optimize.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/operations/optimize.rs:72`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
