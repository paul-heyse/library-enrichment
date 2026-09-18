# `deltalake_core::kernel::schema::partitions::FilterOp`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.schema.partitions.FilterOp.json).

<a id="op-3c1406cb8c72e4b9de4e3a6d"></a>
## FilterOp

`enum` · `deltalake_core::kernel::schema::partitions::FilterOp` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum FilterOp
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L69).

Source: `crates/core/src/kernel/schema/partitions.rs:69`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The comparison operator of a `(column, op, value)` filter literal.

<a id="op-c4d7ca50084561231e2bf0b0"></a>
## Eq

`variant` · `deltalake_core::kernel::schema::partitions::FilterOp::Eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Eq
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L71).

Source: `crates/core/src/kernel/schema/partitions.rs:71`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

`=`

<a id="op-0e7024f5c1c8195b0680c6ec"></a>
## Err

`assoc_type` · `deltalake_core::kernel::schema::partitions::FilterOp::Err` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Err = DeltaTableError
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L117).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::schema::partitions::FilterOp", "path": "FilterOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [138, 2], "filename": "crates/core/src/kernel/schema/partitions.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `crates/core/src/kernel/schema/partitions.rs:117`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5754678cc86f889ee7da27d6"></a>
## Ge

`variant` · `deltalake_core::kernel::schema::partitions::FilterOp::Ge` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Ge
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L81).

Source: `crates/core/src/kernel/schema/partitions.rs:81`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

`>=`

<a id="op-d689646d19dde3108e0c40fc"></a>
## Gt

`variant` · `deltalake_core::kernel::schema::partitions::FilterOp::Gt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Gt
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L79).

Source: `crates/core/src/kernel/schema/partitions.rs:79`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

`>`

<a id="op-413e1d45d4416cb445a0724d"></a>
## In

`variant` · `deltalake_core::kernel::schema::partitions::FilterOp::In` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
In
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L83).

Source: `crates/core/src/kernel/schema/partitions.rs:83`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

`in`

<a id="op-1e2af2fbf7bea526682fc1da"></a>
## Le

`variant` · `deltalake_core::kernel::schema::partitions::FilterOp::Le` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Le
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L77).

Source: `crates/core/src/kernel/schema/partitions.rs:77`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

`<=`

<a id="op-51d695a6ff9d6f6939eba4af"></a>
## Lt

`variant` · `deltalake_core::kernel::schema::partitions::FilterOp::Lt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Lt
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L75).

Source: `crates/core/src/kernel/schema/partitions.rs:75`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

`<`

<a id="op-f890cccd66c525c46dc87971"></a>
## Ne

`variant` · `deltalake_core::kernel::schema::partitions::FilterOp::Ne` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Ne
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L73).

Source: `crates/core/src/kernel/schema/partitions.rs:73`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

`!=`

<a id="op-2a85a78d5226005e7129fa6f"></a>
## NotIn

`variant` · `deltalake_core::kernel::schema::partitions::FilterOp::NotIn` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
NotIn
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L85).

Source: `crates/core/src/kernel/schema/partitions.rs:85`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

`not in`

<a id="op-09f7533082de6a5c71ed72c0"></a>
## as_str

`function` · `deltalake_core::kernel::schema::partitions::FilterOp::as_str` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn as_str(self) -> &'static str
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L90).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::schema::partitions::FilterOp", "path": "FilterOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [108, 2], "filename": "crates/core/src/kernel/schema/partitions.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/schema/partitions.rs:90`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The operator string accepted by [`FromStr`], e.g. `"="` or `"not in"`.

Unresolved upstream links (retained, not inferred): ``FromStr``.

<a id="op-3238c5886adef35d8a1edfd4"></a>
## clone

`function` · `deltalake_core::kernel::schema::partitions::FilterOp::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> FilterOp
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L68).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::schema::partitions::FilterOp", "path": "FilterOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 10], "end": [68, 15], "filename": "crates/core/src/kernel/schema/partitions.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/kernel/schema/partitions.rs:68`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-08b8e5b35c94a8962d97dac9"></a>
## eq

`function` · `deltalake_core::kernel::schema::partitions::FilterOp::eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &FilterOp) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L68).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::schema::partitions::FilterOp", "path": "FilterOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 30], "end": [68, 39], "filename": "crates/core/src/kernel/schema/partitions.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/core/src/kernel/schema/partitions.rs:68`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02e846e78da0bb13a3b9d2e6"></a>
## fmt

`function` · `deltalake_core::kernel::schema::partitions::FilterOp::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L111).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::schema::partitions::FilterOp", "path": "FilterOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 1], "end": [114, 2], "filename": "crates/core/src/kernel/schema/partitions.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `crates/core/src/kernel/schema/partitions.rs:111`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1cadca7afc81dd80b777e3d"></a>
## fmt

`function` · `deltalake_core::kernel::schema::partitions::FilterOp::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L68).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::schema::partitions::FilterOp", "path": "FilterOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 23], "end": [68, 28], "filename": "crates/core/src/kernel/schema/partitions.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/kernel/schema/partitions.rs:68`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-702a8cf3b90eb916d3dc604f"></a>
## from_str

`function` · `deltalake_core::kernel::schema::partitions::FilterOp::from_str` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/partitions.rs#L119).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::schema::partitions::FilterOp", "path": "FilterOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [116, 1], "end": [138, 2], "filename": "crates/core/src/kernel/schema/partitions.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `crates/core/src/kernel/schema/partitions.rs:119`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
