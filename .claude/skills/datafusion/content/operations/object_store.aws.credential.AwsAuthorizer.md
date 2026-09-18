# `object_store::aws::credential::AwsAuthorizer`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.aws.credential.AwsAuthorizer.json).

<a id="op-34b526f0e27b05ac3396cea6"></a>
## AwsAuthorizer

`struct` · `object_store::aws::credential::AwsAuthorizer` · object_store 0.13.2

```rust
struct AwsAuthorizer<'a>
```

Source: `src/aws/credential.rs:108`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Authorize a [`HttpRequest`](../operations/object_store.client.http.body.HttpRequest.md#op-0f9bc5af637ad2947840b60b) with an [`AwsCredential`](../operations/object_store.aws.credential.AwsCredential.md#op-320bb29fbf19dc07e494390f) using [AWS SigV4]

[AWS SigV4]: https://docs.aws.amazon.com/general/latest/gr/sigv4-calculate-signature.html

<a id="op-7186b8c82a8273e5cd259813"></a>
## authorize

`function` · `object_store::aws::credential::AwsAuthorizer::authorize` · object_store 0.13.2

```rust
fn authorize(&self, request: &mut HttpRequest, pre_calculated_digest: Option<&[u8]>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "object_store::aws::credential::AwsAuthorizer", "path": "AwsAuthorizer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [125, 1], "end": [356, 2], "filename": "src/aws/credential.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/credential.rs:173`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Authorize `request` with an optional pre-calculated SHA256 digest by attaching
the relevant [AWS SigV4] headers

# Payload Signature

AWS SigV4 requests must contain the `x-amz-content-sha256` header, it is set as follows:

* If not configured to sign payloads, it is set to `UNSIGNED-PAYLOAD`
* If a `pre_calculated_digest` is provided, it is set to the hex encoding of it
* If it is a streaming request, it is set to `STREAMING-AWS4-HMAC-SHA256-PAYLOAD`
* Otherwise it is set to the hex encoded SHA256 of the request body

[AWS SigV4]: https://docs.aws.amazon.com/IAM/latest/UserGuide/create-signed-request.html

<a id="op-f5a122fc922d64cc7b2a51e5"></a>
## fmt

`function` · `object_store::aws::credential::AwsAuthorizer::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "object_store::aws::credential::AwsAuthorizer", "path": "AwsAuthorizer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 10], "end": [107, 15], "filename": "src/aws/credential.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/aws/credential.rs:107`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1128afa1e5c2a48ba49ef48"></a>
## new

`function` · `object_store::aws::credential::AwsAuthorizer::new` · object_store 0.13.2

```rust
fn new(credential: &'a AwsCredential, service: &'a str, region: &'a str) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "object_store::aws::credential::AwsAuthorizer", "path": "AwsAuthorizer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [125, 1], "end": [356, 2], "filename": "src/aws/credential.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/credential.rs:127`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create a new [`AwsAuthorizer`](../operations/object_store.aws.credential.AwsAuthorizer.md#op-34b526f0e27b05ac3396cea6)

<a id="op-f434702cb3b3f2108d522c6b"></a>
## with_request_payer

`function` · `object_store::aws::credential::AwsAuthorizer::with_request_payer` · object_store 0.13.2

```rust
fn with_request_payer(self, request_payer: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "object_store::aws::credential::AwsAuthorizer", "path": "AwsAuthorizer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [125, 1], "end": [356, 2], "filename": "src/aws/credential.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/credential.rs:155`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set whether to include requester pays headers

<https://docs.aws.amazon.com/AmazonS3/latest/userguide/ObjectsinRequesterPaysBuckets.html>

<a id="op-65ebbf882b46d1ab8dc08692"></a>
## with_sign_payload

`function` · `object_store::aws::credential::AwsAuthorizer::with_sign_payload` · object_store 0.13.2

```rust
fn with_sign_payload(self, signed: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "object_store::aws::credential::AwsAuthorizer", "path": "AwsAuthorizer"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [125, 1], "end": [356, 2], "filename": "src/aws/credential.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/credential.rs:141`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Controls whether this [`AwsAuthorizer`](../operations/object_store.aws.credential.AwsAuthorizer.md#op-34b526f0e27b05ac3396cea6) will attempt to sign the request payload,
the default is `true`
