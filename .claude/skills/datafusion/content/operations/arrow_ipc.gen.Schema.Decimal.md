# `arrow_ipc::gen::Schema::Decimal`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_ipc.gen.Schema.Decimal.json).

<a id="op-5e3c769ab34224a670bc1df3"></a>
## Decimal

`struct` · `arrow_ipc::gen::Schema::Decimal` · arrow-ipc 59.3.0

```rust
struct Decimal<'a>
```

Source: `src/gen/Schema.rs:3104`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Exact decimal value represented as an integer value in two's
complement. Currently only 128-bit (16-byte) and 256-bit (32-byte) integers
are used. The representation uses the endianness indicated
in the Schema.

<a id="op-d748dfadaa4769972491b1fd"></a>
## Inner

`assoc_type` · `arrow_ipc::gen::Schema::Decimal::Inner` · arrow-ipc 59.3.0

```rust
Inner
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Decimal", "path": "Decimal"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3108, 1], "end": [3116, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Schema.rs:3109`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9df513c35b106ff63ed69731"></a>
## VT_BITWIDTH

`assoc_const` · `arrow_ipc::gen::Schema::Decimal::VT_BITWIDTH` · arrow-ipc 59.3.0

```rust
VT_BITWIDTH
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Decimal", "path": "Decimal"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3118, 1], "end": [3172, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:3121`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06a0ea280874d37f56d4c192"></a>
## VT_PRECISION

`assoc_const` · `arrow_ipc::gen::Schema::Decimal::VT_PRECISION` · arrow-ipc 59.3.0

```rust
VT_PRECISION
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Decimal", "path": "Decimal"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3118, 1], "end": [3172, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:3119`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2074c21ee34c9bab33d4235b"></a>
## VT_SCALE

`assoc_const` · `arrow_ipc::gen::Schema::Decimal::VT_SCALE` · arrow-ipc 59.3.0

```rust
VT_SCALE
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Decimal", "path": "Decimal"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3118, 1], "end": [3172, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:3120`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-545ad2d635028b22e4e19df6"></a>
## _tab

`struct_field` · `arrow_ipc::gen::Schema::Decimal::_tab` · arrow-ipc 59.3.0

```rust
_tab: flatbuffers::Table<'a>
```

Source: `src/gen/Schema.rs:3105`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7d63067bec50a4a8ef47159"></a>
## bitWidth

`function` · `arrow_ipc::gen::Schema::Decimal::bitWidth` · arrow-ipc 59.3.0

```rust
fn bitWidth(&self) -> i32
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Decimal", "path": "Decimal"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3118, 1], "end": [3172, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:3162`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Number of bits per value. The only accepted widths are 128 and 256.
We use bitWidth for consistency with Int::bitWidth.

<a id="op-a48848bb496ad209000d6c37"></a>
## clone

`function` · `arrow_ipc::gen::Schema::Decimal::clone` · arrow-ipc 59.3.0

```rust
fn clone(&self) -> Decimal<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Decimal", "path": "Decimal"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3098, 16], "end": [3098, 21], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/gen/Schema.rs:3098`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-afdd9ef556762463e8a12d58"></a>
## create

`function` · `arrow_ipc::gen::Schema::Decimal::create` · arrow-ipc 59.3.0

```rust
fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(_fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>, args: &'args DecimalArgs) -> flatbuffers::WIPOffset<Decimal<'bldr>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Decimal", "path": "Decimal"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3118, 1], "end": [3172, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:3128`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72fc0f705a502183cb802187"></a>
## eq

`function` · `arrow_ipc::gen::Schema::Decimal::eq` · arrow-ipc 59.3.0

```rust
fn eq(&self, other: &Decimal<'a>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Decimal", "path": "Decimal"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3098, 23], "end": [3098, 32], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/gen/Schema.rs:3098`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46208a3c1e46ba337631b82b"></a>
## fmt

`function` · `arrow_ipc::gen::Schema::Decimal::fmt` · arrow-ipc 59.3.0

```rust
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Decimal", "path": "Decimal"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3239, 1], "end": [3247, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/gen/Schema.rs:3240`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6360d780129649c97f20574"></a>
## follow

`function` · `arrow_ipc::gen::Schema::Decimal::follow` · arrow-ipc 59.3.0

```rust
unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Decimal", "path": "Decimal"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3108, 1], "end": [3116, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "flatbuffers::follow::Follow", "path": "Follow"}, "trait_path": "flatbuffers::follow::Follow"}`

Source: `src/gen/Schema.rs:3111`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-51f1e918575a363675b3a562"></a>
## init_from_table

`function` · `arrow_ipc::gen::Schema::Decimal::init_from_table` · arrow-ipc 59.3.0

```rust
unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Decimal", "path": "Decimal"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3118, 1], "end": [3172, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:3124`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb78ae4fa70eb87f009e2518"></a>
## precision

`function` · `arrow_ipc::gen::Schema::Decimal::precision` · arrow-ipc 59.3.0

```rust
fn precision(&self) -> i32
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Decimal", "path": "Decimal"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3118, 1], "end": [3172, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:3141`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Total number of decimal digits

<a id="op-51c79a5e2cc016c05c21d22d"></a>
## run_verifier

`function` · `arrow_ipc::gen::Schema::Decimal::run_verifier` · arrow-ipc 59.3.0

```rust
fn run_verifier(v: &mut flatbuffers::Verifier<'_, '_>, pos: usize) -> Result<(), flatbuffers::InvalidFlatbuffer>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'_"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Decimal", "path": "Decimal"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3174, 1], "end": [3188, 2], "filename": "src/gen/Schema.rs"}, "trait": {"args": null, "id": "flatbuffers::verifier::Verifiable", "path": "Verifiable"}, "trait_path": "flatbuffers::verifier::Verifiable"}`

Source: `src/gen/Schema.rs:3176`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e774aef3e6ce8cef15121667"></a>
## scale

`function` · `arrow_ipc::gen::Schema::Decimal::scale` · arrow-ipc 59.3.0

```rust
fn scale(&self) -> i32
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "arrow_ipc::gen::Schema::Decimal", "path": "Decimal"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3118, 1], "end": [3172, 2], "filename": "src/gen/Schema.rs"}, "trait": null, "trait_path": null}`

Source: `src/gen/Schema.rs:3153`. [Exact documentation build](https://docs.rs/crate/arrow-ipc/59.3.0/json).

Number of digits after the decimal point "."
