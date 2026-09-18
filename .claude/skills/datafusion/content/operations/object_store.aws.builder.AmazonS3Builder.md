# `object_store::aws::builder::AmazonS3Builder`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.aws.builder.AmazonS3Builder.json).

<a id="op-b04708c1b9d591afbc23622a"></a>
## AmazonS3Builder

`struct` · `object_store::aws::builder::AmazonS3Builder` · object_store 0.13.2

```rust
struct AmazonS3Builder
```

Source: `src/aws/builder.rs:127`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Configure a connection to Amazon S3 using the specified credentials in
the specified Amazon region and bucket.

# Example
```
# let REGION = "foo";
# let BUCKET_NAME = "foo";
# let ACCESS_KEY_ID = "foo";
# let SECRET_KEY = "foo";
# use object_store::aws::AmazonS3Builder;
let s3 = AmazonS3Builder::new()
 .with_region(REGION)
 .with_bucket_name(BUCKET_NAME)
 .with_access_key_id(ACCESS_KEY_ID)
 .with_secret_access_key(SECRET_KEY)
 .build();
```

<a id="op-cd798597042e73b8bac8cf94"></a>
## build

`function` · `object_store::aws::builder::AmazonS3Builder::build` · object_store 0.13.2

```rust
fn build(self) -> Result<AmazonS3>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [1258, 2], "filename": "src/aws/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/builder.rs:1078`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create a [`AmazonS3`](../operations/object_store.aws.AmazonS3.md#op-792ec7caebfaa6563754cd2e) instance from the provided values,
consuming `self`.

<a id="op-ff8572bba87a0f34fd4c699b"></a>
## clone

`function` · `object_store::aws::builder::AmazonS3Builder::clone` · object_store 0.13.2

```rust
fn clone(&self) -> AmazonS3Builder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [126, 26], "end": [126, 31], "filename": "src/aws/builder.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/aws/builder.rs:126`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46b89e132055cc1085bc9f4f"></a>
## default

`function` · `object_store::aws::builder::AmazonS3Builder::default` · object_store 0.13.2

```rust
fn default() -> AmazonS3Builder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [126, 17], "end": [126, 24], "filename": "src/aws/builder.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/aws/builder.rs:126`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06327a19649f83f90d133279"></a>
## fmt

`function` · `object_store::aws::builder::AmazonS3Builder::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [126, 10], "end": [126, 15], "filename": "src/aws/builder.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/aws/builder.rs:126`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-16b2363c7229dc2cbc2e5c50"></a>
## from_env

`function` · `object_store::aws::builder::AmazonS3Builder::from_env` · object_store 0.13.2

```rust
fn from_env() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [1258, 2], "filename": "src/aws/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/builder.rs:587`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Fill the [`AmazonS3Builder`](../operations/object_store.aws.builder.AmazonS3Builder.md#op-b04708c1b9d591afbc23622a) with regular AWS environment variables

All environment variables starting with `AWS_` will be evaluated.
Names must match acceptable input to [`AmazonS3ConfigKey::from_str`](../operations/object_store.aws.builder.AmazonS3ConfigKey.md#op-c4a52e4a05e5e77f09c8677b).

Some examples of variables extracted from environment:
* `AWS_ACCESS_KEY_ID` -> access_key_id
* `AWS_SECRET_ACCESS_KEY` -> secret_access_key
* `AWS_DEFAULT_REGION` -> region
* `AWS_ENDPOINT` -> endpoint
* `AWS_ENDPOINT_URL_S3` -> s3_endpoint (takes precedence over endpoint in build)
* `AWS_SESSION_TOKEN` -> token
* `AWS_WEB_IDENTITY_TOKEN_FILE` -> path to file containing web identity token for AssumeRoleWithWebIdentity
* `AWS_ROLE_ARN` -> ARN of the role to assume when using web identity token
* `AWS_ROLE_SESSION_NAME` -> optional session name for web identity role assumption (defaults to "WebIdentitySession")
* `AWS_ENDPOINT_URL_STS` -> optional custom STS endpoint for web identity token exchange (defaults to "https://sts.{region}.amazonaws.com")
* `AWS_CONTAINER_CREDENTIALS_RELATIVE_URI` -> <https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-iam-roles.html>
* `AWS_CONTAINER_CREDENTIALS_FULL_URI` -> <https://docs.aws.amazon.com/sdkref/latest/guide/feature-container-credentials.html>
* `AWS_CONTAINER_AUTHORIZATION_TOKEN_FILE` -> <https://docs.aws.amazon.com/sdkref/latest/guide/feature-container-credentials.html>
* `AWS_ALLOW_HTTP` -> set to "true" to permit HTTP connections without TLS
* `AWS_REQUEST_PAYER` -> set to "true" to permit operations on requester-pays buckets.

# Example
```
use object_store::aws::AmazonS3Builder;

let s3 = AmazonS3Builder::from_env()
    .with_bucket_name("foo")
    .build();
```

<a id="op-60e0cf5f915085416d610715"></a>
## get_config_value

`function` · `object_store::aws::builder::AmazonS3Builder::get_config_value` · object_store 0.13.2

```rust
fn get_config_value(&self, key: &AmazonS3ConfigKey) -> Option<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [1258, 2], "filename": "src/aws/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/builder.rs:711`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Get config value via a [`AmazonS3ConfigKey`](../operations/object_store.aws.builder.AmazonS3ConfigKey.md#op-a19114f8f381c731db48f8f1).

# Example
```
use object_store::aws::{AmazonS3Builder, AmazonS3ConfigKey};

let builder = AmazonS3Builder::from_env()
    .with_bucket_name("foo");
let bucket_name = builder.get_config_value(&AmazonS3ConfigKey::Bucket).unwrap_or_default();
assert_eq!("foo", &bucket_name);
```

<a id="op-98c0ec55207d36561795cfe7"></a>
## new

`function` · `object_store::aws::builder::AmazonS3Builder::new` · object_store 0.13.2

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [1258, 2], "filename": "src/aws/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/builder.rs:553`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Create a new [`AmazonS3Builder`](../operations/object_store.aws.builder.AmazonS3Builder.md#op-b04708c1b9d591afbc23622a) with default values.

<a id="op-02c98b0227c993375c802bf6"></a>
## with_access_key_id

`function` · `object_store::aws::builder::AmazonS3Builder::with_access_key_id` · object_store 0.13.2

```rust
fn with_access_key_id(self, access_key_id: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [1258, 2], "filename": "src/aws/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/builder.rs:823`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the AWS Access Key

Examples: `AKIAIOSFODNN7EXAMPLE`, `ASIA4ZP5EXAMPLETOKEN`

<a id="op-3f41c110a139041cc6d04155"></a>
## with_allow_http

`function` · `object_store::aws::builder::AmazonS3Builder::with_allow_http` · object_store 0.13.2

```rust
fn with_allow_http(self, allow_http: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [1258, 2], "filename": "src/aws/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/builder.rs:893`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets what protocol is allowed.

If `allow_http` is :
* false (default):  Only HTTPS are allowed
* true:  HTTP and HTTPS are allowed

<div class="warning">

**Warning**

If you enable this option, attackers may be able to read the data you request.

</div>

<a id="op-71a55392f68489c46c56e290"></a>
## with_bucket_key

`function` · `object_store::aws::builder::AmazonS3Builder::with_bucket_key` · object_store 0.13.2

```rust
fn with_bucket_key(self, enabled: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [1258, 2], "filename": "src/aws/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/builder.rs:1055`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set whether to enable bucket key for server side encryption. This overrides
the bucket default setting for bucket keys.

When bucket keys are disabled, each object is encrypted with a unique data key.
When bucket keys are enabled, a single data key is used for the entire bucket,
reducing overhead of encryption.

<a id="op-ff7402b03a0ceaf2e892a01c"></a>
## with_bucket_name

`function` · `object_store::aws::builder::AmazonS3Builder::with_bucket_name` · object_store 0.13.2

```rust
fn with_bucket_name(self, bucket_name: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [1258, 2], "filename": "src/aws/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/builder.rs:849`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the bucket_name (required)

<a id="op-4ac9f4459d18483c3b298e1b"></a>
## with_checksum_algorithm

`function` · `object_store::aws::builder::AmazonS3Builder::with_checksum_algorithm` · object_store 0.13.2

```rust
fn with_checksum_algorithm(self, checksum_algorithm: Checksum) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [1258, 2], "filename": "src/aws/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/builder.rs:962`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets the [checksum algorithm] which has to be used for object integrity check during upload.

[checksum algorithm]: https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html

<a id="op-2a11b019f2c2e8a31b62b45a"></a>
## with_client_options

`function` · `object_store::aws::builder::AmazonS3Builder::with_client_options` · object_store 0.13.2

```rust
fn with_client_options(self, options: ClientOptions) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [1258, 2], "filename": "src/aws/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/builder.rs:999`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets the client options, overriding any already set

<a id="op-70e6b3943a3301a9b1c69262"></a>
## with_conditional_put

`function` · `object_store::aws::builder::AmazonS3Builder::with_conditional_put` · object_store 0.13.2

```rust
fn with_conditional_put(self, config: S3ConditionalPut) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [1258, 2], "filename": "src/aws/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/builder.rs:1012`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Configure how to provide conditional put operations.
if not set, the default value will be `S3ConditionalPut::ETagMatch`

<a id="op-c2a08b36724ab755194640a5"></a>
## with_config

`function` · `object_store::aws::builder::AmazonS3Builder::with_config` · object_store 0.13.2

```rust
fn with_config(self, key: AmazonS3ConfigKey, value: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [1258, 2], "filename": "src/aws/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/builder.rs:627`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set an option on the builder via a key - value pair.

<a id="op-c843b4367818823330531bcf"></a>
## with_copy_if_not_exists

`function` · `object_store::aws::builder::AmazonS3Builder::with_copy_if_not_exists` · object_store 0.13.2

```rust
fn with_copy_if_not_exists(self, config: S3CopyIfNotExists) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [1258, 2], "filename": "src/aws/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/builder.rs:1005`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Configure how to provide `copy_if_not_exists`

<a id="op-bcd3cb4f5e13f713f949613f"></a>
## with_credentials

`function` · `object_store::aws::builder::AmazonS3Builder::with_credentials` · object_store 0.13.2

```rust
fn with_credentials(self, credentials: AwsCredentialProvider) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [1258, 2], "filename": "src/aws/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/builder.rs:875`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the credential provider overriding any other options

<a id="op-30f532ec0647fef42be1a1c1"></a>
## with_disable_tagging

`function` · `object_store::aws::builder::AmazonS3Builder::with_disable_tagging` · object_store 0.13.2

```rust
fn with_disable_tagging(self, ignore: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [1258, 2], "filename": "src/aws/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/builder.rs:1018`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

If set to `true` will ignore any tags provided to [`put_opts`](crate::ObjectStore::put_opts)

<a id="op-956f88ce1bf9410029ce1dd1"></a>
## with_dsse_kms_encryption

`function` · `object_store::aws::builder::AmazonS3Builder::with_dsse_kms_encryption` · object_store 0.13.2

```rust
fn with_dsse_kms_encryption(self, kms_key_id: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [1258, 2], "filename": "src/aws/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/builder.rs:1033`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Use dual server side encryption for server side encryption.

<a id="op-368e550da46c755d9cf950d4"></a>
## with_endpoint

`function` · `object_store::aws::builder::AmazonS3Builder::with_endpoint` · object_store 0.13.2

```rust
fn with_endpoint(self, endpoint: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [1258, 2], "filename": "src/aws/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/builder.rs:869`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets the endpoint for communicating with AWS S3.

Defaults to the [region endpoint]. See  [`Self::with_region`](../operations/object_store.aws.builder.AmazonS3Builder.md#op-dc32b02ead3a1e72e007ec7a) for further details.

For example, this might be set to `"http://localhost:4566:`
for testing against a localstack instance.

The `endpoint` field should be consistent with [`Self::with_virtual_hosted_style_request`](../operations/object_store.aws.builder.AmazonS3Builder.md#op-263a0d8e3c8c9c6068303681).
I.e. if `virtual_hosted_style_request` is set to true then `endpoint`
should have the bucket name included.

By default, only HTTPS schemes are enabled.
To connect to an HTTP endpoint, enable [`Self::with_allow_http`](../operations/object_store.aws.builder.AmazonS3Builder.md#op-3f41c110a139041cc6d04155).

[region endpoint]: https://docs.aws.amazon.com/general/latest/gr/s3.html

<a id="op-ccbb352f4c00f108a99cd24e"></a>
## with_http_connector

`function` · `object_store::aws::builder::AmazonS3Builder::with_http_connector` · object_store 0.13.2

```rust
fn with_http_connector<C: HttpConnector>(self, connector: C) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [1258, 2], "filename": "src/aws/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/builder.rs:1071`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The [`HttpConnector`](../operations/object_store.client.http.connection.HttpConnector.md#op-04b72af01cc3e9636d12c25b) to use

On non-WASM32 platforms uses [`reqwest`] by default, on WASM32 platforms must be provided

Unresolved upstream links (retained, not inferred): ``reqwest``.

<a id="op-c9327055692e29470f50caf4"></a>
## with_imdsv1_fallback

`function` · `object_store::aws::builder::AmazonS3Builder::with_imdsv1_fallback` · object_store 0.13.2

```rust
fn with_imdsv1_fallback(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [1258, 2], "filename": "src/aws/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/builder.rs:936`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

By default instance credentials will only be fetched over [IMDSv2], as AWS recommends
against having IMDSv1 enabled on EC2 instances as it is vulnerable to [SSRF attack]

However, certain deployment environments, such as those running old versions of kube2iam,
may not support IMDSv2. This option will enable automatic fallback to using IMDSv1
if the token endpoint returns a 403 error indicating that IMDSv2 is not supported.

This option has no effect if not using instance credentials

[IMDSv2]: https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/configuring-instance-metadata-service.html
[SSRF attack]: https://aws.amazon.com/blogs/security/defense-in-depth-open-firewalls-reverse-proxies-ssrf-vulnerabilities-ec2-instance-metadata-service/

<a id="op-25a0ccdde38ee95062227265"></a>
## with_metadata_endpoint

`function` · `object_store::aws::builder::AmazonS3Builder::with_metadata_endpoint` · object_store 0.13.2

```rust
fn with_metadata_endpoint(self, endpoint: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [1258, 2], "filename": "src/aws/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/builder.rs:973`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the [instance metadata endpoint](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-metadata.html),
used primarily within AWS EC2.

This defaults to the IPv4 endpoint: http://169.254.169.254.
One can alternatively use the IPv6 endpoint http://fd00:ec2::254.

<a id="op-78f4d116bda49609cdf932c8"></a>
## with_proxy_ca_certificate

`function` · `object_store::aws::builder::AmazonS3Builder::with_proxy_ca_certificate` · object_store 0.13.2

```rust
fn with_proxy_ca_certificate(self, proxy_ca_certificate: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [1258, 2], "filename": "src/aws/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/builder.rs:985`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set a trusted proxy CA certificate

<a id="op-07ec57a6409f16cdb5e3ab5a"></a>
## with_proxy_excludes

`function` · `object_store::aws::builder::AmazonS3Builder::with_proxy_excludes` · object_store 0.13.2

```rust
fn with_proxy_excludes(self, proxy_excludes: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [1258, 2], "filename": "src/aws/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/builder.rs:993`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set a list of hosts to exclude from proxy connections

<a id="op-260d43b7d6a31cd8a956d75e"></a>
## with_proxy_url

`function` · `object_store::aws::builder::AmazonS3Builder::with_proxy_url` · object_store 0.13.2

```rust
fn with_proxy_url(self, proxy_url: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [1258, 2], "filename": "src/aws/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/builder.rs:979`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the proxy_url to be used by the underlying client

<a id="op-dc32b02ead3a1e72e007ec7a"></a>
## with_region

`function` · `object_store::aws::builder::AmazonS3Builder::with_region` · object_store 0.13.2

```rust
fn with_region(self, region: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [1258, 2], "filename": "src/aws/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/builder.rs:843`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the region, defaults to `us-east-1`

<a id="op-0435f4b3eafd790a19b9a422"></a>
## with_request_payer

`function` · `object_store::aws::builder::AmazonS3Builder::with_request_payer` · object_store 0.13.2

```rust
fn with_request_payer(self, enabled: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [1258, 2], "filename": "src/aws/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/builder.rs:1063`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set whether to charge requester for bucket operations.

<https://docs.aws.amazon.com/AmazonS3/latest/userguide/RequesterPaysBuckets.html>

<a id="op-cd9cdbc2928a3d4d357827ea"></a>
## with_retry

`function` · `object_store::aws::builder::AmazonS3Builder::with_retry` · object_store 0.13.2

```rust
fn with_retry(self, retry_config: RetryConfig) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [1258, 2], "filename": "src/aws/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/builder.rs:920`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the retry configuration

<a id="op-aed9fc7774d4e5de6fe2bdaa"></a>
## with_s3_express

`function` · `object_store::aws::builder::AmazonS3Builder::with_s3_express` · object_store 0.13.2

```rust
fn with_s3_express(self, s3_express: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [1258, 2], "filename": "src/aws/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/builder.rs:914`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Configure this as an S3 Express One Zone Bucket

<a id="op-d2a8fded5569d00abeef3611"></a>
## with_secret_access_key

`function` · `object_store::aws::builder::AmazonS3Builder::with_secret_access_key` · object_store 0.13.2

```rust
fn with_secret_access_key(self, secret_access_key: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [1258, 2], "filename": "src/aws/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/builder.rs:829`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the AWS Secret Access Key

<a id="op-876e219298cbdc8423ad2d5e"></a>
## with_skip_signature

`function` · `object_store::aws::builder::AmazonS3Builder::with_skip_signature` · object_store 0.13.2

```rust
fn with_skip_signature(self, skip_signature: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [1258, 2], "filename": "src/aws/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/builder.rs:954`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

If enabled, [`AmazonS3`](../operations/object_store.aws.AmazonS3.md#op-792ec7caebfaa6563754cd2e) will not fetch credentials and will not sign requests

This can be useful when interacting with public S3 buckets that deny authorized requests

<a id="op-5dc9e800276a2726f94faaec"></a>
## with_sse_kms_encryption

`function` · `object_store::aws::builder::AmazonS3Builder::with_sse_kms_encryption` · object_store 0.13.2

```rust
fn with_sse_kms_encryption(self, kms_key_id: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [1258, 2], "filename": "src/aws/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/builder.rs:1024`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Use SSE-KMS for server side encryption.

<a id="op-208379977ed5c4a1dc0592ba"></a>
## with_ssec_encryption

`function` · `object_store::aws::builder::AmazonS3Builder::with_ssec_encryption` · object_store 0.13.2

```rust
fn with_ssec_encryption(self, customer_key_base64: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [1258, 2], "filename": "src/aws/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/builder.rs:1043`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Use SSE-C for server side encryption.
Must pass the *base64-encoded* 256-bit customer encryption key.

<a id="op-20b915c0bf34f597015e874f"></a>
## with_token

`function` · `object_store::aws::builder::AmazonS3Builder::with_token` · object_store 0.13.2

```rust
fn with_token(self, token: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [1258, 2], "filename": "src/aws/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/builder.rs:837`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Set the AWS Session Token to use for requests

Should not be used in combination with [`Self::with_allow_http`](../operations/object_store.aws.builder.AmazonS3Builder.md#op-3f41c110a139041cc6d04155).

<a id="op-0b44f70c7dc07bfc147e57d9"></a>
## with_unsigned_payload

`function` · `object_store::aws::builder::AmazonS3Builder::with_unsigned_payload` · object_store 0.13.2

```rust
fn with_unsigned_payload(self, unsigned_payload: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [1258, 2], "filename": "src/aws/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/builder.rs:946`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets if unsigned payload option has to be used.

See [unsigned payload option](https://docs.aws.amazon.com/AmazonS3/latest/API/sig-v4-header-based-auth.html)
* false (default): Signed payload option is used, where the checksum for the request body is computed and included when constructing a canonical request.
* true: Unsigned payload option is used. `UNSIGNED-PAYLOAD` literal is included when constructing a canonical request,

<a id="op-9cef1f09d6c62ccb4b58f962"></a>
## with_url

`function` · `object_store::aws::builder::AmazonS3Builder::with_url` · object_store 0.13.2

```rust
fn with_url(self, url: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [1258, 2], "filename": "src/aws/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/builder.rs:621`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Parse available connection info form a well-known storage URL.

The supported url schemes are:

- `s3://<bucket>/<path>`
- `s3a://<bucket>/<path>`
- `https://s3.<region>.amazonaws.com/<bucket>`
- `https://<bucket>.s3.<region>.amazonaws.com`
- `https://ACCOUNT_ID.r2.cloudflarestorage.com/bucket`

Note: Settings derived from the URL will override any others set on this builder

# Example
```
use object_store::aws::AmazonS3Builder;

let s3 = AmazonS3Builder::from_env()
    .with_url("s3://bucket/path")
    .build();
```

<a id="op-263a0d8e3c8c9c6068303681"></a>
## with_virtual_hosted_style_request

`function` · `object_store::aws::builder::AmazonS3Builder::with_virtual_hosted_style_request` · object_store 0.13.2

```rust
fn with_virtual_hosted_style_request(self, virtual_hosted_style_request: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::aws::builder::AmazonS3Builder", "path": "AmazonS3Builder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [551, 1], "end": [1258, 2], "filename": "src/aws/builder.rs"}, "trait": null, "trait_path": null}`

Source: `src/aws/builder.rs:908`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Sets if virtual hosted style request has to be used.

If `virtual_hosted_style_request` is:
* false (default):  Path style request is used
* true:  Virtual hosted style request is used

If the `endpoint` is provided then it should be
consistent with `virtual_hosted_style_request`.
I.e. if `virtual_hosted_style_request` is set to true
then `endpoint` should have bucket name included.
