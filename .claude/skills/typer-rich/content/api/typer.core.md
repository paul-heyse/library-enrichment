# `typer.core`

Distribution: `typer`

## DEFAULT_MARKUP_MODE

`typer.core.DEFAULT_MARKUP_MODE`

```python
DEFAULT_MARKUP_MODE: MarkupMode = 'rich'
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## HAS_RICH

`typer.core.HAS_RICH`

```python
HAS_RICH = parse_boolean_env_var(os.getenv('TYPER_USE_RICH'), default=True)
```

**Inferred type** (`ty`, not declared in the source): `bool`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## MARKUP_MODE_KEY

`typer.core.MARKUP_MODE_KEY`

```python
MARKUP_MODE_KEY = 'TYPER_RICH_MARKUP_MODE'
```

**Inferred type** (`ty`, not declared in the source): `Literal["TYPER_RICH_MARKUP_MODE"]`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## MarkupMode

`typer.core.MarkupMode`

```python
MarkupMode = Literal['markdown', 'rich', None]
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["markdown", "rich"] | None'> ````

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## TyperArgument

`typer.core.TyperArgument`

```python
class TyperArgument(_click.core.Parameter)
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `_click.core.Parameter`

**Declared members (14)**

- `def add_to_parser(self, parser: _OptionParser, ctx: _click.Context) -> None`
- `def get_error_hint(self, ctx: _click.Context) -> str`
- `def get_help_record(self, ctx: _click.Context) -> tuple[str, str] | None`
- `def get_usage_pieces(self, ctx: _click.Context) -> list[str]`
- `help = help`  _instance-attribute_
- `hidden = hidden`  _instance-attribute_
- `human_readable_name: str`  _property_
- `def make_metavar(self, ctx: _click.Context, usage: bool = False) -> str`
- `param_type_name = 'argument'`  _class-attribute, instance-attribute_
- `rich_help_panel = rich_help_panel`  _instance-attribute_
- `show_choices = show_choices`  _instance-attribute_
- `show_default = show_default`  _instance-attribute_
- `show_envvar = show_envvar`  _instance-attribute_
- `def value_is_missing(self, value: Any) -> bool`

**Inherited (21)**

- from `typer._click.core.Parameter`: `callback`, `consume_value`, `default`, `envvar`, `expose_value`, `get_default`, `handle_parse_result`, `is_eager`, `metavar`, `multiple`, `name`, `nargs`, `opts`, `process_value`, `required`, `resolve_envvar_value`, `secondary_opts`, `shell_complete`, `type`, `type_cast_value`, `value_from_envvar`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## TyperCommand

`typer.core.TyperCommand`

```python
class TyperCommand(_click.core.Command)
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `_click.core.Command`

**Declared members (5)**

- `def format_help(self, ctx: _click.Context, formatter: _click.HelpFormatter) -> None`
- `def format_options(self, ctx: _click.Context, formatter: _click.HelpFormatter) -> None`
- `def main(self, args: Sequence[str] | None = None, prog_name: str | None = None, complete_var: str | None = None, standalone_mode: bool = True, windows_expand_args: bool = True, extra: Any = {}) -> Any`
- `rich_help_panel = rich_help_panel`  _instance-attribute_
- `rich_markup_mode: MarkupMode = rich_markup_mode`  _instance-attribute_

**Inherited (31)**

- from `typer._click.core.Command`: `add_help_option`, `allow_extra_args`, `allow_interspersed_args`, `callback`, `collect_usage_pieces`, `context_class`, `context_settings`, `deprecated`, `epilog`, `format_epilog`, `format_help_text`, `format_usage`, `get_help`, `get_help_option`, `get_help_option_names`, `get_params`, `get_short_help_str`, `get_usage`, `help`, `hidden`, `ignore_unknown_options`, `invoke`, `make_context`, `make_parser`, `name`, `no_args_is_help`, `options_metavar`, `params`, `parse_args`, `shell_complete`, `short_help`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## TyperGroup

`typer.core.TyperGroup`

```python
class TyperGroup(_click.Command)
```

_5 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `_click.Command`

**Declared members (23)**

- `def add_command(self, cmd: _click.Command, name: str | None = None) -> None`
- `allow_extra_args = True`  _class-attribute, instance-attribute_
- `allow_interspersed_args = False`  _class-attribute, instance-attribute_
- `def collect_usage_pieces(self, ctx: _click.Context) -> list[str]`
- `command_class: type[_click.Command] | None = None`  _class-attribute, instance-attribute_
- `commands = cast(MutableMapping[str, _click.Command], commands)`  _instance-attribute_
- `def format_commands(self, ctx: _click.Context, formatter: _click.HelpFormatter) -> None`
- `def format_help(self, ctx: _click.Context, formatter: _click.HelpFormatter) -> None`
- `def format_options(self, ctx: _click.Context, formatter: _click.HelpFormatter) -> None`
- `def get_command(self, ctx: _click.Context, cmd_name: str) -> _click.Command | None`
- `group_class: type[TyperGroup] | type[type] | None = None`  _class-attribute, instance-attribute_
- `def invoke(self, ctx: _click.Context) -> Any`
- `invoke_without_command = invoke_without_command`  _instance-attribute_
- `def list_commands(self, ctx: _click.Context) -> list[str]`
  Returns a list of subcommand names, maintaining the original order of creation (cf Issue #933)
- `def main(self, args: Sequence[str] | None = None, prog_name: str | None = None, complete_var: str | None = None, standalone_mode: bool = True, windows_expand_args: bool = True, extra: Any = {}) -> Any`
- `no_args_is_help = no_args_is_help`  _instance-attribute_
- `def parse_args(self, ctx: _click.Context, args: list[str]) -> list[str]`
- `def resolve_command(self, ctx: _click.Context, args: list[str]) -> tuple[str | None, _click.Command | None, list[str]]`
- `rich_help_panel = rich_help_panel`  _instance-attribute_
- `rich_markup_mode: MarkupMode = rich_markup_mode`  _instance-attribute_
- `def shell_complete(self, ctx: _click.Context, incomplete: str) -> list[CompletionItem]`
  Return a list of completions for the incomplete value. Looks at the names of options, subcommands, and chained multi-commands.
- `subcommand_metavar = subcommand_metavar`  _instance-attribute_
- `suggest_commands = suggest_commands`  _instance-attribute_

**Inherited (24)**

- from `typer._click.core.Command`: `add_help_option`, `callback`, `context_class`, `context_settings`, `deprecated`, `epilog`, `format_epilog`, `format_help_text`, `format_usage`, `get_help`, `get_help_option`, `get_help_option_names`, `get_params`, `get_short_help_str`, `get_usage`, `help`, `hidden`, `ignore_unknown_options`, `make_context`, `make_parser`, `name`, `options_metavar`, `params`, `short_help`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## TyperOption

`typer.core.TyperOption`

```python
class TyperOption(_click.Parameter)
```

_6 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `_click.Parameter`

**Declared members (25)**

- `def add_to_parser(self, parser: _OptionParser, ctx: _click.Context) -> None`
- `allow_from_autoenv = allow_from_autoenv`  _instance-attribute_
- `confirmation_prompt = confirmation_prompt`  _instance-attribute_
- `def consume_value(self, ctx: _click.Context, opts: Mapping[str, _click.Parameter]) -> tuple[Any, _click.core.ParameterSource]`
  For `Option`, the value can be collected from an interactive prompt if the option is a flag that needs a value (and the `prompt` property is set).
- `count = count`  _instance-attribute_
- `def get_error_hint(self, ctx: _click.Context) -> str`
- `def get_help_record(self, ctx: _click.Context) -> tuple[str, str] | None`
- `help = help`  _instance-attribute_
- `hidden = hidden`  _instance-attribute_
- `hide_input = hide_input`  _instance-attribute_
- `is_bool_flag: bool = bool(is_flag and isinstance(self.type, types.BoolParamType))`  _instance-attribute_
- `is_flag: bool = bool(is_flag)`  _instance-attribute_
- `def make_metavar(self, ctx: _click.Context) -> str`
- `param_type_name = 'option'`  _class-attribute, instance-attribute_
- `prompt = prompt_text`  _instance-attribute_
- `def prompt_for_value(self, ctx: _click.Context) -> Any`
  This is an alternative flow that can be activated in the full value processing if a value does not exist.  It will prompt the user until a valid value exists and then returns the processed value as result.
- `prompt_required = prompt_required`  _instance-attribute_
- `def resolve_envvar_value(self, ctx: _click.Context) -> str | None`
- `rich_help_panel = rich_help_panel`  _instance-attribute_
- `show_choices = show_choices`  _instance-attribute_
- `show_default = show_default`  _instance-attribute_
- `show_envvar = show_envvar`  _instance-attribute_
- `type: types.ParamType = types.BoolParamType()`  _instance-attribute_
- `def value_from_envvar(self, ctx: _click.Context) -> Any`
- `def value_is_missing(self, value: Any) -> bool`

**Inherited (19)**

- from `typer._click.core.Parameter`: `callback`, `default`, `envvar`, `expose_value`, `get_default`, `get_usage_pieces`, `handle_parse_result`, `human_readable_name`, `is_eager`, `metavar`, `multiple`, `name`, `nargs`, `opts`, `process_value`, `required`, `secondary_opts`, `shell_complete`, `type_cast_value`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## _extract_default_help_str

`typer.core._extract_default_help_str`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _extract_default_help_str(obj: Union[TyperArgument, TyperOption], ctx: _click.Context) -> Any | Callable[[], Any] | None
```

## _get_default_string

`typer.core._get_default_string`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _get_default_string(obj: Union[TyperArgument, TyperOption], ctx: _click.Context, show_default_is_str: bool, default_value: list[Any] | tuple[Any, ...] | str | Callable[..., Any] | Any) -> str
```

## _main

`typer.core._main`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _main(self: _click.Command, args: Sequence[str] | None = None, prog_name: str | None = None, complete_var: str | None = None, standalone_mode: bool = True, windows_expand_args: bool = True, rich_markup_mode: MarkupMode = DEFAULT_MARKUP_MODE, extra: Any = {}) -> Any
```

## _split_opt

`typer.core._split_opt`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _split_opt(opt: str) -> tuple[str, str]
```

## _typer_format_options

`typer.core._typer_format_options`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _typer_format_options(self: _click.core.Command, ctx: _click.Context, formatter: _click.HelpFormatter) -> None
```

## _typer_main_shell_completion

`typer.core._typer_main_shell_completion`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _typer_main_shell_completion(self: _click.core.Command, ctx_args: MutableMapping[str, Any], prog_name: str, complete_var: str | None = None) -> None
```

## _typer_param_setup_autocompletion_compat

`typer.core._typer_param_setup_autocompletion_compat`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _typer_param_setup_autocompletion_compat(self: _click.Parameter, autocompletion: Callable[[_click.Context, list[str], str], list[tuple[str, str] | str]] | None = None) -> None
```

## _value_is_missing

`typer.core._value_is_missing`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _value_is_missing(param: _click.Parameter, value: Any) -> bool
```

