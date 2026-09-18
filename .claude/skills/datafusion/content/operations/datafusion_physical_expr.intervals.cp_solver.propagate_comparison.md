# `datafusion_physical_expr::intervals::cp_solver::propagate_comparison`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.intervals.cp_solver.propagate_comparison.json).

<a id="op-069a16ae647c5f6fbdc0fc64"></a>
## propagate_comparison

`function` · `datafusion_physical_expr::intervals::cp_solver::propagate_comparison` · datafusion-physical-expr 55.1.0

```rust
fn propagate_comparison(op: &datafusion_expr::Operator, parent: &datafusion_expr::interval_arithmetic::Interval, left_child: &datafusion_expr::interval_arithmetic::Interval, right_child: &datafusion_expr::interval_arithmetic::Interval) -> datafusion_common::Result<Option<(datafusion_expr::interval_arithmetic::Interval, datafusion_expr::interval_arithmetic::Interval)>>
```

Source: `src/intervals/cp_solver.rs:342`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

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
