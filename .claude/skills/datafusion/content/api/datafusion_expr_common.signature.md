# `datafusion_expr_common::signature`

Crate `datafusion-expr-common` · 12 public items · structured records in [`model/datafusion_expr_common.signature.json`](../model/datafusion_expr_common.signature.json)

## FIXED_SIZE_LIST_WILDCARD

`constant` · `datafusion_expr_common::signature::FIXED_SIZE_LIST_WILDCARD`

```rust
const FIXED_SIZE_LIST_WILDCARD: i32 = i32::MIN
```

Constant that is used as a placeholder for any valid fixed size list.
This is used where a function can accept a fixed size list type with any
valid length. It exists to avoid the need to enumerate all possible fixed size list lengths.

---

## TIMEZONE_WILDCARD

`constant` · `datafusion_expr_common::signature::TIMEZONE_WILDCARD`

Also reachable as `datafusion::logical_expr::TIMEZONE_WILDCARD`, `datafusion_expr::TIMEZONE_WILDCARD`

```rust
const TIMEZONE_WILDCARD: &str = "+TZ"
```

Constant that is used as a placeholder for any valid timezone.
This is used where a function can accept a timestamp type with any
valid timezone, it exists to avoid the need to enumerate all possible
timezones. See [`TypeSignature`] for more details.

Type coercion always ensures that functions will be executed using
timestamp arrays that have a valid time zone. Functions must never
return results with this timezone.

---

## Arity

`enum` · `datafusion_expr_common::signature::Arity`

```rust
enum Arity
```

**Variants**: `Fixed`, `Variable`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

Represents the arity (number of arguments) of a function signature

---

## ArrayFunctionArgument

`enum` · `datafusion_expr_common::signature::ArrayFunctionArgument`

Also reachable as `datafusion::logical_expr::ArrayFunctionArgument`, `datafusion_expr::ArrayFunctionArgument`

```rust
enum ArrayFunctionArgument
```

**Variants**: `Element`, `Index`, `Array`, `String`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

---

## ArrayFunctionSignature

`enum` · `datafusion_expr_common::signature::ArrayFunctionSignature`

Also reachable as `datafusion::logical_expr::ArrayFunctionSignature`, `datafusion_expr::ArrayFunctionSignature`

```rust
enum ArrayFunctionSignature
```

**Variants**: `Array`, `RecursiveArray`, `MapArray`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

---

## Coercion

`enum` · `datafusion_expr_common::signature::Coercion`

Also reachable as `datafusion::logical_expr::Coercion`, `datafusion_expr::Coercion`

```rust
enum Coercion
```

**Variants**: `Exact`, `Implicit`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd

**Methods** (8)

```rust
fn allowed_source_types(&self) -> &[TypeSignatureClass]
fn default_casted_type(&self) -> Option<&NativeType>
fn desired_type(&self) -> &TypeSignatureClass
fn encoding_preservation(&self) -> EncodingPreservation
fn implicit_coercion(&self) -> Option<&ImplicitCoercion>
fn new_exact(desired_type: TypeSignatureClass) -> Self
fn new_implicit(desired_type: TypeSignatureClass, allowed_source_types: Vec<TypeSignatureClass>, default_casted_type: NativeType) -> Self
fn with_encoding_preservation(self, encoding_preservation: EncodingPreservation) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Represents type coercion rules for function arguments, specifying both the desired type
and optional implicit coercion rules for source types.

# Examples

```
use datafusion_common::types::{logical_binary, logical_string, NativeType};
use datafusion_expr_common::signature::{Coercion, TypeSignatureClass};

// Exact coercion that only accepts timestamp types
let exact = Coercion::new_exact(TypeSignatureClass::Timestamp);

// Implicit coercion that accepts string types but can coerce from binary types
let implicit = Coercion::new_implicit(
    TypeSignatureClass::Native(logical_string()),
    vec![TypeSignatureClass::Native(logical_binary())],
    NativeType::String,
);
```

There are two variants:

* `Exact` - Only accepts arguments that exactly match the desired type
* `Implicit` - Accepts the desired type and can coerce from specified source types

---

## TypeSignature

`enum` · `datafusion_expr_common::signature::TypeSignature`

Also reachable as `datafusion::logical_expr::TypeSignature`, `datafusion_expr::TypeSignature`

```rust
enum TypeSignature
```

**Variants**: `Variadic`, `UserDefined`, `VariadicAny`, `Uniform`, `Exact`, `Coercible`, `Comparable`, `Any`, `OneOf`, `ArraySignature`, `Numeric`, `String`, `Nullary`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (8)

```rust
fn arity(&self) -> Arity
fn get_example_types(&self) -> Vec<Vec<DataType>>
fn is_one_of(&self) -> bool
fn join_types<T: Display>(types: &[T], delimiter: &str) -> String
fn supports_zero_argument(&self) -> bool
fn to_string_repr(&self) -> Vec<String>
fn to_string_repr_with_names(&self, parameter_names: Option<&[String]>) -> Vec<String>
fn used_to_support_zero_arguments(&self) -> bool
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

The types of arguments for which a function has implementations.

[`TypeSignature`] **DOES NOT** define the types that a user query could call the
function with. DataFusion will automatically coerce (cast) argument types to
one of the supported function signatures, if possible.

# Overview
Functions typically provide implementations for a small number of different
argument [`DataType`]s, rather than all possible combinations. If a user
calls a function with arguments that do not match any of the declared types,
DataFusion will attempt to automatically coerce (add casts to) function
arguments so they match the [`TypeSignature`]. See the [`type_coercion`] module
for more details

# Example: Numeric Functions
For example, a function like `cos` may only provide an implementation for
[`DataType::Float64`]. When users call `cos` with a different argument type,
such as `cos(int_column)`, and type coercion automatically adds a cast such
as `cos(CAST int_column AS DOUBLE)` during planning.

[`type_coercion`]: crate::type_coercion

## Example: Strings

There are several different string types in Arrow, such as
[`DataType::Utf8`], [`DataType::LargeUtf8`], and [`DataType::Utf8View`].

Some functions may have specialized implementations for these types, while others
may be able to handle only one of them. For example, a function that
only works with [`DataType::Utf8View`] would have the following signature:

```
# use arrow::datatypes::DataType;
# use datafusion_expr_common::signature::{TypeSignature};
// Declares the function must be invoked with a single argument of type `Utf8View`.
// if a user calls the function with `Utf8` or `LargeUtf8`, DataFusion will
// automatically add a cast to `Utf8View` during planning.
let type_signature = TypeSignature::Exact(vec![DataType::Utf8View]);
```

# Example: Timestamps

Types to match are represented using Arrow's [`DataType`].  [`DataType::Timestamp`] has an optional variable
timezone specification. To specify a function can handle a timestamp with *ANY* timezone, use
the [`TIMEZONE_WILDCARD`]. For example:

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

---

## TypeSignatureClass

`enum` · `datafusion_expr_common::signature::TypeSignatureClass`

Also reachable as `datafusion::logical_expr::TypeSignatureClass`, `datafusion_expr::TypeSignatureClass`

```rust
enum TypeSignatureClass
```

**Variants**: `Any`, `Timestamp`, `Time`, `Interval`, `Duration`, `Native`, `Integer`, `Float`, `Decimal`, `Numeric`, `Binary`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (2)

```rust
fn default_casted_type(&self, native_type: &NativeType, origin_type: &DataType) -> Result<DataType>
fn matches_native_type(&self, logical_type: &NativeType) -> bool
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Represents the class of types that can be used in a function signature.

This is used to specify what types are valid for function arguments in a more flexible way than
just listing specific DataTypes. For example, TypeSignatureClass::Timestamp matches any timestamp
type regardless of timezone or precision.

Used primarily with [`TypeSignature::Coercible`] to define function signatures that can accept
arguments that can be coerced to a particular class of types.

---

## Volatility

`enum` · `datafusion_expr_common::signature::Volatility`

Also reachable as `datafusion::logical_expr::Volatility`, `datafusion_expr::Volatility`

```rust
enum Volatility
```

**Variants**: `Immutable`, `Stable`, `Volatile`

**Implements**: `core::convert::From`

**Derives**: Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, StructuralPartialEq

How a function's output changes with respect to a fixed input

The volatility of a function determines eligibility for certain
optimizations. You should always define your function to have the strictest
possible volatility to maximize performance and avoid unexpected
results.

---

## EncodingPreservation

`struct` · `datafusion_expr_common::signature::EncodingPreservation`

Also reachable as `datafusion::logical_expr::EncodingPreservation`, `datafusion_expr::EncodingPreservation`

```rust
struct EncodingPreservation
```

**Derives**: Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (3)

```rust
const fn dictionary() -> Self
const fn preserve_dictionary(self) -> bool
const fn with_dictionary(self) -> Self
```

Controls whether a [`Coercion`] preserves an argument's physical encoding
(e.g. dictionary) instead of materializing it to the coerced value type.

---

## ImplicitCoercion

`struct` · `datafusion_expr_common::signature::ImplicitCoercion`

```rust
struct ImplicitCoercion
```

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Defines rules for implicit type coercion, specifying which source types can be
coerced and the default type to use when coercing.

This is used by functions to specify which types they can accept via implicit
coercion in addition to their primary desired type.

# Examples

```
use arrow::datatypes::TimeUnit;

use datafusion_expr_common::signature::{Coercion, ImplicitCoercion, TypeSignatureClass};
use datafusion_common::types::{NativeType, logical_binary};

// Allow coercing from binary types to timestamp, coerce to specific timestamp unit and timezone
let implicit = Coercion::new_implicit(
    TypeSignatureClass::Timestamp,
    vec![TypeSignatureClass::Native(logical_binary())],
    NativeType::Timestamp(TimeUnit::Second, None),
);
```

---

## Signature

`struct` · `datafusion_expr_common::signature::Signature`

Also reachable as `datafusion::logical_expr::Signature`, `datafusion_expr::Signature`

```rust
struct Signature
```

**Fields**: `type_signature`, `volatility`, `parameter_names`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, PartialOrd, StructuralPartialEq

**Methods** (20)

```rust
fn any(arg_count: usize, volatility: Volatility) -> Self
fn array(volatility: Volatility) -> Self
fn array_and_element(volatility: Volatility) -> Self
fn array_and_element_and_optional_index(volatility: Volatility) -> Self
fn array_and_index(volatility: Volatility) -> Self
fn arrays(n: usize, coercion: Option<ListCoercion>, volatility: Volatility) -> Self
fn coercible(target_types: Vec<Coercion>, volatility: Volatility) -> Self
fn comparable(arg_count: usize, volatility: Volatility) -> Self
fn element_and_array(volatility: Volatility) -> Self
fn exact(exact_types: Vec<DataType>, volatility: Volatility) -> Self
fn new(type_signature: TypeSignature, volatility: Volatility) -> Self
fn nullary(volatility: Volatility) -> Self
fn numeric(arg_count: usize, volatility: Volatility) -> Self
fn one_of(type_signatures: Vec<TypeSignature>, volatility: Volatility) -> Self
fn string(arg_count: usize, volatility: Volatility) -> Self
fn uniform(arg_count: usize, valid_types: Vec<DataType>, volatility: Volatility) -> Self
fn user_defined(volatility: Volatility) -> Self
fn variadic(common_types: Vec<DataType>, volatility: Volatility) -> Self
fn variadic_any(volatility: Volatility) -> Self
fn with_parameter_names(self, names: Vec<impl Into<String>>) -> Result<Self>
```

Provides  information necessary for calling a function.

- [`TypeSignature`] defines the argument types that a function has implementations
  for.

- [`Volatility`] defines how the output of the function changes with the input.

---
