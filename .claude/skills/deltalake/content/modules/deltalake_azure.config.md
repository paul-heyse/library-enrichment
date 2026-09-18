# `deltalake_azure::config`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_azure.config.json).

<a id="op-b1bc8b6b1fe46b3bd0916ffe"></a>
## config

`module` · `deltalake_azure::config` · deltalake-azure 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod config
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/azure/src/config.rs#L1).

Source: `crates/azure/src/config.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Auxiliary module for generating a valig Azure configuration.

Azure offers many different ways to authenticate against storage accounts and
provide credentials for a service principal. Some of this configutaion may
partially be specified in the environment. This module establishes a structured
way how we discover valid credentials and some heuristics on how they are prioritized.
