# `object_store::aws::builder::S3EncryptionConfigKey`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.aws.builder.S3EncryptionConfigKey.json).

<a id="op-09e317db6be71f4c9985bef6"></a>
## S3EncryptionConfigKey

`enum` · `object_store::aws::builder::S3EncryptionConfigKey` · object_store 0.13.2

```rust
enum S3EncryptionConfigKey
```

Source: `src/aws/builder.rs:1281`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Encryption configuration options for S3.

These options are used to configure server-side encryption for S3 objects.
To configure them, pass them to [`AmazonS3Builder::with_config`].

[SSE-S3]: https://docs.aws.amazon.com/AmazonS3/latest/userguide/UsingServerSideEncryption.html
[SSE-KMS]: https://docs.aws.amazon.com/AmazonS3/latest/userguide/UsingKMSEncryption.html
[DSSE-KMS]: https://docs.aws.amazon.com/AmazonS3/latest/userguide/UsingDSSEncryption.html
[SSE-C]: https://docs.aws.amazon.com/AmazonS3/latest/userguide/ServerSideEncryptionCustomerKeys.html

<a id="op-fd53df02b5b2eb7b52294001"></a>
## BucketKeyEnabled

`variant` · `object_store::aws::builder::S3EncryptionConfigKey::BucketKeyEnabled` · object_store 0.13.2

```rust
BucketKeyEnabled
```

Source: `src/aws/builder.rs:1310`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

If set to true, will use the bucket's default KMS key for server-side encryption.
If set to false, will disable the use of the bucket's default KMS key for server-side encryption.

Supported keys:
- `aws_sse_bucket_key_enabled`
- `sse_bucket_key_enabled`

<a id="op-04b4d5f7b34939f3091e05f1"></a>
## CustomerEncryptionKey

`variant` · `object_store::aws::builder::S3EncryptionConfigKey::CustomerEncryptionKey` · object_store 0.13.2

```rust
CustomerEncryptionKey
```

Source: `src/aws/builder.rs:1319`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The base64 encoded, 256-bit customer encryption key to use for server-side encryption.

If set, [ServerSideEncryption](Self::ServerSideEncryption) must be "sse-c".

Supported keys:
- `aws_sse_customer_key_base64`
- `sse_customer_key_base64`

<a id="op-5a9eb8019e64bb578cd04da5"></a>
## KmsKeyId

`variant` · `object_store::aws::builder::S3EncryptionConfigKey::KmsKeyId` · object_store 0.13.2

```rust
KmsKeyId
```

Source: `src/aws/builder.rs:1303`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The KMS key ID to use for server-side encryption.

If set, [ServerSideEncryption](Self::ServerSideEncryption) must be "aws:kms" or "aws:kms:dsse".

Supported keys:
- `aws_sse_kms_key_id`
- `sse_kms_key_id`

Example: `arn:aws:kms:us-east-1:123456789012:key/abcd-1234-efgh-5678`

<a id="op-18a4ea768c5c8a0587acb219"></a>
## ServerSideEncryption

`variant` · `object_store::aws::builder::S3EncryptionConfigKey::ServerSideEncryption` · object_store 0.13.2

```rust
ServerSideEncryption
```

Source: `src/aws/builder.rs:1293`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Type of encryption to use.

If set, must be one of
- "AES256" (SSE-S3),
- "aws:kms" (SSE-KMS),
- "aws:kms:dsse" (DSSE-KMS) or
- "sse-c"

Supported keys:
- `aws_server_side_encryption`
- `server_side_encryption`
