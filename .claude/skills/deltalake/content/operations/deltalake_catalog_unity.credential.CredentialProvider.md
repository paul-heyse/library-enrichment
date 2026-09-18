# `deltalake_catalog_unity::credential::CredentialProvider`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_unity.credential.CredentialProvider.json).

<a id="op-0cdf16a81ab49d6a55c4a24c"></a>
## CredentialProvider

`enum` · `deltalake_catalog_unity::credential::CredentialProvider` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum CredentialProvider
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L47).

Source: `crates/catalog-unity/src/credential.rs:47`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Provides credentials for use when signing requests

<a id="op-9e0f5617cbad41e6498c5a96"></a>
## BearerToken

`variant` · `deltalake_catalog_unity::credential::CredentialProvider::BearerToken` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
BearerToken
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L49).

Source: `crates/catalog-unity/src/credential.rs:49`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

static bearer token

<a id="op-3d009a1cda94ca103fe8f90e"></a>
## TokenCredential

`variant` · `deltalake_catalog_unity::credential::CredentialProvider::TokenCredential` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
TokenCredential
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L52).

Source: `crates/catalog-unity/src/credential.rs:52`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

a credential to fetch expiring auth tokens

<a id="op-60464cb0d48cd121a51803c1"></a>
## fmt

`function` · `deltalake_catalog_unity::credential::CredentialProvider::fmt` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/credential.rs#L46).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::credential::CredentialProvider", "path": "CredentialProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 10], "end": [46, 15], "filename": "crates/catalog-unity/src/credential.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/catalog-unity/src/credential.rs:46`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
