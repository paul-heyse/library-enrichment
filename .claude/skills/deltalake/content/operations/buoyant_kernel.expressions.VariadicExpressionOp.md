# `buoyant_kernel::expressions::VariadicExpressionOp`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.expressions.VariadicExpressionOp.json).

<a id="op-12674a8740ae2bb3863f66a6"></a>
## VariadicExpressionOp

`enum` · `buoyant_kernel::expressions::VariadicExpressionOp` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum VariadicExpressionOp
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L104).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:104`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A variadic expression operator.

<a id="op-41ff2df0bd693d9adf29f055"></a>
## Array

`variant` · `buoyant_kernel::expressions::VariadicExpressionOp::Array` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Array
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L115).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:115`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Construct an Array by evaluating each input expression. For example, the expression
`Array(1, (1 + 2), col("my_int_col"))` evaluates to the array
`[1, 3, <my_int_col value>]` per row. All inputs must share the same element type.
Requires at least one element; the element type is inferred from the inputs.

For static array literals whose elements are all compile-time constants, use
[`Scalar::Array`](../operations/buoyant_kernel.expressions.scalars.Scalar.md#op-638fce115db1354576f0a8b9) instead. The difference is that `Array` is evaluated at runtime, while
`Scalar::Array` is evaluated at compile time.

<a id="op-4cd31a2fbda27af6c6fc06c7"></a>
## Coalesce

`variant` · `buoyant_kernel::expressions::VariadicExpressionOp::Coalesce` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Coalesce
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L106).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:106`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Collapse multiple values into one by taking the first non-null value

<a id="op-eb7fcc3306fa7011884b3fe2"></a>
## clone

`function` · `buoyant_kernel::expressions::VariadicExpressionOp::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> VariadicExpressionOp
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L103).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::VariadicExpressionOp", "path": "VariadicExpressionOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 17], "end": [103, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:103`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64215ba89217e28bca5ef239"></a>
## deserialize

`function` · `buoyant_kernel::expressions::VariadicExpressionOp::deserialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L103).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::VariadicExpressionOp", "path": "VariadicExpressionOp"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 62], "end": [103, 73], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:103`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-12dc7e3401c3e14798933abd"></a>
## eq

`function` · `buoyant_kernel::expressions::VariadicExpressionOp::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &VariadicExpressionOp) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L103).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::VariadicExpressionOp", "path": "VariadicExpressionOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 30], "end": [103, 39], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:103`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6be8b414f97d02f7fcb658f5"></a>
## fmt

`function` · `buoyant_kernel::expressions::VariadicExpressionOp::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L103).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::VariadicExpressionOp", "path": "VariadicExpressionOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 10], "end": [103, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:103`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f08aec1b15bbe80c00265826"></a>
## fmt

`function` · `buoyant_kernel::expressions::VariadicExpressionOp::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L942).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::VariadicExpressionOp", "path": "VariadicExpressionOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [941, 1], "end": [949, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:942`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-570f2bfd4c28e7adbd4bfa3b"></a>
## hash

`function` · `buoyant_kernel::expressions::VariadicExpressionOp::hash` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L103).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::VariadicExpressionOp", "path": "VariadicExpressionOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 45], "end": [103, 49], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:103`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cdfae41c57ce796b7a0e10c7"></a>
## serialize

`function` · `buoyant_kernel::expressions::VariadicExpressionOp::serialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L103).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::VariadicExpressionOp", "path": "VariadicExpressionOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 51], "end": [103, 60], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:103`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
