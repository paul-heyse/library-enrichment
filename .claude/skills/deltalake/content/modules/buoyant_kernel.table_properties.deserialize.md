# `buoyant_kernel::table_properties::deserialize`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.table_properties.deserialize.json).

<a id="op-223acacebabc1be950a4991f"></a>
## deserialize

`module` · `buoyant_kernel::table_properties::deserialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod deserialize
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_properties/deserialize.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_properties/deserialize.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

For now we just use simple functions to deserialize table properties from strings. This allows
us to relatively simply implement the functionality described in the protocol and expose
'simple' types to the user in the [`TableProperties`](../operations/buoyant_kernel.table_properties.TableProperties.md#op-a6b333464916b5b04ef86dcd) struct. E.g. we can expose a `bool`
directly instead of a `BoolConfig` type that we implement `Deserialize` for.
