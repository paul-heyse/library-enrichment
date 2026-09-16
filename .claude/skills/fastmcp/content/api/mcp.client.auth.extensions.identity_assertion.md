# `mcp.client.auth.extensions.identity_assertion`

Distribution: `mcp`

## _DEFAULT_PORTS

`mcp.client.auth.extensions.identity_assertion._DEFAULT_PORTS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_DEFAULT_PORTS = {'https': 443, 'http': 80}
```

## IdentityAssertionOAuthProvider

`mcp.client.auth.extensions.identity_assertion.IdentityAssertionOAuthProvider`

```python
class IdentityAssertionOAuthProvider(RedirectAwareAuth)
```

**Bases** `RedirectAwareAuth`

**Declared members (1)**

- `requires_response_body = True`  _class-attribute, instance-attribute_

**Inherited (1)**

- from `mcp.shared._httpx_utils.RedirectAwareAuth`: `async_auth_flow`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

`httpx2.Auth` for the SEP-990 ID-JAG flow (RFC 7523 jwt-bearer grant) against a configured AS.

The authorization server `issuer` is fixed at construction; metadata is fetched from its
RFC 8414 well-known and the ID-JAG and client secret are sent only to that issuer's token
endpoint. The resource server is never consulted for AS selection. The ID-JAG is fetched lazily
from `assertion_provider` so a fresh assertion is used on each exchange.

Example:
    ```python
    async def fetch_id_jag(audience: str, resource: str) -> str:
        # `audience` is the configured issuer (the ID-JAG `aud`); `resource` is the MCP
        # server's identifier (the ID-JAG `resource` claim). Obtaining the ID-JAG from the
        # enterprise IdP is deployment-specific and not handled by the SDK.
        return await my_idp.issue_id_jag(audience=audience, resource=resource)


    provider = IdentityAssertionOAuthProvider(
        server_url="https://mcp.example.com/mcp",
        storage=my_token_storage,
        client_id="my-client-id",
        client_secret="my-client-secret",
        issuer="https://auth.example.com",
        assertion_provider=fetch_id_jag,
    )
    ```


## _origin

`mcp.client.auth.extensions.identity_assertion._origin`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _origin(url: str) -> tuple[str, str, int | None]
```

Return the (scheme, host, port) origin of a URL for same-origin comparison.

The port is normalized to the scheme's default so an explicit `:443`/`:80` compares equal to the
same origin written without a port.


