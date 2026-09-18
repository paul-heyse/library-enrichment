# `datafusion_expr::simplify::ExprSimplifyResult`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.simplify.ExprSimplifyResult.json).

<a id="op-e99cc91e51c5aaf2a6e6620b"></a>
## ExprSimplifyResult

`enum` · `datafusion_expr::simplify::ExprSimplifyResult` · datafusion-expr 55.1.0

```rust
enum ExprSimplifyResult
```

Source: `src/simplify.rs:184`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Was the expression simplified?

<a id="op-4c6a034b3d193e02c09a161c"></a>
## Original

`variant` · `datafusion_expr::simplify::ExprSimplifyResult::Original` · datafusion-expr 55.1.0

```rust
Original
```

Source: `src/simplify.rs:189`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The function call could not be simplified, and the arguments
are return unmodified.

<a id="op-f47d87abe48500ac96df4515"></a>
## Simplified

`variant` · `datafusion_expr::simplify::ExprSimplifyResult::Simplified` · datafusion-expr 55.1.0

```rust
Simplified
```

Source: `src/simplify.rs:186`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The function call was simplified to an entirely new Expr

<a id="op-5be2a7079e21024f236dc479"></a>
## fmt

`function` · `datafusion_expr::simplify::ExprSimplifyResult::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::simplify::ExprSimplifyResult", "path": "ExprSimplifyResult"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [183, 10], "end": [183, 15], "filename": "src/simplify.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/simplify.rs:183`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
