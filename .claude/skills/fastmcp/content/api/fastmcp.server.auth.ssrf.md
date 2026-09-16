# `fastmcp.server.auth.ssrf`

Distribution: `fastmcp`

## IPV4_TRANSLATED_PREFIX

`fastmcp.server.auth.ssrf.IPV4_TRANSLATED_PREFIX`

```python
IPV4_TRANSLATED_PREFIX = ipaddress.IPv6Network('0:0:0:0:ffff:0:0:0/96')
```

**Inferred type** (`ty`, not declared in the source): `IPv6Network`

## ISATAP_INTERFACE_IDS

`fastmcp.server.auth.ssrf.ISATAP_INTERFACE_IDS`

```python
ISATAP_INTERFACE_IDS = (b'\x00\x00^\xfe', b'\x02\x00^\xfe')
```

**Inferred type** (`ty`, not declared in the source): `tuple[Literal[b"\x00\x00^\xfe"], Literal[b"\x02\x00^\xfe"]]`

## LOW32_OFFSETS

`fastmcp.server.auth.ssrf.LOW32_OFFSETS`

```python
LOW32_OFFSETS = (12, 13, 14, 15)
```

**Inferred type** (`ty`, not declared in the source): `tuple[Literal[12], Literal[13], Literal[14], Literal[15]]`

## NAT64_PREFIXES

`fastmcp.server.auth.ssrf.NAT64_PREFIXES`

```python
NAT64_PREFIXES: tuple[tuple[ipaddress.IPv6Network, tuple[tuple[int, int, int, int], ...]], ...] = ((ipaddress.IPv6Network('64:ff9b::/96'), ((12, 13, 14, 15),)), (ipaddress.IPv6Network('64:ff9b:1::/48'), ((6, 7, 9, 10), (7, 9, 10, 11), (9, 10, 11, 12), (12, 13, 14, 15))))
```

## logger

`fastmcp.server.auth.ssrf.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## SSRFError

Import as `fastmcp.server.auth.cimd.SSRFError`  ·  defined at `fastmcp.server.auth.ssrf.SSRFError`

```python
class SSRFError(Exception)
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Exception`

Raised when an SSRF protection check fails.


## SSRFFetchError

Import as `fastmcp.server.auth.cimd.SSRFFetchError`  ·  defined at `fastmcp.server.auth.ssrf.SSRFFetchError`

```python
class SSRFFetchError(Exception)
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Exception`

Raised when SSRF-safe fetch fails.


## SSRFFetchResponse

`fastmcp.server.auth.ssrf.SSRFFetchResponse`

```python
class SSRFFetchResponse
```

**Declared members (3)**

- `content: bytes`  _instance-attribute_
- `headers: dict[str, str]`  _instance-attribute_
- `status_code: int`  _instance-attribute_

Response payload from an SSRF-safe fetch.


## ValidatedURL

`fastmcp.server.auth.ssrf.ValidatedURL`

```python
class ValidatedURL
```

**Declared members (6)**

- `hostname: str`  _instance-attribute_
- `original_url: str`  _instance-attribute_
- `path: str`  _instance-attribute_
- `port: int`  _instance-attribute_
- `proxy_url: str | None = None`  _class-attribute, instance-attribute_
- `resolved_ips: list[str]`  _instance-attribute_

A URL that has been validated for SSRF with resolved IPs.


## _FetchTarget

`fastmcp.server.auth.ssrf._FetchTarget`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _FetchTarget
```

**Declared members (4)**

- `host_header: str | None`  _instance-attribute_
- `proxy_url: str | None = None`  _class-attribute, instance-attribute_
- `sni_hostname: str | None`  _instance-attribute_
- `url: str`  _instance-attribute_

A single connection attempt for an SSRF-safe fetch.

In pinned (default) mode there is one target per resolved IP: the request goes to
an IP-literal URL with Host and SNI pinned to the validated hostname. In proxy
mode (FASTMCP_SSRF_TRUST_PROXY) there is a single target: the original hostname
URL with no pinning and an explicit ``proxy_url``, so the request is dialed
through the trusted proxy and the proxy (not httpx's environment-proxy routing)
owns DNS and TLS.


## _build_fetch_targets

`fastmcp.server.auth.ssrf._build_fetch_targets`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _build_fetch_targets(validated: ValidatedURL) -> list[_FetchTarget]
```

Build the ordered connection attempts for a validated URL.

An empty ``resolved_ips`` means proxy mode (see :func:`validate_url`): a single
unpinned request to the original hostname URL, explicitly routed through
``validated.proxy_url``. Otherwise, one pinned IP-literal request per resolved
IP, tried in order with fallback on connection error.


## _configured_proxy_url

`fastmcp.server.auth.ssrf._configured_proxy_url`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _configured_proxy_url() -> str | None
```

Return the proxy URL to route proxy-trust fetches through, if any is set.

Reads ``HTTPS_PROXY``/``https_proxy`` first, falling back to ``ALL_PROXY``/
``all_proxy``. This is a simple presence check: no host matching, no ``NO_PROXY``
evaluation. The caller passes the returned URL to httpx2 explicitly (with
``trust_env`` disabled) so there is no routing decision left for httpx2 to make
differently than this function assumed — see the module docstring and
:func:`validate_url` for why that matters.

Returns:
    The configured proxy URL, or None if none of the supported variables are set.


## _embedded_ipv4_addresses

`fastmcp.server.auth.ssrf._embedded_ipv4_addresses`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _embedded_ipv4_addresses(ip: ipaddress.IPv6Address) -> set[ipaddress.IPv4Address]
```

Return IPv4 addresses embedded in known IPv6 transition forms.


## format_ip_for_url

`fastmcp.server.auth.ssrf.format_ip_for_url`

```python
def format_ip_for_url(ip_str: str) -> str
```

Format IP address for use in URL (bracket IPv6 addresses).

IPv6 addresses must be bracketed in URLs to distinguish the address from
the port separator. For example: https://[2001:db8::1]:443/path

Args:
    ip_str: IP address string

Returns:
    IP string suitable for URL (IPv6 addresses are bracketed)


## is_ip_allowed

`fastmcp.server.auth.ssrf.is_ip_allowed`

```python
def is_ip_allowed(ip_str: str) -> bool
```

Check if an IP address is allowed (must be globally routable unicast).

Uses ip.is_global which catches:
- Private (10.x, 172.16-31.x, 192.168.x)
- Loopback (127.x, ::1)
- Link-local (169.254.x, fe80::) - includes AWS metadata!
- Reserved, unspecified
- RFC6598 Carrier-Grade NAT (100.64.0.0/10) - can point to internal networks
- IPv6 transition forms that embed blocked IPv4 targets

Additionally blocks multicast addresses (not caught by is_global).

Args:
    ip_str: IP address string to check

Returns:
    True if the IP is allowed (public unicast internet), False if blocked


## resolve_hostname

`fastmcp.server.auth.ssrf.resolve_hostname`

```python
async def resolve_hostname(hostname: str, port: int = 443) -> list[str]
```

Resolve hostname to IP addresses using DNS.

Args:
    hostname: Hostname to resolve
    port: Port number (used for getaddrinfo)

Returns:
    List of resolved IP addresses

Raises:
    SSRFError: If resolution fails


## ssrf_safe_fetch

Import as `fastmcp.server.auth.providers.jwt.ssrf_safe_fetch`  ·  defined at `fastmcp.server.auth.ssrf.ssrf_safe_fetch`

```python
async def ssrf_safe_fetch(url: str, require_path: bool = False, max_size: int = 5120, timeout: float = 10.0, overall_timeout: float = 30.0) -> bytes
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Fetch URL with comprehensive SSRF protection and DNS pinning.

Security measures:
1. HTTPS only
2. DNS resolution with IP validation
3. Connects to validated IP directly (DNS pinning prevents rebinding)
4. Response size limit
5. Redirects disabled
6. Overall timeout

Args:
    url: URL to fetch
    require_path: If True, require non-root path
    max_size: Maximum response size in bytes (default 5KB)
    timeout: Per-operation timeout in seconds
    overall_timeout: Overall timeout for entire operation

Returns:
    Response body as bytes

Raises:
    SSRFError: If SSRF validation fails
    SSRFFetchError: If fetch fails


## ssrf_safe_fetch_response

Import as `fastmcp.server.auth.cimd.ssrf_safe_fetch_response`  ·  defined at `fastmcp.server.auth.ssrf.ssrf_safe_fetch_response`

```python
async def ssrf_safe_fetch_response(url: str, require_path: bool = False, max_size: int = 5120, timeout: float = 10.0, overall_timeout: float = 30.0, request_headers: Mapping[str, str] | None = None, allowed_status_codes: set[int] | None = None) -> SSRFFetchResponse
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Fetch URL with SSRF protection and return response metadata.

This is equivalent to :func:`ssrf_safe_fetch` but returns response headers
and status code, and supports conditional request headers.


## validate_url

Import as `fastmcp.server.auth.cimd.validate_url`  ·  defined at `fastmcp.server.auth.ssrf.validate_url`

```python
async def validate_url(url: str, require_path: bool = False) -> ValidatedURL
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Validate URL for SSRF and resolve to IPs.

Args:
    url: URL to validate
    require_path: If True, require non-root path (for CIMD)

Returns:
    ValidatedURL with resolved IPs

Raises:
    SSRFError: If the URL is invalid, resolves to blocked IPs, or proxy-trust
        mode is enabled but no configured proxy will route the request.


