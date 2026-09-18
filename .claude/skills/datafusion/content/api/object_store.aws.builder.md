# `object_store::aws::builder`

Crate `object_store` · 3 public items · structured records in [`model/object_store.aws.builder.json`](../model/object_store.aws.builder.json)

## AmazonS3ConfigKey

`enum` · `object_store::aws::builder::AmazonS3ConfigKey`

Also reachable as `object_store::aws::AmazonS3ConfigKey`

```rust
enum AmazonS3ConfigKey
```

**Variants**: `AccessKeyId`, `SecretAccessKey`, `Region`, `DefaultRegion`, `Bucket`, `Endpoint`, `S3Endpoint`, `Token`, `ImdsV1Fallback`, `VirtualHostedStyleRequest`, `UnsignedPayload`, `Checksum`, `MetadataEndpoint`, `ContainerCredentialsRelativeUri`, `ContainerCredentialsFullUri`, `ContainerAuthorizationTokenFile`, `WebIdentityTokenFile`, `RoleArn`, `RoleSessionName`, `StsEndpoint`, `CopyIfNotExists`, `ConditionalPut`, `SkipSignature`, `DisableTagging`, `S3Express`, `RequestPayer`, `Client`, `Encryption`

**Implements**: `core::convert::AsRef`, `core::str::traits::FromStr`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Copy, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**via `core::convert::AsRef`**

```rust
fn as_ref(&self) -> &str
```

**via `core::str::traits::FromStr`**

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Full member, field, variant and typed contracts](../operations/object_store.aws.builder.AmazonS3ConfigKey.md).


Configuration keys for [`AmazonS3Builder`]

Configuration via keys can be done via [`AmazonS3Builder::with_config`]

# Example
```
# use object_store::aws::{AmazonS3Builder, AmazonS3ConfigKey};
let builder = AmazonS3Builder::new()
    .with_config("aws_access_key_id".parse().unwrap(), "my-access-key-id")
    .with_config(AmazonS3ConfigKey::DefaultRegion, "my-default-region");
```

---

## S3EncryptionConfigKey

`enum` · `object_store::aws::builder::S3EncryptionConfigKey`

```rust
enum S3EncryptionConfigKey
```

**Variants**: `ServerSideEncryption`, `KmsKeyId`, `BucketKeyEnabled`, `CustomerEncryptionKey`

[Full member, field, variant and typed contracts](../operations/object_store.aws.builder.S3EncryptionConfigKey.md).


Encryption configuration options for S3.

These options are used to configure server-side encryption for S3 objects.
To configure them, pass them to [`AmazonS3Builder::with_config`].

[SSE-S3]: https://docs.aws.amazon.com/AmazonS3/latest/userguide/UsingServerSideEncryption.html
[SSE-KMS]: https://docs.aws.amazon.com/AmazonS3/latest/userguide/UsingKMSEncryption.html
[DSSE-KMS]: https://docs.aws.amazon.com/AmazonS3/latest/userguide/UsingDSSEncryption.html
[SSE-C]: https://docs.aws.amazon.com/AmazonS3/latest/userguide/ServerSideEncryptionCustomerKeys.html

---

## AmazonS3Builder

`struct` · `object_store::aws::builder::AmazonS3Builder`

Also reachable as `object_store::aws::AmazonS3Builder`

```rust
struct AmazonS3Builder
```

**Derives**: Clone, Debug, Default

**Methods** (35)

```rust
fn build(self) -> Result<AmazonS3>
fn from_env() -> Self
fn get_config_value(&self, key: &AmazonS3ConfigKey) -> Option<String>
fn new() -> Self
fn with_access_key_id(self, access_key_id: impl Into<String>) -> Self
fn with_allow_http(self, allow_http: bool) -> Self
fn with_bucket_key(self, enabled: bool) -> Self
fn with_bucket_name(self, bucket_name: impl Into<String>) -> Self
fn with_checksum_algorithm(self, checksum_algorithm: Checksum) -> Self
fn with_client_options(self, options: ClientOptions) -> Self
fn with_conditional_put(self, config: S3ConditionalPut) -> Self
fn with_config(self, key: AmazonS3ConfigKey, value: impl Into<String>) -> Self
fn with_copy_if_not_exists(self, config: S3CopyIfNotExists) -> Self
fn with_credentials(self, credentials: AwsCredentialProvider) -> Self
fn with_disable_tagging(self, ignore: bool) -> Self
fn with_dsse_kms_encryption(self, kms_key_id: impl Into<String>) -> Self
fn with_endpoint(self, endpoint: impl Into<String>) -> Self
fn with_http_connector<C: HttpConnector>(self, connector: C) -> Self
fn with_imdsv1_fallback(self) -> Self
fn with_metadata_endpoint(self, endpoint: impl Into<String>) -> Self
fn with_proxy_ca_certificate(self, proxy_ca_certificate: impl Into<String>) -> Self
fn with_proxy_excludes(self, proxy_excludes: impl Into<String>) -> Self
fn with_proxy_url(self, proxy_url: impl Into<String>) -> Self
fn with_region(self, region: impl Into<String>) -> Self
fn with_request_payer(self, enabled: bool) -> Self
fn with_retry(self, retry_config: RetryConfig) -> Self
fn with_s3_express(self, s3_express: bool) -> Self
fn with_secret_access_key(self, secret_access_key: impl Into<String>) -> Self
fn with_skip_signature(self, skip_signature: bool) -> Self
fn with_sse_kms_encryption(self, kms_key_id: impl Into<String>) -> Self
fn with_ssec_encryption(self, customer_key_base64: impl Into<String>) -> Self
fn with_token(self, token: impl Into<String>) -> Self
fn with_unsigned_payload(self, unsigned_payload: bool) -> Self
fn with_url(self, url: impl Into<String>) -> Self
fn with_virtual_hosted_style_request(self, virtual_hosted_style_request: bool) -> Self
```

[Full member, field, variant and typed contracts](../operations/object_store.aws.builder.AmazonS3Builder.md).


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

---
