# `object_store::azure::credential::AzureCredential`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.azure.credential.AzureCredential.json).

<a id="op-7239f510cc92765d8438fabb"></a>
## AzureCredential

`enum` · `object_store::azure::credential::AzureCredential` · object_store 0.13.2

```rust
enum AzureCredential
```

Source: `src/azure/credential.rs:126`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

An Azure storage credential

<a id="op-7d7cfd518abb2ff32457ce80"></a>
## AccessKey

`variant` · `object_store::azure::credential::AzureCredential::AccessKey` · object_store 0.13.2

```rust
AccessKey
```

Source: `src/azure/credential.rs:130`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

A shared access key

<https://learn.microsoft.com/en-us/rest/api/storageservices/authorize-with-shared-key>

<a id="op-1eec073ac3d79e1c6c45fb66"></a>
## BearerToken

`variant` · `object_store::azure::credential::AzureCredential::BearerToken` · object_store 0.13.2

```rust
BearerToken
```

Source: `src/azure/credential.rs:138`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

An authorization token

<https://learn.microsoft.com/en-us/rest/api/storageservices/authorize-with-azure-active-directory>

<a id="op-f35ff7a297487ed89bc43f07"></a>
## SASToken

`variant` · `object_store::azure::credential::AzureCredential::SASToken` · object_store 0.13.2

```rust
SASToken
```

Source: `src/azure/credential.rs:134`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

A shared access signature

<https://learn.microsoft.com/en-us/rest/api/storageservices/delegate-access-with-shared-access-signature>

<a id="op-fee2acf66affb85ad47b63b1"></a>
## eq

`function` · `object_store::azure::credential::AzureCredential::eq` · object_store 0.13.2

```rust
fn eq(&self, other: &AzureCredential) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::credential::AzureCredential", "path": "AzureCredential"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [125, 21], "end": [125, 30], "filename": "src/azure/credential.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/azure/credential.rs:125`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03d57ee58ad6a973cd61e7f0"></a>
## fmt

`function` · `object_store::azure::credential::AzureCredential::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::credential::AzureCredential", "path": "AzureCredential"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [125, 10], "end": [125, 15], "filename": "src/azure/credential.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/azure/credential.rs:125`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b9b329768f0d42903857f42"></a>
## sensitive_request

`function` · `object_store::azure::credential::AzureCredential::sensitive_request` · object_store 0.13.2

```rust
fn sensitive_request(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::azure::credential::AzureCredential", "path": "AzureCredential"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [151, 2], "filename": "src/azure/credential.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/credential.rs:143`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Determines if the credential requires the request be treated as sensitive
