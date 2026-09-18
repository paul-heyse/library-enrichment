# `buoyant_kernel::scan::data_skipping::DataSkippingFilter`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.scan.data_skipping.DataSkippingFilter.json).

<a id="op-5dd036bf42f274f2ce23a0b0"></a>
## DataSkippingFilter

`struct` · `buoyant_kernel::scan::data_skipping::DataSkippingFilter` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct DataSkippingFilter
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/data_skipping.rs#L91).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/data_skipping.rs:91`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22dd80471ffe56f332708880"></a>
## filter_evaluator

`struct_field` · `buoyant_kernel::scan::data_skipping::DataSkippingFilter::filter_evaluator` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
filter_evaluator: std::sync::Arc<dyn PredicateEvaluator>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/data_skipping.rs#L100).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/data_skipping.rs:100`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-032631a43e7dfeef20fa37ab"></a>
## metrics

`struct_field` · `buoyant_kernel::scan::data_skipping::DataSkippingFilter::metrics` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
metrics: Option<std::sync::Arc<scan::metrics::ScanMetrics>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/data_skipping.rs#L102).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/data_skipping.rs:102`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Metrics to record data skipping statistics.

<a id="op-bf63f639a194080959ce7c04"></a>
## skipping_evaluator

`struct_field` · `buoyant_kernel::scan::data_skipping::DataSkippingFilter::skipping_evaluator` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
skipping_evaluator: std::sync::Arc<dyn PredicateEvaluator>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/data_skipping.rs#L99).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/data_skipping.rs:99`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa621530b86277b4c29c5360"></a>
## stats_evaluator

`struct_field` · `buoyant_kernel::scan::data_skipping::DataSkippingFilter::stats_evaluator` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
stats_evaluator: std::sync::Arc<dyn ExpressionEvaluator>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/scan/data_skipping.rs#L98).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/scan/data_skipping.rs:98`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Evaluator that extracts file-level statistics from the input batch. The caller provides
the expression at construction time, which determines where stats come from:
- Scan path: `column_expr!("stats_parsed")` reads the already-parsed struct from a
  transformed batch (where `add.*` fields are flattened to top-level columns).
- Table changes path: `Expression::parse_json(column_expr!("add.stats"), schema)` parses
  JSON from a raw action batch (where stats are nested under `add.stats`).
