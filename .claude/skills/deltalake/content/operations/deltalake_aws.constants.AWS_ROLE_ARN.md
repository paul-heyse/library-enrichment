# `deltalake_aws::constants::AWS_ROLE_ARN`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_aws.constants.AWS_ROLE_ARN.json).

<a id="op-6378d2b05ab731deda328e2f"></a>
## AWS_ROLE_ARN

`constant` · `deltalake_aws::constants::AWS_ROLE_ARN` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
const AWS_ROLE_ARN: &str = "AWS_ROLE_ARN"
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/constants.rs#L89).

Source: `crates/aws/src/constants.rs:89`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The role name to use for web identity.

NOTE: web identity related options are set in the environment when
creating an instance of [S3StorageOptions](crate::storage::S3StorageOptions).
See also <https://docs.rs/rusoto_sts/0.47.0/rusoto_sts/struct.WebIdentityProvider.html#method.from_k8s_env>.
