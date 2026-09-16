# `fastmcp.utilities.docstring_parsing`

Distribution: `fastmcp`

## _PARSERS

`fastmcp.utilities.docstring_parsing._PARSERS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_PARSERS = ('google', 'numpy', 'sphinx')
```

## logger

`fastmcp.utilities.docstring_parsing.logger`

```python
logger = logging.getLogger('griffe')
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## ParsedDocstring

Import as `fastmcp.tools.function_parsing.ParsedDocstring`  ·  defined at `fastmcp.utilities.docstring_parsing.ParsedDocstring`

```python
class ParsedDocstring
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (2)**

- `description: str | None = None`  _class-attribute, instance-attribute_
- `parameters: dict[str, str] = field(default_factory=dict)`  _class-attribute, instance-attribute_

The extracted description and per-parameter descriptions from a docstring.


## parse_docstring

Import as `fastmcp.tools.function_parsing.parse_docstring`  ·  defined at `fastmcp.utilities.docstring_parsing.parse_docstring`

```python
def parse_docstring(fn: Callable[..., Any]) -> ParsedDocstring
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Parse a function's docstring into a summary and parameter descriptions.

Tries Google, NumPy, and Sphinx parsers in order, using the first one that
successfully extracts parameter descriptions. If none do, returns the full
docstring as the description with no parameter descriptions.


