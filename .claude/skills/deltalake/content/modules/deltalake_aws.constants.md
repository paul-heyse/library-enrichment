# `deltalake_aws::constants`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_aws.constants.json).

<a id="op-1da55f2e5231c3610d5e2034"></a>
## constants

`module` · `deltalake_aws::constants` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod constants
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/constants.rs#L1).

Source: `crates/aws/src/constants.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Constants used for modifying and configuring various AWS S3 (or similar) connections with
delta-rs

Storage option keys to use when creating [`S3StorageOptions`].

The same key should be used whether passing a key in the hashmap or setting it as an environment variable.
Provided keys may include configuration for the S3 backend and also the optional DynamoDb lock used for atomic rename.
