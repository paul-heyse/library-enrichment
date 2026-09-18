# `arrow_data::data::BufferSpec`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_data.data.BufferSpec.json).

<a id="op-fa6af8773b4e82fa39427e11"></a>
## BufferSpec

`enum` · `arrow_data::data::BufferSpec` · arrow-data 59.3.0

```rust
enum BufferSpec
```

Source: `src/data.rs:1983`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Layout specification for a single data type buffer

<a id="op-16c79a128cac173312e2c3a6"></a>
## AlwaysNull

`variant` · `arrow_data::data::BufferSpec::AlwaysNull` · arrow-data 59.3.0

```rust
AlwaysNull
```

Source: `src/data.rs:2011`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Buffer is always null. Unused currently in Rust implementation,
(used in C++ for Union type)

<a id="op-626797ea824210143a425c29"></a>
## BitMap

`variant` · `arrow_data::data::BufferSpec::BitMap` · arrow-data 59.3.0

```rust
BitMap
```

Source: `src/data.rs:2007`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Buffer holds a bitmap.

Note: Unlike the C++ implementation, the null/validity buffer
is handled specially rather than as another of the buffers in
the spec, so this variant is only used for the Boolean type.

<a id="op-1d156935023f6782fe3c438f"></a>
## FixedWidth

`variant` · `arrow_data::data::BufferSpec::FixedWidth` · arrow-data 59.3.0

```rust
FixedWidth
```

Source: `src/data.rs:1994`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Each element is a fixed width primitive, with the given `byte_width` and `alignment`

`alignment` is the alignment required by Rust for an array of the corresponding primitive,
see [`Layout::array`](std::alloc::Layout::array) and [`std::mem::align_of`].

Arrow-rs requires that all buffers have at least this alignment, to allow for
[slice](std::slice) based APIs. Alignment in excess of this is not required to allow
for array slicing and interoperability with `Vec`, which cannot be over-aligned.

Note that these alignment requirements will vary between architectures

Unresolved upstream links (retained, not inferred): ``std::mem::align_of``, `std::slice`, `std::alloc::Layout::array`.

<a id="op-675899d6e980f6565db38dd5"></a>
## VariableWidth

`variant` · `arrow_data::data::BufferSpec::VariableWidth` · arrow-data 59.3.0

```rust
VariableWidth
```

Source: `src/data.rs:2001`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Variable width, such as string data for utf8 data

<a id="op-1d1c96ee787f282bcafa1470"></a>
## eq

`function` · `arrow_data::data::BufferSpec::eq` · arrow-data 59.3.0

```rust
fn eq(&self, other: &BufferSpec) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::BufferSpec", "path": "BufferSpec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1982, 17], "end": [1982, 26], "filename": "src/data.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/data.rs:1982`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a5b2630aa7d7de5877dea73"></a>
## fmt

`function` · `arrow_data::data::BufferSpec::fmt` · arrow-data 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_data::data::BufferSpec", "path": "BufferSpec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1982, 10], "end": [1982, 15], "filename": "src/data.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/data.rs:1982`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
