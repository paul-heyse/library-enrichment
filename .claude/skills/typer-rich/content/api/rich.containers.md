# `rich.containers`

Distribution: `rich`

## T

`rich.containers.T`

```python
T = TypeVar('T')
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

## Lines

`rich.containers.Lines`

```python
class Lines
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (4)**

- `def append(self, line: Text) -> None`
- `def extend(self, lines: Iterable[Text]) -> None`
- `def justify(self, console: Console, width: int, justify: JustifyMethod = 'left', overflow: OverflowMethod = 'fold') -> None`
  Justify and overflow text to a given width.
- `def pop(self, index: int = -1) -> Text`

A list subclass which can render to the console.


## Renderables

`rich.containers.Renderables`

```python
class Renderables
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (1)**

- `def append(self, renderable: RenderableType) -> None`

A list subclass which renders its contents to the console.


