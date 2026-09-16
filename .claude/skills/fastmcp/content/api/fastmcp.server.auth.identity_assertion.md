# `fastmcp.server.auth.identity_assertion`

Distribution: `fastmcp`

## ID_JAG_GRANT_PROFILE

`fastmcp.server.auth.identity_assertion.ID_JAG_GRANT_PROFILE`

```python
ID_JAG_GRANT_PROFILE = 'urn:ietf:params:oauth:grant-profile:id-jag'
```

**Inferred type** (`ty`, not declared in the source): `Literal["urn:ietf:params:oauth:grant-profile:id-jag"]`

## ID_JAG_TYP

`fastmcp.server.auth.identity_assertion.ID_JAG_TYP`

```python
ID_JAG_TYP = 'oauth-id-jag+jwt'
```

**Inferred type** (`ty`, not declared in the source): `Literal["oauth-id-jag+jwt"]`

## JWT_BEARER_GRANT_TYPE

Import as `fastmcp.server.auth.oauth_proxy.proxy.JWT_BEARER_GRANT_TYPE`  ·  defined at `fastmcp.server.auth.identity_assertion.JWT_BEARER_GRANT_TYPE`

```python
JWT_BEARER_GRANT_TYPE = 'urn:ietf:params:oauth:grant-type:jwt-bearer'
```

**Inferred type** (`ty`, not declared in the source): `Literal["urn:ietf:params:oauth:grant-type:jwt-bearer"]`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## SUPPORTED_ASSERTION_ALGORITHMS

`fastmcp.server.auth.identity_assertion.SUPPORTED_ASSERTION_ALGORITHMS`

```python
SUPPORTED_ASSERTION_ALGORITHMS = frozenset({'RS256', 'RS384', 'RS512', 'PS256', 'PS384', 'PS512', 'ES256', 'ES384', 'ES512'})
```

**Inferred type** (`ty`, not declared in the source): `frozenset[str]`

## logger

`fastmcp.server.auth.identity_assertion.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## IdentityAssertion

Import as `fastmcp.server.auth.IdentityAssertion`  ·  defined at `fastmcp.server.auth.identity_assertion.IdentityAssertion`

```python
class IdentityAssertion(BaseModel)
```

**Also exported as** `fastmcp.server.auth.IdentityAssertion`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (7)**

- `access_token_expiry_seconds: int = Field(default=300, gt=0, description='Lifetime, in seconds, of the short-lived access token minted from an ID-JAG. SEP-990 relies on the client re-exchanging a fresh assertion, so this is intentionally short and no refresh token is issued.')`  _class-attribute, instance-attribute_
- `algorithm: str | None = Field(default=None, description='JWS signing algorithm the trusted issuers use (e.g. `ES256`, `PS256`). When omitted, verification defaults to `RS256`; IdPs signing with another algorithm must set this explicitly. When issuers use different algorithms, override per issuer with `algorithms`.')`  _class-attribute, instance-attribute_
- `algorithms: dict[str, str] | None = Field(default=None, description='Optional per-issuer signing-algorithm override, keyed by the issuer string (mirroring `jwks_uris`). Issuers absent here fall back to `algorithm`.')`  _class-attribute, instance-attribute_
- `audience: str | None = Field(default=None, description="Expected `aud` value on the ID-JAG. When omitted, the audience is this server's issuer identifier — the `issuer` published in its authorization server metadata, which is `issuer_url` when set and `base_url` otherwise — and that is where the ID-JAG's `aud` must point per SEP-990. Override only when the IdP mints assertions bound to a different audience identifier.")`  _class-attribute, instance-attribute_
- `jwks_uris: dict[str, str] | None = Field(default=None, description='Optional explicit JWKS URI per issuer, keyed by the issuer string. When an issuer is absent here, its JWKS URI is discovered via OIDC. Provide this for issuers that do not publish an OIDC discovery document.')`  _class-attribute, instance-attribute_
- `required_scopes: list[str] | None = Field(default=None, description='Scopes that must be present on the issued access token.')`  _class-attribute, instance-attribute_
- `trusted_issuers: list[str] = Field(..., description="Issuer (`iss`) values the authorization server accepts on an ID-JAG. Each must exactly match the assertion's `iss` claim. For each issuer, the JWKS used to verify the assertion signature is discovered via OIDC (`{issuer}/.well-known/openid-configuration`) unless overridden in `jwks_uris`.")`  _class-attribute, instance-attribute_

Configuration for server-side identity assertion (ID-JAG) support.

When attached to an :class:`~fastmcp.server.auth.oauth_proxy.OAuthProxy` via the
``identity_assertion`` parameter, the proxy's token endpoint accepts the RFC 7523
``jwt-bearer`` grant carrying an ID-JAG issued by one of the ``trusted_issuers``,
and mints a short-lived FastMCP access token for the asserted subject.

Example:
    ```python
    from fastmcp.server.auth import OAuthProxy, IdentityAssertion

    auth = OAuthProxy(
        ...,
        identity_assertion=IdentityAssertion(
            trusted_issuers=["https://login.acme-corp.com"],
        ),
    )
    ```


## IdentityAssertionError

Import as `fastmcp.server.auth.oauth_proxy.proxy.IdentityAssertionError`  ·  defined at `fastmcp.server.auth.identity_assertion.IdentityAssertionError`

```python
class IdentityAssertionError(Exception)
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Exception`

Raised when an ID-JAG fails validation.

The message is for server-side logging only; the token endpoint maps this to a
generic OAuth error response and does not leak the detail to the client.


## IdentityAssertionValidator

Import as `fastmcp.server.auth.oauth_proxy.proxy.IdentityAssertionValidator`  ·  defined at `fastmcp.server.auth.identity_assertion.IdentityAssertionValidator`

```python
class IdentityAssertionValidator
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (5)**

- `CLOCK_SKEW_SECONDS = 30`  _class-attribute, instance-attribute_
- `MAX_ASSERTION_LIFETIME = 300`  _class-attribute, instance-attribute_
- `audience: str | list[str] = config.audience`  _instance-attribute_
- `config = config`  _instance-attribute_
- `async def validate(self, assertion: str, client_id: str, resource_url: str | None) -> dict`  _async_
  Validate an ID-JAG and return its claims.

Validates ID-JAG assertions for the SEP-990 jwt-bearer grant.

Reuses :class:`JWTVerifier` for signature, issuer, audience, and expiry checks
(with JWKS fetching and caching), and layers on the SEP-990 processing rules
that the generic verifier does not cover: the ``typ`` JOSE header, a mandatory
``sub``, and ``jti`` replay rejection.

JTI replay protection mirrors :class:`CIMDAssertionValidator`: seen ``jti``
values are cached until the assertion would expire anyway, with periodic
cleanup and an emergency size cap. Like CIMD, the cache is per-process, so
replay protection is not shared across horizontally-scaled workers or
replicas; see the identity-assertion docs for the deployment caveat.


## _assertion_scopes

`fastmcp.server.auth.identity_assertion._assertion_scopes`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _assertion_scopes(claims: dict) -> list[str]
```

Extract the scopes an ID-JAG grants, from `scope` or `scp`.


## _decode_unverified_claims

`fastmcp.server.auth.identity_assertion._decode_unverified_claims`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _decode_unverified_claims(token: str) -> dict
```

Decode a JWT payload without verifying the signature.

Used only to read the `iss` claim so we can select the trusted issuer's key
before performing the real, signature-verifying decode.


## _numeric_date_claim

`fastmcp.server.auth.identity_assertion._numeric_date_claim`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _numeric_date_claim(claims: dict, name: str) -> float | None
```

Read a NumericDate claim (RFC 7519 §2), rejecting non-numeric values.

A validly-signed assertion could still carry a malformed `exp`/`iat`/`nbf`
(e.g. a string, from a misbehaving IdP); comparing against it directly
would raise `TypeError` outside the validation-error path. `bool` is
excluded even though it subclasses `int` in Python — `true`/`false` are
not timestamps.


## normalize_resource_url

Import as `fastmcp.server.auth.oauth_proxy.proxy.normalize_resource_url`  ·  defined at `fastmcp.server.auth.identity_assertion.normalize_resource_url`

```python
def normalize_resource_url(url: str) -> str
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Normalize a resource URL by removing query parameters and trailing slashes.

RFC 8707 allows clients to include query parameters in resource URLs, but
the server's configured resource URL typically doesn't include them. This
normalizes both sides for comparison by stripping query and fragment.


## server_url_has_query

Import as `fastmcp.server.auth.oauth_proxy.proxy.server_url_has_query`  ·  defined at `fastmcp.server.auth.identity_assertion.server_url_has_query`

```python
def server_url_has_query(url: str) -> bool
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Check if a URL has query parameters.


