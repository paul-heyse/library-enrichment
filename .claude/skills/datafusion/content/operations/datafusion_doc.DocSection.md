# `datafusion_doc::DocSection`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_doc.DocSection.json).

<a id="op-d377c517b3dbc32bfdba69bf"></a>
## DocSection

`struct` · `datafusion_doc::DocSection` · datafusion-doc 55.1.0

```rust
struct DocSection
```

Source: `src/lib.rs:169`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1dcc6875564658c4857be7c6"></a>
## clone

`function` · `datafusion_doc::DocSection::clone` · datafusion-doc 55.1.0

```rust
fn clone(&self) -> DocSection
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_doc::DocSection", "path": "DocSection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [168, 17], "end": [168, 22], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/lib.rs:168`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3fc80a7bbdb60a56f14927f6"></a>
## default

`function` · `datafusion_doc::DocSection::default` · datafusion-doc 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_doc::DocSection", "path": "DocSection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [191, 2], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/lib.rs:184`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

Returns a "default" Doc section.

This is suitable for user defined functions that do not appear in the
DataFusion documentation.

<a id="op-e2ecef1a5fcb300070dcc1f0"></a>
## description

`struct_field` · `datafusion_doc::DocSection::description` · datafusion-doc 55.1.0

```rust
description: Option<&'static str>
```

Source: `src/lib.rs:176`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

An optional description for the doc section

<a id="op-4adeb30a5e3708e4f970a163"></a>
## eq

`function` · `datafusion_doc::DocSection::eq` · datafusion-doc 55.1.0

```rust
fn eq(&self, other: &DocSection) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_doc::DocSection", "path": "DocSection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [168, 24], "end": [168, 33], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/lib.rs:168`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52c986554a5660ba310402d8"></a>
## fmt

`function` · `datafusion_doc::DocSection::fmt` · datafusion-doc 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_doc::DocSection", "path": "DocSection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [168, 10], "end": [168, 15], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/lib.rs:168`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4de76328d1f99b8720c75a5e"></a>
## hash

`function` · `datafusion_doc::DocSection::hash` · datafusion-doc 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_doc::DocSection", "path": "DocSection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [168, 39], "end": [168, 43], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/lib.rs:168`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71dab2b5b6dae7cdab05ff58"></a>
## include

`struct_field` · `datafusion_doc::DocSection::include` · datafusion-doc 55.1.0

```rust
include: bool
```

Source: `src/lib.rs:172`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

True to include this doc section in the public
documentation, false otherwise

<a id="op-6b88475ce8252df7a9b3dd50"></a>
## label

`struct_field` · `datafusion_doc::DocSection::label` · datafusion-doc 55.1.0

```rust
label: &'static str
```

Source: `src/lib.rs:174`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

A display label for the doc section. For example: "Math Expressions"
