# `deltalake_aws::register_handlers`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_aws.register_handlers.json).

<a id="op-1bc1cda4486cb01abc5673f3"></a>
## register_handlers

`function` · `deltalake_aws::register_handlers` · deltalake-aws 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn register_handlers(_additional_prefixes: Option<url::Url>)
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/aws/src/lib.rs#L57).

Source: `crates/aws/src/lib.rs:57`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Register an [ObjectStoreFactory] for common S3 url schemes.

[ObjectStoreFactory]: deltalake_core::logstore::ObjectStoreFactory
