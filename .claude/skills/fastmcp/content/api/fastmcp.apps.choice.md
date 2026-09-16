# `fastmcp.apps.choice`

Distribution: `fastmcp`

## Choice

`fastmcp.apps.choice.Choice`

```python
class Choice(FastMCPApp)
```

**Bases** `FastMCPApp`

**Inherited (22)**

- from `fastmcp.apps.app.FastMCPApp`: `add_tool`, `lifespan`, `name`, `run`, `tool`, `ui`
- from `fastmcp.server.providers.base.Provider`: `add_transform`, `disable`, `enable`, `get_app_tool`, `get_prompt`, `get_resource`, `get_resource_template`, `get_tasks`, `get_tool`, `get_tool_by_hash`, `list_prompts`, `list_resource_templates`, `list_resources`, `list_tools`, `transforms`, `wrap_transform`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A Provider that lets the user choose from a set of options.

The LLM calls ``choose`` with a prompt and a list of options.
The user sees a card with one button per option. Clicking a button
sends the selection back into the conversation via ``SendMessage``,
triggering the LLM's next turn.

Example::

    from fastmcp import FastMCP
    from fastmcp.apps.choice import Choice

    mcp = FastMCP("My Server")
    mcp.add_provider(Choice())


