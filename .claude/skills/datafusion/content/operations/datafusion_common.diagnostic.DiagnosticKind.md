# `datafusion_common::diagnostic::DiagnosticKind`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.diagnostic.DiagnosticKind.json).

<a id="op-d878697ca52bce7f2a69ff11"></a>
## DiagnosticKind

`enum` · `datafusion_common::diagnostic::DiagnosticKind` · datafusion-common 55.1.0

```rust
enum DiagnosticKind
```

Source: `src/diagnostic.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A [`Diagnostic`](../operations/datafusion_common.diagnostic.Diagnostic.md#op-c0ce5c2af0812acb9f56efc1) can either be a hard error that prevents the query from
being planned and executed, or a warning that indicates potential issues,
performance problems, or causes for unexpected results, but is non-fatal.
This enum expresses these two possibilities.

<a id="op-1eea5052a2714ee72c0802a7"></a>
## Error

`variant` · `datafusion_common::diagnostic::DiagnosticKind::Error` · datafusion-common 55.1.0

```rust
Error
```

Source: `src/diagnostic.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24a1fcb4f1118f4d3c369118"></a>
## Warning

`variant` · `datafusion_common::diagnostic::DiagnosticKind::Warning` · datafusion-common 55.1.0

```rust
Warning
```

Source: `src/diagnostic.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6f91e4128d8797d9aa337b1d"></a>
## clone

`function` · `datafusion_common::diagnostic::DiagnosticKind::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> DiagnosticKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::diagnostic::DiagnosticKind", "path": "DiagnosticKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 17], "end": [81, 22], "filename": "src/diagnostic.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/diagnostic.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86ac6cbfbf128464423be493"></a>
## eq

`function` · `datafusion_common::diagnostic::DiagnosticKind::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &DiagnosticKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::diagnostic::DiagnosticKind", "path": "DiagnosticKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 30], "end": [81, 39], "filename": "src/diagnostic.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/diagnostic.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c914bc4aa4eeb9a50637444"></a>
## fmt

`function` · `datafusion_common::diagnostic::DiagnosticKind::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::diagnostic::DiagnosticKind", "path": "DiagnosticKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 10], "end": [81, 15], "filename": "src/diagnostic.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/diagnostic.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
