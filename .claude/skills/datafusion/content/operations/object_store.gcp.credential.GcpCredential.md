# `object_store::gcp::credential::GcpCredential`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.gcp.credential.GcpCredential.json).

<a id="op-9e7bcde58ae0710ab251579b"></a>
## GcpCredential

`struct` · `object_store::gcp::credential::GcpCredential` · object_store 0.13.2

```rust
struct GcpCredential
```

Source: `src/gcp/credential.rs:170`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

A Google Cloud Storage Credential

<a id="op-59ef920a4b7f2359b39fcb4e"></a>
## bearer

`struct_field` · `object_store::gcp::credential::GcpCredential::bearer` · object_store 0.13.2

```rust
bearer: String
```

Source: `src/gcp/credential.rs:172`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

An HTTP bearer token

<a id="op-cfd7632d6bf5e1e433ea55f6"></a>
## eq

`function` · `object_store::gcp::credential::GcpCredential::eq` · object_store 0.13.2

```rust
fn eq(&self, other: &GcpCredential) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::credential::GcpCredential", "path": "GcpCredential"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [169, 21], "end": [169, 30], "filename": "src/gcp/credential.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/gcp/credential.rs:169`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6bf143c63387571d9a491014"></a>
## fmt

`function` · `object_store::gcp::credential::GcpCredential::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::credential::GcpCredential", "path": "GcpCredential"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [169, 10], "end": [169, 15], "filename": "src/gcp/credential.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/gcp/credential.rs:169`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.
