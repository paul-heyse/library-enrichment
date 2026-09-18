# `datafusion_sql::planner::PlannerContext`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_sql.planner.PlannerContext.json).

<a id="op-585ec5739d703822ab2a0d38"></a>
## PlannerContext

`struct` · `datafusion_sql::planner::PlannerContext` · datafusion-sql 55.1.0

```rust
struct PlannerContext
```

Source: `src/planner.rs:257`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Struct to store the states used by the Planner. The Planner will leverage the states
to resolve CTEs, Views, subqueries and PREPARE statements. The states include
Common Table Expression (CTE) provided with WITH clause and
Parameter Data Types provided with PREPARE statement and the query schema of the
outer query plan.

# Cloning

Only the `ctes` are truly cloned when the `PlannerContext` is cloned.
This helps resolve scoping issues of CTEs.
By using cloning, a subquery can inherit CTEs from the outer query
and can also define its own private CTEs without affecting the outer query.

<a id="op-6c677e483062d4205260f0c4"></a>
## append_outer_query_schema

`function` · `datafusion_sql::planner::PlannerContext::append_outer_query_schema` · datafusion-sql 55.1.0

```rust
fn append_outer_query_schema(&mut self, schema: DFSchemaRef)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::PlannerContext", "path": "PlannerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [287, 1], "end": [433, 2], "filename": "src/planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/planner.rs:334`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Sets the outer query schema, returning the existing one, if
any

<a id="op-8f7871dda447f6bc2862ed93"></a>
## clone

`function` · `datafusion_sql::planner::PlannerContext::clone` · datafusion-sql 55.1.0

```rust
fn clone(&self) -> PlannerContext
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::PlannerContext", "path": "PlannerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [256, 17], "end": [256, 22], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/planner.rs:256`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ac0052804740e1aa50b6886"></a>
## contains_cte

`function` · `datafusion_sql::planner::PlannerContext::contains_cte` · datafusion-sql 55.1.0

```rust
fn contains_cte(&self, cte_name: &str) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::PlannerContext", "path": "PlannerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [287, 1], "end": [433, 2], "filename": "src/planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/planner.rs:390`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Returns true if there is a Common Table Expression (CTE) /
Subquery for the specified name

<a id="op-ec7038e655a443f8d03d5b15"></a>
## default

`function` · `datafusion_sql::planner::PlannerContext::default` · datafusion-sql 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::PlannerContext", "path": "PlannerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [281, 1], "end": [285, 2], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/planner.rs:282`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71eba65fad2008537a057ae5"></a>
## extend_outer_from_schema

`function` · `datafusion_sql::planner::PlannerContext::extend_outer_from_schema` · datafusion-sql 55.1.0

```rust
fn extend_outer_from_schema(&mut self, schema: &DFSchemaRef) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::PlannerContext", "path": "PlannerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [287, 1], "end": [433, 2], "filename": "src/planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/planner.rs:375`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Extends the FROM schema, returning the existing one, if any

<a id="op-8d677e8bb0ce6dd6ba200569"></a>
## fmt

`function` · `datafusion_sql::planner::PlannerContext::fmt` · datafusion-sql 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::PlannerContext", "path": "PlannerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [256, 10], "end": [256, 15], "filename": "src/planner.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/planner.rs:256`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-727dc5b6f18a510dce0b69e4"></a>
## get_cte

`function` · `datafusion_sql::planner::PlannerContext::get_cte` · datafusion-sql 55.1.0

```rust
fn get_cte(&self, cte_name: &str) -> Option<&LogicalPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::PlannerContext", "path": "PlannerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [287, 1], "end": [433, 2], "filename": "src/planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/planner.rs:403`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Return a plan for the Common Table Expression (CTE) / Subquery for the
specified name

<a id="op-22a9d6fc1d5ebc5f5d858ac6"></a>
## insert_cte

`function` · `datafusion_sql::planner::PlannerContext::insert_cte` · datafusion-sql 55.1.0

```rust
fn insert_cte(&mut self, cte_name: impl Into<String>, plan: LogicalPlan)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::PlannerContext", "path": "PlannerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [287, 1], "end": [433, 2], "filename": "src/planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/planner.rs:396`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Inserts a LogicalPlan for the Common Table Expression (CTE) /
Subquery for the specified name

<a id="op-fdac6fe6e21e80b80f2655e3"></a>
## lambda_parameters

`function` · `datafusion_sql::planner::PlannerContext::lambda_parameters` · datafusion-sql 55.1.0

```rust
fn lambda_parameters(&self) -> &HashMap<String, FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::PlannerContext", "path": "PlannerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [287, 1], "end": [433, 2], "filename": "src/planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/planner.rs:407`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c5c8de221251c9529850486"></a>
## latest_outer_query_schema

`function` · `datafusion_sql::planner::PlannerContext::latest_outer_query_schema` · datafusion-sql 55.1.0

```rust
fn latest_outer_query_schema(&self) -> Option<&DFSchemaRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::PlannerContext", "path": "PlannerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [287, 1], "end": [433, 2], "filename": "src/planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/planner.rs:339`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

The schema of the adjacent outer relation

<a id="op-865af1f6996b675447f8648c"></a>
## new

`function` · `datafusion_sql::planner::PlannerContext::new` · datafusion-sql 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::PlannerContext", "path": "PlannerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [287, 1], "end": [433, 2], "filename": "src/planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/planner.rs:289`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Create an empty PlannerContext

<a id="op-4efdb592c77a51763993695a"></a>
## outer_from_schema

`function` · `datafusion_sql::planner::PlannerContext::outer_from_schema` · datafusion-sql 55.1.0

```rust
fn outer_from_schema(&self) -> Option<Arc<DFSchema>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::PlannerContext", "path": "PlannerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [287, 1], "end": [433, 2], "filename": "src/planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/planner.rs:361`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3992d04dd919599e4aef3427"></a>
## outer_queries_schemas

`function` · `datafusion_sql::planner::PlannerContext::outer_queries_schemas` · datafusion-sql 55.1.0

```rust
fn outer_queries_schemas(&self) -> &[DFSchemaRef]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::PlannerContext", "path": "PlannerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [287, 1], "end": [433, 2], "filename": "src/planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/planner.rs:312`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Return the stack of outer relations' schemas, the outer most
relation are at the first entry

<a id="op-e9748656dd4bd57deac858a5"></a>
## outer_schemas_iter

`function` · `datafusion_sql::planner::PlannerContext::outer_schemas_iter` · datafusion-sql 55.1.0

```rust
fn outer_schemas_iter(&self) -> impl Iterator<Item = &DFSchemaRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::PlannerContext", "path": "PlannerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [287, 1], "end": [433, 2], "filename": "src/planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/planner.rs:328`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Return an iterator of the subquery relations' schemas, innermost
relation is returned first.

This order corresponds to the order of resolution when looking up column
references in subqueries, which start from the innermost relation and
then look up the outer relations one by one until a match is found or no
more outer relation exist.

NOTE this is *REVERSED* order of [`Self::outer_queries_schemas`](../operations/datafusion_sql.planner.PlannerContext.md#op-3992d04dd919599e4aef3427)

This is useful to resolve the column reference in the subquery by
looking up the outer query schemas one by one.

<a id="op-5f9282bebed1b208a671a050"></a>
## pop_outer_query_schema

`function` · `datafusion_sql::planner::PlannerContext::pop_outer_query_schema` · datafusion-sql 55.1.0

```rust
fn pop_outer_query_schema(&mut self) -> Option<DFSchemaRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::PlannerContext", "path": "PlannerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [287, 1], "end": [433, 2], "filename": "src/planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/planner.rs:344`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Remove the schema of the adjacent outer relation

<a id="op-717d74335c389a9ca8ca6ebb"></a>
## prepare_param_data_types

`function` · `datafusion_sql::planner::PlannerContext::prepare_param_data_types` · datafusion-sql 55.1.0

```rust
fn prepare_param_data_types(&self) -> &[Option<FieldRef>]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::PlannerContext", "path": "PlannerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [287, 1], "end": [433, 2], "filename": "src/planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/planner.rs:384`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Return the types of parameters (`$1`, `$2`, etc) if known

<a id="op-ebfc508f0c52082acae626b3"></a>
## set_outer_from_schema

`function` · `datafusion_sql::planner::PlannerContext::set_outer_from_schema` · datafusion-sql 55.1.0

```rust
fn set_outer_from_schema(&mut self, schema: Option<DFSchemaRef>) -> Option<DFSchemaRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::PlannerContext", "path": "PlannerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [287, 1], "end": [433, 2], "filename": "src/planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/planner.rs:366`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Sets the outer FROM schema, returning the existing one, if any

<a id="op-7e7d6bd9519a42ac64d26551"></a>
## set_table_schema

`function` · `datafusion_sql::planner::PlannerContext::set_table_schema` · datafusion-sql 55.1.0

```rust
fn set_table_schema(&mut self, schema: Option<DFSchemaRef>) -> Option<DFSchemaRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::PlannerContext", "path": "PlannerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [287, 1], "end": [433, 2], "filename": "src/planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/planner.rs:348`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b9429d74e2da5636b9b969c"></a>
## table_schema

`function` · `datafusion_sql::planner::PlannerContext::table_schema` · datafusion-sql 55.1.0

```rust
fn table_schema(&self) -> Option<DFSchemaRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::PlannerContext", "path": "PlannerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [287, 1], "end": [433, 2], "filename": "src/planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/planner.rs:356`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3030cdf1c2cb6f51565e811"></a>
## with_lambda_parameters

`function` · `datafusion_sql::planner::PlannerContext::with_lambda_parameters` · datafusion-sql 55.1.0

```rust
fn with_lambda_parameters(self, parameters: impl IntoIterator<Item = FieldRef>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::PlannerContext", "path": "PlannerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [287, 1], "end": [433, 2], "filename": "src/planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/planner.rs:411`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6ee95cd8d25877a29dc0a11"></a>
## with_prepare_param_data_types

`function` · `datafusion_sql::planner::PlannerContext::with_prepare_param_data_types` · datafusion-sql 55.1.0

```rust
fn with_prepare_param_data_types(self, prepare_param_data_types: Vec<Option<FieldRef>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_sql::planner::PlannerContext", "path": "PlannerContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [287, 1], "end": [433, 2], "filename": "src/planner.rs"}, "trait": null, "trait_path": null}`

Source: `src/planner.rs:302`. [Exact documentation build](https://docs.rs/crate/datafusion-sql/55.1.0/json).

Update the PlannerContext with provided prepare_param_data_types
