# `datafusion_common::diagnostic::Diagnostic`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.diagnostic.Diagnostic.json).

<a id="op-c0ce5c2af0812acb9f56efc1"></a>
## Diagnostic

`struct` · `datafusion_common::diagnostic::Diagnostic` · datafusion-common 55.1.0

```rust
struct Diagnostic
```

Source: `src/diagnostic.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Additional contextual information intended for end users, to help them
understand what went wrong by providing human-readable messages, and
locations in the source query that relate to the error in some way.

You can think of a single [`Diagnostic`](../operations/datafusion_common.diagnostic.Diagnostic.md#op-c0ce5c2af0812acb9f56efc1) as a single "block" of output from
rustc. i.e. either an error or a warning, optionally with some notes and
help messages.

Example:

```rust
# use datafusion_common::{Location, Span, Diagnostic};
let span = Some(Span {
    start: Location { line: 2, column: 1 },
    end: Location {
        line: 4,
        column: 15,
    },
});
let diagnostic = Diagnostic::new_error("Something went wrong", span)
    .with_help("Have you tried turning it on and off again?", None);
```

<a id="op-81694399de50702c0133531a"></a>
## add_help

`function` · `datafusion_common::diagnostic::Diagnostic::add_help` · datafusion-common 55.1.0

```rust
fn add_help(&mut self, message: impl Into<String>, span: Option<Span>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::diagnostic::Diagnostic", "path": "Diagnostic"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [149, 2], "filename": "src/diagnostic.rs"}, "trait": null, "trait_path": null}`

Source: `src/diagnostic.rs:131`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Adds a "help" to the [`Diagnostic`](../operations/datafusion_common.diagnostic.Diagnostic.md#op-c0ce5c2af0812acb9f56efc1), which can have zero or many. A
"help" helps the user understand how they might fix the error or
warning. It can refer to an arbitrary location in the SQL query, or to
no location.

<a id="op-e3bee48ea24fae66590f04b0"></a>
## add_note

`function` · `datafusion_common::diagnostic::Diagnostic::add_note` · datafusion-common 55.1.0

```rust
fn add_note(&mut self, message: impl Into<String>, span: Option<Span>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::diagnostic::Diagnostic", "path": "Diagnostic"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [149, 2], "filename": "src/diagnostic.rs"}, "trait": null, "trait_path": null}`

Source: `src/diagnostic.rs:120`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Adds a "note" to the [`Diagnostic`](../operations/datafusion_common.diagnostic.Diagnostic.md#op-c0ce5c2af0812acb9f56efc1), which can have zero or many. A "note"
helps contextualize the error and helps the end user understand why it
occurred. It can refer to an arbitrary location in the SQL query, or to
no location.

<a id="op-a6f72663e7de6009c9f87452"></a>
## clone

`function` · `datafusion_common::diagnostic::Diagnostic::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> Diagnostic
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::diagnostic::Diagnostic", "path": "Diagnostic"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 17], "end": [42, 22], "filename": "src/diagnostic.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/diagnostic.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4b1956e4f7cc00ada1a55dc"></a>
## fmt

`function` · `datafusion_common::diagnostic::Diagnostic::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::diagnostic::Diagnostic", "path": "Diagnostic"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 10], "end": [42, 15], "filename": "src/diagnostic.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/diagnostic.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7fa026a8a30f2832d485986"></a>
## helps

`struct_field` · `datafusion_common::diagnostic::Diagnostic::helps` · datafusion-common 55.1.0

```rust
helps: Vec<DiagnosticHelp>
```

Source: `src/diagnostic.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bf874dfbf52a0b9ff801c320"></a>
## kind

`struct_field` · `datafusion_common::diagnostic::Diagnostic::kind` · datafusion-common 55.1.0

```rust
kind: DiagnosticKind
```

Source: `src/diagnostic.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26e3b0ae1b2c550201d38101"></a>
## message

`struct_field` · `datafusion_common::diagnostic::Diagnostic::message` · datafusion-common 55.1.0

```rust
message: String
```

Source: `src/diagnostic.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72a886f1a0789f27ef57b141"></a>
## new_error

`function` · `datafusion_common::diagnostic::Diagnostic::new_error` · datafusion-common 55.1.0

```rust
fn new_error(message: impl Into<String>, span: Option<Span>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::diagnostic::Diagnostic", "path": "Diagnostic"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [149, 2], "filename": "src/diagnostic.rs"}, "trait": null, "trait_path": null}`

Source: `src/diagnostic.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Creates a new [`Diagnostic`](../operations/datafusion_common.diagnostic.Diagnostic.md#op-c0ce5c2af0812acb9f56efc1) for a fatal error that prevents the SQL
query from being planned and executed. Optionally takes in a [`Span`](../operations/datafusion_common.spans.Span.md#op-f6759e6258e5342ee1352ef2) to
describe the location in the source code that caused the error, should
be provided when available.

<a id="op-d347d1ca8e85e9176f2e3dbd"></a>
## new_warning

`function` · `datafusion_common::diagnostic::Diagnostic::new_warning` · datafusion-common 55.1.0

```rust
fn new_warning(message: impl Into<String>, span: Option<Span>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::diagnostic::Diagnostic", "path": "Diagnostic"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [149, 2], "filename": "src/diagnostic.rs"}, "trait": null, "trait_path": null}`

Source: `src/diagnostic.rs:106`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Creates a new [`Diagnostic`](../operations/datafusion_common.diagnostic.Diagnostic.md#op-c0ce5c2af0812acb9f56efc1) for a NON-fatal warning, such as a
performance problem, or possible cause for undesired results. Optionally
takes in a [`Span`](../operations/datafusion_common.spans.Span.md#op-f6759e6258e5342ee1352ef2) to describe the location in the source code that
caused the error, should be provided when available.

<a id="op-6cb9c7a5e34bae9547e67588"></a>
## notes

`struct_field` · `datafusion_common::diagnostic::Diagnostic::notes` · datafusion-common 55.1.0

```rust
notes: Vec<DiagnosticNote>
```

Source: `src/diagnostic.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c67fe2b76fbce8c3341214d"></a>
## span

`struct_field` · `datafusion_common::diagnostic::Diagnostic::span` · datafusion-common 55.1.0

```rust
span: Option<Span>
```

Source: `src/diagnostic.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c3844e059ec91b7ffb9cb4e"></a>
## with_help

`function` · `datafusion_common::diagnostic::Diagnostic::with_help` · datafusion-common 55.1.0

```rust
fn with_help(self, message: impl Into<String>, span: Option<Span>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::diagnostic::Diagnostic", "path": "Diagnostic"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [149, 2], "filename": "src/diagnostic.rs"}, "trait": null, "trait_path": null}`

Source: `src/diagnostic.rs:145`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Like [`Diagnostic::add_help`](../operations/datafusion_common.diagnostic.Diagnostic.md#op-81694399de50702c0133531a), but returns `self` to allow chaining.

<a id="op-b59122a32c1590b9af85d045"></a>
## with_note

`function` · `datafusion_common::diagnostic::Diagnostic::with_note` · datafusion-common 55.1.0

```rust
fn with_note(self, message: impl Into<String>, span: Option<Span>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::diagnostic::Diagnostic", "path": "Diagnostic"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 1], "end": [149, 2], "filename": "src/diagnostic.rs"}, "trait": null, "trait_path": null}`

Source: `src/diagnostic.rs:139`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Like [`Diagnostic::add_note`](../operations/datafusion_common.diagnostic.Diagnostic.md#op-e3bee48ea24fae66590f04b0), but returns `self` to allow chaining.
