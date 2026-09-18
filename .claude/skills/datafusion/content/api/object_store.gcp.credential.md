# `object_store::gcp::credential`

Crate `object_store` · 4 public items · structured records in [`model/object_store.gcp.credential.json`](../model/object_store.gcp.credential.json)

## Error

`enum` · `object_store::gcp::credential::Error`

```rust
enum Error
```

**Variants**: `OpenCredentials`, `DecodeCredentials`, `MissingKey`, `InvalidKey`, `Sign`, `Encode`, `UnsupportedKey`, `TokenRequest`, `TokenResponseBody`, `ReadPem`

[Full member, field, variant and typed contracts](../operations/object_store.gcp.credential.Error.md).


---

## GcpCredential

`struct` · `object_store::gcp::credential::GcpCredential`

Also reachable as `object_store::gcp::GcpCredential`

```rust
struct GcpCredential
```

**Fields**: `bearer`

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/object_store.gcp.credential.GcpCredential.md).


A Google Cloud Storage Credential

---

## GcpSigningCredential

`struct` · `object_store::gcp::credential::GcpSigningCredential`

Also reachable as `object_store::gcp::GcpSigningCredential`

```rust
struct GcpSigningCredential
```

**Fields**: `email`, `private_key`

**Derives**: Debug

[Full member, field, variant and typed contracts](../operations/object_store.gcp.credential.GcpSigningCredential.md).


A Google Cloud Storage Credential for signing

---

## ServiceAccountKey

`struct` · `object_store::gcp::credential::ServiceAccountKey`

Also reachable as `object_store::gcp::ServiceAccountKey`

```rust
struct ServiceAccountKey
```

**Derives**: Debug

**Methods** (3)

```rust
fn from_der(key: &[u8]) -> std::result::Result<Self, Error>
fn from_pem(encoded: &[u8]) -> std::result::Result<Self, Error>
fn from_pkcs8(key: &[u8]) -> std::result::Result<Self, Error>
```

[Full member, field, variant and typed contracts](../operations/object_store.gcp.credential.ServiceAccountKey.md).


A private RSA key for a service account

---
