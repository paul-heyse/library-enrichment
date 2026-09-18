# `datafusion_common::diagnostic::DiagnosticNote`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.diagnostic.DiagnosticNote.json).

<a id="op-b9afb4b09fd02a1ddde78e5d"></a>
## DiagnosticNote

`struct` · `datafusion_common::diagnostic::DiagnosticNote` · datafusion-common 55.1.0

```rust
struct DiagnosticNote
```

Source: `src/diagnostic.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A note enriches a [`Diagnostic`](../operations/datafusion_common.diagnostic.Diagnostic.md#op-c0ce5c2af0812acb9f56efc1) with extra information, possibly referring
to different locations in the original SQL query, that helps contextualize
the error and helps the end user understand why it occurred.

Example:
SELECT id, name FROM users GROUP BY id
Note:      ^^^^ 'name' is not in the GROUP BY clause

<a id="op-95a91224acc71288d05855d8"></a>
## clone

`function` · `datafusion_common::diagnostic::DiagnosticNote::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> DiagnosticNote
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::diagnostic::DiagnosticNote", "path": "DiagnosticNote"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 17], "end": [58, 22], "filename": "src/diagnostic.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/diagnostic.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d8e2bbd4bdbec3f6abab775"></a>
## fmt

`function` · `datafusion_common::diagnostic::DiagnosticNote::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::diagnostic::DiagnosticNote", "path": "DiagnosticNote"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 10], "end": [58, 15], "filename": "src/diagnostic.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/diagnostic.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11ace0324652a5f569aa583a"></a>
## message

`struct_field` · `datafusion_common::diagnostic::DiagnosticNote::message` · datafusion-common 55.1.0

```rust
message: String
```

Source: `src/diagnostic.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5155a0c22c00b50a853db87b"></a>
## span

`struct_field` · `datafusion_common::diagnostic::DiagnosticNote::span` · datafusion-common 55.1.0

```rust
span: Option<Span>
```

Source: `src/diagnostic.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
