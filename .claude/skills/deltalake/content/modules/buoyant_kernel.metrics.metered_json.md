# `buoyant_kernel::metrics::metered_json`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.metrics.metered_json.json).

<a id="op-855062dbc146ae9fba0a0741"></a>
## metered_json

`module` · `buoyant_kernel::metrics::metered_json` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod metered_json
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/metrics/metered_json.rs#L1).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/metrics/metered_json.rs:1`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

[`MeteredJsonHandler`](../operations/buoyant_kernel.metrics.metered_json.MeteredJsonHandler.md#op-4917f5cfca39dfff79dc6c93) wraps any [`JsonHandler`](../operations/buoyant_kernel.JsonHandler.md#op-8c2157a5cd619a78e70c9841) so its `read_json_files` emits the
kernel's standard `JsonReadCompleted` span, carrying `(num_files, bytes_read)` exactly
once when the returned iterator is exhausted or dropped. `parse_json` and
`write_json_file` pass through.
