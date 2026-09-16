# `mcp.server.mcpserver.resources.types`

Distribution: `mcp`

## _TEXTUAL_APPLICATION_TYPES

`mcp.server.mcpserver.resources.types._TEXTUAL_APPLICATION_TYPES`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_TEXTUAL_APPLICATION_TYPES = frozenset({'application/json', 'application/xml'})
```

## BinaryResource

Import as `mcp.server.mcpserver.resources.BinaryResource`  ·  defined at `mcp.server.mcpserver.resources.types.BinaryResource`

```python
class BinaryResource(Resource)
```

**Also exported as** `mcp.server.mcpserver.resources.BinaryResource`

**Bases** `Resource`

**Declared members (2)**

- `data: bytes = Field(description='Binary content of the resource')`  _class-attribute, instance-attribute_
- `async def read(self) -> bytes`  _async_
  Read the binary content.

**Inherited (9)**

- from `mcp.server.mcpserver.resources.base.Resource`: `annotations`, `description`, `icons`, `meta`, `mime_type`, `name`, `set_default_name`, `title`, `uri`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A resource that reads from bytes.


## DirectoryResource

Import as `mcp.server.mcpserver.resources.DirectoryResource`  ·  defined at `mcp.server.mcpserver.resources.types.DirectoryResource`

```python
class DirectoryResource(Resource)
```

**Also exported as** `mcp.server.mcpserver.resources.DirectoryResource`

**Bases** `Resource`

**Declared members (7)**

- `def list_files(self) -> list[Path]`
  List files in the directory.
- `mime_type: str = Field(default='application/json', description='MIME type of the resource content')`  _class-attribute, instance-attribute_
- `path: Path = Field(description='Path to the directory')`  _class-attribute, instance-attribute_
- `pattern: str | None = Field(default=None, description='Optional glob pattern to filter files')`  _class-attribute, instance-attribute_
- `async def read(self) -> str`  _async_
  Read the directory listing.
- `recursive: bool = Field(default=False, description='Whether to list files recursively')`  _class-attribute, instance-attribute_
- `def validate_absolute_path(cls, path: Path) -> Path`  _classmethod_
  Ensure path is absolute.

**Inherited (8)**

- from `mcp.server.mcpserver.resources.base.Resource`: `annotations`, `description`, `icons`, `meta`, `name`, `set_default_name`, `title`, `uri`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A resource that lists files in a directory.


## FileResource

Import as `mcp.server.mcpserver.resources.FileResource`  ·  defined at `mcp.server.mcpserver.resources.types.FileResource`

```python
class FileResource(Resource)
```

**Also exported as** `mcp.server.mcpserver.resources.FileResource`

**Bases** `Resource`

**Declared members (5)**

- `encoding: str | None = Field(default_factory=lambda data: _default_file_encoding(data['mime_type']), description='Text encoding used to decode the file, or None to serve its bytes as a blob')`  _class-attribute, instance-attribute_
- `path: Path = Field(description='Path to the file')`  _class-attribute, instance-attribute_
- `async def read(self) -> str | bytes`  _async_
  Read the file content.
- `def validate_absolute_path(cls, path: Path) -> Path`  _classmethod_
  Ensure path is absolute.
- `def validate_text_encoding(cls, encoding: str | None) -> str | None`  _classmethod_
  Ensure the encoding names a usable text codec, so a mistake fails at construction not at read.

**Inherited (9)**

- from `mcp.server.mcpserver.resources.base.Resource`: `annotations`, `description`, `icons`, `meta`, `mime_type`, `name`, `set_default_name`, `title`, `uri`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A resource that reads from a file.

The file is decoded with `encoding` and served as text, or read as bytes and
served as a base64 blob when `encoding` is None. When `encoding` is omitted it
defaults to the `charset` declared in `mime_type`, else `"utf-8-sig"` for
textual mime types (`text/*`, JSON, XML) and None for everything else; pass
it explicitly to override either way.


## FunctionResource

Import as `mcp.server.mcpserver.resources.FunctionResource`  ·  defined at `mcp.server.mcpserver.resources.types.FunctionResource`

```python
class FunctionResource(Resource)
```

**Also exported as** `mcp.server.mcpserver.resources.FunctionResource`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Resource`

**Declared members (3)**

- `fn: Callable[[], Any] = Field(exclude=True)`  _class-attribute, instance-attribute_
- `def from_function(cls, fn: Callable[..., Any], uri: str, name: str | None = None, title: str | None = None, description: str | None = None, mime_type: str | None = None, icons: list[Icon] | None = None, annotations: Annotations | None = None, meta: dict[str, Any] | None = None) -> FunctionResource`  _classmethod_
  Create a FunctionResource from a function.
- `async def read(self) -> str | bytes`  _async_
  Read the resource by calling the wrapped function.

**Inherited (9)**

- from `mcp.server.mcpserver.resources.base.Resource`: `annotations`, `description`, `icons`, `meta`, `mime_type`, `name`, `set_default_name`, `title`, `uri`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A resource that defers data loading by wrapping a function.

The function is only called when the resource is read, allowing for lazy loading
of potentially expensive data. This is particularly useful when listing resources,
as the function won't be called until the resource is actually accessed.

The function can return:
- str for text content (default)
- bytes for binary content
- other types will be converted to JSON


## HttpResource

Import as `mcp.server.mcpserver.resources.HttpResource`  ·  defined at `mcp.server.mcpserver.resources.types.HttpResource`

```python
class HttpResource(Resource)
```

**Also exported as** `mcp.server.mcpserver.resources.HttpResource`

**Bases** `Resource`

**Declared members (3)**

- `mime_type: str = Field(default='application/json', description='MIME type of the resource content')`  _class-attribute, instance-attribute_
- `async def read(self) -> str | bytes`  _async_
  Read the HTTP content.
- `url: str = Field(description='URL to fetch content from')`  _class-attribute, instance-attribute_

**Inherited (8)**

- from `mcp.server.mcpserver.resources.base.Resource`: `annotations`, `description`, `icons`, `meta`, `name`, `set_default_name`, `title`, `uri`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A resource that reads from an HTTP endpoint.


## TextResource

Import as `mcp.server.mcpserver.resources.TextResource`  ·  defined at `mcp.server.mcpserver.resources.types.TextResource`

```python
class TextResource(Resource)
```

**Also exported as** `mcp.server.mcpserver.resources.TextResource`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Bases** `Resource`

**Declared members (2)**

- `async def read(self) -> str`  _async_
  Read the text content.
- `text: str = Field(description='Text content of the resource')`  _class-attribute, instance-attribute_

**Inherited (9)**

- from `mcp.server.mcpserver.resources.base.Resource`: `annotations`, `description`, `icons`, `meta`, `mime_type`, `name`, `set_default_name`, `title`, `uri`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A resource that reads from a string.


## _default_file_encoding

`mcp.server.mcpserver.resources.types._default_file_encoding`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _default_file_encoding(mime_type: str) -> str | None
```

The encoding a file of this mime type is decoded with by default.

A declared `charset=` parameter wins. Otherwise textual types (`text/*`, JSON,
XML) are `utf-8-sig` — UTF-8 that also tolerates a byte-order mark — and
everything else is bytes (None).


