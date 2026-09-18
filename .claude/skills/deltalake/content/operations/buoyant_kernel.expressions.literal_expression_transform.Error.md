# `buoyant_kernel::expressions::literal_expression_transform::Error`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.expressions.literal_expression_transform.Error.json).

<a id="op-8c489b287c9e6eae765a3dd3"></a>
## Error

`enum` · `buoyant_kernel::expressions::literal_expression_transform::Error` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum Error
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/literal_expression_transform.rs#L23).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/literal_expression_transform.rs:23`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Any error for [`LiteralExpressionTransform`]

<a id="op-e59d3b96f590d0bdcb02c607"></a>
## EmptyStack

`variant` · `buoyant_kernel::expressions::literal_expression_transform::Error::EmptyStack` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
EmptyStack
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/literal_expression_transform.rs#L38).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/literal_expression_transform.rs:38`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Empty expression stack after performing the transform

<a id="op-7bd04419ebd8e329ce3e9896"></a>
## ExcessScalars

`variant` · `buoyant_kernel::expressions::literal_expression_transform::Error::ExcessScalars` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
ExcessScalars
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/literal_expression_transform.rs#L30).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/literal_expression_transform.rs:30`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Insufficient number of scalars (too many) to create a single-row expression

<a id="op-bf36e4bf4a40c8b9e04ca234"></a>
## InsufficientScalars

`variant` · `buoyant_kernel::expressions::literal_expression_transform::Error::InsufficientScalars` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
InsufficientScalars
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/literal_expression_transform.rs#L34).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/literal_expression_transform.rs:34`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Insufficient number of scalars (too few) to create a single-row expression

<a id="op-46354869d8417b9150090dd2"></a>
## Schema

`variant` · `buoyant_kernel::expressions::literal_expression_transform::Error::Schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Schema
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/literal_expression_transform.rs#L26).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/literal_expression_transform.rs:26`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Schema mismatch error

<a id="op-621b3a31d6fcb8926f1ea5cf"></a>
## Unsupported

`variant` · `buoyant_kernel::expressions::literal_expression_transform::Error::Unsupported` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Unsupported
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/literal_expression_transform.rs#L42).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/literal_expression_transform.rs:42`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Unsupported operation

<a id="op-56d954ce3a11d9d741e7ecfa"></a>
## fmt

`function` · `buoyant_kernel::expressions::literal_expression_transform::Error::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/literal_expression_transform.rs#L22).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::literal_expression_transform::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 28], "end": [22, 33], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/literal_expression_transform.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/literal_expression_transform.rs:22`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d05ca0db7907cc1290b35767"></a>
## fmt

`function` · `buoyant_kernel::expressions::literal_expression_transform::Error::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/literal_expression_transform.rs#L22).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::literal_expression_transform::Error", "path": "Error"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 10], "end": [22, 26], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/literal_expression_transform.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/literal_expression_transform.rs:22`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
