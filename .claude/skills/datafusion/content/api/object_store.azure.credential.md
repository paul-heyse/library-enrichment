# `object_store::azure::credential`

Crate `object_store` · 4 public items · structured records in [`model/object_store.azure.credential.json`](../model/object_store.azure.credential.json)

## AzureCredential

`enum` · `object_store::azure::credential::AzureCredential`

Also reachable as `object_store::azure::AzureCredential`

```rust
enum AzureCredential
```

**Variants**: `AccessKey`, `SASToken`, `BearerToken`

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn sensitive_request(&self) -> bool
```

[Full member, field, variant and typed contracts](../operations/object_store.azure.credential.AzureCredential.md).


An Azure storage credential

---

## Error

`enum` · `object_store::azure::credential::Error`

```rust
enum Error
```

**Variants**: `TokenRequest`, `TokenResponseBody`, `FederatedTokenFile`, `InvalidAccessKey`, `AzureCli`, `AzureCliResponse`, `SASforSASNotSupported`

[Full member, field, variant and typed contracts](../operations/object_store.azure.credential.Error.md).


---

## AzureAccessKey

`struct` · `object_store::azure::credential::AzureAccessKey`

Also reachable as `object_store::azure::AzureAccessKey`

```rust
struct AzureAccessKey
```

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (1)

```rust
fn try_new(key: &str) -> std::result::Result<Self, Error>
```

[Full member, field, variant and typed contracts](../operations/object_store.azure.credential.AzureAccessKey.md).


A shared Azure Storage Account Key

---

## AzureAuthorizer

`struct` · `object_store::azure::credential::AzureAuthorizer`

Also reachable as `object_store::azure::AzureAuthorizer`

```rust
struct AzureAuthorizer<'a>
```

**Derives**: Debug

**Methods** (2)

```rust
fn authorize(&self, request: &mut HttpRequest)
fn new(credential: &'a AzureCredential, account: &'a str) -> Self
```

[Full member, field, variant and typed contracts](../operations/object_store.azure.credential.AzureAuthorizer.md).


Authorize a [`HttpRequest`] with an [`AzureAuthorizer`]

---
