# `rich._stack`

Distribution: `rich`

## T

`rich._stack.T`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
T = TypeVar('T')
```

## Stack

Import as `rich.markdown.Stack`  ·  defined at `rich._stack.Stack`

```python
class Stack(List[T])
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `List[T]`

**Declared members (2)**

- `def push(self, item: T) -> None`
  Push an item on to the stack (append in stack nomenclature).
- `top: T`  _property_
  Get top of stack.

A small shim over builtin list.


