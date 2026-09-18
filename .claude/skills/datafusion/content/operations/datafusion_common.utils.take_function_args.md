# `datafusion_common::utils::take_function_args`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.take_function_args.json).

<a id="op-bdebbfe5654755406d436505"></a>
## take_function_args

`function` · `datafusion_common::utils::take_function_args` · datafusion-common 55.1.0

```rust
fn take_function_args<const N: usize, T>(function_name: &str, args: impl IntoIterator<Item = T>) -> Result<[T; N]>
```

Source: `src/utils/mod.rs:1186`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Converts a collection of function arguments into a fixed-size array of length N
producing a reasonable error message in case of unexpected number of arguments.

# Example
```
# use datafusion_common::Result;
# use datafusion_common::utils::take_function_args;
# use datafusion_common::ScalarValue;
fn my_function(args: &[ScalarValue]) -> Result<()> {
    // function expects 2 args, so create a 2-element array
    let [arg1, arg2] = take_function_args("my_function", args)?;
    // ... do stuff..
    Ok(())
}

// Calling the function with 1 argument produces an error:
let args = vec![ScalarValue::Int32(Some(10))];
let err = my_function(&args).unwrap_err();
assert_eq!(
    err.to_string(),
    "Execution error: my_function function requires 2 arguments, got 1"
);
// Calling the function with 2 arguments works great
let args = vec![ScalarValue::Int32(Some(10)), ScalarValue::Int32(Some(20))];
my_function(&args).unwrap();
```
