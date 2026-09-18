# `buoyant_kernel::transforms::schema::SchemaTransform`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.transforms.schema.SchemaTransform.json).

<a id="op-3cfb0c7f41e8328c3cb5b04f"></a>
## SchemaTransform

`trait` · `buoyant_kernel::transforms::schema::SchemaTransform` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
trait SchemaTransform<'a>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/schema.rs#L51).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs:51`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Generic framework for describing recursive bottom-up schema transforms.

The transform can start from whatever schema element is available
(e.g. [`Self::transform_struct`](../operations/buoyant_kernel.transforms.schema.SchemaTransform.md#op-dd9d6d72aa8a68a3d1829cdd) to start with [`StructType`](../operations/buoyant_kernel.schema.StructType.md#op-ac9d10e1b52f1ac2eacc4e98)), or it can start from the generic
[`Self::transform`](../operations/buoyant_kernel.transforms.schema.SchemaTransform.md#op-695cbebb8622aa52bf09188a).

The provided `transform_xxx` methods all default to no-op (usually by invoking the corresponding
recursive helper method), and implementations should selectively override specific
`transform_xxx` methods as needed for the task at hand.

# Recursive helper methods

The provided `recurse_into_xxx` methods encapsulate the boilerplate work of recursing into the
child schema elements of each schema element. Except as specifically noted otherwise, these
recursive helpers all behave uniformly, based on the number of children the schema element has:

* Leaf (no children) - Leaf `transform_xxx` methods simply return their argument unchanged, and
  no corresponding `recurse_into_xxx` method is provided.

* Unary (single child) - If the child was filtered out, filter out the parent. If the child
  changed, build a new parent around it. Otherwise, return the parent unchanged.

* Binary (two children) - If either child was filtered out, filter out the parent. If at least
  one child changed, build a new parent around them. Otherwise, return the parent unchanged.

* Variadic (0+ children) - If no children remain (all filtered out), filter out the parent.
  Otherwise, if at least one child changed or was filtered out, build a new parent around the
  children. Otherwise, return the parent unchanged.

Implementations can call these as needed, but will generally not need to override them.

# Transform carrier selection

Implementations choose an output [`Carrier`](../operations/buoyant_kernel.transforms.carrier.Carrier.md#op-329287d77829220b89182e9d) instance based on the operation to be
performed. That carrier determines the return type of each transform method.

For example, a simple read-only visitor would use `()` as a carrier, while a validity checker
could use `DeltaResult<()>` instead. A mutating transform uses `Cow<_>`, returning `Cow::Owned`
for changed/replaced nodes, and a filtering transform uses `Option<Cow<_>>`, where `None`
indicates the node should be dropped rather than replaced. `DeltaResult<Cow<_>>` and
`Result<Option<Cow<_>>, E>` round out the set as fallible mutating and fitering transforms that
short circuit immediately upon `Err`.

<a id="op-8a579b2333500ef1ba88b8c5"></a>
## Output

`assoc_type` · `buoyant_kernel::transforms::schema::SchemaTransform::Output` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Output
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/schema.rs#L56).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs:56`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

[`Carrier`](../operations/buoyant_kernel.transforms.carrier.Carrier.md#op-329287d77829220b89182e9d) output type for transformed nodes.

Implementations can use [`crate::transforms::transform_output_type`](../operations/buoyant_kernel.transform_output_type.md#op-ad05bd806f416e4d185977fc) to define `Output`
and `Residual` together.

<a id="op-10ac12f10c5107b0f20202a0"></a>
## Residual

`assoc_type` · `buoyant_kernel::transforms::schema::SchemaTransform::Residual` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Residual
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/schema.rs#L74).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs:74`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Residual type propagated by this transform's output [`Carrier`](../operations/buoyant_kernel.transforms.carrier.Carrier.md#op-329287d77829220b89182e9d). This type is statically
derivable, but still required due to limitations in rust type system expressiveness.

Implementations can use [`crate::transforms::transform_output_type`](../operations/buoyant_kernel.transform_output_type.md#op-ad05bd806f416e4d185977fc) to define `Output`
and `Residual` together. Or, define it manually like this:
```rust,no_run
# use std::borrow::Cow;
# use buoyant_kernel as delta_kernel;
# use delta_kernel::transforms::SchemaTransform;
# use delta_kernel::transforms::Carrier;
# struct X;
# impl<'a> SchemaTransform<'a> for X {
#     type Output<T: std::borrow::ToOwned + ?Sized + 'a> = Cow<'a, T>;
type Residual = <Self::Output<()> as Carrier<'a, ()>>::Residual;
# }
```

<a id="op-3de22ba53d216dc7d59bc8ea"></a>
## recurse_into_array

`function` · `buoyant_kernel::transforms::schema::SchemaTransform::recurse_into_array` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn recurse_into_array(&mut self, atype: &'a ArrayType) -> Self::Output<ArrayType>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/schema.rs#L174).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs:174`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Recursively transforms an array's element type (unary).

<a id="op-71e80472de465d35701ec29a"></a>
## recurse_into_map

`function` · `buoyant_kernel::transforms::schema::SchemaTransform::recurse_into_map` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn recurse_into_map(&mut self, mtype: &'a MapType) -> Self::Output<MapType>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/schema.rs#L184).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs:184`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Recursively transforms a map's key and value types (binary).

<a id="op-dd7152af327f31db3f605a29"></a>
## recurse_into_struct

`function` · `buoyant_kernel::transforms::schema::SchemaTransform::recurse_into_struct` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn recurse_into_struct(&mut self, stype: &'a StructType) -> Self::Output<StructType>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/schema.rs#L168).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs:168`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Recursively transforms a struct's fields (variadic).

<a id="op-7fcd79d2abe9770307c24cc6"></a>
## recurse_into_struct_field

`function` · `buoyant_kernel::transforms::schema::SchemaTransform::recurse_into_struct_field` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn recurse_into_struct_field(&mut self, field: &'a StructField) -> Self::Output<StructField>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/schema.rs#L157).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs:157`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Recursively transforms a struct field's data type (unary).

<a id="op-695cbebb8622aa52bf09188a"></a>
## transform

`function` · `buoyant_kernel::transforms::schema::SchemaTransform::transform` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform(&mut self, data_type: &'a DataType) -> Self::Output<DataType>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/schema.rs#L131).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs:131`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

General entry point for a recursive traversal over any data type. Also invoked internally to
dispatch on nested data types encountered during the traversal.

<a id="op-fc4b8b612564073a1db977d9"></a>
## transform_array

`function` · `buoyant_kernel::transforms::schema::SchemaTransform::transform_array` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_array(&mut self, atype: &'a ArrayType) -> Self::Output<ArrayType>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/schema.rs#L95).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs:95`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Called for each array encountered during the traversal. The provided implementation just
forwards to [`Self::recurse_into_array`](../operations/buoyant_kernel.transforms.schema.SchemaTransform.md#op-3de22ba53d216dc7d59bc8ea).

<a id="op-f632330f578710d014fb16b6"></a>
## transform_array_element

`function` · `buoyant_kernel::transforms::schema::SchemaTransform::transform_array_element` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_array_element(&mut self, etype: &'a DataType) -> Self::Output<DataType>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/schema.rs#L101).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs:101`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Called for each array element type encountered during the traversal. The provided
implementation forwards to [`Self::transform`](../operations/buoyant_kernel.transforms.schema.SchemaTransform.md#op-695cbebb8622aa52bf09188a).

<a id="op-cea290765cd172b5d21ceccd"></a>
## transform_map

`function` · `buoyant_kernel::transforms::schema::SchemaTransform::transform_map` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_map(&mut self, mtype: &'a MapType) -> Self::Output<MapType>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/schema.rs#L107).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs:107`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Called for each map encountered during the traversal. The provided implementation just
forwards to [`Self::recurse_into_map`](../operations/buoyant_kernel.transforms.schema.SchemaTransform.md#op-71e80472de465d35701ec29a).

<a id="op-492844e0de82d57c5267f130"></a>
## transform_map_key

`function` · `buoyant_kernel::transforms::schema::SchemaTransform::transform_map_key` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_map_key(&mut self, etype: &'a DataType) -> Self::Output<DataType>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/schema.rs#L113).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs:113`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Called for each map key encountered during the traversal. The provided implementation
forwards to [`Self::transform`](../operations/buoyant_kernel.transforms.schema.SchemaTransform.md#op-695cbebb8622aa52bf09188a).

<a id="op-7f5bb0fd344cce50cac3abdd"></a>
## transform_map_value

`function` · `buoyant_kernel::transforms::schema::SchemaTransform::transform_map_value` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_map_value(&mut self, etype: &'a DataType) -> Self::Output<DataType>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/schema.rs#L119).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs:119`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Called for each map value encountered during the traversal. The provided implementation
forwards to [`Self::transform`](../operations/buoyant_kernel.transforms.schema.SchemaTransform.md#op-695cbebb8622aa52bf09188a).

<a id="op-f6800546fda94be509a77b17"></a>
## transform_primitive

`function` · `buoyant_kernel::transforms::schema::SchemaTransform::transform_primitive` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_primitive(&mut self, ptype: &'a PrimitiveType) -> Self::Output<PrimitiveType>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/schema.rs#L77).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs:77`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Called for each primitive encountered during the traversal (leaf).

<a id="op-dd9d6d72aa8a68a3d1829cdd"></a>
## transform_struct

`function` · `buoyant_kernel::transforms::schema::SchemaTransform::transform_struct` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_struct(&mut self, stype: &'a StructType) -> Self::Output<StructType>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/schema.rs#L83).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs:83`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Called for each struct encountered during the traversal. The provided implementation just
forwards to [`Self::recurse_into_struct`](../operations/buoyant_kernel.transforms.schema.SchemaTransform.md#op-dd7152af327f31db3f605a29).

<a id="op-422c0cca9272df35bf912045"></a>
## transform_struct_field

`function` · `buoyant_kernel::transforms::schema::SchemaTransform::transform_struct_field` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_struct_field(&mut self, field: &'a StructField) -> Self::Output<StructField>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/schema.rs#L89).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs:89`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Called for each struct field encountered during the traversal. The provided implementation
forwards to [`Self::recurse_into_struct_field`](../operations/buoyant_kernel.transforms.schema.SchemaTransform.md#op-7fcd79d2abe9770307c24cc6).

<a id="op-10f27a0b01e27d5a5abc5e8f"></a>
## transform_variant

`function` · `buoyant_kernel::transforms::schema::SchemaTransform::transform_variant` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn transform_variant(&mut self, stype: &'a StructType) -> Self::Output<StructType>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/transforms/schema.rs#L125).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/transforms/schema.rs:125`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Called for each variant value encountered. The provided implementation just
forwards to [`Self::recurse_into_struct`](../operations/buoyant_kernel.transforms.schema.SchemaTransform.md#op-dd7152af327f31db3f605a29).
