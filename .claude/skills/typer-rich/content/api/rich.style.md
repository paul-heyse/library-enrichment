# `rich.style`

Distribution: `rich`

## NULL_STYLE

`rich.style.NULL_STYLE`

```python
NULL_STYLE = Style()
```

**Inferred type** (`ty`, not declared in the source): `Style`

## StyleType

`rich.style.StyleType`

```python
StyleType = Union[str, 'Style']
```

**Inferred type** (`ty`, not declared in the source): ````xml <types.UnionType special-form 'str | Style'> ````

_16 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## _hash_getter

`rich.style._hash_getter`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_hash_getter = attrgetter('_color', '_bgcolor', '_attributes', '_set_attributes', '_link', '_meta')
```

## _id_generator

`rich.style._id_generator`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_id_generator = count(getrandbits(24))
```

## Style

`rich.style.Style`

```python
class Style
```

_21 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (36)**

- `STYLE_ATTRIBUTES = {'dim': 'dim', 'd': 'dim', 'bold': 'bold', 'b': 'bold', 'italic': 'italic', 'i': 'italic', 'underline': 'underline', 'u': 'underline', 'blink': 'blink', 'blink2': 'blink2', 'reverse': 'reverse', 'r': 'reverse', 'conceal': 'conceal', 'c': 'conceal', 'strike': 'strike', 's': 'strike', 'underline2': 'underline2', 'uu': 'underline2', 'frame': 'frame', 'encircle': 'encircle', 'overline': 'overline', 'o': 'overline'}`  _class-attribute, instance-attribute_
- `background_style: Style`  _property_
  A Style with background only.
- `bgcolor: Optional[Color]`  _property_
  The background color or None if it is not set.
- `blink = _Bit(4)`  _class-attribute, instance-attribute_
- `blink2 = _Bit(5)`  _class-attribute, instance-attribute_
- `bold = _Bit(0)`  _class-attribute, instance-attribute_
- `def chain(cls, styles: Style = ()) -> Style`  _classmethod_
  Combine styles from positional argument in to a single style.
- `def clear_meta_and_links(self) -> Style`  _cached_
  Get a copy of this style with link and meta information removed.
- `color: Optional[Color]`  _property_
  The foreground color or None if it is not set.
- `def combine(cls, styles: Iterable[Style]) -> Style`  _classmethod_
  Combine styles and get result.
- `conceal = _Bit(7)`  _class-attribute, instance-attribute_
- `dim = _Bit(1)`  _class-attribute, instance-attribute_
- `encircle = _Bit(11)`  _class-attribute, instance-attribute_
- `frame = _Bit(10)`  _class-attribute, instance-attribute_
- `def from_color(cls, color: Optional[Color] = None, bgcolor: Optional[Color] = None) -> Style`  _classmethod_
  Create a new style with colors and no attributes.
- `def from_meta(cls, meta: Optional[Dict[str, Any]]) -> Style`  _classmethod_
  Create a new style with meta data.
- `def get_html_style(self, theme: Optional[TerminalTheme] = None) -> str`  _cached_
  Get a CSS style rule.
- `italic = _Bit(2)`  _class-attribute, instance-attribute_
- `link: Optional[str]`  _property_
  Link text, if set.
- `link_id: str`  _property_
  Get a link id, used in ansi code for links.
- `meta: Dict[str, Any]`  _property_
  Get meta information (can not be changed after construction).
- `def normalize(cls, style: str) -> str`  _cached, classmethod_
  Normalize a style definition so that styles with the same effect have the same string representation.
- `def null(cls) -> Style`  _classmethod_
  Create an 'null' style, equivalent to Style(), but more performant.
- `def on(cls, meta: Optional[Dict[str, Any]] = None, handlers: Any = {}) -> Style`  _classmethod_
  Create a blank style with meta information.
- `overline = _Bit(12)`  _class-attribute, instance-attribute_
- `def parse(cls, style_definition: str) -> Style`  _cached, classmethod_
  Parse a style definition.
- `def pick_first(cls, values: Optional[StyleType] = ()) -> StyleType`  _classmethod_
  Pick first non-None style.
- `def render(self, text: str = '', color_system: Optional[ColorSystem] = ColorSystem.TRUECOLOR, legacy_windows: bool = False) -> str`
  Render the ANSI codes for the style.
- `reverse = _Bit(6)`  _class-attribute, instance-attribute_
- `strike = _Bit(8)`  _class-attribute, instance-attribute_
- `def test(self, text: Optional[str] = None) -> None`
  Write text with style directly to terminal.
- `transparent_background: bool`  _property_
  Check if the style specified a transparent background.
- `underline = _Bit(3)`  _class-attribute, instance-attribute_
- `underline2 = _Bit(9)`  _class-attribute, instance-attribute_
- `def update_link(self, link: Optional[str] = None) -> Style`
  Get a copy with a different value for link.
- `without_color: Style`  _property_
  Get a copy of the style with color removed.

A terminal style.

A terminal style consists of a color (`color`), a background color (`bgcolor`), and a number of attributes, such
as bold, italic etc. The attributes have 3 states: they can either be on
(``True``), off (``False``), or not set (``None``).

Args:
    color (Union[Color, str], optional): Color of terminal text. Defaults to None.
    bgcolor (Union[Color, str], optional): Color of terminal background. Defaults to None.
    bold (bool, optional): Enable bold text. Defaults to None.
    dim (bool, optional): Enable dim text. Defaults to None.
    italic (bool, optional): Enable italic text. Defaults to None.
    underline (bool, optional): Enable underlined text. Defaults to None.
    blink (bool, optional): Enabled blinking text. Defaults to None.
    blink2 (bool, optional): Enable fast blinking text. Defaults to None.
    reverse (bool, optional): Enabled reverse text. Defaults to None.
    conceal (bool, optional): Enable concealed text. Defaults to None.
    strike (bool, optional): Enable strikethrough text. Defaults to None.
    underline2 (bool, optional): Enable doubly underlined text. Defaults to None.
    frame (bool, optional): Enable framed text. Defaults to None.
    encircle (bool, optional): Enable encircled text. Defaults to None.
    overline (bool, optional): Enable overlined text. Defaults to None.
    link (str, link): Link URL. Defaults to None.


## StyleStack

`rich.style.StyleStack`

```python
class StyleStack
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (3)**

- `current: Style`  _property_
  Get the Style at the top of the stack.
- `def pop(self) -> Style`
  Pop last style and discard.
- `def push(self, style: Style) -> None`
  Push a new style on to the stack.

A stack of styles.


## _Bit

`rich.style._Bit`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _Bit
```

**Declared members (1)**

- `bit = 1 << bit_no`  _instance-attribute_

A descriptor to get/set a style attribute bit.


