# `datafusion_functions_aggregate::regr::RegrType`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.regr.RegrType.json).

<a id="op-f41b84c3affd77e01dd26267"></a>
## RegrType

`enum` · `datafusion_functions_aggregate::regr::RegrType` · datafusion-functions-aggregate 55.1.0

```rust
enum RegrType
```

Source: `src/regr.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-837c02fbddf75943edc24450"></a>
## AvgX

`variant` · `datafusion_functions_aggregate::regr::RegrType::AvgX` · datafusion-functions-aggregate 55.1.0

```rust
AvgX
```

Source: `src/regr.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Variant for `regr_avgx` aggregate expression
Returns the average of the independent variable for non-null pairs in aggregate columns.
Given input column X: `regr_avgx(Y, X)` returns the average of X values.

<a id="op-7a4643ff3521ad328f27a290"></a>
## AvgY

`variant` · `datafusion_functions_aggregate::regr::RegrType::AvgY` · datafusion-functions-aggregate 55.1.0

```rust
AvgY
```

Source: `src/regr.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Variant for `regr_avgy` aggregate expression
Returns the average of the dependent variable for non-null pairs in aggregate columns.
Given input column Y: `regr_avgy(Y, X)` returns the average of Y values.

<a id="op-db7df5a11835c2447cb7e058"></a>
## Count

`variant` · `datafusion_functions_aggregate::regr::RegrType::Count` · datafusion-functions-aggregate 55.1.0

```rust
Count
```

Source: `src/regr.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Variant for `regr_count` aggregate expression
Returns the number of input rows for which both expressions are not null.
Given input column Y and X: `regr_count(Y, X)` returns the count of non-null pairs.

<a id="op-df845dfd12d2720c187616ad"></a>
## Intercept

`variant` · `datafusion_functions_aggregate::regr::RegrType::Intercept` · datafusion-functions-aggregate 55.1.0

```rust
Intercept
```

Source: `src/regr.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Variant for `regr_intercept` aggregate expression
Returns the intercept of the linear regression line for non-null pairs in aggregate columns.
Given input column Y and X: `regr_intercept(Y, X)` returns the intercept (b in Y = k*X + b) using minimal
RSS fitting.

<a id="op-8adef3daf50807b624623e2b"></a>
## R2

`variant` · `datafusion_functions_aggregate::regr::RegrType::R2` · datafusion-functions-aggregate 55.1.0

```rust
R2
```

Source: `src/regr.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Variant for `regr_r2` aggregate expression
Returns the coefficient of determination (R-squared value) of the linear regression line for non-null pairs in aggregate columns.
The R-squared value represents the proportion of variance in Y that is predictable from X.

<a id="op-9269a85f54e590948afeac5b"></a>
## SXX

`variant` · `datafusion_functions_aggregate::regr::RegrType::SXX` · datafusion-functions-aggregate 55.1.0

```rust
SXX
```

Source: `src/regr.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Variant for `regr_sxx` aggregate expression
Returns the sum of squares of the independent variable for non-null pairs in aggregate columns.
Given input column X: `regr_sxx(Y, X)` returns the sum of squares of deviations of X from its mean.

<a id="op-221d3c7954345cd778912e61"></a>
## SXY

`variant` · `datafusion_functions_aggregate::regr::RegrType::SXY` · datafusion-functions-aggregate 55.1.0

```rust
SXY
```

Source: `src/regr.rs:111`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Variant for `regr_sxy` aggregate expression
Returns the sum of products of pairs of numbers for non-null pairs in aggregate columns.
Given input column Y and X: `regr_sxy(Y, X)` returns the sum of products of the deviations of Y and X from their respective means.

<a id="op-dbe3b3110b9876d68289fd19"></a>
## SYY

`variant` · `datafusion_functions_aggregate::regr::RegrType::SYY` · datafusion-functions-aggregate 55.1.0

```rust
SYY
```

Source: `src/regr.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Variant for `regr_syy` aggregate expression
Returns the sum of squares of the dependent variable for non-null pairs in aggregate columns.
Given input column Y: `regr_syy(Y, X)` returns the sum of squares of deviations of Y from its mean.

<a id="op-1506aa38eba54280b7ededeb"></a>
## Slope

`variant` · `datafusion_functions_aggregate::regr::RegrType::Slope` · datafusion-functions-aggregate 55.1.0

```rust
Slope
```

Source: `src/regr.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Variant for `regr_slope` aggregate expression
Returns the slope of the linear regression line for non-null pairs in aggregate columns.
Given input column Y and X: `regr_slope(Y, X)` returns the slope (k in Y = k*X + b) using minimal
RSS (Residual Sum of Squares) fitting.

<a id="op-55eefe76bf332eb90d62940e"></a>
## clone

`function` · `datafusion_functions_aggregate::regr::RegrType::clone` · datafusion-functions-aggregate 55.1.0

```rust
fn clone(&self) -> RegrType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::regr::RegrType", "path": "RegrType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 17], "end": [72, 22], "filename": "src/regr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/regr.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3bc9084502b20b2483b807e1"></a>
## eq

`function` · `datafusion_functions_aggregate::regr::RegrType::eq` · datafusion-functions-aggregate 55.1.0

```rust
fn eq(&self, other: &RegrType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::regr::RegrType", "path": "RegrType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 24], "end": [72, 33], "filename": "src/regr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/regr.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c9d213f43ec243b1a73298aa"></a>
## fmt

`function` · `datafusion_functions_aggregate::regr::RegrType::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::regr::RegrType", "path": "RegrType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 10], "end": [72, 15], "filename": "src/regr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/regr.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-078a43d7a5a87705671ffb13"></a>
## hash

`function` · `datafusion_functions_aggregate::regr::RegrType::hash` · datafusion-functions-aggregate 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::regr::RegrType", "path": "RegrType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 35], "end": [72, 39], "filename": "src/regr.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/regr.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
