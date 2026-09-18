# `deltalake_lakefs::errors::LakeFSConfigError`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_lakefs.errors.LakeFSConfigError.json).

<a id="op-3e516705c242c80851e74b2a"></a>
## LakeFSConfigError

`enum` · `deltalake_lakefs::errors::LakeFSConfigError` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum LakeFSConfigError
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/errors.rs#L8).

Source: `crates/lakefs/src/errors.rs:8`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aada8ee737b4700cdc7c3c8f"></a>
## EndpointMissing

`variant` · `deltalake_lakefs::errors::LakeFSConfigError::EndpointMissing` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
EndpointMissing
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/errors.rs#L11).

Source: `crates/lakefs/src/errors.rs:11`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Missing endpoint

<a id="op-378a02f1815d46200d552185"></a>
## PasswordCredentialMissing

`variant` · `deltalake_lakefs::errors::LakeFSConfigError::PasswordCredentialMissing` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
PasswordCredentialMissing
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/errors.rs#L19).

Source: `crates/lakefs/src/errors.rs:19`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Missing password

<a id="op-e3d3d57d7d5bc8575337dbfa"></a>
## UsernameCredentialMissing

`variant` · `deltalake_lakefs::errors::LakeFSConfigError::UsernameCredentialMissing` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
UsernameCredentialMissing
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/errors.rs#L15).

Source: `crates/lakefs/src/errors.rs:15`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Missing username

<a id="op-244589d66d68db29248140e5"></a>
## fmt

`function` · `deltalake_lakefs::errors::LakeFSConfigError::fmt` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/errors.rs#L7).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_lakefs::errors::LakeFSConfigError", "path": "LakeFSConfigError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7, 28], "end": [7, 33], "filename": "crates/lakefs/src/errors.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/lakefs/src/errors.rs:7`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-83ff25f27a57b007b3d1f741"></a>
## fmt

`function` · `deltalake_lakefs::errors::LakeFSConfigError::fmt` · deltalake-lakefs 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/lakefs/src/errors.rs#L7).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_lakefs::errors::LakeFSConfigError", "path": "LakeFSConfigError"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7, 10], "end": [7, 26], "filename": "crates/lakefs/src/errors.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `crates/lakefs/src/errors.rs:7`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
