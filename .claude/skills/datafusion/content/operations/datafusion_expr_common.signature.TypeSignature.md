# `datafusion_expr_common::signature::TypeSignature`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.signature.TypeSignature.json).

<a id="op-569c9180a7cabd6afe650cff"></a>
## TypeSignature

`enum` · `datafusion_expr_common::signature::TypeSignature` · datafusion-expr-common 55.1.0

```rust
enum TypeSignature
```

Source: `src/signature.rs:157`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

The types of arguments for which a function has implementations.

[`TypeSignature`](../operations/datafusion_expr_common.signature.TypeSignature.md#op-569c9180a7cabd6afe650cff) **DOES NOT** define the types that a user query could call the
function with. DataFusion will automatically coerce (cast) argument types to
one of the supported function signatures, if possible.

# Overview
Functions typically provide implementations for a small number of different
argument [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c)s, rather than all possible combinations. If a user
calls a function with arguments that do not match any of the declared types,
DataFusion will attempt to automatically coerce (add casts to) function
arguments so they match the [`TypeSignature`](../operations/datafusion_expr_common.signature.TypeSignature.md#op-569c9180a7cabd6afe650cff). See the [`type_coercion`] module
for more details

# Example: Numeric Functions
For example, a function like `cos` may only provide an implementation for
[`DataType::Float64`](../operations/arrow_schema.datatype.DataType.md#op-c7ea9edacf7b91c7728828e8). When users call `cos` with a different argument type,
such as `cos(int_column)`, and type coercion automatically adds a cast such
as `cos(CAST int_column AS DOUBLE)` during planning.

[`type_coercion`]: crate::type_coercion

## Example: Strings

There are several different string types in Arrow, such as
[`DataType::Utf8`](../operations/arrow_schema.datatype.DataType.md#op-4e52d4ade5cb975c8f4c452a), [`DataType::LargeUtf8`](../operations/arrow_schema.datatype.DataType.md#op-d897e1ec91191af958fa69ca), and [`DataType::Utf8View`](../operations/arrow_schema.datatype.DataType.md#op-a3c8435fd0f9a132833a8fc0).

Some functions may have specialized implementations for these types, while others
may be able to handle only one of them. For example, a function that
only works with [`DataType::Utf8View`](../operations/arrow_schema.datatype.DataType.md#op-a3c8435fd0f9a132833a8fc0) would have the following signature:

```
# use arrow::datatypes::DataType;
# use datafusion_expr_common::signature::{TypeSignature};
// Declares the function must be invoked with a single argument of type `Utf8View`.
// if a user calls the function with `Utf8` or `LargeUtf8`, DataFusion will
// automatically add a cast to `Utf8View` during planning.
let type_signature = TypeSignature::Exact(vec![DataType::Utf8View]);
```

# Example: Timestamps

Types to match are represented using Arrow's [`DataType`](../operations/arrow_schema.datatype.DataType.md#op-bf69df5b14436e006d3a531c).  [`DataType::Timestamp`](../operations/arrow_schema.datatype.DataType.md#op-4311beb64d86f8afbc59cba2) has an optional variable
timezone specification. To specify a function can handle a timestamp with *ANY* timezone, use
the [`TIMEZONE_WILDCARD`](../operations/datafusion_expr_common.signature.TIMEZONE_WILDCARD.md#op-5c6a30f50b3ea94ff4230ef5). For example:

```
# use arrow::datatypes::{DataType, TimeUnit};
# use datafusion_expr_common::signature::{TIMEZONE_WILDCARD, TypeSignature};
let type_signature = TypeSignature::Exact(vec![
    // A nanosecond precision timestamp with ANY timezone
    // matches  Timestamp(Nanosecond, Some("+0:00"))
    // matches  Timestamp(Nanosecond, Some("+5:00"))
    // does not match  Timestamp(Nanosecond, None)
    DataType::Timestamp(TimeUnit::Nanosecond, Some(TIMEZONE_WILDCARD.into())),
]);
```

<a id="op-59a951c9a5e477e9ecb9974d"></a>
## Any

`variant` · `datafusion_expr_common::signature::TypeSignature::Any` · datafusion-expr-common 55.1.0

```rust
Any
```

Source: `src/signature.rs:219`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

One or more arguments of arbitrary types.

For functions that take no arguments (e.g. `random()`), use [`TypeSignature::Nullary`](../operations/datafusion_expr_common.signature.TypeSignature.md#op-924c5ed8b17703b662fd11c1).

<a id="op-302ce821e62498e3ca09bdb4"></a>
## ArraySignature

`variant` · `datafusion_expr_common::signature::TypeSignature::ArraySignature` · datafusion-expr-common 55.1.0

```rust
ArraySignature
```

Source: `src/signature.rs:231`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

A function that has an [`ArrayFunctionSignature`](../operations/datafusion_expr_common.signature.ArrayFunctionSignature.md#op-de5878d4c45db0c90f1ffaf5)

<a id="op-3487ca51800f476df4c4ab66"></a>
## Coercible

`variant` · `datafusion_expr_common::signature::TypeSignature::Coercible` · datafusion-expr-common 55.1.0

```rust
Coercible
```

Source: `src/signature.rs:195`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

One or more arguments belonging to the [`TypeSignatureClass`](../operations/datafusion_expr_common.signature.TypeSignatureClass.md#op-dffc1e98d93d952fdb6f02ca), in order.

[`Coercion`](../operations/datafusion_expr_common.signature.Coercion.md#op-4db7bfb3816e2d62df923c32) contains not only the desired type but also the allowed
casts. For example, if you expect a function has string type, but you
also allow it to be casted from binary type.

For functions that take no arguments (e.g. `random()`), see [`TypeSignature::Nullary`](../operations/datafusion_expr_common.signature.TypeSignature.md#op-924c5ed8b17703b662fd11c1).

<a id="op-28dc1952ee76b7b1d843aba1"></a>
## Comparable

`variant` · `datafusion_expr_common::signature::TypeSignature::Comparable` · datafusion-expr-common 55.1.0

```rust
Comparable
```

Source: `src/signature.rs:215`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

One or more arguments coercible to a single, comparable type.

Each argument will be coerced to a single type using the
coercion rules described in [`comparison_coercion`].

# Examples

If the `nullif(1, 2)` function is called with `i32` and `i64` arguments
the types will both be coerced to `i64` before the function is invoked.

If the `nullif('1', 2)` function is called with `Utf8` and `i64` arguments
the types will both be coerced to `Int64` before the function is invoked
(numeric is preferred over string).

Note:
- For functions that take no arguments (e.g. `random()`), see [`TypeSignature::Nullary`](../operations/datafusion_expr_common.signature.TypeSignature.md#op-924c5ed8b17703b662fd11c1).
- If all arguments have type [`DataType::Null`](../operations/arrow_schema.datatype.DataType.md#op-0e6b89945b6e61141a2fa7ee), they are coerced to `Utf8`

[`comparison_coercion`]: crate::type_coercion::binary::comparison_coercion

<a id="op-bde9c2e3d919655f2b2cc5bf"></a>
## Exact

`variant` · `datafusion_expr_common::signature::TypeSignature::Exact` · datafusion-expr-common 55.1.0

```rust
Exact
```

Source: `src/signature.rs:187`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

One or more arguments with exactly the specified types in order.

For functions that take no arguments (e.g. `random()`), use [`TypeSignature::Nullary`](../operations/datafusion_expr_common.signature.TypeSignature.md#op-924c5ed8b17703b662fd11c1).

<a id="op-924c5ed8b17703b662fd11c1"></a>
## Nullary

`variant` · `datafusion_expr_common::signature::TypeSignature::Nullary` · datafusion-expr-common 55.1.0

```rust
Nullary
```

Source: `src/signature.rs:252`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No arguments

<a id="op-2d66132a97f901280e0cf788"></a>
## Numeric

`variant` · `datafusion_expr_common::signature::TypeSignature::Numeric` · datafusion-expr-common 55.1.0

```rust
Numeric
```

Source: `src/signature.rs:239`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

One or more arguments of numeric types, coerced to a common numeric type.

See [`NativeType::is_numeric`] to know which type is considered numeric

For functions that take no arguments (e.g. `random()`), use [`TypeSignature::Nullary`](../operations/datafusion_expr_common.signature.TypeSignature.md#op-924c5ed8b17703b662fd11c1).

[`NativeType::is_numeric`]: datafusion_common::types::NativeType::is_numeric

Unresolved upstream links (retained, not inferred): `datafusion_common::types::NativeType::is_numeric`.

<a id="op-dace4a06957ffe853cadab3c"></a>
## OneOf

`variant` · `datafusion_expr_common::signature::TypeSignature::OneOf` · datafusion-expr-common 55.1.0

```rust
OneOf
```

Source: `src/signature.rs:229`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Matches exactly one of a list of [`TypeSignature`](../operations/datafusion_expr_common.signature.TypeSignature.md#op-569c9180a7cabd6afe650cff)s.

Coercion is attempted to match the signatures in order, and stops after
the first success, if any.

# Examples

Since `make_array` takes 0 or more arguments with arbitrary types, its `TypeSignature`
is `OneOf(vec![Any(0), VariadicAny])`.

<a id="op-b23eb94d5d0fc95e97c5b48a"></a>
## String

`variant` · `datafusion_expr_common::signature::TypeSignature::String` · datafusion-expr-common 55.1.0

```rust
String
```

Source: `src/signature.rs:250`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

One or arguments of all the same string types.

The precedence of type from high to low is Utf8View, LargeUtf8 and Utf8.
Null is considered as `Utf8` by default
Dictionary with string value type is also handled.

For example, if a function is called with (utf8, large_utf8), all
arguments will be coerced to  `LargeUtf8`

For functions that take no arguments (e.g. `random()`), use [`TypeSignature::Nullary`](../operations/datafusion_expr_common.signature.TypeSignature.md#op-924c5ed8b17703b662fd11c1).

<a id="op-2c3acbe938ab29bf5167f8a2"></a>
## Uniform

`variant` · `datafusion_expr_common::signature::TypeSignature::Uniform` · datafusion-expr-common 55.1.0

```rust
Uniform
```

Source: `src/signature.rs:183`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

One or more arguments of an arbitrary but equal type out of a list of valid types.

# Examples

1. A function of one argument of f64 is `Uniform(1, vec![DataType::Float64])`
2. A function of one argument of f64 or f32 is `Uniform(1, vec![DataType::Float32, DataType::Float64])`

<a id="op-9a54792c384e19dbdaacb169"></a>
## UserDefined

`variant` · `datafusion_expr_common::signature::TypeSignature::UserDefined` · datafusion-expr-common 55.1.0

```rust
UserDefined
```

Source: `src/signature.rs:174`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

The acceptable signature and coercions rules are special for this
function.

If this signature is specified,
DataFusion will call [`ScalarUDFImpl::coerce_types`] to prepare argument types.

[`ScalarUDFImpl::coerce_types`]: https://docs.rs/datafusion/latest/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.coerce_types

<a id="op-7aac02d9b19ab50aa0d494c0"></a>
## Variadic

`variant` · `datafusion_expr_common::signature::TypeSignature::Variadic` · datafusion-expr-common 55.1.0

```rust
Variadic
```

Source: `src/signature.rs:166`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

One or more arguments of a common type out of a list of valid types.

For functions that take no arguments (e.g. `random()`), see [`TypeSignature::Nullary`](../operations/datafusion_expr_common.signature.TypeSignature.md#op-924c5ed8b17703b662fd11c1).

# Examples

A function such as `concat` is `Variadic(vec![DataType::Utf8,
DataType::LargeUtf8])`

<a id="op-126baa1b7dc89216b0e10e58"></a>
## VariadicAny

`variant` · `datafusion_expr_common::signature::TypeSignature::VariadicAny` · datafusion-expr-common 55.1.0

```rust
VariadicAny
```

Source: `src/signature.rs:176`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

One or more arguments with arbitrary types

<a id="op-b16502081975454fcd2a9ad7"></a>
## arity

`function` · `datafusion_expr_common::signature::TypeSignature::arity` · datafusion-expr-common 55.1.0

```rust
fn arity(&self) -> Arity
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::TypeSignature", "path": "TypeSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [323, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:279`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Returns the arity (expected number of arguments) for this type signature.

Returns `Arity::Fixed(n)` for signatures with a specific argument count,
or `Arity::Variable` for variable-arity signatures like `Variadic`, `VariadicAny`, `UserDefined`.

# Examples

```
# use datafusion_expr_common::signature::{TypeSignature, Arity};
# use arrow::datatypes::DataType;
// Exact signature has fixed arity
let sig = TypeSignature::Exact(vec![DataType::Int32, DataType::Utf8]);
assert_eq!(sig.arity(), Arity::Fixed(2));

// Variadic signature has variable arity
let sig = TypeSignature::VariadicAny;
assert_eq!(sig.arity(), Arity::Variable);
```

<a id="op-c2d68c6deb3ffa80e98e359b"></a>
## clone

`function` · `datafusion_expr_common::signature::TypeSignature::clone` · datafusion-expr-common 55.1.0

```rust
fn clone(&self) -> TypeSignature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::TypeSignature", "path": "TypeSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 17], "end": [156, 22], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/signature.rs:156`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a147419749cc6f6cdd24092d"></a>
## eq

`function` · `datafusion_expr_common::signature::TypeSignature::eq` · datafusion-expr-common 55.1.0

```rust
fn eq(&self, other: &TypeSignature) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::TypeSignature", "path": "TypeSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 24], "end": [156, 33], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/signature.rs:156`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c086d214f7282e7b18668ec"></a>
## fmt

`function` · `datafusion_expr_common::signature::TypeSignature::fmt` · datafusion-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::TypeSignature", "path": "TypeSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [325, 1], "end": [360, 2], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/signature.rs:326`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3ba48e39efc01fe0fbb43ad"></a>
## fmt

`function` · `datafusion_expr_common::signature::TypeSignature::fmt` · datafusion-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::TypeSignature", "path": "TypeSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 10], "end": [156, 15], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/signature.rs:156`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c8dbe8e6c7b5c720e8c6a4c"></a>
## get_example_types

`function` · `datafusion_expr_common::signature::TypeSignature::get_example_types` · datafusion-expr-common 55.1.0

```rust
fn get_example_types(&self) -> Vec<Vec<DataType>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::TypeSignature", "path": "TypeSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [612, 1], "end": [943, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:889`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Return example acceptable types for this `TypeSignature`'

Returns a `Vec<DataType>` for each argument to the function

This is used for `information_schema` and can be used to generate
documentation or error messages.

<a id="op-38a20915f6795a8983a7b3eb"></a>
## hash

`function` · `datafusion_expr_common::signature::TypeSignature::hash` · datafusion-expr-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::TypeSignature", "path": "TypeSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 51], "end": [156, 55], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/signature.rs:156`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-552e7e66be97ef1edb3180b0"></a>
## is_one_of

`function` · `datafusion_expr_common::signature::TypeSignature::is_one_of` · datafusion-expr-common 55.1.0

```rust
fn is_one_of(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::TypeSignature", "path": "TypeSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [255, 1], "end": [323, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:257`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d90eb3bb03e6844459cf649d"></a>
## join_types

`function` · `datafusion_expr_common::signature::TypeSignature::join_types` · datafusion-expr-common 55.1.0

```rust
fn join_types<T: Display>(types: &[T], delimiter: &str) -> String
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::TypeSignature", "path": "TypeSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [612, 1], "end": [943, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:854`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Helper function to join types with specified delimiter.

<a id="op-0277b6fd0517ffc32ef46bb2"></a>
## partial_cmp

`function` · `datafusion_expr_common::signature::TypeSignature::partial_cmp` · datafusion-expr-common 55.1.0

```rust
fn partial_cmp(&self, other: &TypeSignature) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::TypeSignature", "path": "TypeSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 39], "end": [156, 49], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/signature.rs:156`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-83f12f6b544a89a058f9b7c1"></a>
## supports_zero_argument

`function` · `datafusion_expr_common::signature::TypeSignature::supports_zero_argument` · datafusion-expr-common 55.1.0

```rust
fn supports_zero_argument(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::TypeSignature", "path": "TypeSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [612, 1], "end": [943, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:863`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Check whether 0 input argument is valid for given `TypeSignature`

<a id="op-f84c54ec026c87f2bd59e8ef"></a>
## to_string_repr

`function` · `datafusion_expr_common::signature::TypeSignature::to_string_repr` · datafusion-expr-common 55.1.0

```rust
fn to_string_repr(&self) -> Vec<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::TypeSignature", "path": "TypeSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [612, 1], "end": [943, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:613`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af681b941c5c8ad139b99c44"></a>
## to_string_repr_with_names

`function` · `datafusion_expr_common::signature::TypeSignature::to_string_repr_with_names` · datafusion-expr-common 55.1.0

```rust
fn to_string_repr_with_names(&self, parameter_names: Option<&[String]>) -> Vec<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::TypeSignature", "path": "TypeSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [612, 1], "end": [943, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:688`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Return string representation of the function signature with parameter names.

This method is similar to [`Self::to_string_repr`](../operations/datafusion_expr_common.signature.TypeSignature.md#op-f84c54ec026c87f2bd59e8ef) but uses parameter names
instead of types when available. This is useful for generating more helpful
error messages.

# Arguments
* `parameter_names` - Optional slice of parameter names. When provided, these
  names will be used instead of type names in the output.

# Examples
```
# use datafusion_expr_common::signature::TypeSignature;
# use arrow::datatypes::DataType;
let sig = TypeSignature::Exact(vec![DataType::Int32, DataType::Utf8]);

// Without names: shows types only
assert_eq!(sig.to_string_repr_with_names(None), vec!["Int32, Utf8"]);

// With names: shows parameter names with types
assert_eq!(
    sig.to_string_repr_with_names(Some(&["id".to_string(), "name".to_string()])),
    vec!["id: Int32, name: Utf8"]
);
```

<a id="op-c3f3bdb542803c8dcaf56ee0"></a>
## used_to_support_zero_arguments

`function` · `datafusion_expr_common::signature::TypeSignature::used_to_support_zero_arguments` · datafusion-expr-common 55.1.0

```rust
fn used_to_support_zero_arguments(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::TypeSignature", "path": "TypeSignature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [612, 1], "end": [943, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:876`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Returns true if the signature currently supports or used to supported 0
input arguments in a previous version of DataFusion.
