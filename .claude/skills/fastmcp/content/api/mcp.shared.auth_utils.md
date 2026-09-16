# `mcp.shared.auth_utils`

Distribution: `mcp`

## calculate_token_expiry

Import as `mcp.client.auth.oauth2.calculate_token_expiry`  ·  defined at `mcp.shared.auth_utils.calculate_token_expiry`

```python
def calculate_token_expiry(expires_in: int | str | None) -> float | None
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Calculate token expiry timestamp from expires_in seconds.

Args:
    expires_in: Seconds until token expiration (may be string from some servers)

Returns:
    Unix timestamp when token expires, or None if no expiry specified


## check_resource_allowed

Import as `mcp.client.auth.oauth2.check_resource_allowed`  ·  defined at `mcp.shared.auth_utils.check_resource_allowed`

```python
def check_resource_allowed(requested_resource: str, configured_resource: str) -> bool
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Check if a requested resource URL matches a configured resource URL.

A requested resource matches if it has the same scheme, domain, port,
and its path starts with the configured resource's path. This allows
hierarchical matching where a token for a parent resource can be used
for child resources.

Args:
    requested_resource: The resource URL being requested
    configured_resource: The resource URL that has been configured

Returns:
    True if the requested resource matches the configured resource


## resource_url_from_server_url

Import as `mcp.client.auth.oauth2.resource_url_from_server_url`  ·  defined at `mcp.shared.auth_utils.resource_url_from_server_url`

```python
def resource_url_from_server_url(url: str | HttpUrl | AnyUrl) -> str
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Convert server URL to canonical resource URL per RFC 8707.

RFC 8707 section 2 states that resource URIs "MUST NOT include a fragment component".
Returns absolute URI with lowercase scheme/host for canonical form.

Args:
    url: Server URL to convert

Returns:
    Canonical resource URL string


