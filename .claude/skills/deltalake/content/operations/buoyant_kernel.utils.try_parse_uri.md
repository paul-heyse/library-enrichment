# `buoyant_kernel::utils::try_parse_uri`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.utils.try_parse_uri.json).

<a id="op-b92d40729043175b47a5e2fc"></a>
## try_parse_uri

`function` · `buoyant_kernel::utils::try_parse_uri` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_parse_uri(uri: impl AsRef<str>) -> DeltaResult<url::Url>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/utils.rs#L63).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/utils.rs:63`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Try to parse string uri into a URL for a table path. This will do it's best to handle things
like `/local/paths`, and even `../relative/paths`.
