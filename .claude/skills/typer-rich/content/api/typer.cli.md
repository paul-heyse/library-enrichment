# `typer.cli`

Distribution: `typer`

## app

`typer.cli.app`

```python
app = typer.Typer()
```

**Inferred type** (`ty`, not declared in the source): `Typer`

## default_app_names

`typer.cli.default_app_names`

```python
default_app_names = ('app', 'cli', 'main')
```

**Inferred type** (`ty`, not declared in the source): `tuple[Literal["app"], Literal["cli"], Literal["main"]]`

## default_func_names

`typer.cli.default_func_names`

```python
default_func_names = ('main', 'cli', 'app')
```

**Inferred type** (`ty`, not declared in the source): `tuple[Literal["main"], Literal["cli"], Literal["app"]]`

## state

`typer.cli.state`

```python
state = State()
```

**Inferred type** (`ty`, not declared in the source): `State`

## utils_app

`typer.cli.utils_app`

```python
utils_app = typer.Typer(help='Extra utility commands for Typer apps.')
```

**Inferred type** (`ty`, not declared in the source): `Typer`

## State

`typer.cli.State`

```python
class State
```

**Declared members (4)**

- `app: str | None = None`  _instance-attribute_
- `file: Path | None = None`  _instance-attribute_
- `func: str | None = None`  _instance-attribute_
- `module: str | None = None`  _instance-attribute_

## TyperCLIGroup

`typer.cli.TyperCLIGroup`

```python
class TyperCLIGroup(typer.core.TyperGroup)
```

**Bases** `typer.core.TyperGroup`

**Declared members (4)**

- `def get_command(self, ctx: _click.Context, name: str) -> Command | None`
- `def invoke(self, ctx: _click.Context) -> Any`
- `def list_commands(self, ctx: _click.Context) -> list[str]`
- `def maybe_add_run(self, ctx: _click.Context) -> None`

**Inherited (44)**

- from `typer._click.core.Command`: `add_help_option`, `callback`, `context_class`, `context_settings`, `deprecated`, `epilog`, `format_epilog`, `format_help_text`, `format_usage`, `get_help`, `get_help_option`, `get_help_option_names`, `get_params`, `get_short_help_str`, `get_usage`, `help`, `hidden`, `ignore_unknown_options`, `make_context`, `make_parser`, `name`, `options_metavar`, `params`, `short_help`
- from `typer.core.TyperGroup`: `add_command`, `allow_extra_args`, `allow_interspersed_args`, `collect_usage_pieces`, `command_class`, `commands`, `format_commands`, `format_help`, `format_options`, `group_class`, `invoke_without_command`, `main`, `no_args_is_help`, `parse_args`, `resolve_command`, `rich_help_panel`, `rich_markup_mode`, `shell_complete`, `subcommand_metavar`, `suggest_commands`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

## _parse_html

`typer.cli._parse_html`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _parse_html(to_parse: bool, input_text: str) -> str
```

## callback

`typer.cli.callback`

```python
def callback(ctx: typer.Context, path_or_module: str = typer.Argument(None, metavar='PATH_OR_MODULE'), app: str = typer.Option(None, help='The typer app object/variable to use.'), func: str = typer.Option(None, help='The function to convert to Typer.'), version: bool = typer.Option(False, '--version', help='Print version and exit.', callback=print_version)) -> None
```

Run Typer scripts with completion, without having to create a package.

You probably want to install completion for the typer command:

$ typer --install-completion

https://typer.tiangolo.com/


## docs

`typer.cli.docs`

```python
def docs(ctx: typer.Context, name: str = typer.Option('', help='The name of the CLI program to use in docs.'), output: Path | None = typer.Option(None, help='An output file to write docs to, like README.md.', file_okay=True, dir_okay=False), title: str | None = typer.Option(None, help='The title for the documentation page. If not provided, the name of the program is used.')) -> None
```

Generate Markdown docs for a Typer app.


## get_docs_for_click

`typer.cli.get_docs_for_click`

```python
def get_docs_for_click(obj: Command, ctx: typer.Context, indent: int = 0, name: str = '', call_prefix: str = '', title: str | None = None) -> str
```

## get_typer_from_module

`typer.cli.get_typer_from_module`

```python
def get_typer_from_module(module: Any) -> typer.Typer | None
```

## get_typer_from_state

`typer.cli.get_typer_from_state`

```python
def get_typer_from_state() -> typer.Typer | None
```

## main

`typer.cli.main`

```python
def main() -> Any
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## maybe_add_run_to_cli

`typer.cli.maybe_add_run_to_cli`

```python
def maybe_add_run_to_cli(cli: TyperGroup) -> None
```

## maybe_update_state

`typer.cli.maybe_update_state`

```python
def maybe_update_state(ctx: _click.Context) -> None
```

## print_version

`typer.cli.print_version`

```python
def print_version(ctx: _click.Context, param: TyperOption, value: bool) -> None
```

