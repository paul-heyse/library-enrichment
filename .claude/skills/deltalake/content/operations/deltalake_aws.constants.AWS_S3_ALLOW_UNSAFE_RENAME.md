# `deltalake_aws::constants::AWS_S3_ALLOW_UNSAFE_RENAME`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_aws.constants.AWS_S3_ALLOW_UNSAFE_RENAME.json).

<a id="op-c48b550a96a3ee9d0d35f18e"></a>
## AWS_S3_ALLOW_UNSAFE_RENAME

`constant` · `deltalake_aws::constants::AWS_S3_ALLOW_UNSAFE_RENAME` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
const AWS_S3_ALLOW_UNSAFE_RENAME: &str = "AWS_S3_ALLOW_UNSAFE_RENAME"
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/constants.rs#L101).

Source: `crates/aws/src/constants.rs:101`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

If set to "true", allows creating commits without concurrent writer protection.
Only safe if there is one writer to a given table.
