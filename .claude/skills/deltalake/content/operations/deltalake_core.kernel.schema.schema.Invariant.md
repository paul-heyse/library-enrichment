# `deltalake_core::kernel::schema::schema::Invariant`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.schema.schema.Invariant.json).

<a id="op-8af80947e2b10046fb6eefed"></a>
## Invariant

`struct` · `deltalake_core::kernel::schema::schema::Invariant` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct Invariant
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/schema.rs#L23).

Source: `crates/core/src/kernel/schema/schema.rs:23`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

An invariant for a column that is enforced on all writes to a Delta table.

<a id="op-8c2add07ed6bbe6715754850"></a>
## as_any

`function` · `deltalake_core::kernel::schema::schema::Invariant::as_any` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn as_any(&self) -> &dyn Any
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/schema.rs#L49).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::schema::schema::Invariant", "path": "Invariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [52, 2], "filename": "crates/core/src/kernel/schema/schema.rs"}, "trait": {"args": null, "id": "deltalake_core::kernel::schema::DataCheck", "path": "DataCheck"}, "trait_path": "deltalake_core::kernel::schema::DataCheck"}`

Source: `crates/core/src/kernel/schema/schema.rs:49`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ca03ff158fcc55a445d37a9"></a>
## clone

`function` · `deltalake_core::kernel::schema::schema::Invariant::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> Invariant
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/schema.rs#L22).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::schema::schema::Invariant", "path": "Invariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 41], "end": [22, 46], "filename": "crates/core/src/kernel/schema/schema.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/kernel/schema/schema.rs:22`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c0201303438d2852d875ae1"></a>
## default

`function` · `deltalake_core::kernel::schema::schema::Invariant::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> Invariant
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/schema.rs#L22).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::schema::schema::Invariant", "path": "Invariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 32], "end": [22, 39], "filename": "crates/core/src/kernel/schema/schema.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/kernel/schema/schema.rs:22`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-114d77d971739dcfe77fdf1a"></a>
## eq

`function` · `deltalake_core::kernel::schema::schema::Invariant::eq` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &Invariant) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/schema.rs#L22).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::schema::schema::Invariant", "path": "Invariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 14], "end": [22, 23], "filename": "crates/core/src/kernel/schema/schema.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/core/src/kernel/schema/schema.rs:22`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e0271e300f1547a61ddb6ead"></a>
## field_name

`struct_field` · `deltalake_core::kernel::schema::schema::Invariant::field_name` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
field_name: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/schema.rs#L25).

Source: `crates/core/src/kernel/schema/schema.rs:25`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The full path to the field.

<a id="op-305dc6e20df33f7f89e748f8"></a>
## fmt

`function` · `deltalake_core::kernel::schema::schema::Invariant::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/schema.rs#L22).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::schema::schema::Invariant", "path": "Invariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [22, 25], "end": [22, 30], "filename": "crates/core/src/kernel/schema/schema.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/kernel/schema/schema.rs:22`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da14ce376418eb706ba9bb19"></a>
## get_expression

`function` · `deltalake_core::kernel::schema::schema::Invariant::get_expression` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_expression(&self) -> &str
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/schema.rs#L45).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::schema::schema::Invariant", "path": "Invariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [52, 2], "filename": "crates/core/src/kernel/schema/schema.rs"}, "trait": {"args": null, "id": "deltalake_core::kernel::schema::DataCheck", "path": "DataCheck"}, "trait_path": "deltalake_core::kernel::schema::DataCheck"}`

Source: `crates/core/src/kernel/schema/schema.rs:45`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e14e865db2f3bf3650b7a55b"></a>
## get_name

`function` · `deltalake_core::kernel::schema::schema::Invariant::get_name` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_name(&self) -> &str
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/schema.rs#L41).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::schema::schema::Invariant", "path": "Invariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [52, 2], "filename": "crates/core/src/kernel/schema/schema.rs"}, "trait": {"args": null, "id": "deltalake_core::kernel::schema::DataCheck", "path": "DataCheck"}, "trait_path": "deltalake_core::kernel::schema::DataCheck"}`

Source: `crates/core/src/kernel/schema/schema.rs:41`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f26088bd85f496fb6a4c280"></a>
## invariant_sql

`struct_field` · `deltalake_core::kernel::schema::schema::Invariant::invariant_sql` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
invariant_sql: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/schema.rs#L27).

Source: `crates/core/src/kernel/schema/schema.rs:27`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The SQL string that must always evaluate to true.

<a id="op-40546c645bce91757999aac5"></a>
## new

`function` · `deltalake_core::kernel::schema::schema::Invariant::new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(field_name: &str, invariant_sql: &str) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/schema.rs#L32).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::kernel::schema::schema::Invariant", "path": "Invariant"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 1], "end": [38, 2], "filename": "crates/core/src/kernel/schema/schema.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/kernel/schema/schema.rs:32`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new invariant
