# `rich._export_format`

Distribution: `rich`

## CONSOLE_HTML_FORMAT

Import as `rich.console.CONSOLE_HTML_FORMAT`  ·  defined at `rich._export_format.CONSOLE_HTML_FORMAT`

```python
CONSOLE_HTML_FORMAT = '<!DOCTYPE html>\n<html>\n<head>\n<meta charset="UTF-8">\n<style>\n{stylesheet}\nbody {{\n    color: {foreground};\n    background-color: {background};\n}}\n</style>\n</head>\n<body>\n    <pre style="font-family:Menlo,\'DejaVu Sans Mono\',consolas,\'Courier New\',monospace"><code style="font-family:inherit">{code}</code></pre>\n</body>\n</html>\n'
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## CONSOLE_SVG_FORMAT

Import as `rich.console.CONSOLE_SVG_FORMAT`  ·  defined at `rich._export_format.CONSOLE_SVG_FORMAT`

```python
CONSOLE_SVG_FORMAT = '<svg class="rich-terminal" viewBox="0 0 {width} {height}" xmlns="http://www.w3.org/2000/svg">\n    <!-- Generated with Rich https://www.textualize.io -->\n    <style>\n\n    @font-face {{\n        font-family: "Fira Code";\n        src: local("FiraCode-Regular"),\n                url("https://cdnjs.cloudflare.com/ajax/libs/firacode/6.2.0/woff2/FiraCode-Regular.woff2") format("woff2"),\n                url("https://cdnjs.cloudflare.com/ajax/libs/firacode/6.2.0/woff/FiraCode-Regular.woff") format("woff");\n        font-style: normal;\n        font-weight: 400;\n    }}\n    @font-face {{\n        font-family: "Fira Code";\n        src: local("FiraCode-Bold"),\n                url("https://cdnjs.cloudflare.com/ajax/libs/firacode/6.2.0/woff2/FiraCode-Bold.woff2") format("woff2"),\n                url("https://cdnjs.cloudflare.com/ajax/libs/firacode/6.2.0/woff/FiraCode-Bold.woff") format("woff");\n        font-style: bold;\n        font-weight: 700;\n    }}\n\n    .{unique_id}-matrix {{\n        font-family: Fira Code, monospace;\n        font-size: {char_height}px;\n        line-height: {line_height}px;\n        font-variant-east-asian: full-width;\n    }}\n\n    .{unique_id}-title {{\n        font-size: 18px;\n        font-weight: bold;\n        font-family: arial;\n    }}\n\n    {styles}\n    </style>\n\n    <defs>\n    <clipPath id="{unique_id}-clip-terminal">\n      <rect x="0" y="0" width="{terminal_width}" height="{terminal_height}" />\n    </clipPath>\n    {lines}\n    </defs>\n\n    {chrome}\n    <g transform="translate({terminal_x}, {terminal_y})" clip-path="url(#{unique_id}-clip-terminal)">\n    {backgrounds}\n    <g class="{unique_id}-matrix">\n    {matrix}\n    </g>\n    </g>\n</svg>\n'
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## _SVG_CLASSES_PREFIX

`rich._export_format._SVG_CLASSES_PREFIX`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_SVG_CLASSES_PREFIX = 'rich-svg'
```

## _SVG_FONT_FAMILY

`rich._export_format._SVG_FONT_FAMILY`

> Not nameable: every path to this item passes through a private module. It is real and reachable at runtime, but you cannot import or annotate it.

```python
_SVG_FONT_FAMILY = 'Rich Fira Code'
```

