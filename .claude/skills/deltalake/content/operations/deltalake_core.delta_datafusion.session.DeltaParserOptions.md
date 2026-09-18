# `deltalake_core::delta_datafusion::session::DeltaParserOptions`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.session.DeltaParserOptions.json).

<a id="op-4ba3596f2cb3b7f99ad788e3"></a>
## DeltaParserOptions

`struct` · `deltalake_core::delta_datafusion::session::DeltaParserOptions` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DeltaParserOptions
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/session.rs#L252).

Source: `crates/core/src/delta_datafusion/session.rs:252`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A wrapper for sql_parser's ParserOptions to capture sane default table defaults

<a id="op-68fc9926e57005e1e23c0e44"></a>
## default

`function` · `deltalake_core::delta_datafusion::session::DeltaParserOptions::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/session.rs#L257).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::delta_datafusion::session::DeltaParserOptions", "path": "DeltaParserOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [256, 1], "end": [265, 2], "filename": "crates/core/src/delta_datafusion/session.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/delta_datafusion/session.rs:257`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4562b92bbfd108701020154"></a>
## inner

`struct_field` · `deltalake_core::delta_datafusion::session::DeltaParserOptions::inner` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
inner: datafusion::sql::planner::ParserOptions
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/session.rs#L253).

Source: `crates/core/src/delta_datafusion/session.rs:253`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
