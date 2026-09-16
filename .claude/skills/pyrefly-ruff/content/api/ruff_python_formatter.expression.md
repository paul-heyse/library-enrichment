# `ruff_python_formatter::expression`

Crate `ruff_python_formatter` · 3 public items · structured records in [`model/ruff_python_formatter.expression.json`](../model/ruff_python_formatter.expression.json)

## AttributeState

`enum` · `ruff_python_formatter::expression::AttributeState`

```rust
enum AttributeState
```

**Variants**: `CallLikePreceding`, `FirstCallLike`, `BeforeFirstCallLike`

Records information about the current position within
a call chain.

---

## CallChainLayout

`enum` · `ruff_python_formatter::expression::CallChainLayout`

```rust
enum CallChainLayout
```

**Variants**: `Default`, `Fluent`, `NonFluent`

A call chain consists only of attribute access (`.` operator), function/method calls and
subscripts. We use fluent style for the call chain if there are at least two attribute dots
after call parentheses or subscript brackets. In case of fluent style the parentheses/bracket
will close on the previous line and the dot gets its own line, otherwise the line will start
with the closing parentheses/bracket and the dot follows immediately after.

Below, the left hand side of the addition has only a single attribute access after a call, the
second `.filter`. The first `.filter` is a call, but it doesn't follow a call. The right hand
side has two, the `.limit_results` after the call and the `.filter` after the subscript, so it
gets formatted in fluent style. The outer expression we assign to `blogs` has zero since the
`.all` follows attribute parentheses and not call parentheses.

```python
blogs = (
    Blog.objects.filter(
        entry__headline__contains="Lennon",
    ).filter(
        entry__pub_date__year=2008,
    )
    + Blog.objects.filter(
        entry__headline__contains="McCartney",
    )
    .limit_results[:10]
    .filter(
        entry__pub_date__year=2010,
    )
).all()
```

In [`preview`](crate::preview::is_fluent_layout_split_first_call_enabled), we also track the position of the leftmost call or
subscript on an attribute in the chain and break just before the dot.

So, for example, the right-hand summand in the above expression
would get formatted as:
```python
    Blog.objects
    .filter(
        entry__headline__contains="McCartney",
    )
    .limit_results[:10]
    .filter(
        entry__pub_date__year=2010,
    )
```

---

## FormatExpr

`struct` · `ruff_python_formatter::expression::FormatExpr`

```rust
struct FormatExpr
```

---
