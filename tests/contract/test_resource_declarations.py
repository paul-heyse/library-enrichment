"""Pure catalog construction; no MCP client, transport, daemon or service state."""

from jsonschema import Draft202012Validator

from enrichment_mcp import presentation
from enrichment_mcp.server import TOOL_NAMES, NativeResourceTemplate, build_server


async def test_native_catalog_constructs_all_tools_and_resources() -> None:
    server = build_server()
    assert {tool.name for tool in await server.list_tools()} == set(TOOL_NAMES)
    resources = {str(resource.uri): resource for resource in await server.list_resources()}
    templates = {
        resource.uri_template: resource for resource in await server.list_resource_templates()
    }
    assert len(resources) == 1
    assert len(templates) == 4
    for binding in presentation.resources():
        if binding["source"]["kind"] == "guidance":
            resource = resources[binding["uri"]]
            assert resource.name == binding["name"]
        else:
            template = templates[binding["uri"]]
            assert isinstance(template, NativeResourceTemplate)
            assert template.rpc_method == binding["rpc"]
            # FastMCP resolves refs and drops unused definitions during construction.
            assert template.parameters["properties"] == binding["parameters"]["properties"]
            assert template.parameters["required"] == binding["parameters"]["required"]
            assert template.parameters["additionalProperties"] is False
            validator = Draft202012Validator(template.parameters)
            parameter = binding["source"]["parameter"]
            assert list(validator.iter_errors({}))
            assert list(validator.iter_errors({parameter: ""}))
            if "pattern" in template.parameters["properties"][parameter]:
                assert list(validator.iter_errors({parameter: "invalid"}))
