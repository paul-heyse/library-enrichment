# `datafusion_expr_common::sort_properties::ExprProperties`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.sort_properties.ExprProperties.json).

<a id="op-51897a0df15cf12898939b7d"></a>
## ExprProperties

`struct` · `datafusion_expr_common::sort_properties::ExprProperties` · datafusion-expr-common 55.1.0

```rust
struct ExprProperties
```

Source: `src/sort_properties.rs:135`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Represents the properties of a `PhysicalExpr`, including its sorting,
range, and whether it preserves lexicographical ordering.

<a id="op-8d7803f3e9177f36af0674aa"></a>
## clone

`function` · `datafusion_expr_common::sort_properties::ExprProperties::clone` · datafusion-expr-common 55.1.0

```rust
fn clone(&self) -> ExprProperties
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::sort_properties::ExprProperties", "path": "ExprProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 17], "end": [134, 22], "filename": "src/sort_properties.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/sort_properties.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c38fc6bb787a4db91fb948c5"></a>
## fmt

`function` · `datafusion_expr_common::sort_properties::ExprProperties::fmt` · datafusion-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::sort_properties::ExprProperties", "path": "ExprProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 10], "end": [134, 15], "filename": "src/sort_properties.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sort_properties.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5ccc29b5153d34f17141e01"></a>
## new_unknown

`function` · `datafusion_expr_common::sort_properties::ExprProperties::new_unknown` · datafusion-expr-common 55.1.0

```rust
fn new_unknown() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::sort_properties::ExprProperties", "path": "ExprProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 1], "end": [240, 2], "filename": "src/sort_properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_properties.rs:204`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Creates a new `ExprProperties` instance with unknown sort properties,
unknown range, and unknown lexicographical ordering preservation.

<a id="op-2ba98f8a96c838499ec6bd27"></a>
## preserves_lex_ordering

`struct_field` · `datafusion_expr_common::sort_properties::ExprProperties::preserves_lex_ordering` · datafusion-expr-common 55.1.0

```rust
preserves_lex_ordering: bool
```

Source: `src/sort_properties.rs:150`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Indicates whether the expression preserves lexicographical ordering
of its inputs.

This is a *non-strict* (monotone) property: inputs advancing in
lexicographical order never make the output decrease, but distinct
inputs may map to equal outputs (ties). See
[`Self::strictly_order_preserving`](../operations/datafusion_expr_common.sort_properties.ExprProperties.md#op-2ec9acd60d62113a73cd3a5f) for the strict variant and an
explanation of the difference.

<a id="op-44f1c30f34532afd7b2fc362"></a>
## range

`struct_field` · `datafusion_expr_common::sort_properties::ExprProperties::range` · datafusion-expr-common 55.1.0

```rust
range: interval_arithmetic::Interval
```

Source: `src/sort_properties.rs:141`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

A closed interval representing the range of possible values for
the expression. Used to compute reliable bounds.

<a id="op-5341c1cfe24e647a1c249110"></a>
## sort_properties

`struct_field` · `datafusion_expr_common::sort_properties::ExprProperties::sort_properties` · datafusion-expr-common 55.1.0

```rust
sort_properties: SortProperties
```

Source: `src/sort_properties.rs:138`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Properties that describe the sorting behavior of the expression,
such as whether it is ordered, unordered, or a singleton value.

<a id="op-2ec9acd60d62113a73cd3a5f"></a>
## strictly_order_preserving

`struct_field` · `datafusion_expr_common::sort_properties::ExprProperties::strictly_order_preserving` · datafusion-expr-common 55.1.0

```rust
strictly_order_preserving: bool
```

Source: `src/sort_properties.rs:198`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Indicates whether the expression is strictly order-preserving with
respect to its inputs that are `Ordered`: the output is ordered in the
same direction, equal outputs can only result from equal values of
those inputs (i.e. the mapping is one-to-one), and nulls map to nulls.

i.e. setting this to true means that `a.cmp(b) == f(a).cmp(f(b))`

# Difference from [`Self::preserves_lex_ordering`](../operations/datafusion_expr_common.sort_properties.ExprProperties.md#op-2ba98f8a96c838499ec6bd27)

The two properties differ in both their premise and their strictness:

- `preserves_lex_ordering` assumes the inputs advance in
  *lexicographical* order (a later input may decrease whenever an
  earlier one increases), and only promises a non-decreasing output,
  allowing distinct inputs to collapse into equal outputs; `floor`,
  `date_trunc` and narrowing casts do exactly that.
- `strictly_order_preserving` assumes every `Ordered` input advances
  *simultaneously* (component-wise, which is what actually holds when
  all of them are sorted in the data), and promises a strict output:
  equal outputs only from equal inputs.

For an expression with a single ordered input the premises coincide,
and this field is simply the stronger claim: it implies
`preserves_lex_ordering`. With multiple ordered inputs, neither
implies the other: a lexicographical-ordering-preserving expression
need not be strict (distinct inputs may still produce equal outputs),
while `a + b` over two ordered, overflow-free inputs is strict but not
lexicographical (under the lexicographical premise `b` may decrease
while `a` increases, making the sum decrease).

The distinction matters for suffix sort keys. Optimizers use this
field to substitute a sort key with an expression computed from it:
if data is sorted by `[x, y]`, it is also sorted by `[expr(x), y]`.
That claim requires `y` to be sorted within each run of equal
`expr(x)` values, which only holds if equal outputs imply equal `x`
values. With a merely monotone expression such as `floor`, one output
run can span several `x` groups, and `y` restarts at each group:

```text
sorted by [x, y]:  (1.2, 5), (1.8, 1), (2.5, 3)
[floor(x), y]:     (1, 5),   (1, 1),   (2, 3)   <-- y not sorted within
                                                    the "1" run
```

Hence a monotone expression only justifies the length-1 ordering
`[expr(x)]`, while a strictly order-preserving one keeps the entire
suffix valid. When in doubt, set to `false`.

<a id="op-e172d1f01b7ee22cbb7f78cc"></a>
## with_order

`function` · `datafusion_expr_common::sort_properties::ExprProperties::with_order` · datafusion-expr-common 55.1.0

```rust
fn with_order(self, order: SortProperties) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::sort_properties::ExprProperties", "path": "ExprProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 1], "end": [240, 2], "filename": "src/sort_properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_properties.rs:214`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Sets the sorting properties of the expression and returns the modified instance.

<a id="op-909b21c6a582ba46227bb1b5"></a>
## with_preserves_lex_ordering

`function` · `datafusion_expr_common::sort_properties::ExprProperties::with_preserves_lex_ordering` · datafusion-expr-common 55.1.0

```rust
fn with_preserves_lex_ordering(self, preserves_lex_ordering: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::sort_properties::ExprProperties", "path": "ExprProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 1], "end": [240, 2], "filename": "src/sort_properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_properties.rs:226`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Sets whether the expression maintains lexicographical ordering and returns the modified instance.

<a id="op-2957a180b2fc507fd865c358"></a>
## with_range

`function` · `datafusion_expr_common::sort_properties::ExprProperties::with_range` · datafusion-expr-common 55.1.0

```rust
fn with_range(self, range: Interval) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::sort_properties::ExprProperties", "path": "ExprProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 1], "end": [240, 2], "filename": "src/sort_properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_properties.rs:220`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Sets the range of the expression and returns the modified instance.

<a id="op-f3945cdbcada69d34c89f195"></a>
## with_strictly_order_preserving

`function` · `datafusion_expr_common::sort_properties::ExprProperties::with_strictly_order_preserving` · datafusion-expr-common 55.1.0

```rust
fn with_strictly_order_preserving(self, strictly_order_preserving: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::sort_properties::ExprProperties", "path": "ExprProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [201, 1], "end": [240, 2], "filename": "src/sort_properties.rs"}, "trait": null, "trait_path": null}`

Source: `src/sort_properties.rs:233`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Sets whether the expression is strictly order-preserving and returns
the modified instance.
