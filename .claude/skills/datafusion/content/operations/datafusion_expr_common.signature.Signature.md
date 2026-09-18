# `datafusion_expr_common::signature::Signature`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.signature.Signature.json).

<a id="op-3501f77593554c0ab3e03e9e"></a>
## Signature

`struct` · `datafusion_expr_common::signature::Signature` · datafusion-expr-common 55.1.0

```rust
struct Signature
```

Source: `src/signature.rs:1274`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Provides  information necessary for calling a function.

- [`TypeSignature`](../operations/datafusion_expr_common.signature.TypeSignature.md#op-569c9180a7cabd6afe650cff) defines the argument types that a function has implementations
  for.

- [`Volatility`](../operations/datafusion_expr_common.signature.Volatility.md#op-2e27042945dba7db012302a0) defines how the output of the function changes with the input.

<a id="op-bd6a24038816f9730befe3bf"></a>
## any

`function` · `datafusion_expr_common::signature::Signature::any` · datafusion-expr-common 55.1.0

```rust
fn any(arg_count: usize, volatility: Volatility) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Signature", "path": "Signature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1288, 1], "end": [1560, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:1388`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

A specified number of arguments of any type

<a id="op-f673868a6f70bc658879bd00"></a>
## array

`function` · `datafusion_expr_common::signature::Signature::array` · datafusion-expr-common 55.1.0

```rust
fn array(volatility: Volatility) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Signature", "path": "Signature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1288, 1], "end": [1560, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:1500`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Specialized [Signature](../operations/datafusion_expr_common.signature.Signature.md#op-3501f77593554c0ab3e03e9e) for ArrayEmpty and similar functions.

<a id="op-d74421bfa2817531ab2878ad"></a>
## array_and_element

`function` · `datafusion_expr_common::signature::Signature::array_and_element` · datafusion-expr-common 55.1.0

```rust
fn array_and_element(volatility: Volatility) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Signature", "path": "Signature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1288, 1], "end": [1560, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:1406`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Specialized [Signature](../operations/datafusion_expr_common.signature.Signature.md#op-3501f77593554c0ab3e03e9e) for ArrayAppend and similar functions.

<a id="op-de1ced711371d28f1a7db55a"></a>
## array_and_element_and_optional_index

`function` · `datafusion_expr_common::signature::Signature::array_and_element_and_optional_index` · datafusion-expr-common 55.1.0

```rust
fn array_and_element_and_optional_index(volatility: Volatility) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Signature", "path": "Signature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1288, 1], "end": [1560, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:1458`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Specialized [Signature](../operations/datafusion_expr_common.signature.Signature.md#op-3501f77593554c0ab3e03e9e) for Array functions with an optional index.

<a id="op-4b2cbf1457d7a1ac4a3d4e6a"></a>
## array_and_index

`function` · `datafusion_expr_common::signature::Signature::array_and_index` · datafusion-expr-common 55.1.0

```rust
fn array_and_index(volatility: Volatility) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Signature", "path": "Signature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1288, 1], "end": [1560, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:1483`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Specialized [Signature](../operations/datafusion_expr_common.signature.Signature.md#op-3501f77593554c0ab3e03e9e) for ArrayElement and similar functions.

<a id="op-e4d14e8115f56a3b8300c753"></a>
## arrays

`function` · `datafusion_expr_common::signature::Signature::arrays` · datafusion-expr-common 55.1.0

```rust
fn arrays(n: usize, coercion: Option<ListCoercion>, volatility: Volatility) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Signature", "path": "Signature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1288, 1], "end": [1560, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:1440`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Specialized [Signature](../operations/datafusion_expr_common.signature.Signature.md#op-3501f77593554c0ab3e03e9e) for functions that take a fixed number of arrays.

<a id="op-5899d285ff8420d76da36540"></a>
## clone

`function` · `datafusion_expr_common::signature::Signature::clone` · datafusion-expr-common 55.1.0

```rust
fn clone(&self) -> Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Signature", "path": "Signature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1273, 17], "end": [1273, 22], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/signature.rs:1273`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86b5f3f02743366b8fbfab15"></a>
## coercible

`function` · `datafusion_expr_common::signature::Signature::coercible` · datafusion-expr-common 55.1.0

```rust
fn coercible(target_types: Vec<Coercion>, volatility: Volatility) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Signature", "path": "Signature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1288, 1], "end": [1560, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:1362`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Target coerce types in order

<a id="op-f459663e6031c48703f37ac7"></a>
## comparable

`function` · `datafusion_expr_common::signature::Signature::comparable` · datafusion-expr-common 55.1.0

```rust
fn comparable(arg_count: usize, volatility: Volatility) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Signature", "path": "Signature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1288, 1], "end": [1560, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:1371`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Used for function that expects comparable data types, it will try to coerced all the types into single final one.

<a id="op-c6614af35796c2248f8ddde6"></a>
## element_and_array

`function` · `datafusion_expr_common::signature::Signature::element_and_array` · datafusion-expr-common 55.1.0

```rust
fn element_and_array(volatility: Volatility) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Signature", "path": "Signature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1288, 1], "end": [1560, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:1423`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Specialized [Signature](../operations/datafusion_expr_common.signature.Signature.md#op-3501f77593554c0ab3e03e9e) for ArrayPrepend and similar functions.

<a id="op-d7e061f3a5747fc91cfa302d"></a>
## eq

`function` · `datafusion_expr_common::signature::Signature::eq` · datafusion-expr-common 55.1.0

```rust
fn eq(&self, other: &Signature) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Signature", "path": "Signature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1273, 24], "end": [1273, 33], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/signature.rs:1273`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25eb4efab18b1f41d1bde24d"></a>
## exact

`function` · `datafusion_expr_common::signature::Signature::exact` · datafusion-expr-common 55.1.0

```rust
fn exact(exact_types: Vec<DataType>, volatility: Volatility) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Signature", "path": "Signature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1288, 1], "end": [1560, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:1353`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Exactly matches the types in `exact_types`, in order.

<a id="op-fc955383dd9da69fb885bd03"></a>
## fmt

`function` · `datafusion_expr_common::signature::Signature::fmt` · datafusion-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Signature", "path": "Signature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1273, 10], "end": [1273, 15], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/signature.rs:1273`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-53ce7006fb3f3d306586a110"></a>
## hash

`function` · `datafusion_expr_common::signature::Signature::hash` · datafusion-expr-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Signature", "path": "Signature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1273, 51], "end": [1273, 55], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/signature.rs:1273`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad17f85fb51a38323fcf91ac"></a>
## new

`function` · `datafusion_expr_common::signature::Signature::new` · datafusion-expr-common 55.1.0

```rust
fn new(type_signature: TypeSignature, volatility: Volatility) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Signature", "path": "Signature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1288, 1], "end": [1560, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:1290`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Creates a new Signature from a given type signature and volatility.

<a id="op-695406a2dac2323814d610a8"></a>
## nullary

`function` · `datafusion_expr_common::signature::Signature::nullary` · datafusion-expr-common 55.1.0

```rust
fn nullary(volatility: Volatility) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Signature", "path": "Signature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1288, 1], "end": [1560, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:1379`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a65dba71d9a2bb771ad2741"></a>
## numeric

`function` · `datafusion_expr_common::signature::Signature::numeric` · datafusion-expr-common 55.1.0

```rust
fn numeric(arg_count: usize, volatility: Volatility) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Signature", "path": "Signature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1288, 1], "end": [1560, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:1315`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

A specified number of numeric arguments

<a id="op-9b92cc1c56cc5d345dd95ab5"></a>
## one_of

`function` · `datafusion_expr_common::signature::Signature::one_of` · datafusion-expr-common 55.1.0

```rust
fn one_of(type_signatures: Vec<TypeSignature>, volatility: Volatility) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Signature", "path": "Signature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1288, 1], "end": [1560, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:1397`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Any one of a list of [TypeSignature](../operations/datafusion_expr_common.signature.TypeSignature.md#op-569c9180a7cabd6afe650cff)s.

<a id="op-5c26d7fef409abaf76298679"></a>
## parameter_names

`struct_field` · `datafusion_expr_common::signature::Signature::parameter_names` · datafusion-expr-common 55.1.0

```rust
parameter_names: Option<Vec<String>>
```

Source: `src/signature.rs:1285`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Optional parameter names for the function arguments.

If provided, enables named argument notation for function calls (e.g., `func(a => 1, b => 2)`).
The length must match the number of arguments defined by `type_signature`.

Defaults to `None`, meaning only positional arguments are supported.

<a id="op-0478998c4a02d97d23f1f918"></a>
## partial_cmp

`function` · `datafusion_expr_common::signature::Signature::partial_cmp` · datafusion-expr-common 55.1.0

```rust
fn partial_cmp(&self, other: &Signature) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Signature", "path": "Signature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1273, 39], "end": [1273, 49], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/signature.rs:1273`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49979beed3080f662e3f462f"></a>
## string

`function` · `datafusion_expr_common::signature::Signature::string` · datafusion-expr-common 55.1.0

```rust
fn string(arg_count: usize, volatility: Volatility) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Signature", "path": "Signature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1288, 1], "end": [1560, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:1324`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

A specified number of string arguments

<a id="op-da2b03d1fd952dfaf566f807"></a>
## type_signature

`struct_field` · `datafusion_expr_common::signature::Signature::type_signature` · datafusion-expr-common 55.1.0

```rust
type_signature: TypeSignature
```

Source: `src/signature.rs:1276`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

The data types that the function accepts. See [TypeSignature](../operations/datafusion_expr_common.signature.TypeSignature.md#op-569c9180a7cabd6afe650cff) for more information.

<a id="op-3127fbaba5148f076194a4aa"></a>
## uniform

`function` · `datafusion_expr_common::signature::Signature::uniform` · datafusion-expr-common 55.1.0

```rust
fn uniform(arg_count: usize, valid_types: Vec<DataType>, volatility: Volatility) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Signature", "path": "Signature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1288, 1], "end": [1560, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:1341`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

A fixed number of arguments of the same type, from those listed in `valid_types`.

<a id="op-881193ac8beda48e60a62974"></a>
## user_defined

`function` · `datafusion_expr_common::signature::Signature::user_defined` · datafusion-expr-common 55.1.0

```rust
fn user_defined(volatility: Volatility) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Signature", "path": "Signature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1288, 1], "end": [1560, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:1306`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

User-defined coercion rules for the function.

<a id="op-95fbe177007cf6ebbc4c56cb"></a>
## variadic

`function` · `datafusion_expr_common::signature::Signature::variadic` · datafusion-expr-common 55.1.0

```rust
fn variadic(common_types: Vec<DataType>, volatility: Volatility) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Signature", "path": "Signature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1288, 1], "end": [1560, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:1298`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

An arbitrary number of arguments with the same type, from those listed in `common_types`.

<a id="op-64743bf9a87fc506f4d2d5bb"></a>
## variadic_any

`function` · `datafusion_expr_common::signature::Signature::variadic_any` · datafusion-expr-common 55.1.0

```rust
fn variadic_any(volatility: Volatility) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Signature", "path": "Signature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1288, 1], "end": [1560, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:1333`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

An arbitrary number of arguments of any type.

<a id="op-a89faaa38785d6bb9cb4093b"></a>
## volatility

`struct_field` · `datafusion_expr_common::signature::Signature::volatility` · datafusion-expr-common 55.1.0

```rust
volatility: Volatility
```

Source: `src/signature.rs:1278`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

The volatility of the function. See [Volatility](../operations/datafusion_expr_common.signature.Volatility.md#op-2e27042945dba7db012302a0) for more information.

<a id="op-deb7d27f0fe8318138a2cda4"></a>
## with_parameter_names

`function` · `datafusion_expr_common::signature::Signature::with_parameter_names` · datafusion-expr-common 55.1.0

```rust
fn with_parameter_names(self, names: Vec<impl Into<String>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Signature", "path": "Signature"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1288, 1], "end": [1560, 2], "filename": "src/signature.rs"}, "trait": null, "trait_path": null}`

Source: `src/signature.rs:1519`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Add parameter names to this signature, enabling named argument notation.

# Example
```
# use datafusion_expr_common::signature::{Signature, Volatility};
# use arrow::datatypes::DataType;
let sig =
    Signature::exact(vec![DataType::Int32, DataType::Utf8], Volatility::Immutable)
        .with_parameter_names(vec!["count".to_string(), "name".to_string()]);
```

# Errors
Returns an error if the number of parameter names doesn't match the signature's arity.
For signatures with variable arity (e.g., `Variadic`, `VariadicAny`), parameter names
cannot be specified.
