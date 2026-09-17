# Turning a command-line string into your own type

**kind** `abstract-base` · **supported** `documented` · **subject** `click`

An abstract base. The required members are the obligations; everything else is provided, and provided is where the capability hides.

Upstream documents this.

## Entry points

- `typer._click.types.ParamType` · defined at `typer._click.types.ParamType` — not importable; reached subclassed-via · `api/typer._click.types.md`

## Mental model

`ParamType` lives in `typer._click.types` -- Typer's vendored copy of Click -- and is not exported from `typer` at all. Every annotation you write becomes one of its subclasses: `int` becomes an `IntParamType`, `Annotated[int, typer.Option(min=1)]` becomes an `IntRange`. Customizing the field types with Click-specific types is not supported, and this is the list of types that statement is about.

## Implementors (16)

- `typer._click.types.BoolParamType`
- `typer._click.types.CompositeParamType`
- `typer._click.types.DateTime`
- `typer._click.types.File`
- `typer._click.types.FloatParamType`
- `typer._click.types.FloatRange`
- `typer._click.types.FuncParamType`
- `typer._click.types.IntParamType`
- `typer._click.types.IntRange`
- `typer._click.types.StringParamType`
- `typer._click.types.Tuple`
- `typer._click.types.UUIDParameterType`
- `typer._click.types._NumberParamTypeBase`
- `typer._click.types._NumberRangeBase`
- `typer.main.TyperChoice`
- `typer.models.TyperPath`

## Decision rules

- A standard conversion exists: use the annotation. That is Typer's whole proposition.
- You need a custom conversion: prefer a `typer.Option(parser=...)` or a callback over subclassing a private class.
- You are reading a `ParamType` row to understand what your annotation DOES: that is what these rows are for.

## Anti-patterns

- `from typer._click.types import ParamType`: it is private, upstream says so, and the path is not part of any promise.
- Passing a `click.ParamType` from the installed `click` package: different class, different library, and it still type-checks.

## Observed

Probe `V003` — see `content/probes/00-index.md`.
