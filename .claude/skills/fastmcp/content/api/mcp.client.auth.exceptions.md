# `mcp.client.auth.exceptions`

Distribution: `mcp`

## OAuthFlowError

Import as `mcp.client.auth.OAuthFlowError`  ·  defined at `mcp.client.auth.exceptions.OAuthFlowError`

```python
class OAuthFlowError(Exception)
```

**Also exported as** `mcp.client.auth.OAuthFlowError`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Exception`

Base exception for OAuth flow errors.


## OAuthRegistrationError

Import as `mcp.client.auth.OAuthRegistrationError`  ·  defined at `mcp.client.auth.exceptions.OAuthRegistrationError`

```python
class OAuthRegistrationError(OAuthFlowError)
```

**Also exported as** `mcp.client.auth.OAuthRegistrationError`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `OAuthFlowError`

Raised when client registration fails.


## OAuthTokenError

Import as `mcp.client.auth.OAuthTokenError`  ·  defined at `mcp.client.auth.exceptions.OAuthTokenError`

```python
class OAuthTokenError(OAuthFlowError)
```

**Also exported as** `mcp.client.auth.OAuthTokenError`

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `OAuthFlowError`

Raised when token operations fail.


