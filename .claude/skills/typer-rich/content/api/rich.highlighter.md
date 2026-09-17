# `rich.highlighter`

Distribution: `rich`

## console

`rich.highlighter.console`

```python
console = Console()
```

**Inferred type** (`ty`, not declared in the source): `Console`

## Highlighter

`rich.highlighter.Highlighter`

```python
class Highlighter(ABC)
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `ABC`

**Declared members (1)**

- `def highlight(self, text: Text) -> None`  _abstractmethod_
  Apply highlighting in place to text.

Abstract base class for highlighters.


## ISO8601Highlighter

`rich.highlighter.ISO8601Highlighter`

```python
class ISO8601Highlighter(RegexHighlighter)
```

**Bases** `RegexHighlighter`

**Declared members (2)**

- `base_style: str = 'iso8601.'`  _class-attribute_
- `highlights: Sequence[str] = ['^(?P<year>[0-9]{4})-(?P<month>1[0-2]|0[1-9])$', '^(?P<date>(?P<year>[0-9]{4})(?P<month>1[0-2]|0[1-9])(?P<day>3[01]|0[1-9]|[12][0-9]))$', '^(?P<date>(?P<year>[0-9]{4})-?(?P<day>36[0-6]|3[0-5][0-9]|[12][0-9]{2}|0[1-9][0-9]|00[1-9]))$', '^(?P<date>(?P<year>[0-9]{4})-?W(?P<week>5[0-3]|[1-4][0-9]|0[1-9]))$', '^(?P<date>(?P<year>[0-9]{4})-?W(?P<week>5[0-3]|[1-4][0-9]|0[1-9])-?(?P<day>[1-7]))$', '^(?P<time>(?P<hour>2[0-3]|[01][0-9]):?(?P<minute>[0-5][0-9]))$', '^(?P<time>(?P<hour>2[0-3]|[01][0-9])(?P<minute>[0-5][0-9])(?P<second>[0-5][0-9]))$', '^(?P<timezone>(Z|[+-](?:2[0-3]|[01][0-9])(?::?(?:[0-5][0-9]))?))$', '^(?P<time>(?P<hour>2[0-3]|[01][0-9])(?P<minute>[0-5][0-9])(?P<second>[0-5][0-9]))(?P<timezone>Z|[+-](?:2[0-3]|[01][0-9])(?::?(?:[0-5][0-9]))?)$', '^(?P<date>(?P<year>[0-9]{4})(?P<hyphen>-)?(?P<month>1[0-2]|0[1-9])(?(hyphen)-)(?P<day>3[01]|0[1-9]|[12][0-9])) (?P<time>(?P<hour>2[0-3]|[01][0-9])(?(hyphen):)(?P<minute>[0-5][0-9])(?(hyphen):)(?P<second>[0-5][0-9]))$', '^(?P<date>(?P<year>-?(?:[1-9][0-9]*)?[0-9]{4})-(?P<month>1[0-2]|0[1-9])-(?P<day>3[01]|0[1-9]|[12][0-9]))(?P<timezone>Z|[+-](?:2[0-3]|[01][0-9]):[0-5][0-9])?$', '^(?P<time>(?P<hour>2[0-3]|[01][0-9]):(?P<minute>[0-5][0-9]):(?P<second>[0-5][0-9])(?P<frac>\\.[0-9]+)?)(?P<timezone>Z|[+-](?:2[0-3]|[01][0-9]):[0-5][0-9])?$', '^(?P<date>(?P<year>-?(?:[1-9][0-9]*)?[0-9]{4})-(?P<month>1[0-2]|0[1-9])-(?P<day>3[01]|0[1-9]|[12][0-9]))T(?P<time>(?P<hour>2[0-3]|[01][0-9]):(?P<minute>[0-5][0-9]):(?P<second>[0-5][0-9])(?P<ms>\\.[0-9]+)?)(?P<timezone>Z|[+-](?:2[0-3]|[01][0-9]):[0-5][0-9])?$']`  _class-attribute_

**Inherited (1)**

- from `rich.highlighter.RegexHighlighter`: `highlight`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Highlights the ISO8601 date time strings.
Regex reference: https://www.oreilly.com/library/view/regular-expressions-cookbook/9781449327453/ch04s07.html


## JSONHighlighter

`rich.highlighter.JSONHighlighter`

```python
class JSONHighlighter(RegexHighlighter)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `RegexHighlighter`

**Declared members (5)**

- `JSON_STR = '(?<![\\\\\\w])(?P<str>b?\\".*?(?<!\\\\)\\")'`  _class-attribute, instance-attribute_
- `JSON_WHITESPACE = {' ', '\n', '\r', '\t'}`  _class-attribute, instance-attribute_
- `base_style: str = 'json.'`  _class-attribute_
- `def highlight(self, text: Text) -> None`
- `highlights: Sequence[str] = [_combine_regex('(?P<brace>[\\{\\[\\(\\)\\]\\}])', '\\b(?P<bool_true>true)\\b|\\b(?P<bool_false>false)\\b|\\b(?P<null>null)\\b', '(?P<number>(?<!\\w)\\-?[0-9]+\\.?[0-9]*(e[\\-\\+]?\\d+?)?\\b|0x[0-9a-fA-F]*)', JSON_STR)]`  _class-attribute_

Highlights JSON


## NullHighlighter

`rich.highlighter.NullHighlighter`

```python
class NullHighlighter(Highlighter)
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Highlighter`

**Declared members (1)**

- `def highlight(self, text: Text) -> None`
  Nothing to do

A highlighter object that doesn't highlight.

May be used to disable highlighting entirely.


## RegexHighlighter

`rich.highlighter.RegexHighlighter`

```python
class RegexHighlighter(Highlighter)
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Highlighter`

**Declared members (3)**

- `base_style: str = ''`  _class-attribute_
- `def highlight(self, text: Text) -> None`
  Highlight :class:`rich.text.Text` using regular expressions.
- `highlights: Sequence[str] = []`  _class-attribute_

Applies highlighting from a list of regular expressions.


## ReprHighlighter

`rich.highlighter.ReprHighlighter`

```python
class ReprHighlighter(RegexHighlighter)
```

_9 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `RegexHighlighter`

**Declared members (2)**

- `base_style = 'repr.'`  _class-attribute, instance-attribute_
- `highlights: Sequence[str] = ['(?P<tag_start><)(?P<tag_name>[-\\w.:|]*)(?P<tag_contents>[\\w\\W]*)(?P<tag_end>>)', '(?P<attrib_name>[\\w_]{1,50})=(?P<attrib_value>"?[\\w_]+"?)?', '(?P<brace>[][{}()])', _combine_regex('(?P<ipv4>[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3})', '(?P<ipv6>([A-Fa-f0-9]{1,4}::?){1,7}[A-Fa-f0-9]{1,4})', '(?P<eui64>(?:[0-9A-Fa-f]{1,2}-){7}[0-9A-Fa-f]{1,2}|(?:[0-9A-Fa-f]{1,2}:){7}[0-9A-Fa-f]{1,2}|(?:[0-9A-Fa-f]{4}\\.){3}[0-9A-Fa-f]{4})', '(?P<eui48>(?:[0-9A-Fa-f]{1,2}-){5}[0-9A-Fa-f]{1,2}|(?:[0-9A-Fa-f]{1,2}:){5}[0-9A-Fa-f]{1,2}|(?:[0-9A-Fa-f]{4}\\.){2}[0-9A-Fa-f]{4})', '(?P<uuid>[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12})', '(?P<call>[\\w.]*?)\\(', '\\b(?P<bool_true>True)\\b|\\b(?P<bool_false>False)\\b|\\b(?P<none>None)\\b', '(?P<ellipsis>\\.\\.\\.)', '(?P<number_complex>(?<!\\w)(?:\\-?[0-9]+\\.?[0-9]*(?:e[-+]?\\d+?)?)(?:[-+](?:[0-9]+\\.?[0-9]*(?:e[-+]?\\d+)?))?j)', '(?P<number>(?<!\\w)\\-?[0-9]+\\.?[0-9]*(e[-+]?\\d+?)?\\b|0x[0-9a-fA-F]*)', '(?P<path>\\B(/[-\\w._+]+)*\\/)(?P<filename>[-\\w._+]*)?', '(?<![\\\\\\w])(?P<str>b?\'\'\'.*?(?<!\\\\)\'\'\'|b?\'.*?(?<!\\\\)\'|b?\\"\\"\\".*?(?<!\\\\)\\"\\"\\"|b?\\".*?(?<!\\\\)\\")', '(?P<url>(file|https|http|ws|wss)://[-0-9a-zA-Z$_+!`(),.?/;:&=%#~@]*)')]`  _class-attribute_

**Inherited (1)**

- from `rich.highlighter.RegexHighlighter`: `highlight`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Highlights the text typically produced from ``__repr__`` methods.


## _combine_regex

`rich.highlighter._combine_regex`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _combine_regex(regexes: str = ()) -> str
```

Combine a number of regexes in to a single regex.

Returns:
    str: New regex with all regexes ORed together.


