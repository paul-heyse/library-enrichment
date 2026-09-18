# `deltalake_aws::constants::AWS_S3_ADDRESSING_STYLE`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_aws.constants.AWS_S3_ADDRESSING_STYLE.json).

<a id="op-707a07461ce68475def93779"></a>
## AWS_S3_ADDRESSING_STYLE

`constant` · `deltalake_aws::constants::AWS_S3_ADDRESSING_STYLE` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
const AWS_S3_ADDRESSING_STYLE: &str = "AWS_S3_ADDRESSING_STYLE"
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/constants.rs#L45).

Source: `crates/aws/src/constants.rs:45`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Uses either "path" (the default) or "virtual", which turns on
[virtual host addressing](http://docs.aws.amazon.com/AmazonS3/latest/dev/VirtualHosting.html).
