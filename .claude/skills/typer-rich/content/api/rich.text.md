# `rich.text`

Distribution: `rich`

## DEFAULT_JUSTIFY

`rich.text.DEFAULT_JUSTIFY`

```python
DEFAULT_JUSTIFY: JustifyMethod = 'default'
```

## DEFAULT_OVERFLOW

`rich.text.DEFAULT_OVERFLOW`

```python
DEFAULT_OVERFLOW: OverflowMethod = 'fold'
```

## GetStyleCallable

`rich.text.GetStyleCallable`

```python
GetStyleCallable = Callable[[str], Optional[StyleType]]
```

**Inferred type** (`ty`, not declared in the source): ````xml <Callable special-form '(str, /) -> str | Style | None'> ````

## TextType

`rich.text.TextType`

```python
TextType = Union[str, 'Text']
```

**Inferred type** (`ty`, not declared in the source): ````xml <types.UnionType special-form 'str | Text'> ``` --- A plain string or a :class:`Text` instance.`

_10 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

A plain string or a :class:`Text` instance.


## _re_whitespace

`rich.text._re_whitespace`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_re_whitespace = re.compile('\\s+$')
```

## console

`rich.text.console`

```python
console = Console()
```

**Inferred type** (`ty`, not declared in the source): `Console`

## text

`rich.text.text`

```python
text = Text('\nLorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.\n')
```

**Inferred type** (`ty`, not declared in the source): `Text`

## Span

`rich.text.Span`

```python
class Span(NamedTuple)
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `NamedTuple`

**Declared members (7)**

- `end: int`  _instance-attribute_
  Span end index.
- `def extend(self, cells: int) -> Span`
  Extend the span by the given number of cells.
- `def move(self, offset: int) -> Span`
  Move start and end by a given offset.
- `def right_crop(self, offset: int) -> Span`
  Crop the span at the given offset.
- `def split(self, offset: int) -> Tuple[Span, Optional[Span]]`
  Split a span in to 2 from a given offset.
- `start: int`  _instance-attribute_
  Span start index.
- `style: Union[str, Style]`  _instance-attribute_
  Style associated with the span.

A marked up region in some text.


## Text

`rich.text.Text`

```python
class Text(JupyterMixin)
```

_30 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `JupyterMixin`

**Declared members (46)**

- `def align(self, align: AlignMethod, width: int, character: str = ' ') -> None`
  Align text to a given width.
- `def append(self, text: Union[Text, str], style: Optional[Union[str, Style]] = None) -> Text`
  Add text with an optional style.
- `def append_text(self, text: Text) -> Text`
  Append another Text instance. This method is more performant than Text.append, but only works for Text.
- `def append_tokens(self, tokens: Iterable[Tuple[str, Optional[StyleType]]]) -> Text`
  Append iterable of str and style. Style may be a Style instance or a str style definition.
- `def apply_meta(self, meta: Dict[str, Any], start: int = 0, end: Optional[int] = None) -> None`
  Apply metadata to the text, or a portion of the text.
- `def assemble(cls, parts: Union[str, Text, Tuple[str, StyleType]] = (), style: Union[str, Style] = '', justify: Optional[JustifyMethod] = None, overflow: Optional[OverflowMethod] = None, no_wrap: Optional[bool] = None, end: str = '\n', tab_size: int = 8, meta: Optional[Dict[str, Any]] = None) -> Text`  _classmethod_
  Construct a text instance by combining a sequence of strings with optional styles. The positional arguments should be either strings, or a tuple of string + style.
- `def blank_copy(self, plain: str = '') -> Text`
  Return a new Text instance with copied metadata (but not the string or spans).
- `cell_len: int`  _property_
  Get the number of cells required to render this text.
- `def copy_styles(self, text: Text) -> None`
  Copy styles from another Text instance.
- `def detect_indentation(self) -> int`
  Auto-detect indentation of code.
- `def divide(self, offsets: Iterable[int]) -> Lines`
  Divide text into a number of lines at given offsets.
- `end = end`  _instance-attribute_
- `def expand_tabs(self, tab_size: Optional[int] = None) -> None`
  Converts tabs to spaces.
- `def extend_style(self, spaces: int) -> None`
  Extend the Text given number of spaces where the spaces have the same style as the last character.
- `def fit(self, width: int) -> Lines`
  Fit the text in to given width by chopping in to lines.
- `def from_ansi(cls, text: str, style: Union[str, Style] = '', justify: Optional[JustifyMethod] = None, overflow: Optional[OverflowMethod] = None, no_wrap: Optional[bool] = None, end: str = '\n', tab_size: Optional[int] = 8) -> Text`  _classmethod_
  Create a Text object from a string containing ANSI escape codes.
- `def from_markup(cls, text: str, style: Union[str, Style] = '', emoji: bool = True, emoji_variant: Optional[EmojiVariant] = None, justify: Optional[JustifyMethod] = None, overflow: Optional[OverflowMethod] = None, end: str = '\n') -> Text`  _classmethod_
  Create Text instance from markup.
- `def get_style_at_offset(self, console: Console, offset: int) -> Style`
  Get the style of a character at give offset.
- `def highlight_regex(self, re_highlight: Union[Pattern[str], str], style: Optional[Union[GetStyleCallable, StyleType]] = None, style_prefix: str = '') -> int`
  Highlight text with a regular expression, where group names are translated to styles.
- `def highlight_words(self, words: Iterable[str], style: Union[str, Style], case_sensitive: bool = True) -> int`
  Highlight words with a style.
- `def join(self, lines: Iterable[Text]) -> Text`
  Join text together with this instance as the separator.
- `justify: Optional[JustifyMethod] = justify`  _instance-attribute_
- `markup: str`  _property_
  Get console markup to render this Text.
- `no_wrap = no_wrap`  _instance-attribute_
- `def on(self, meta: Optional[Dict[str, Any]] = None, handlers: Any = {}) -> Text`
  Apply event handlers (used by Textual project).
- `overflow: Optional[OverflowMethod] = overflow`  _instance-attribute_
- `def pad(self, count: int, character: str = ' ') -> None`
  Pad left and right with a given number of characters.
- `def pad_left(self, count: int, character: str = ' ') -> None`
  Pad the left with a given character.
- `def pad_right(self, count: int, character: str = ' ') -> None`
  Pad the right with a given character.
- `plain: str`  _property, writable_
  Get the text as a single string.
- `def remove_suffix(self, suffix: str) -> None`
  Remove a suffix if it exists.
- `def render(self, console: Console, end: str = '') -> Iterable[Segment]`
  Render the text as Segments.
- `def right_crop(self, amount: int = 1) -> None`
  Remove a number of characters from the end of the text.
- `def rstrip(self) -> None`
  Strip whitespace from end of text.
- `def rstrip_end(self, size: int) -> None`
  Remove whitespace beyond a certain width at the end of the text.
- `def set_length(self, new_length: int) -> None`
  Set new length of the text, clipping or padding is required.
- `spans: List[Span]`  _property, writable_
  Get a reference to the internal list of spans.
- `def split(self, separator: str = '\n', include_separator: bool = False, allow_blank: bool = False) -> Lines`
  Split rich text in to lines, preserving styles.
- `style = style`  _instance-attribute_
- `def styled(cls, text: str, style: StyleType = '', justify: Optional[JustifyMethod] = None, overflow: Optional[OverflowMethod] = None) -> Text`  _classmethod_
  Construct a Text instance with a pre-applied styled. A style applied in this way won't be used to pad the text when it is justified.
- `def stylize(self, style: Union[str, Style], start: int = 0, end: Optional[int] = None) -> None`
  Apply a style to the text, or a portion of the text.
- `def stylize_before(self, style: Union[str, Style], start: int = 0, end: Optional[int] = None) -> None`
  Apply a style to the text, or a portion of the text. Styles will be applied before other styles already present.
- `tab_size = tab_size`  _instance-attribute_
- `def truncate(self, max_width: int, overflow: Optional[OverflowMethod] = None, pad: bool = False) -> None`
  Truncate text if it is longer that a given width.
- `def with_indent_guides(self, indent_size: Optional[int] = None, character: str = '│', style: StyleType = 'dim green') -> Text`
  Adds indent guide lines to text.
- `def wrap(self, console: Console, width: int, justify: Optional[JustifyMethod] = None, overflow: Optional[OverflowMethod] = None, tab_size: int = 8, no_wrap: Optional[bool] = None) -> Lines`
  Word wrap the text.

Text with color / style.

Args:
    text (str, optional): Default unstyled text. Defaults to "".
    style (Union[str, Style], optional): Base style for text. Defaults to "".
    justify (str, optional): Justify method: "left", "center", "full", "right". Defaults to None.
    overflow (str, optional): Overflow method: "crop", "fold", "ellipsis". Defaults to None.
    no_wrap (bool, optional): Disable text wrapping, or None for default. Defaults to None.
    end (str, optional): Character to end text with. Defaults to "\\n".
    tab_size (int): Number of spaces per tab, or ``None`` to use ``console.tab_size``. Defaults to None.
    spans (List[Span], optional). A list of predefined style spans. Defaults to None.


