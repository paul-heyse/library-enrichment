# `rich.markup`

Distribution: `rich`

## MARKUP

`rich.markup.MARKUP`

```python
MARKUP = ['[red]Hello World[/red]', '[magenta]Hello [b]World[/b]', '[bold]Bold[italic] bold and italic [/bold]italic[/italic]', 'Click [link=https://www.willmcgugan.com]here[/link] to visit my Blog', ':warning-emoji: [bold red blink] DANGER![/]']
```

**Inferred type** (`ty`, not declared in the source): `list[str]`

## RE_HANDLER

`rich.markup.RE_HANDLER`

```python
RE_HANDLER = re.compile('^([\\w.]*?)(\\(.*?\\))?$')
```

**Inferred type** (`ty`, not declared in the source): `Pattern[str]`

## RE_TAGS

`rich.markup.RE_TAGS`

```python
RE_TAGS = re.compile('((\\\\*)\\[([a-z#/@][^[]*?)])', re.VERBOSE)
```

**Inferred type** (`ty`, not declared in the source): `Pattern[str]`

## _EscapeSubMethod

`rich.markup._EscapeSubMethod`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_EscapeSubMethod = Callable[[_ReSubCallable, str], str]
```

## _ReStringMatch

`rich.markup._ReStringMatch`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_ReStringMatch = Match[str]
```

## _ReSubCallable

`rich.markup._ReSubCallable`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_ReSubCallable = Callable[[_ReStringMatch], str]
```

## grid

`rich.markup.grid`

```python
grid = Table('Markup', 'Result', padding=(0, 1))
```

**Inferred type** (`ty`, not declared in the source): `Table`

## Tag

`rich.markup.Tag`

```python
class Tag(NamedTuple)
```

**Bases** `NamedTuple`

**Declared members (3)**

- `markup: str`  _property_
  Get the string representation of this tag.
- `name: str`  _instance-attribute_
  The tag name. e.g. 'bold'.
- `parameters: Optional[str]`  _instance-attribute_
  Any additional parameters after the name.

A tag in console markup.


## _parse

`rich.markup._parse`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _parse(markup: str) -> Iterable[Tuple[int, Optional[str], Optional[Tag]]]
```

Parse markup in to an iterable of tuples of (position, text, tag).

Args:
    markup (str): A string containing console markup


## escape

`rich.markup.escape`

```python
def escape(markup: str, _escape: _EscapeSubMethod = re.compile('(\\\\*)(\\[[a-z#/@][^[]*?])').sub) -> str
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Escapes text so that it won't be interpreted as markup.

Args:
    markup (str): Content to be inserted in to markup.

Returns:
    str: Markup with square brackets escaped.


## render

`rich.markup.render`

```python
def render(markup: str, style: Union[str, Style] = '', emoji: bool = True, emoji_variant: Optional[EmojiVariant] = None) -> Text
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Render console markup in to a Text instance.

Args:
    markup (str): A string containing console markup.
    style: (Union[str, Style]): The style to use.
    emoji (bool, optional): Also render emoji code. Defaults to True.
    emoji_variant (str, optional): Optional emoji variant, either "text" or "emoji". Defaults to None.


Raises:
    MarkupError: If there is a syntax error in the markup.

Returns:
    Text: A test instance.


