# `fastmcp.server.auth.redirect_validation`

Distribution: `fastmcp`

## DEFAULT_LOCALHOST_PATTERNS

`fastmcp.server.auth.redirect_validation.DEFAULT_LOCALHOST_PATTERNS`

```python
DEFAULT_LOCALHOST_PATTERNS = ['http://localhost:*', 'http://127.0.0.1:*']
```

**Inferred type** (`ty`, not declared in the source): `list[str]`

## UNSAFE_REDIRECT_URI_SCHEMES

`fastmcp.server.auth.redirect_validation.UNSAFE_REDIRECT_URI_SCHEMES`

```python
UNSAFE_REDIRECT_URI_SCHEMES = frozenset({'javascript', 'data', 'file', 'vbscript'})
```

**Inferred type** (`ty`, not declared in the source): `frozenset[str]`

## _has_dot_segments

`fastmcp.server.auth.redirect_validation._has_dot_segments`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _has_dot_segments(path: str) -> bool
```

Return True if a URI path contains `.` or `..` segments.

Browsers collapse dot-segments when resolving a 302 Location per RFC
3986 §5.2.4. Allowing them through the allowlist lets an attacker craft
a URI that passes pattern matching but lands on a different path after
redirect. Checks both the raw path and its percent-decoded form so that
encoded variants like `/foo/%2e%2e/bar` are rejected.


## _is_unsafe_redirect_uri

`fastmcp.server.auth.redirect_validation._is_unsafe_redirect_uri`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _is_unsafe_redirect_uri(uri: str) -> bool
```

## _match_host

`fastmcp.server.auth.redirect_validation._match_host`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _match_host(uri_host: str | None, pattern_host: str | None) -> bool
```

Match host component, supporting *.example.com wildcard patterns.

Args:
    uri_host: The host from the URI being validated
    pattern_host: The host pattern (may start with *.)

Returns:
    True if the host matches


## _match_path

`fastmcp.server.auth.redirect_validation._match_path`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _match_path(uri_path: str, pattern_path: str) -> bool
```

Match path component using fnmatch for wildcard support.

Args:
    uri_path: The path from the URI
    pattern_path: The path pattern (may contain * wildcards)

Returns:
    True if the path matches


## _match_port

`fastmcp.server.auth.redirect_validation._match_port`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _match_port(uri_port: str | None, pattern_port: str | None, uri_scheme: str) -> bool
```

Match port component, supporting * wildcard for any port.

Args:
    uri_port: The port from the URI (None if default, string otherwise)
    pattern_port: The port from the pattern (None if default, "*" for wildcard)
    uri_scheme: The URI scheme (http/https) for default port handling

Returns:
    True if the port matches


## _parse_host_port

`fastmcp.server.auth.redirect_validation._parse_host_port`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _parse_host_port(netloc: str) -> tuple[str | None, str | None]
```

Parse host and port from netloc, handling wildcards.

Args:
    netloc: The netloc component (e.g., "localhost:8080" or "localhost:*")

Returns:
    Tuple of (host, port_str) where port_str may be "*" or a number string


## add_query_params

`fastmcp.server.auth.redirect_validation.add_query_params`

```python
def add_query_params(url: str, params: dict[str, str]) -> str
```

Append query parameters to a URL while preserving existing parameters.

The existing query string is appended to verbatim rather than decoded
and re-serialized, since registered redirect URIs may carry opaque or
signed query strings whose exact bytes matter to the receiving client
(for example, a valueless `?flag` must not become `?flag=`, and
non-UTF-8 percent-encoded sequences must not be replaced).


## build_client_redirect

Import as `fastmcp.server.auth.oauth_proxy.proxy.build_client_redirect`  ·  defined at `fastmcp.server.auth.redirect_validation.build_client_redirect`

```python
def build_client_redirect(url: str, params: dict[str, str], iss: str) -> str
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Build a client-facing authorization redirect that carries exactly one `iss`.

Every redirect this server sends back to an OAuth client from the
authorization endpoint -- success (carrying `code`) or error (carrying
`error`) -- must carry the proxy's RFC 9207 issuer exactly once (RFC
6749 §3.1 forbids a response parameter from appearing more than once).
A registered redirect_uri can legitimately carry its own `iss` query
parameter already (e.g. `https://client.example/callback?iss=tenant`),
so blindly appending the server's issuer on top of that would duplicate
it -- this is what every client-facing redirect site must get right,
and the reason this helper exists instead of five call sites each
reimplementing the same invariant by hand.

`params` is appended to `url` via `add_query_params` (verbatim, without
re-encoding the existing query -- see that function's docstring), and
`iss` is then set idempotently via `replace_query_param`: an existing
occurrence -- whether contributed by the registered redirect_uri or
already present in `url` -- is overwritten with the canonical value;
otherwise `iss` is appended.

`iss` is keyword-only and required so a caller cannot forget to pass
it. `params` must not itself contain an `"iss"` key -- pass it via the
`iss` keyword instead, so there is exactly one place the value can come
from.

Args:
    url: The redirect target -- normally the client's registered
        redirect_uri.
    params: The response parameters to append (e.g. `code`/`state`, or
        `error`/`error_description`). Must not include `"iss"`.
    iss: The canonical RFC 9207 issuer, byte-for-byte equal to the
        discovery document's `issuer` (`str(self.base_url)` /
        `self._issuer` -- never the rstripped `self._base_url`).

Returns:
    `url` with `params` appended and exactly one `iss` query parameter
    set to `iss`.


## is_loopback_host

Import as `fastmcp.server.auth.oauth_proxy.models.is_loopback_host`  ·  defined at `fastmcp.server.auth.redirect_validation.is_loopback_host`

```python
def is_loopback_host(host: str | None) -> bool
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Check if a host is a loopback address.

Per RFC 8252 §7.3, loopback covers the whole reserved loopback range, not
just the two familiar literals: IPv4 `127.0.0.0/8` (so `127.0.0.2` and
`127.5.5.5` are loopback just as much as `127.0.0.1`) and IPv6 `::1`. IP
hosts are therefore classified with `ipaddress.ip_address().is_loopback`
rather than string equality — checking only `127.0.0.1` would let a web
client register `https://127.0.0.2/callback` and slip past the
non-loopback requirement.

Names are handled per RFC 6761 §6.3, which reserves the entire `localhost`
namespace for the local machine: the exact name `localhost` *and* any
subdomain of it (`app.localhost`, `api.app.localhost`). `.localhost` is a
reserved TLD that cannot be registered, so a subdomain of it always resolves
to the loopback interface and must count as loopback in both directions —
otherwise a web client could register `https://app.localhost/callback` and
slip past the non-loopback requirement, while a native client using
`http://app.localhost:3000/callback` would be wrongly rejected.

The suffix test is anchored on a leading dot so it cannot be spoofed by a
registrable domain: `localhost.evil.com` and `notlocalhost` are ordinary
public names and are *not* loopback.

Hosts are also normalized before classification: bracketed IPv6 literals
(`[::1]`) are unwrapped, and a single trailing dot (the absolute/FQDN form,
e.g. `localhost.` or `127.0.0.1.`) is stripped, since it denotes the same
host. Non-IP hosts fall through to the name check without raising.


## is_redirect_uri_allowed_for_application_type

Import as `fastmcp.server.auth.oauth_proxy.proxy.is_redirect_uri_allowed_for_application_type`  ·  defined at `fastmcp.server.auth.redirect_validation.is_redirect_uri_allowed_for_application_type`

```python
def is_redirect_uri_allowed_for_application_type(redirect_uri: str | AnyUrl, application_type: str | None) -> bool
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Check a redirect URI against RFC 7591 / SEP-837 `application_type` rules.

`application_type` governs which redirect URIs a Dynamically Registered
Client may use (RFC 7591 §2, OpenID Connect Dynamic Client Registration §2):

- `"web"` clients must use `https` redirect URIs on a non-loopback host.
  Loopback `http`, `https://localhost`, and app/custom schemes are rejected.
  This is the restriction SEP-837 actually asks for.
- `"native"` clients keep every scheme FastMCP already allowed, except that
  `http` is restricted to loopback hosts (RFC 8252 §7.3, any port). App and
  private-use schemes pass through untouched: `vscode://`,
  `com.example.app:/callback`, `myapp://callback`, `urn:ietf:wg:oauth:2.0:oob`.

Deliberately absent: any attempt to classify a native client's scheme as
"private-use" versus "a network transport". There is no sound test. The IANA
registry cannot separate them — `vscode` is registered *because* it is an
app-dispatch scheme, alongside transports like `coap` and `smb` — and
reverse-domain notation fails too, since `iris.beep` and
`microsoft.windows.camera` are registered while `myapp` is not. Every
formulation either rejects schemes real MCP clients depend on or admits the
ones it meant to exclude, so native scheme filtering is left to the
unsafe-scheme check below.

Unsafe browser schemes (`javascript:`, `data:`, `file:`, `vbscript:`) are
always rejected regardless of `application_type`. That check predates this
function and is unchanged by it.

The MCP SDK defaults `application_type` to `"native"` because MCP clients
typically register loopback redirect URIs, so omitting the field preserves
the behavior clients relied on before this check existed. `None` — which a
registered-client record carries when the field was never set — is treated
the same way.


## matches_allowed_pattern

Import as `fastmcp.server.auth.cimd.matches_allowed_pattern`  ·  defined at `fastmcp.server.auth.redirect_validation.matches_allowed_pattern`

```python
def matches_allowed_pattern(uri: str, pattern: str) -> bool
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Securely check if a URI matches an allowed pattern with wildcard support.

This function parses both the URI and pattern as URLs, comparing each
component separately to prevent bypass attacks like userinfo injection.

Patterns support wildcards:
- http://localhost:* matches any localhost port
- http://127.0.0.1:* matches any 127.0.0.1 port
- https://*.example.com/* matches any subdomain of example.com
- https://app.example.com/auth/* matches any path under /auth/

Security: Rejects URIs with userinfo (user:pass@host) which could bypass
naive string matching (e.g., http://localhost@evil.com).

Args:
    uri: The redirect URI to validate
    pattern: The allowed pattern (may contain wildcards)

Returns:
    True if the URI matches the pattern


## replace_query_param

`fastmcp.server.auth.redirect_validation.replace_query_param`

```python
def replace_query_param(url: str, key: str, value: str) -> str
```

Replace the first occurrence of `key` in a URL's query string in place.

Like `add_query_params`, this does not round-trip the query through
`parse_qsl`/`urlencode`: every segment other than the matched one is
passed through byte-for-byte, so opaque or non-UTF-8 percent-encoded
values elsewhere in the query are left untouched. Only the matched
segment's encoding is replaced (with `key=value`, freshly
`urlencode`d). If `key` is not present, it is appended, matching
`add_query_params`'s behavior.


## validate_redirect_uri

Import as `fastmcp.server.auth.oauth_proxy.proxy.validate_redirect_uri`  ·  defined at `fastmcp.server.auth.redirect_validation.validate_redirect_uri`

```python
def validate_redirect_uri(redirect_uri: str | AnyUrl | None, allowed_patterns: list[str] | None) -> bool
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Validate a redirect URI against allowed patterns.

Args:
    redirect_uri: The redirect URI to validate
    allowed_patterns: List of allowed patterns. If None, ordinary URIs are allowed
                     for DCR compatibility, while unsafe browser schemes are rejected.
                     If empty list, no URIs are allowed.
                     To restrict to localhost only, explicitly pass DEFAULT_LOCALHOST_PATTERNS.

Returns:
    True if the redirect URI is allowed


