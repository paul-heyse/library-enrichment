# `rich.syntax`

Distribution: `rich`

## ANSI_DARK

`rich.syntax.ANSI_DARK`

```python
ANSI_DARK: Dict[TokenType, Style] = {Token: Style(), Whitespace: Style(color='bright_black'), Comment: Style(dim=True), Comment.Preproc: Style(color='bright_cyan'), Keyword: Style(color='bright_blue'), Keyword.Type: Style(color='bright_cyan'), Operator.Word: Style(color='bright_magenta'), Name.Builtin: Style(color='bright_cyan'), Name.Function: Style(color='bright_green'), Name.Namespace: Style(color='bright_cyan', underline=True), Name.Class: Style(color='bright_green', underline=True), Name.Exception: Style(color='bright_cyan'), Name.Decorator: Style(color='bright_magenta', bold=True), Name.Variable: Style(color='bright_red'), Name.Constant: Style(color='bright_red'), Name.Attribute: Style(color='bright_cyan'), Name.Tag: Style(color='bright_blue'), String: Style(color='yellow'), Number: Style(color='bright_blue'), Generic.Deleted: Style(color='bright_red'), Generic.Inserted: Style(color='bright_green'), Generic.Heading: Style(bold=True), Generic.Subheading: Style(color='bright_magenta', bold=True), Generic.Prompt: Style(bold=True), Generic.Error: Style(color='bright_red'), Error: Style(color='red', underline=True)}
```

## ANSI_LIGHT

`rich.syntax.ANSI_LIGHT`

```python
ANSI_LIGHT: Dict[TokenType, Style] = {Token: Style(), Whitespace: Style(color='white'), Comment: Style(dim=True), Comment.Preproc: Style(color='cyan'), Keyword: Style(color='blue'), Keyword.Type: Style(color='cyan'), Operator.Word: Style(color='magenta'), Name.Builtin: Style(color='cyan'), Name.Function: Style(color='green'), Name.Namespace: Style(color='cyan', underline=True), Name.Class: Style(color='green', underline=True), Name.Exception: Style(color='cyan'), Name.Decorator: Style(color='magenta', bold=True), Name.Variable: Style(color='red'), Name.Constant: Style(color='red'), Name.Attribute: Style(color='cyan'), Name.Tag: Style(color='bright_blue'), String: Style(color='yellow'), Number: Style(color='blue'), Generic.Deleted: Style(color='bright_red'), Generic.Inserted: Style(color='green'), Generic.Heading: Style(bold=True), Generic.Subheading: Style(color='magenta', bold=True), Generic.Prompt: Style(bold=True), Generic.Error: Style(color='bright_red'), Error: Style(color='red', underline=True)}
```

## DEFAULT_THEME

`rich.syntax.DEFAULT_THEME`

```python
DEFAULT_THEME = 'monokai'
```

**Inferred type** (`ty`, not declared in the source): `Literal["monokai"]`

## NUMBERS_COLUMN_DEFAULT_PADDING

`rich.syntax.NUMBERS_COLUMN_DEFAULT_PADDING`

```python
NUMBERS_COLUMN_DEFAULT_PADDING = 2
```

**Inferred type** (`ty`, not declared in the source): `Literal[2]`

## RICH_SYNTAX_THEMES

`rich.syntax.RICH_SYNTAX_THEMES`

```python
RICH_SYNTAX_THEMES = {'ansi_light': ANSI_LIGHT, 'ansi_dark': ANSI_DARK}
```

**Inferred type** (`ty`, not declared in the source): `dict[str, dict[tuple[str, ...], Style]]`

## SyntaxPosition

`rich.syntax.SyntaxPosition`

```python
SyntaxPosition = Tuple[int, int]
```

**Inferred type** (`ty`, not declared in the source): ````xml <class 'tuple[int, int]'> ````

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## TokenType

`rich.syntax.TokenType`

```python
TokenType = Tuple[str, ...]
```

**Inferred type** (`ty`, not declared in the source): ````xml <class 'tuple[str, ...]'> ````

## WINDOWS

`rich.syntax.WINDOWS`

```python
WINDOWS = sys.platform == 'win32'
```

**Inferred type** (`ty`, not declared in the source): `Literal[False]`

## args

`rich.syntax.args`

```python
args = parser.parse_args()
```

**Inferred type** (`ty`, not declared in the source): `Namespace`

## code

`rich.syntax.code`

```python
code = sys.stdin.read()
```

**Inferred type** (`ty`, not declared in the source): `str | Any`

## console

`rich.syntax.console`

```python
console = Console(force_terminal=args.force_color, width=args.width)
```

**Inferred type** (`ty`, not declared in the source): `Console`

## parser

`rich.syntax.parser`

```python
parser = argparse.ArgumentParser(description='Render syntax to the console with Rich')
```

**Inferred type** (`ty`, not declared in the source): `ArgumentParser`

## syntax

`rich.syntax.syntax`

```python
syntax = Syntax(code=code, lexer=args.lexer_name, line_numbers=args.line_numbers, word_wrap=args.word_wrap, theme=args.theme, background_color=args.background_color, indent_guides=args.indent_guides, padding=args.padding, highlight_lines={args.highlight_line})
```

**Inferred type** (`ty`, not declared in the source): `Syntax`

## ANSISyntaxTheme

`rich.syntax.ANSISyntaxTheme`

```python
class ANSISyntaxTheme(SyntaxTheme)
```

**Bases** `SyntaxTheme`

**Declared members (3)**

- `def get_background_style(self) -> Style`
- `def get_style_for_token(self, token_type: TokenType) -> Style`
  Look up style in the style map.
- `style_map = style_map`  _instance-attribute_

Syntax theme to use standard colors.


## PaddingProperty

`rich.syntax.PaddingProperty`

```python
class PaddingProperty
```

Descriptor to get and set padding.


## PygmentsSyntaxTheme

`rich.syntax.PygmentsSyntaxTheme`

```python
class PygmentsSyntaxTheme(SyntaxTheme)
```

**Bases** `SyntaxTheme`

**Declared members (2)**

- `def get_background_style(self) -> Style`
- `def get_style_for_token(self, token_type: TokenType) -> Style`
  Get a style from a Pygments class.

Syntax theme that delegates to Pygments theme.


## Syntax

`rich.syntax.Syntax`

```python
class Syntax(JupyterMixin)
```

_7 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `JupyterMixin`

**Declared members (20)**

- `background_color = background_color`  _instance-attribute_
- `background_style = Style(bgcolor=background_color) if background_color else Style()`  _instance-attribute_
- `code = code`  _instance-attribute_
- `code_width = code_width`  _instance-attribute_
- `dedent = dedent`  _instance-attribute_
- `default_lexer: Lexer`  _property_
  A Pygments Lexer to use if one is not specified or invalid.
- `def from_path(cls, path: str, encoding: str = 'utf-8', lexer: Optional[Union[Lexer, str]] = None, theme: Union[str, SyntaxTheme] = DEFAULT_THEME, dedent: bool = False, line_numbers: bool = False, line_range: Optional[Tuple[int, int]] = None, start_line: int = 1, highlight_lines: Optional[Set[int]] = None, code_width: Optional[int] = None, tab_size: int = 4, word_wrap: bool = False, background_color: Optional[str] = None, indent_guides: bool = False, padding: PaddingDimensions = 0) -> 'Syntax'`  _classmethod_
  Construct a Syntax object from a file.
- `def get_theme(cls, name: Union[str, SyntaxTheme]) -> SyntaxTheme`  _classmethod_
  Get a syntax theme instance.
- `def guess_lexer(cls, path: str, code: Optional[str] = None) -> str`  _classmethod_
  Guess the alias of the Pygments lexer to use based on a path and an optional string of code. If code is supplied, it will use a combination of the code and the filename to determine the best lexer to use. For example, if the file is ``inde…
- `def highlight(self, code: str, line_range: Optional[Tuple[Optional[int], Optional[int]]] = None) -> Text`
  Highlight code and return a Text instance.
- `highlight_lines = highlight_lines or set()`  _instance-attribute_
- `indent_guides = indent_guides`  _instance-attribute_
- `lexer: Optional[Lexer]`  _property_
  The lexer for this syntax, or None if no lexer was found.
- `line_numbers = line_numbers`  _instance-attribute_
- `line_range = line_range`  _instance-attribute_
- `padding = PaddingProperty()`  _class-attribute, instance-attribute_
- `start_line = start_line`  _instance-attribute_
- `def stylize_range(self, style: StyleType, start: SyntaxPosition, end: SyntaxPosition, style_before: bool = False) -> None`
  Adds a custom style on a part of the code, that will be applied to the syntax display when it's rendered. Line numbers are 1-based, while column indexes are 0-based.
- `tab_size = tab_size`  _instance-attribute_
- `word_wrap = word_wrap`  _instance-attribute_

Construct a Syntax object to render syntax highlighted code.

Args:
    code (str): Code to highlight.
    lexer (Lexer | str): Lexer to use (see https://pygments.org/docs/lexers/)
    theme (str, optional): Color theme, aka Pygments style (see https://pygments.org/docs/styles/#getting-a-list-of-available-styles). Defaults to "monokai".
    dedent (bool, optional): Enable stripping of initial whitespace. Defaults to False.
    line_numbers (bool, optional): Enable rendering of line numbers. Defaults to False.
    start_line (int, optional): Starting number for line numbers. Defaults to 1.
    line_range (Tuple[int | None, int | None], optional): If given should be a tuple of the start and end line to render.
        A value of None in the tuple indicates the range is open in that direction.
    highlight_lines (Set[int]): A set of line numbers to highlight.
    code_width: Width of code to render (not including line numbers), or ``None`` to use all available width.
    tab_size (int, optional): Size of tabs. Defaults to 4.
    word_wrap (bool, optional): Enable word wrapping.
    background_color (str, optional): Optional background color, or None to use theme color. Defaults to None.
    indent_guides (bool, optional): Show indent guides. Defaults to False.
    padding (PaddingDimensions): Padding to apply around the syntax. Defaults to 0 (no padding).


## SyntaxTheme

`rich.syntax.SyntaxTheme`

```python
class SyntaxTheme(ABC)
```

**Bases** `ABC`

**Declared members (2)**

- `def get_background_style(self) -> Style`  _abstractmethod_
  Get the background color.
- `def get_style_for_token(self, token_type: TokenType) -> Style`  _abstractmethod_
  Get a style for a given Pygments token.

Base class for a syntax theme.


## _SyntaxHighlightRange

`rich.syntax._SyntaxHighlightRange`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _SyntaxHighlightRange(NamedTuple)
```

**Bases** `NamedTuple`

**Declared members (4)**

- `end: SyntaxPosition`  _instance-attribute_
- `start: SyntaxPosition`  _instance-attribute_
- `style: StyleType`  _instance-attribute_
- `style_before: bool = False`  _class-attribute, instance-attribute_

A range to highlight in a Syntax object.
`start` and `end` are 2-integers tuples, where the first integer is the line number
(starting from 1) and the second integer is the column index (starting from 0).


## _get_code_index_for_syntax_position

`rich.syntax._get_code_index_for_syntax_position`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _get_code_index_for_syntax_position(newlines_offsets: Sequence[int], position: SyntaxPosition) -> Optional[int]
```

Returns the index of the code string for the given positions.

Args:
    newlines_offsets (Sequence[int]): The offset of each newline character found in the code snippet.
    position (SyntaxPosition): The position to search for.

Returns:
    Optional[int]: The index of the code string for this position, or `None`
        if the given position's line number is out of range (if it's the column that is out of range
        we silently clamp its value so that it reaches the end of the line)


