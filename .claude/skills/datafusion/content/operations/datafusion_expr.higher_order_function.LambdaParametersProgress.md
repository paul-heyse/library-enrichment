# `datafusion_expr::higher_order_function::LambdaParametersProgress`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.higher_order_function.LambdaParametersProgress.json).

<a id="op-e1f67db71a04644dca0b3add"></a>
## LambdaParametersProgress

`enum` · `datafusion_expr::higher_order_function::LambdaParametersProgress` · datafusion-expr 55.1.0

```rust
enum LambdaParametersProgress
```

Source: `src/higher_order_function.rs:500`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Represents a step during the resolution of the parameters of all lambdas of a given
higher-order function via [HigherOrderUDFImpl::lambda_parameters](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-8a89b1f3117f6cf763ede484). It's valid that the
fields of a given lambda changes between steps, and is up to the implementation to
provide during the function evaluation the parameters that matches the fields returned
at the [LambdaParametersProgress::Complete](../operations/datafusion_expr.higher_order_function.LambdaParametersProgress.md#op-56b7f43783579d5d25c47969) step. See [HigherOrderUDFImpl::lambda_parameters](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-8a89b1f3117f6cf763ede484)
docs for more details

<a id="op-56b7f43783579d5d25c47969"></a>
## Complete

`variant` · `datafusion_expr::higher_order_function::LambdaParametersProgress::Complete` · datafusion-expr 55.1.0

```rust
Complete
```

Source: `src/higher_order_function.rs:509`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

There are no unmet dependencies and all parameters are known, [HigherOrderUDFImpl::lambda_parameters](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-8a89b1f3117f6cf763ede484)
will not be called again

<a id="op-f0b9f54af5c1839b89e16765"></a>
## Partial

`variant` · `datafusion_expr::higher_order_function::LambdaParametersProgress::Partial` · datafusion-expr 55.1.0

```rust
Partial
```

Source: `src/higher_order_function.rs:506`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The parameters of some lambdas are unknown due to a dependency on another lambda output field
or are placeholders due to a dependency on it's own output field. It's perfectly valid to
contain only `Some`'s and not a single `None`, representing lambdas that depends only on itself
and not on others. [HigherOrderUDFImpl::lambda_parameters](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-8a89b1f3117f6cf763ede484) will be called again with the output
field of all lambdas with known parameters.
