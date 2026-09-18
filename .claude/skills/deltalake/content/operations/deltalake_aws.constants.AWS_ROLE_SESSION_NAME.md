# `deltalake_aws::constants::AWS_ROLE_SESSION_NAME`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_aws.constants.AWS_ROLE_SESSION_NAME.json).

<a id="op-7c53d2c0535525f06d22a17f"></a>
## AWS_ROLE_SESSION_NAME

`constant` · `deltalake_aws::constants::AWS_ROLE_SESSION_NAME` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
const AWS_ROLE_SESSION_NAME: &str = "AWS_ROLE_SESSION_NAME"
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/constants.rs#L95).

Source: `crates/aws/src/constants.rs:95`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The role session name to use for web identity.

NOTE: web identity related options are set in the environment when
creating an instance of [S3StorageOptions](crate::storage::S3StorageOptions).
See also <https://docs.rs/rusoto_sts/0.47.0/rusoto_sts/struct.WebIdentityProvider.html#method.from_k8s_env>.
