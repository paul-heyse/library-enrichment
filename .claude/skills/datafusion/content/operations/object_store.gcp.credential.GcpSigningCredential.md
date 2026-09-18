# `object_store::gcp::credential::GcpSigningCredential`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.gcp.credential.GcpSigningCredential.json).

<a id="op-e7b2777de2728adb0f286741"></a>
## GcpSigningCredential

`struct` · `object_store::gcp::credential::GcpSigningCredential` · object_store 0.13.2

```rust
struct GcpSigningCredential
```

Source: `src/gcp/credential.rs:110`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

A Google Cloud Storage Credential for signing

<a id="op-b7f030e93499f93981e47a7b"></a>
## email

`struct_field` · `object_store::gcp::credential::GcpSigningCredential::email` · object_store 0.13.2

```rust
email: String
```

Source: `src/gcp/credential.rs:112`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The email of the service account

<a id="op-b246b8b42afe3971b9ea8fe0"></a>
## fmt

`function` · `object_store::gcp::credential::GcpSigningCredential::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::gcp::credential::GcpSigningCredential", "path": "GcpSigningCredential"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [109, 10], "end": [109, 15], "filename": "src/gcp/credential.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/gcp/credential.rs:109`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-875098d798916095d583dd2a"></a>
## private_key

`struct_field` · `object_store::gcp::credential::GcpSigningCredential::private_key` · object_store 0.13.2

```rust
private_key: Option<ServiceAccountKey>
```

Source: `src/gcp/credential.rs:122`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

An optional RSA private key

If provided this will be used to sign the URL, otherwise a call will be made to
[`iam.serviceAccounts.signBlob`]. This allows supporting credential sources
that don't expose the service account private key, e.g. [IMDS].

[IMDS]: https://cloud.google.com/docs/authentication/get-id-token#metadata-server
[`iam.serviceAccounts.signBlob`]: https://cloud.google.com/storage/docs/authentication/creating-signatures
