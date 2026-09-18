# `deltalake_aws::constants::AWS_SECRET_ACCESS_KEY_DYNAMODB`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_aws.constants.AWS_SECRET_ACCESS_KEY_DYNAMODB.json).

<a id="op-2bfe7e61038007b49b912961"></a>
## AWS_SECRET_ACCESS_KEY_DYNAMODB

`constant` · `deltalake_aws::constants::AWS_SECRET_ACCESS_KEY_DYNAMODB` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
const AWS_SECRET_ACCESS_KEY_DYNAMODB: &str = "AWS_SECRET_ACCESS_KEY_DYNAMODB"
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/constants.rs#L28).

Source: `crates/aws/src/constants.rs:28`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

If DynamoDB secret key is different from S3 secret key, set this to the DynamoDB secret key.
If it is supplied, this secret key takes precedence over the global secret key set in AWS_SECRET_ACCESS_KEY for DynamoDB
