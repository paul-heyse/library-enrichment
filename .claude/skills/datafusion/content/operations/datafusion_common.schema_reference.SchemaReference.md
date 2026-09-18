# `datafusion_common::schema_reference::SchemaReference`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.schema_reference.SchemaReference.json).

<a id="op-dea974eb53daf4015d02f2e6"></a>
## SchemaReference

`enum` · `datafusion_common::schema_reference::SchemaReference` · datafusion-common 55.1.0

```rust
enum SchemaReference
```

Source: `src/schema_reference.rs:21`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59dcc488413dc6195f31b4e0"></a>
## Bare

`variant` · `datafusion_common::schema_reference::SchemaReference::Bare` · datafusion-common 55.1.0

```rust
Bare
```

Source: `src/schema_reference.rs:22`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f0757fee6db03d05978e2ce"></a>
## Full

`variant` · `datafusion_common::schema_reference::SchemaReference::Full` · datafusion-common 55.1.0

```rust
Full
```

Source: `src/schema_reference.rs:23`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f1aebd25d475ff47a311f0e"></a>
## clone

`function` · `datafusion_common::schema_reference::SchemaReference::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> SchemaReference
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::schema_reference::SchemaReference", "path": "SchemaReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [20, 17], "end": [20, 22], "filename": "src/schema_reference.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/schema_reference.rs:20`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff831bb8feb29459a928777f"></a>
## cmp

`function` · `datafusion_common::schema_reference::SchemaReference::cmp` · datafusion-common 55.1.0

```rust
fn cmp(&self, other: &SchemaReference) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::schema_reference::SchemaReference", "path": "SchemaReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [20, 57], "end": [20, 60], "filename": "src/schema_reference.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/schema_reference.rs:20`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-202cff3d5fecaa621aadfc5e"></a>
## eq

`function` · `datafusion_common::schema_reference::SchemaReference::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &SchemaReference) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::schema_reference::SchemaReference", "path": "SchemaReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [20, 24], "end": [20, 33], "filename": "src/schema_reference.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/schema_reference.rs:20`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46e509d83a100c5e7c23f6cb"></a>
## fmt

`function` · `datafusion_common::schema_reference::SchemaReference::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::schema_reference::SchemaReference", "path": "SchemaReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [20, 10], "end": [20, 15], "filename": "src/schema_reference.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/schema_reference.rs:20`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df7714e8c0a58b94fc55e15a"></a>
## fmt

`function` · `datafusion_common::schema_reference::SchemaReference::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::schema_reference::SchemaReference", "path": "SchemaReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 1], "end": [43, 2], "filename": "src/schema_reference.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/schema_reference.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4216d9a2f45359a3cf7c3149"></a>
## hash

`function` · `datafusion_common::schema_reference::SchemaReference::hash` · datafusion-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::schema_reference::SchemaReference", "path": "SchemaReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [20, 39], "end": [20, 43], "filename": "src/schema_reference.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/schema_reference.rs:20`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-223675f129a13db1bec8edda"></a>
## partial_cmp

`function` · `datafusion_common::schema_reference::SchemaReference::partial_cmp` · datafusion-common 55.1.0

```rust
fn partial_cmp(&self, other: &SchemaReference) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::schema_reference::SchemaReference", "path": "SchemaReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [20, 45], "end": [20, 55], "filename": "src/schema_reference.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/schema_reference.rs:20`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94dae5b36561f1c040ba3a70"></a>
## schema_name

`function` · `datafusion_common::schema_reference::SchemaReference::schema_name` · datafusion-common 55.1.0

```rust
fn schema_name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::schema_reference::SchemaReference", "path": "SchemaReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [26, 1], "end": [34, 2], "filename": "src/schema_reference.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema_reference.rs:28`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Get only the schema name that this references.
