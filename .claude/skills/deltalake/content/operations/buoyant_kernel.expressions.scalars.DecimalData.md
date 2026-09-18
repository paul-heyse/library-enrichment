# `buoyant_kernel::expressions::scalars::DecimalData`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.expressions.scalars.DecimalData.json).

<a id="op-8650fc4239f52716a72d93b3"></a>
## DecimalData

`struct` · `buoyant_kernel::expressions::scalars::DecimalData` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DecimalData
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L15).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:15`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b640a1711429562d86146926"></a>
## bits

`function` · `buoyant_kernel::expressions::scalars::DecimalData::bits` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn bits(&self) -> i128
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L34).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::DecimalData", "path": "DecimalData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [20, 1], "end": [49, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:34`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ae55f1eb5577cf4f9f7653f"></a>
## clone

`function` · `buoyant_kernel::expressions::scalars::DecimalData::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> DecimalData
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L14).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::DecimalData", "path": "DecimalData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [14, 17], "end": [14, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:14`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-61121fcf20d3444866c668d5"></a>
## deserialize

`function` · `buoyant_kernel::expressions::scalars::DecimalData::deserialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L14).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::DecimalData", "path": "DecimalData"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [14, 46], "end": [14, 57], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:14`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f9522831fe00f656ee7ac43"></a>
## eq

`function` · `buoyant_kernel::expressions::scalars::DecimalData::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &DecimalData) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L14).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::DecimalData", "path": "DecimalData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [14, 24], "end": [14, 33], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:14`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-702b67d77ba39591da53d33b"></a>
## fmt

`function` · `buoyant_kernel::expressions::scalars::DecimalData::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L14).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::DecimalData", "path": "DecimalData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [14, 10], "end": [14, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:14`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca6d12e2ce2ce9a9dbc482f5"></a>
## precision

`function` · `buoyant_kernel::expressions::scalars::DecimalData::precision` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn precision(&self) -> u8
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L42).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::DecimalData", "path": "DecimalData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [20, 1], "end": [49, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:42`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4de352e69a90537f82286087"></a>
## scale

`function` · `buoyant_kernel::expressions::scalars::DecimalData::scale` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn scale(&self) -> u8
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L46).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::DecimalData", "path": "DecimalData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [20, 1], "end": [49, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:46`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8cca680dbdb1d4cbd5767eab"></a>
## serialize

`function` · `buoyant_kernel::expressions::scalars::DecimalData::serialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L14).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::DecimalData", "path": "DecimalData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [14, 35], "end": [14, 44], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:14`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-436dba7b520001b8929e41fb"></a>
## try_new

`function` · `buoyant_kernel::expressions::scalars::DecimalData::try_new` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_new(bits: impl Into<i128>, ty: DecimalType) -> DeltaResult<Self>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L21).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::DecimalData", "path": "DecimalData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [20, 1], "end": [49, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:21`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7fc64123a9345f888b522a90"></a>
## ty

`function` · `buoyant_kernel::expressions::scalars::DecimalData::ty` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn ty(&self) -> &DecimalType
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L38).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::DecimalData", "path": "DecimalData"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [20, 1], "end": [49, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:38`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-786678fd1ee7ee43bf936783"></a>
## bits

`struct_field` · `buoyant_kernel::expressions::scalars::DecimalData::bits` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
bits: i128
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L16).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:16`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80658f3eeb17d28f7b7314ef"></a>
## ty

`struct_field` · `buoyant_kernel::expressions::scalars::DecimalData::ty` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
ty: schema::DecimalType
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L17).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:17`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
