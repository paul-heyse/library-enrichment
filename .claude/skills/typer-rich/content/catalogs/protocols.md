# Protocol conformance

Structural conformance is not decidable by reading member names, which is why this
index refused to guess at it. It *is* decidable by asking the type checker, so each row
below is a probe it adjudicated:

```python
def _p(c: SomeClass) -> SomeProtocol:
    return c
```

No diagnostic means assignable. A `bad-return` names the obligation that failed.

51 of 51 probed pairs are assignable, across
3 protocols.

**What this does not say.** Only classes whose public members are a superset of the
protocol's were probed -- a class missing a member cannot satisfy it, so asking would
waste a probe, but it also means absence from this table is not a verdict. And the
conformance suite records that this checker deviates on **variance**
(`protocols_variance.py`, and the ParamSpec and TypeVarTuple variance tests), so a
verdict that turns on variance is worth confirming rather than trusting.

## Edge

Import as `rich._ratio.Edge`.

Satisfied by 2:

- `rich._ratio.E`
- `rich.layout.Layout`

## ConsoleRenderable

Import as `rich.console.ConsoleRenderable`.

Satisfied by 42:

- `rich.__main__.ColorBox`
- `rich.align.Align`
- `rich.align.VerticalCenter`
- `rich.bar.Bar`
- `rich.columns.Columns`
- `rich.console.Group`
- `rich.console.NewLine`
- `rich.console.ScreenUpdate`
- `rich.constrain.Constrain`
- `rich.containers.Lines`
- `rich.containers.Renderables`
- `rich.control.Control`
- … and 30 more

## RichCast

Import as `rich.console.RichCast`.

Satisfied by 7:

- `rich._inspect.Inspect`
- `rich.color.Color`
- `rich.json.JSON`
- `rich.palette.Palette`
- `rich.progress.Progress`
- `rich.prompt.InvalidResponse`
- `rich.status.Status`
