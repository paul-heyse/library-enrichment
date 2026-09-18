# `buoyant_kernel::expressions::MapToStructExpression`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.expressions.MapToStructExpression.json).

<a id="op-0343b75651d1ad5e7379f876"></a>
## MapToStructExpression

`struct` · `buoyant_kernel::expressions::MapToStructExpression` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct MapToStructExpression
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L552).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:552`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Transforms a `Map<String, String>` column into a struct whose schema is provided by the
evaluator's output type (via `result_type`). Each row in the map column becomes one row in
the output struct column: a `key` -> `value` mapping in the map means the struct field named
`key` receives `value`, parsed into the field's target type via [`PrimitiveType::parse_scalar`].
An empty-string value is the exception (aligning with Spark): it casts to itself for string, to
empty bytes for binary, and to null for every other type.

- Missing keys produce null values
- Parse errors are propagated (indicating a broken table)
- Duplicate map keys are resolved by taking the rightmost entry

[`PrimitiveType::parse_scalar`]: crate::schema::PrimitiveType::parse_scalar

<a id="op-6b8e87b4947af3cffa433414"></a>
## clone

`function` · `buoyant_kernel::expressions::MapToStructExpression::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> MapToStructExpression
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L551).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::MapToStructExpression", "path": "MapToStructExpression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 10], "end": [551, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:551`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e016a5cea5fa5cde2dc268d"></a>
## deserialize

`function` · `buoyant_kernel::expressions::MapToStructExpression::deserialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L551).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::MapToStructExpression", "path": "MapToStructExpression"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 46], "end": [551, 57], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:551`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-954c5b9425a201fd2f306983"></a>
## eq

`function` · `buoyant_kernel::expressions::MapToStructExpression::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &MapToStructExpression) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L551).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::MapToStructExpression", "path": "MapToStructExpression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 24], "end": [551, 33], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:551`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac5af06b0abc294975f9fb51"></a>
## fmt

`function` · `buoyant_kernel::expressions::MapToStructExpression::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L551).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::MapToStructExpression", "path": "MapToStructExpression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 17], "end": [551, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:551`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb5b7931e5c207a26d2e3f37"></a>
## map_expr

`struct_field` · `buoyant_kernel::expressions::MapToStructExpression::map_expr` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
map_expr: Box<Expression>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L554).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:554`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The expression that evaluates to a `Map<String, String>` column.

<a id="op-fb061464452addb845b98b5c"></a>
## serialize

`function` · `buoyant_kernel::expressions::MapToStructExpression::serialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/expressions/mod.rs#L551).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::MapToStructExpression", "path": "MapToStructExpression"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 35], "end": [551, 44], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/expressions/mod.rs:551`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
