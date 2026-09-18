# `deltalake_aws::constants::AWS_S3_LOCKING_PROVIDER`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_aws.constants.AWS_S3_LOCKING_PROVIDER.json).

<a id="op-a2fdeb20ec71aa56a14f5429"></a>
## AWS_S3_LOCKING_PROVIDER

`constant` · `deltalake_aws::constants::AWS_S3_LOCKING_PROVIDER` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
const AWS_S3_LOCKING_PROVIDER: &str = "AWS_S3_LOCKING_PROVIDER"
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/constants.rs#L49).

Source: `crates/aws/src/constants.rs:49`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Locking provider to use for safe atomic rename.
`dynamodb` is currently the only supported locking provider.
If not set, safe atomic rename is not available.
