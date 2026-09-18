# `datafusion_ffi::session`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.session.json).

<a id="op-1f21bebf619e8a4e9da87c2d"></a>
## session

`module` · `datafusion_ffi::session` · datafusion-ffi 55.1.0

```rust
mod session
```

Source: `src/session/mod.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

FFI support for [`Session`](../operations/datafusion_session.session.Session.md#op-75302dfa669885e17a5c9093).

# Delegating physical planning

Consider a session owned by library A that uses a query planner owned by
library C. After A installs C's planner, [`ForeignSession::query_planner`](../operations/datafusion_ffi.session.ForeignSession.md#op-0e8ec6d47a5050d8bd11c711)
returns C's planner and [`ForeignSession::create_physical_plan`](../operations/datafusion_ffi.session.ForeignSession.md#op-c3fa7bdb67ffa8abd4a99b78) dispatches
to C's planner. C must not call `create_physical_plan`, or invoke the planner
returned by `query_planner`, to delegate planning back to A. Repeating either
self-call recurses until the stack is exhausted.

To delegate safely, A must export its original planner before installing C's
planner, and C must retain and invoke that planner directly. See the
[`crate::query_planner`](../modules/datafusion_ffi.query_planner.md#op-3caae2f40567c01aae388544) module for details.
