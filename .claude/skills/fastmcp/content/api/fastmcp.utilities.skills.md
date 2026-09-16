# `fastmcp.utilities.skills`

Distribution: `fastmcp`

## SkillFile

`fastmcp.utilities.skills.SkillFile`

```python
class SkillFile
```

**Declared members (3)**

- `hash: str`  _instance-attribute_
- `path: str`  _instance-attribute_
- `size: int`  _instance-attribute_

Information about a file within a skill.


## SkillManifest

`fastmcp.utilities.skills.SkillManifest`

```python
class SkillManifest
```

**Declared members (2)**

- `files: list[SkillFile]`  _instance-attribute_
- `name: str`  _instance-attribute_

Full manifest of a skill including all files.


## SkillSummary

`fastmcp.utilities.skills.SkillSummary`

```python
class SkillSummary
```

**Declared members (3)**

- `description: str`  _instance-attribute_
- `name: str`  _instance-attribute_
- `uri: str`  _instance-attribute_

Summary information about a skill available on a server.


## download_skill

`fastmcp.utilities.skills.download_skill`

```python
async def download_skill(client: Client, skill_name: str, target_dir: str | Path, overwrite: bool = False) -> Path
```

Download a skill and all its files to a local directory.

Creates a subdirectory named after the skill containing all files.

Args:
    client: Connected FastMCP client
    skill_name: Name of the skill to download
    target_dir: Directory where skill folder will be created
    overwrite: If True, overwrite existing skill directory. If False
        (default), raise FileExistsError if directory exists.

Returns:
    Path to the downloaded skill directory

Raises:
    ValueError: If skill cannot be found or downloaded
    FileExistsError: If skill directory exists and overwrite=False

Example:
    ```python
    from fastmcp import Client
    from fastmcp.utilities.skills import download_skill

    async with Client("http://skills-server/mcp") as client:
        skill_path = await download_skill(
            client,
            "pdf-processing",
            "~/.claude/skills"
        )
        print(f"Downloaded to: {skill_path}")
    ```


## get_skill_manifest

`fastmcp.utilities.skills.get_skill_manifest`

```python
async def get_skill_manifest(client: Client, skill_name: str) -> SkillManifest
```

Get the manifest for a specific skill.

Args:
    client: Connected FastMCP client
    skill_name: Name of the skill

Returns:
    SkillManifest with file listing

Raises:
    ValueError: If manifest cannot be read or parsed


## list_skills

`fastmcp.utilities.skills.list_skills`

```python
async def list_skills(client: Client) -> list[SkillSummary]
```

List all available skills from an MCP server.

Discovers skills by finding resources with URIs matching the
`skill://{name}/SKILL.md` pattern.

Args:
    client: Connected FastMCP client

Returns:
    List of SkillSummary objects with name, description, and URI

Example:
    ```python
    from fastmcp import Client
    from fastmcp.utilities.skills import list_skills

    async with Client("http://skills-server/mcp") as client:
        skills = await list_skills(client)
        for skill in skills:
            print(f"{skill.name}: {skill.description}")
    ```


## sync_skills

`fastmcp.utilities.skills.sync_skills`

```python
async def sync_skills(client: Client, target_dir: str | Path, overwrite: bool = False) -> list[Path]
```

Download all available skills from a server.

Args:
    client: Connected FastMCP client
    target_dir: Directory where skill folders will be created
    overwrite: If True, overwrite existing files

Returns:
    List of paths to downloaded skill directories

Example:
    ```python
    from fastmcp import Client
    from fastmcp.utilities.skills import sync_skills

    async with Client("http://skills-server/mcp") as client:
        paths = await sync_skills(client, "~/.claude/skills")
        print(f"Downloaded {len(paths)} skills")
    ```


