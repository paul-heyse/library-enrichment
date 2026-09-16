# `mcp.shared.extension`

Distribution: `mcp`

## _IDENTIFIER_RE

`mcp.shared.extension._IDENTIFIER_RE`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_IDENTIFIER_RE = re.compile(f'{_LABEL}(?:\\.{_LABEL})*/{_NAME}')
```

## _LABEL

`mcp.shared.extension._LABEL`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_LABEL = '[A-Za-z](?:[A-Za-z0-9-]*[A-Za-z0-9])?'
```

## _NAME

`mcp.shared.extension._NAME`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_NAME = '[A-Za-z0-9](?:[A-Za-z0-9._-]*[A-Za-z0-9])?'
```

## __all__

`mcp.shared.extension.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['validate_extension_identifier']
```

## validate_extension_identifier

Import as `mcp.client.client.validate_extension_identifier`  ·  defined at `mcp.shared.extension.validate_extension_identifier`

```python
def validate_extension_identifier(identifier: Any, owner: str) -> None
```

_5 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Raise `TypeError` unless `identifier` is a `vendor-prefix/name` string.

SEP-2133 requires extension identifiers to carry a reverse-DNS prefix.


