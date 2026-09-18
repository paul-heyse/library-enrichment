# `deltalake_aws`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_aws.json).

<a id="op-4bf9415f29dc8c2d96862c0d"></a>
## deltalake_aws

`module` · `deltalake_aws` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod deltalake_aws
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/lib.rs#L1).

Source: `crates/aws/src/lib.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

AWS S3 and similar tooling for delta-rs

This module also contains the [S3DynamoDbLogStore](crate::logstore::S3DynamoDbLogStore)
implementation for concurrent writer support with AWS S3 specifically.
