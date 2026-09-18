# `object_store::CopyOptions`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.CopyOptions.json).

<a id="op-bfcf06eee61cba2721fef09f"></a>
## CopyOptions

`struct` · `object_store::CopyOptions` · object_store 0.13.2

```rust
struct CopyOptions
```

Source: `src/lib.rs:1891`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Options for a copy request

<a id="op-d1a7ff07e0a31b26cc9ee6dc"></a>
## clone

`function` · `object_store::CopyOptions::clone` · object_store 0.13.2

```rust
fn clone(&self) -> CopyOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::CopyOptions", "path": "CopyOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1890, 17], "end": [1890, 22], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/lib.rs:1890`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c9087ffe6a27a530e03fd66"></a>
## default

`function` · `object_store::CopyOptions::default` · object_store 0.13.2

```rust
fn default() -> CopyOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::CopyOptions", "path": "CopyOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1890, 24], "end": [1890, 31], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/lib.rs:1890`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14d12b1f6704ef83845fa031"></a>
## eq

`function` · `object_store::CopyOptions::eq` · object_store 0.13.2

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::CopyOptions", "path": "CopyOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1928, 1], "end": [1941, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/lib.rs:1929`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-40cf7224bf5a2981708955f0"></a>
## extensions

`struct_field` · `object_store::CopyOptions::extensions` · object_store 0.13.2

```rust
extensions: Extensions
```

Source: `src/lib.rs:1900`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Implementation-specific extensions. Intended for use by [`ObjectStore`](../operations/object_store.ObjectStore.md#op-94894eaf9e5f6b785baca8ca) implementations
that need to pass context-specific information (like tracing spans) via trait methods.

These extensions are ignored entirely by backends offered through this crate.

They are also excluded from [`PartialEq`] and [`Eq`].

Unresolved upstream links (retained, not inferred): ``Eq``, ``PartialEq``.

<a id="op-3bd16cb9fbb475bf0fc916fc"></a>
## fmt

`function` · `object_store::CopyOptions::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::CopyOptions", "path": "CopyOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1890, 10], "end": [1890, 15], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/lib.rs:1890`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a773790d33f202bb0d0a1768"></a>
## mode

`struct_field` · `object_store::CopyOptions::mode` · object_store 0.13.2

```rust
mode: CopyMode
```

Source: `src/lib.rs:1893`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Configure the [`CopyMode`](../operations/object_store.CopyMode.md#op-5e9ca7dad569b06b1370d098) for this operation

<a id="op-42cbb169a284eee75ca6a2d6"></a>
## new

`function` · `object_store::CopyOptions::new` · object_store 0.13.2

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::CopyOptions", "path": "CopyOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1903, 1], "end": [1926, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1905`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create a new [`CopyOptions`](../operations/object_store.CopyOptions.md#op-bfcf06eee61cba2721fef09f)

<a id="op-2c88025df01852072d92eb0a"></a>
## with_extensions

`function` · `object_store::CopyOptions::with_extensions` · object_store 0.13.2

```rust
fn with_extensions(self, extensions: Extensions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::CopyOptions", "path": "CopyOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1903, 1], "end": [1926, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1922`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets the `extensions`.

See [`CopyOptions::extensions`](../operations/object_store.CopyOptions.md#op-40cf7224bf5a2981708955f0).

<a id="op-c60ac58f6c5e8ce70ef1f81e"></a>
## with_mode

`function` · `object_store::CopyOptions::with_mode` · object_store 0.13.2

```rust
fn with_mode(self, mode: CopyMode) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::CopyOptions", "path": "CopyOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1903, 1], "end": [1926, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:1913`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets the `mode.

See [`CopyOptions::mode`](../operations/object_store.CopyOptions.md#op-a773790d33f202bb0d0a1768).
