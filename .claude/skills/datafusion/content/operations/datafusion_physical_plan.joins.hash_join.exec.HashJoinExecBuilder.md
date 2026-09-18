# `datafusion_physical_plan::joins::hash_join::exec::HashJoinExecBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.hash_join.exec.HashJoinExecBuilder.json).

<a id="op-7fe4403930fb5708259d14a1"></a>
## HashJoinExecBuilder

`struct` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExecBuilder` · datafusion-physical-plan 55.1.0

```rust
struct HashJoinExecBuilder
```

Source: `src/joins/hash_join/exec.rs:288`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Helps to build [`HashJoinExec`](../operations/datafusion_physical_plan.joins.hash_join.exec.HashJoinExec.md#op-41aad8f48fa144bd1cf86810).

Builder can be created from an existing [`HashJoinExec`](../operations/datafusion_physical_plan.joins.hash_join.exec.HashJoinExec.md#op-41aad8f48fa144bd1cf86810) using [`From::from`].
In this case, all its fields are inherited. If a field that affects the node's
properties is modified, they will be automatically recomputed during the build.

# Adding setters

When adding a new setter, it is necessary to ensure that the `preserve_properties`
flag is set to false if modifying the field requires a recomputation of the plan's
properties.


Unresolved upstream links (retained, not inferred): ``From::from``.

<a id="op-daba45646cda6af9a66ced68"></a>
## build

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExecBuilder::build` · datafusion-physical-plan 55.1.0

```rust
fn build(self) -> Result<HashJoinExec>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExecBuilder", "path": "HashJoinExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [293, 1], "end": [518, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/exec.rs:420`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Build resulting execution plan.

<a id="op-f56f2438de2dde268f43b436"></a>
## build_exec

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExecBuilder::build_exec` · datafusion-physical-plan 55.1.0

```rust
fn build_exec(self) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExecBuilder", "path": "HashJoinExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [293, 1], "end": [518, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/exec.rs:415`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Build result as a dyn execution plan.

<a id="op-e9804c2521a2cf6967189bc1"></a>
## from

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExecBuilder::from` · datafusion-physical-plan 55.1.0

```rust
fn from(exec: &HashJoinExec) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExecBuilder", "path": "HashJoinExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [520, 1], "end": [545, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExec", "path": "HashJoinExec"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/joins/hash_join/exec.rs:521`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3976a76f3ac7078a494cf622"></a>
## new

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExecBuilder::new` · datafusion-physical-plan 55.1.0

```rust
fn new(left: Arc<dyn ExecutionPlan>, right: Arc<dyn ExecutionPlan>, on: Vec<(PhysicalExprRef, PhysicalExprRef)>, join_type: JoinType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExecBuilder", "path": "HashJoinExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [293, 1], "end": [518, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/exec.rs:295`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Make a new [`HashJoinExecBuilder`](../operations/datafusion_physical_plan.joins.hash_join.exec.HashJoinExecBuilder.md#op-7fe4403930fb5708259d14a1).

<a id="op-06972369cae8d4a99805c31b"></a>
## recompute_properties

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExecBuilder::recompute_properties` · datafusion-physical-plan 55.1.0

```rust
fn recompute_properties(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExecBuilder", "path": "HashJoinExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [293, 1], "end": [518, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/exec.rs:386`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Require to recompute plan properties.

<a id="op-54933176e7c0e3928ca5c5cb"></a>
## reset_state

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExecBuilder::reset_state` · datafusion-physical-plan 55.1.0

```rust
fn reset_state(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExecBuilder", "path": "HashJoinExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [293, 1], "end": [518, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/exec.rs:407`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Reset runtime state.

<a id="op-fdc37c2160623ef49253e59f"></a>
## with_fetch

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExecBuilder::with_fetch` · datafusion-physical-plan 55.1.0

```rust
fn with_fetch(self, fetch: Option<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExecBuilder", "path": "HashJoinExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [293, 1], "end": [518, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/exec.rs:380`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set fetch property.

<a id="op-9a7063540230f5ff9a327764"></a>
## with_filter

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExecBuilder::with_filter` · datafusion-physical-plan 55.1.0

```rust
fn with_filter(self, filter: Option<JoinFilter>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExecBuilder", "path": "HashJoinExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [293, 1], "end": [518, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/exec.rs:348`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set optional filter.

<a id="op-d6d9e10703cd724d88ef10f1"></a>
## with_new_children

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExecBuilder::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(self, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExecBuilder", "path": "HashJoinExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [293, 1], "end": [518, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/exec.rs:392`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Replace children.

<a id="op-e90af38db2849d191f9f5093"></a>
## with_null_aware

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExecBuilder::with_null_aware` · datafusion-physical-plan 55.1.0

```rust
fn with_null_aware(self, null_aware: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExecBuilder", "path": "HashJoinExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [293, 1], "end": [518, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/exec.rs:374`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set null aware property.

<a id="op-9ade5e4fa6fafd948357e59f"></a>
## with_null_equality

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExecBuilder::with_null_equality` · datafusion-physical-plan 55.1.0

```rust
fn with_null_equality(self, null_equality: NullEquality) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExecBuilder", "path": "HashJoinExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [293, 1], "end": [518, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/exec.rs:368`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set null equality property.

<a id="op-4c42c67c2ba56084d320eb70"></a>
## with_on

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExecBuilder::with_on` · datafusion-physical-plan 55.1.0

```rust
fn with_on(self, on: Vec<(PhysicalExprRef, PhysicalExprRef)>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExecBuilder", "path": "HashJoinExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [293, 1], "end": [518, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/exec.rs:354`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set expressions to join on.

<a id="op-9dd74b484ff73dad101e26ae"></a>
## with_partition_mode

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExecBuilder::with_partition_mode` · datafusion-physical-plan 55.1.0

```rust
fn with_partition_mode(self, mode: PartitionMode) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExecBuilder", "path": "HashJoinExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [293, 1], "end": [518, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/exec.rs:361`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set partition mode.

<a id="op-c416b1c6fdcf847c3b300c66"></a>
## with_projection

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExecBuilder::with_projection` · datafusion-physical-plan 55.1.0

```rust
fn with_projection(self, projection: Option<Vec<usize>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExecBuilder", "path": "HashJoinExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [293, 1], "end": [518, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/exec.rs:336`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set projection from the vector.

<a id="op-37455fe2b76116e1cc769833"></a>
## with_projection_ref

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExecBuilder::with_projection_ref` · datafusion-physical-plan 55.1.0

```rust
fn with_projection_ref(self, projection: Option<ProjectionRef>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExecBuilder", "path": "HashJoinExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [293, 1], "end": [518, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/exec.rs:341`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set projection from the shared reference.

<a id="op-fd699849a6a335cace6b964e"></a>
## with_type

`function` · `datafusion_physical_plan::joins::hash_join::exec::HashJoinExecBuilder::with_type` · datafusion-physical-plan 55.1.0

```rust
fn with_type(self, join_type: JoinType) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::hash_join::exec::HashJoinExecBuilder", "path": "HashJoinExecBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [293, 1], "end": [518, 2], "filename": "src/joins/hash_join/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/hash_join/exec.rs:329`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set join type.
