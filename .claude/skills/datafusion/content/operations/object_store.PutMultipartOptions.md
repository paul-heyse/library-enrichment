# `object_store::PutMultipartOptions`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.PutMultipartOptions.json).

<a id="op-f0610fbfc551432bb3870154"></a>
## PutMultipartOptions

`struct` · `object_store::PutMultipartOptions` · object_store 0.13.2

```rust
struct PutMultipartOptions
```

Source: `src/lib.rs:1813`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Options for [`ObjectStore::put_multipart_opts`](../operations/object_store.ObjectStore.md#op-27dfce2ef1fd51c4e2938336)

<a id="op-a4b34ad1534cf01025f911b7"></a>
## attributes

`struct_field` · `object_store::PutMultipartOptions::attributes` · object_store 0.13.2

```rust
attributes: Attributes
```

Source: `src/lib.rs:1821`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Provide a set of [`Attributes`](../operations/object_store.attributes.Attributes.md#op-196267bf7f6d968d4ceb156e)

Implementations that don't support an attribute should return an error

<a id="op-67cf855a45395cfbd9d0752a"></a>
## clone

`function` · `object_store::PutMultipartOptions::clone` · object_store 0.13.2

```rust
fn clone(&self) -> PutMultipartOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::PutMultipartOptions", "path": "PutMultipartOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1812, 17], "end": [1812, 22], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/lib.rs:1812`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8802f4dd48c7460127bd9e5"></a>
## default

`function` · `object_store::PutMultipartOptions::default` · object_store 0.13.2

```rust
fn default() -> PutMultipartOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::PutMultipartOptions", "path": "PutMultipartOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1812, 24], "end": [1812, 31], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/lib.rs:1812`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a542eaa6a6ba684756f6a206"></a>
## eq

`function` · `object_store::PutMultipartOptions::eq` · object_store 0.13.2

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::PutMultipartOptions", "path": "PutMultipartOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1831, 1], "end": [1845, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/lib.rs:1832`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f56c3682ebed2139eb3f61a5"></a>
## extensions

`struct_field` · `object_store::PutMultipartOptions::extensions` · object_store 0.13.2

```rust
extensions: Extensions
```

Source: `src/lib.rs:1828`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Implementation-specific extensions. Intended for use by [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) implementations
that need to pass context-specific information (like tracing spans) via trait methods.

These extensions are ignored entirely by backends offered through this crate.

They are also excluded from [`PartialEq`] and [`Eq`].

Unresolved upstream links (retained, not inferred): ``Eq``, ``PartialEq``.

<a id="op-274018bf6bb5bfc5ab1c9ff1"></a>
## fmt

`function` · `object_store::PutMultipartOptions::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::PutMultipartOptions", "path": "PutMultipartOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1812, 10], "end": [1812, 15], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/lib.rs:1812`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69c9bd6c80cc4fdb631682dd"></a>
## from

`function` · `object_store::PutMultipartOptions::from` · object_store 0.13.2

```rust
fn from(attributes: Attributes) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::PutMultipartOptions", "path": "PutMultipartOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1858, 1], "end": [1865, 2], "filename": "src/lib.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "object_store::attributes::Attributes", "path": "Attributes"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/lib.rs:1859`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d28e29bf2023ac9309e592db"></a>
## from

`function` · `object_store::PutMultipartOptions::from` · object_store 0.13.2

```rust
fn from(tags: TagSet) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::PutMultipartOptions", "path": "PutMultipartOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1849, 1], "end": [1856, 2], "filename": "src/lib.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "object_store::tags::TagSet", "path": "TagSet"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/lib.rs:1850`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-305e0f5ab326eddccb83c51f"></a>
## tags

`struct_field` · `object_store::PutMultipartOptions::tags` · object_store 0.13.2

```rust
tags: TagSet
```

Source: `src/lib.rs:1817`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Provide a [`TagSet`](../operations/object_store.tags.TagSet.md#op-de03e4ce862050b5b69144c8) for this object

Implementations that don't support object tagging should ignore this
