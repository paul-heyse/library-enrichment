# `mcp.server.elicitation`

Distribution: `mcp`

## ElicitSchemaModelT

Import as `mcp.server.mcpserver.context.ElicitSchemaModelT`  ·  defined at `mcp.server.elicitation.ElicitSchemaModelT`

```python
ElicitSchemaModelT = TypeVar('ElicitSchemaModelT', bound=BaseModel)
```

**Inferred type** (`ty`, not declared in the source): `TypeVar`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## ElicitationResult

Import as `mcp.server.mcpserver.ElicitationResult`  ·  defined at `mcp.server.elicitation.ElicitationResult`

```python
ElicitationResult = TypeAliasType('ElicitationResult', AcceptedElicitation[ElicitSchemaModelT] | DeclinedElicitation | CancelledElicitation, type_params=(ElicitSchemaModelT,))
```

**Inferred type** (`ty`, not declared in the source): `type ElicitationResult[ElicitSchemaModelT] = AcceptedElicitation[ElicitSchemaModelT] | DeclinedElicitation | CancelledElicitation`

**Also exported as** `mcp.server.mcpserver.ElicitationResult`, `mcp.server.mcpserver.resolve.ElicitationResult`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## UrlElicitationResult

Import as `mcp.server.mcpserver.context.UrlElicitationResult`  ·  defined at `mcp.server.elicitation.UrlElicitationResult`

```python
UrlElicitationResult = AcceptedUrlElicitation | DeclinedElicitation | CancelledElicitation
```

**Inferred type** (`ty`, not declared in the source): ````xml <types.UnionType special-form 'AcceptedUrlElicitation | DeclinedElicitation | CancelledElicitation'> ````

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## _PRIMITIVE_SCHEMA_ADAPTER

`mcp.server.elicitation._PRIMITIVE_SCHEMA_ADAPTER`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_PRIMITIVE_SCHEMA_ADAPTER = TypeAdapter[PrimitiveSchemaDefinition](PrimitiveSchemaDefinition)
```

## AcceptedElicitation

Import as `mcp.server.mcpserver.AcceptedElicitation`  ·  defined at `mcp.server.elicitation.AcceptedElicitation`

```python
class AcceptedElicitation(BaseModel, Generic[ElicitSchemaModelT])
```

**Also exported as** `mcp.server.mcpserver.AcceptedElicitation`, `mcp.server.mcpserver.resolve.AcceptedElicitation`

**Bases** `BaseModel`, `Generic[ElicitSchemaModelT]`

**Declared members (2)**

- `action: Literal['accept'] = 'accept'`  _class-attribute, instance-attribute_
- `data: ElicitSchemaModelT`  _instance-attribute_

Result when user accepts the elicitation.


## AcceptedUrlElicitation

`mcp.server.elicitation.AcceptedUrlElicitation`

```python
class AcceptedUrlElicitation(BaseModel)
```

**Bases** `BaseModel`

**Declared members (1)**

- `action: Literal['accept'] = 'accept'`  _class-attribute, instance-attribute_

Result when user accepts a URL mode elicitation.


## CancelledElicitation

Import as `mcp.server.mcpserver.CancelledElicitation`  ·  defined at `mcp.server.elicitation.CancelledElicitation`

```python
class CancelledElicitation(BaseModel)
```

**Also exported as** `fastmcp.server.elicitation.CancelledElicitation`, `mcp.server.mcpserver.CancelledElicitation`, `mcp.server.mcpserver.resolve.CancelledElicitation`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (1)**

- `action: Literal['cancel'] = 'cancel'`  _class-attribute, instance-attribute_

Result when user cancels the elicitation.


## DeclinedElicitation

Import as `mcp.server.mcpserver.DeclinedElicitation`  ·  defined at `mcp.server.elicitation.DeclinedElicitation`

```python
class DeclinedElicitation(BaseModel)
```

**Also exported as** `fastmcp.server.elicitation.DeclinedElicitation`, `mcp.server.mcpserver.DeclinedElicitation`, `mcp.server.mcpserver.resolve.DeclinedElicitation`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `BaseModel`

**Declared members (1)**

- `action: Literal['decline'] = 'decline'`  _class-attribute, instance-attribute_

Result when user declines the elicitation.


## _ElicitationJsonSchema

`mcp.server.elicitation._ElicitationJsonSchema`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _ElicitationJsonSchema(GenerateJsonSchema)
```

**Bases** `GenerateJsonSchema`

**Declared members (2)**

- `def default_schema(self, schema: core_schema.WithDefaultSchema) -> JsonSchemaValue`
- `def nullable_schema(self, schema: core_schema.NullableSchema) -> JsonSchemaValue`

JSON-Schema generator that flattens `T | None` to `T` and drops `None` defaults.

The spec's `PrimitiveSchemaDefinition` admits no `anyOf` or null type; an
optional field is expressed by leaving it out of `required`, which pydantic
already does for any field with a default.


## _validate_rendered_properties

`mcp.server.elicitation._validate_rendered_properties`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _validate_rendered_properties(json_schema: dict[str, Any]) -> None
```

Reject any `properties` entry the spec's `PrimitiveSchemaDefinition` won't accept.

Catches whatever the renderer let through that isn't spec-valid: bare
`list[str]` (no enum), multi-primitive unions, nested models.


## elicit_url

Import as `mcp.server.mcpserver.context.elicit_url`  ·  defined at `mcp.server.elicitation.elicit_url`

```python
async def elicit_url(session: ServerSession, message: str, url: str, elicitation_id: str, related_request_id: RequestId | None = None) -> UrlElicitationResult
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Elicit information from the user via out-of-band URL navigation (URL mode).

This method directs the user to an external URL where sensitive interactions can
occur without passing data through the MCP client. Use this for:
- Collecting sensitive credentials (API keys, passwords)
- OAuth authorization flows with third-party services
- Payment and subscription flows
- Any interaction where data should not pass through the LLM context

The response indicates whether the user consented to navigate to the URL.
The actual interaction happens out-of-band. When the elicitation completes,
the server should send an ElicitCompleteNotification to notify the client.

Args:
    session: The server session
    message: Human-readable explanation of why the interaction is needed
    url: The URL the user should navigate to
    elicitation_id: Unique identifier for tracking this elicitation
    related_request_id: Optional ID of the request that triggered this elicitation

Returns:
    UrlElicitationResult indicating accept, decline, or cancel


## elicit_with_validation

Import as `mcp.server.mcpserver.context.elicit_with_validation`  ·  defined at `mcp.server.elicitation.elicit_with_validation`

```python
async def elicit_with_validation(session: ServerSession, message: str, schema: type[ElicitSchemaModelT], related_request_id: RequestId | None = None) -> ElicitationResult[ElicitSchemaModelT]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Elicit information from the client/user with schema validation (form mode).

This method can be used to interactively ask for additional information from the
client within a tool's execution. The client might display the message to the
user and collect a response according to the provided schema. If the client
is an agent, it might decide how to handle the elicitation -- either by asking
the user or automatically generating a response.

For sensitive data like credentials or OAuth flows, use elicit_url() instead.

Raises:
    ValueError: If the client accepted the elicitation without supplying
        content, or with content that does not match the requested schema.


## render_elicitation_schema

Import as `mcp.server.mcpserver.resolve.render_elicitation_schema`  ·  defined at `mcp.server.elicitation.render_elicitation_schema`

```python
def render_elicitation_schema(schema: type[BaseModel]) -> dict[str, Any]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Render a model as the spec-valid `requested_schema` for an elicitation.

Raises:
    TypeError: If a field renders as something the spec's
        `PrimitiveSchemaDefinition` does not accept.


