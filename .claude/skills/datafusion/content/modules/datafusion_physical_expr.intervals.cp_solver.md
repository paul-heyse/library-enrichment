# `datafusion_physical_expr::intervals::cp_solver`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.intervals.cp_solver.json).

<a id="op-cfbaa171e379ecd5df060692"></a>
## cp_solver

`module` · `datafusion_physical_expr::intervals::cp_solver` · datafusion-physical-expr 55.1.0

```rust
mod cp_solver
```

Source: `src/intervals/cp_solver.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Constraint propagator/solver for custom [`PhysicalExpr`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-fe8284c43330456b0d4e6af7) graphs.

The constraint propagator/solver in DataFusion uses interval arithmetic to
perform mathematical operations on intervals, which represent a range of
possible values rather than a single point value. This allows for the
propagation of ranges through mathematical operations, and can be used to
compute bounds for a complicated expression. The key idea is that by
breaking down a complicated expression into simpler terms, and then
combining the bounds for those simpler terms, one can obtain bounds for the
overall expression.

This way of using interval arithmetic to compute bounds for a complex
expression by combining the bounds for the constituent terms within the
original expression allows us to reason about the range of possible values
of the expression. This information later can be used in range pruning of
the provably unnecessary parts of `RecordBatch`es.

# Example

For example, consider a mathematical expression such as `x^2 + y = 4` \[1\].
Since this expression would be a binary tree in [`PhysicalExpr`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-fe8284c43330456b0d4e6af7) notation,
this type of an hierarchical computation is well-suited for a graph based
implementation. In such an implementation, an equation system `f(x) = 0` is
represented by a directed acyclic expression graph (DAEG).

In order to use interval arithmetic to compute bounds for this expression,
one would first determine intervals that represent the possible values of
`x` and `y` Let's say that the interval for `x` is `[1, 2]` and the interval
for `y` is `[-3, 1]`. In the chart below, you can see how the computation
takes place.

# References

1. Kabak, Mehmet Ozan. Analog Circuit Start-Up Behavior Analysis: An Interval
   Arithmetic Based Approach, Chapter 4. Stanford University, 2015.
2. Moore, Ramon E. Interval analysis. Vol. 4. Englewood Cliffs: Prentice-Hall, 1966.
3. F. Messine, "Deterministic global optimization using interval constraint
   propagation techniques," RAIRO-Operations Research, vol. 38, no. 04,
   pp. 277-293, 2004.

# Illustration

## Computing bounds for an expression using interval arithmetic

```text
            +-----+                         +-----+
       +----|  +  |----+               +----|  +  |----+
       |    |     |    |               |    |     |    |
       |    +-----+    |               |    +-----+    |
       |               |               |               |
   +-----+           +-----+       +-----+           +-----+
   |   2 |           |  y  |       |   2 | [1, 4]    |  y  |
   |[.]  |           |     |       |[.]  |           |     |
   +-----+           +-----+       +-----+           +-----+
      |                               |
      |                               |
    +---+                           +---+
    | x | [1, 2]                    | x | [1, 2]
    +---+                           +---+

 (a) Bottom-up evaluation: Step 1 (b) Bottom up evaluation: Step 2

                                     [1 - 3, 4 + 1] = [-2, 5]
            +-----+                         +-----+
       +----|  +  |----+               +----|  +  |----+
       |    |     |    |               |    |     |    |
       |    +-----+    |               |    +-----+    |
       |               |               |               |
   +-----+           +-----+       +-----+           +-----+
   |   2 |[1, 4]     |  y  |       |   2 |[1, 4]     |  y  |
   |[.]  |           |     |       |[.]  |           |     |
   +-----+           +-----+       +-----+           +-----+
      |              [-3, 1]          |              [-3, 1]
      |                               |
    +---+                           +---+
    | x | [1, 2]                    | x | [1, 2]
    +---+                           +---+

 (c) Bottom-up evaluation: Step 3 (d) Bottom-up evaluation: Step 4
```

## Top-down constraint propagation using inverse semantics

```text
   [-2, 5] ∩ [4, 4] = [4, 4]               [4, 4]
           +-----+                         +-----+
      +----|  +  |----+               +----|  +  |----+
      |    |     |    |               |    |     |    |
      |    +-----+    |               |    +-----+    |
      |               |               |               |
   +-----+           +-----+       +-----+           +-----+
   |   2 | [1, 4]    |  y  |       |   2 | [1, 4]    |  y  | [0, 1]*
   |[.]  |           |     |       |[.]  |           |     |
   +-----+           +-----+       +-----+           +-----+
     |              [-3, 1]          |
     |                               |
   +---+                           +---+
   | x | [1, 2]                    | x | [1, 2]
   +---+                           +---+

 (a) Top-down propagation: Step 1 (b) Top-down propagation: Step 2

                                    [1 - 3, 4 + 1] = [-2, 5]
           +-----+                         +-----+
      +----|  +  |----+               +----|  +  |----+
      |    |     |    |               |    |     |    |
      |    +-----+    |               |    +-----+    |
      |               |               |               |
   +-----+           +-----+       +-----+           +-----+
   |   2 |[3, 4]**   |  y  |       |   2 |[3, 4]     |  y  |
   |[.]  |           |     |       |[.]  |           |     |
   +-----+           +-----+       +-----+           +-----+
     |              [0, 1]           |              [-3, 1]
     |                               |
   +---+                           +---+
   | x | [1, 2]                    | x | [sqrt(3), 2]***
   +---+                           +---+

 (c) Top-down propagation: Step 3  (d) Top-down propagation: Step 4

   * [-3, 1] ∩ ([4, 4] - [1, 4]) = [0, 1]
   ** [1, 4] ∩ ([4, 4] - [0, 1]) = [3, 4]
   *** [1, 2] ∩ [sqrt(3), sqrt(4)] = [sqrt(3), 2]
```
