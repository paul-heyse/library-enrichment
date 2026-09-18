# `object_store::client::Certificate`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.client.Certificate.json).

<a id="op-7adfb375980663ce042e2770"></a>
## Certificate

`struct` · `object_store::client::Certificate` · object_store 0.13.2

```rust
struct Certificate
```

Source: `src/client/mod.rs:265`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Represents a CA certificate provided by the user.

This is used to configure the client to trust a specific certificate. See
[Self::from_pem](../operations/object_store.client.Certificate.md#op-1d3501d3f0b93f683973f84b) for an example

<a id="op-7a702e40c550635036a163a3"></a>
## clone

`function` · `object_store::client::Certificate::clone` · object_store 0.13.2

```rust
fn clone(&self) -> Certificate
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::Certificate", "path": "Certificate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [263, 17], "end": [263, 22], "filename": "src/client/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/client/mod.rs:263`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-592d2b9729e67a60d952154d"></a>
## fmt

`function` · `object_store::client::Certificate::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::Certificate", "path": "Certificate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [263, 10], "end": [263, 15], "filename": "src/client/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/client/mod.rs:263`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b36900a6975ec74c0438dad"></a>
## from_der

`function` · `object_store::client::Certificate::from_der` · object_store 0.13.2

```rust
fn from_der(der: &[u8]) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::Certificate", "path": "Certificate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [268, 1], "end": [308, 2], "filename": "src/client/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/mod.rs:303`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create a `Certificate` from a binary DER encoded certificate.

<a id="op-1d3501d3f0b93f683973f84b"></a>
## from_pem

`function` · `object_store::client::Certificate::from_pem` · object_store 0.13.2

```rust
fn from_pem(pem: &[u8]) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::Certificate", "path": "Certificate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [268, 1], "end": [308, 2], "filename": "src/client/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/mod.rs:283`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create a `Certificate` from a PEM encoded certificate.

# Example from a PEM file

```no_run
# use object_store::Certificate;
# use std::fs::File;
# use std::io::Read;
let mut buf = Vec::new();
File::open("my_cert.pem").unwrap()
  .read_to_end(&mut buf).unwrap();
let cert = Certificate::from_pem(&buf).unwrap();

```

<a id="op-23f9abcb258738f7520fb711"></a>
## from_pem_bundle

`function` · `object_store::client::Certificate::from_pem_bundle` · object_store 0.13.2

```rust
fn from_pem_bundle(pem_bundle: &[u8]) -> Result<Vec<Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::Certificate", "path": "Certificate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [268, 1], "end": [308, 2], "filename": "src/client/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/client/mod.rs:294`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create a collection of `Certificate` from a PEM encoded certificate
bundle.

Files that contain such collections have extensions such as `.crt`,
`.cer` and `.pem` files.
