# `deltalake_aws::constants::AWS_ENDPOINT_URL_DYNAMODB`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_aws.constants.AWS_ENDPOINT_URL_DYNAMODB.json).

<a id="op-931ac48d73acf4211c1410f5"></a>
## AWS_ENDPOINT_URL_DYNAMODB

`constant` · `deltalake_aws::constants::AWS_ENDPOINT_URL_DYNAMODB` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
const AWS_ENDPOINT_URL_DYNAMODB: &str = "AWS_ENDPOINT_URL_DYNAMODB"
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/constants.rs#L19).

Source: `crates/aws/src/constants.rs:19`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Custom DynamoDB

If DynamoDB endpoint is not supplied, will use S3 endpoint (AWS_ENDPOINT_URL)
If it is supplied, this endpoint takes precedence over the global endpoint set in AWS_ENDPOINT_URL for DynamoDB
