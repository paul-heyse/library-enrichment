# `rich.measure`

Distribution: `rich`

## Measurement

`rich.measure.Measurement`

```python
class Measurement(NamedTuple)
```

_18 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `NamedTuple`

**Declared members (8)**

- `def clamp(self, min_width: Optional[int] = None, max_width: Optional[int] = None) -> Measurement`
  Clamp a measurement within the specified range.
- `def get(cls, console: Console, options: ConsoleOptions, renderable: RenderableType) -> Measurement`  _classmethod_
  Get a measurement for a renderable.
- `maximum: int`  _instance-attribute_
  Maximum number of cells required to render.
- `minimum: int`  _instance-attribute_
  Minimum number of cells required to render.
- `def normalize(self) -> Measurement`
  Get measurement that ensures that minimum <= maximum and minimum >= 0
- `span: int`  _property_
  Get difference between maximum and minimum.
- `def with_maximum(self, width: int) -> Measurement`
  Get a RenderableWith where the widths are <= width.
- `def with_minimum(self, width: int) -> Measurement`
  Get a RenderableWith where the widths are >= width.

Stores the minimum and maximum widths (in characters) required to render an object.


## measure_renderables

`rich.measure.measure_renderables`

```python
def measure_renderables(console: Console, options: ConsoleOptions, renderables: Sequence[RenderableType]) -> Measurement
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Get a measurement that would fit a number of renderables.

Args:
    console (~rich.console.Console): Console instance.
    options (~rich.console.ConsoleOptions): Console options.
    renderables (Iterable[RenderableType]): One or more renderable objects.

Returns:
    Measurement: Measurement object containing range of character widths required to
        contain all given renderables.


