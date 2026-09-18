# `datafusion_common::diagnostic::DiagnosticHelp`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.diagnostic.DiagnosticHelp.json).

<a id="op-fc60e353c6ef3e29b513752c"></a>
## DiagnosticHelp

`struct` · `datafusion_common::diagnostic::DiagnosticHelp` · datafusion-common 55.1.0

```rust
struct DiagnosticHelp
```

Source: `src/diagnostic.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A "help" enriches a [`Diagnostic`](../operations/datafusion_common.diagnostic.Diagnostic.md#op-c0ce5c2af0812acb9f56efc1) with extra information, possibly
referring to different locations in the original SQL query, that helps the
user understand how they might fix the error or warning.

Example:
SELECT id, name FROM users GROUP BY id
Help: Add 'name' here                 ^^^^

<a id="op-19aa330ac849666975e76841"></a>
## clone

`function` · `datafusion_common::diagnostic::DiagnosticHelp::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> DiagnosticHelp
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::diagnostic::DiagnosticHelp", "path": "DiagnosticHelp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 17], "end": [71, 22], "filename": "src/diagnostic.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/diagnostic.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d397f289206354835a6b4864"></a>
## fmt

`function` · `datafusion_common::diagnostic::DiagnosticHelp::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::diagnostic::DiagnosticHelp", "path": "DiagnosticHelp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 10], "end": [71, 15], "filename": "src/diagnostic.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/diagnostic.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-37a4c6997411b1d140aa1187"></a>
## message

`struct_field` · `datafusion_common::diagnostic::DiagnosticHelp::message` · datafusion-common 55.1.0

```rust
message: String
```

Source: `src/diagnostic.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c9474ac17d413e2b704ce750"></a>
## span

`struct_field` · `datafusion_common::diagnostic::DiagnosticHelp::span` · datafusion-common 55.1.0

```rust
span: Option<Span>
```

Source: `src/diagnostic.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
