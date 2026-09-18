# `datafusion_physical_plan::buffer`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.buffer.json).

<a id="op-bcf33191d4dfca85d99170f4"></a>
## buffer

`module` · `datafusion_physical_plan::buffer` · datafusion-physical-plan 55.1.0

```rust
mod buffer
```

Source: `src/buffer.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

[`BufferExec`](../operations/datafusion_physical_plan.buffer.BufferExec.md#op-dfabfc6e02e65fdcdbb664da) decouples production and consumption on messages by buffering the input in the
background up to a certain capacity.
