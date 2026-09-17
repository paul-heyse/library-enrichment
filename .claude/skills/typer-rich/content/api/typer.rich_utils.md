# `typer.rich_utils`

Distribution: `typer`

## ABORTED_TEXT

`typer.rich_utils.ABORTED_TEXT`

```python
ABORTED_TEXT = _('Aborted.')
```

**Inferred type** (`ty`, not declared in the source): `str`

## ALIGN_COMMANDS_PANEL

`typer.rich_utils.ALIGN_COMMANDS_PANEL`

```python
ALIGN_COMMANDS_PANEL: Literal['left', 'center', 'right'] = 'left'
```

## ALIGN_ERRORS_PANEL

`typer.rich_utils.ALIGN_ERRORS_PANEL`

```python
ALIGN_ERRORS_PANEL: Literal['left', 'center', 'right'] = 'left'
```

## ALIGN_OPTIONS_PANEL

`typer.rich_utils.ALIGN_OPTIONS_PANEL`

```python
ALIGN_OPTIONS_PANEL: Literal['left', 'center', 'right'] = 'left'
```

## ANSI_PREFIX

`typer.rich_utils.ANSI_PREFIX`

```python
ANSI_PREFIX = '\x1b['
```

**Inferred type** (`ty`, not declared in the source): `Literal["\u{1b}["]`

## ARGUMENTS_PANEL_TITLE

`typer.rich_utils.ARGUMENTS_PANEL_TITLE`

```python
ARGUMENTS_PANEL_TITLE = _('Arguments')
```

**Inferred type** (`ty`, not declared in the source): `str`

## COLOR_SYSTEM

`typer.rich_utils.COLOR_SYSTEM`

```python
COLOR_SYSTEM: Literal['auto', 'standard', '256', 'truecolor', 'windows'] | None = 'auto'
```

## COMMANDS_PANEL_TITLE

`typer.rich_utils.COMMANDS_PANEL_TITLE`

```python
COMMANDS_PANEL_TITLE = _('Commands')
```

**Inferred type** (`ty`, not declared in the source): `str`

## DEFAULT_STRING

`typer.rich_utils.DEFAULT_STRING`

```python
DEFAULT_STRING = _('[default: {}]')
```

**Inferred type** (`ty`, not declared in the source): `str`

## DEPRECATED_STRING

`typer.rich_utils.DEPRECATED_STRING`

```python
DEPRECATED_STRING = _('(deprecated) ')
```

**Inferred type** (`ty`, not declared in the source): `str`

## ENVVAR_STRING

`typer.rich_utils.ENVVAR_STRING`

```python
ENVVAR_STRING = _('[env var: {}]')
```

**Inferred type** (`ty`, not declared in the source): `str`

## ERRORS_PANEL_TITLE

`typer.rich_utils.ERRORS_PANEL_TITLE`

```python
ERRORS_PANEL_TITLE = _('Error')
```

**Inferred type** (`ty`, not declared in the source): `str`

## FORCE_TERMINAL

`typer.rich_utils.FORCE_TERMINAL`

```python
FORCE_TERMINAL = True if (getenv('GITHUB_ACTIONS') or getenv('FORCE_COLOR') or getenv('PY_COLORS')) else None
```

**Inferred type** (`ty`, not declared in the source): `Literal[True] | None`

## MARKUP_MODE_MARKDOWN

`typer.rich_utils.MARKUP_MODE_MARKDOWN`

```python
MARKUP_MODE_MARKDOWN = 'markdown'
```

**Inferred type** (`ty`, not declared in the source): `Literal["markdown"]`

## MARKUP_MODE_RICH

`typer.rich_utils.MARKUP_MODE_RICH`

```python
MARKUP_MODE_RICH = 'rich'
```

**Inferred type** (`ty`, not declared in the source): `Literal["rich"]`

## MAX_WIDTH

`typer.rich_utils.MAX_WIDTH`

```python
MAX_WIDTH = int(_TERMINAL_WIDTH) if _TERMINAL_WIDTH else None
```

**Inferred type** (`ty`, not declared in the source): `int | None`

## MarkupModeStrict

`typer.rich_utils.MarkupModeStrict`

```python
MarkupModeStrict = Literal['markdown', 'rich']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["markdown", "rich"]'> ````

## OPTIONS_PANEL_TITLE

`typer.rich_utils.OPTIONS_PANEL_TITLE`

```python
OPTIONS_PANEL_TITLE = _('Options')
```

**Inferred type** (`ty`, not declared in the source): `str`

## RANGE_STRING

`typer.rich_utils.RANGE_STRING`

```python
RANGE_STRING = ' [{}]'
```

**Inferred type** (`ty`, not declared in the source): `Literal[" [{}]"]`

## REQUIRED_LONG_STRING

`typer.rich_utils.REQUIRED_LONG_STRING`

```python
REQUIRED_LONG_STRING = _('[required]')
```

**Inferred type** (`ty`, not declared in the source): `str`

## REQUIRED_SHORT_STRING

`typer.rich_utils.REQUIRED_SHORT_STRING`

```python
REQUIRED_SHORT_STRING = '*'
```

**Inferred type** (`ty`, not declared in the source): `Literal["*"]`

## RICH_HELP

`typer.rich_utils.RICH_HELP`

```python
RICH_HELP = _("Try [blue]'{command_path} {help_option}'[/] for help.")
```

**Inferred type** (`ty`, not declared in the source): `str`

## STYLE_ABORTED

`typer.rich_utils.STYLE_ABORTED`

```python
STYLE_ABORTED = 'red'
```

**Inferred type** (`ty`, not declared in the source): `Literal["red"]`

## STYLE_COMMANDS_PANEL_BORDER

`typer.rich_utils.STYLE_COMMANDS_PANEL_BORDER`

```python
STYLE_COMMANDS_PANEL_BORDER = 'dim'
```

**Inferred type** (`ty`, not declared in the source): `Literal["dim"]`

## STYLE_COMMANDS_TABLE_BORDER_STYLE

`typer.rich_utils.STYLE_COMMANDS_TABLE_BORDER_STYLE`

```python
STYLE_COMMANDS_TABLE_BORDER_STYLE = None
```

**Inferred type** (`ty`, not declared in the source): `None`

## STYLE_COMMANDS_TABLE_BOX

`typer.rich_utils.STYLE_COMMANDS_TABLE_BOX`

```python
STYLE_COMMANDS_TABLE_BOX = ''
```

**Inferred type** (`ty`, not declared in the source): `Literal[""]`

## STYLE_COMMANDS_TABLE_FIRST_COLUMN

`typer.rich_utils.STYLE_COMMANDS_TABLE_FIRST_COLUMN`

```python
STYLE_COMMANDS_TABLE_FIRST_COLUMN = 'bold cyan'
```

**Inferred type** (`ty`, not declared in the source): `Literal["bold cyan"]`

## STYLE_COMMANDS_TABLE_LEADING

`typer.rich_utils.STYLE_COMMANDS_TABLE_LEADING`

```python
STYLE_COMMANDS_TABLE_LEADING = 0
```

**Inferred type** (`ty`, not declared in the source): `Literal[0]`

## STYLE_COMMANDS_TABLE_PADDING

`typer.rich_utils.STYLE_COMMANDS_TABLE_PADDING`

```python
STYLE_COMMANDS_TABLE_PADDING = (0, 1)
```

**Inferred type** (`ty`, not declared in the source): `tuple[Literal[0], Literal[1]]`

## STYLE_COMMANDS_TABLE_PAD_EDGE

`typer.rich_utils.STYLE_COMMANDS_TABLE_PAD_EDGE`

```python
STYLE_COMMANDS_TABLE_PAD_EDGE = False
```

**Inferred type** (`ty`, not declared in the source): `Literal[False]`

## STYLE_COMMANDS_TABLE_ROW_STYLES

`typer.rich_utils.STYLE_COMMANDS_TABLE_ROW_STYLES`

```python
STYLE_COMMANDS_TABLE_ROW_STYLES = None
```

**Inferred type** (`ty`, not declared in the source): `None`

## STYLE_COMMANDS_TABLE_SHOW_LINES

`typer.rich_utils.STYLE_COMMANDS_TABLE_SHOW_LINES`

```python
STYLE_COMMANDS_TABLE_SHOW_LINES = False
```

**Inferred type** (`ty`, not declared in the source): `Literal[False]`

## STYLE_DEPRECATED

`typer.rich_utils.STYLE_DEPRECATED`

```python
STYLE_DEPRECATED = 'red'
```

**Inferred type** (`ty`, not declared in the source): `Literal["red"]`

## STYLE_DEPRECATED_COMMAND

`typer.rich_utils.STYLE_DEPRECATED_COMMAND`

```python
STYLE_DEPRECATED_COMMAND = 'dim'
```

**Inferred type** (`ty`, not declared in the source): `Literal["dim"]`

## STYLE_ERRORS_PANEL_BORDER

`typer.rich_utils.STYLE_ERRORS_PANEL_BORDER`

```python
STYLE_ERRORS_PANEL_BORDER = 'red'
```

**Inferred type** (`ty`, not declared in the source): `Literal["red"]`

## STYLE_ERRORS_SUGGESTION

`typer.rich_utils.STYLE_ERRORS_SUGGESTION`

```python
STYLE_ERRORS_SUGGESTION = 'dim'
```

**Inferred type** (`ty`, not declared in the source): `Literal["dim"]`

## STYLE_HELPTEXT

`typer.rich_utils.STYLE_HELPTEXT`

```python
STYLE_HELPTEXT = 'dim'
```

**Inferred type** (`ty`, not declared in the source): `Literal["dim"]`

## STYLE_HELPTEXT_FIRST_LINE

`typer.rich_utils.STYLE_HELPTEXT_FIRST_LINE`

```python
STYLE_HELPTEXT_FIRST_LINE = ''
```

**Inferred type** (`ty`, not declared in the source): `Literal[""]`

## STYLE_NEGATIVE_OPTION

`typer.rich_utils.STYLE_NEGATIVE_OPTION`

```python
STYLE_NEGATIVE_OPTION = 'bold magenta'
```

**Inferred type** (`ty`, not declared in the source): `Literal["bold magenta"]`

## STYLE_NEGATIVE_SWITCH

`typer.rich_utils.STYLE_NEGATIVE_SWITCH`

```python
STYLE_NEGATIVE_SWITCH = 'bold red'
```

**Inferred type** (`ty`, not declared in the source): `Literal["bold red"]`

## STYLE_OPTION

`typer.rich_utils.STYLE_OPTION`

```python
STYLE_OPTION = 'bold cyan'
```

**Inferred type** (`ty`, not declared in the source): `Literal["bold cyan"]`

## STYLE_OPTIONS_PANEL_BORDER

`typer.rich_utils.STYLE_OPTIONS_PANEL_BORDER`

```python
STYLE_OPTIONS_PANEL_BORDER = 'dim'
```

**Inferred type** (`ty`, not declared in the source): `Literal["dim"]`

## STYLE_OPTIONS_TABLE_BORDER_STYLE

`typer.rich_utils.STYLE_OPTIONS_TABLE_BORDER_STYLE`

```python
STYLE_OPTIONS_TABLE_BORDER_STYLE = None
```

**Inferred type** (`ty`, not declared in the source): `None`

## STYLE_OPTIONS_TABLE_BOX

`typer.rich_utils.STYLE_OPTIONS_TABLE_BOX`

```python
STYLE_OPTIONS_TABLE_BOX = ''
```

**Inferred type** (`ty`, not declared in the source): `Literal[""]`

## STYLE_OPTIONS_TABLE_LEADING

`typer.rich_utils.STYLE_OPTIONS_TABLE_LEADING`

```python
STYLE_OPTIONS_TABLE_LEADING = 0
```

**Inferred type** (`ty`, not declared in the source): `Literal[0]`

## STYLE_OPTIONS_TABLE_PADDING

`typer.rich_utils.STYLE_OPTIONS_TABLE_PADDING`

```python
STYLE_OPTIONS_TABLE_PADDING = (0, 1)
```

**Inferred type** (`ty`, not declared in the source): `tuple[Literal[0], Literal[1]]`

## STYLE_OPTIONS_TABLE_PAD_EDGE

`typer.rich_utils.STYLE_OPTIONS_TABLE_PAD_EDGE`

```python
STYLE_OPTIONS_TABLE_PAD_EDGE = False
```

**Inferred type** (`ty`, not declared in the source): `Literal[False]`

## STYLE_OPTIONS_TABLE_ROW_STYLES

`typer.rich_utils.STYLE_OPTIONS_TABLE_ROW_STYLES`

```python
STYLE_OPTIONS_TABLE_ROW_STYLES = None
```

**Inferred type** (`ty`, not declared in the source): `None`

## STYLE_OPTIONS_TABLE_SHOW_LINES

`typer.rich_utils.STYLE_OPTIONS_TABLE_SHOW_LINES`

```python
STYLE_OPTIONS_TABLE_SHOW_LINES = False
```

**Inferred type** (`ty`, not declared in the source): `Literal[False]`

## STYLE_OPTION_DEFAULT

`typer.rich_utils.STYLE_OPTION_DEFAULT`

```python
STYLE_OPTION_DEFAULT = 'dim'
```

**Inferred type** (`ty`, not declared in the source): `Literal["dim"]`

## STYLE_OPTION_ENVVAR

`typer.rich_utils.STYLE_OPTION_ENVVAR`

```python
STYLE_OPTION_ENVVAR = 'dim yellow'
```

**Inferred type** (`ty`, not declared in the source): `Literal["dim yellow"]`

## STYLE_OPTION_HELP

`typer.rich_utils.STYLE_OPTION_HELP`

```python
STYLE_OPTION_HELP = ''
```

**Inferred type** (`ty`, not declared in the source): `Literal[""]`

## STYLE_REQUIRED_LONG

`typer.rich_utils.STYLE_REQUIRED_LONG`

```python
STYLE_REQUIRED_LONG = 'dim red'
```

**Inferred type** (`ty`, not declared in the source): `Literal["dim red"]`

## STYLE_REQUIRED_SHORT

`typer.rich_utils.STYLE_REQUIRED_SHORT`

```python
STYLE_REQUIRED_SHORT = 'red'
```

**Inferred type** (`ty`, not declared in the source): `Literal["red"]`

## STYLE_SWITCH

`typer.rich_utils.STYLE_SWITCH`

```python
STYLE_SWITCH = 'bold green'
```

**Inferred type** (`ty`, not declared in the source): `Literal["bold green"]`

## STYLE_TYPES

`typer.rich_utils.STYLE_TYPES`

```python
STYLE_TYPES = 'bold yellow'
```

**Inferred type** (`ty`, not declared in the source): `Literal["bold yellow"]`

## STYLE_TYPES_SEPARATOR

`typer.rich_utils.STYLE_TYPES_SEPARATOR`

```python
STYLE_TYPES_SEPARATOR = 'dim'
```

**Inferred type** (`ty`, not declared in the source): `Literal["dim"]`

## STYLE_USAGE

`typer.rich_utils.STYLE_USAGE`

```python
STYLE_USAGE = 'yellow'
```

**Inferred type** (`ty`, not declared in the source): `Literal["yellow"]`

## STYLE_USAGE_COMMAND

`typer.rich_utils.STYLE_USAGE_COMMAND`

```python
STYLE_USAGE_COMMAND = 'bold'
```

**Inferred type** (`ty`, not declared in the source): `Literal["bold"]`

## _RICH_HELP_PANEL_NAME

`typer.rich_utils._RICH_HELP_PANEL_NAME`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_RICH_HELP_PANEL_NAME = 'rich_help_panel'
```

## _TERMINAL_WIDTH

`typer.rich_utils._TERMINAL_WIDTH`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_TERMINAL_WIDTH = getenv('TERMINAL_WIDTH')
```

## _TYPER_FORCE_DISABLE_TERMINAL

`typer.rich_utils._TYPER_FORCE_DISABLE_TERMINAL`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_TYPER_FORCE_DISABLE_TERMINAL = getenv('_TYPER_FORCE_DISABLE_TERMINAL')
```

## highlighter

`typer.rich_utils.highlighter`

```python
highlighter = OptionHighlighter()
```

**Inferred type** (`ty`, not declared in the source): `OptionHighlighter`

## negative_highlighter

`typer.rich_utils.negative_highlighter`

```python
negative_highlighter = NegativeOptionHighlighter()
```

**Inferred type** (`ty`, not declared in the source): `NegativeOptionHighlighter`

## types_highlighter

`typer.rich_utils.types_highlighter`

```python
types_highlighter = TypesHighlighter()
```

**Inferred type** (`ty`, not declared in the source): `TypesHighlighter`

## NegativeOptionHighlighter

`typer.rich_utils.NegativeOptionHighlighter`

```python
class NegativeOptionHighlighter(RegexHighlighter)
```

**Bases** `RegexHighlighter`

**Declared members (1)**

- `highlights = ['(^|\\W)(?P<negative_switch>\\-\\w+)(?![a-zA-Z0-9])', '(^|\\W)(?P<negative_option>\\-\\-[\\w\\-]+)(?![a-zA-Z0-9])']`  _class-attribute, instance-attribute_

## OptionHighlighter

`typer.rich_utils.OptionHighlighter`

```python
class OptionHighlighter(RegexHighlighter)
```

**Bases** `RegexHighlighter`

**Declared members (1)**

- `highlights = ['(^|\\W)(?P<switch>\\-\\w+)(?![a-zA-Z0-9])', '(^|\\W)(?P<option>\\-\\-[\\w\\-]+)(?![a-zA-Z0-9])', '(?P<types>\\<[^\\>]+\\>)', '(?P<usage>Usage: )']`  _class-attribute, instance-attribute_

Highlights our special options.


## TypesHighlighter

`typer.rich_utils.TypesHighlighter`

```python
class TypesHighlighter(RegexHighlighter)
```

**Bases** `RegexHighlighter`

**Declared members (1)**

- `highlights = ['^(?P<types_sep>(\\[|<))', '(?P<types_sep>\\|)', '(?P<types_sep>(\\]|>))(\\.\\.\\.)?$']`  _class-attribute, instance-attribute_

## _fix_linebreaks

`typer.rich_utils._fix_linebreaks`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _fix_linebreaks(paragraphs: list[str], markup_mode: MarkupModeStrict) -> str
```

## _get_help_text

`typer.rich_utils._get_help_text`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _get_help_text(obj: _click.Command | TyperGroup, markup_mode: MarkupModeStrict) -> Iterable[Markdown | Text]
```

Build primary help text for a click command or group.

Returns the prose help text for a command or group, rendered either as a
Rich Text object or as Markdown.
If the command is marked as deprecated, the deprecated string will be prepended.


## _get_parameter_help

`typer.rich_utils._get_parameter_help`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _get_parameter_help(param: TyperOption | TyperArgument | _click.Parameter, ctx: _click.Context, markup_mode: MarkupModeStrict) -> Columns
```

Build primary help text for a click option or argument.

Returns the prose help text for an option or argument, rendered either
as a Rich Text object or as Markdown.
Additional elements are appended to show the default and required status if
applicable.


## _get_rich_console

`typer.rich_utils._get_rich_console`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _get_rich_console(stderr: bool = False) -> Console
```

## _has_ansi_character

`typer.rich_utils._has_ansi_character`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _has_ansi_character(text: str) -> bool
```

## _make_command_help

`typer.rich_utils._make_command_help`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _make_command_help(help_text: str, markup_mode: MarkupModeStrict) -> Text | Markdown
```

Build cli help text for a click group command.

That is, when calling help on groups with multiple subcommands
(not the main help text when calling the subcommand help).

Returns the first paragraph of help text for a command, rendered either as a
Rich Text object or as Markdown.
Ignores single newlines as paragraph markers, looks for double only.


## _make_rich_text

`typer.rich_utils._make_rich_text`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _make_rich_text(text: str, style: str = '', markup_mode: MarkupModeStrict) -> Markdown | Text
```

Take a string, remove indentations, and return styled text.

If `markup_mode` is `"rich"`, the text is parsed for Rich markup strings.
If `markup_mode` is `"markdown"`, parse as Markdown.


## _print_commands_panel

`typer.rich_utils._print_commands_panel`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _print_commands_panel(name: str, commands: list[_click.Command], markup_mode: MarkupModeStrict, console: Console, cmd_len: int) -> None
```

## _print_options_panel

`typer.rich_utils._print_options_panel`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _print_options_panel(name: str, params: list[TyperOption] | list[TyperArgument], ctx: _click.Context, markup_mode: MarkupModeStrict, console: Console) -> None
```

## escape_before_html_export

`typer.rich_utils.escape_before_html_export`

```python
def escape_before_html_export(input_text: str) -> str
```

Ensure that the input string can be used for HTML export.


## get_traceback

`typer.rich_utils.get_traceback`

```python
def get_traceback(exc: BaseException, exception_config: DeveloperExceptionConfig, internal_dir_names: list[str]) -> Traceback
```

## rich_abort_error

`typer.rich_utils.rich_abort_error`

```python
def rich_abort_error() -> None
```

Print richly formatted abort error.


## rich_format_error

`typer.rich_utils.rich_format_error`

```python
def rich_format_error(self: _click.ClickException) -> None
```

Print richly formatted click errors.

Called by custom exception handler to print richly formatted click errors.
Mimics original _click.ClickException.echo() function but with rich formatting.


## rich_format_help

`typer.rich_utils.rich_format_help`

```python
def rich_format_help(obj: _click.Command | TyperGroup, ctx: _click.Context, markup_mode: MarkupModeStrict) -> None
```

Print nicely formatted help text using rich.

Based on original code from rich-cli, by @willmcgugan.
https://github.com/Textualize/rich-cli/blob/8a2767c7a340715fc6fbf4930ace717b9b2fc5e5/src/rich_cli/__main__.py#L162-L236

Replacement for the click function format_help().
Takes a command or group and builds the help text output.


## rich_render_text

`typer.rich_utils.rich_render_text`

```python
def rich_render_text(text: str) -> str
```

Remove rich tags and render a pure text representation


## rich_to_html

`typer.rich_utils.rich_to_html`

```python
def rich_to_html(input_text: str) -> str
```

Print the HTML version of a rich-formatted input string.

This function does not provide a full HTML page, but can be used to insert
HTML-formatted text spans into a markdown file.


