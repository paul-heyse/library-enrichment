# `deltalake_aws::constants::AWS_ACCESS_KEY_ID_DYNAMODB`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_aws.constants.AWS_ACCESS_KEY_ID_DYNAMODB.json).

<a id="op-3d999fe111647ef67fbb1afa"></a>
## AWS_ACCESS_KEY_ID_DYNAMODB

`constant` · `deltalake_aws::constants::AWS_ACCESS_KEY_ID_DYNAMODB` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
const AWS_ACCESS_KEY_ID_DYNAMODB: &str = "AWS_ACCESS_KEY_ID_DYNAMODB"
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/constants.rs#L25).

Source: `crates/aws/src/constants.rs:25`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

If DynamoDB access key is different from S3 access key, set this to the DynamoDB access key.
If it is supplied, this access key takes precedence over the global access key set in AWS_ACCESS_KEY_ID for DynamoDB
