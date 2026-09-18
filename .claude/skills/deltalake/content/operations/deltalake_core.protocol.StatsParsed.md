# `deltalake_core::protocol::StatsParsed`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.protocol.StatsParsed.json).

<a id="op-b8d508aae95222ea87ed8dbb"></a>
## StatsParsed

`struct` · `deltalake_core::protocol::StatsParsed` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct StatsParsed
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L137).

Source: `crates/core/src/protocol/mod.rs:137`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

File stats parsed from raw parquet format.

<a id="op-c2d6273fe0aa8bc118b6044a"></a>
## default

`function` · `deltalake_core::protocol::StatsParsed::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> StatsParsed
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L136).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::StatsParsed", "path": "StatsParsed"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 17], "end": [136, 24], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/protocol/mod.rs:136`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8f7fafa2f844547a3aa64bb"></a>
## fmt

`function` · `deltalake_core::protocol::StatsParsed::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L136).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::StatsParsed", "path": "StatsParsed"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [136, 10], "end": [136, 15], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/protocol/mod.rs:136`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-84db9a12cc3702368870eea3"></a>
## max_values

`struct_field` · `deltalake_core::protocol::StatsParsed::max_values` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
max_values: std::collections::HashMap<String, parquet::record::Field>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L146).

Source: `crates/core/src/protocol/mod.rs:146`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Contains a value larger than all values present in the file for all columns.
Contains a value larger than all values present in the file for all columns.

<a id="op-5239e3814d2b4da58b741652"></a>
## min_values

`struct_field` · `deltalake_core::protocol::StatsParsed::min_values` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
min_values: std::collections::HashMap<String, parquet::record::Field>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L143).

Source: `crates/core/src/protocol/mod.rs:143`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Contains a value smaller than all values present in the file for all columns.

<a id="op-85df04fc5a31c2fc58988c73"></a>
## null_count

`struct_field` · `deltalake_core::protocol::StatsParsed::null_count` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
null_count: std::collections::HashMap<String, i64>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L148).

Source: `crates/core/src/protocol/mod.rs:148`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The number of null values for all columns.

<a id="op-8dd89ebe934778cca45705bc"></a>
## num_records

`struct_field` · `deltalake_core::protocol::StatsParsed::num_records` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_records: i64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L139).

Source: `crates/core/src/protocol/mod.rs:139`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Number of records in the file associated with the log action.
