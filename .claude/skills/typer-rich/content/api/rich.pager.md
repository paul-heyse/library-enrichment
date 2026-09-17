# `rich.pager`

Distribution: `rich`

## console

`rich.pager.console`

```python
console = Console()
```

**Inferred type** (`ty`, not declared in the source): `Console`

## Pager

`rich.pager.Pager`

```python
class Pager(ABC)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `ABC`

**Declared members (1)**

- `def show(self, content: str) -> None`  _abstractmethod_
  Show content in pager.

Base class for a pager.


## SystemPager

`rich.pager.SystemPager`

```python
class SystemPager(Pager)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Pager`

**Declared members (1)**

- `def show(self, content: str) -> None`
  Use the same pager used by pydoc.

Uses the pager installed on the system.


