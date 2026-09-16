# `mcp.shared.uri_template`

Distribution: `mcp`

## DEFAULT_MAX_TEMPLATE_LENGTH

`mcp.shared.uri_template.DEFAULT_MAX_TEMPLATE_LENGTH`

```python
DEFAULT_MAX_TEMPLATE_LENGTH = 8192
```

**Inferred type** (`ty`, not declared in the source): `Literal[8192]`

## DEFAULT_MAX_URI_LENGTH

`mcp.shared.uri_template.DEFAULT_MAX_URI_LENGTH`

```python
DEFAULT_MAX_URI_LENGTH = 65536
```

**Inferred type** (`ty`, not declared in the source): `Literal[65536]`

## DEFAULT_MAX_VARIABLES

`mcp.shared.uri_template.DEFAULT_MAX_VARIABLES`

```python
DEFAULT_MAX_VARIABLES = 256
```

**Inferred type** (`ty`, not declared in the source): `Literal[256]`

## Operator

`mcp.shared.uri_template.Operator`

```python
Operator = Literal['', '+', '#', '.', '/', ';', '?', '&']
```

**Inferred type** (`ty`, not declared in the source): ````xml <special-form 'Literal["", "+", "#", ".", "/", ... omitted 3 literals]'> ````

## _Atom

`mcp.shared.uri_template._Atom`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_Atom: TypeAlias = _Lit | _Cap
```

## _OPERATORS

`mcp.shared.uri_template._OPERATORS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_OPERATORS: frozenset[str] = frozenset({'+', '#', '.', '/', ';', '?', '&'})
```

## _OPERATOR_SPECS

`mcp.shared.uri_template._OPERATOR_SPECS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_OPERATOR_SPECS: dict[Operator, _OperatorSpec] = {'': _OperatorSpec(prefix='', separator=',', named=False, allow_reserved=False, ifemp=''), '+': _OperatorSpec(prefix='', separator=',', named=False, allow_reserved=True, ifemp=''), '#': _OperatorSpec(prefix='#', separator=',', named=False, allow_reserved=True, ifemp=''), '.': _OperatorSpec(prefix='.', separator='.', named=False, allow_reserved=False, ifemp=''), '/': _OperatorSpec(prefix='/', separator='/', named=False, allow_reserved=False, ifemp=''), ';': _OperatorSpec(prefix=';', separator=';', named=True, allow_reserved=False, ifemp=''), '?': _OperatorSpec(prefix='?', separator='&', named=True, allow_reserved=False, ifemp='='), '&': _OperatorSpec(prefix='&', separator='&', named=True, allow_reserved=False, ifemp='=')}
```

## _PCT_TRIPLET_RE

`mcp.shared.uri_template._PCT_TRIPLET_RE`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_PCT_TRIPLET_RE = re.compile('%[0-9A-Fa-f]{2}')
```

## _Part

`mcp.shared.uri_template._Part`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_Part = str | _Expression
```

## _RESERVED

`mcp.shared.uri_template._RESERVED`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_RESERVED = ":/?#[]@!$&'()*+,;="
```

## _STOP_CHARS

`mcp.shared.uri_template._STOP_CHARS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_STOP_CHARS: dict[Operator, str] = {'': '/?#&,', '+': '?#', '#': '', '.': './?#', '/': '/?#', ';': ';/?#', '?': '&#', '&': '&#'}
```

## _VARNAME_RE

`mcp.shared.uri_template._VARNAME_RE`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_VARNAME_RE = re.compile('^[A-Za-z0-9_]+(?:\\.[A-Za-z0-9_]+)*$')
```

## __all__

`mcp.shared.uri_template.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['DEFAULT_MAX_TEMPLATE_LENGTH', 'DEFAULT_MAX_VARIABLES', 'DEFAULT_MAX_URI_LENGTH', 'InvalidUriTemplate', 'Operator', 'UriTemplate', 'Variable']
```

## InvalidUriTemplate

Import as `mcp.InvalidUriTemplate`  ·  defined at `mcp.shared.uri_template.InvalidUriTemplate`

```python
class InvalidUriTemplate(ValueError)
```

**Also exported as** `mcp.InvalidUriTemplate`

**Bases** `ValueError`

**Declared members (2)**

- `position = position`  _instance-attribute_
- `template = template`  _instance-attribute_

Raised when a URI template string is malformed or unsupported.

Attributes:
    template: The template string that failed to parse.
    position: Character offset where the error was detected, or None
        if the error is not tied to a specific position.


## UriTemplate

Import as `mcp.UriTemplate`  ·  defined at `mcp.shared.uri_template.UriTemplate`

```python
class UriTemplate
```

**Also exported as** `mcp.UriTemplate`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (8)**

- `def expand(self, variables: Mapping[str, str | Sequence[str]]) -> str`
  Expand the template by substituting variable values.
- `def is_template(value: str) -> bool`  _staticmethod_
  Check whether a string contains URI template expressions.
- `def match(self, uri: str, max_uri_length: int = DEFAULT_MAX_URI_LENGTH) -> dict[str, str | list[str]] | None`
  Match a concrete URI against this template and extract variables.
- `def parse(cls, template: str, max_length: int = DEFAULT_MAX_TEMPLATE_LENGTH, max_variables: int = DEFAULT_MAX_VARIABLES) -> UriTemplate`  _classmethod_
  Parse a URI template string.
- `query_variable_names: frozenset[str]`  _property_
  Names of variables that :meth:`match` treats as optional query parameters.
- `template: str`  _instance-attribute_
- `variable_names: list[str]`  _property_
  All variable names in the template, in order of appearance.
- `variables: list[Variable]`  _property_
  All variables in the template, in order of appearance.

A parsed RFC 6570 URI template.

Construct via :meth:`parse`. Instances are immutable and hashable;
equality is based on the template string alone.


## Variable

`mcp.shared.uri_template.Variable`

```python
class Variable
```

**Declared members (3)**

- `explode: bool = False`  _class-attribute, instance-attribute_
- `name: str`  _instance-attribute_
- `operator: Operator`  _instance-attribute_

A single variable within a URI template expression.


## _Cap

`mcp.shared.uri_template._Cap`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _Cap
```

**Declared members (2)**

- `ifemp: bool = False`  _class-attribute, instance-attribute_
- `var: Variable`  _instance-attribute_

A single-variable capture in the flattened match-atom sequence.

``ifemp`` marks the ``;`` operator's optional-equals quirk: ``{;id}``
expands to ``;id=value`` or bare ``;id`` when the value is empty, so
the scan must accept both forms.


## _Expression

`mcp.shared.uri_template._Expression`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _Expression
```

**Declared members (2)**

- `operator: Operator`  _instance-attribute_
- `variables: list[Variable]`  _instance-attribute_

A parsed ``{...}`` expression: one operator, one or more variables.


## _Lit

`mcp.shared.uri_template._Lit`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _Lit
```

**Declared members (1)**

- `text: str`  _instance-attribute_

A literal run in the flattened match-atom sequence.


## _OperatorSpec

`mcp.shared.uri_template._OperatorSpec`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _OperatorSpec
```

**Declared members (5)**

- `allow_reserved: bool`  _instance-attribute_
  Keep reserved characters unencoded ({+var}, {#var}).
- `ifemp: str`  _instance-attribute_
  Suffix after a named variable whose expanded value is empty (RFC §A): '' for ;, '=' for ?/&.
- `named: bool`  _instance-attribute_
  Emit ``name=value`` pairs (query/path-param style) rather than bare values.
- `prefix: str`  _instance-attribute_
  Leading character emitted before the first variable.
- `separator: str`  _instance-attribute_
  Character between variables (and between exploded list items).

Expansion behavior for a single operator (RFC 6570 §3.2, Table in §A).


## _check_duplicate_variables

`mcp.shared.uri_template._check_duplicate_variables`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _check_duplicate_variables(template: str, variables: list[Variable]) -> None
```

Reject templates that use the same variable name more than once.

RFC 6570 requires repeated variables to expand to the same value,
which would require backreference matching with potentially
exponential cost. Rather than silently returning only the last
captured value, we reject at parse time.

Raises:
    InvalidUriTemplate: If any variable name appears more than once.


## _check_single_query_expression

`mcp.shared.uri_template._check_single_query_expression`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _check_single_query_expression(template: str, parts: list[_Part]) -> None
```

Reject templates with more than one ``{?...}`` expression.

The ``?`` operator emits a leading ``?``, so two such expressions
expand to a URI with two ``?`` characters — malformed per RFC 3986
§3.4. Use ``{?a,b}`` or ``{?a}{&b}`` for multiple query parameters.


## _encode

`mcp.shared.uri_template._encode`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _encode(value: str, allow_reserved: bool) -> str
```

Percent-encode a value per RFC 6570 §3.2.1.

Simple expansion encodes everything except unreserved characters.
Reserved expansion (``{+var}``, ``{#var}``) additionally keeps
RFC 3986 reserved characters intact and passes through existing
``%XX`` pct-triplets unchanged (RFC 6570 §3.2.3). A bare ``%`` not
followed by two hex digits is still encoded to ``%25``.


## _expand_expression

`mcp.shared.uri_template._expand_expression`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _expand_expression(expr: _Expression, variables: Mapping[str, str | Sequence[str]]) -> str
```

Expand a single ``{...}`` expression into its URI fragment.

Walks the expression's variables, encoding and joining defined ones
according to the operator's spec. Undefined variables are skipped
(RFC 6570 §2.3); if all are undefined, the expression contributes
nothing (no prefix is emitted).


## _extract_greedy

`mcp.shared.uri_template._extract_greedy`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _extract_greedy(var: Variable, raw: str) -> str | list[str] | None
```

Decode the greedy variable's isolated middle span.

For scalar greedy (``{+var}``, ``{#var}``) this is a stop-char
validation and a single ``unquote``. For explode variables the span
is a run of separator-delimited segments (``/a/b/c`` or
``;keys=a;keys=b``) that is split, validated, and decoded per item.


## _flatten

`mcp.shared.uri_template._flatten`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _flatten(parts: list[_Part]) -> list[_Atom]
```

Lower expressions into a flat sequence of literals and single-variable captures.

Operator prefixes and separators become explicit ``_Lit`` atoms so
the scan only ever sees two atom kinds. Adjacent literals are
coalesced so that anchor-finding (``find``/``rfind``) operates on
the longest possible literal, reducing false matches.

Explode variables emit no lead literal: the explode capture
includes its own separator-prefixed repetitions (``{/a*}`` →
``/x/y/z``, not ``/`` then ``x/y/z``).


## _is_greedy

`mcp.shared.uri_template._is_greedy`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_greedy(var: Variable) -> bool
```

Return True if this variable can span multiple path segments.

Reserved/fragment expansion and explode variables are the only
constructs whose match range is not bounded by a single structural
delimiter. A template may contain at most one such variable.


## _is_str_sequence

`mcp.shared.uri_template._is_str_sequence`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_str_sequence(value: object) -> bool
```

Check if value is a non-string sequence whose items are all strings.


## _parse

`mcp.shared.uri_template._parse`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _parse(template: str, max_variables: int) -> tuple[list[_Part], list[Variable]]
```

Split a template into an ordered sequence of literals and expressions.

Walks the string, alternating between collecting literal runs and
parsing ``{...}`` expressions. The resulting ``parts`` sequence
preserves positional interleaving so ``match()`` and ``expand()`` can
walk it in order.

Raises:
    InvalidUriTemplate: On unclosed braces, too many expressions, or
        any error surfaced by :func:`_parse_expression`.


## _parse_expression

`mcp.shared.uri_template._parse_expression`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _parse_expression(template: str, body: str, pos: int) -> _Expression
```

Parse the body of a single ``{...}`` expression.

The body is everything between the braces. It consists of an optional
leading operator character followed by one or more comma-separated
variable specifiers. Each specifier is a name with an optional
trailing ``*`` (explode modifier).

Args:
    template: The full template string, for error reporting.
    body: The expression body, braces excluded.
    pos: Character offset of the opening brace, for error reporting.

Raises:
    InvalidUriTemplate: On empty body, invalid variable names, or
        unsupported modifiers.


## _parse_query

`mcp.shared.uri_template._parse_query`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _parse_query(query: str) -> dict[str, str]
```

Parse a query string into a name→value mapping.

Unlike ``urllib.parse.parse_qs``, this follows RFC 3986 semantics:
``+`` is a literal sub-delim, not a space. Form-urlencoding treats
``+`` as space for HTML form submissions, but RFC 6570 and MCP
resource URIs follow RFC 3986 where only ``%20`` encodes a space.

Parameter names are **not** percent-decoded. RFC 6570 expansion
never encodes variable names, so a legitimate match will always
have the name in literal form. Decoding names would let
``%74oken=evil&token=real`` shadow the real ``token`` parameter
via first-wins.

Duplicate keys keep the first value. Pairs without ``=`` are
treated as empty-valued.


## _partition_greedy

`mcp.shared.uri_template._partition_greedy`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _partition_greedy(atoms: list[_Atom], template: str) -> tuple[list[_Atom], Variable | None, list[_Atom]]
```

Split atoms at the single greedy variable, if any.

Returns ``(prefix, greedy_var, suffix)``. If there is no greedy
variable the entire atom list is returned as the suffix so that
the right-to-left scan (which matches regex-greedy semantics)
handles it.

Raises:
    InvalidUriTemplate: If two variables are adjacent with no
        literal between them — whether or not one is the
        multi-segment variable, the scan has nothing to anchor the
        boundary on — or if more than one multi-segment variable
        is present (two are inherently ambiguous: there is no
        principled way to decide which one absorbs an extra
        segment).


## _scan_prefix

`mcp.shared.uri_template._scan_prefix`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _scan_prefix(atoms: Sequence[_Atom], uri: str, start: int, limit: int) -> tuple[dict[str, str | list[str]], int] | None
```

Scan atoms left-to-right from ``start``, not exceeding ``limit``.

Each bounded variable takes the minimum span that lets its
following literal match (found via ``find``), leaving the
greedy variable as much of the URI as possible.


## _scan_suffix

`mcp.shared.uri_template._scan_suffix`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _scan_suffix(atoms: Sequence[_Atom], uri: str, end: int, anchored: bool) -> tuple[dict[str, str | list[str]], int] | None
```

Scan atoms right-to-left from ``end``, returning captures and start position.

Each bounded variable takes the minimum span that lets its
preceding literal match (found via ``rfind``), which makes the
*first* variable in template order greedy — identical to Python
regex semantics for a sequence of greedy groups.

When ``anchored`` is true the atom sequence is the entire template
(no greedy variable), so ``atoms[0]`` must match at URI position 0
rather than at its rightmost occurrence.


## _split_query_tail

`mcp.shared.uri_template._split_query_tail`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _split_query_tail(parts: list[_Part]) -> tuple[list[_Part], list[Variable]]
```

Separate trailing ``?``/``&`` expressions from the path portion.

Lenient query matching (order-agnostic, partial, ignores extras)
applies when a template ends with one or more consecutive ``?``/``&``
expressions and the preceding path portion contains no literal
``?``. If the path has a literal ``?`` (e.g., ``?fixed=1{&page}``),
the URI's ``?`` split won't align with the template's expression
boundary, so the strict scan is used instead.

Returns:
    A pair ``(path_parts, query_vars)``. If lenient matching does
    not apply, ``query_vars`` is empty and ``path_parts`` is the
    full input.


