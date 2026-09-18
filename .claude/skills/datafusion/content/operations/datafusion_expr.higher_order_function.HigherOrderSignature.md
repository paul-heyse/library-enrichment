# `datafusion_expr::higher_order_function::HigherOrderSignature`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.higher_order_function.HigherOrderSignature.json).

<a id="op-57014c54db47f90a3230a79a"></a>
## HigherOrderSignature

`struct` · `datafusion_expr::higher_order_function::HigherOrderSignature` · datafusion-expr 55.1.0

```rust
struct HigherOrderSignature
```

Source: `src/higher_order_function.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Provides information necessary for calling a higher order function.

- [`HigherOrderTypeSignature`](../operations/datafusion_expr.higher_order_function.HigherOrderTypeSignature.md#op-fbe23e3717bc225a891eae94) defines the argument types that a function has implementations
  for.

- [`Volatility`](../operations/datafusion_expr_common.signature.Volatility.md#op-2e27042945dba7db012302a0) defines how the output of the function changes with the input.

<a id="op-9929cb64b6904ae0195db18d"></a>
## any

`function` · `datafusion_expr::higher_order_function::HigherOrderSignature::any` · datafusion-expr 55.1.0

```rust
fn any(arg_count: usize, volatility: Volatility) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderSignature", "path": "HigherOrderSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 1], "end": [160, 2], "filename": "src/higher_order_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/higher_order_function.rs:132`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A specified number of arguments of any type

<a id="op-e758951024ef5e04aa902011"></a>
## clone

`function` · `datafusion_expr::higher_order_function::HigherOrderSignature::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> HigherOrderSignature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderSignature", "path": "HigherOrderSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 17], "end": [89, 22], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/higher_order_function.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-40c1b43aa3fd90557a55d816"></a>
## eq

`function` · `datafusion_expr::higher_order_function::HigherOrderSignature::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &HigherOrderSignature) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderSignature", "path": "HigherOrderSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 24], "end": [89, 33], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/higher_order_function.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f62ef4423960421e2d58ff4"></a>
## exact

`function` · `datafusion_expr::higher_order_function::HigherOrderSignature::exact` · datafusion-expr 55.1.0

```rust
fn exact(args: Vec<ValueOrLambda<(), ()>>, volatility: Volatility) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderSignature", "path": "HigherOrderSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 1], "end": [160, 2], "filename": "src/higher_order_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/higher_order_function.rs:153`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Exactly the specified arguments in the given order, with arbitrary types.
DataFusion will call [`HigherOrderUDFImpl::coerce_value_types`](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-f20fe249c57b53787b057f56) to prepare the value
argument types.

# Example
A function that takes one value argument followed by one lambda:
```
# use datafusion_expr::{HigherOrderSignature, ValueOrLambda, Volatility};
let sig = HigherOrderSignature::exact(
    vec![ValueOrLambda::Value(()), ValueOrLambda::Lambda(())],
    Volatility::Immutable,
);
```

<a id="op-4e1dd66141b9ca9aff07cc34"></a>
## fmt

`function` · `datafusion_expr::higher_order_function::HigherOrderSignature::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderSignature", "path": "HigherOrderSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 10], "end": [89, 15], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/higher_order_function.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9fd45b7c6dde41a3c0ddd4a1"></a>
## hash

`function` · `datafusion_expr::higher_order_function::HigherOrderSignature::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderSignature", "path": "HigherOrderSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 51], "end": [89, 55], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/higher_order_function.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8616a5b8e9e186a386aee3f3"></a>
## lambda_parameters_max_iterations

`struct_field` · `datafusion_expr::higher_order_function::HigherOrderSignature::lambda_parameters_max_iterations` · datafusion-expr 55.1.0

```rust
lambda_parameters_max_iterations: usize
```

Source: `src/higher_order_function.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The max number of times to call [HigherOrderUDFImpl::lambda_parameters](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-8a89b1f3117f6cf763ede484) before raising an error.
Used to guard against implementations that causes an infinite loop by endlessly returning
[LambdaParametersProgress::Partial](../operations/datafusion_expr.higher_order_function.LambdaParametersProgress.md#op-f0b9f54af5c1839b89e16765). Defaults to 256

<a id="op-3291e5696954973c0ec49aa6"></a>
## new

`function` · `datafusion_expr::higher_order_function::HigherOrderSignature::new` · datafusion-expr 55.1.0

```rust
fn new(type_signature: HigherOrderTypeSignature, volatility: Volatility) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderSignature", "path": "HigherOrderSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 1], "end": [160, 2], "filename": "src/higher_order_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/higher_order_function.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Creates a new `HigherOrderSignature` from a given type signature and volatility.

<a id="op-569881c73e4938c1e709d5a6"></a>
## partial_cmp

`function` · `datafusion_expr::higher_order_function::HigherOrderSignature::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &HigherOrderSignature) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderSignature", "path": "HigherOrderSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 39], "end": [89, 49], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/higher_order_function.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06a2a0d901e0ffa0d0dc3390"></a>
## type_signature

`struct_field` · `datafusion_expr::higher_order_function::HigherOrderSignature::type_signature` · datafusion-expr 55.1.0

```rust
type_signature: HigherOrderTypeSignature
```

Source: `src/higher_order_function.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The data types that the function accepts. See [HigherOrderTypeSignature](../operations/datafusion_expr.higher_order_function.HigherOrderTypeSignature.md#op-fbe23e3717bc225a891eae94) for more information.

<a id="op-4f2ad936ff25c549058b48f6"></a>
## user_defined

`function` · `datafusion_expr::higher_order_function::HigherOrderSignature::user_defined` · datafusion-expr 55.1.0

```rust
fn user_defined(volatility: Volatility) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderSignature", "path": "HigherOrderSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 1], "end": [160, 2], "filename": "src/higher_order_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/higher_order_function.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

User-defined coercion rules for the function.

<a id="op-625b7a3797e109214a21faa7"></a>
## variadic_any

`function` · `datafusion_expr::higher_order_function::HigherOrderSignature::variadic_any` · datafusion-expr 55.1.0

```rust
fn variadic_any(volatility: Volatility) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::HigherOrderSignature", "path": "HigherOrderSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [103, 1], "end": [160, 2], "filename": "src/higher_order_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/higher_order_function.rs:123`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

An arbitrary number of lambdas or arguments of any type.

<a id="op-a72dc21a35ae5e93524c12d4"></a>
## volatility

`struct_field` · `datafusion_expr::higher_order_function::HigherOrderSignature::volatility` · datafusion-expr 55.1.0

```rust
volatility: datafusion_expr_common::signature::Volatility
```

Source: `src/higher_order_function.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The volatility of the function. See [Volatility](../operations/datafusion_expr_common.signature.Volatility.md#op-2e27042945dba7db012302a0) for more information.
