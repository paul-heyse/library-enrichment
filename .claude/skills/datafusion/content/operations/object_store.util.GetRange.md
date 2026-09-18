# `object_store::util::GetRange`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.util.GetRange.json).

<a id="op-7d6ea29467dd35b38ee376c9"></a>
## GetRange

`enum` · `object_store::util::GetRange` · object_store 0.13.2

```rust
enum GetRange
```

Source: `src/util.rs:193`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Request only a portion of an object's bytes

These can be created from [usize] ranges, like

```rust
# use object_store::GetRange;
let range1: GetRange = (50..150).into();
let range2: GetRange = (50..=150).into();
let range3: GetRange = (50..).into();
let range4: GetRange = (..150).into();
```

Implementations may wish to inspect [`GetResult`] for the exact byte
range returned.

[`GetResult`]: crate::GetResult

Unresolved upstream links (retained, not inferred): `usize`.

<a id="op-65a32023cea31c5abe6ac3e6"></a>
## Bounded

`variant` · `object_store::util::GetRange::Bounded` · object_store 0.13.2

```rust
Bounded
```

Source: `src/util.rs:203`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Request a specific range of bytes

If the given range is zero-length or starts after the end of the object,
an error will be returned. Additionally, if the range ends after the end
of the object, the entire remainder of the object will be returned.
Otherwise, the exact requested range will be returned.

Note that range is u64 (i.e., not usize),
as `object_store` supports 32-bit architectures such as WASM

<a id="op-55297ab86e554fe8e6d3354b"></a>
## Offset

`variant` · `object_store::util::GetRange::Offset` · object_store 0.13.2

```rust
Offset
```

Source: `src/util.rs:205`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Request all bytes starting from a given byte offset

<a id="op-ecad7c4ce285e30618e827a5"></a>
## Suffix

`variant` · `object_store::util::GetRange::Suffix` · object_store 0.13.2

```rust
Suffix
```

Source: `src/util.rs:207`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Request up to the last n bytes

<a id="op-1ee0e8c5264c6db8c6153ebb"></a>
## as_range

`function` · `object_store::util::GetRange::as_range` · object_store 0.13.2

```rust
fn as_range(&self, len: u64) -> Result<Range<u64>, InvalidGetRange>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::util::GetRange", "path": "GetRange"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [272, 2], "filename": "src/util.rs"}, "trait": null, "trait_path": null}`

Source: `src/util.rs:244`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Convert to a [`Range`] if [valid](Self::is_valid).

Unresolved upstream links (retained, not inferred): ``Range``.

<a id="op-c7e57a5183fe6950be9e96cc"></a>
## clone

`function` · `object_store::util::GetRange::clone` · object_store 0.13.2

```rust
fn clone(&self) -> GetRange
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::util::GetRange", "path": "GetRange"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [192, 32], "end": [192, 37], "filename": "src/util.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/util.rs:192`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c58ad40b101b55be0322e3d4"></a>
## eq

`function` · `object_store::util::GetRange::eq` · object_store 0.13.2

```rust
fn eq(&self, other: &GetRange) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::util::GetRange", "path": "GetRange"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [192, 17], "end": [192, 26], "filename": "src/util.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/util.rs:192`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c7976848f4c8d763f232fa8"></a>
## fmt

`function` · `object_store::util::GetRange::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::util::GetRange", "path": "GetRange"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [274, 1], "end": [282, 2], "filename": "src/util.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/util.rs:275`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d864399858627377c0f524e8"></a>
## fmt

`function` · `object_store::util::GetRange::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::util::GetRange", "path": "GetRange"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [192, 10], "end": [192, 15], "filename": "src/util.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/util.rs:192`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a469e4214bb469c421a35bb"></a>
## from

`function` · `object_store::util::GetRange::from` · object_store 0.13.2

```rust
fn from(value: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::util::GetRange", "path": "GetRange"}}, "generics": {"params": [{"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "u64"}}], "constraints": []}}, "id": "core::ops::range::RangeBounds", "path": "RangeBounds"}}}], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [284, 1], "end": [298, 2], "filename": "src/util.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/util.rs:285`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cabffe2037593e602c26d78d"></a>
## is_valid

`function` · `object_store::util::GetRange::is_valid` · object_store 0.13.2

```rust
fn is_valid(&self) -> Result<(), InvalidGetRange>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::util::GetRange", "path": "GetRange"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [272, 2], "filename": "src/util.rs"}, "trait": null, "trait_path": null}`

Source: `src/util.rs:225`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Check if the range is valid.
