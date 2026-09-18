# `deltalake_core::logstore::CommitOrBytes`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.CommitOrBytes.json).

<a id="op-e7aa19d4afcd1cbbd6f0b02a"></a>
## CommitOrBytes

`enum` · `deltalake_core::logstore::CommitOrBytes` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum CommitOrBytes
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L312).

Source: `crates/core/src/logstore/mod.rs:312`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Holder whether it's tmp_commit path or commit bytes

<a id="op-08a07e623c83378221c21446"></a>
## LogBytes

`variant` · `deltalake_core::logstore::CommitOrBytes::LogBytes` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
LogBytes
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L316).

Source: `crates/core/src/logstore/mod.rs:316`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Bytes of the log, to be used by logstoers which use Conditional Put

<a id="op-42699ade2823a27d837a0af2"></a>
## TmpCommit

`variant` · `deltalake_core::logstore::CommitOrBytes::TmpCommit` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
TmpCommit
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L314).

Source: `crates/core/src/logstore/mod.rs:314`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Path of the tmp commit, to be used by logstores which use CopyIfNotExists

<a id="op-4279c31c040fa1255dbc42a3"></a>
## clone

`function` · `deltalake_core::logstore::CommitOrBytes::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> CommitOrBytes
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L311).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::CommitOrBytes", "path": "CommitOrBytes"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [311, 10], "end": [311, 15], "filename": "crates/core/src/logstore/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/logstore/mod.rs:311`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
