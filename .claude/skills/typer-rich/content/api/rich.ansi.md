# `rich.ansi`

Distribution: `rich`

## SGR_STYLE_MAP

`rich.ansi.SGR_STYLE_MAP`

```python
SGR_STYLE_MAP = {1: 'bold', 2: 'dim', 3: 'italic', 4: 'underline', 5: 'blink', 6: 'blink2', 7: 'reverse', 8: 'conceal', 9: 'strike', 21: 'underline2', 22: 'not dim not bold', 23: 'not italic', 24: 'not underline', 25: 'not blink', 26: 'not blink2', 27: 'not reverse', 28: 'not conceal', 29: 'not strike', 30: 'color(0)', 31: 'color(1)', 32: 'color(2)', 33: 'color(3)', 34: 'color(4)', 35: 'color(5)', 36: 'color(6)', 37: 'color(7)', 39: 'default', 40: 'on color(0)', 41: 'on color(1)', 42: 'on color(2)', 43: 'on color(3)', 44: 'on color(4)', 45: 'on color(5)', 46: 'on color(6)', 47: 'on color(7)', 49: 'on default', 51: 'frame', 52: 'encircle', 53: 'overline', 54: 'not frame not encircle', 55: 'not overline', 90: 'color(8)', 91: 'color(9)', 92: 'color(10)', 93: 'color(11)', 94: 'color(12)', 95: 'color(13)', 96: 'color(14)', 97: 'color(15)', 100: 'on color(8)', 101: 'on color(9)', 102: 'on color(10)', 103: 'on color(11)', 104: 'on color(12)', 105: 'on color(13)', 106: 'on color(14)', 107: 'on color(15)'}
```

**Inferred type** (`ty`, not declared in the source): `dict[int, str]`

## console

`rich.ansi.console`

```python
console = Console(record=True)
```

**Inferred type** (`ty`, not declared in the source): `Console`

## decoder

`rich.ansi.decoder`

```python
decoder = AnsiDecoder()
```

**Inferred type** (`ty`, not declared in the source): `AnsiDecoder`

## re_ansi

`rich.ansi.re_ansi`

```python
re_ansi = re.compile('\n(?:\\x1b[0-?])|\n(?:\\x1b\\](.*?)\\x1b\\\\)|\n(?:\\x1b([(@-Z\\\\-_]|\\[[0-?]*[ -/]*[@-~]))\n', re.VERBOSE)
```

**Inferred type** (`ty`, not declared in the source): `Pattern[str]`

## stdout

`rich.ansi.stdout`

```python
stdout = io.BytesIO()
```

**Inferred type** (`ty`, not declared in the source): `BytesIO`

## stdout_result

`rich.ansi.stdout_result`

```python
stdout_result = stdout.getvalue().decode('utf-8')
```

**Inferred type** (`ty`, not declared in the source): `str`

## AnsiDecoder

`rich.ansi.AnsiDecoder`

```python
class AnsiDecoder
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (3)**

- `def decode(self, terminal_text: str) -> Iterable[Text]`
  Decode ANSI codes in an iterable of lines.
- `def decode_line(self, line: str) -> Text`
  Decode a line containing ansi codes.
- `style = Style.null()`  _instance-attribute_

Translate ANSI code in to styled Text.


## _AnsiToken

`rich.ansi._AnsiToken`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _AnsiToken(NamedTuple)
```

**Bases** `NamedTuple`

**Declared members (3)**

- `osc: Optional[str] = ''`  _class-attribute, instance-attribute_
- `plain: str = ''`  _class-attribute, instance-attribute_
- `sgr: Optional[str] = ''`  _class-attribute, instance-attribute_

Result of ansi tokenized string.


## _ansi_tokenize

`rich.ansi._ansi_tokenize`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _ansi_tokenize(ansi_text: str) -> Iterable[_AnsiToken]
```

Tokenize a string in to plain text and ANSI codes.

Args:
    ansi_text (str): A String containing ANSI codes.

Yields:
    AnsiToken: A named tuple of (plain, sgr, osc)


## read

`rich.ansi.read`

```python
def read(fd: int) -> bytes
```

