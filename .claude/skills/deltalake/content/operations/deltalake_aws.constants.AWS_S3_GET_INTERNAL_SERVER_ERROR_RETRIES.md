# `deltalake_aws::constants::AWS_S3_GET_INTERNAL_SERVER_ERROR_RETRIES`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_aws.constants.AWS_S3_GET_INTERNAL_SERVER_ERROR_RETRIES.json).

<a id="op-f04d6dfb98424cf23ed05f49"></a>
## AWS_S3_GET_INTERNAL_SERVER_ERROR_RETRIES

`constant` · `deltalake_aws::constants::AWS_S3_GET_INTERNAL_SERVER_ERROR_RETRIES` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
const AWS_S3_GET_INTERNAL_SERVER_ERROR_RETRIES: &str = "AWS_S3_GET_INTERNAL_SERVER_ERROR_RETRIES"
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/constants.rs#L76).

Source: `crates/aws/src/constants.rs:76`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The number of retries for S3 GET requests failed with 500 Internal Server Error.
