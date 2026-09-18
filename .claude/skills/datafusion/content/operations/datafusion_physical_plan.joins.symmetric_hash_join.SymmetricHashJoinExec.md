# `datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.symmetric_hash_join.SymmetricHashJoinExec.json).

<a id="op-511e003503c5f02d17e62166"></a>
## SymmetricHashJoinExec

`struct` · `datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec` · datafusion-physical-plan 55.1.0

```rust
struct SymmetricHashJoinExec
```

Source: `src/joins/symmetric_hash_join.rs:175`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

A symmetric hash join with range conditions is when both streams are hashed on the
join key and the resulting hash tables are used to join the streams.
The join is considered symmetric because the hash table is built on the join keys from both
streams, and the matching of rows is based on the values of the join keys in both streams.
This type of join is efficient in streaming context as it allows for fast lookups in the hash
table, rather than having to scan through one or both of the streams to find matching rows, also it
only considers the elements from the stream that fall within a certain sliding window (w/ range conditions),
making it more efficient and less likely to store stale data. This enables operating on unbounded streaming
data without any memory issues.

For each input stream, create a hash table.
  - For each new [RecordBatch](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) in build side, hash and insert into inputs hash table. Update offsets.
  - Test if input is equal to a predefined set of other inputs.
  - If so record the visited rows. If the matched row results must be produced (INNER, LEFT), output the [RecordBatch](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34).
  - Try to prune other side (probe) with new [RecordBatch](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34).
  - If the join type indicates that the unmatched rows results must be produced (LEFT, FULL etc.),
    output the [RecordBatch](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) when a pruning happens or at the end of the data.


``` text
                       +-------------------------+
                       |                         |
  left stream ---------|  Left OneSideHashJoiner |---+
                       |                         |   |
                       +-------------------------+   |
                                                     |
                                                     |--------- Joined output
                                                     |
                       +-------------------------+   |
                       |                         |   |
 right stream ---------| Right OneSideHashJoiner |---+
                       |                         |
                       +-------------------------+

Prune build side when the new RecordBatch comes to the probe side. We utilize interval arithmetic
on JoinFilter's sorted PhysicalExprs to calculate the joinable range.


              PROBE SIDE          BUILD SIDE
                BUFFER              BUFFER
            +-------------+     +------------+
            |             |     |            |    Unjoinable
            |             |     |            |    Range
            |             |     |            |
            |             |  |---------------------------------
            |             |  |  |            |
            |             |  |  |            |
            |             | /   |            |
            |             | |   |            |
            |             | |   |            |
            |             | |   |            |
            |             | |   |            |
            |             | |   |            |    Joinable
            |             |/    |            |    Range
            |             ||    |            |
            |+-----------+||    |            |
            || Record    ||     |            |
            || Batch     ||     |            |
            |+-----------+||    |            |
            +-------------+\    +------------+
                            |
                            \
                             |---------------------------------

 This happens when range conditions are provided on sorted columns. E.g.

       SELECT * FROM left_table, right_table
       ON
         left_key = right_key AND
         left_time > right_time - INTERVAL 12 MINUTES AND left_time < right_time + INTERVAL 2 HOUR

or
      SELECT * FROM left_table, right_table
       ON
         left_key = right_key AND
         left_sorted > right_sorted - 3 AND left_sorted < right_sorted + 10

For general purpose, in the second scenario, when the new data comes to probe side, the conditions can be used to
determine a specific threshold for discarding rows from the inner buffer. For example, if the sort order the
two columns ("left_sorted" and "right_sorted") are ascending (it can be different in another scenarios)
and the join condition is "left_sorted > right_sorted - 3" and the latest value on the right input is 1234, meaning
that the left side buffer must only keep rows where "leftTime > rightTime - 3 > 1234 - 3 > 1231" ,
making the smallest value in 'left_sorted' 1231 and any rows below (since ascending)
than that can be dropped from the inner buffer.
```

<a id="op-eb1d32f30f700dbb31bed477"></a>
## apply_expressions

`function` · `datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec::apply_expressions` · datafusion-physical-plan 55.1.0

```rust
fn apply_expressions(&self, f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>) -> Result<TreeNodeRecursion>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec", "path": "SymmetricHashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [406, 1], "end": [762, 2], "filename": "src/joins/symmetric_hash_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/symmetric_hash_join.rs:456`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9bcf06dd61995b9ebb890641"></a>
## check_if_order_information_available

`function` · `datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec::check_if_order_information_available` · datafusion-physical-plan 55.1.0

```rust
fn check_if_order_information_available(&self) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec", "path": "SymmetricHashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 1], "end": [364, 2], "filename": "src/joins/symmetric_hash_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/symmetric_hash_join.rs:338`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Check if order information covers every column in the filter expression.

<a id="op-c2e96c53ec0599f2f79c3a66"></a>
## children

`function` · `datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec::children` · datafusion-physical-plan 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec", "path": "SymmetricHashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [406, 1], "end": [762, 2], "filename": "src/joins/symmetric_hash_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/symmetric_hash_join.rs:452`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-00ebe9e33b24396220a90320"></a>
## clone

`function` · `datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> SymmetricHashJoinExec
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec", "path": "SymmetricHashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [174, 17], "end": [174, 22], "filename": "src/joins/symmetric_hash_join.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/joins/symmetric_hash_join.rs:174`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-173aa16d0ec155cdceea6105"></a>
## execute

`function` · `datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec", "path": "SymmetricHashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [406, 1], "end": [762, 2], "filename": "src/joins/symmetric_hash_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/symmetric_hash_join.rs:522`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aeb31c637422def23d69aeca"></a>
## filter

`function` · `datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec::filter` · datafusion-physical-plan 55.1.0

```rust
fn filter(&self) -> Option<&JoinFilter>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec", "path": "SymmetricHashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 1], "end": [364, 2], "filename": "src/joins/symmetric_hash_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/symmetric_hash_join.rs:308`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Filters applied before join output

<a id="op-28b69e1c9846e2c6f02e08b6"></a>
## fmt

`function` · `datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec", "path": "SymmetricHashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [174, 10], "end": [174, 15], "filename": "src/joins/symmetric_hash_join.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/joins/symmetric_hash_join.rs:174`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01f1146450e9a0501c3a6c2f"></a>
## fmt_as

`function` · `datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec::fmt_as` · datafusion-physical-plan 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec", "path": "SymmetricHashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [366, 1], "end": [404, 2], "filename": "src/joins/symmetric_hash_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/joins/symmetric_hash_join.rs:367`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d732ccdcb235107cb9f012ce"></a>
## input_distribution_requirements

`function` · `datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec::input_distribution_requirements` · datafusion-physical-plan 55.1.0

```rust
fn input_distribution_requirements(&self) -> InputDistributionRequirements
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec", "path": "SymmetricHashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [406, 1], "end": [762, 2], "filename": "src/joins/symmetric_hash_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/symmetric_hash_join.rs:419`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b43dd4bdebaf900961e64927"></a>
## join_type

`function` · `datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec::join_type` · datafusion-physical-plan 55.1.0

```rust
fn join_type(&self) -> &JoinType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec", "path": "SymmetricHashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 1], "end": [364, 2], "filename": "src/joins/symmetric_hash_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/symmetric_hash_join.rs:313`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

How the join is performed

<a id="op-86afd7d238fdfdcf023e29e8"></a>
## left

`function` · `datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec::left` · datafusion-physical-plan 55.1.0

```rust
fn left(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec", "path": "SymmetricHashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 1], "end": [364, 2], "filename": "src/joins/symmetric_hash_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/symmetric_hash_join.rs:293`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

left stream

<a id="op-8bbb419c1884225bd14266f5"></a>
## left_sort_exprs

`function` · `datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec::left_sort_exprs` · datafusion-physical-plan 55.1.0

```rust
fn left_sort_exprs(&self) -> Option<&LexOrdering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec", "path": "SymmetricHashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 1], "end": [364, 2], "filename": "src/joins/symmetric_hash_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/symmetric_hash_join.rs:328`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Get left_sort_exprs

<a id="op-2976bddc34e2b4eab1a143a6"></a>
## metrics

`function` · `datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec::metrics` · datafusion-physical-plan 55.1.0

```rust
fn metrics(&self) -> Option<MetricsSet>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec", "path": "SymmetricHashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [406, 1], "end": [762, 2], "filename": "src/joins/symmetric_hash_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/symmetric_hash_join.rs:518`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2911ce96d16b34066762f02c"></a>
## name

`function` · `datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec::name` · datafusion-physical-plan 55.1.0

```rust
fn name(&self) -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec", "path": "SymmetricHashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [406, 1], "end": [762, 2], "filename": "src/joins/symmetric_hash_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/symmetric_hash_join.rs:407`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-628612b2b9a6991762be6f3f"></a>
## null_equality

`function` · `datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec::null_equality` · datafusion-physical-plan 55.1.0

```rust
fn null_equality(&self) -> NullEquality
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec", "path": "SymmetricHashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 1], "end": [364, 2], "filename": "src/joins/symmetric_hash_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/symmetric_hash_join.rs:318`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Get null_equality

<a id="op-2cb4c917b7864e1d72894f7c"></a>
## on

`function` · `datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec::on` · datafusion-physical-plan 55.1.0

```rust
fn on(&self) -> &[(PhysicalExprRef, PhysicalExprRef)]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec", "path": "SymmetricHashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 1], "end": [364, 2], "filename": "src/joins/symmetric_hash_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/symmetric_hash_join.rs:303`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set of common columns used to join on

<a id="op-335f1815b69dbe0a9a8db228"></a>
## partition_mode

`function` · `datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec::partition_mode` · datafusion-physical-plan 55.1.0

```rust
fn partition_mode(&self) -> StreamJoinPartitionMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec", "path": "SymmetricHashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 1], "end": [364, 2], "filename": "src/joins/symmetric_hash_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/symmetric_hash_join.rs:323`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Get partition mode

<a id="op-77806b70d5c2b1761f1496c2"></a>
## properties

`function` · `datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec::properties` · datafusion-physical-plan 55.1.0

```rust
fn properties(&self) -> &Arc<PlanProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec", "path": "SymmetricHashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [406, 1], "end": [762, 2], "filename": "src/joins/symmetric_hash_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/symmetric_hash_join.rs:411`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0f3ec90a9805a0bf9565f75"></a>
## replace_children

`function` · `datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec::replace_children` · datafusion-physical-plan 55.1.0

```rust
fn replace_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>, options: ReplaceChildrenOptions) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec", "path": "SymmetricHashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [406, 1], "end": [762, 2], "filename": "src/joins/symmetric_hash_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/symmetric_hash_join.rs:465`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb33e8fb24417978f15beaef"></a>
## required_input_distribution

`function` · `datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec::required_input_distribution` · datafusion-physical-plan 55.1.0

```rust
fn required_input_distribution(&self) -> Vec<Distribution>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec", "path": "SymmetricHashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [406, 1], "end": [762, 2], "filename": "src/joins/symmetric_hash_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/symmetric_hash_join.rs:415`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05ef1b431582cae5bd79fbde"></a>
## required_input_ordering

`function` · `datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec::required_input_ordering` · datafusion-physical-plan 55.1.0

```rust
fn required_input_ordering(&self) -> Vec<Option<OrderingRequirements>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec", "path": "SymmetricHashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [406, 1], "end": [762, 2], "filename": "src/joins/symmetric_hash_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/symmetric_hash_join.rs:441`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c258554ead70e3d01ab73bc7"></a>
## right

`function` · `datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec::right` · datafusion-physical-plan 55.1.0

```rust
fn right(&self) -> &Arc<dyn ExecutionPlan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec", "path": "SymmetricHashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 1], "end": [364, 2], "filename": "src/joins/symmetric_hash_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/symmetric_hash_join.rs:298`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

right stream

<a id="op-17eb8c56c0b0a2e63ac5d089"></a>
## right_sort_exprs

`function` · `datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec::right_sort_exprs` · datafusion-physical-plan 55.1.0

```rust
fn right_sort_exprs(&self) -> Option<&LexOrdering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec", "path": "SymmetricHashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 1], "end": [364, 2], "filename": "src/joins/symmetric_hash_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/symmetric_hash_join.rs:333`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Get right_sort_exprs

<a id="op-796898b31c4dc363388563de"></a>
## try_from_proto

`function` · `datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec::try_from_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_from_proto(node: &datafusion_proto_models::protobuf::PhysicalPlanNode, ctx: &proto::ExecutionPlanDecodeCtx<'_>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec", "path": "SymmetricHashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [765, 1], "end": [930, 2], "filename": "src/joins/symmetric_hash_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/symmetric_hash_join.rs:771`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Reconstruct a [`SymmetricHashJoinExec`](../operations/datafusion_physical_plan.joins.symmetric_hash_join.SymmetricHashJoinExec.md#op-511e003503c5f02d17e62166) from its protobuf representation.

The exact inverse of [`ExecutionPlan::try_to_proto`].

[`ExecutionPlan::try_to_proto`]: crate::ExecutionPlan::try_to_proto

<a id="op-0d8a872b4c62115929c5de84"></a>
## try_new

`function` · `datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec::try_new` · datafusion-physical-plan 55.1.0

```rust
fn try_new(left: Arc<dyn ExecutionPlan>, right: Arc<dyn ExecutionPlan>, on: JoinOn, filter: Option<JoinFilter>, join_type: &JoinType, null_equality: NullEquality, left_sort_exprs: Option<LexOrdering>, right_sort_exprs: Option<LexOrdering>, mode: StreamJoinPartitionMode) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec", "path": "SymmetricHashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [204, 1], "end": [364, 2], "filename": "src/joins/symmetric_hash_join.rs"}, "trait": null, "trait_path": null}`

Source: `src/joins/symmetric_hash_join.rs:212`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Tries to create a new [SymmetricHashJoinExec](../operations/datafusion_physical_plan.joins.symmetric_hash_join.SymmetricHashJoinExec.md#op-511e003503c5f02d17e62166).
# Error
This function errors when:
- It is not possible to join the left and right sides on keys `on`, or
- It fails to construct `SortedFilterExpr`s, or
- It fails to create the [ExprIntervalGraph](../operations/datafusion_physical_expr.intervals.cp_solver.ExprIntervalGraph.md#op-a9fe51c501098e92f55a1510).

<a id="op-69ff0585d1f8017802bbc7db"></a>
## try_swapping_with_projection

`function` · `datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec::try_swapping_with_projection` · datafusion-physical-plan 55.1.0

```rust
fn try_swapping_with_projection(&self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec", "path": "SymmetricHashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [406, 1], "end": [762, 2], "filename": "src/joins/symmetric_hash_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/symmetric_hash_join.rs:626`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Tries to swap the projection with its input [`SymmetricHashJoinExec`](../operations/datafusion_physical_plan.joins.symmetric_hash_join.SymmetricHashJoinExec.md#op-511e003503c5f02d17e62166). If it can be done,
it returns the new swapped version having the [`SymmetricHashJoinExec`](../operations/datafusion_physical_plan.joins.symmetric_hash_join.SymmetricHashJoinExec.md#op-511e003503c5f02d17e62166) as the top plan.
Otherwise, it returns None.

<a id="op-134fe3dd3e2499e3f6922e22"></a>
## try_to_proto

`function` · `datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec::try_to_proto` · datafusion-physical-plan 55.1.0

```rust
fn try_to_proto(&self, ctx: &proto::ExecutionPlanEncodeCtx<'_>) -> Result<Option<datafusion_proto_models::protobuf::PhysicalPlanNode>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec", "path": "SymmetricHashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [406, 1], "end": [762, 2], "filename": "src/joins/symmetric_hash_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/symmetric_hash_join.rs:663`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad7f678ed5497bb385030434"></a>
## with_new_children

`function` · `datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec::with_new_children` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec", "path": "SymmetricHashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [406, 1], "end": [762, 2], "filename": "src/joins/symmetric_hash_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/symmetric_hash_join.rs:498`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db3f23fff936ca46624141d0"></a>
## with_new_children_and_same_properties

`function` · `datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec::with_new_children_and_same_properties` · datafusion-physical-plan 55.1.0

```rust
fn with_new_children_and_same_properties(Arc<self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::symmetric_hash_join::SymmetricHashJoinExec", "path": "SymmetricHashJoinExec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [406, 1], "end": [762, 2], "filename": "src/joins/symmetric_hash_join.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::execution_plan::ExecutionPlan", "path": "ExecutionPlan"}, "trait_path": "datafusion_physical_plan::execution_plan::ExecutionPlan"}`

Source: `src/joins/symmetric_hash_join.rs:508`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
