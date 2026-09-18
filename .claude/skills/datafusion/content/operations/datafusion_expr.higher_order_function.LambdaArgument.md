# `datafusion_expr::higher_order_function::LambdaArgument`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.higher_order_function.LambdaArgument.json).

<a id="op-1f561a5260a23b5768a385f4"></a>
## LambdaArgument

`struct` · `datafusion_expr::higher_order_function::LambdaArgument` · datafusion-expr 55.1.0

```rust
struct LambdaArgument
```

Source: `src/higher_order_function.rs:236`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A lambda argument to a HigherOrderFunction

<a id="op-87932dfa3010615ecc329c54"></a>
## clone

`function` · `datafusion_expr::higher_order_function::LambdaArgument::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> LambdaArgument
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::LambdaArgument", "path": "LambdaArgument"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 10], "end": [235, 15], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/higher_order_function.rs:235`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d54194bc223aaacbf732ac2"></a>
## evaluate

`function` · `datafusion_expr::higher_order_function::LambdaArgument::evaluate` · datafusion-expr 55.1.0

```rust
fn evaluate(&self, args: &[&dyn Fn() -> Result<ArrayRef>], spread_captures: impl FnOnce(&[ArrayRef]) -> Result<Vec<ArrayRef>>) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::LambdaArgument", "path": "LambdaArgument"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [279, 1], "end": [397, 2], "filename": "src/higher_order_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/higher_order_function.rs:372`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Evaluate this lambda
`args` should evaluate to the value of each parameter
of the correspondent lambda returned in [HigherOrderUDFImpl::lambda_parameters](../operations/datafusion_expr.higher_order_function.HigherOrderUDFImpl.md#op-8a89b1f3117f6cf763ede484).

Only the closures in `args` for parameters the lambda body actually
references are called; closures for declared-but-unused parameters
are skipped entirely. Callers should not rely on every closure in
`args` being invoked.

`spread_captures` is responsible for transforming the captured column arrays
so they align with the evaluation batch. Captures are snapshotted from the
outer batch at construction time, giving one value per outer row, but the
function may evaluate the lambda body over a batch with a different number
of rows. It is the function's responsibility to provide the appropriate
`spread_captures` closure to expand (or otherwise reshape) the captures
to match.

Taking as an example the following table:

```sql
CREATE TABLE t (arr INT[], a INT) AS VALUES
  ([1, 2, 3], 10),
  ([],        20),
  ([4],       30);
```

`SELECT array_transform(arr, v -> v + a) from t` would execute over three outer rows:

```text
arr (ListArray):  [[1, 2, 3], [], [4]]   -- 3 outer rows, 4 total elements
a   (captured):   [10,        20,  30]   -- one value per outer row
```

`array_transform` flattens the list elements into a single batch of 4 rows,
so `spread_captures` must repeat/drop captured values to match:

```text
v (flattened args): [1,  2,  3,  4]
a (spread):         [10, 10, 10, 30]  -- 10 repeated for 3 elements in row 0,
                                       -- 20 dropped for the empty sublist in row 1,
                                       -- 30 once for the single element in row 2
```

The lambda body `v + a` then evaluates element-wise over these 4-row arrays,
producing `[11, 12, 13, 34]`, which `array_transform` reassembles into `[[11, 12, 13], [], [34]]`.

If the lambda has no captures, `spread_captures` is never called.

<a id="op-f9d2a1758976d5e592bf6035"></a>
## fmt

`function` · `datafusion_expr::higher_order_function::LambdaArgument::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::LambdaArgument", "path": "LambdaArgument"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [235, 17], "end": [235, 22], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/higher_order_function.rs:235`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06249e7b965cc9abf6b74726"></a>
## new

`function` · `datafusion_expr::higher_order_function::LambdaArgument::new` · datafusion-expr 55.1.0

```rust
fn new(params: Vec<FieldRef>, body: Arc<dyn PhysicalExpr>, captures: Option<RecordBatch>, used_param_indices: &[usize]) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::higher_order_function::LambdaArgument", "path": "LambdaArgument"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [279, 1], "end": [397, 2], "filename": "src/higher_order_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/higher_order_function.rs:286`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

# Preconditions

Every index in `used_param_indices` must be `< params.len()`;
violating this panics on out-of-bounds indexing below. Callers should
pass `LambdaExpr::used_param_indices()`, which always indexes into the
same `params` list, rather than constructing indices by hand.
