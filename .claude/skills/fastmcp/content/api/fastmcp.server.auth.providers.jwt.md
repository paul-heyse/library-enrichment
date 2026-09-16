# `fastmcp.server.auth.providers.jwt`

Distribution: `fastmcp`

## JWKKeyData

`fastmcp.server.auth.providers.jwt.JWKKeyData`

```python
JWKKeyData: TypeAlias = dict[str, str | list[str]]
```

## SUPPORTED_JWS_HEADER_FIELDS

`fastmcp.server.auth.providers.jwt.SUPPORTED_JWS_HEADER_FIELDS`

```python
SUPPORTED_JWS_HEADER_FIELDS = frozenset(JWS_HEADER_REGISTRY)
```

**Inferred type** (`ty`, not declared in the source): `frozenset[str]`

## logger

`fastmcp.server.auth.providers.jwt.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## JWKData

`fastmcp.server.auth.providers.jwt.JWKData`

```python
class JWKData(TypedDict)
```

**Bases** `TypedDict`

**Declared members (10)**

- `alg: str`  _instance-attribute_
- `crv: str`  _instance-attribute_
- `e: str`  _instance-attribute_
- `kid: str`  _instance-attribute_
- `kty: str`  _instance-attribute_
- `n: str`  _instance-attribute_
- `use: str`  _instance-attribute_
- `x: str`  _instance-attribute_
- `x5c: list[str]`  _instance-attribute_
- `x5t: str`  _instance-attribute_

JSON Web Key data structure.


## JWKSData

`fastmcp.server.auth.providers.jwt.JWKSData`

```python
class JWKSData(TypedDict)
```

**Bases** `TypedDict`

**Declared members (1)**

- `keys: list[JWKData]`  _instance-attribute_

JSON Web Key Set data structure.


## JWTVerifier

Import as `fastmcp.server.auth.JWTVerifier`  ·  defined at `fastmcp.server.auth.providers.jwt.JWTVerifier`

```python
class JWTVerifier(TokenVerifier)
```

**Also exported as** `fastmcp.server.auth.JWTVerifier`

_12 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `TokenVerifier`

**Declared members (9)**

- `algorithm = algorithm`  _instance-attribute_
- `audience = audience`  _instance-attribute_
- `issuer = issuer`  _instance-attribute_
- `jwks_uri = jwks_uri`  _instance-attribute_
- `async def load_access_token(self, token: str) -> AccessToken | None`  _async_
  Validate a JWT bearer token and return an AccessToken when the token is valid.
- `logger = get_logger(__name__)`  _instance-attribute_
- `public_key = public_key`  _instance-attribute_
- `ssrf_safe = ssrf_safe`  _instance-attribute_
- `async def verify_token(self, token: str) -> AccessToken | None`  _async_
  Verify a bearer token and return access info if valid.

**Inherited (10)**

- from `fastmcp.server.auth.auth.AuthProvider`: `base_url`, `challenge_scopes`, `get_challenge_scopes`, `get_middleware`, `get_routes`, `get_well_known_routes`, `required_scopes`, `resource_base_url`, `scopes_supported`, `set_mcp_path`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

JWT token verifier supporting asymmetric (RSA/ECDSA/EdDSA) and symmetric (HMAC) algorithms.

This verifier validates JWT tokens using various signing algorithms:
- **Asymmetric algorithms** (RS256/384/512, ES256/384/512, PS256/384/512,
  Ed25519, Ed448, and legacy EdDSA):
  Uses public/private key pairs. Ideal for external clients and services where
  only the authorization server has the private key.
- **Symmetric algorithms** (HS256/384/512): Uses a shared secret for both
  signing and verification. Perfect for internal microservices and trusted
  environments where the secret can be securely shared.

Use this when:
- You have JWT tokens issued by an external service (asymmetric)
- You need JWKS support for automatic key rotation (asymmetric)
- You have internal microservices sharing a secret key (symmetric)
- Your tokens contain standard OAuth scopes and claims


## RSAKeyPair

Import as `fastmcp.contrib.component_manager.example.RSAKeyPair`  ·  defined at `fastmcp.server.auth.providers.jwt.RSAKeyPair`

```python
class RSAKeyPair
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (4)**

- `def create_token(self, subject: str = 'fastmcp-user', issuer: str = 'https://fastmcp.example.com', audience: str | list[str] | None = None, scopes: list[str] | None = None, expires_in_seconds: int = 3600, additional_claims: dict[str, Any] | None = None, kid: str | None = None) -> str`
  Generate a test JWT token for testing purposes.
- `def generate(cls) -> RSAKeyPair`  _classmethod_
  Generate an RSA key pair for testing.
- `private_key: SecretStr`  _instance-attribute_
- `public_key: str`  _instance-attribute_

RSA key pair for JWT testing.


## StaticTokenVerifier

Import as `fastmcp.server.auth.StaticTokenVerifier`  ·  defined at `fastmcp.server.auth.providers.jwt.StaticTokenVerifier`

```python
class StaticTokenVerifier(TokenVerifier)
```

**Also exported as** `fastmcp.server.auth.StaticTokenVerifier`

**Bases** `TokenVerifier`

**Declared members (2)**

- `tokens = tokens`  _instance-attribute_
- `async def verify_token(self, token: str) -> AccessToken | None`  _async_
  Verify token against static token dictionary.

**Inherited (10)**

- from `fastmcp.server.auth.auth.AuthProvider`: `base_url`, `challenge_scopes`, `get_challenge_scopes`, `get_middleware`, `get_routes`, `get_well_known_routes`, `required_scopes`, `resource_base_url`, `scopes_supported`, `set_mcp_path`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Simple static token verifier for testing and development.

This verifier validates tokens against a predefined dictionary of valid token
strings and their associated claims. When a token string matches a key in the
dictionary, the verifier returns the corresponding claims as if the token was
validated by a real authorization server.

Use this when:
- You're developing or testing locally without a real OAuth server
- You need predictable tokens for automated testing
- You want to simulate different users/scopes without complex setup
- You're prototyping and need simple API key-style authentication

WARNING: Never use this in production - tokens are stored in plain text!


## _has_unsupported_critical_headers

`fastmcp.server.auth.providers.jwt._has_unsupported_critical_headers`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _has_unsupported_critical_headers(header: dict[str, Any]) -> bool
```

## _import_key_for_algorithm

`fastmcp.server.auth.providers.jwt._import_key_for_algorithm`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _import_key_for_algorithm(key: str | bytes | JWKKeyData, algorithm: str)
```

## _jwk_to_pem

`fastmcp.server.auth.providers.jwt._jwk_to_pem`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _jwk_to_pem(key_data: JWKKeyData) -> str
```

## _key_type_for_algorithm

`fastmcp.server.auth.providers.jwt._key_type_for_algorithm`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _key_type_for_algorithm(algorithm: str) -> Literal['oct', 'RSA', 'EC', 'OKP']
```

## _looks_like_pem_public_key

`fastmcp.server.auth.providers.jwt._looks_like_pem_public_key`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _looks_like_pem_public_key(key: str | bytes) -> bool
```

Return True when key text appears to be PEM-encoded asymmetric key material.


