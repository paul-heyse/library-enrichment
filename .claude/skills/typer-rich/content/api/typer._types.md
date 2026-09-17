# `typer._types`

Distribution: `typer`

## ParamTypeValue

`typer._types.ParamTypeValue`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
ParamTypeValue = TypeVar('ParamTypeValue')
```

## TyperChoice

Import as `typer.main.TyperChoice`  ·  defined at `typer._types.TyperChoice`

```python
class TyperChoice(types.ParamType, Generic[ParamTypeValue])
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `types.ParamType`, `Generic[ParamTypeValue]`

**Declared members (9)**

- `case_sensitive = case_sensitive`  _instance-attribute_
- `choices: Sequence[ParamTypeValue] = tuple(choices)`  _instance-attribute_
- `def convert(self, value: Any, param: _click.Parameter | None, ctx: _click.Context | None) -> ParamTypeValue`
  For a given value from the parser, normalize it and find its matching normalized value in the list of choices. Then return the matched "original" choice.
- `def get_invalid_choice_message(self, value: Any, ctx: _click.Context | None) -> str`
  Get the error message when the given choice is invalid.
- `def get_metavar(self, param: _click.Parameter, ctx: _click.Context) -> str | None`
- `def get_missing_message(self, param: _click.Parameter, ctx: _click.Context | None) -> str`
  Message shown when no choice is passed.
- `name = 'choice'`  _class-attribute, instance-attribute_
- `def normalize_choice(self, choice: ParamTypeValue, ctx: _click.Context | None) -> str`
- `def shell_complete(self, ctx: _click.Context, param: _click.Parameter, incomplete: str) -> list[CompletionItem]`
  Complete choices that start with the incomplete value.

**Inherited (5)**

- from `typer._click.types.ParamType`: `arity`, `envvar_list_splitter`, `fail`, `is_composite`, `split_envvar_value`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

