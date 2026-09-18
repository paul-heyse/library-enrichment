# `buoyant_kernel::transforms::schema::SchemaDepthChecker`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.transforms.schema.SchemaDepthChecker.json).

<a id="op-9f2db0d8a492e5335ea055f8"></a>
## SchemaDepthChecker

`struct` · `buoyant_kernel::transforms::schema::SchemaDepthChecker` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct SchemaDepthChecker
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/schema.rs#L200).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs:200`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A schema "transform" that doesn't actually change the schema at all. Instead, it measures the
maximum depth of a schema, with a depth limit to prevent stack overflow. Useful for verifying
that a schema has reasonable depth before attempting to work with it.

<a id="op-4290fbd186842913953342db"></a>
## Output

`assoc_type` · `buoyant_kernel::transforms::schema::SchemaDepthChecker::Output` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Output = Result<(), ()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/schema.rs#L246).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transforms::schema::SchemaDepthChecker", "path": "SchemaDepthChecker"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [245, 1], "end": [260, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::transforms::schema::SchemaTransform", "path": "SchemaTransform"}, "trait_path": "buoyant_kernel::transforms::schema::SchemaTransform"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs:246`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9dc7eb74274c6b84a03829f2"></a>
## Residual

`assoc_type` · `buoyant_kernel::transforms::schema::SchemaDepthChecker::Residual` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Residual = <<SchemaDepthChecker as SchemaTransform>::Output<()> as Carrier>::Residual
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/schema.rs#L246).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transforms::schema::SchemaDepthChecker", "path": "SchemaDepthChecker"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [245, 1], "end": [260, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::transforms::schema::SchemaTransform", "path": "SchemaTransform"}, "trait_path": "buoyant_kernel::transforms::schema::SchemaTransform"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs:246`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81601241cc65f59753b3f9d0"></a>
## check

`function` · `buoyant_kernel::transforms::schema::SchemaDepthChecker::check` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn check(data_type: &DataType, depth_limit: usize) -> usize
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/schema.rs#L210).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transforms::schema::SchemaDepthChecker", "path": "SchemaDepthChecker"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [206, 1], "end": [244, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs:210`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Depth-checks the given data type against a given depth limit. The return value is the
largest depth seen, which is capped at one more than the depth limit (indicating the
recursion was terminated).

<a id="op-0d4dde39d8b54cf63226736c"></a>
## transform_array

`function` · `buoyant_kernel::transforms::schema::SchemaDepthChecker::transform_array` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_array(&mut self, atype: &'a ArrayType) -> Result<(), ()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/schema.rs#L254).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transforms::schema::SchemaDepthChecker", "path": "SchemaDepthChecker"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [245, 1], "end": [260, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::transforms::schema::SchemaTransform", "path": "SchemaTransform"}, "trait_path": "buoyant_kernel::transforms::schema::SchemaTransform"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs:254`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e904696a6896d321f9baf26d"></a>
## transform_map

`function` · `buoyant_kernel::transforms::schema::SchemaDepthChecker::transform_map` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_map(&mut self, mtype: &'a MapType) -> Result<(), ()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/schema.rs#L257).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transforms::schema::SchemaDepthChecker", "path": "SchemaDepthChecker"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [245, 1], "end": [260, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::transforms::schema::SchemaTransform", "path": "SchemaTransform"}, "trait_path": "buoyant_kernel::transforms::schema::SchemaTransform"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs:257`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-094926cf95ad77738c38b050"></a>
## transform_struct

`function` · `buoyant_kernel::transforms::schema::SchemaDepthChecker::transform_struct` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_struct(&mut self, stype: &'a StructType) -> Result<(), ()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/schema.rs#L248).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transforms::schema::SchemaDepthChecker", "path": "SchemaDepthChecker"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [245, 1], "end": [260, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::transforms::schema::SchemaTransform", "path": "SchemaTransform"}, "trait_path": "buoyant_kernel::transforms::schema::SchemaTransform"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs:248`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-037d8c5e635784a6ca08132c"></a>
## transform_struct_field

`function` · `buoyant_kernel::transforms::schema::SchemaDepthChecker::transform_struct_field` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_struct_field(&mut self, field: &'a StructField) -> Result<(), ()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/schema.rs#L251).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::transforms::schema::SchemaDepthChecker", "path": "SchemaDepthChecker"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [245, 1], "end": [260, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::transforms::schema::SchemaTransform", "path": "SchemaTransform"}, "trait_path": "buoyant_kernel::transforms::schema::SchemaTransform"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs:251`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-87819dc67798d45cab8d7b4c"></a>
## call_count

`struct_field` · `buoyant_kernel::transforms::schema::SchemaDepthChecker::call_count` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
call_count: usize
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/schema.rs#L204).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs:204`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b58ecd9f3e0f10e5064bac4"></a>
## current_depth

`struct_field` · `buoyant_kernel::transforms::schema::SchemaDepthChecker::current_depth` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
current_depth: usize
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/schema.rs#L203).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs:203`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5c6d455663137a47f676aef"></a>
## depth_limit

`struct_field` · `buoyant_kernel::transforms::schema::SchemaDepthChecker::depth_limit` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
depth_limit: usize
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/schema.rs#L201).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs:201`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5ba26d99eda5d67a932be2d"></a>
## max_depth_seen

`struct_field` · `buoyant_kernel::transforms::schema::SchemaDepthChecker::max_depth_seen` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
max_depth_seen: usize
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/schema.rs#L202).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs:202`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
