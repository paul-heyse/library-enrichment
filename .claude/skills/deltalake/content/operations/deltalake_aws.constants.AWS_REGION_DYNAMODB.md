# `deltalake_aws::constants::AWS_REGION_DYNAMODB`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_aws.constants.AWS_REGION_DYNAMODB.json).

<a id="op-87cb7228b5668d99504fa946"></a>
## AWS_REGION_DYNAMODB

`constant` · `deltalake_aws::constants::AWS_REGION_DYNAMODB` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
const AWS_REGION_DYNAMODB: &str = "AWS_REGION_DYNAMODB"
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/constants.rs#L22).

Source: `crates/aws/src/constants.rs:22`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

If DynamoDB region is different from S3 region, set this to the DynamoDB region.
If it is supplied, this region takes precedence over the global region set in AWS_REGION for DynamoDB
