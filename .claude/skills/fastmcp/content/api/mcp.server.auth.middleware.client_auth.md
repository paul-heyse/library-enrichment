# `mcp.server.auth.middleware.client_auth`

Distribution: `mcp`

## AuthenticationError

Import as `mcp.server.auth.handlers.token.AuthenticationError`  ·  defined at `mcp.server.auth.middleware.client_auth.AuthenticationError`

```python
class AuthenticationError(Exception)
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Exception`

**Declared members (1)**

- `message = message`  _instance-attribute_

## ClientAuthenticator

Import as `mcp.server.auth.routes.ClientAuthenticator`  ·  defined at `mcp.server.auth.middleware.client_auth.ClientAuthenticator`

```python
class ClientAuthenticator
```

_6 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (2)**

- `async def authenticate_request(self, request: Request) -> OAuthClientInformationFull`  _async_
  Authenticate a client from an HTTP request.
- `provider = provider`  _instance-attribute_

ClientAuthenticator is a callable which validates requests from a client
application, used to verify /token calls.

If, during registration, the client requested to be issued a secret, the
authenticator asserts that /token calls must be authenticated with
that same secret.

NOTE: clients can opt for no authentication during registration, in which case this
logic is skipped.


