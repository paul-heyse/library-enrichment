# `object_store::gcp::credential::ServiceAccountKey`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.gcp.credential.ServiceAccountKey.json).

<a id="op-eeee0a0ef278256de6551696"></a>
## ServiceAccountKey

`struct` · `object_store::gcp::credential::ServiceAccountKey` · object_store 0.13.2

```rust
struct ServiceAccountKey
```

Source: `src/gcp/credential.rs:127`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

A private RSA key for a service account

<a id="op-1cf96c60c5488abeccb9bbaf"></a>
## fmt

`function` · `object_store::gcp::credential::ServiceAccountKey::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::credential::ServiceAccountKey", "path": "ServiceAccountKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [126, 10], "end": [126, 15], "filename": "src/gcp/credential.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/gcp/credential.rs:126`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e06b422077657a6b19b56998"></a>
## from_der

`function` · `object_store::gcp::credential::ServiceAccountKey::from_der` · object_store 0.13.2

```rust
fn from_der(key: &[u8]) -> std::result::Result<Self, Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::credential::ServiceAccountKey", "path": "ServiceAccountKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [166, 2], "filename": "src/gcp/credential.rs"}, "trait": null, "trait_path": null}`

Source: `src/gcp/credential.rs:149`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Parses an unencrypted PKCS#8-encoded RSA private key.

<a id="op-eb9effddde4a641625791654"></a>
## from_pem

`function` · `object_store::gcp::credential::ServiceAccountKey::from_pem` · object_store 0.13.2

```rust
fn from_pem(encoded: &[u8]) -> std::result::Result<Self, Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::credential::ServiceAccountKey", "path": "ServiceAccountKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [166, 2], "filename": "src/gcp/credential.rs"}, "trait": null, "trait_path": null}`

Source: `src/gcp/credential.rs:131`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Parses a pem-encoded RSA key

<a id="op-6cc82a6126eabee8dc20633e"></a>
## from_pkcs8

`function` · `object_store::gcp::credential::ServiceAccountKey::from_pkcs8` · object_store 0.13.2

```rust
fn from_pkcs8(key: &[u8]) -> std::result::Result<Self, Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::credential::ServiceAccountKey", "path": "ServiceAccountKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 1], "end": [166, 2], "filename": "src/gcp/credential.rs"}, "trait": null, "trait_path": null}`

Source: `src/gcp/credential.rs:144`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Parses an unencrypted PKCS#8-encoded RSA private key.
