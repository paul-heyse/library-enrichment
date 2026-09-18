# `datafusion_session::table::TableFunctionArgs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_session.table.TableFunctionArgs.json).

<a id="op-2996d044af59b53bc0f6525a"></a>
## TableFunctionArgs

`struct` · `datafusion_session::table::TableFunctionArgs` · datafusion-session 55.1.0

```rust
struct TableFunctionArgs<'e, 's>
```

Source: `src/table.rs:576`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Describes arguments provided to the table function call.

<a id="op-902a7a55aa4060de68e867a7"></a>
## exprs

`function` · `datafusion_session::table::TableFunctionArgs::exprs` · datafusion-session 55.1.0

```rust
fn exprs(&self) -> &'e [Expr]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'e"}, {"lifetime": "'s"}], "constraints": []}}, "id": "datafusion_session::table::TableFunctionArgs", "path": "TableFunctionArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'e"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'s"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [583, 1], "end": [598, 2], "filename": "src/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/table.rs:590`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Get expressions passed as the called function arguments.

<a id="op-60f6e0186f9bac97f644152f"></a>
## new

`function` · `datafusion_session::table::TableFunctionArgs::new` · datafusion-session 55.1.0

```rust
fn new(exprs: &'e [Expr], session: &'s dyn Session) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'e"}, {"lifetime": "'s"}], "constraints": []}}, "id": "datafusion_session::table::TableFunctionArgs", "path": "TableFunctionArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'e"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'s"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [583, 1], "end": [598, 2], "filename": "src/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/table.rs:585`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Make a new [`TableFunctionArgs`](../operations/datafusion_session.table.TableFunctionArgs.md#op-2996d044af59b53bc0f6525a).

<a id="op-c0fe1a4b101c8c75e6cc57b6"></a>
## session

`function` · `datafusion_session::table::TableFunctionArgs::session` · datafusion-session 55.1.0

```rust
fn session(&self) -> &'s dyn Session
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'e"}, {"lifetime": "'s"}], "constraints": []}}, "id": "datafusion_session::table::TableFunctionArgs", "path": "TableFunctionArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'e"}, {"kind": {"lifetime": {"outlives": []}}, "name": "'s"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [583, 1], "end": [598, 2], "filename": "src/table.rs"}, "trait": null, "trait_path": null}`

Source: `src/table.rs:595`. [Exact documentation build](https://docs.rs/crate/datafusion-session/55.1.0/json).

Get a session where the table function is called.
