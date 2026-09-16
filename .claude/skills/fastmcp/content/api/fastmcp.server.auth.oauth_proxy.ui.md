# `fastmcp.server.auth.oauth_proxy.ui`

Distribution: `fastmcp`

## create_consent_html

Import as `fastmcp.server.auth.oauth_proxy.consent.create_consent_html`  ·  defined at `fastmcp.server.auth.oauth_proxy.ui.create_consent_html`

```python
def create_consent_html(client_id: str, redirect_uri: str, scopes: list[str], txn_id: str, csrf_token: str, client_name: str | None = None, title: str = 'Application Access Request', server_name: str | None = None, server_icon_url: str | None = None, server_website_url: str | None = None, client_website_url: str | None = None, csp_policy: str | None = None, is_cimd_client: bool = False, cimd_domain: str | None = None) -> str
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Create a styled HTML consent page for OAuth authorization requests.

Args:
    csp_policy: Content Security Policy override.
        If None, uses the built-in CSP policy with appropriate directives.
        If empty string "", disables CSP entirely (no meta tag is rendered).
        If a non-empty string, uses that as the CSP policy value.


## create_error_html

Import as `fastmcp.server.auth.oauth_proxy.proxy.create_error_html`  ·  defined at `fastmcp.server.auth.oauth_proxy.ui.create_error_html`

```python
def create_error_html(error_title: str, error_message: str, error_details: dict[str, str] | None = None, server_name: str | None = None, server_icon_url: str | None = None) -> str
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Create a styled HTML error page for OAuth errors.

Args:
    error_title: The error title (e.g., "OAuth Error", "Authorization Failed")
    error_message: The main error message to display
    error_details: Optional dictionary of error details to show (e.g., `{"Error Code": "invalid_client"}`)
    server_name: Optional server name to display
    server_icon_url: Optional URL to server icon/logo

Returns:
    Complete HTML page as a string


