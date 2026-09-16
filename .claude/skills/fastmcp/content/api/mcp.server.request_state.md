# `mcp.server.request_state`

Distribution: `mcp`

## _ENVELOPE_VERSION

`mcp.server.request_state._ENVELOPE_VERSION`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_ENVELOPE_VERSION = 1
```

## _FUTURE_SKEW

`mcp.server.request_state._FUTURE_SKEW`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_FUTURE_SKEW = 60.0
```

## _KDF_INFO

`mcp.server.request_state._KDF_INFO`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_KDF_INFO = b'mcp/request-state/v1/aes-256-gcm'
```

## _KID_INFO

`mcp.server.request_state._KID_INFO`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_KID_INFO = b'mcp/request-state/v1/kid:'
```

## _KID_LEN

`mcp.server.request_state._KID_LEN`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_KID_LEN = 4
```

## _MRTR_METHODS

`mcp.server.request_state._MRTR_METHODS`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_MRTR_METHODS = INPUT_REQUIRED_METHODS
```

## _NONCE_LEN

`mcp.server.request_state._NONCE_LEN`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_NONCE_LEN = 12
```

## _PRINCIPAL_LABEL

`mcp.server.request_state._PRINCIPAL_LABEL`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_PRINCIPAL_LABEL = b'mcp/request-state/principal:'
```

## _RoundBinding

`mcp.server.request_state._RoundBinding`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_RoundBinding = tuple[str, str, str | None]
```

The (target, args-digest, principal) one round's envelope binds, computed once per round.


## _TOKEN_PREFIX

`mcp.server.request_state._TOKEN_PREFIX`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_TOKEN_PREFIX = 'v1.'
```

## __all__

`mcp.server.request_state.__all__`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
__all__ = ['AESGCMRequestStateCodec', 'InvalidRequestState', 'RequestStateBoundary', 'RequestStateCodec', 'RequestStateSecurity', 'authenticated_principal']
```

## logger

`mcp.server.request_state.logger`

```python
logger = logging.getLogger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## AESGCMRequestStateCodec

Import as `mcp.server.mcpserver.AESGCMRequestStateCodec`  ·  defined at `mcp.server.request_state.AESGCMRequestStateCodec`

```python
class AESGCMRequestStateCodec
```

**Also exported as** `mcp.server.mcpserver.AESGCMRequestStateCodec`

**Declared members (2)**

- `def seal(self, payload: bytes) -> str`
- `def unseal(self, token: str) -> bytes`

Built-in codec: AES-256-GCM under key(s) derived with HKDF-SHA256.

Tokens are encrypted, not merely signed, so clients cannot read the state.
`keys[0]` seals; all keys unseal (rotation, see `RequestStateSecurity`).
Each token carries a 4-byte non-secret key fingerprint for an O(1) ring
lookup, and the "v1." prefix and fingerprint are bound into the GCM
associated data, so a token cannot be replayed into another format version
or ring slot. Key bytes are copied at construction.


## InvalidRequestState

Import as `mcp.server.mcpserver.InvalidRequestState`  ·  defined at `mcp.server.request_state.InvalidRequestState`

```python
class InvalidRequestState(Exception)
```

**Also exported as** `mcp.server.mcpserver.InvalidRequestState`

**Bases** `Exception`

A sealed `requestState` token failed verification.

The message is a log-only reason code; the boundary never puts it on the wire.


## RequestStateBoundary

Import as `mcp.server.mcpserver.RequestStateBoundary`  ·  defined at `mcp.server.request_state.RequestStateBoundary`

```python
class RequestStateBoundary
```

**Also exported as** `mcp.server.mcpserver.RequestStateBoundary`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Server middleware sealing/unsealing `requestState` at the wire boundary.

Acts only on the multi-round-trip carriers (tools/call, prompts/get,
resources/read); every other method passes through untouched.

Inbound state is verified (codec unseal plus claims check) and replaced
with the plaintext the server minted before any interceptor or handler
runs; failure answers -32602 with the frozen message "Invalid or expired
requestState", the real reason going to the server log only. Outbound, an
`input_required` result carrying `requestState` is sealed in a fresh
claims envelope; handlers and resolvers never call the codec.

`default_audience` seeds the audience claim when the policy sets none, and
must be stated explicitly: it is the service identity that stops state
minted by another service sharing the same keys. `MCPServer` installs this
middleware with its server name by default (under an ephemeral policy
unless `request_state_security=` supplies one); lowlevel `Server` users
append one to `server.middleware`, passing their server's name (or `None`
to deliberately leave tokens audience-free).


## RequestStateCodec

Import as `mcp.server.mcpserver.RequestStateCodec`  ·  defined at `mcp.server.request_state.RequestStateCodec`

```python
class RequestStateCodec(Protocol)
```

**Also exported as** `mcp.server.mcpserver.RequestStateCodec`

**Bases** `Protocol`

**Declared members (2)**

- `def seal(self, payload: bytes) -> str`
  Return an opaque URL-safe token protecting `payload`.
- `def unseal(self, token: str) -> bytes`
  Reverse `seal`.

Authenticated crypto over the framework's request-state envelope.

The framework stamps and re-verifies every envelope claim (expiry, request
binding, principal); a codec only provides integrity and, ideally,
confidentiality (a sign-only codec leaves the payload client-readable).

Requirements: `unseal(seal(payload))` round-trips, and `unseal` raises
`InvalidRequestState` for any token it did not mint unmodified; tokens
never name their algorithm (version with a format prefix bound under the
authentication tag, RFC 8725); comparisons are constant-time. Both methods
are synchronous, so cache key material rather than calling a KMS per token.


## RequestStateSecurity

Import as `mcp.server.mcpserver.RequestStateSecurity`  ·  defined at `mcp.server.request_state.RequestStateSecurity`

```python
class RequestStateSecurity
```

**Also exported as** `mcp.server.mcpserver.RequestStateSecurity`

_4 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (5)**

- `audience: str | None = audience`  _instance-attribute_
- `bind_principal: Callable[[ServerRequestContext[Any, Any]], str | None] | None = bind_principal`  _instance-attribute_
- `codec: RequestStateCodec`  _instance-attribute_
- `def ephemeral(cls, ttl: float = 600.0, audience: str | None = None) -> RequestStateSecurity`  _classmethod_
  Protection under a key generated now and held only by this process.
- `ttl: float = ttl`  _instance-attribute_

Policy for protecting `requestState`: codec, TTL, principal, audience.

Exactly one of `keys` or `codec`:

    RequestStateSecurity(keys=[secret])      # built-in AES-256-GCM
    RequestStateSecurity(codec=MyKmsCodec()) # bring your own crypto
    RequestStateSecurity.ephemeral()         # process-local key

`keys` is the rotation ring: `keys[0]` seals, every key unseals.
Zero-downtime rotation, each phase fully rolled out before the next:
`keys=[old, new]`, then `keys=[new, old]`, then `keys=[new]` after one TTL.

The boundary enforces expiry, request binding, audience, and principal for
every codec, fail-closed in both directions. `audience=None` defers to the
boundary's `default_audience` (`MCPServer` passes its server name).


## _b64u

`mcp.server.request_state._b64u`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _b64u(data: bytes) -> str
```

## _b64u_decode

`mcp.server.request_state._b64u_decode`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _b64u_decode(text: str) -> bytes
```

Strict inverse of `_b64u`: only the canonical unpadded encoding decodes.


## _bound_principal

`mcp.server.request_state._bound_principal`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _bound_principal(security: RequestStateSecurity, ctx: ServerRequestContext[Any, Any], fail: Callable[[str], NoReturn]) -> str | None
```

Run `bind_principal` under the deny-on-error discipline, in one place for both directions.

`fail` converts a failure into the calling direction's wire shape: the
frozen rejection when verifying, the sanitized internal error when sealing.


## _derive_key

`mcp.server.request_state._derive_key`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _derive_key(secret: bytes) -> bytes
```

Stretch an operator secret (>= 32 bytes, any format) into the AES-256 key.


## _principal_bytes

`mcp.server.request_state._principal_bytes`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _principal_bytes(principal: str) -> bytes
```

## _principal_claim

`mcp.server.request_state._principal_claim`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _principal_claim(principal: str) -> str
```

## _principal_matches

`mcp.server.request_state._principal_matches`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _principal_matches(claim: str, principal: str) -> bool
```

## _reject

`mcp.server.request_state._reject`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _reject(method: str, reason: str) -> NoReturn
```

Refuse a round: frozen wire error, real reason to the server log only.


## _request_identity

`mcp.server.request_state._request_identity`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _request_identity(method: str, params: Mapping[str, Any] | None) -> tuple[str, str]
```

Salient (target, args-digest) for the request a token binds to.

Per-method allowlist, never a denylist: a future wire field cannot silently join the digest.


## authenticated_principal

Import as `mcp.server.mcpserver.authenticated_principal`  ·  defined at `mcp.server.request_state.authenticated_principal`

```python
def authenticated_principal(ctx: ServerRequestContext[Any, Any]) -> str | None
```

**Also exported as** `mcp.server.mcpserver.authenticated_principal`

Default principal binding: the authenticated (client, issuer, subject) identity.

Uses the same components session ownership uses, so two users of one OAuth
client are distinct principals whenever the token verifier supplies a
subject, and the binding degrades to the client identity when it does not.
Returns `None` (state not principal-bound) on unauthenticated transports.


## compact_json

Import as `mcp.server.mcpserver.resolve.compact_json`  ·  defined at `mcp.server.request_state.compact_json`

```python
def compact_json(value: Any, sort_keys: bool = False) -> str
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Canonical JSON for everything the state path digests or seals.

ASCII output keeps the encode total: a lone surrogate in client-supplied
text escapes instead of raising. Anything consuming this must parse with
stdlib `json.loads`, which accepts those escapes (pydantic's JSON parser
does not).


