# `fastmcp.utilities.auth`

Distribution: `fastmcp`

## _decode_jwt_part

`fastmcp.utilities.auth._decode_jwt_part`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _decode_jwt_part(token: str, part_index: int) -> dict[str, Any]
```

Decode a JWT part (header or payload) without signature verification.

Args:
    token: JWT token string (header.payload.signature)
    part_index: 0 for header, 1 for payload

Returns:
    Decoded part as a dictionary

Raises:
    ValueError: If token is not a valid JWT format


## decode_jwt_header

Import as `fastmcp.server.auth.identity_assertion.decode_jwt_header`  ·  defined at `fastmcp.utilities.auth.decode_jwt_header`

```python
def decode_jwt_header(token: str) -> dict[str, Any]
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Decode JWT header without signature verification.

Useful for extracting the key ID (kid) for JWKS lookup.

Args:
    token: JWT token string (header.payload.signature)

Returns:
    Decoded header as a dictionary

Raises:
    ValueError: If token is not a valid JWT format


## decode_jwt_payload

Import as `fastmcp.server.auth.providers.azure.decode_jwt_payload`  ·  defined at `fastmcp.utilities.auth.decode_jwt_payload`

```python
def decode_jwt_payload(token: str) -> dict[str, Any]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Decode JWT payload without signature verification.

Use only for tokens received directly from trusted sources (e.g., IdP token endpoints).

Args:
    token: JWT token string (header.payload.signature)

Returns:
    Decoded payload as a dictionary

Raises:
    ValueError: If token is not a valid JWT format


## parse_scopes

Import as `fastmcp.server.auth.providers.aws.parse_scopes`  ·  defined at `fastmcp.utilities.auth.parse_scopes`

```python
def parse_scopes(value: Any) -> list[str] | None
```

_17 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Parse scopes from environment variables or settings values.

Accepts either a JSON array string, a comma- or space-separated string,
a list of strings, or ``None``. Returns a list of scopes or ``None`` if
no value is provided.


