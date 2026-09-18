# `deltalake_core::logstore::storage::runtime::IORuntime`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.storage.runtime.IORuntime.json).

<a id="op-efee47ae417b2a7e236e3c83"></a>
## IORuntime

`enum` · `deltalake_core::logstore::storage::runtime::IORuntime` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum IORuntime
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L85).

Source: `crates/core/src/logstore/storage/runtime.rs:85`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Provide custom Tokio RT or a runtime config

<a id="op-4199a73c99179f14847e9ec0"></a>
## Config

`variant` · `deltalake_core::logstore::storage::runtime::IORuntime::Config` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Config
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L89).

Source: `crates/core/src/logstore/storage/runtime.rs:89`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Configuration for tokio runtime

<a id="op-42909dc705fc843837688f9c"></a>
## RT

`variant` · `deltalake_core::logstore::storage::runtime::IORuntime::RT` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
RT
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L87).

Source: `crates/core/src/logstore/storage/runtime.rs:87`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Tokio RT handle

<a id="op-4b3d70e4db7f1ddfc0ec8e01"></a>
## clone

`function` · `deltalake_core::logstore::storage::runtime::IORuntime::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> IORuntime
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L84).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::storage::runtime::IORuntime", "path": "IORuntime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 17], "end": [84, 22], "filename": "crates/core/src/logstore/storage/runtime.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/logstore/storage/runtime.rs:84`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d666754e5da3812947a0ff2"></a>
## default

`function` · `deltalake_core::logstore::storage::runtime::IORuntime::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L93).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::storage::runtime::IORuntime", "path": "IORuntime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [96, 2], "filename": "crates/core/src/logstore/storage/runtime.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/logstore/storage/runtime.rs:93`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c9eeaf7e9f18b508c059211"></a>
## fmt

`function` · `deltalake_core::logstore::storage::runtime::IORuntime::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L84).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::storage::runtime::IORuntime", "path": "IORuntime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 10], "end": [84, 15], "filename": "crates/core/src/logstore/storage/runtime.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/logstore/storage/runtime.rs:84`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2bdf1d91b005d224491fb5b2"></a>
## get_handle

`function` · `deltalake_core::logstore::storage::runtime::IORuntime::get_handle` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_handle(&self) -> Handle
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/storage/runtime.rs#L100).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::logstore::storage::runtime::IORuntime", "path": "IORuntime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [107, 2], "filename": "crates/core/src/logstore/storage/runtime.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/logstore/storage/runtime.rs:100`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Retrieves the Tokio runtime for IO bound operations
