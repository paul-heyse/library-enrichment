# `buoyant_kernel::struct_patch::ProjectionStructPatchBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.struct_patch.ProjectionStructPatchBuilder.json).

<a id="op-c538ec8abbe84f3fd9494ac1"></a>
## ProjectionStructPatchBuilder

`struct` · `buoyant_kernel::struct_patch::ProjectionStructPatchBuilder` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ProjectionStructPatchBuilder<'a>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L655).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:655`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Builds schema and expression patches together over an input schema.

Emitted fields are paired with the expression that produces them, keeping the output schema and
sparse expression patch structurally aligned. Because it is bound to the input schema,
[`replace_expr`](Self::replace_expr) can preserve an existing [`StructField`](../operations/buoyant_kernel.schema.StructField.md#op-ebbc3bdce7a671f7a7d1f58d) while replacing
only its expression.

<a id="op-4f723ef933aacceb936fb1b2"></a>
## append

`function` · `buoyant_kernel::struct_patch::ProjectionStructPatchBuilder::append` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn append(self, field: StructField, expr: impl Into<ExpressionRef>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L835).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::ProjectionStructPatchBuilder", "path": "ProjectionStructPatchBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [672, 1], "end": [871, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:835`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Emits `field`, produced by `expr`, after all input fields and field-specific insertions.

<a id="op-63ae673e2e1437be2919e7d2"></a>
## append_at

`function` · `buoyant_kernel::struct_patch::ProjectionStructPatchBuilder::append_at` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn append_at(self, struct_path: impl CollectInto<ColumnName>, field: StructField, expr: impl Into<ExpressionRef>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L840).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::ProjectionStructPatchBuilder", "path": "ProjectionStructPatchBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [672, 1], "end": [871, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:840`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Emits `field`, produced by `expr`, after all fields of a nested struct.

<a id="op-e206219cf8473fbc5dd409b0"></a>
## build

`function` · `buoyant_kernel::struct_patch::ProjectionStructPatchBuilder::build` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn build(self) -> DeltaResult<(SchemaRef, ExpressionRef)>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L865).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::ProjectionStructPatchBuilder", "path": "ProjectionStructPatchBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [672, 1], "end": [871, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:865`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Builds a full output schema together with a matching sparse struct-patch expression.
Untouched input fields pass through implicitly and only inserted, replaced, dropped, or
nested fields are recorded.

The returned schema is always dense (it enumerates every output field), since a schema has
no sparse representation; only the expression side is sparse. The two halves stay
structurally consistent: applying the patch to a row of the builder's input schema yields a
struct matching the returned schema.

# Errors

Returns an error if a `with_*` call produced a conflicting operation, the input path cannot
be resolved to a struct, a required field patch references a missing input field, a nested
field patch targets a non-struct field, or the resulting output schema is invalid.

<a id="op-ac4e6afca61f33300a7a5c75"></a>
## drop

`function` · `buoyant_kernel::struct_patch::ProjectionStructPatchBuilder::drop` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn drop(self, field_name: impl Into<String>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L710).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::ProjectionStructPatchBuilder", "path": "ProjectionStructPatchBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [672, 1], "end": [871, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:710`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Records a field drop. The dropped field is omitted from both the output schema and the
emitted expressions.

<a id="op-c6caa4a743bf22d2e4a1a1da"></a>
## drop_at

`function` · `buoyant_kernel::struct_patch::ProjectionStructPatchBuilder::drop_at` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn drop_at(self, struct_path: impl CollectInto<ColumnName>, field_name: impl Into<String>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L716).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::ProjectionStructPatchBuilder", "path": "ProjectionStructPatchBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [672, 1], "end": [871, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:716`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Records a field drop in a nested struct.

<a id="op-fe339d830a3315eeec27b63f"></a>
## drop_if_exists

`function` · `buoyant_kernel::struct_patch::ProjectionStructPatchBuilder::drop_if_exists` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn drop_if_exists(self, field_name: impl Into<String>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L726).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::ProjectionStructPatchBuilder", "path": "ProjectionStructPatchBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [672, 1], "end": [871, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:726`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Records an optional field drop, tolerated when the input field is absent.

<a id="op-c02e5ca8acd0872e78b5c8d7"></a>
## drop_if_exists_at

`function` · `buoyant_kernel::struct_patch::ProjectionStructPatchBuilder::drop_if_exists_at` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn drop_if_exists_at(self, struct_path: impl CollectInto<ColumnName>, field_name: impl Into<String>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L732).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::ProjectionStructPatchBuilder", "path": "ProjectionStructPatchBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [672, 1], "end": [871, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:732`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Records an optional field drop in a nested struct.

<a id="op-885de2477b73005dd432c782"></a>
## fmt

`function` · `buoyant_kernel::struct_patch::ProjectionStructPatchBuilder::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L654).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::ProjectionStructPatchBuilder", "path": "ProjectionStructPatchBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [654, 10], "end": [654, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:654`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c30ced1f873a68ae6065cd5a"></a>
## insert_after

`function` · `buoyant_kernel::struct_patch::ProjectionStructPatchBuilder::insert_after` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn insert_after(self, field_name: impl Into<String>, field: StructField, expr: impl Into<ExpressionRef>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L814).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::ProjectionStructPatchBuilder", "path": "ProjectionStructPatchBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [672, 1], "end": [871, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:814`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Emits `field`, produced by `expr`, immediately after the named input field.

<a id="op-7772364c40012ede7c2b22d1"></a>
## insert_after_at

`function` · `buoyant_kernel::struct_patch::ProjectionStructPatchBuilder::insert_after_at` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn insert_after_at(self, struct_path: impl CollectInto<ColumnName>, field_name: impl Into<String>, field: StructField, expr: impl Into<ExpressionRef>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L824).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::ProjectionStructPatchBuilder", "path": "ProjectionStructPatchBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [672, 1], "end": [871, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:824`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Emits `field`, produced by `expr`, after the named input field of a nested struct.

<a id="op-25dfee8473f7bd5a4e5dfc25"></a>
## new

`function` · `buoyant_kernel::struct_patch::ProjectionStructPatchBuilder::new` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(input_schema: &'a StructType) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L690).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::ProjectionStructPatchBuilder", "path": "ProjectionStructPatchBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [672, 1], "end": [871, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:690`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a new top-level projection patch builder over `input_schema`.

<a id="op-6a438371335845e6dc06f8db"></a>
## new_nested

`function` · `buoyant_kernel::struct_patch::ProjectionStructPatchBuilder::new_nested` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new_nested(input_schema: &'a StructType, path: impl CollectInto<ColumnName>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L699).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::ProjectionStructPatchBuilder", "path": "ProjectionStructPatchBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [672, 1], "end": [871, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:699`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a projection patch builder over `input_schema` that operates on the nested struct
identified by `path`.

<a id="op-915b2421aabb1ff5d64c5010"></a>
## prepend

`function` · `buoyant_kernel::struct_patch::ProjectionStructPatchBuilder::prepend` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn prepend(self, field: StructField, expr: impl Into<ExpressionRef>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L799).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::ProjectionStructPatchBuilder", "path": "ProjectionStructPatchBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [672, 1], "end": [871, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:799`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Emits `field`, produced by `expr`, before all input fields.

<a id="op-058af3f1768927780c15107a"></a>
## prepend_at

`function` · `buoyant_kernel::struct_patch::ProjectionStructPatchBuilder::prepend_at` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn prepend_at(self, struct_path: impl CollectInto<ColumnName>, field: StructField, expr: impl Into<ExpressionRef>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L804).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::ProjectionStructPatchBuilder", "path": "ProjectionStructPatchBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [672, 1], "end": [871, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:804`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Emits `field`, produced by `expr`, before all fields of a nested struct.

<a id="op-eb98affdfdc73e45e9423590"></a>
## replace

`function` · `buoyant_kernel::struct_patch::ProjectionStructPatchBuilder::replace` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn replace(self, field_name: impl Into<String>, field: StructField, expr: impl Into<ExpressionRef>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L744).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::ProjectionStructPatchBuilder", "path": "ProjectionStructPatchBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [672, 1], "end": [871, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:744`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Replaces the named field with `field`, produced by `expr`.

<a id="op-5a0dc339fdcad578440da9b7"></a>
## replace_at

`function` · `buoyant_kernel::struct_patch::ProjectionStructPatchBuilder::replace_at` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn replace_at(self, struct_path: impl CollectInto<ColumnName>, field_name: impl Into<String>, field: StructField, expr: impl Into<ExpressionRef>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L754).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::ProjectionStructPatchBuilder", "path": "ProjectionStructPatchBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [672, 1], "end": [871, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:754`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Replaces the named field in a nested struct with `field`, produced by `expr`.

<a id="op-e5b37e8cc027257209bc96a3"></a>
## replace_expr

`function` · `buoyant_kernel::struct_patch::ProjectionStructPatchBuilder::replace_expr` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn replace_expr(self, field_name: impl Into<String>, expr: impl Into<ExpressionRef>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L767).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::ProjectionStructPatchBuilder", "path": "ProjectionStructPatchBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [672, 1], "end": [871, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:767`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Replaces the named field's expression while preserving its input [`StructField`](../operations/buoyant_kernel.schema.StructField.md#op-ebbc3bdce7a671f7a7d1f58d).

Any lookup error is recorded on the builder and returned by [`build`](Self::build).

<a id="op-dbb3e26e71f3a28eb2f57ea7"></a>
## replace_expr_at

`function` · `buoyant_kernel::struct_patch::ProjectionStructPatchBuilder::replace_expr_at` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn replace_expr_at(self, struct_path: impl CollectInto<ColumnName>, field_name: impl Into<String>, expr: impl Into<ExpressionRef>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L778).

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "buoyant_kernel::struct_patch::ProjectionStructPatchBuilder", "path": "ProjectionStructPatchBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [672, 1], "end": [871, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:778`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Replaces a nested field's expression while preserving its input [`StructField`](../operations/buoyant_kernel.schema.StructField.md#op-ebbc3bdce7a671f7a7d1f58d).

Any lookup error is recorded on the builder and returned by [`build`](Self::build).

<a id="op-f6351150675066e606ac16be"></a>
## inner

`struct_field` · `buoyant_kernel::struct_patch::ProjectionStructPatchBuilder::inner` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
inner: StructPatchBuilder<(schema::StructField, expressions::ExpressionRef)>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L657).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:657`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d27da8a252f29b1a82e0797"></a>
## input_schema

`struct_field` · `buoyant_kernel::struct_patch::ProjectionStructPatchBuilder::input_schema` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
input_schema: &'a schema::StructType
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L656).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:656`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
