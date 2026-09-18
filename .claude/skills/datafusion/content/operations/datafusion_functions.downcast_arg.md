# `datafusion_functions::downcast_arg`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.downcast_arg.json).

<a id="op-8cc2ee2c2925609b07331669"></a>
## downcast_arg

`macro` · `datafusion_functions::downcast_arg` · datafusion-functions 55.1.0

```rust
macro_rules! downcast_arg
```

Source: `src/macros.rs:197`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Downcast an argument to a specific array type, returning an internal error
if the cast fails

$ARG: ArrayRef
$ARRAY_TYPE: the type of array to cast the argument to
