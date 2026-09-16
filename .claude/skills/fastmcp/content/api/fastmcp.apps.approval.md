# `fastmcp.apps.approval`

Distribution: `fastmcp`

## Approval

`fastmcp.apps.approval.Approval`

```python
class Approval(FastMCPApp)
```

**Bases** `FastMCPApp`

**Inherited (22)**

- from `fastmcp.apps.app.FastMCPApp`: `add_tool`, `lifespan`, `name`, `run`, `tool`, `ui`
- from `fastmcp.server.providers.base.Provider`: `add_transform`, `disable`, `enable`, `get_app_tool`, `get_prompt`, `get_resource`, `get_resource_template`, `get_tasks`, `get_tool`, `get_tool_by_hash`, `list_prompts`, `list_resource_templates`, `list_resources`, `list_tools`, `transforms`, `wrap_transform`

Signatures for inherited members are on the base's own page, and every one of them is a row in `content/index/members.tsv`.

A Provider that adds human-in-the-loop approval to a server.

The LLM calls the ``request_approval`` tool with a summary and
optional details. The user sees an approval card with Approve and
Reject buttons. Clicking either sends a message back into the
conversation (via ``SendMessage``), triggering the LLM's next turn.

The message appears as if the user sent it, so the LLM sees
something like ``'"Deploy v3.2 to production" is APPROVED'``.

Example::

    from fastmcp import FastMCP
    from fastmcp.apps.approval import Approval

    mcp = FastMCP("My Server")
    mcp.add_provider(Approval())

Customized::

    Approval(
        title="Deploy Gate",
        approve_text="Ship it",
        approve_variant="default",
        reject_text="Abort",
        reject_variant="destructive",
    )


