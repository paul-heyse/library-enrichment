# `datafusion_physical_plan::proto`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.proto.json).

<a id="op-003f7212d73dcf1424b8f88e"></a>
## proto

`module` · `datafusion_physical_plan::proto` · datafusion-physical-plan 55.1.0

```rust
mod proto
```

Source: `src/proto.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Serialization hooks for [`ExecutionPlan`], mirroring the
`try_to_proto`/`try_from_proto` pattern used for `PhysicalExpr`.

# Why the indirection

An `ExecutionPlan` must be able to (de)serialize its child plans and its
child physical expressions recursively. The concrete recursion lives in
`datafusion-proto` (it owns the extension codec, the session context and the
central converter), but `datafusion-proto` sits *above* `datafusion-physical-plan`
in the crate graph. To let a plan drive that recursion without a dependency
cycle, this module defines:

* [`ExecutionPlanEncodeCtx`](../operations/datafusion_physical_plan.proto.ExecutionPlanEncodeCtx.md#op-ea54982e4141e05ebe735b2b) / [`ExecutionPlanDecodeCtx`](../operations/datafusion_physical_plan.proto.ExecutionPlanDecodeCtx.md#op-0b19f98851d9520d41eceb2c) — the stable,
  concrete context types a plan author interacts with. New capabilities can
  be added here without changing every plan's hook signature.
* [`ExecutionPlanEncode`] / [`ExecutionPlanDecode`] — internal dispatch
  traits, *defined* here but *implemented* in `datafusion-proto`, that the
  context types delegate to. This is the dependency inversion that keeps the
  proto types flowing in one direction only. They are `#[doc(hidden)]`: not
  public API, `pub` only because their implementors live in another crate.

`datafusion-physical-plan` depends on the pure prost types in
`datafusion-proto-models` (feature `proto`), never on `datafusion-proto`.

# Function-carrying plans

Plans that reference UD(A/W)Fs (`AggregateExec`, the window execs, …) also
ride the hook: the context exposes typed, *bytes-only* function serde —
[`encode_udaf`](ExecutionPlanEncodeCtx::encode_udaf) /
[`decode_udaf`](ExecutionPlanDecodeCtx::decode_udaf) and the udf/udwf
siblings. These take/return `datafusion-expr` types plus `Vec<u8>` and never
name a proto type, so the `PhysicalExtensionCodec` (which only
`datafusion-proto` can name) stays fully encapsulated behind the adapter that
backs these traits. The lookup-order policy (payload → codec; else registry →
codec fallback) lives once, in that adapter, rather than in every plan.

This is possible because `datafusion-physical-plan` sits *above*
`datafusion-expr` in the crate graph; the expression-side ctx (in
`physical-expr-common`, *below* `datafusion-expr`) cannot do this, which is
why `ScalarFunctionExpr` remains special-cased there.

[`ExecutionPlan`]: crate::ExecutionPlan

Unresolved upstream links (retained, not inferred): ``ExecutionPlanDecode``, ``ExecutionPlanEncode``.
