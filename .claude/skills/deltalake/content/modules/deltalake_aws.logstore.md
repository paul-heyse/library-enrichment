# `deltalake_aws::logstore`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_aws.logstore.json).

<a id="op-983ea1823b5f2d1eeb5a5bd0"></a>
## logstore

`module` · `deltalake_aws::logstore` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod logstore
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/logstore/mod.rs#L1).

Source: `crates/aws/src/logstore/mod.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Contains the different logstore implementations for S3.
- S3LogStore (used when copy-if-not-exists or unsafe_rename is passed)
- S3DynamoDBLogStore (used when DynamoDB is the locking client)
