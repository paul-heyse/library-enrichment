# `object_store::aws::builder::AmazonS3ConfigKey`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.aws.builder.AmazonS3ConfigKey.json).

<a id="op-a19114f8f381c731db48f8f1"></a>
## AmazonS3ConfigKey

`enum` · `object_store::aws::builder::AmazonS3ConfigKey` · object_store 0.13.2

```rust
enum AmazonS3ConfigKey
```

Source: `src/aws/builder.rs:209`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Configuration keys for [`AmazonS3Builder`](../operations/object_store.aws.builder.AmazonS3Builder.md#op-b04708c1b9d591afbc23622a)

Configuration via keys can be done via [`AmazonS3Builder::with_config`](../operations/object_store.aws.builder.AmazonS3Builder.md#op-c2a08b36724ab755194640a5)

# Example
```
# use object_store::aws::{AmazonS3Builder, AmazonS3ConfigKey};
let builder = AmazonS3Builder::new()
    .with_config("aws_access_key_id".parse().unwrap(), "my-access-key-id")
    .with_config(AmazonS3ConfigKey::DefaultRegion, "my-default-region");
```

<a id="op-d1b6b800b625f5e711af405b"></a>
## AccessKeyId

`variant` · `object_store::aws::builder::AmazonS3ConfigKey::AccessKeyId` · object_store 0.13.2

```rust
AccessKeyId
```

Source: `src/aws/builder.rs:217`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

AWS Access Key

See [`AmazonS3Builder::with_access_key_id`](../operations/object_store.aws.builder.AmazonS3Builder.md#op-02c98b0227c993375c802bf6) for details.

Supported keys:
- `aws_access_key_id`
- `access_key_id`

<a id="op-9b7a34d9d2910e745e22aea1"></a>
## Bucket

`variant` · `object_store::aws::builder::AmazonS3ConfigKey::Bucket` · object_store 0.13.2

```rust
Bucket
```

Source: `src/aws/builder.rs:255`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Bucket name

See [`AmazonS3Builder::with_bucket_name`](../operations/object_store.aws.builder.AmazonS3Builder.md#op-ff7402b03a0ceaf2e892a01c) for details.

Supported keys:
- `aws_bucket`
- `aws_bucket_name`
- `bucket`
- `bucket_name`

<a id="op-1c2c963fac59b0268cd0c1f8"></a>
## Checksum

`variant` · `object_store::aws::builder::AmazonS3ConfigKey::Checksum` · object_store 0.13.2

```rust
Checksum
```

Source: `src/aws/builder.rs:317`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the checksum algorithm for this client

See [`AmazonS3Builder::with_checksum_algorithm`](../operations/object_store.aws.builder.AmazonS3Builder.md#op-4ac9f4459d18483c3b298e1b) for details.

<a id="op-fb29f66c9520210400b3d097"></a>
## Client

`variant` · `object_store::aws::builder::AmazonS3ConfigKey::Client` · object_store 0.13.2

```rust
Client
```

Source: `src/aws/builder.rs:447`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Client options

<a id="op-db596a928417674d5fe32a82"></a>
## ConditionalPut

`variant` · `object_store::aws::builder::AmazonS3ConfigKey::ConditionalPut` · object_store 0.13.2

```rust
ConditionalPut
```

Source: `src/aws/builder.rs:411`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Configure how to provide conditional put operations

See [`S3ConditionalPut`](../operations/object_store.aws.precondition.S3ConditionalPut.md#op-6d48bb95dd976ecea4dd3593) for details.

Supported keys:
- `aws_conditional_put`
- `conditional_put`

<a id="op-42715677697d9720f491b49d"></a>
## ContainerAuthorizationTokenFile

`variant` · `object_store::aws::builder::AmazonS3ConfigKey::ContainerAuthorizationTokenFile` · object_store 0.13.2

```rust
ContainerAuthorizationTokenFile
```

Source: `src/aws/builder.rs:359`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the authorization token in plain text when used in EKS to authenticate with ContainerCredentialsFullUri

<https://docs.aws.amazon.com/sdkref/latest/guide/feature-container-credentials.html>

Supported keys:
- `aws_container_authorization_token_file`
- `container_authorization_token_file`

Example: `/var/run/secrets/eks.amazonaws.com/serviceaccount/token`

<a id="op-4314408a23d1f8e3252b26ce"></a>
## ContainerCredentialsFullUri

`variant` · `object_store::aws::builder::AmazonS3ConfigKey::ContainerCredentialsFullUri` · object_store 0.13.2

```rust
ContainerCredentialsFullUri
```

Source: `src/aws/builder.rs:348`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the container credentials full URI when used in EKS

<https://docs.aws.amazon.com/sdkref/latest/guide/feature-container-credentials.html>

Supported keys:
- `aws_container_credentials_full_uri`
- `container_credentials_full_uri`

Example: `http://169.254.170.2/v2/credentials/abc123`

<a id="op-bb6d6f23e59fd98fa0084336"></a>
## ContainerCredentialsRelativeUri

`variant` · `object_store::aws::builder::AmazonS3ConfigKey::ContainerCredentialsRelativeUri` · object_store 0.13.2

```rust
ContainerCredentialsRelativeUri
```

Source: `src/aws/builder.rs:337`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the container credentials relative URI when used in ECS

<https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-iam-roles.html>

Supported keys:
- `aws_container_credentials_relative_uri`
- `container_credentials_relative_uri`

Example: `/v2/credentials/abc123`

<a id="op-b79ef5ca3b2c214074352b49"></a>
## CopyIfNotExists

`variant` · `object_store::aws::builder::AmazonS3ConfigKey::CopyIfNotExists` · object_store 0.13.2

```rust
CopyIfNotExists
```

Source: `src/aws/builder.rs:402`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Configure how to provide `copy_if_not_exists`

See [`S3CopyIfNotExists`](../operations/object_store.aws.precondition.S3CopyIfNotExists.md#op-2c842b27294b70ddd3d5ebd4) for details.

Supported keys:
- `aws_copy_if_not_exists`
- `copy_if_not_exists`

<a id="op-5b58f7ae51cc03f9e1b0f515"></a>
## DefaultRegion

`variant` · `object_store::aws::builder::AmazonS3ConfigKey::DefaultRegion` · object_store 0.13.2

```rust
DefaultRegion
```

Source: `src/aws/builder.rs:244`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Default region

See [`AmazonS3Builder::with_region`](../operations/object_store.aws.builder.AmazonS3Builder.md#op-dc32b02ead3a1e72e007ec7a) for details.

Supported keys:
- `aws_default_region`
- `default_region`

<a id="op-a849da018e112c5991dfddca"></a>
## DisableTagging

`variant` · `object_store::aws::builder::AmazonS3ConfigKey::DisableTagging` · object_store 0.13.2

```rust
DisableTagging
```

Source: `src/aws/builder.rs:430`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Disable tagging objects

If set to `true` will ignore any tags provided to [`put_opts`](crate::ObjectStore::put_opts).
This can be desirable if not supported by the backing store

Supported keys:
- `aws_disable_tagging`
- `disable_tagging`

<a id="op-daafdcc6e11b568553f263f0"></a>
## Encryption

`variant` · `object_store::aws::builder::AmazonS3ConfigKey::Encryption` · object_store 0.13.2

```rust
Encryption
```

Source: `src/aws/builder.rs:450`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Encryption options

<a id="op-9d9504599ef3439ee9e9918c"></a>
## Endpoint

`variant` · `object_store::aws::builder::AmazonS3ConfigKey::Endpoint` · object_store 0.13.2

```rust
Endpoint
```

Source: `src/aws/builder.rs:266`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets custom endpoint for communicating with AWS S3.

See [`AmazonS3Builder::with_endpoint`](../operations/object_store.aws.builder.AmazonS3Builder.md#op-368e550da46c755d9cf950d4) for details.

Supported keys:
- `aws_endpoint`
- `aws_endpoint_url`
- `endpoint`
- `endpoint_url`

<a id="op-661d3aeb678a5fa44b866fdf"></a>
## Err

`assoc_type` · `object_store::aws::builder::AmazonS3ConfigKey::Err` · object_store 0.13.2

```rust
Err
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3ConfigKey", "path": "AmazonS3ConfigKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [488, 1], "end": [549, 2], "filename": "src/aws/builder.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/aws/builder.rs:489`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e077ce8d0aabc4b736ee29d"></a>
## ImdsV1Fallback

`variant` · `object_store::aws::builder::AmazonS3ConfigKey::ImdsV1Fallback` · object_store 0.13.2

```rust
ImdsV1Fallback
```

Source: `src/aws/builder.rs:294`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Fall back to ImdsV1

See [`AmazonS3Builder::with_imdsv1_fallback`](../operations/object_store.aws.builder.AmazonS3Builder.md#op-c9327055692e29470f50caf4) for details.

Supported keys:
- `aws_imdsv1_fallback`
- `imdsv1_fallback`

<a id="op-bea98946f822ab40992d4f22"></a>
## MetadataEndpoint

`variant` · `object_store::aws::builder::AmazonS3ConfigKey::MetadataEndpoint` · object_store 0.13.2

```rust
MetadataEndpoint
```

Source: `src/aws/builder.rs:326`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the instance metadata endpoint

See [`AmazonS3Builder::with_metadata_endpoint`](../operations/object_store.aws.builder.AmazonS3Builder.md#op-25a0ccdde38ee95062227265) for details.

Supported keys:
- `aws_metadata_endpoint`
- `metadata_endpoint`

<a id="op-6602a088dfee41f388a821d1"></a>
## Region

`variant` · `object_store::aws::builder::AmazonS3ConfigKey::Region` · object_store 0.13.2

```rust
Region
```

Source: `src/aws/builder.rs:235`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Region

See [`AmazonS3Builder::with_region`](../operations/object_store.aws.builder.AmazonS3Builder.md#op-dc32b02ead3a1e72e007ec7a) for details.

Supported keys:
- `aws_region`
- `region`

<a id="op-a680a363214c75a1ef0cf460"></a>
## RequestPayer

`variant` · `object_store::aws::builder::AmazonS3ConfigKey::RequestPayer` · object_store 0.13.2

```rust
RequestPayer
```

Source: `src/aws/builder.rs:444`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Enable Support for S3 Requester Pays

Supported keys:
- `aws_request_payer`
- `request_payer`

<a id="op-646c713f437850bfebb9a139"></a>
## RoleArn

`variant` · `object_store::aws::builder::AmazonS3ConfigKey::RoleArn` · object_store 0.13.2

```rust
RoleArn
```

Source: `src/aws/builder.rs:377`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Role ARN to assume when using web identity token

Supported keys:
- `aws_role_arn`
- `role_arn`

Example: `arn:aws:iam::123456789012:role/MyWebIdentityRole`

<a id="op-d39057fda7e3136e331537d5"></a>
## RoleSessionName

`variant` · `object_store::aws::builder::AmazonS3ConfigKey::RoleSessionName` · object_store 0.13.2

```rust
RoleSessionName
```

Source: `src/aws/builder.rs:384`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Session name for web identity role assumption

Supported keys:
- `aws_role_session_name`
- `role_session_name`

<a id="op-23b6ec6edc87741113349ad6"></a>
## S3Endpoint

`variant` · `object_store::aws::builder::AmazonS3ConfigKey::S3Endpoint` · object_store 0.13.2

```rust
S3Endpoint
```

Source: `src/aws/builder.rs:274`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Service-specific S3 endpoint URL

When set, takes precedence over [`Endpoint`](Self::Endpoint) in the build method.

Supported keys:
- `aws_endpoint_url_s3`

<a id="op-3b0479345428e66bb734e508"></a>
## S3Express

`variant` · `object_store::aws::builder::AmazonS3ConfigKey::S3Express` · object_store 0.13.2

```rust
S3Express
```

Source: `src/aws/builder.rs:437`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Enable Support for S3 Express One Zone

Supported keys:
- `aws_s3_express`
- `s3_express`

<a id="op-cd66d4410c46cb4495aef461"></a>
## SecretAccessKey

`variant` · `object_store::aws::builder::AmazonS3ConfigKey::SecretAccessKey` · object_store 0.13.2

```rust
SecretAccessKey
```

Source: `src/aws/builder.rs:226`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Secret Access Key

See [`AmazonS3Builder::with_secret_access_key`](../operations/object_store.aws.builder.AmazonS3Builder.md#op-d2a8fded5569d00abeef3611) for details.

Supported keys:
- `aws_secret_access_key`
- `secret_access_key`

<a id="op-642e51ee95620d825362788d"></a>
## SkipSignature

`variant` · `object_store::aws::builder::AmazonS3ConfigKey::SkipSignature` · object_store 0.13.2

```rust
SkipSignature
```

Source: `src/aws/builder.rs:420`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Skip signing request

See [`AmazonS3Builder::with_skip_signature`](../operations/object_store.aws.builder.AmazonS3Builder.md#op-876e219298cbdc8423ad2d5e) for details.

Supported keys:
- `aws_skip_signature`
- `skip_signature`

<a id="op-f644244ec0adf0f86dde5144"></a>
## StsEndpoint

`variant` · `object_store::aws::builder::AmazonS3ConfigKey::StsEndpoint` · object_store 0.13.2

```rust
StsEndpoint
```

Source: `src/aws/builder.rs:393`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Custom STS endpoint for web identity token exchange

Defaults to `https://sts.{region}.amazonaws.com`

Supported keys:
- `aws_endpoint_url_sts`
- `endpoint_url_sts`

<a id="op-ecce29d6fa6529aca6bb7f20"></a>
## Token

`variant` · `object_store::aws::builder::AmazonS3ConfigKey::Token` · object_store 0.13.2

```rust
Token
```

Source: `src/aws/builder.rs:285`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Token to use for requests (passed to underlying provider)

See [`AmazonS3Builder::with_token`](../operations/object_store.aws.builder.AmazonS3Builder.md#op-20b915c0bf34f597015e874f) for details.

Supported keys:
- `aws_session_token`
- `aws_token`
- `session_token`
- `token`

<a id="op-7c255bbef2d02125c6c8db6e"></a>
## UnsignedPayload

`variant` · `object_store::aws::builder::AmazonS3ConfigKey::UnsignedPayload` · object_store 0.13.2

```rust
UnsignedPayload
```

Source: `src/aws/builder.rs:312`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Avoid computing payload checksum when calculating signature.

See [`AmazonS3Builder::with_unsigned_payload`](../operations/object_store.aws.builder.AmazonS3Builder.md#op-0b44f70c7dc07bfc147e57d9) for details.

Supported keys:
- `aws_unsigned_payload`
- `unsigned_payload`

<a id="op-dba6298d55d99cc067240271"></a>
## VirtualHostedStyleRequest

`variant` · `object_store::aws::builder::AmazonS3ConfigKey::VirtualHostedStyleRequest` · object_store 0.13.2

```rust
VirtualHostedStyleRequest
```

Source: `src/aws/builder.rs:303`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

If virtual hosted style request has to be used

See [`AmazonS3Builder::with_virtual_hosted_style_request`](../operations/object_store.aws.builder.AmazonS3Builder.md#op-263a0d8e3c8c9c6068303681) for details.

Supported keys:
- `aws_virtual_hosted_style_request`
- `virtual_hosted_style_request`

<a id="op-72143d6e0370e84dd535adc5"></a>
## WebIdentityTokenFile

`variant` · `object_store::aws::builder::AmazonS3ConfigKey::WebIdentityTokenFile` · object_store 0.13.2

```rust
WebIdentityTokenFile
```

Source: `src/aws/builder.rs:368`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Web identity token file path for AssumeRoleWithWebIdentity

Supported keys:
- `aws_web_identity_token_file`
- `web_identity_token_file`

Example: `/var/run/secrets/eks.amazonaws.com/serviceaccount/token`

<a id="op-2a829bd34aa35f8ff5665ad5"></a>
## as_ref

`function` · `object_store::aws::builder::AmazonS3ConfigKey::as_ref` · object_store 0.13.2

```rust
fn as_ref(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3ConfigKey", "path": "AmazonS3ConfigKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [453, 1], "end": [486, 2], "filename": "src/aws/builder.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "str"}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}, "trait_path": "core::convert::AsRef"}`

Source: `src/aws/builder.rs:454`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4421587aa9f65bdc7ee5eeb2"></a>
## clone

`function` · `object_store::aws::builder::AmazonS3ConfigKey::clone` · object_store 0.13.2

```rust
fn clone(&self) -> AmazonS3ConfigKey
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3ConfigKey", "path": "AmazonS3ConfigKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [207, 31], "end": [207, 36], "filename": "src/aws/builder.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/aws/builder.rs:207`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-42181c28818b2a2ddfb79f90"></a>
## deserialize

`function` · `object_store::aws::builder::AmazonS3ConfigKey::deserialize` · object_store 0.13.2

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3ConfigKey", "path": "AmazonS3ConfigKey"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [207, 62], "end": [207, 73], "filename": "src/aws/builder.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/aws/builder.rs:207`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-40984a46901861173da44264"></a>
## eq

`function` · `object_store::aws::builder::AmazonS3ConfigKey::eq` · object_store 0.13.2

```rust
fn eq(&self, other: &AmazonS3ConfigKey) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3ConfigKey", "path": "AmazonS3ConfigKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [207, 10], "end": [207, 19], "filename": "src/aws/builder.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/aws/builder.rs:207`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a028caf0cf2c252d1aaceb1"></a>
## fmt

`function` · `object_store::aws::builder::AmazonS3ConfigKey::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3ConfigKey", "path": "AmazonS3ConfigKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [207, 38], "end": [207, 43], "filename": "src/aws/builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/aws/builder.rs:207`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4a52e4a05e5e77f09c8677b"></a>
## from_str

`function` · `object_store::aws::builder::AmazonS3ConfigKey::from_str` · object_store 0.13.2

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3ConfigKey", "path": "AmazonS3ConfigKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [488, 1], "end": [549, 2], "filename": "src/aws/builder.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/aws/builder.rs:491`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-836fc11b50f5289ecebb58a0"></a>
## hash

`function` · `object_store::aws::builder::AmazonS3ConfigKey::hash` · object_store 0.13.2

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3ConfigKey", "path": "AmazonS3ConfigKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [207, 25], "end": [207, 29], "filename": "src/aws/builder.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/aws/builder.rs:207`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2a917983f1134562a62e036"></a>
## serialize

`function` · `object_store::aws::builder::AmazonS3ConfigKey::serialize` · object_store 0.13.2

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3ConfigKey", "path": "AmazonS3ConfigKey"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [207, 51], "end": [207, 60], "filename": "src/aws/builder.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/aws/builder.rs:207`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.
