# `buoyant_kernel::table_properties::deserialize::ParseIntervalError`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.table_properties.deserialize.ParseIntervalError.json).

<a id="op-73594b17599e8b713d36eb34"></a>
## ParseIntervalError

`enum` · `buoyant_kernel::table_properties::deserialize::ParseIntervalError` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum ParseIntervalError
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/deserialize.rs#L155).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/deserialize.rs:155`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c0b61b7119526f6e09a6b1c"></a>
## NegativeInterval

`variant` · `buoyant_kernel::table_properties::deserialize::ParseIntervalError::NegativeInterval` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
NegativeInterval
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/deserialize.rs#L164).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/deserialize.rs:164`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Negative intervals aren't supported

<a id="op-e6eb13e3be5fb6c7baab37e7"></a>
## NotAnInterval

`variant` · `buoyant_kernel::table_properties::deserialize::ParseIntervalError::NotAnInterval` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
NotAnInterval
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/deserialize.rs#L158).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/deserialize.rs:158`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The input string is not a valid interval

<a id="op-940632c05cb0d906a3a2bde5"></a>
## ParseIntError

`variant` · `buoyant_kernel::table_properties::deserialize::ParseIntervalError::ParseIntError` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
ParseIntError
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/deserialize.rs#L161).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/deserialize.rs:161`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Couldn't parse the input string as an integer

<a id="op-bf06b5a0c162ae39073222c1"></a>
## UnknownUnit

`variant` · `buoyant_kernel::table_properties::deserialize::ParseIntervalError::UnknownUnit` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
UnknownUnit
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/deserialize.rs#L170).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/deserialize.rs:170`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Unknown unit

<a id="op-fc04f3b522e32344a34dda82"></a>
## UnsupportedInterval

`variant` · `buoyant_kernel::table_properties::deserialize::ParseIntervalError::UnsupportedInterval` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
UnsupportedInterval
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/deserialize.rs#L167).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/deserialize.rs:167`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Unsupported interval

<a id="op-2985360f80f085c6a4451a8f"></a>
## fmt

`function` · `buoyant_kernel::table_properties::deserialize::ParseIntervalError::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/deserialize.rs#L154).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_properties::deserialize::ParseIntervalError", "path": "ParseIntervalError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 10], "end": [154, 26], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/deserialize.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/deserialize.rs:154`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c2f791538f1f9c2f5528a5e"></a>
## fmt

`function` · `buoyant_kernel::table_properties::deserialize::ParseIntervalError::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/deserialize.rs#L154).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_properties::deserialize::ParseIntervalError", "path": "ParseIntervalError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [154, 28], "end": [154, 33], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/deserialize.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/deserialize.rs:154`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
