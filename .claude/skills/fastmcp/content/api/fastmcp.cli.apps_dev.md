# `fastmcp.cli.apps_dev`

Distribution: `fastmcp`

## _EXT_APPS_VERSION

`fastmcp.cli.apps_dev._EXT_APPS_VERSION`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_EXT_APPS_VERSION = '1.0.1'
```

## _HOST_HTML_TEMPLATE

`fastmcp.cli.apps_dev._HOST_HTML_TEMPLATE`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_HOST_HTML_TEMPLATE = '<!doctype html>\n<html>\n<head>\n  <meta charset="UTF-8">\n  <title>FastMCP Dev — {tool_name}</title>\n{import_map_tag}\n  <style>\n    html, body {{ margin: 0; padding: 0; width: 100%; height: 100vh; overflow: hidden; }}\n    #app-frame {{ width: 100%; height: 100%; border: none; display: none; }}\n    #status {{\n      display: flex; align-items: center; justify-content: center; height: 100vh;\n      font-family: system-ui, sans-serif; color: #666; font-size: 1rem;\n    }}\n  </style>\n</head>\n<body>\n  <div id="status">Launching {tool_name}…</div>\n  <iframe id="app-frame"></iframe>\n  <script type="module">\n    import {{ AppBridge, PostMessageTransport, getToolUiResourceUri }}\n      from "/js/app-bridge.js";\n    import {{ Client }}\n      from "https://esm.sh/@modelcontextprotocol/sdk@{mcp_sdk_version}/client/index.js";\n    import {{ StreamableHTTPClientTransport }}\n      from "https://esm.sh/@modelcontextprotocol/sdk@{mcp_sdk_version}/client/streamableHttp.js";\n\n    const toolName = {tool_name_json};\n    const toolArgs = {tool_args_json};\n    const status = document.getElementById("status");\n    const iframe  = document.getElementById("app-frame");\n\n    async function main() {{\n      // Connect to the proxied MCP server (same-origin, no CORS needed)\n      const client = new Client({{ name: "fastmcp-dev", version: "1.0.0" }});\n      await client.connect(\n        new StreamableHTTPClientTransport(new URL("/mcp", window.location.origin))\n      );\n\n      // Find the tool and its UI resource URI\n      const {{ tools }} = await client.listTools();\n      const tool = tools.find(t => t.name === toolName);\n      if (!tool) throw new Error("Tool not found: " + toolName);\n\n      const uiUri = getToolUiResourceUri(tool);\n      if (!uiUri) throw new Error("Tool has no UI resource: " + toolName);\n\n      // The Prefab renderer calls earlyBridge.connect() at module-load time\n      // (synchronously, before React mounts) so it sends its ui/initialize\n      // request very early — potentially before the iframe\'s load event fires.\n      // Fix: create the AppBridge and call bridge.connect() BEFORE loading the\n      // iframe so our window.addEventListener is registered first.  We pass\n      // null as the PostMessageTransport source so early messages from the\n      // not-yet-known renderer window are not filtered out.  After the iframe\n      // loads we update transport.eventTarget / .eventSource to the real\n      // renderer window; the load-event microtask always runs before the\n      // message macrotask, so the response reaches the correct window.\n      const serverCaps = client.getServerCapabilities();\n      const transport = new PostMessageTransport(iframe.contentWindow, null);\n      const bridge = new AppBridge(\n        client,\n        {{ name: "fastmcp-dev", version: "1.0.0" }},\n        {{\n          openLinks: {{}},\n          serverTools: serverCaps?.tools,\n          serverResources: serverCaps?.resources,\n        }},\n        {{\n          hostContext: {{\n            theme: window.matchMedia("(prefers-color-scheme: dark)").matches\n              ? "dark" : "light",\n            platform: "web",\n            containerDimensions: {{ maxHeight: 8000 }},\n            displayMode: "inline",\n            availableDisplayModes: ["inline", "fullscreen"],\n          }},\n        }},\n      );\n\n      bridge.onopenlink = async ({{ url }}) => {{\n        window.open(url, "_blank", "noopener,noreferrer");\n        return {{}};\n      }};\n      bridge.onmessage = async () => ({{}});\n\n      // When the View initializes: send input args, call the tool, send result\n      bridge.oninitialized = async () => {{\n        await bridge.sendToolInput({{ arguments: toolArgs }});\n        const result = await client.callTool({{ name: toolName, arguments: toolArgs }});\n        await bridge.sendToolResult(result);\n        status.style.display = "none";\n        iframe.style.display = "block";\n        // Prevent horizontal scrollbar when vertical scrollbar appears\n        try {{ iframe.contentDocument.documentElement.style.overflowX = "hidden"; }} catch(e) {{}}\n      }};\n\n      // Start listening before the iframe loads\n      await bridge.connect(transport);\n\n      // Now load the renderer HTML via the server-side proxy\n      const frameUrl = "/ui-resource?uri=" + encodeURIComponent(uiUri);\n      const loaded = new Promise(r => {{ iframe.addEventListener("load", r, {{ once: true }}); }});\n      iframe.src = frameUrl;\n      await loaded;\n\n      // Update transport to the real renderer window.  This microtask runs\n      // before the ui/initialize message macrotask, ensuring the response\n      // is dispatched to the correct window.\n      transport.eventTarget = iframe.contentWindow;\n      transport.eventSource = iframe.contentWindow;\n    }}\n\n    main().catch(err => {{\n      status.textContent = "Error: " + err.message;\n      console.error(err);\n    }});\n  </script>\n</body>\n</html>\n'
```

## _HOST_SHELL

`fastmcp.cli.apps_dev._HOST_SHELL`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_HOST_SHELL = '<!doctype html>\n<html>\n<head>\n  <meta charset="UTF-8">\n  <title>{title}</title>\n{import_map_tag}\n  <style>\n    html, body {{ margin: 0; padding: 0; width: 100%; height: 100vh; overflow: hidden; }}\n    #app-frame {{ width: 100%; height: 100%; border: none; display: none; }}\n    #status {{\n      display: flex; align-items: center; justify-content: center; height: 100vh;\n      font-family: system-ui, sans-serif; color: #666; font-size: 1rem;\n    }}\n  </style>\n</head>\n<body>\n  <div id="status" style="display:{status_display}">{status_text}</div>\n  <iframe id="app-frame" style="display:{frame_display}"></iframe>\n  <script type="module">\n    import {{ AppBridge, PostMessageTransport }}\n      from "/js/app-bridge.js";\n    import {{ Client }}\n      from "https://esm.sh/@modelcontextprotocol/sdk@{mcp_sdk_version}/client/index.js";\n    import {{ StreamableHTTPClientTransport }}\n      from "https://esm.sh/@modelcontextprotocol/sdk@{mcp_sdk_version}/client/streamableHttp.js";\n\n    const status = document.getElementById("status");\n    const iframe  = document.getElementById("app-frame");\n\n    async function main() {{\n      const client = new Client({{ name: "fastmcp-dev", version: "1.0.0" }});\n      await client.connect(\n        new StreamableHTTPClientTransport(new URL("/mcp", window.location.origin))\n      );\n      const serverCaps = client.getServerCapabilities();\n\n      // Set iframe src after adding load listener to avoid race condition\n      const loaded = new Promise(r => iframe.addEventListener("load", r, {{ once: true }}));\n      iframe.src = {iframe_src_json};\n      await loaded;\n\n      const transport = new PostMessageTransport(\n        iframe.contentWindow,\n        iframe.contentWindow,\n      );\n      const bridge = new AppBridge(\n        client,\n        {{ name: "fastmcp-dev", version: "1.0.0" }},\n        {{\n          openLinks: {{}},\n          serverTools: serverCaps?.tools,\n          serverResources: serverCaps?.resources,\n        }},\n        {{\n          hostContext: {{\n            theme: window.matchMedia("(prefers-color-scheme: dark)").matches\n              ? "dark" : "light",\n            platform: "web",\n            containerDimensions: {{ maxHeight: 8000 }},\n            displayMode: "inline",\n            availableDisplayModes: ["inline", "fullscreen"],\n          }},\n        }},\n      );\n\n      bridge.onmessage = async () => ({{}});\n      {on_open_link}\n      {on_initialized}\n\n      await bridge.connect(transport);\n    }}\n\n    main().catch(err => {{\n      console.error(err);\n      if (status) {{\n        status.style.display = "flex";\n        status.textContent = "Error: " + err.message;\n      }}\n    }});\n  </script>\n</body>\n</html>\n'
```

## _LOG_PANEL_HTML

`fastmcp.cli.apps_dev._LOG_PANEL_HTML`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_LOG_PANEL_HTML = '<style>\n  #mcp-log-panel {\n    position: fixed; top: 0; left: 0; bottom: 0; width: 360px;\n    z-index: 10000;\n    font-family: ui-monospace, SFMono-Regular, "SF Mono", Menlo, monospace;\n    font-size: 12px; background: #1e1e2e; color: #cdd6f4;\n    border-right: 1px solid #45475a;\n    display: flex; flex-direction: column;\n  }\n  #mcp-log-panel.hidden { display: none; }\n  #app-frame {\n    width: 100% !important; height: 100% !important;\n    margin-left: 0 !important;\n  }\n  #mcp-log-resize {\n    position: absolute; right: -3px; top: 0; bottom: 0; width: 6px;\n    cursor: col-resize; z-index: 1;\n  }\n  #mcp-log-resize:hover, #mcp-log-resize.active { background: #585b70; }\n  #mcp-log-header {\n    display: flex; justify-content: space-between; align-items: center;\n    padding: 10px 12px; background: #181825;\n    border-bottom: 1px solid #45475a; flex-shrink: 0;\n  }\n  #mcp-log-brand {\n    display: flex; align-items: center; gap: 8px;\n  }\n  #mcp-log-brand svg { flex-shrink: 0; }\n  #mcp-log-brand-text {\n    font-weight: 700; font-size: 13px; color: #cdd6f4;\n    letter-spacing: -0.3px;\n  }\n  #mcp-log-count-badge {\n    font-size: 11px; color: #6c7086; font-weight: 400;\n  }\n  #mcp-log-actions { display: flex; gap: 6px; }\n  #mcp-log-actions button {\n    background: #313244; color: #cdd6f4; border: 1px solid #45475a;\n    padding: 2px 8px; border-radius: 3px; cursor: pointer;\n    font-size: 11px; font-family: inherit;\n  }\n  #mcp-log-actions button:hover { background: #45475a; }\n  #mcp-log-entries { flex: 1; overflow-y: auto; }\n  .log-entry {\n    padding: 6px 12px; border-bottom: 1px solid #232334; cursor: pointer;\n  }\n  .log-entry:hover { background: #313244; }\n  .log-entry.error { background: rgba(243, 139, 168, 0.08); }\n  .log-entry.error:hover { background: rgba(243, 139, 168, 0.14); }\n  .log-entry.error .log-method { color: #f38ba8; }\n  .log-primary {\n    display: flex; justify-content: space-between;\n    align-items: baseline; gap: 8px;\n  }\n  .log-left {\n    display: flex; gap: 6px; align-items: baseline; min-width: 0;\n  }\n  .log-dir { flex-shrink: 0; }\n  .log-dir.request { color: #89b4fa; }\n  .log-dir.response { color: #a6e3a1; }\n  .log-dir.error { color: #f38ba8; }\n  .log-dir.bridge { color: #cba6f7; }\n  .log-dir.notification { color: #fab387; }\n  .log-method {\n    color: #f9e2af; font-weight: 600;\n    white-space: nowrap; overflow: hidden; text-overflow: ellipsis;\n  }\n  .log-meta { color: #6c7086; font-size: 11px; white-space: nowrap; flex-shrink: 0; }\n  .log-subtitle {\n    color: #a6adc8; font-size: 11px; padding-left: 22px;\n    overflow: hidden; text-overflow: ellipsis; white-space: nowrap;\n    margin-top: 1px;\n  }\n  .log-detail {\n    display: none; padding: 8px 12px 4px 22px; background: #11111b;\n    white-space: pre-wrap; word-break: break-all;\n    color: #bac2de; font-size: 11px; line-height: 1.4;\n    margin-top: 4px; border-radius: 4px;\n  }\n  .log-entry.expanded .log-detail { display: block; }\n  @keyframes log-flash {\n    from { background: rgba(137, 180, 250, 0.22); }\n    to { background: transparent; }\n  }\n  @keyframes log-flash-error {\n    from { background: rgba(243, 139, 168, 0.25); }\n    to { background: rgba(243, 139, 168, 0.08); }\n  }\n  .log-entry.new { animation: log-flash 2s ease-out; }\n  .log-entry.error.new { animation: log-flash-error 2s ease-out; }\n  .log-copy {\n    opacity: 0; transition: opacity 0.15s;\n    background: #313244; color: #a6adc8; border: 1px solid #45475a;\n    padding: 1px 6px; border-radius: 3px; cursor: pointer;\n    font-size: 10px; font-family: inherit; flex-shrink: 0;\n  }\n  .log-entry:hover .log-copy { opacity: 1; }\n  .log-copy:hover { background: #45475a; color: #cdd6f4; }\n  #mcp-log-open {\n    position: fixed; bottom: 12px; left: 12px; z-index: 10000;\n    background: #181825; color: #cdd6f4; border: 1px solid #45475a;\n    padding: 6px 12px; border-radius: 6px; cursor: pointer;\n    font-family: ui-monospace, SFMono-Regular, "SF Mono", Menlo, monospace;\n    font-size: 11px; display: block;\n  }\n  #mcp-log-open:hover { background: #313244; }\n  #mcp-log-filters {\n    display: flex; gap: 8px; align-items: center;\n    padding: 6px 12px; border-bottom: 1px solid #45475a; flex-shrink: 0;\n  }\n  .log-seg {\n    display: inline-flex; border: 1px solid #45475a; border-radius: 6px;\n    overflow: hidden;\n  }\n  .log-seg button {\n    background: transparent; color: #6c7086; border: none;\n    border-right: 1px solid #45475a; padding: 3px 10px; cursor: pointer;\n    font-size: 10px; font-family: inherit; transition: all 0.15s;\n  }\n  .log-seg button:last-child { border-right: none; }\n  .log-seg button:hover { background: rgba(205, 214, 244, 0.06); }\n  .log-seg button.active[data-filter="tools"] { background: rgba(137, 180, 250, 0.15); color: #89b4fa; }\n  .log-seg button.active[data-filter="notifications"] { background: rgba(250, 179, 135, 0.15); color: #fab387; }\n  .log-seg button.active[data-filter="bridge"] { background: rgba(203, 166, 247, 0.15); color: #cba6f7; }\n  .log-seg button.active[data-filter="errors"] { background: rgba(243, 139, 168, 0.15); color: #f38ba8; }\n  #mcp-log-filters-label {\n    font-size: 9px; color: #6c7086; text-transform: uppercase;\n    letter-spacing: 0.5px; font-weight: 600;\n  }\n  #mcp-log-level-select {\n    background: #313244; color: #cdd6f4; border: 1px solid #45475a;\n    border-radius: 6px; padding: 3px 8px; cursor: pointer;\n    font-size: 10px; font-family: inherit;\n  }\n  #mcp-log-level-select option { background: #1e1e2e; }\n  .log-level {\n    font-size: 9px; padding: 0 5px; border-radius: 3px;\n    font-weight: 600; text-transform: uppercase; letter-spacing: 0.3px;\n    flex-shrink: 0; line-height: 16px;\n  }\n  .log-level-debug { background: #313244; color: #6c7086; }\n  .log-level-info { background: rgba(137, 180, 250, 0.15); color: #89b4fa; }\n  .log-level-warning { background: rgba(249, 226, 175, 0.15); color: #f9e2af; }\n  .log-level-error { background: rgba(243, 139, 168, 0.15); color: #f38ba8; }\n  .log-level-notice { background: rgba(148, 226, 213, 0.15); color: #94e2d5; }\n  .log-level-critical { background: rgba(243, 139, 168, 0.2); color: #f38ba8; }\n  .log-level-alert { background: rgba(243, 139, 168, 0.25); color: #f38ba8; }\n  .log-level-emergency { background: rgba(243, 139, 168, 0.3); color: #f38ba8; }\n</style>\n<div id="mcp-log-panel" class="hidden">\n  <div id="mcp-log-resize"></div>\n  <div id="mcp-log-header">\n    <div id="mcp-log-brand">\n      <svg width="20" height="20" viewBox="0 0 196 196" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M145.747 44.611L145.355 44.3877L144.96 44.611L86.0283 78.5276V171.267L86.4014 171.499L99.6674 179.667V86.3859L159 52.2379L145.747 44.611Z" fill="#cdd6f4"/><path d="M121.616 30.2714L121.224 30.0454L120.832 30.2714L61.8975 64.188V156.928L62.2732 157.156L75.5393 165.325V72.0463L134.869 37.8983L121.616 30.2714Z" fill="#cdd6f4"/><path d="M97.4894 16.3818L97.0973 16.1558L96.7025 16.3818L37.7705 50.3038V142.066L51.4096 150.463V58.1567L110.742 24.0086L97.4894 16.3818Z" fill="#cdd6f4"/><path d="M131.23 113.671L124.979 117.266L124.584 117.494V117.5L116.796 121.987L110.547 125.581L110.152 125.807V141.51L144.564 121.709V121.698L158.999 113.394V97.6851L139.277 109.034L131.23 113.671Z" fill="#cdd6f4"/></svg>\n      <span id="mcp-log-brand-text">FastMCP Apps</span>\n      <span id="mcp-log-count-badge">· <span id="mcp-log-count">0</span></span>\n    </div>\n    <div id="mcp-log-actions">\n      <button id="mcp-log-reset" onclick="window.location.href=\'/\'">&#8592; Back</button>\n      <script>if (window.location.pathname === "/") document.getElementById("mcp-log-reset").style.display = "none";</script>\n      <button id="mcp-log-clear">Clear</button>\n      <button id="mcp-log-close">×</button>\n    </div>\n  </div>\n  <div id="mcp-log-filters">\n    <span id="mcp-log-filters-label">Show</span>\n    <div class="log-seg">\n      <button class="active" data-filter="tools">Tools</button>\n      <button class="active" data-filter="notifications">Logs</button>\n      <button class="active" data-filter="bridge">Host</button>\n      <button class="active" data-filter="errors">Errors</button>\n    </div>\n    <select id="mcp-log-level-select">\n      <option value="debug">Debug+</option>\n      <option value="info">Info+</option>\n      <option value="warning">Warn+</option>\n      <option value="error">Error+</option>\n      <option value="critical">Critical+</option>\n    </select>\n  </div>\n  <div id="mcp-log-entries"></div>\n</div>\n<button id="mcp-log-open">MCP Log</button>\n<script>\n(function() {\n  var lastId = 0, totalCount = 0, panelWidth = 360;\n  var panel = document.getElementById("mcp-log-panel");\n  var entries = document.getElementById("mcp-log-entries");\n  var countEl = document.getElementById("mcp-log-count");\n  var openBtn = document.getElementById("mcp-log-open");\n  var resizeHandle = document.getElementById("mcp-log-resize");\n  var allFilterKeys = ["tools", "notifications", "bridge", "errors"];\n\n  function syncURL() {\n    var params = new URLSearchParams(window.location.search);\n    if (!panel.classList.contains("hidden")) {\n      params.set("log", "open");\n    } else {\n      params.delete("log");\n    }\n    var on = [];\n    for (var i = 0; i < allFilterKeys.length; i++) {\n      if (activeFilters[allFilterKeys[i]]) on.push(allFilterKeys[i]);\n    }\n    if (on.length === allFilterKeys.length) {\n      params.delete("filters");\n    } else {\n      params.set("filters", on.join(","));\n    }\n    if (minLevel === 0) {\n      params.delete("level");\n    } else {\n      params.set("level", levelOrder[minLevel]);\n    }\n    var qs = params.toString();\n    var url = window.location.pathname + (qs ? "?" + qs : "");\n    history.replaceState(null, "", url);\n  }\n\n  function setFrameLayout(w) {\n    var frame = document.getElementById("app-frame");\n    if (!frame) return;\n    frame.style.setProperty("width", w, "important");\n    frame.style.setProperty("margin-left", w === "100%" ? "0" : panelWidth + "px", "important");\n  }\n\n  document.getElementById("mcp-log-close").addEventListener("click", function() {\n    panel.classList.add("hidden");\n    openBtn.style.display = "block";\n    setFrameLayout("100%");\n    syncURL();\n  });\n\n  openBtn.addEventListener("click", function() {\n    panel.classList.remove("hidden");\n    openBtn.style.display = "none";\n    setFrameLayout("calc(100% - " + panelWidth + "px)");\n    entries.scrollTop = entries.scrollHeight;\n    syncURL();\n  });\n\n  resizeHandle.addEventListener("mousedown", function(e) {\n    e.preventDefault();\n    resizeHandle.classList.add("active");\n    var frame = document.getElementById("app-frame");\n    if (frame) frame.style.pointerEvents = "none";\n    document.addEventListener("mousemove", onResize);\n    document.addEventListener("mouseup", stopResize);\n  });\n\n  function onResize(e) {\n    var w = Math.max(200, Math.min(e.clientX, window.innerWidth * 0.8));\n    panelWidth = w;\n    panel.style.width = w + "px";\n    setFrameLayout("calc(100% - " + w + "px)");\n  }\n\n  function stopResize() {\n    resizeHandle.classList.remove("active");\n    var frame = document.getElementById("app-frame");\n    if (frame) frame.style.pointerEvents = "";\n    document.removeEventListener("mousemove", onResize);\n    document.removeEventListener("mouseup", stopResize);\n  }\n\n  document.getElementById("mcp-log-clear").addEventListener("click", function() {\n    entries.innerHTML = "";\n    totalCount = 0;\n    countEl.textContent = "0";\n    fetch("/api/logs/clear", { method: "POST" });\n  });\n\n  var activeFilters = {tools: true, notifications: true, bridge: true, errors: true};\n  var levelOrder = ["debug", "info", "notice", "warning", "error", "critical", "alert", "emergency"];\n  var minLevel = 0;\n\n  // Restore state from URL params\n  (function restoreURL() {\n    var params = new URLSearchParams(window.location.search);\n    if (params.get("log") === "open") {\n      panel.classList.remove("hidden");\n      openBtn.style.display = "none";\n      setFrameLayout("calc(100% - " + panelWidth + "px)");\n    }\n    var fp = params.get("filters");\n    if (fp !== null) {\n      var on = fp ? fp.split(",") : [];\n      for (var i = 0; i < allFilterKeys.length; i++) {\n        var k = allFilterKeys[i];\n        activeFilters[k] = on.indexOf(k) !== -1;\n        var btn = document.querySelector("[data-filter=\'" + k + "\']");\n        if (btn) btn.classList.toggle("active", activeFilters[k]);\n      }\n    }\n    var lp = params.get("level");\n    if (lp) {\n      var idx = levelOrder.indexOf(lp);\n      if (idx >= 0) {\n        minLevel = idx;\n        document.getElementById("mcp-log-level-select").value = lp;\n      }\n    }\n  })();\n\n  document.getElementById("mcp-log-filters").addEventListener("click", function(e) {\n    var btn = e.target.closest("[data-filter]");\n    if (!btn) return;\n    var f = btn.dataset.filter;\n    activeFilters[f] = !activeFilters[f];\n    btn.classList.toggle("active", activeFilters[f]);\n    applyFilters();\n    syncURL();\n  });\n\n  document.getElementById("mcp-log-level-select").addEventListener("change", function(e) {\n    minLevel = levelOrder.indexOf(e.target.value);\n    applyFilters();\n    syncURL();\n  });\n\n  function shouldShow(el) {\n    var cat = el.dataset.category || "";\n    if (activeFilters[cat] === false) return false;\n    var lv = el.dataset.level;\n    if (lv && levelOrder.indexOf(lv) < minLevel) return false;\n    return true;\n  }\n\n  function applyFilters() {\n    var items = entries.querySelectorAll(".log-entry");\n    for (var i = 0; i < items.length; i++) {\n      items[i].style.display = shouldShow(items[i]) ? "" : "none";\n    }\n  }\n\n  function summarize(entry) {\n    var b = entry.body;\n    if (!b) return "";\n    if (entry.direction === "request" || entry.direction === "notification") {\n      if (b.method === "tools/call" && b.params) return b.params.name || "";\n      if (b.method === "resources/read" && b.params) return b.params.uri || "";\n      if (b.method === "notifications/message" && b.params) {\n        var d = b.params.data;\n        if (d && typeof d === "object") return d.msg || d.message || JSON.stringify(d);\n        return d || b.params.level || "";\n      }\n      return "";\n    }\n    if (b.error) return "error: " + (b.error.message || JSON.stringify(b.error));\n    if (b.result && typeof b.result === "object") {\n      if (Array.isArray(b.result.tools)) return b.result.tools.length + " tools";\n      if (Array.isArray(b.result.resources)) return b.result.resources.length + " resources";\n      if (Array.isArray(b.result.prompts)) return b.result.prompts.length + " prompts";\n      if (b.result.content) {\n        var first = b.result.content[0];\n        if (first && first.text) {\n          return first.text.length > 60 ? first.text.slice(0, 60) + "…" : first.text;\n        }\n        return b.result.content.length + " content item(s)";\n      }\n    }\n    return "";\n  }\n\n  function formatTime(ts) {\n    var d = new Date(ts * 1000);\n    return String(d.getHours()).padStart(2, "0") + ":"\n      + String(d.getMinutes()).padStart(2, "0") + ":"\n      + String(d.getSeconds()).padStart(2, "0");\n  }\n\n  function renderEntry(entry) {\n    var div = document.createElement("div");\n    var isError = entry.direction === "response" && entry.body\n      && (entry.body.error || (entry.body.result && entry.body.result.isError));\n    div.className = "log-entry" + (isError ? " error" : "");\n    var dirClass = isError ? "error" : entry.direction;\n    var arrows = {request: "→", response: "←", bridge: "↑", notification: "↓"};\n\n    // Categorize for filtering\n    if (isError) div.dataset.category = "errors";\n    else if (entry.direction === "bridge") div.dataset.category = "bridge";\n    else if (entry.direction === "notification") div.dataset.category = "notifications";\n    else div.dataset.category = "tools";\n\n    var primary = document.createElement("div");\n    primary.className = "log-primary";\n\n    var left = document.createElement("div");\n    left.className = "log-left";\n\n    var dirEl = document.createElement("span");\n    dirEl.className = "log-dir " + dirClass;\n    dirEl.textContent = arrows[entry.direction] || "←";\n\n    var methodEl = document.createElement("span");\n    methodEl.className = "log-method";\n    methodEl.textContent = entry.method || "";\n\n    left.appendChild(dirEl);\n    left.appendChild(methodEl);\n\n    // Log level badge for notifications\n    if (entry.direction === "notification" && entry.body && entry.body.params) {\n      var level = (entry.body.params.level || "").toLowerCase();\n      if (level) {\n        div.dataset.level = level;\n        var lvl = document.createElement("span");\n        lvl.className = "log-level log-level-" + level;\n        lvl.textContent = level;\n        left.appendChild(lvl);\n      }\n    }\n\n    var metaEl = document.createElement("span");\n    metaEl.className = "log-meta";\n    metaEl.textContent = entry.duration_ms != null\n      ? entry.duration_ms + "ms"\n      : formatTime(entry.timestamp);\n\n    var copyBtn = document.createElement("button");\n    copyBtn.className = "log-copy";\n    copyBtn.textContent = "Copy";\n    copyBtn.addEventListener("click", function(e) {\n      e.stopPropagation();\n      navigator.clipboard.writeText(JSON.stringify(entry.body, null, 2));\n      copyBtn.textContent = "Copied";\n      setTimeout(function() { copyBtn.textContent = "Copy"; }, 1000);\n    });\n\n    primary.appendChild(left);\n    primary.appendChild(metaEl);\n    primary.appendChild(copyBtn);\n    div.appendChild(primary);\n\n    var summary = summarize(entry);\n    if (summary) {\n      var subtitle = document.createElement("div");\n      subtitle.className = "log-subtitle";\n      subtitle.textContent = summary;\n      div.appendChild(subtitle);\n    }\n\n    var detail = document.createElement("div");\n    detail.className = "log-detail";\n    detail.textContent = JSON.stringify(entry.body, null, 2);\n    div.appendChild(detail);\n\n    div.addEventListener("click", function() { div.classList.toggle("expanded"); });\n    return div;\n  }\n\n  var polling = false;\n  var firstPoll = true;\n  function poll() {\n    if (polling) return;\n    polling = true;\n    fetch("/api/logs?since=" + lastId)\n      .then(function(r) { return r.ok ? r.json() : []; })\n      .then(function(data) {\n        if (!data || !data.length) return;\n        lastId = data[data.length - 1].id;\n        totalCount += data.length;\n        countEl.textContent = String(totalCount);\n        var panelVisible = !panel.classList.contains("hidden");\n        var atBottom = !panelVisible || entries.scrollHeight - entries.scrollTop - entries.clientHeight < 40;\n        for (var i = 0; i < data.length; i++) {\n          var el = renderEntry(data[i]);\n          el.classList.add("new");\n          if (!shouldShow(el)) el.style.display = "none";\n          entries.appendChild(el);\n        }\n        if (atBottom || (firstPoll && panelVisible)) entries.scrollTop = entries.scrollHeight;\n        firstPoll = false;\n      })\n      .catch(function() {})\n      .finally(function() { polling = false; });\n  }\n\n  window.addEventListener("message", function(event) {\n    var data = event.data;\n    if (typeof data === "string") {\n      try { data = JSON.parse(data); } catch(e) { return; }\n    }\n    if (!data || typeof data !== "object") return;\n    if (!data.jsonrpc && !data.method) return;\n    fetch("/api/logs/bridge", {\n      method: "POST",\n      headers: {"Content-Type": "application/json"},\n      body: JSON.stringify({body: data})\n    });\n  });\n\n  setInterval(poll, 500);\n  poll();\n})();\n</script>\n'
```

## _MCP_SDK_VERSION

`fastmcp.cli.apps_dev._MCP_SDK_VERSION`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_MCP_SDK_VERSION = '1.25.2'
```

## logger

`fastmcp.cli.apps_dev.logger`

```python
logger = get_logger(__name__)
```

**Inferred type** (`ty`, not declared in the source): `Logger`

## _MessageLog

`fastmcp.cli.apps_dev._MessageLog`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
class _MessageLog
```

**Declared members (5)**

- `def clear(self) -> None`
- `def get_since(self, since_id: int = 0) -> list[dict[str, Any]]`
- `def log_bridge(self, body: dict[str, Any]) -> None`
- `def log_request(self, body: dict[str, Any]) -> None`
- `def log_response(self, body: dict[str, Any]) -> None`

In-memory buffer of MCP JSON-RPC messages flowing through the proxy.


## _build_picker_html

`fastmcp.cli.apps_dev._build_picker_html`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _build_picker_html(tools: list[dict[str, Any]]) -> str
```

Build Prefab picker page: dropdown selector with per-tool forms.


## _fetch_app_bridge_bundle

`fastmcp.cli.apps_dev._fetch_app_bridge_bundle`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _fetch_app_bridge_bundle(version: str, sdk_version: str) -> tuple[str, str]
```

Async wrapper around _fetch_app_bridge_bundle_sync.


## _fetch_app_bridge_bundle_sync

`fastmcp.cli.apps_dev._fetch_app_bridge_bundle_sync`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _fetch_app_bridge_bundle_sync(version: str, sdk_version: str) -> tuple[str, str]
```

Download app-bridge.js and build an import-map that fixes Zod v4 on esm.sh.

Returns ``(app_bridge_js, import_map_json)`` where *import_map_json* is a
JSON string ready to embed in a ``<script type="importmap">`` tag.

Background
----------
esm.sh's ``zod@x.y.z/es2022/v4.mjs`` only re-exports ``{z, default}``,
losing all individual named exports (``custom``, ``string``, etc.).  The
MCP SDK does ``import * as t from "zod/v4"`` and calls ``t.custom(…)``
which fails.  ``zod@x.y.z/es2022/v4/classic/index.mjs`` exports everything
correctly.  An import-map that remaps the broken URL to the working one
fixes all modules in the page's graph, including those loaded cross-origin
from esm.sh.

ext-apps app-bridge.js imports the SDK via bare specifiers
(``@modelcontextprotocol/sdk/types.js`` etc.) that the browser cannot
resolve.  We rewrite them to concrete esm.sh URLs before serving.

Caching
-------
``app_bridge_js`` is cached on disk because it is static for a given
ext-apps + SDK version pair (downloaded from npm).

The import map is NOT cached.  It contains the concrete zod version that
esm.sh resolves ``zod@^x`` to at fetch time, and that version can change
when esm.sh publishes a new zod release.  A stale cached import map would
redirect the old version URL while the browser loads the new version URL,
so the redirect would not apply and ``t.custom`` would be undefined.


## _has_ui_resource

`fastmcp.cli.apps_dev._has_ui_resource`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _has_ui_resource(tool: dict[str, Any]) -> bool
```

Return True if the tool has a UI resourceUri in its metadata.


## _inject_log_panel

`fastmcp.cli.apps_dev._inject_log_panel`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _inject_log_panel(html: str) -> str
```

Inject the MCP message log panel before </body>.


## _json_for_script

`fastmcp.cli.apps_dev._json_for_script`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _json_for_script(value: Any) -> str
```

Serialize JSON for embedding inside an HTML script element.


## _list_tools

`fastmcp.cli.apps_dev._list_tools`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _list_tools(mcp_url: str) -> list[dict[str, Any]]
```

Return raw tool dicts from the MCP server at mcp_url.


## _log_response_bytes

`fastmcp.cli.apps_dev._log_response_bytes`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _log_response_bytes(log: _MessageLog, raw: bytes, content_type: str) -> None
```

Parse accumulated proxy response bytes and log as message entries.


## _make_dev_app

`fastmcp.cli.apps_dev._make_dev_app`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _make_dev_app(mcp_url: str, app_bridge_js: str, import_map_tag: str, message_log: _MessageLog, log_panel: bool) -> Starlette
```

Build the Starlette dev server application.


## _model_from_schema

`fastmcp.cli.apps_dev._model_from_schema`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
def _model_from_schema(tool_name: str, input_schema: dict[str, Any]) -> type[Any]
```

Dynamically create a Pydantic model from a JSON Schema for form generation.


## _read_mcp_resource

`fastmcp.cli.apps_dev._read_mcp_resource`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _read_mcp_resource(mcp_url: str, uri: str) -> str | None
```

Read an MCP resource by URI and return its text content.


## _start_user_server

`fastmcp.cli.apps_dev._start_user_server`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _start_user_server(server_spec: str, mcp_port: int, reload: bool = True, host: str = '127.0.0.1') -> asyncio.subprocess.Process
```

Start the user's MCP server as a subprocess on mcp_port.


## _wait_for_server

`fastmcp.cli.apps_dev._wait_for_server`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
async def _wait_for_server(url: str, timeout: float = 15.0) -> bool
```

Poll until the server is accepting connections.


## run_dev_apps

`fastmcp.cli.apps_dev.run_dev_apps`

```python
async def run_dev_apps(server_spec: str, mcp_port: int = 8000, dev_port: int = 8080, reload: bool = True, host: str = '127.0.0.1', log_panel: bool = True) -> None
```

Start the full dev environment for a FastMCPApp server.

Starts the user's MCP server on *mcp_port*, starts the Prefab dev UI
on *dev_port* (with an /mcp proxy to the user's server), then opens
the browser.


