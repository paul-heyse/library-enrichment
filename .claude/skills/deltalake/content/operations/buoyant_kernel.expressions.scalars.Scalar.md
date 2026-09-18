# `buoyant_kernel::expressions::scalars::Scalar`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.expressions.scalars.Scalar.json).

<a id="op-8dd45baeed3da91aa357441a"></a>
## Scalar

`enum` · `buoyant_kernel::expressions::scalars::Scalar` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum Scalar
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L223).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:223`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A single value, which can be null. Used for representing literal values
in [Expressions][crate::expressions::Expression](../operations/buoyant_kernel.expressions.Expression.md#op-b6e8a7405239f2ae57fc5e51).

NOTE: `PartialEq` uses physical (structural) comparison semantics.
For SQL NULL semantics, use [`Scalar::logical_eq`](../operations/buoyant_kernel.expressions.scalars.Scalar.md#op-e14167ccfb30a29f907745ca) or [`Scalar::logical_partial_cmp`](../operations/buoyant_kernel.expressions.scalars.Scalar.md#op-f22b5cfb1ff29786f30f4045).

<a id="op-638fce115db1354576f0a8b9"></a>
## Array

`variant` · `buoyant_kernel::expressions::scalars::Scalar::Array` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Array
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L261).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:261`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Array Value

<a id="op-1d1949dc334a5d8a3dfd54cc"></a>
## Binary

`variant` · `buoyant_kernel::expressions::scalars::Scalar::Binary` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Binary
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L253).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:253`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Binary data

<a id="op-87de76635a5dab3ef04a8162"></a>
## Boolean

`variant` · `buoyant_kernel::expressions::scalars::Scalar::Boolean` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Boolean
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L239).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:239`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

true or false value

<a id="op-f5b07a7e1eb8641a488c1ac5"></a>
## Byte

`variant` · `buoyant_kernel::expressions::scalars::Scalar::Byte` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Byte
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L231).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:231`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

8bit integer

<a id="op-2ad8e2728a19c2e4bf875688"></a>
## Date

`variant` · `buoyant_kernel::expressions::scalars::Scalar::Date` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Date
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L251).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:251`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Date stored as a signed 32bit int days since UNIX epoch 1970-01-01

<a id="op-56585c46aaf6f9526915428f"></a>
## Decimal

`variant` · `buoyant_kernel::expressions::scalars::Scalar::Decimal` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Decimal
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L255).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:255`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Decimal value with a given precision and scale.

<a id="op-e2905b8dee61e499fd34d258"></a>
## Double

`variant` · `buoyant_kernel::expressions::scalars::Scalar::Double` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Double
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L235).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:235`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

64bit floating point

<a id="op-279537db93b6e86ef870f8d0"></a>
## Error

`assoc_type` · `buoyant_kernel::expressions::scalars::Scalar::Error` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Error = Error
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L689).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::schema::derive_macro_utils::ToDataType", "path": "ToDataType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::schema::derive_macro_utils::ToDataType", "path": "ToDataType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [684, 1], "end": [699, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "std::collections::hash::map::HashMap", "path": "HashMap"}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:689`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c2f6b8a0b90bee6528f2e49"></a>
## Error

`assoc_type` · `buoyant_kernel::expressions::scalars::Scalar::Error` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Error = Error
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L675).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::schema::derive_macro_utils::ToDataType", "path": "ToDataType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::schema::derive_macro_utils::ToDataType", "path": "ToDataType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [670, 1], "end": [682, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "V"}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "std::collections::hash::map::HashMap", "path": "HashMap"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:675`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-56dd4ec9257f6ea664d7d98f"></a>
## Error

`assoc_type` · `buoyant_kernel::expressions::scalars::Scalar::Error` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Error = Error
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L646).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::schema::derive_macro_utils::ToDataType", "path": "ToDataType"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [642, 1], "end": [654, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:646`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b560bb1baeff34a40ab2e82d"></a>
## Error

`assoc_type` · `buoyant_kernel::expressions::scalars::Scalar::Error` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Error = Error
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L661).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::schema::derive_macro_utils::ToDataType", "path": "ToDataType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::schema::derive_macro_utils::ToDataType", "path": "ToDataType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [656, 1], "end": [668, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "std::collections::hash::map::HashMap", "path": "HashMap"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:661`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca5492b82728166219202ff3"></a>
## Error

`assoc_type` · `buoyant_kernel::expressions::scalars::Scalar::Error` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Error = Error
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L620).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::schema::derive_macro_utils::ToDataType", "path": "ToDataType"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [616, 1], "end": [627, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:620`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de43e58135c898b52b535f81"></a>
## Error

`assoc_type` · `buoyant_kernel::expressions::scalars::Scalar::Error` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Error = Error
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L216).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "crate::expressions::Scalar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [215, 1], "end": [230, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::Format", "path": "Format"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:216`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd113c1e04c2b7932ef0508b"></a>
## Error

`assoc_type` · `buoyant_kernel::expressions::scalars::Scalar::Error` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Error = Error
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L633).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::schema::derive_macro_utils::ToDataType", "path": "ToDataType"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [629, 1], "end": [640, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:633`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-652ee8c54ddf4dff0c286ba0"></a>
## Float

`variant` · `buoyant_kernel::expressions::scalars::Scalar::Float` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Float
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L233).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:233`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

32bit floating point

<a id="op-61689b9d73919448f240d7f1"></a>
## Integer

`variant` · `buoyant_kernel::expressions::scalars::Scalar::Integer` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Integer
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L225).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:225`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

32bit integer

<a id="op-42b2966903b151ec9814148f"></a>
## Long

`variant` · `buoyant_kernel::expressions::scalars::Scalar::Long` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Long
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L227).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:227`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

64bit integer

<a id="op-f24311d8c06b159fcff49217"></a>
## Map

`variant` · `buoyant_kernel::expressions::scalars::Scalar::Map` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Map
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L263).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:263`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Map Value

<a id="op-73fc9142a0e40177a4e3a372"></a>
## Null

`variant` · `buoyant_kernel::expressions::scalars::Scalar::Null` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Null
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L257).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:257`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Null value with a given data type.

<a id="op-1bdb8246fd91f788f7047bcd"></a>
## Short

`variant` · `buoyant_kernel::expressions::scalars::Scalar::Short` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Short
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L229).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:229`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

16bit integer

<a id="op-038c5a9925dc446f9b009e46"></a>
## String

`variant` · `buoyant_kernel::expressions::scalars::Scalar::String` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
String
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L237).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:237`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

utf-8 encoded string.

<a id="op-a0946873b4bed17b2f091d67"></a>
## Struct

`variant` · `buoyant_kernel::expressions::scalars::Scalar::Struct` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Struct
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L259).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:259`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Struct value

<a id="op-33f262509879ba96804765ba"></a>
## Timestamp

`variant` · `buoyant_kernel::expressions::scalars::Scalar::Timestamp` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Timestamp
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L241).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:241`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Microsecond precision timestamp, adjusted to UTC.

<a id="op-4c7c0a9756378a10d0902729"></a>
## TimestampNanos

`variant` · `buoyant_kernel::expressions::scalars::Scalar::TimestampNanos` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
TimestampNanos
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L246).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:246`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Nanosecond precision timestamp, adjusted to UTC.

<a id="op-494afe2b482064348ba9053e"></a>
## TimestampNanosNtz

`variant` · `buoyant_kernel::expressions::scalars::Scalar::TimestampNanosNtz` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
TimestampNanosNtz
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L249).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:249`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Nanosecond precision timestamp, with no timezone.

<a id="op-0d7929a424a8e25d51721ff0"></a>
## TimestampNtz

`variant` · `buoyant_kernel::expressions::scalars::Scalar::TimestampNtz` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
TimestampNtz
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L243).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:243`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Microsecond precision timestamp, with no timezone.

<a id="op-9dfc220f485b2dc81c673843"></a>
## clone

`function` · `buoyant_kernel::expressions::scalars::Scalar::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> Scalar
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L222).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [222, 17], "end": [222, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:222`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d0463470f79e294b0c41f86"></a>
## data_type

`function` · `buoyant_kernel::expressions::scalars::Scalar::data_type` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn data_type(&self) -> DataType
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L267).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [266, 1], "end": [377, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:267`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e474dea21d979e7b81c05f1"></a>
## decimal

`function` · `buoyant_kernel::expressions::scalars::Scalar::decimal` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn decimal(bits: impl Into<i128>, precision: u8, scale: u8) -> DeltaResult<Self>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L310).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [266, 1], "end": [377, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:310`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Constructs a Decimal value from raw parts

<a id="op-1032a2bbf253a695aad0f98a"></a>
## deserialize

`function` · `buoyant_kernel::expressions::scalars::Scalar::deserialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L222).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [222, 35], "end": [222, 46], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:222`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e0f27cfaee3a6ba2ff2ec8ca"></a>
## eq

`function` · `buoyant_kernel::expressions::scalars::Scalar::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &Scalar) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L222).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [222, 48], "end": [222, 57], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:222`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a7193ede72e1808571817db"></a>
## fmt

`function` · `buoyant_kernel::expressions::scalars::Scalar::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L380).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [379, 1], "end": [453, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:380`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-730baf6d1b090978753c4045"></a>
## fmt

`function` · `buoyant_kernel::expressions::scalars::Scalar::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L222).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [222, 10], "end": [222, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:222`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b51021356eb362a351f81ec"></a>
## from

`function` · `buoyant_kernel::expressions::scalars::Scalar::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(i: i32) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L551).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [550, 1], "end": [554, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i32"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:551`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1840c92b590fb8775f0e1ca0"></a>
## from

`function` · `buoyant_kernel::expressions::scalars::Scalar::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(b: &[u8]) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L605).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [604, 1], "end": [608, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"slice": {"primitive": "u8"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:605`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e36ff827010679dc4347517"></a>
## from

`function` · `buoyant_kernel::expressions::scalars::Scalar::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(t: &T) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L599).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Copy", "path": "Copy"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [598, 1], "end": [602, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"generic": "T"}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:599`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-225a299ba6ea20d23749cd70"></a>
## from

`function` · `buoyant_kernel::expressions::scalars::Scalar::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(t: Option<T>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L703).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::schema::derive_macro_utils::ToDataType", "path": "ToDataType"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [702, 1], "end": [709, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:703`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25c3e75d493bd3244eb1667d"></a>
## from

`function` · `buoyant_kernel::expressions::scalars::Scalar::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(b: bytes::Bytes) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L611).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [610, 1], "end": [614, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "bytes::bytes::Bytes", "path": "Bytes"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:611`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3724968e007f90f675afb080"></a>
## from

`function` · `buoyant_kernel::expressions::scalars::Scalar::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(value: String) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L593).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [592, 1], "end": [596, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:593`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-414d10652fdf8cbd44e76dc6"></a>
## from

`function` · `buoyant_kernel::expressions::scalars::Scalar::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(feature: TableFeature) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_features/mod.rs#L776).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "crate::expressions::Scalar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [775, 1], "end": [779, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_features/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_features::TableFeature", "path": "TableFeature"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_features/mod.rs:776`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4eb35155e6f98f9e62271ede"></a>
## from

`function` · `buoyant_kernel::expressions::scalars::Scalar::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(i: f32) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L563).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [562, 1], "end": [566, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "f32"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:563`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-532a1561b2885ae43c25bd70"></a>
## from

`function` · `buoyant_kernel::expressions::scalars::Scalar::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(b: bool) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L575).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [574, 1], "end": [578, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "bool"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:575`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-85cd9e9425b13e2a71fbab9f"></a>
## from

`function` · `buoyant_kernel::expressions::scalars::Scalar::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(i: f64) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L569).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [568, 1], "end": [572, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "f64"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:569`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-866ef070eaf375de9b5ee15c"></a>
## from

`function` · `buoyant_kernel::expressions::scalars::Scalar::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(s: &str) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L587).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [586, 1], "end": [590, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:587`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b82d2c815e2bb9000264e013"></a>
## from

`function` · `buoyant_kernel::expressions::scalars::Scalar::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(i: i16) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L545).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [544, 1], "end": [548, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i16"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:545`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9287e2ff769eb5ac1135b5b"></a>
## from

`function` · `buoyant_kernel::expressions::scalars::Scalar::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(i: i8) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L539).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [538, 1], "end": [542, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i8"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:539`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c60a879bd25a9dbbf2eabf50"></a>
## from

`function` · `buoyant_kernel::expressions::scalars::Scalar::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(i: i64) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L557).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [556, 1], "end": [560, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "i64"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:557`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e10025ca5f2dafd58987f5f5"></a>
## from

`function` · `buoyant_kernel::expressions::scalars::Scalar::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(map_data: MapData) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L718).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [717, 1], "end": [721, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::MapData", "path": "MapData"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:718`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1aab861c6e1892bd1a7f0ea"></a>
## from

`function` · `buoyant_kernel::expressions::scalars::Scalar::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(array_data: ArrayData) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L712).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [711, 1], "end": [715, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::ArrayData", "path": "ArrayData"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:712`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f05193f1a7b38712b566ef60"></a>
## from

`function` · `buoyant_kernel::expressions::scalars::Scalar::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from(d: DecimalData) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L581).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [580, 1], "end": [584, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::DecimalData", "path": "DecimalData"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:581`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-557accc77a3cad1b2ce62530"></a>
## is_null

`function` · `buoyant_kernel::expressions::scalars::Scalar::is_null` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn is_null(&self) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L294).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [266, 1], "end": [377, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:294`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns true if this scalar is null.

<a id="op-e14167ccfb30a29f907745ca"></a>
## logical_eq

`function` · `buoyant_kernel::expressions::scalars::Scalar::logical_eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn logical_eq(&self, other: &Self) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L467).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [455, 1], "end": [536, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:467`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Logical (SQL semantics) equality comparison of two scalars.

Returns `None` if the scalars cannot be compared (different types, NULL values, or
unsupported types like Struct/Array/Map).

Logical (SQL semantics) equality comparison of two scalars.

Returns `true` if the scalars are logically equal, `false` otherwise.

NOTE: This implements SQL NULL semantics where NULL is incomparable to everything,
including itself, so `NULL != NULL` (returns `false`).

<a id="op-f22b5cfb1ff29786f30f4045"></a>
## logical_partial_cmp

`function` · `buoyant_kernel::expressions::scalars::Scalar::logical_partial_cmp` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn logical_partial_cmp(&self, other: &Self) -> Option<Ordering>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L488).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [455, 1], "end": [536, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:488`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Logical (SQL semantics) comparison of two scalars.

Returns `None` if the scalars are incomparable (different types, NULL values, or
unsupported types like Struct/Array/Map).

NOTE: This implements SQL NULL semantics where NULL is incomparable to everything,
including itself.

<a id="op-3034d817974063b9c7307fab"></a>
## null

`function` · `buoyant_kernel::expressions::scalars::Scalar::null` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn null(data_type: impl Into<DataType>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L305).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [266, 1], "end": [377, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:305`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Constructs a null `Scalar` of the given type. Accepts anything convertible into a
[`DataType`](../operations/buoyant_kernel.schema.DataType.md#op-ae66d4bfcb2bbb0ad1521faa), so container types like [`StructType`], [`ArrayType`], and [`MapType`]
can be passed directly without an explicit `DataType::from(...)` wrapper.

[`StructType`]: crate::schema::StructType
[`ArrayType`]: crate::schema::ArrayType
[`MapType`]: crate::schema::MapType

<a id="op-9c8ce08c84c4a953cd6886f4"></a>
## physical_eq

`function` · `buoyant_kernel::expressions::scalars::Scalar::physical_eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn physical_eq(&self, other: &Self) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L477).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [455, 1], "end": [536, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:477`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Physical (structural) equality comparison of two scalars.

Returns `true` if the scalars are structurally identical, `false` otherwise.

Unlike logical comparison, this treats `Null(dt1) == Null(dt2)` as `true` when `dt1 == dt2`.
This is used for query plan comparison, not SQL evaluation.

<a id="op-ee7f85ee02bd4af38e4c0bad"></a>
## serialize

`function` · `buoyant_kernel::expressions::scalars::Scalar::serialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L222).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [222, 24], "end": [222, 33], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:222`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2bb39d88d2656464ad2d1a1e"></a>
## to_array

`function` · `buoyant_kernel::expressions::scalars::Scalar::to_array` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn to_array(&self, num_rows: usize) -> DeltaResult<ArrayRef>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_expression/mod.rs#L31).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "crate::expressions::Scalar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 1], "end": [233, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_expression/mod.rs:31`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Convert scalar to arrow array.

<a id="op-0d26acefbc1612950ce5751e"></a>
## try_add

`function` · `buoyant_kernel::expressions::scalars::Scalar::try_add` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_add(&self, other: &Scalar) -> Option<Scalar>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L327).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [266, 1], "end": [377, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:327`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Attempts to add two scalars, returning None if they were incompatible.

<a id="op-f47a48b64b48331fd21a789d"></a>
## try_div

`function` · `buoyant_kernel::expressions::scalars::Scalar::try_div` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_div(&self, other: &Scalar) -> Option<Scalar>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L366).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [266, 1], "end": [377, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:366`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Attempts to divide two scalars, returning None if they were incompatible.

<a id="op-467709fbc825ffdeaeba1b5b"></a>
## try_from

`function` · `buoyant_kernel::expressions::scalars::Scalar::try_from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_from(vec: Vec<Option<T>>) -> Result<Self, Self::Error>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L635).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::schema::derive_macro_utils::ToDataType", "path": "ToDataType"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [629, 1], "end": [640, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:635`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55ab228c80dfa0ff38474575"></a>
## try_from

`function` · `buoyant_kernel::expressions::scalars::Scalar::try_from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_from(map: HashMap<K, V>) -> Result<Self, Self::Error>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L663).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::schema::derive_macro_utils::ToDataType", "path": "ToDataType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::schema::derive_macro_utils::ToDataType", "path": "ToDataType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [656, 1], "end": [668, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "std::collections::hash::map::HashMap", "path": "HashMap"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:663`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-671115793923d9e62461d257"></a>
## try_from

`function` · `buoyant_kernel::expressions::scalars::Scalar::try_from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_from(format: Format) -> DeltaResult<Self>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/actions/mod.rs#L218).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "crate::expressions::Scalar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [215, 1], "end": [230, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::actions::Format", "path": "Format"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/actions/mod.rs:218`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-707277405375672c3969ec55"></a>
## try_from

`function` · `buoyant_kernel::expressions::scalars::Scalar::try_from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_from(opt: Option<HashMap<K, V>>) -> Result<Self, Self::Error>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L691).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::schema::derive_macro_utils::ToDataType", "path": "ToDataType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::schema::derive_macro_utils::ToDataType", "path": "ToDataType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [684, 1], "end": [699, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"generic": "V"}}], "constraints": []}}, "id": "std::collections::hash::map::HashMap", "path": "HashMap"}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:691`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b61f5a69d48b557294ec414d"></a>
## try_from

`function` · `buoyant_kernel::expressions::scalars::Scalar::try_from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_from(vec: Vec<T>) -> Result<Self, Self::Error>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L622).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::schema::derive_macro_utils::ToDataType", "path": "ToDataType"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [616, 1], "end": [627, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:622`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c51967290ceadafc5a08a3bf"></a>
## try_from

`function` · `buoyant_kernel::expressions::scalars::Scalar::try_from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_from(opt: Option<Vec<T>>) -> Result<Self, Self::Error>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L648).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::schema::derive_macro_utils::ToDataType", "path": "ToDataType"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [642, 1], "end": [654, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:648`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d3599b70be8e0ad3a0005317"></a>
## try_from

`function` · `buoyant_kernel::expressions::scalars::Scalar::try_from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_from(map: HashMap<K, Option<V>>) -> Result<Self, Self::Error>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L677).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "K"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "V"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::schema::derive_macro_utils::ToDataType", "path": "ToDataType"}}}], "generic_params": [], "type": {"generic": "K"}}}, {"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "buoyant_kernel::schema::derive_macro_utils::ToDataType", "path": "ToDataType"}}}], "generic_params": [], "type": {"generic": "V"}}}]}, "is_negative": false, "span": {"begin": [670, 1], "end": [682, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "K"}}, {"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "V"}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "std::collections::hash::map::HashMap", "path": "HashMap"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:677`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-579ee5f38a324cef9c9dfed2"></a>
## try_mul

`function` · `buoyant_kernel::expressions::scalars::Scalar::try_mul` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_mul(&self, other: &Scalar) -> Option<Scalar>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L353).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [266, 1], "end": [377, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:353`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Attempts to multiply two scalars, returning None if they were incompatible.

<a id="op-1f5af0d5a9498582a524b22e"></a>
## try_sub

`function` · `buoyant_kernel::expressions::scalars::Scalar::try_sub` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_sub(&self, other: &Scalar) -> Option<Scalar>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/scalars.rs#L340).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::scalars::Scalar", "path": "Scalar"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [266, 1], "end": [377, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/scalars.rs:340`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Attempts to subtract two scalars, returning None if they were incompatible.
