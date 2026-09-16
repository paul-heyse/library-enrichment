# Language servers: LSP and TSP

`pyrefly lsp` and `pyrefly tsp` both speak framed JSON-RPC on stdio and both answer
`initialize`. The capabilities below are the servers' own replies, captured during
acquisition — not a reading of the source.

Both advertise these 23 capabilities:

- `callHierarchyProvider`
- `codeActionProvider`
- `codeLensProvider`
- `completionProvider`
- `declarationProvider`
- `definitionProvider`
- `documentHighlightProvider`
- `documentSymbolProvider`
- `foldingRangeProvider`
- `hoverProvider`
- `implementationProvider`
- `inlayHintProvider`
- `notebookDocumentSync`
- `positionEncoding`
- `referencesProvider`
- `renameProvider`
- `selectionRangeProvider`
- `signatureHelpProvider`
- `textDocumentSync`
- `typeDefinitionProvider`
- `typeHierarchyProvider`
- `workspace`
- `workspaceSymbolProvider`

The ones that matter for analysis, and that a weaker server either lacks or answers
unreliably: `referencesProvider`, `callHierarchyProvider`, `typeHierarchyProvider`,
`implementationProvider`, `declarationProvider`, `typeDefinitionProvider` and
`inlayHintProvider`.

TSP additionally advertises: `experimental`.

```json
{
 "typeServerMultiConnection": {
  "supportedTransports": [
   "ipc"
  ]
 }
}
```


## The Type Server Protocol as types

`tsp_types` is indexed here as 80 Rust items, so the protocol's request,
response and notification shapes are readable as declarations rather than inferred from
traffic. Start at `content/api/tsp_types.md`.

LSP answers one position at a time. For whole-project definitions, references and call
edges, use the Glean report instead — see `cross-references.md`.
