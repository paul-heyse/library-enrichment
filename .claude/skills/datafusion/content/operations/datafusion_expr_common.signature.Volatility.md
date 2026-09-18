# `datafusion_expr_common::signature::Volatility`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr_common.signature.Volatility.json).

<a id="op-2e27042945dba7db012302a0"></a>
## Volatility

`enum` · `datafusion_expr_common::signature::Volatility` · datafusion-expr-common 55.1.0

```rust
enum Volatility
```

Source: `src/signature.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

How a function's output changes with respect to a fixed input

The volatility of a function determines eligibility for certain
optimizations. You should always define your function to have the strictest
possible volatility to maximize performance and avoid unexpected
results.

<a id="op-0c6bf94a7ecf7f1f856c8782"></a>
## Immutable

`variant` · `datafusion_expr_common::signature::Volatility::Immutable` · datafusion-expr-common 55.1.0

```rust
Immutable
```

Source: `src/signature.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

Always returns the same output when given the same input.

DataFusion will inline immutable functions during planning.

For example, the `abs` function is immutable, so `abs(-1)` will be
evaluated and replaced  with `1` during planning rather than invoking
the function at runtime.

<a id="op-7c4b71005bec8cf13759d8a1"></a>
## Stable

`variant` · `datafusion_expr_common::signature::Volatility::Stable` · datafusion-expr-common 55.1.0

```rust
Stable
```

Source: `src/signature.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

May return different values given the same input across different
queries but must return the same value for a given input within a query.

For example, the `now()` function is stable, because the query `select
col1, now() from t1`, will return different results each time it is run,
but within the same query, the output of the `now()` function has the
same value for each output row.

DataFusion will inline `Stable` functions when possible. For example,
`Stable` functions are inlined when planning a query for execution, but
not in View definitions or prepared statements.

<a id="op-c5fabfb9955a7015068c463b"></a>
## Volatile

`variant` · `datafusion_expr_common::signature::Volatility::Volatile` · datafusion-expr-common 55.1.0

```rust
Volatile
```

Source: `src/signature.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

May change the return value from evaluation to evaluation.

Multiple invocations of a volatile function may return different results
when used in the same query on different rows. An example of this is the
`random()` function.

DataFusion can not evaluate such functions during planning or push these
predicates into scans. In the query `select col1, random() from t1`,
`random()` function will be evaluated for each output row, resulting in
a unique random value for each row.

<a id="op-ef6075acff1e8baf220fbcfe"></a>
## clone

`function` · `datafusion_expr_common::signature::Volatility::clone` · datafusion-expr-common 55.1.0

```rust
fn clone(&self) -> Volatility
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Volatility", "path": "Volatility"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 49], "end": [55, 54], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/signature.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df9d25166113cb1dd296327c"></a>
## cmp

`function` · `datafusion_expr_common::signature::Volatility::cmp` · datafusion-expr-common 55.1.0

```rust
fn cmp(&self, other: &Volatility) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Volatility", "path": "Volatility"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 44], "end": [55, 47], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/signature.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f32c00bb28dae86e80f79e2"></a>
## eq

`function` · `datafusion_expr_common::signature::Volatility::eq` · datafusion-expr-common 55.1.0

```rust
fn eq(&self, other: &Volatility) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Volatility", "path": "Volatility"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 17], "end": [55, 26], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/signature.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4afee16c4092e061e12672ed"></a>
## fmt

`function` · `datafusion_expr_common::signature::Volatility::fmt` · datafusion-expr-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Volatility", "path": "Volatility"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 10], "end": [55, 15], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/signature.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e884df9efb4192054ca3adaa"></a>
## hash

`function` · `datafusion_expr_common::signature::Volatility::hash` · datafusion-expr-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Volatility", "path": "Volatility"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 62], "end": [55, 66], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/signature.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34df71a7f594ab21aa35db7f"></a>
## partial_cmp

`function` · `datafusion_expr_common::signature::Volatility::partial_cmp` · datafusion-expr-common 55.1.0

```rust
fn partial_cmp(&self, other: &Volatility) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr_common::signature::Volatility", "path": "Volatility"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 32], "end": [55, 42], "filename": "src/signature.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/signature.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
