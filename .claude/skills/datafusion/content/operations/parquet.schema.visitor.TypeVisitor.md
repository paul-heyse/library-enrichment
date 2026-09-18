# `parquet::schema::visitor::TypeVisitor`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.schema.visitor.TypeVisitor.json).

<a id="op-e36822a3f402272dff18a59a"></a>
## TypeVisitor

`trait` · `parquet::schema::visitor::TypeVisitor` · parquet 59.3.0

```rust
trait TypeVisitor<R, C>
```

Source: `src/schema/visitor.rs:26`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A utility trait to help user to traverse against parquet type.

<a id="op-b95ea24cc676ec24eb4c603a"></a>
## dispatch

`function` · `parquet::schema::visitor::TypeVisitor::dispatch` · parquet 59.3.0

```rust
fn dispatch(&mut self, cur_type: TypePtr, context: C) -> Result<R>
```

Source: `src/schema/visitor.rs:104`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A utility method which detects input type and calls corresponding method.

<a id="op-92d6bac29bbf7edd8851d97a"></a>
## visit_list

`function` · `parquet::schema::visitor::TypeVisitor::visit_list` · parquet 59.3.0

```rust
fn visit_list(&mut self, list_type: TypePtr, context: C) -> Result<R>
```

Source: `src/schema/visitor.rs:51`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Default implementation when visiting a list.

It checks list type definition and calls [`Self::visit_list_with_item`](../operations/parquet.schema.visitor.TypeVisitor.md#op-73500399ab2a40e60caada49) with extracted
item type.

To fully understand this algorithm, please refer to
[parquet doc](https://github.com/apache/parquet-format/blob/master/LogicalTypes.md).

For example, a standard list type looks like:

```text
required/optional group my_list (LIST) {
```

In such a case, [`Self::visit_list_with_item`](../operations/parquet.schema.visitor.TypeVisitor.md#op-73500399ab2a40e60caada49) will be called with `my_list` as the list
type, and `element` as the `item_type`


<a id="op-73500399ab2a40e60caada49"></a>
## visit_list_with_item

`function` · `parquet::schema::visitor::TypeVisitor::visit_list_with_item` · parquet 59.3.0

```rust
fn visit_list_with_item(&mut self, list_type: TypePtr, item_type: TypePtr, context: C) -> Result<R>
```

Source: `src/schema/visitor.rs:119`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Called by `visit_list`.

<a id="op-917a4d308b84de74947fbf0e"></a>
## visit_map

`function` · `parquet::schema::visitor::TypeVisitor::visit_map` · parquet 59.3.0

```rust
fn visit_map(&mut self, map_type: TypePtr, context: C) -> Result<R>
```

Source: `src/schema/visitor.rs:101`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Called when a map type hit.

<a id="op-6d7b3e9373328c3436348629"></a>
## visit_primitive

`function` · `parquet::schema::visitor::TypeVisitor::visit_primitive` · parquet 59.3.0

```rust
fn visit_primitive(&mut self, primitive_type: TypePtr, context: C) -> Result<R>
```

Source: `src/schema/visitor.rs:28`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Called when a primitive type hit.

<a id="op-876c5babb2439c88bc566147"></a>
## visit_struct

`function` · `parquet::schema::visitor::TypeVisitor::visit_struct` · parquet 59.3.0

```rust
fn visit_struct(&mut self, struct_type: TypePtr, context: C) -> Result<R>
```

Source: `src/schema/visitor.rs:98`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Called when a struct type hit.
