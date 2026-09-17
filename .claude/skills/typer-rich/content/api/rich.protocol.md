# `rich.protocol`

Distribution: `rich`

## _GIBBERISH

`rich.protocol._GIBBERISH`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_GIBBERISH = 'aihwerij235234ljsdnp34ksodfipwoe234234jlskjdf'
```

## is_renderable

`rich.protocol.is_renderable`

```python
def is_renderable(check_object: Any) -> bool
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Check if an object may be rendered by Rich.


## rich_cast

`rich.protocol.rich_cast`

```python
def rich_cast(renderable: object) -> RenderableType
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Cast an object to a renderable by calling __rich__ if present.

Args:
    renderable (object): A potentially renderable object

Returns:
    object: The result of recursively calling __rich__.


