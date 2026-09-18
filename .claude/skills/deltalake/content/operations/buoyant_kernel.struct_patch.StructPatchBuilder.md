# `buoyant_kernel::struct_patch::StructPatchBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.struct_patch.StructPatchBuilder.json).

<a id="op-efa0d2ea419cd24c4cbedbb3"></a>
## StructPatchBuilder

`struct` · `buoyant_kernel::struct_patch::StructPatchBuilder` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct StructPatchBuilder<Item>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L90).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:90`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Builds a sparse struct patch from a sequence of requested patch operations.

The builder records user intent, checks for conflicting destructive operations, and lowers
nested field paths into recursive struct patches. The same builder surface drives both
expression patching
([`ExpressionStructPatchBuilder`](crate::expressions::ExpressionStructPatchBuilder)) and schema
patching ([`SchemaStructPatchBuilder`](crate::schema::SchemaStructPatchBuilder)); only the
terminal `build` step differs.

<a id="op-1f279e5edf7f6ee91d21f864"></a>
## append

`function` · `buoyant_kernel::struct_patch::StructPatchBuilder::append` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn append(self, item: impl Into<Item>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L317).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Item"}}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::StructPatchBuilder", "path": "StructPatchBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "Item"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [217, 1], "end": [357, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:317`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Records an item to append after all input fields and field-specific insertions.

<a id="op-cd78c7fcff6594d7fc747334"></a>
## append_at

`function` · `buoyant_kernel::struct_patch::StructPatchBuilder::append_at` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn append_at(self, struct_path: impl CollectInto<ColumnName>, item: impl Into<Item>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L322).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Item"}}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::StructPatchBuilder", "path": "StructPatchBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "Item"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [217, 1], "end": [357, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:322`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Records an item to append after all fields of a nested struct.

<a id="op-46a9f2ed66fe8a23a3bdb5ae"></a>
## build

`function` · `buoyant_kernel::struct_patch::StructPatchBuilder::build` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn build(self, input_schema: &StructType) -> DeltaResult<StructType>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L568).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructField", "path": "crate::schema::StructField"}}}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::StructPatchBuilder", "path": "StructPatchBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [556, 1], "end": [572, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:568`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Builds the output struct schema for this patch over `input_schema`.

If this builder targets a nested path (via [`new_nested`](Self::new_nested)), the returned
schema is the patched schema for the nested struct at that path, not the full top-level
input schema.

# Errors

Returns an error if a builder call produced a conflicting operation, the input path cannot
be resolved to a struct, a required field patch references a missing input field, a nested
field patch targets a non-struct field, or the resulting output schema is invalid.

<a id="op-a813d958b73a68d5e95425c9"></a>
## build

`function` · `buoyant_kernel::struct_patch::StructPatchBuilder::build` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn build(self) -> DeltaResult<ExpressionStructPatch>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L472).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::ExpressionRef", "path": "crate::expressions::ExpressionRef"}}}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::StructPatchBuilder", "path": "StructPatchBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [464, 1], "end": [476, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:472`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Builds the final expression patch.

# Errors

Returns an error when builder calls request multiple drop/replace operations for the
same field, or when a destructive operation on one field overlapped with an operation on a
nested child field.

<a id="op-de237ece1499bb7d53fa978a"></a>
## drop

`function` · `buoyant_kernel::struct_patch::StructPatchBuilder::drop` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn drop(self, field_name: impl Into<String>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L238).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Item"}}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::StructPatchBuilder", "path": "StructPatchBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "Item"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [217, 1], "end": [357, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:238`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Records a field drop.

<a id="op-765d6c2f3f5030717e79f088"></a>
## drop_at

`function` · `buoyant_kernel::struct_patch::StructPatchBuilder::drop_at` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn drop_at(self, struct_path: impl CollectInto<ColumnName>, field_name: impl Into<String>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L243).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Item"}}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::StructPatchBuilder", "path": "StructPatchBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "Item"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [217, 1], "end": [357, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:243`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Records a field drop in a nested struct.

<a id="op-e1e715c5806e97432d1cb8d2"></a>
## drop_if_exists

`function` · `buoyant_kernel::struct_patch::StructPatchBuilder::drop_if_exists` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn drop_if_exists(self, field_name: impl Into<String>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L252).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Item"}}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::StructPatchBuilder", "path": "StructPatchBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "Item"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [217, 1], "end": [357, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:252`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Records an optional field drop.

<a id="op-26037aa32e8e2eb9df86520a"></a>
## drop_if_exists_at

`function` · `buoyant_kernel::struct_patch::StructPatchBuilder::drop_if_exists_at` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn drop_if_exists_at(self, struct_path: impl CollectInto<ColumnName>, field_name: impl Into<String>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L257).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Item"}}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::StructPatchBuilder", "path": "StructPatchBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "Item"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [217, 1], "end": [357, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:257`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Records an optional field drop in a nested struct.

<a id="op-1550eefcb15784493615e2d2"></a>
## fmt

`function` · `buoyant_kernel::struct_patch::StructPatchBuilder::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L89).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Item"}}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::StructPatchBuilder", "path": "StructPatchBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::fmt::Debug", "path": "$crate::fmt::Debug"}}}], "default": null, "is_synthetic": false}}, "name": "Item"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 10], "end": [89, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:89`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06f263730b0ce7bcb0b4b9cd"></a>
## insert_after

`function` · `buoyant_kernel::struct_patch::StructPatchBuilder::insert_after` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn insert_after(self, field_name: impl Into<String>, item: impl Into<Item>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L300).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Item"}}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::StructPatchBuilder", "path": "StructPatchBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "Item"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [217, 1], "end": [357, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:300`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Records an item to insert after the named field.

<a id="op-8e588188f01257ed9e72d47d"></a>
## insert_after_at

`function` · `buoyant_kernel::struct_patch::StructPatchBuilder::insert_after_at` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn insert_after_at(self, struct_path: impl CollectInto<ColumnName>, field_name: impl Into<String>, item: impl Into<Item>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L305).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Item"}}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::StructPatchBuilder", "path": "StructPatchBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "Item"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [217, 1], "end": [357, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:305`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Records an item to insert after the named field in a nested struct.

<a id="op-e258849436edb1c6dd7ebb8f"></a>
## new

`function` · `buoyant_kernel::struct_patch::StructPatchBuilder::new` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new() -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L220).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Item"}}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::StructPatchBuilder", "path": "StructPatchBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "Item"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [217, 1], "end": [357, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:220`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a new top-level patch builder.

<a id="op-a7e6c270a10fb3762176ae05"></a>
## new_nested

`function` · `buoyant_kernel::struct_patch::StructPatchBuilder::new_nested` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new_nested(path: impl CollectInto<ColumnName>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L229).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Item"}}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::StructPatchBuilder", "path": "StructPatchBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "Item"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [217, 1], "end": [357, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:229`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a new builder that operates on fields of a nested struct identified by `path`.

<a id="op-b5cb32469379cef4742799f7"></a>
## prepend

`function` · `buoyant_kernel::struct_patch::StructPatchBuilder::prepend` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn prepend(self, item: impl Into<Item>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L283).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Item"}}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::StructPatchBuilder", "path": "StructPatchBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "Item"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [217, 1], "end": [357, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:283`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Records an item to emit before processing the first input field.

<a id="op-7ca3c73b9ee830468ef585b1"></a>
## prepend_at

`function` · `buoyant_kernel::struct_patch::StructPatchBuilder::prepend_at` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn prepend_at(self, struct_path: impl CollectInto<ColumnName>, item: impl Into<Item>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L288).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Item"}}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::StructPatchBuilder", "path": "StructPatchBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "Item"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [217, 1], "end": [357, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:288`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Records an item to emit before processing the first input field of a nested struct.

<a id="op-3c42aba2802b1f109be2bdae"></a>
## replace

`function` · `buoyant_kernel::struct_patch::StructPatchBuilder::replace` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn replace(self, field_name: impl Into<String>, item: impl Into<Item>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L266).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Item"}}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::StructPatchBuilder", "path": "StructPatchBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "Item"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [217, 1], "end": [357, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:266`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Records a field replacement.

<a id="op-8d3135298a09cb63e6804b1d"></a>
## replace_at

`function` · `buoyant_kernel::struct_patch::StructPatchBuilder::replace_at` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn replace_at(self, struct_path: impl CollectInto<ColumnName>, field_name: impl Into<String>, item: impl Into<Item>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L271).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "Item"}}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::StructPatchBuilder", "path": "StructPatchBuilder"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "Item"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [217, 1], "end": [357, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:271`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Records a field replacement in a nested struct.

<a id="op-dca943012ab7490a382307b8"></a>
## error

`struct_field` · `buoyant_kernel::struct_patch::StructPatchBuilder::error` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
error: DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L97).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:97`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The first error produced by a builder call, surfaced by `build`. Once set, later calls are
skipped so the original (most relevant) error is preserved.

<a id="op-ddbeb98b1f258bbfff345c7a"></a>
## input_path

`struct_field` · `buoyant_kernel::struct_patch::StructPatchBuilder::input_path` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
input_path: Option<expressions::ColumnName>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L92).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:92`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

None for a top-level patch; otherwise the path of the nested struct this patch targets.

<a id="op-f505cf1da9dddd7c3d20de3d"></a>
## root

`struct_field` · `buoyant_kernel::struct_patch::StructPatchBuilder::root` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
root: StructPatchNode<Item>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L94).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:94`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The patch tree assembled so far, with each builder call applied eagerly.
