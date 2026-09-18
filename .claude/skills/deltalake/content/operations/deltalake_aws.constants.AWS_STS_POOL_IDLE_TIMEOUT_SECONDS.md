# `deltalake_aws::constants::AWS_STS_POOL_IDLE_TIMEOUT_SECONDS`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_aws.constants.AWS_STS_POOL_IDLE_TIMEOUT_SECONDS.json).

<a id="op-0ad38d24c4b28f000fc0785f"></a>
## AWS_STS_POOL_IDLE_TIMEOUT_SECONDS

`constant` · `deltalake_aws::constants::AWS_STS_POOL_IDLE_TIMEOUT_SECONDS` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
const AWS_STS_POOL_IDLE_TIMEOUT_SECONDS: &str = "AWS_STS_POOL_IDLE_TIMEOUT_SECONDS"
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/constants.rs#L74).

Source: `crates/aws/src/constants.rs:74`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The `pool_idle_timeout` for the as3_constants sts client. See
the reasoning in `AWS_S3_POOL_IDLE_TIMEOUT_SECONDS`.
