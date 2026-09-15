# `datafusion_physical_expr::intervals::cp_solver`

Crate `datafusion-physical-expr` · 5 public items · structured records in [`model/datafusion_physical_expr.intervals.cp_solver.json`](../model/datafusion_physical_expr.intervals.cp_solver.json)

## PropagationResult

`enum` · `datafusion_physical_expr::intervals::cp_solver::PropagationResult`

```rust
enum PropagationResult
```

**Variants**: `CannotPropagate`, `Infeasible`, `Success`

**Derives**: Debug, PartialEq, StructuralPartialEq

This object encapsulates all possible constraint propagation results.

---

## propagate_arithmetic

`function` · `datafusion_physical_expr::intervals::cp_solver::propagate_arithmetic`

```rust
fn propagate_arithmetic(op: &datafusion_expr::Operator, parent: &datafusion_expr::interval_arithmetic::Interval, left_child: &datafusion_expr::interval_arithmetic::Interval, right_child: &datafusion_expr::interval_arithmetic::Interval) -> datafusion_common::Result<Option<(datafusion_expr::interval_arithmetic::Interval, datafusion_expr::interval_arithmetic::Interval)>>
```

This function refines intervals `left_child` and `right_child` by applying
constraint propagation through `parent` via operation. The main idea is
that we can shrink ranges of variables x and y using parent interval p.

Assuming that x,y and p has ranges `[xL, xU]`, `[yL, yU]`, and `[pL, pU]`, we
apply the following operations:
- For plus operation, specifically, we would first do
    - `[xL, xU]` <- (`[pL, pU]` - `[yL, yU]`) ∩ `[xL, xU]`, and then
    - `[yL, yU]` <- (`[pL, pU]` - `[xL, xU]`) ∩ `[yL, yU]`.
- For minus operation, specifically, we would first do
    - `[xL, xU]` <- (`[yL, yU]` + `[pL, pU]`) ∩ `[xL, xU]`, and then
    - `[yL, yU]` <- (`[xL, xU]` - `[pL, pU]`) ∩ `[yL, yU]`.
- For multiplication operation, specifically, we would first do
    - `[xL, xU]` <- (`[pL, pU]` / `[yL, yU]`) ∩ `[xL, xU]`, and then
    - `[yL, yU]` <- (`[pL, pU]` / `[xL, xU]`) ∩ `[yL, yU]`.
- For division operation, specifically, we would first do
    - `[xL, xU]` <- (`[yL, yU]` * `[pL, pU]`) ∩ `[xL, xU]`, and then
    - `[yL, yU]` <- (`[xL, xU]` / `[pL, pU]`) ∩ `[yL, yU]`.

---

## propagate_comparison

`function` · `datafusion_physical_expr::intervals::cp_solver::propagate_comparison`

```rust
fn propagate_comparison(op: &datafusion_expr::Operator, parent: &datafusion_expr::interval_arithmetic::Interval, left_child: &datafusion_expr::interval_arithmetic::Interval, right_child: &datafusion_expr::interval_arithmetic::Interval) -> datafusion_common::Result<Option<(datafusion_expr::interval_arithmetic::Interval, datafusion_expr::interval_arithmetic::Interval)>>
```

This function refines intervals `left_child` and `right_child` by applying
comparison propagation through `parent` via operation. The main idea is
that we can shrink ranges of variables x and y using parent interval p.
Two intervals can be ordered in 6 ways for a Gt `>` operator:
```text
                          (1): Infeasible, short-circuit
left:   |        ================                                               |
right:  |                           ========================                    |

                            (2): Update both interval
left:   |              ======================                                   |
right:  |                             ======================                    |
                                         |
                                         V
left:   |                             =======                                   |
right:  |                             =======                                   |

                            (3): Update left interval
left:   |                  ==============================                       |
right:  |                           ==========                                  |
                                         |
                                         V
left:   |                           =====================                       |
right:  |                           ==========                                  |

                            (4): Update right interval
left:   |                           ==========                                  |
right:  |                   ===========================                         |
                                         |
                                         V
left:   |                           ==========                                  |
right   |                   ==================                                  |

                                  (5): No change
left:   |                       ============================                    |
right:  |               ===================                                     |

                                  (6): No change
left:   |                                    ====================               |
right:  |                ===============                                        |

        -inf --------------------------------------------------------------- +inf
```

---

## ExprIntervalGraph

`struct` · `datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraph`

```rust
struct ExprIntervalGraph
```

**Derives**: Clone, Debug

**Methods** (9)

```rust
fn assign_intervals(&mut self, assignments: &[(usize, Interval)])
fn evaluate_bounds(&mut self) -> Result<&Interval>
fn gather_node_indices(&mut self, exprs: &[Arc<dyn PhysicalExpr>]) -> Vec<(Arc<dyn PhysicalExpr>, usize)>
fn get_interval(&self, index: usize) -> Interval
fn node_count(&self) -> usize
fn size(&self) -> usize
fn try_new(expr: Arc<dyn PhysicalExpr>, schema: &Schema) -> Result<Self>
fn update_intervals(&self, assignments: &mut [(usize, Interval)])
fn update_ranges(&mut self, leaf_bounds: &mut [(usize, Interval)], given_range: Interval) -> Result<PropagationResult>
```

This object implements a directed acyclic expression graph (DAEG) that
is used to compute ranges for expressions through interval arithmetic.

---

## ExprIntervalGraphNode

`struct` · `datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraphNode`

```rust
struct ExprIntervalGraphNode
```

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug, PartialEq

**Methods** (4)

```rust
fn interval(&self) -> &Interval
fn make_node(node: &ExprTreeNode<NodeIndex>, schema: &Schema) -> Result<Self>
fn new_unbounded(expr: Arc<dyn PhysicalExpr>, dt: &DataType) -> Result<Self>
fn new_with_interval(expr: Arc<dyn PhysicalExpr>, interval: Interval) -> Self
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

This is a node in the DAEG; it encapsulates a reference to the actual
[`PhysicalExpr`] as well as an interval containing expression bounds.

---
