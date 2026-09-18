# `deltalake_aws::constants::AWS_S3_POOL_IDLE_TIMEOUT_SECONDS`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_aws.constants.AWS_S3_POOL_IDLE_TIMEOUT_SECONDS.json).

<a id="op-f61d8a739140f05443ea1c4f"></a>
## AWS_S3_POOL_IDLE_TIMEOUT_SECONDS

`constant` · `deltalake_aws::constants::AWS_S3_POOL_IDLE_TIMEOUT_SECONDS` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
const AWS_S3_POOL_IDLE_TIMEOUT_SECONDS: &str = "AWS_S3_POOL_IDLE_TIMEOUT_SECONDS"
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/constants.rs#L71).

Source: `crates/aws/src/constants.rs:71`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The `pool_idle_timeout` option of aws http client.

Has to be lower than 20 seconds, which is
default S3 server timeout <https://aws.amazon.com/premiumsupport/knowledge-center/s3-socket-connection-timeout-error/>.
However, since rusoto uses hyper as a client, its default timeout is 90 seconds
<https://docs.rs/hyper/0.13.2/hyper/client/struct.Builder.html#method.keep_alive_timeout>.
Hence, the `connection closed before message completed` could occur.
To avoid that, the default value of this setting is 15 seconds if it's not set otherwise.
