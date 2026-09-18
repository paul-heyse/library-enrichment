# `deltalake_aws::constants::AWS_WEB_IDENTITY_TOKEN_FILE`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_aws.constants.AWS_WEB_IDENTITY_TOKEN_FILE.json).

<a id="op-d208c148a3e4a1235908c904"></a>
## AWS_WEB_IDENTITY_TOKEN_FILE

`constant` · `deltalake_aws::constants::AWS_WEB_IDENTITY_TOKEN_FILE` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
const AWS_WEB_IDENTITY_TOKEN_FILE: &str = "AWS_WEB_IDENTITY_TOKEN_FILE"
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/constants.rs#L83).

Source: `crates/aws/src/constants.rs:83`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The web identity token file to use when using a web identity provider.

NOTE: web identity related options are set in the environment when
creating an instance of [S3StorageOptions](crate::storage::S3StorageOptions).
See also <https://docs.rs/rusoto_sts/0.47.0/rusoto_sts/struct.WebIdentityProvider.html#method.from_k8s_env>.
