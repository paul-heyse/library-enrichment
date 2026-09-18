# `object_store::azure::credential::AzureAuthorizer`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.azure.credential.AzureAuthorizer.json).

<a id="op-b51861c18f0b23164e8951c2"></a>
## AzureAuthorizer

`struct` · `object_store::azure::credential::AzureAuthorizer` · object_store 0.13.2

```rust
struct AzureAuthorizer<'a>
```

Source: `src/azure/credential.rs:224`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Authorize a [`HttpRequest`](../operations/object_store.client.http.body.HttpRequest.md#op-0f9bc5af637ad2947840b60b) with an [`AzureAuthorizer`](../operations/object_store.azure.credential.AzureAuthorizer.md#op-b51861c18f0b23164e8951c2)

<a id="op-d39063ee7b7bc8da473395b8"></a>
## authorize

`function` · `object_store::azure::credential::AzureAuthorizer::authorize` · object_store 0.13.2

```rust
fn authorize(&self, request: &mut HttpRequest)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "object_store::azure::credential::AzureAuthorizer", "path": "AzureAuthorizer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 1], "end": [271, 2], "filename": "src/azure/credential.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/credential.rs:239`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Authorize `request`

<a id="op-5c3b672a8ccc16374d3cb3df"></a>
## fmt

`function` · `object_store::azure::credential::AzureAuthorizer::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "object_store::azure::credential::AzureAuthorizer", "path": "AzureAuthorizer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 10], "end": [223, 15], "filename": "src/azure/credential.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/azure/credential.rs:223`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-194d42ca1cc03183de3dcbeb"></a>
## new

`function` · `object_store::azure::credential::AzureAuthorizer::new` · object_store 0.13.2

```rust
fn new(credential: &'a AzureCredential, account: &'a str) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "object_store::azure::credential::AzureAuthorizer", "path": "AzureAuthorizer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [229, 1], "end": [271, 2], "filename": "src/azure/credential.rs"}, "trait": null, "trait_path": null}`

Source: `src/azure/credential.rs:231`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create a new [`AzureAuthorizer`](../operations/object_store.azure.credential.AzureAuthorizer.md#op-b51861c18f0b23164e8951c2)
