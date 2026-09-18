# `deltalake_aws::constants::S3_OPTS`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_aws.constants.S3_OPTS.json).

<a id="op-7d150869efebb73ef61ce2a8"></a>
## S3_OPTS

`constant` · `deltalake_aws::constants::S3_OPTS` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
const S3_OPTS: &[&str] = _
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/constants.rs#L118).

Source: `crates/aws/src/constants.rs:118`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The list of option keys owned by the S3 module.
Option keys not contained in this list will be added to the `extra_opts`
field of [S3StorageOptions](crate::storage::S3StorageOptions).
