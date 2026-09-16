# `fastmcp.server.providers.skills._common`

Distribution: `fastmcp`

## SkillFileInfo

`fastmcp.server.providers.skills._common.SkillFileInfo`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class SkillFileInfo
```

**Declared members (3)**

- `hash: str`  _instance-attribute_
- `path: str`  _instance-attribute_
- `size: int`  _instance-attribute_

Information about a file within a skill.


## SkillInfo

Import as `fastmcp.server.providers.skills.skill_provider.SkillInfo`  ·  defined at `fastmcp.server.providers.skills._common.SkillInfo`

```python
class SkillInfo
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

**Declared members (6)**

- `description: str`  _instance-attribute_
- `files: list[SkillFileInfo] = field(default_factory=list)`  _class-attribute, instance-attribute_
- `frontmatter: dict[str, Any] = field(default_factory=dict)`  _class-attribute, instance-attribute_
- `main_file: str`  _instance-attribute_
- `name: str`  _instance-attribute_
- `path: Path`  _instance-attribute_

Parsed information about a skill.


## compute_file_hash

`fastmcp.server.providers.skills._common.compute_file_hash`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def compute_file_hash(path: Path) -> str
```

Compute SHA256 hash of a file.


## parse_frontmatter

Import as `fastmcp.server.providers.skills.skill_provider.parse_frontmatter`  ·  defined at `fastmcp.server.providers.skills._common.parse_frontmatter`

```python
def parse_frontmatter(content: str) -> tuple[dict[str, Any], str]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Parse YAML frontmatter from markdown content.

Args:
    content: Markdown content potentially starting with ---

Returns:
    Tuple of (frontmatter dict, remaining content)


## scan_skill_files

Import as `fastmcp.server.providers.skills.skill_provider.scan_skill_files`  ·  defined at `fastmcp.server.providers.skills._common.scan_skill_files`

```python
def scan_skill_files(skill_dir: Path) -> list[SkillFileInfo]
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Scan a skill directory for all files.


