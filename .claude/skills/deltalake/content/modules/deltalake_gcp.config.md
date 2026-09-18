# `deltalake_gcp::config`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_gcp.config.json).

<a id="op-79c520eddd55d8b21875b600"></a>
## config

`module` · `deltalake_gcp::config` · deltalake-gcp 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod config
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/gcp/src/config.rs#L1).

Source: `crates/gcp/src/config.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Auxiliary module for generating a valid Google cloud configuration.

Google offers few ways to authenticate against storage accounts and
provide credentials for a service principal. Some of this configuration may
partially be specified in the environment. This module establishes a structured
way how we discover valid credentials and some heuristics on how they are prioritized.
