# `mcp.shared.inbound`

Distribution: `mcp`

## ERROR_CODE_HTTP_STATUS

`mcp.shared.inbound.ERROR_CODE_HTTP_STATUS`

```python
ERROR_CODE_HTTP_STATUS: Final[Mapping[int, int]] = MappingProxyType({PARSE_ERROR: 400, INVALID_REQUEST: 400, INVALID_PARAMS: 400, HEADER_MISMATCH: 400, MISSING_REQUIRED_CLIENT_CAPABILITY: 400, UNSUPPORTED_PROTOCOL_VERSION: 400, METHOD_NOT_FOUND: 404})
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

HTTP status to send for a JSON-RPC `error.code`.

Consulted for classifier-origin *and* handler-origin errors, so one table
decides the wire status regardless of where the error was produced. Unmapped
codes fall back to the caller's default (typically 200).


## MCP_METHOD_HEADER

Import as `mcp.client.session.MCP_METHOD_HEADER`  ·  defined at `mcp.shared.inbound.MCP_METHOD_HEADER`

```python
MCP_METHOD_HEADER: Final = 'mcp-method'
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Canonical lowercase name of the HTTP header carrying the JSON-RPC method.


## MCP_NAME_HEADER

Import as `mcp.client.session.MCP_NAME_HEADER`  ·  defined at `mcp.shared.inbound.MCP_NAME_HEADER`

```python
MCP_NAME_HEADER: Final = 'mcp-name'
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Canonical lowercase name of the HTTP header carrying the resource name (tool/prompt/resource URI).


## MCP_PARAM_HEADER_PREFIX

`mcp.shared.inbound.MCP_PARAM_HEADER_PREFIX`

```python
MCP_PARAM_HEADER_PREFIX: Final = 'Mcp-Param-'
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Prefix the `x-mcp-header` token is joined to, forming the per-parameter HTTP header name.


## MCP_PROTOCOL_VERSION_HEADER

Import as `mcp.client.session.MCP_PROTOCOL_VERSION_HEADER`  ·  defined at `mcp.shared.inbound.MCP_PROTOCOL_VERSION_HEADER`

```python
MCP_PROTOCOL_VERSION_HEADER: Final = 'mcp-protocol-version'
```

_8 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Canonical lowercase name of the HTTP header carrying the MCP protocol version.


## NAME_BEARING_METHODS

Import as `mcp.client.session.NAME_BEARING_METHODS`  ·  defined at `mcp.shared.inbound.NAME_BEARING_METHODS`

```python
NAME_BEARING_METHODS: Final[Mapping[str, str]] = MappingProxyType({'tools/call': 'name', 'prompts/get': 'name', 'resources/read': 'uri'})
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Method → params key whose value is mirrored as the `Mcp-Name` HTTP header.

Shared by client emit (which header to send) and server validate (which body
field to compare against), so both ends agree on the field by construction.


## X_MCP_HEADER_KEY

`mcp.shared.inbound.X_MCP_HEADER_KEY`

```python
X_MCP_HEADER_KEY: Final = 'x-mcp-header'
```

JSON-Schema property annotation that designates an `Mcp-Param-*` HTTP header.


## _B64_SENTINEL

`mcp.shared.inbound._B64_SENTINEL`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_B64_SENTINEL = re.compile('^=\\?base64\\?(?P<payload>.*)\\?=$')
```

## _CANONICAL_DECIMAL

`mcp.shared.inbound._CANONICAL_DECIMAL`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_CANONICAL_DECIMAL = re.compile('^-?[0-9]+(\\.[0-9]+)?$')
```

## _HEADER_SAFE

`mcp.shared.inbound._HEADER_SAFE`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_HEADER_SAFE = re.compile('^[\\x20-\\x7E]*$')
```

## _RFC9110_TOKEN

`mcp.shared.inbound._RFC9110_TOKEN`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_RFC9110_TOKEN = re.compile("^[!#$%&'*+\\-.^_`|~0-9A-Za-z]+$")
```

## _ROUTING_HEADER_NAMES

`mcp.shared.inbound._ROUTING_HEADER_NAMES`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_ROUTING_HEADER_NAMES: Final = frozenset({MCP_PROTOCOL_VERSION_HEADER, MCP_METHOD_HEADER, MCP_NAME_HEADER})
```

## _SUBSCHEMA_LIST

`mcp.shared.inbound._SUBSCHEMA_LIST`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_SUBSCHEMA_LIST: Final = frozenset({'allOf', 'anyOf', 'oneOf', 'prefixItems'})
```

## _SUBSCHEMA_MAP

`mcp.shared.inbound._SUBSCHEMA_MAP`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_SUBSCHEMA_MAP: Final = frozenset({'patternProperties', 'dependentSchemas', '$defs', 'definitions'})
```

## _SUBSCHEMA_SINGLE

`mcp.shared.inbound._SUBSCHEMA_SINGLE`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_SUBSCHEMA_SINGLE: Final = frozenset({'items', 'contains', 'unevaluatedItems', 'additionalProperties', 'propertyNames', 'unevaluatedProperties', 'not', 'if', 'then', 'else', 'contentSchema'})
```

## _X_MCP_HEADER_PRIMITIVE_TYPES

`mcp.shared.inbound._X_MCP_HEADER_PRIMITIVE_TYPES`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_X_MCP_HEADER_PRIMITIVE_TYPES: Final = frozenset({'string', 'integer', 'boolean'})
```

## __all__

`mcp.shared.inbound.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['ERROR_CODE_HTTP_STATUS', 'InboundLadderRejection', 'InboundModernRoute', 'MCP_METHOD_HEADER', 'MCP_NAME_HEADER', 'MCP_PARAM_HEADER_PREFIX', 'MCP_PROTOCOL_VERSION_HEADER', 'NAME_BEARING_METHODS', 'X_MCP_HEADER_KEY', 'classify_inbound_request', 'decode_header_value', 'encode_header_value', 'find_duplicated_routing_header', 'find_invalid_x_mcp_header', 'mcp_param_headers', 'unsupported_protocol_version_rejection', 'validate_mcp_param_headers', 'x_mcp_header_map']
```

## InboundLadderRejection

Import as `mcp.server.runner.InboundLadderRejection`  ·  defined at `mcp.shared.inbound.InboundLadderRejection`

```python
class InboundLadderRejection
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (3)**

- `code: int`  _instance-attribute_
- `data: Any = None`  _class-attribute, instance-attribute_
- `message: str`  _instance-attribute_

The first ladder rung that failed, as JSON-RPC error fields.


## InboundModernRoute

`mcp.shared.inbound.InboundModernRoute`

```python
class InboundModernRoute
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (3)**

- `client_capabilities: Any`  _instance-attribute_
- `client_info: Any`  _instance-attribute_
- `protocol_version: str`  _instance-attribute_

A modern-protocol request whose envelope passed every ladder rung.

`client_info` and `client_capabilities` are the raw envelope values; the
classifier checks presence only, not shape, and `client_info` is `None`
when the (optional, SHOULD-include) key is absent. Method existence is not
a ladder rung — kernel dispatch is the single source of truth for that.


## _annotated_positions

`mcp.shared.inbound._annotated_positions`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _annotated_positions(input_schema: Any) -> Iterator[tuple[tuple[str, ...], str, dict[str, Any]]]
```

Yield `(path, token, schema)` for every statically-reachable `x-mcp-header` annotation.

Shared by client emit and server validate so both ends agree on what counts as a declared header.


## _mcp_param_value_matches

`mcp.shared.inbound._mcp_param_value_matches`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _mcp_param_value_matches(prop_type: Any, value: Any, rendered: str, decoded: str) -> bool
```

True when a decoded `Mcp-Param-*` header value agrees with the body argument.

Integer-typed declarations with an integral body value compare numerically
(`42` matches `42.0`, the spec's SHOULD) for canonical-decimal headers —
exact, no float round-trip, so values beyond the IEEE754 safe range still
compare. Anything else compares against `rendered`, the emit-side rendering.


## _render_header_scalar

`mcp.shared.inbound._render_header_scalar`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _render_header_scalar(value: Any) -> str | None
```

Render `value` the way the client mirrors it into a header, or `None` when no rendering exists.

Shared by emit and validate so both sides agree on what is mirrorable:
non-primitives and ints beyond CPython's int-to-str digit limit are not.


## _value_at_path

`mcp.shared.inbound._value_at_path`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _value_at_path(arguments: Mapping[str, Any], path: tuple[str, ...]) -> Any
```

Read the value at a `properties`-key path in `arguments`, or `None` if any step is missing or non-mapping.


## _walk_schema_positions

`mcp.shared.inbound._walk_schema_positions`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _walk_schema_positions(root: Any) -> Iterator[tuple[tuple[str, ...] | None, dict[str, Any]]]
```

Yield `(properties_path, schema)` for every schema position in `root`.

`properties_path` is the chain of `properties` keys from the root to the
position, or `None` once any other applicator keyword has been crossed.
The root itself yields `()`. Only the JSON Schema 2020-12 applicators
listed above are entered; instance-data keywords are not, and `$ref` is
not dereferenced, so the walk terminates on any finite JSON value. An
explicit stack keeps the function total even on pathologically deep input.


## classify_inbound_request

Import as `mcp.server.runner.classify_inbound_request`  ·  defined at `mcp.shared.inbound.classify_inbound_request`

```python
def classify_inbound_request(body: Mapping[str, Any], headers: Mapping[str, str] | None = None, supported_modern_versions: Sequence[str] = MODERN_PROTOCOL_VERSIONS) -> InboundModernRoute | InboundLadderRejection
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Run the modern-protocol validation ladder over a decoded JSON-RPC body.

Rungs, in order — first failure wins:

1. `params._meta` is a mapping carrying the required envelope pair
   (protocol version, client capabilities) → else
   :data:`~mcp_types.jsonrpc.INVALID_PARAMS` naming the missing key(s)
   (basic/index.mdx "Per-request protocol fields"). Client info is
   optional (SHOULD-include, spec PR #3002); absent reads as `None`.
2. When `headers` is given, `MCP-Protocol-Version` equals the envelope's
   protocol version, `Mcp-Method` equals `body.method`, and — for the
   methods in :data:`NAME_BEARING_METHODS` — `Mcp-Name` equals the named
   body param → else :data:`~mcp_types.jsonrpc.HEADER_MISMATCH`. Runs
   before the supported-version rung so a client that disagrees with itself
   is told so, rather than told the body's version is unsupported.
3. The envelope's protocol version is a string in
   `supported_modern_versions` → non-string values are
   :data:`~mcp_types.jsonrpc.INVALID_PARAMS` (a shape defect, not a
   negotiation outcome), else
   :data:`~mcp_types.jsonrpc.UNSUPPORTED_PROTOCOL_VERSION` with
   `data = {"supported": [...], "requested": <value>}`.

Method existence is *not* a rung: kernel dispatch owns that decision so
custom-registered methods route and the answer lives in one place.

Args:
    body: The decoded JSON-RPC request mapping. Envelope shape
        (`jsonrpc` / `id`) is not checked here.
    headers: Transport headers keyed by lowercase name, or `None` to
        skip the header rung (non-HTTP callers).
    supported_modern_versions: Modern protocol revisions this server
        accepts on the per-request-envelope path.


## decode_header_value

`mcp.shared.inbound.decode_header_value`

```python
def decode_header_value(value: str | None) -> str | None
```

Inverse of :func:`encode_header_value`.

Returns the value verbatim unless it carries the `=?base64?...?=` sentinel,
in which case the payload is decoded as UTF-8. A malformed sentinel (bad
base64, non-canonical base64, or bad UTF-8) yields `None` so a corrupt
header never matches a body value by accident. `None` in → `None` out so
callers can pass `headers.get(...)` directly.


## encode_header_value

Import as `mcp.client.session.encode_header_value`  ·  defined at `mcp.shared.inbound.encode_header_value`

```python
def encode_header_value(value: str) -> str
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Wrap `value` in the `=?base64?...?=` sentinel when it would not survive an HTTP field round-trip.

Plain printable ASCII without leading/trailing whitespace passes verbatim;
anything else (control chars, non-ASCII, edge whitespace, or a value that
already looks like the sentinel) is base64-wrapped so the receiver can
recover the exact bytes.


## find_duplicated_routing_header

`mcp.shared.inbound.find_duplicated_routing_header`

```python
def find_duplicated_routing_header(headers: Iterable[tuple[str, str]]) -> str | None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Name of a routing header supplied more than once in raw header lines, or `None`.

Takes raw `(name, value)` pairs — a folded mapping hides duplicates. A
duplicate is rejected because first-copy and last-copy readers would
disagree. `Mcp-Param-*` duplicates are :func:`validate_mcp_param_headers`'s job.


## find_invalid_x_mcp_header

Import as `mcp.client.session.find_invalid_x_mcp_header`  ·  defined at `mcp.shared.inbound.find_invalid_x_mcp_header`

```python
def find_invalid_x_mcp_header(input_schema: Any) -> str | None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Return a reason string if any `x-mcp-header` annotation in `input_schema` is invalid; else `None`.

Walks every JSON Schema 2020-12 schema position. An annotation is valid
only when it sits on a property statically reachable from the root via a
chain of pure `properties` keys, names a non-empty RFC 9110 token, is on
an integer/string/boolean property, and is case-insensitively unique
across the whole schema. A `None` / non-mapping schema has no schema
positions and returns `None`.


## mcp_param_headers

Import as `mcp.client.session.mcp_param_headers`  ·  defined at `mcp.shared.inbound.mcp_param_headers`

```python
def mcp_param_headers(header_map: Mapping[tuple[str, ...], str], arguments: Mapping[str, Any]) -> dict[str, str]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Build the `Mcp-Param-*` headers a `tools/call` mirrors from its arguments.

For each `(path, token)` in `header_map`, read the value at that property
path in `arguments` and, when it is present and not `None`, emit
`Mcp-Param-<token>` carrying it: `bool` as `true`/`false`, other scalars via
`str`, each passed through :func:`encode_header_value` so a non-token value
is base64-wrapped. A path that hits a missing key or a non-mapping node is
skipped, matching the spec's "omit the header when no value is present",
as is a value with no header rendering.


## unsupported_protocol_version_rejection

`mcp.shared.inbound.unsupported_protocol_version_rejection`

```python
def unsupported_protocol_version_rejection(requested: str, supported_modern_versions: Sequence[str] = MODERN_PROTOCOL_VERSIONS) -> InboundLadderRejection | None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

The `UNSUPPORTED_PROTOCOL_VERSION` rejection for `requested`, or `None` if it is served.

The request ladder's last rung, shared with the transport's notification arm
so both message kinds name the same `supported` list in the same words.


## validate_mcp_param_headers

`mcp.shared.inbound.validate_mcp_param_headers`

```python
def validate_mcp_param_headers(input_schema: Any, arguments: Mapping[str, Any], headers: Mapping[str, str]) -> InboundLadderRejection | None
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Compare a `tools/call` request's `Mcp-Param-*` headers against its body arguments.

Each annotated property's header and argument must agree: present together
and equal after sentinel decoding, or absent together (`null` counts as
absent). Returns the first failure as a `HEADER_MISMATCH` rejection, else `None`.

A header whose argument is absent or unrenderable is deliberately rejected:
the spec's purpose clause is exactly an intermediary routing on a value the
body never carried. A duplicated recognized header is rejected — first-copy
and last-copy readers would disagree. A schema :func:`find_invalid_x_mcp_header`
rejects validates nothing: conforming clients drop the tool and emit no headers.


## x_mcp_header_map

Import as `mcp.client.session.x_mcp_header_map`  ·  defined at `mcp.shared.inbound.x_mcp_header_map`

```python
def x_mcp_header_map(input_schema: Any) -> dict[tuple[str, ...], str]
```

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Map each property carrying a valid `x-mcp-header` to its annotation token, keyed by property path.

The key is the chain of `properties` keys from the schema root to the
annotated property; a top-level property has a one-element path, a nested
one a longer path. Call only on a schema that
:func:`find_invalid_x_mcp_header` accepts; an invalid schema yields an
undefined subset.


