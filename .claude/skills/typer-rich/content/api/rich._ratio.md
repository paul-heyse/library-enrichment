# `rich._ratio`

Distribution: `rich`

## resolved

`rich._ratio.resolved`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
resolved = ratio_resolve(110, [E(None, 1, 1), E(None, 1, 1), E(None, 1, 1)])
```

## E

`rich._ratio.E`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class E
```

**Declared members (3)**

- `minimum_size: int = 1`  _class-attribute, instance-attribute_
- `ratio: int = 1`  _class-attribute, instance-attribute_
- `size: Optional[int] = None`  _class-attribute, instance-attribute_

## Edge

`rich._ratio.Edge`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class Edge(Protocol)
```

**Bases** `Protocol`

**Declared members (3)**

- `minimum_size: int = 1`  _class-attribute, instance-attribute_
- `ratio: int = 1`  _class-attribute, instance-attribute_
- `size: Optional[int] = None`  _class-attribute, instance-attribute_

Any object that defines an edge (such as Layout).


## ratio_distribute

Import as `rich.table.ratio_distribute`  ·  defined at `rich._ratio.ratio_distribute`

```python
def ratio_distribute(total: int, ratios: List[int], minimums: Optional[List[int]] = None) -> List[int]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Distribute an integer total in to parts based on ratios.

Args:
    total (int): The total to divide.
    ratios (List[int]): A list of integer ratios.
    minimums (List[int]): List of minimum values for each slot.

Returns:
    List[int]: A list of integers guaranteed to sum to total.


## ratio_reduce

Import as `rich.table.ratio_reduce`  ·  defined at `rich._ratio.ratio_reduce`

```python
def ratio_reduce(total: int, ratios: List[int], maximums: List[int], values: List[int]) -> List[int]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Divide an integer total in to parts based on ratios.

Args:
    total (int): The total to divide.
    ratios (List[int]): A list of integer ratios.
    maximums (List[int]): List of maximums values for each slot.
    values (List[int]): List of values

Returns:
    List[int]: A list of integers guaranteed to sum to total.


## ratio_resolve

Import as `rich.layout.ratio_resolve`  ·  defined at `rich._ratio.ratio_resolve`

```python
def ratio_resolve(total: int, edges: Sequence[Edge]) -> List[int]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Divide total space to satisfy size, ratio, and minimum_size, constraints.

The returned list of integers should add up to total in most cases, unless it is
impossible to satisfy all the constraints. For instance, if there are two edges
with a minimum size of 20 each and `total` is 30 then the returned list will be
greater than total. In practice, this would mean that a Layout object would
clip the rows that would overflow the screen height.

Args:
    total (int): Total number of characters.
    edges (List[Edge]): Edges within total space.

Returns:
    List[int]: Number of characters for each edge.


