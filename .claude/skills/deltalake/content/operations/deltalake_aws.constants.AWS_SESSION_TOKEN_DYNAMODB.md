# `deltalake_aws::constants::AWS_SESSION_TOKEN_DYNAMODB`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_aws.constants.AWS_SESSION_TOKEN_DYNAMODB.json).

<a id="op-ef9748654edced2caec22224"></a>
## AWS_SESSION_TOKEN_DYNAMODB

`constant` · `deltalake_aws::constants::AWS_SESSION_TOKEN_DYNAMODB` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
const AWS_SESSION_TOKEN_DYNAMODB: &str = "AWS_SESSION_TOKEN_DYNAMODB"
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/constants.rs#L31).

Source: `crates/aws/src/constants.rs:31`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

If DynamoDB session token is different from S3 session token, set this to the DynamoDB session token.
If it is supplied, this session token takes precedence over the global session token set in AWS_SESSION_TOKEN for DynamoDB
