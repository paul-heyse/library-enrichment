# `object_store::RenameOptions`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.RenameOptions.json).

<a id="op-8a930a0b4aa6831d1d1ccb09"></a>
## RenameOptions

`struct` · `object_store::RenameOptions` · object_store 0.13.2

```rust
struct RenameOptions
```

Source: `src/lib.rs:1961`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Options for a rename request

<a id="op-1ee35ba400c4b9880a208d10"></a>
## clone

`function` · `object_store::RenameOptions::clone` · object_store 0.13.2

```rust
fn clone(&self) -> RenameOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::RenameOptions", "path": "RenameOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1960, 17], "end": [1960, 22], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/lib.rs:1960`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ff16da0472712cfe1bbb11a"></a>
## default

`function` · `object_store::RenameOptions::default` · object_store 0.13.2

```rust
fn default() -> RenameOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::RenameOptions", "path": "RenameOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1960, 24], "end": [1960, 31], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/lib.rs:1960`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f5c23295f6961823568d8f4"></a>
## eq

`function` · `object_store::RenameOptions::eq` · object_store 0.13.2

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::RenameOptions", "path": "RenameOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1998, 1], "end": [2011, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/lib.rs:1999`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-187a82c25ee1405c6ad2e177"></a>
## extensions

`struct_field` · `object_store::RenameOptions::extensions` · object_store 0.13.2

```rust
extensions: Extensions
```

Source: `src/lib.rs:1970`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Implementation-specific extensions. Intended for use by [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) implementations
that need to pass context-specific information (like tracing spans) via trait methods.

These extensions are ignored entirely by backends offered through this crate.

They are also excluded from [`PartialEq`] and [`Eq`].

Unresolved upstream links (retained, not inferred): ``Eq``, ``PartialEq``.

<a id="op-7408da40752cc4f0c4a8d765"></a>
## fmt

`function` · `object_store::RenameOptions::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::RenameOptions", "path": "RenameOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1960, 10], "end": [1960, 15], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/lib.rs:1960`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc749287de219d74bc6e712f"></a>
## new

`function` · `object_store::RenameOptions::new` · object_store 0.13.2

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::RenameOptions", "path": "RenameOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1973, 1], "end": [1996, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1975`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create a new [`RenameOptions`](../operations/object_store.RenameOptions.md#op-8a930a0b4aa6831d1d1ccb09)

<a id="op-bfa20cbae1c3cbf67505e5de"></a>
## target_mode

`struct_field` · `object_store::RenameOptions::target_mode` · object_store 0.13.2

```rust
target_mode: RenameTargetMode
```

Source: `src/lib.rs:1963`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Configure the [`RenameTargetMode`](../operations/object_store.RenameTargetMode.md#op-fc30f2bd3edb0f54097fba9b) for this operation

<a id="op-0c2777654a5694abf5c94a5b"></a>
## with_extensions

`function` · `object_store::RenameOptions::with_extensions` · object_store 0.13.2

```rust
fn with_extensions(self, extensions: Extensions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::RenameOptions", "path": "RenameOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1973, 1], "end": [1996, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1992`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets the `extensions`.

See [`RenameOptions::extensions`](../operations/object_store.RenameOptions.md#op-187a82c25ee1405c6ad2e177).

<a id="op-e775cabb94a476c474d24611"></a>
## with_target_mode

`function` · `object_store::RenameOptions::with_target_mode` · object_store 0.13.2

```rust
fn with_target_mode(self, target_mode: RenameTargetMode) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::RenameOptions", "path": "RenameOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1973, 1], "end": [1996, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1983`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets the `target_mode=.

See [`RenameOptions::target_mode`](../operations/object_store.RenameOptions.md#op-bfa20cbae1c3cbf67505e5de).
