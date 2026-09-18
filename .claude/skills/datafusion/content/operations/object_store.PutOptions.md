# `object_store::PutOptions`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.PutOptions.json).

<a id="op-f23e0e062d9c064f1e9c5bf1"></a>
## PutOptions

`struct` · `object_store::PutOptions` · object_store 0.13.2

```rust
struct PutOptions
```

Source: `src/lib.rs:1739`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Options for a put request

<a id="op-06146252022204938c7cb2fb"></a>
## attributes

`struct_field` · `object_store::PutOptions::attributes` · object_store 0.13.2

```rust
attributes: Attributes
```

Source: `src/lib.rs:1749`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Provide a set of [`Attributes`](../operations/object_store.attributes.Attributes.md#op-196267bf7f6d968d4ceb156e)

Implementations that don't support an attribute should return an error

<a id="op-4738aa84fa137817c50187c4"></a>
## clone

`function` · `object_store::PutOptions::clone` · object_store 0.13.2

```rust
fn clone(&self) -> PutOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::PutOptions", "path": "PutOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1738, 17], "end": [1738, 22], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/lib.rs:1738`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a996a48b11bc5539197ff48f"></a>
## default

`function` · `object_store::PutOptions::default` · object_store 0.13.2

```rust
fn default() -> PutOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::PutOptions", "path": "PutOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1738, 24], "end": [1738, 31], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/lib.rs:1738`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cbee7afa5beff42c7ad7f00e"></a>
## eq

`function` · `object_store::PutOptions::eq` · object_store 0.13.2

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::PutOptions", "path": "PutOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1759, 1], "end": [1775, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/lib.rs:1760`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa0fe94a33de20399587d82e"></a>
## extensions

`struct_field` · `object_store::PutOptions::extensions` · object_store 0.13.2

```rust
extensions: Extensions
```

Source: `src/lib.rs:1756`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Implementation-specific extensions. Intended for use by [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) implementations
that need to pass context-specific information (like tracing spans) via trait methods.

These extensions are ignored entirely by backends offered through this crate.

They are also excluded from [`PartialEq`] and [`Eq`].

Unresolved upstream links (retained, not inferred): ``Eq``, ``PartialEq``.

<a id="op-fe246025d782775ba79002c0"></a>
## fmt

`function` · `object_store::PutOptions::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::PutOptions", "path": "PutOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1738, 10], "end": [1738, 15], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/lib.rs:1738`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ce7cca03d3f18265e29bc5a"></a>
## from

`function` · `object_store::PutOptions::from` · object_store 0.13.2

```rust
fn from(mode: PutMode) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::PutOptions", "path": "PutOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1779, 1], "end": [1786, 2], "filename": "src/lib.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "object_store::PutMode", "path": "PutMode"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/lib.rs:1780`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ddf449682c13eb347c4aaf90"></a>
## from

`function` · `object_store::PutOptions::from` · object_store 0.13.2

```rust
fn from(tags: TagSet) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::PutOptions", "path": "PutOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1788, 1], "end": [1795, 2], "filename": "src/lib.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "object_store::tags::TagSet", "path": "TagSet"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/lib.rs:1789`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6e3f7f76794a2270d611cbd"></a>
## from

`function` · `object_store::PutOptions::from` · object_store 0.13.2

```rust
fn from(attributes: Attributes) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::PutOptions", "path": "PutOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1797, 1], "end": [1804, 2], "filename": "src/lib.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "object_store::attributes::Attributes", "path": "Attributes"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/lib.rs:1798`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4fd3a2771373f7ed756c097e"></a>
## mode

`struct_field` · `object_store::PutOptions::mode` · object_store 0.13.2

```rust
mode: PutMode
```

Source: `src/lib.rs:1741`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Configure the [`PutMode`](../operations/object_store.PutMode.md#op-6d1a652a18d92397a0c86baf) for this operation

<a id="op-93fd903caf9d35292edab8ba"></a>
## tags

`struct_field` · `object_store::PutOptions::tags` · object_store 0.13.2

```rust
tags: TagSet
```

Source: `src/lib.rs:1745`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Provide a [`TagSet`](../operations/object_store.tags.TagSet.md#op-de03e4ce862050b5b69144c8) for this object

Implementations that don't support object tagging should ignore this
