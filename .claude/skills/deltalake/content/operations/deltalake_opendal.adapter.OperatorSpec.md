# `deltalake_opendal::adapter::OperatorSpec`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_opendal.adapter.OperatorSpec.json).

<a id="op-3eb3c99b88cef66f9b9f4a35"></a>
## OperatorSpec

`struct` · `deltalake_opendal::adapter::OperatorSpec` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct OperatorSpec
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/adapter.rs#L15).

Source: `crates/opendal/src/adapter.rs:15`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Everything the generic factories need to build an OpenDAL-backed store for a
delta table: which OpenDAL service to use, its config, and where the table
lives within the resulting operator.

<a id="op-d3ff7fce936c01e5a3234961"></a>
## clone

`function` · `deltalake_opendal::adapter::OperatorSpec::clone` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> OperatorSpec
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/adapter.rs#L14).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_opendal::adapter::OperatorSpec", "path": "OperatorSpec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [14, 17], "end": [14, 22], "filename": "crates/opendal/src/adapter.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/opendal/src/adapter.rs:14`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd61dcb52418e4cc867bd175"></a>
## config

`struct_field` · `deltalake_opendal::adapter::OperatorSpec::config` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
config: Vec<(String, String)>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/adapter.rs#L19).

Source: `crates/opendal/src/adapter.rs:19`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Config key/value pairs consumed by [`opendal::Operator::via_iter`].

Unresolved upstream links (retained, not inferred): ``opendal::Operator::via_iter``.

<a id="op-3ad31a4a5f5c5dc290816536"></a>
## fmt

`function` · `deltalake_opendal::adapter::OperatorSpec::fmt` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/adapter.rs#L14).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_opendal::adapter::OperatorSpec", "path": "OperatorSpec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [14, 10], "end": [14, 15], "filename": "crates/opendal/src/adapter.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/opendal/src/adapter.rs:14`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c4ea4e90885687f631ff537"></a>
## scheme

`struct_field` · `deltalake_opendal::adapter::OperatorSpec::scheme` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
scheme: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/adapter.rs#L17).

Source: `crates/opendal/src/adapter.rs:17`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

OpenDAL service scheme, e.g. `"fs"`, `"memory"`, `"s3"`.

<a id="op-8fa47be7bb0dbd6fe45eecdd"></a>
## table_prefix

`struct_field` · `deltalake_opendal::adapter::OperatorSpec::table_prefix` · deltalake-opendal 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
table_prefix: object_store::path::Path
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/adapter.rs#L21).

Source: `crates/opendal/src/adapter.rs:21`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The table prefix relative to the operator root.
