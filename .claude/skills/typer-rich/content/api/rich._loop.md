# `rich._loop`

Distribution: `rich`

## T

`rich._loop.T`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
T = TypeVar('T')
```

## loop_first

Import as `rich.tree.loop_first`  ·  defined at `rich._loop.loop_first`

```python
def loop_first(values: Iterable[T]) -> Iterable[Tuple[bool, T]]
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Iterate and generate a tuple with a flag for first value.


## loop_first_last

Import as `rich.table.loop_first_last`  ·  defined at `rich._loop.loop_first_last`

```python
def loop_first_last(values: Iterable[T]) -> Iterable[Tuple[bool, bool, T]]
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Iterate and generate a tuple with a flag for first and last value.


## loop_last

Import as `rich.box.loop_last`  ·  defined at `rich._loop.loop_last`

```python
def loop_last(values: Iterable[T]) -> Iterable[Tuple[bool, T]]
```

_9 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Iterate and generate a tuple with a flag for last value.


