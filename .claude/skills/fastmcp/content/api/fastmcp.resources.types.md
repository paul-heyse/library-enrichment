# `fastmcp.resources.types`

Distribution: `fastmcp`

## logger

`fastmcp.resources.types.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## BinaryResource

Import as `fastmcp.resources.BinaryResource`  ·  defined at `fastmcp.resources.types.BinaryResource`

```python
class BinaryResource(Resource)
```

**Also exported as** `fastmcp.resources.BinaryResource`

**Bases** `Resource`

**Declared members (2)**

- `data: bytes = Field(description='Binary content of the resource')`  _class-attribute, instance-attribute_
- `async def read(self) -> ResourceResult`  _async_
  Read the binary content.

**Inherited (24)**

- from `fastmcp.resources.base.Resource`: `KEY_PREFIX`, `annotations`, `auth`, `convert_result`, `from_function`, `get_span_attributes`, `key`, `mime_type`, `name`, `set_default_mime_type`, `set_default_name`, `to_mcp_resource`, `uri`
- from `fastmcp.utilities.components.FastMCPComponent`: `description`, `disable`, `enable`, `get_meta`, `icons`, `make_key`, `meta`, `tags`, `task_config`, `title`, `version`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A resource that reads from bytes.


## DirectoryResource

Import as `fastmcp.resources.DirectoryResource`  ·  defined at `fastmcp.resources.types.DirectoryResource`

```python
class DirectoryResource(Resource)
```

**Also exported as** `fastmcp.resources.DirectoryResource`

**Bases** `Resource`

**Declared members (7)**

- `async def list_files(self) -> list[Path]`  _async_
  List files in the directory.
- `mime_type: str = Field(default='application/json', description='MIME type of the resource content')`  _class-attribute, instance-attribute_
- `path: Path = Field(description='Path to the directory')`  _class-attribute, instance-attribute_
- `pattern: str | None = Field(default=None, description='Optional glob pattern to filter files')`  _class-attribute, instance-attribute_
- `async def read(self) -> ResourceResult`  _async_
  Read the directory listing.
- `recursive: bool = Field(default=False, description='Whether to list files recursively')`  _class-attribute, instance-attribute_
- `def validate_absolute_path(cls, path: Path) -> Path`  _classmethod_
  Ensure path is absolute.

**Inherited (23)**

- from `fastmcp.resources.base.Resource`: `KEY_PREFIX`, `annotations`, `auth`, `convert_result`, `from_function`, `get_span_attributes`, `key`, `name`, `set_default_mime_type`, `set_default_name`, `to_mcp_resource`, `uri`
- from `fastmcp.utilities.components.FastMCPComponent`: `description`, `disable`, `enable`, `get_meta`, `icons`, `make_key`, `meta`, `tags`, `task_config`, `title`, `version`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A resource that lists files in a directory.


## FileResource

Import as `fastmcp.resources.FileResource`  ·  defined at `fastmcp.resources.types.FileResource`

```python
class FileResource(Resource)
```

**Also exported as** `fastmcp.resources.FileResource`

**Bases** `Resource`

**Declared members (7)**

- `encoding: str | None = Field(default='utf-8', description="Encoding to use when reading text files. Defaults to 'utf-8' for cross-platform compatibility. Set to None to use the system default encoding.")`  _class-attribute, instance-attribute_
- `is_binary: bool = Field(default=False, description='Whether to read the file as binary data')`  _class-attribute, instance-attribute_
- `mime_type: str = Field(default='text/plain', description='MIME type of the resource content')`  _class-attribute, instance-attribute_
- `path: Path = Field(description='Path to the file')`  _class-attribute, instance-attribute_
- `async def read(self) -> ResourceResult`  _async_
  Read the file content.
- `def set_binary_from_mime_type(cls, is_binary: bool, info: ValidationInfo) -> bool`  _classmethod_
  Set is_binary based on mime_type if not explicitly set.
- `def validate_absolute_path(cls, path: Path) -> Path`  _classmethod_
  Ensure path is absolute.

**Inherited (23)**

- from `fastmcp.resources.base.Resource`: `KEY_PREFIX`, `annotations`, `auth`, `convert_result`, `from_function`, `get_span_attributes`, `key`, `name`, `set_default_mime_type`, `set_default_name`, `to_mcp_resource`, `uri`
- from `fastmcp.utilities.components.FastMCPComponent`: `description`, `disable`, `enable`, `get_meta`, `icons`, `make_key`, `meta`, `tags`, `task_config`, `title`, `version`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A resource that reads from a file.

Set is_binary=True to read file as binary data instead of text.


## HttpResource

Import as `fastmcp.resources.HttpResource`  ·  defined at `fastmcp.resources.types.HttpResource`

```python
class HttpResource(Resource)
```

**Also exported as** `fastmcp.resources.HttpResource`

**Bases** `Resource`

**Declared members (3)**

- `mime_type: str = Field(default='application/json', description='MIME type of the resource content')`  _class-attribute, instance-attribute_
- `async def read(self) -> ResourceResult`  _async_
  Read the HTTP content.
- `url: str = Field(description='URL to fetch content from')`  _class-attribute, instance-attribute_

**Inherited (23)**

- from `fastmcp.resources.base.Resource`: `KEY_PREFIX`, `annotations`, `auth`, `convert_result`, `from_function`, `get_span_attributes`, `key`, `name`, `set_default_mime_type`, `set_default_name`, `to_mcp_resource`, `uri`
- from `fastmcp.utilities.components.FastMCPComponent`: `description`, `disable`, `enable`, `get_meta`, `icons`, `make_key`, `meta`, `tags`, `task_config`, `title`, `version`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A resource that reads from an HTTP endpoint.


## TextResource

Import as `fastmcp.resources.TextResource`  ·  defined at `fastmcp.resources.types.TextResource`

```python
class TextResource(Resource)
```

**Also exported as** `fastmcp.resources.TextResource`

**Bases** `Resource`

**Declared members (2)**

- `async def read(self) -> ResourceResult`  _async_
  Read the text content.
- `text: str = Field(description='Text content of the resource')`  _class-attribute, instance-attribute_

**Inherited (24)**

- from `fastmcp.resources.base.Resource`: `KEY_PREFIX`, `annotations`, `auth`, `convert_result`, `from_function`, `get_span_attributes`, `key`, `mime_type`, `name`, `set_default_mime_type`, `set_default_name`, `to_mcp_resource`, `uri`
- from `fastmcp.utilities.components.FastMCPComponent`: `description`, `disable`, `enable`, `get_meta`, `icons`, `make_key`, `meta`, `tags`, `task_config`, `title`, `version`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A resource that reads from a string.


