# `buoyant_kernel::expressions::column_names::ColumnName`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.expressions.column_names.ColumnName.json).

<a id="op-9a9657c136296c6d9c579ddd"></a>
## ColumnName

`struct` · `buoyant_kernel::expressions::column_names::ColumnName` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ColumnName
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/column_names.rs#L12).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs:12`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A (possibly nested) column name.

<a id="op-0ffcac849c8c07772cde5f65"></a>
## Err

`assoc_type` · `buoyant_kernel::expressions::column_names::ColumnName::Err` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Err = Error
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/column_names.rs#L276).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::column_names::ColumnName", "path": "ColumnName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [275, 1], "end": [284, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs:276`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d367c6136768836df52fcaab"></a>
## IntoIter

`assoc_type` · `buoyant_kernel::expressions::column_names::ColumnName::IntoIter` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type IntoIter = IntoIter<<ColumnName as IntoIterator>::Item>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/column_names.rs#L143).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::column_names::ColumnName", "path": "ColumnName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [148, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs:143`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb4ab35ec77098985a4e1268"></a>
## Item

`assoc_type` · `buoyant_kernel::expressions::column_names::ColumnName::Item` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Item = String
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/column_names.rs#L142).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::column_names::ColumnName", "path": "ColumnName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [148, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs:142`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48463365a04bd488e8741913"></a>
## Target

`assoc_type` · `buoyant_kernel::expressions::column_names::ColumnName::Target` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Target = [String]
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/column_names.rs#L151).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::column_names::ColumnName", "path": "ColumnName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [156, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs:151`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41a4e33f50cc741ebced5d81"></a>
## borrow

`function` · `buoyant_kernel::expressions::column_names::ColumnName::borrow` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn borrow(&self) -> &[String]
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/column_names.rs#L160).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::column_names::ColumnName", "path": "ColumnName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [159, 1], "end": [163, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"slice": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}}], "constraints": []}}, "id": "core::borrow::Borrow", "path": "Borrow"}, "trait_path": "core::borrow::Borrow"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs:160`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-031497f69b39cc512f05ef15"></a>
## clone

`function` · `buoyant_kernel::expressions::column_names::ColumnName::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> ColumnName
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/column_names.rs#L11).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::column_names::ColumnName", "path": "ColumnName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11, 17], "end": [11, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs:11`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71b1a03522d2faaad9057569"></a>
## cmp

`function` · `buoyant_kernel::expressions::column_names::ColumnName::cmp` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn cmp(&self, other: &ColumnName) -> cmp::Ordering
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/column_names.rs#L11).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::column_names::ColumnName", "path": "ColumnName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11, 60], "end": [11, 63], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs:11`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b9dd29116ff3df6b162d8d8"></a>
## default

`function` · `buoyant_kernel::expressions::column_names::ColumnName::default` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> ColumnName
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/column_names.rs#L11).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::column_names::ColumnName", "path": "ColumnName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11, 24], "end": [11, 31], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs:11`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bccacb20320dcf9ea2cecfae"></a>
## deref

`function` · `buoyant_kernel::expressions::column_names::ColumnName::deref` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deref(&self) -> &[String]
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/column_names.rs#L153).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::column_names::ColumnName", "path": "ColumnName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [150, 1], "end": [156, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs"}, "trait": {"args": null, "id": "core::ops::deref::Deref", "path": "Deref"}, "trait_path": "core::ops::deref::Deref"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs:153`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-60f297c800527de9a4fda9ec"></a>
## deserialize

`function` · `buoyant_kernel::expressions::column_names::ColumnName::deserialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/column_names.rs#L11).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::column_names::ColumnName", "path": "ColumnName"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11, 76], "end": [11, 87], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs:11`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b04f42e01bc1b948102a3b8"></a>
## eq

`function` · `buoyant_kernel::expressions::column_names::ColumnName::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &ColumnName) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/column_names.rs#L11).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::column_names::ColumnName", "path": "ColumnName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11, 33], "end": [11, 42], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs:11`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-551991f0f5388e00ed043efd"></a>
## fmt

`function` · `buoyant_kernel::expressions::column_names::ColumnName::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/column_names.rs#L11).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::column_names::ColumnName", "path": "ColumnName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11, 10], "end": [11, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs:11`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0504876278cbf121da2317f"></a>
## fmt

`function` · `buoyant_kernel::expressions::column_names::ColumnName::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/column_names.rs#L215).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::column_names::ColumnName", "path": "ColumnName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [214, 1], "end": [243, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs:215`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a7df7e385f91a38015632c9"></a>
## from_iter

`function` · `buoyant_kernel::expressions::column_names::ColumnName::from_iter` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/column_names.rs#L127).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::column_names::ColumnName", "path": "ColumnName"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::convert::Into", "path": "Into"}}}], "default": null, "is_synthetic": false}}, "name": "A"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [126, 1], "end": [131, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "A"}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs:127`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45703eab1546bffb37b72037"></a>
## from_iter

`function` · `buoyant_kernel::expressions::column_names::ColumnName::from_iter` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_iter<T: IntoIterator<Item = ColumnName>>(iter: T) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/column_names.rs#L135).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::column_names::ColumnName", "path": "ColumnName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [139, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::column_names::ColumnName", "path": "ColumnName"}}}], "constraints": []}}, "id": "core::iter::traits::collect::FromIterator", "path": "FromIterator"}, "trait_path": "core::iter::traits::collect::FromIterator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs:135`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99abe988b085c3d9c58a11ff"></a>
## from_naive_str_split

`function` · `buoyant_kernel::expressions::column_names::ColumnName::from_naive_str_split` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_naive_str_split(name: impl AsRef<str>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/column_names.rs#L38).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::column_names::ColumnName", "path": "ColumnName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [16, 1], "end": [122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs:38`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Naively splits a string at dots to create a column name.

This method is _NOT_ recommended for production use, as it does not attempt to interpret
special characters in field names. For example, many systems would interpret the field name
`"a.b" . c ` as equivalent to `ColumnName::new(["\"a.b\"", "c"])` (two fields, whitespace
padding ignored), but this method would return three fields, including whitespace:

```
# use buoyant_kernel as delta_kernel;
# use delta_kernel::expressions::ColumnName;
assert_eq!(
    ColumnName::from_naive_str_split(" \"a.b\" . c "),
    ColumnName::new([" \"a", "b\" ", " c "])
);
```

<a id="op-b18252b4de46378bc0cd5555"></a>
## from_str

`function` · `buoyant_kernel::expressions::column_names::ColumnName::from_str` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_str(s: &str) -> DeltaResult<Self>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/column_names.rs#L278).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::column_names::ColumnName", "path": "ColumnName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [275, 1], "end": [284, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs:278`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4bc4c7b6d67dccfdc1d93ec"></a>
## hash

`function` · `buoyant_kernel::expressions::column_names::ColumnName::hash` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn hash<H: Hasher>(&self, hasher: &mut H)
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/column_names.rs#L178).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::column_names::ColumnName", "path": "ColumnName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [177, 1], "end": [181, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs:178`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc883eef66095d5338221545"></a>
## into_inner

`function` · `buoyant_kernel::expressions::column_names::ColumnName::into_inner` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_inner(self) -> Vec<String>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/column_names.rs#L98).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::column_names::ColumnName", "path": "ColumnName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [16, 1], "end": [122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs:98`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Consumes this column name and returns the path of field names.

<a id="op-1b3f52d34ca7c607e3b8349f"></a>
## into_iter

`function` · `buoyant_kernel::expressions::column_names::ColumnName::into_iter` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_iter(self) -> Self::IntoIter
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/column_names.rs#L145).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::column_names::ColumnName", "path": "ColumnName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [148, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs:145`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59a52660ee6859af97e974ab"></a>
## join

`function` · `buoyant_kernel::expressions::column_names::ColumnName::join` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn join(&self, right: &ColumnName) -> ColumnName
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/column_names.rs#L88).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::column_names::ColumnName", "path": "ColumnName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [16, 1], "end": [122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs:88`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Joins this column with another, concatenating their fields into a single nested column path.

NOTE: This is a convenience method that copies two arguments without consuming them. If more
arguments are needed, or if performance is a concern, it is recommended to use
[`FromIterator for ColumnName`](#impl-FromIterator<ColumnName>-for-ColumnName) instead:

```
# use buoyant_kernel as delta_kernel;
# use delta_kernel::expressions::ColumnName;
let x = ColumnName::new(["a", "b"]);
let y = ColumnName::new(["c", "d"]);
let joined: ColumnName = [x, y].into_iter().collect();
assert_eq!(joined, ColumnName::new(["a", "b", "c", "d"]));
```

<a id="op-d36e45e5a7503d1d58460476"></a>
## new

`function` · `buoyant_kernel::expressions::column_names::ColumnName::new` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(iter: impl CollectInto<Self>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/column_names.rs#L19).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::column_names::ColumnName", "path": "ColumnName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [16, 1], "end": [122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs:19`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a new column name from input satisfying `FromIterator for ColumnName`. The provided
field names are concatenated into a single path.

<a id="op-a6136e454cba857f22a83174"></a>
## parent

`function` · `buoyant_kernel::expressions::column_names::ColumnName::parent` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn parent(&self) -> Option<ColumnName>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/column_names.rs#L115).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::column_names::ColumnName", "path": "ColumnName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [16, 1], "end": [122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs:115`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the parent of this column name, or `None` if this is a top-level column.

# Examples

```
# use buoyant_kernel as delta_kernel;
# use delta_kernel::expressions::ColumnName;
let path = ColumnName::new(["user", "address", "street"]);
assert_eq!(path.parent(), Some(ColumnName::new(["user", "address"])));

let path = ColumnName::new(["user"]);
assert_eq!(path.parent(), None);
```

<a id="op-a0df435b7f239c63dff2a2e2"></a>
## parse_column_name_list

`function` · `buoyant_kernel::expressions::column_names::ColumnName::parse_column_name_list` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn parse_column_name_list(names: impl AsRef<str>) -> DeltaResult<Vec<ColumnName>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/column_names.rs#L53).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::column_names::ColumnName", "path": "ColumnName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [16, 1], "end": [122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs:53`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Parses a comma-separated list of column names, properly accounting for escapes and special
characters, e.g.:

```
# use buoyant_kernel as delta_kernel;
# use delta_kernel::expressions::ColumnName;
assert_eq!(
    &ColumnName::parse_column_name_list("a.b , c.`d , e` . f").unwrap(),
    &[ColumnName::new(["a", "b"]), ColumnName::new(["c", "d , e", "f"])]
);
```

<a id="op-7bb36a56675345ecc071cec4"></a>
## partial_cmp

`function` · `buoyant_kernel::expressions::column_names::ColumnName::partial_cmp` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn partial_cmp(&self, other: &ColumnName) -> option::Option<cmp::Ordering>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/column_names.rs#L11).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::column_names::ColumnName", "path": "ColumnName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11, 44], "end": [11, 54], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs:11`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f63a5c58acd0008bd42b6887"></a>
## path

`function` · `buoyant_kernel::expressions::column_names::ColumnName::path` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn path(&self) -> &[String]
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/column_names.rs#L93).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::column_names::ColumnName", "path": "ColumnName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [16, 1], "end": [122, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs:93`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The path of field names for this column name

<a id="op-bf8fa665ff69ccada8dfa1bd"></a>
## serialize

`function` · `buoyant_kernel::expressions::column_names::ColumnName::serialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/column_names.rs#L11).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::column_names::ColumnName", "path": "ColumnName"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11, 65], "end": [11, 74], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs:11`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d658aa89f22345f9d47e718"></a>
## path

`struct_field` · `buoyant_kernel::expressions::column_names::ColumnName::path` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
path: Vec<String>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/column_names.rs#L13).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/column_names.rs:13`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
