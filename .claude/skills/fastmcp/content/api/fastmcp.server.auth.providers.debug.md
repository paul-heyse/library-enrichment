# `fastmcp.server.auth.providers.debug`

Distribution: `fastmcp`

## logger

`fastmcp.server.auth.providers.debug.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## DebugTokenVerifier

Import as `fastmcp.server.auth.DebugTokenVerifier`  ·  defined at `fastmcp.server.auth.providers.debug.DebugTokenVerifier`

```python
class DebugTokenVerifier(TokenVerifier)
```

**Also exported as** `fastmcp.server.auth.DebugTokenVerifier`

**Bases** `TokenVerifier`

**Declared members (4)**

- `client_id = client_id`  _instance-attribute_
- `scopes = scopes or []`  _instance-attribute_
- `validate = validate`  _instance-attribute_
- `async def verify_token(self, token: str) -> AccessToken | None`  _async_
  Verify token using custom validation logic.

**Inherited (10)**

- from `fastmcp.server.auth.auth.AuthProvider`: `base_url`, `challenge_scopes`, `get_challenge_scopes`, `get_middleware`, `get_routes`, `get_well_known_routes`, `required_scopes`, `resource_base_url`, `scopes_supported`, `set_mcp_path`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

Token verifier with custom validation logic.

This verifier delegates token validation to a user-provided callable.
By default, it accepts all non-empty tokens (useful for testing).

Use cases:
- Testing: Accept any token without real verification
- Development: Custom validation logic for prototyping
- Opaque tokens: When you have tokens with no introspection endpoint

WARNING: This bypasses standard security checks. Only use in controlled
environments or when you understand the security implications.


