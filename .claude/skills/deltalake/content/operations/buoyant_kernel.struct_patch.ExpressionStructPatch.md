# `buoyant_kernel::struct_patch::ExpressionStructPatch`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.struct_patch.ExpressionStructPatch.json).

<a id="op-1f79369df99deab3abc687ed"></a>
## ExpressionStructPatch

`struct` · `buoyant_kernel::struct_patch::ExpressionStructPatch` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ExpressionStructPatch
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L53).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:53`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A sparse expression patch over the fields of one input struct.

`ExpressionStructPatch` achieves `O(changes)` space complexity instead of `O(schema_width)` by
only specifying fields that actually change (inserted, replaced, or deleted). Any input field
not specifically mentioned by the patch is passed through, unmodified and with the same relative
field ordering. This is particularly useful for wide schemas where only a few columns need to be
modified and/or dropped, or where a small number of columns need to be injected.

<a id="op-8cedcc8d9223da825a935b55"></a>
## Error

`assoc_type` · `buoyant_kernel::struct_patch::ExpressionStructPatch::Error` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Error = Error
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L479).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::struct_patch::ExpressionStructPatch", "path": "ExpressionStructPatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [478, 1], "end": [484, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::StructPatchBuilder", "path": "StructPatchBuilder"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:479`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4252cc26a8129624f6ac9596"></a>
## appended_fields

`struct_field` · `buoyant_kernel::struct_patch::ExpressionStructPatch::appended_fields` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
appended_fields: Vec<expressions::ExpressionRef>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L62).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:62`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A list of new fields to emit after all input fields and field-specific insertions.

<a id="op-df00203926c6911f59710397"></a>
## clone

`function` · `buoyant_kernel::struct_patch::ExpressionStructPatch::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> ExpressionStructPatch
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L52).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::struct_patch::ExpressionStructPatch", "path": "ExpressionStructPatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 17], "end": [52, 22], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:52`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec721c714e02043b996b079d"></a>
## default

`function` · `buoyant_kernel::struct_patch::ExpressionStructPatch::default` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> ExpressionStructPatch
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L52).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::struct_patch::ExpressionStructPatch", "path": "ExpressionStructPatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 35], "end": [52, 42], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:52`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80d15281eddb6aff102ca4a7"></a>
## deserialize

`function` · `buoyant_kernel::struct_patch::ExpressionStructPatch::deserialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L52).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::struct_patch::ExpressionStructPatch", "path": "ExpressionStructPatch"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 55], "end": [52, 66], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:52`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3670f0bd9131d9eed5a1971e"></a>
## eq

`function` · `buoyant_kernel::struct_patch::ExpressionStructPatch::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &ExpressionStructPatch) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L52).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::struct_patch::ExpressionStructPatch", "path": "ExpressionStructPatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 24], "end": [52, 33], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:52`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4bbf7027b00e2404c6d2b6a"></a>
## field_patches

`struct_field` · `buoyant_kernel::struct_patch::ExpressionStructPatch::field_patches` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
field_patches: std::collections::HashMap<String, ExpressionFieldPatch>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L58).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:58`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A mapping from named input fields to the patch to be performed on each field.

<a id="op-866a1544735073ff9d95ba72"></a>
## fmt

`function` · `buoyant_kernel::struct_patch::ExpressionStructPatch::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L52).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::struct_patch::ExpressionStructPatch", "path": "ExpressionStructPatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 10], "end": [52, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:52`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f3cd19a86670f653c22902d"></a>
## input_path

`struct_field` · `buoyant_kernel::struct_patch::ExpressionStructPatch::input_path` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
input_path: Option<expressions::ColumnName>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L56).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:56`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The path to the nested input struct this patch operates on (if any). If no path is given,
the patch operates directly on top-level columns.

<a id="op-b2243eb9edd59f5a355da4ad"></a>
## input_path

`function` · `buoyant_kernel::struct_patch::ExpressionStructPatch::input_path` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn input_path(&self) -> Option<&ColumnName>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L74).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::struct_patch::ExpressionStructPatch", "path": "ExpressionStructPatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [77, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:74`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

None if this is a top-level patch. Otherwise, the path of this nested patch.

<a id="op-4b2434d7fed84e2591737441"></a>
## is_empty

`function` · `buoyant_kernel::struct_patch::ExpressionStructPatch::is_empty` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn is_empty(&self) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L67).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::struct_patch::ExpressionStructPatch", "path": "ExpressionStructPatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [77, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:67`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

True if this patch makes no changes to the selected input struct.

<a id="op-305731068fbf3effc0bf2477"></a>
## prepended_fields

`struct_field` · `buoyant_kernel::struct_patch::ExpressionStructPatch::prepended_fields` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
prepended_fields: Vec<expressions::ExpressionRef>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L60).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:60`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A list of new fields to emit before processing the first input field.

<a id="op-1df509965524125f6f42d72c"></a>
## serialize

`function` · `buoyant_kernel::struct_patch::ExpressionStructPatch::serialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L52).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::struct_patch::ExpressionStructPatch", "path": "ExpressionStructPatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 44], "end": [52, 53], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:52`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8300861e14175aeaa742c34f"></a>
## try_from

`function` · `buoyant_kernel::struct_patch::ExpressionStructPatch::try_from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_from(builder: StructPatchBuilder<ExpressionRef>) -> DeltaResult<Self>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L481).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::struct_patch::ExpressionStructPatch", "path": "ExpressionStructPatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [478, 1], "end": [484, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::Expression", "path": "Expression"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::StructPatchBuilder", "path": "StructPatchBuilder"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:481`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
