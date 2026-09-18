# `object_store::azure::credential::AzureAccessKey`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.azure.credential.AzureAccessKey.json).

<a id="op-94b6dfe205323b479e12703e"></a>
## AzureAccessKey

`struct` · `object_store::azure::credential::AzureAccessKey` · object_store 0.13.2

```rust
struct AzureAccessKey
```

Source: `src/azure/credential.rs:111`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

A shared Azure Storage Account Key

<a id="op-0ab7dc293be062215aa23fda"></a>
## clone

`function` · `object_store::azure::credential::AzureAccessKey::clone` · object_store 0.13.2

```rust
fn clone(&self) -> AzureAccessKey
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::credential::AzureAccessKey", "path": "AzureAccessKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 17], "end": [110, 22], "filename": "src/azure/credential.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/azure/credential.rs:110`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8de6609d64f12456d6928aa"></a>
## eq

`function` · `object_store::azure::credential::AzureAccessKey::eq` · object_store 0.13.2

```rust
fn eq(&self, other: &AzureAccessKey) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::credential::AzureAccessKey", "path": "AzureAccessKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 28], "end": [110, 37], "filename": "src/azure/credential.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/azure/credential.rs:110`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3607bd2f7230e66468de5d69"></a>
## fmt

`function` · `object_store::azure::credential::AzureAccessKey::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::credential::AzureAccessKey", "path": "AzureAccessKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [110, 10], "end": [110, 15], "filename": "src/azure/credential.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/azure/credential.rs:110`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5cad7286dde4afc26918e9ae"></a>
## try_new

`function` · `object_store::azure::credential::AzureAccessKey::try_new` · object_store 0.13.2

```rust
fn try_new(key: &str) -> std::result::Result<Self, Error>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::credential::AzureAccessKey", "path": "AzureAccessKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 1], "end": [122, 2], "filename": "src/azure/credential.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/credential.rs:115`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create a new [`AzureAccessKey`](../operations/object_store.azure.credential.AzureAccessKey.md#op-94b6dfe205323b479e12703e), checking it for validity
