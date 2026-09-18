# `datafusion_functions::downcast_named_arg`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.downcast_named_arg.json).

<a id="op-5b70621092521b95ec6710ed"></a>
## downcast_named_arg

`macro` · `datafusion_functions::downcast_named_arg` · datafusion-functions 55.1.0

```rust
macro_rules! downcast_named_arg
```

Source: `src/macros.rs:179`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Downcast a named argument to a specific array type, returning an internal error
if the cast fails

$ARG: ArrayRef
$NAME: name of the argument (for error messages)
$ARRAY_TYPE: the type of array to cast the argument to
