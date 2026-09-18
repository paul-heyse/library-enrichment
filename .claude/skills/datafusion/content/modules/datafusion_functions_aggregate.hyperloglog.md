# `datafusion_functions_aggregate::hyperloglog`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.hyperloglog.json).

<a id="op-29269102a03e2fd4b6cb7bdc"></a>
## hyperloglog

`module` · `datafusion_functions_aggregate::hyperloglog` · datafusion-functions-aggregate 55.1.0

```rust
mod hyperloglog
```

Source: `src/hyperloglog.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

# HyperLogLog

`hyperloglog` is a module that contains a modified version
of [redis's implementation](https://github.com/redis/redis/blob/4930d19e70c391750479951022e207e19111eb55/src/hyperloglog.c)
with some modification based on strong assumption of usage
within datafusion, so that function can
be efficiently implemented.

Specifically, like Redis's version, this HLL structure uses
2**14 = 16384 registers, which means the standard error is
1.04/(16384**0.5) = 0.8125%. Unlike Redis, the register takes
up full [`u8`] size instead of a raw int* and thus saves some
tricky bit shifting techniques used in the original version.
This results in a memory usage increase from 12Kib to 16Kib.
Also only the dense version is adopted, so there's no automatic
conversion, largely to simplify the code.

This module also borrows some code structure from [pdatastructs.rs](https://github.com/crepererum/pdatastructs.rs/blob/3997ed50f6b6871c9e53c4c5e0f48f431405fc63/src/hyperloglog.rs).

Unresolved upstream links (retained, not inferred): ``u8``.
