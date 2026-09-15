# `object_store::aws::credential`

Crate `object_store` · 2 public items · structured records in [`model/object_store.aws.credential.json`](../model/object_store.aws.credential.json)

## AwsAuthorizer

`struct` · `object_store::aws::credential::AwsAuthorizer`

Also reachable as `object_store::aws::AwsAuthorizer`

```rust
struct AwsAuthorizer<'a>
```

**Derives**: Debug

**Methods** (4)

```rust
fn authorize(&self, request: &mut HttpRequest, pre_calculated_digest: Option<&[u8]>)
fn new(credential: &'a AwsCredential, service: &'a str, region: &'a str) -> Self
fn with_request_payer(self, request_payer: bool) -> Self
fn with_sign_payload(self, signed: bool) -> Self
```

Authorize a [`HttpRequest`] with an [`AwsCredential`] using [AWS SigV4]

[AWS SigV4]: https://docs.aws.amazon.com/general/latest/gr/sigv4-calculate-signature.html

---

## AwsCredential

`struct` · `object_store::aws::credential::AwsCredential`

Also reachable as `object_store::aws::AwsCredential`

```rust
struct AwsCredential
```

**Fields**: `key_id`, `secret_key`, `token`

**Derives**: Debug, Eq, PartialEq, StructuralPartialEq

A set of AWS security credentials

---
