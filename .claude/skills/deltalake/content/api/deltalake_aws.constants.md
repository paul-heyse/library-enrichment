# `deltalake_aws::constants`

Crate `deltalake-aws` · 48 public items · structured records in [`model/deltalake_aws.constants.json`](../model/deltalake_aws.constants.json)

## ATTR_COMPLETE

`constant` · `deltalake_aws::constants::ATTR_COMPLETE`

```rust
const ATTR_COMPLETE: &str = "complete"
```

---

## ATTR_EXPIRE_TIME

`constant` · `deltalake_aws::constants::ATTR_EXPIRE_TIME`

```rust
const ATTR_EXPIRE_TIME: &str = "expireTime"
```

---

## ATTR_FILE_NAME

`constant` · `deltalake_aws::constants::ATTR_FILE_NAME`

```rust
const ATTR_FILE_NAME: &str = "fileName"
```

---

## ATTR_TABLE_PATH

`constant` · `deltalake_aws::constants::ATTR_TABLE_PATH`

```rust
const ATTR_TABLE_PATH: &str = "tablePath"
```

---

## ATTR_TEMP_PATH

`constant` · `deltalake_aws::constants::ATTR_TEMP_PATH`

```rust
const ATTR_TEMP_PATH: &str = "tempPath"
```

---

## AWS_ACCESS_KEY_ID

`constant` · `deltalake_aws::constants::AWS_ACCESS_KEY_ID`

```rust
const AWS_ACCESS_KEY_ID: &str = "AWS_ACCESS_KEY_ID"
```

The AWS_ACCESS_KEY_ID to use for S3.

---

## AWS_ACCESS_KEY_ID_DYNAMODB

`constant` · `deltalake_aws::constants::AWS_ACCESS_KEY_ID_DYNAMODB`

```rust
const AWS_ACCESS_KEY_ID_DYNAMODB: &str = "AWS_ACCESS_KEY_ID_DYNAMODB"
```

If DynamoDB access key is different from S3 access key, set this to the DynamoDB access key.
If it is supplied, this access key takes precedence over the global access key set in AWS_ACCESS_KEY_ID for DynamoDB

---

## AWS_ALLOW_HTTP

`constant` · `deltalake_aws::constants::AWS_ALLOW_HTTP`

```rust
const AWS_ALLOW_HTTP: &str = "AWS_ALLOW_HTTP"
```

Allow http connections - mainly useful for integration tests

---

## AWS_EC2_METADATA_DISABLED

`constant` · `deltalake_aws::constants::AWS_EC2_METADATA_DISABLED`

```rust
const AWS_EC2_METADATA_DISABLED: &str = "AWS_EC2_METADATA_DISABLED"
```

If set to "true", disables the imds client
Defaults to "true"

---

## AWS_EC2_METADATA_TIMEOUT

`constant` · `deltalake_aws::constants::AWS_EC2_METADATA_TIMEOUT`

```rust
const AWS_EC2_METADATA_TIMEOUT: &str = "AWS_EC2_METADATA_TIMEOUT"
```

The timeout in milliseconds for the EC2 metadata endpoint
Defaults to 100

---

## AWS_ENDPOINT_URL

`constant` · `deltalake_aws::constants::AWS_ENDPOINT_URL`

```rust
const AWS_ENDPOINT_URL: &str = "AWS_ENDPOINT_URL"
```

Custom S3 endpoint.

---

## AWS_ENDPOINT_URL_DYNAMODB

`constant` · `deltalake_aws::constants::AWS_ENDPOINT_URL_DYNAMODB`

```rust
const AWS_ENDPOINT_URL_DYNAMODB: &str = "AWS_ENDPOINT_URL_DYNAMODB"
```

Custom DynamoDB

If DynamoDB endpoint is not supplied, will use S3 endpoint (AWS_ENDPOINT_URL)
If it is supplied, this endpoint takes precedence over the global endpoint set in AWS_ENDPOINT_URL for DynamoDB

---

## AWS_FORCE_CREDENTIAL_LOAD

`constant` · `deltalake_aws::constants::AWS_FORCE_CREDENTIAL_LOAD`

```rust
const AWS_FORCE_CREDENTIAL_LOAD: &str = "AWS_FORCE_CREDENTIAL_LOAD"
```

Force the delta-rs to attempt to load AWS credentials

---

## AWS_IAM_ROLE_ARN

`constant` · `deltalake_aws::constants::AWS_IAM_ROLE_ARN`

```rust
const AWS_IAM_ROLE_ARN: &str = "AWS_IAM_ROLE_ARN"
```

The role to assume for S3 writes.

---

## AWS_IAM_ROLE_SESSION_NAME

`constant` · `deltalake_aws::constants::AWS_IAM_ROLE_SESSION_NAME`

```rust
const AWS_IAM_ROLE_SESSION_NAME: &str = "AWS_IAM_ROLE_SESSION_NAME"
```

The role session name to use when a role is assumed. If not provided a random session name is generated.

---

## AWS_PROFILE

`constant` · `deltalake_aws::constants::AWS_PROFILE`

```rust
const AWS_PROFILE: &str = "AWS_PROFILE"
```

The AWS profile.

---

## AWS_REGION

`constant` · `deltalake_aws::constants::AWS_REGION`

```rust
const AWS_REGION: &str = "AWS_REGION"
```

The AWS region.

---

## AWS_REGION_DYNAMODB

`constant` · `deltalake_aws::constants::AWS_REGION_DYNAMODB`

```rust
const AWS_REGION_DYNAMODB: &str = "AWS_REGION_DYNAMODB"
```

If DynamoDB region is different from S3 region, set this to the DynamoDB region.
If it is supplied, this region takes precedence over the global region set in AWS_REGION for DynamoDB

---

## AWS_ROLE_ARN

`constant` · `deltalake_aws::constants::AWS_ROLE_ARN`

```rust
const AWS_ROLE_ARN: &str = "AWS_ROLE_ARN"
```

The role name to use for web identity.

NOTE: web identity related options are set in the environment when
creating an instance of [S3StorageOptions](crate::storage::S3StorageOptions).
See also <https://docs.rs/rusoto_sts/0.47.0/rusoto_sts/struct.WebIdentityProvider.html#method.from_k8s_env>.

---

## AWS_ROLE_SESSION_NAME

`constant` · `deltalake_aws::constants::AWS_ROLE_SESSION_NAME`

```rust
const AWS_ROLE_SESSION_NAME: &str = "AWS_ROLE_SESSION_NAME"
```

The role session name to use for web identity.

NOTE: web identity related options are set in the environment when
creating an instance of [S3StorageOptions](crate::storage::S3StorageOptions).
See also <https://docs.rs/rusoto_sts/0.47.0/rusoto_sts/struct.WebIdentityProvider.html#method.from_k8s_env>.

---

## AWS_S3_ADDRESSING_STYLE

`constant` · `deltalake_aws::constants::AWS_S3_ADDRESSING_STYLE`

```rust
const AWS_S3_ADDRESSING_STYLE: &str = "AWS_S3_ADDRESSING_STYLE"
```

Uses either "path" (the default) or "virtual", which turns on
[virtual host addressing](http://docs.aws.amazon.com/AmazonS3/latest/dev/VirtualHosting.html).

---

## AWS_S3_ALLOW_UNSAFE_RENAME

`constant` · `deltalake_aws::constants::AWS_S3_ALLOW_UNSAFE_RENAME`

```rust
const AWS_S3_ALLOW_UNSAFE_RENAME: &str = "AWS_S3_ALLOW_UNSAFE_RENAME"
```

If set to "true", allows creating commits without concurrent writer protection.
Only safe if there is one writer to a given table.

---

## AWS_S3_ASSUME_ROLE_ARN

`constant` · `deltalake_aws::constants::AWS_S3_ASSUME_ROLE_ARN`

> **Deprecated** — since 0.20.0: Please use AWS_IAM_ROLE_ARN instead

```rust
const AWS_S3_ASSUME_ROLE_ARN: &str = "AWS_S3_ASSUME_ROLE_ARN"
```

The role to assume. Please use [AWS_IAM_ROLE_ARN] instead

---

## AWS_S3_GET_INTERNAL_SERVER_ERROR_RETRIES

`constant` · `deltalake_aws::constants::AWS_S3_GET_INTERNAL_SERVER_ERROR_RETRIES`

```rust
const AWS_S3_GET_INTERNAL_SERVER_ERROR_RETRIES: &str = "AWS_S3_GET_INTERNAL_SERVER_ERROR_RETRIES"
```

The number of retries for S3 GET requests failed with 500 Internal Server Error.

---

## AWS_S3_LOCKING_PROVIDER

`constant` · `deltalake_aws::constants::AWS_S3_LOCKING_PROVIDER`

```rust
const AWS_S3_LOCKING_PROVIDER: &str = "AWS_S3_LOCKING_PROVIDER"
```

Locking provider to use for safe atomic rename.
`dynamodb` is currently the only supported locking provider.
If not set, safe atomic rename is not available.

---

## AWS_S3_POOL_IDLE_TIMEOUT_SECONDS

`constant` · `deltalake_aws::constants::AWS_S3_POOL_IDLE_TIMEOUT_SECONDS`

```rust
const AWS_S3_POOL_IDLE_TIMEOUT_SECONDS: &str = "AWS_S3_POOL_IDLE_TIMEOUT_SECONDS"
```

The `pool_idle_timeout` option of aws http client.

Has to be lower than 20 seconds, which is
default S3 server timeout <https://aws.amazon.com/premiumsupport/knowledge-center/s3-socket-connection-timeout-error/>.
However, since rusoto uses hyper as a client, its default timeout is 90 seconds
<https://docs.rs/hyper/0.13.2/hyper/client/struct.Builder.html#method.keep_alive_timeout>.
Hence, the `connection closed before message completed` could occur.
To avoid that, the default value of this setting is 15 seconds if it's not set otherwise.

---

## AWS_S3_ROLE_SESSION_NAME

`constant` · `deltalake_aws::constants::AWS_S3_ROLE_SESSION_NAME`

> **Deprecated** — since 0.20.0: Please use AWS_IAM_ROLE_SESSION_NAME instead

```rust
const AWS_S3_ROLE_SESSION_NAME: &str = "AWS_S3_ROLE_SESSION_NAME"
```

The role session name to use when a role is assumed. If not provided a random session name is generated.

---

## AWS_SECRET_ACCESS_KEY

`constant` · `deltalake_aws::constants::AWS_SECRET_ACCESS_KEY`

```rust
const AWS_SECRET_ACCESS_KEY: &str = "AWS_SECRET_ACCESS_KEY"
```

The AWS_SECRET_ACCESS_KEY to use for S3.

---

## AWS_SECRET_ACCESS_KEY_DYNAMODB

`constant` · `deltalake_aws::constants::AWS_SECRET_ACCESS_KEY_DYNAMODB`

```rust
const AWS_SECRET_ACCESS_KEY_DYNAMODB: &str = "AWS_SECRET_ACCESS_KEY_DYNAMODB"
```

If DynamoDB secret key is different from S3 secret key, set this to the DynamoDB secret key.
If it is supplied, this secret key takes precedence over the global secret key set in AWS_SECRET_ACCESS_KEY for DynamoDB

---

## AWS_SESSION_TOKEN

`constant` · `deltalake_aws::constants::AWS_SESSION_TOKEN`

```rust
const AWS_SESSION_TOKEN: &str = "AWS_SESSION_TOKEN"
```

The AWS_SESSION_TOKEN to use for S3.

---

## AWS_SESSION_TOKEN_DYNAMODB

`constant` · `deltalake_aws::constants::AWS_SESSION_TOKEN_DYNAMODB`

```rust
const AWS_SESSION_TOKEN_DYNAMODB: &str = "AWS_SESSION_TOKEN_DYNAMODB"
```

If DynamoDB session token is different from S3 session token, set this to the DynamoDB session token.
If it is supplied, this session token takes precedence over the global session token set in AWS_SESSION_TOKEN for DynamoDB

---

## AWS_STS_POOL_IDLE_TIMEOUT_SECONDS

`constant` · `deltalake_aws::constants::AWS_STS_POOL_IDLE_TIMEOUT_SECONDS`

```rust
const AWS_STS_POOL_IDLE_TIMEOUT_SECONDS: &str = "AWS_STS_POOL_IDLE_TIMEOUT_SECONDS"
```

The `pool_idle_timeout` for the as3_constants sts client. See
the reasoning in `AWS_S3_POOL_IDLE_TIMEOUT_SECONDS`.

---

## AWS_WEB_IDENTITY_TOKEN_FILE

`constant` · `deltalake_aws::constants::AWS_WEB_IDENTITY_TOKEN_FILE`

```rust
const AWS_WEB_IDENTITY_TOKEN_FILE: &str = "AWS_WEB_IDENTITY_TOKEN_FILE"
```

The web identity token file to use when using a web identity provider.

NOTE: web identity related options are set in the environment when
creating an instance of [S3StorageOptions](crate::storage::S3StorageOptions).
See also <https://docs.rs/rusoto_sts/0.47.0/rusoto_sts/struct.WebIdentityProvider.html#method.from_k8s_env>.

---

## BILLING_MODE_KEY_NAME

`constant` · `deltalake_aws::constants::BILLING_MODE_KEY_NAME`

```rust
const BILLING_MODE_KEY_NAME: &str = "DELTA_DYNAMO_BILLING_MODE"
```

---

## CONDITION_UPDATE_INCOMPLETE

`constant` · `deltalake_aws::constants::CONDITION_UPDATE_INCOMPLETE`

```rust
const CONDITION_UPDATE_INCOMPLETE: &str = "complete = :f"
```

---

## DEFAULT_COMMIT_ENTRY_EXPIRATION_DELAY

`constant` · `deltalake_aws::constants::DEFAULT_COMMIT_ENTRY_EXPIRATION_DELAY`

```rust
const DEFAULT_COMMIT_ENTRY_EXPIRATION_DELAY: std::time::Duration = _
```

---

## DEFAULT_LOCK_TABLE_NAME

`constant` · `deltalake_aws::constants::DEFAULT_LOCK_TABLE_NAME`

```rust
const DEFAULT_LOCK_TABLE_NAME: &str = "delta_log"
```

---

## DEFAULT_S3_GET_INTERNAL_SERVER_ERROR_RETRIES

`constant` · `deltalake_aws::constants::DEFAULT_S3_GET_INTERNAL_SERVER_ERROR_RETRIES`

```rust
const DEFAULT_S3_GET_INTERNAL_SERVER_ERROR_RETRIES: usize = 10
```

---

## DEFAULT_S3_POOL_IDLE_TIMEOUT_SECONDS

`constant` · `deltalake_aws::constants::DEFAULT_S3_POOL_IDLE_TIMEOUT_SECONDS`

```rust
const DEFAULT_S3_POOL_IDLE_TIMEOUT_SECONDS: u64 = 15
```

---

## DEFAULT_STS_POOL_IDLE_TIMEOUT_SECONDS

`constant` · `deltalake_aws::constants::DEFAULT_STS_POOL_IDLE_TIMEOUT_SECONDS`

```rust
const DEFAULT_STS_POOL_IDLE_TIMEOUT_SECONDS: u64 = 10
```

---

## KEY_TYPE_HASH

`constant` · `deltalake_aws::constants::KEY_TYPE_HASH`

```rust
const KEY_TYPE_HASH: &str = "HASH"
```

---

## KEY_TYPE_RANGE

`constant` · `deltalake_aws::constants::KEY_TYPE_RANGE`

```rust
const KEY_TYPE_RANGE: &str = "RANGE"
```

---

## LOCK_TABLE_KEY_NAME

`constant` · `deltalake_aws::constants::LOCK_TABLE_KEY_NAME`

```rust
const LOCK_TABLE_KEY_NAME: &str = "DELTA_DYNAMO_TABLE_NAME"
```

---

## MAX_ELAPSED_REQUEST_TIME_KEY_NAME

`constant` · `deltalake_aws::constants::MAX_ELAPSED_REQUEST_TIME_KEY_NAME`

```rust
const MAX_ELAPSED_REQUEST_TIME_KEY_NAME: &str = "DELTA_DYNAMO_MAX_ELAPSED_REQUEST_TIME"
```

---

## S3_OPTS

`constant` · `deltalake_aws::constants::S3_OPTS`

```rust
const S3_OPTS: &[&str] = _
```

The list of option keys owned by the S3 module.
Option keys not contained in this list will be added to the `extra_opts`
field of [S3StorageOptions](crate::storage::S3StorageOptions).

---

## STRING_TYPE

`constant` · `deltalake_aws::constants::STRING_TYPE`

```rust
const STRING_TYPE: &str = "S"
```

---

## CONDITION_DELETE_INCOMPLETE

`static` · `deltalake_aws::constants::CONDITION_DELETE_INCOMPLETE`

```rust
static CONDITION_DELETE_INCOMPLETE: std::sync::LazyLock<String>
```

---

## CONDITION_EXPR_CREATE

`static` · `deltalake_aws::constants::CONDITION_EXPR_CREATE`

```rust
static CONDITION_EXPR_CREATE: std::sync::LazyLock<String>
```

---
