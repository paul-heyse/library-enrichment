# `fastmcp.utilities.ui`

Distribution: `fastmcp`

## BASE_STYLES

`fastmcp.utilities.ui.BASE_STYLES`

```python
BASE_STYLES = "\n    * {\n        margin: 0;\n        padding: 0;\n        box-sizing: border-box;\n    }\n\n    body {\n        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;\n        margin: 0;\n        padding: 0;\n        min-height: 100vh;\n        display: flex;\n        align-items: center;\n        justify-content: center;\n        background: #f9fafb;\n        color: #0a0a0a;\n    }\n\n    .container {\n        background: #ffffff;\n        border: 1px solid #e5e7eb;\n        padding: 3rem 2.5rem;\n        border-radius: 1rem;\n        box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);\n        text-align: center;\n        max-width: 36rem;\n        margin: 1rem;\n        width: 100%;\n    }\n\n    @media (max-width: 640px) {\n        .container {\n            padding: 2rem 1.5rem;\n            margin: 0.5rem;\n        }\n    }\n\n    .logo {\n        width: 64px;\n        height: auto;\n        margin-bottom: 1.5rem;\n        display: block;\n        margin-left: auto;\n        margin-right: auto;\n    }\n\n    h1 {\n        font-size: 1.5rem;\n        font-weight: 600;\n        margin-bottom: 1.5rem;\n        color: #111827;\n    }\n"
```

**Inferred type** (`ty`, not declared in the source): `Literal["\n * {\n margin: 0;\n padding: 0;\n box-sizing: border-box;\n }\n\n body {\n font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;\n margin: 0;\n padding: 0;\n min-height: 100vh;\n display: flex;\n align-items: center;\n justify-content: center;\n background: #f9fafb;\n color: #0a0a0a;\n }\n\n .container {\n background: #ffffff;\n border: 1px solid #e5e7eb;\n padding: 3rem 2.5rem;\n border-radius: 1rem;\n box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);\n text-align: center;\n max-width: 36rem;\n margin: 1rem;\n width: 100%;\n }\n\n @media (max-width: 640px) {\n .container {\n padding: 2rem 1.5rem;\n margin: 0.5rem;\n }\n }\n\n .logo {\n width: 64px;\n height: auto;\n margin-bottom: 1.5rem;\n display: block;\n margin-left: auto;\n margin-right: auto;\n }\n\n h1 {\n font-size: 1.5rem;\n font-weight: 600;\n margin-bottom: 1.5rem;\n color: #111827;\n }\n"]`

## BUTTON_STYLES

Import as `fastmcp.server.auth.oauth_proxy.ui.BUTTON_STYLES`  ·  defined at `fastmcp.utilities.ui.BUTTON_STYLES`

```python
BUTTON_STYLES = '\n    .button-group {\n        display: flex;\n        gap: 0.75rem;\n        margin-top: 1.5rem;\n        justify-content: center;\n    }\n\n    button {\n        padding: 0.75rem 2rem;\n        font-size: 0.9375rem;\n        font-weight: 500;\n        border-radius: 0.5rem;\n        border: none;\n        cursor: pointer;\n        transition: all 0.15s;\n        font-family: inherit;\n    }\n\n    button:hover {\n        transform: translateY(-1px);\n        box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);\n    }\n\n    .btn-approve, .btn-primary {\n        background: #10b981;\n        color: #ffffff;\n        min-width: 120px;\n    }\n\n    .btn-deny, .btn-secondary {\n        background: #6b7280;\n        color: #ffffff;\n        min-width: 120px;\n    }\n'
```

**Inferred type** (`ty`, not declared in the source): `Literal["\n .button-group {\n display: flex;\n gap: 0.75rem;\n margin-top: 1.5rem;\n justify-content: center;\n }\n\n button {\n padding: 0.75rem 2rem;\n font-size: 0.9375rem;\n font-weight: 500;\n border-radius: 0.5rem;\n border: none;\n cursor: pointer;\n transition: all 0.15s;\n font-family: inherit;\n }\n\n button:hover {\n transform: translateY(-1px);\n box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);\n }\n\n .btn-approve, .btn-primary {\n background: #10b981;\n color: #ffffff;\n min-width: 120px;\n }\n\n .btn-deny, .btn-secondary {\n background: #6b7280;\n color: #ffffff;\n min-width: 120px;\n }\n"]`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## DETAILS_STYLES

Import as `fastmcp.server.auth.oauth_proxy.ui.DETAILS_STYLES`  ·  defined at `fastmcp.utilities.ui.DETAILS_STYLES`

```python
DETAILS_STYLES = '\n    details {\n        margin-bottom: 1.5rem;\n        text-align: left;\n    }\n\n    summary {\n        cursor: pointer;\n        font-size: 0.875rem;\n        color: #6b7280;\n        font-weight: 600;\n        list-style: none;\n        padding: 0.5rem;\n        border-radius: 0.25rem;\n    }\n\n    summary:hover {\n        background: #f9fafb;\n    }\n\n    summary::marker {\n        display: none;\n    }\n\n    summary::before {\n        content: "▶";\n        display: inline-block;\n        margin-right: 0.5rem;\n        transition: transform 0.2s;\n        font-size: 0.75rem;\n    }\n\n    details[open] summary::before {\n        transform: rotate(90deg);\n    }\n'
```

**Inferred type** (`ty`, not declared in the source): `Literal["\n details {\n margin-bottom: 1.5rem;\n text-align: left;\n }\n\n summary {\n cursor: pointer;\n font-size: 0.875rem;\n color: #6b7280;\n font-weight: 600;\n list-style: none;\n padding: 0.5rem;\n border-radius: 0.25rem;\n }\n\n summary:hover {\n background: #f9fafb;\n }\n\n summary::marker {\n display: none;\n }\n\n summary::before {\n content: \"▶\";\n display: inline-block;\n margin-right: 0.5rem;\n transition: transform 0.2s;\n font-size: 0.75rem;\n }\n\n details[open] summary::before {\n transform: rotate(90deg);\n }\n"]`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## DETAIL_BOX_STYLES

Import as `fastmcp.server.auth.oauth_proxy.ui.DETAIL_BOX_STYLES`  ·  defined at `fastmcp.utilities.ui.DETAIL_BOX_STYLES`

```python
DETAIL_BOX_STYLES = "\n    .detail-box {\n        background: #f9fafb;\n        border: 1px solid #e5e7eb;\n        border-radius: 0.5rem;\n        padding: 1rem;\n        margin-bottom: 1.5rem;\n        text-align: left;\n    }\n\n    .detail-row {\n        display: flex;\n        padding: 0.5rem 0;\n        border-bottom: 1px solid #e5e7eb;\n    }\n\n    .detail-row:last-child {\n        border-bottom: none;\n    }\n\n    .detail-label {\n        font-weight: 600;\n        min-width: 160px;\n        color: #6b7280;\n        font-size: 0.875rem;\n        flex-shrink: 0;\n        padding-right: 1rem;\n    }\n\n    .detail-value {\n        flex: 1;\n        font-family: 'SF Mono', 'Monaco', 'Consolas', 'Courier New', monospace;\n        font-size: 0.75rem;\n        color: #111827;\n        word-break: break-all;\n        overflow-wrap: break-word;\n    }\n"
```

**Inferred type** (`ty`, not declared in the source): `Literal["\n .detail-box {\n background: #f9fafb;\n border: 1px solid #e5e7eb;\n border-radius: 0.5rem;\n padding: 1rem;\n margin-bottom: 1.5rem;\n text-align: left;\n }\n\n .detail-row {\n display: flex;\n padding: 0.5rem 0;\n border-bottom: 1px solid #e5e7eb;\n }\n\n .detail-row:last-child {\n border-bottom: none;\n }\n\n .detail-label {\n font-weight: 600;\n min-width: 160px;\n color: #6b7280;\n font-size: 0.875rem;\n flex-shrink: 0;\n padding-right: 1rem;\n }\n\n .detail-value {\n flex: 1;\n font-family: 'SF Mono', 'Monaco', 'Consolas', 'Courier New', monospace;\n font-size: 0.75rem;\n color: #111827;\n word-break: break-all;\n overflow-wrap: break-word;\n }\n"]`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## FASTMCP_LOGO_URL

`fastmcp.utilities.ui.FASTMCP_LOGO_URL`

```python
FASTMCP_LOGO_URL = 'https://gofastmcp.com/assets/brand/blue-logo.png'
```

**Inferred type** (`ty`, not declared in the source): `Literal["https://gofastmcp.com/assets/brand/blue-logo.png"]`

## HELPER_TEXT_STYLES

Import as `fastmcp.client.oauth_callback.HELPER_TEXT_STYLES`  ·  defined at `fastmcp.utilities.ui.HELPER_TEXT_STYLES`

```python
HELPER_TEXT_STYLES = '\n    .close-instruction, .help-text {\n        font-size: 0.875rem;\n        color: #6b7280;\n        margin-top: 1.5rem;\n    }\n'
```

**Inferred type** (`ty`, not declared in the source): `Literal["\n .close-instruction, .help-text {\n font-size: 0.875rem;\n color: #6b7280;\n margin-top: 1.5rem;\n }\n"]`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## INFO_BOX_STYLES

Import as `fastmcp.client.oauth_callback.INFO_BOX_STYLES`  ·  defined at `fastmcp.utilities.ui.INFO_BOX_STYLES`

```python
INFO_BOX_STYLES = "\n    .info-box {\n        background: #f0f9ff;\n        border: 1px solid #bae6fd;\n        border-radius: 0.5rem;\n        padding: 1rem;\n        margin-bottom: 1.5rem;\n        text-align: left;\n        font-size: 0.9375rem;\n        line-height: 1.5;\n        color: #374151;\n    }\n\n    .info-box p {\n        margin-bottom: 0.5rem;\n    }\n\n    .info-box p:last-child {\n        margin-bottom: 0;\n    }\n\n    .info-box.centered {\n        text-align: center;\n    }\n\n    .info-box.error {\n        background: #fef2f2;\n        border-color: #fecaca;\n        color: #991b1b;\n    }\n\n    .info-box strong {\n        color: #0ea5e9;\n        font-weight: 600;\n    }\n\n    .info-box .server-name-link {\n        color: #0ea5e9;\n        text-decoration: underline;\n        font-weight: 600;\n        cursor: pointer;\n        transition: opacity 0.15s;\n    }\n\n    .info-box .server-name-link:hover {\n        opacity: 0.8;\n    }\n\n    /* Monospace info box - gray styling with code font */\n    .info-box-mono {\n        background: #f9fafb;\n        border: 1px solid #e5e7eb;\n        border-radius: 0.5rem;\n        padding: 0.875rem;\n        margin: 1.25rem 0;\n        font-size: 0.875rem;\n        color: #6b7280;\n        font-family: 'SF Mono', 'Monaco', 'Consolas', 'Courier New', monospace;\n        text-align: left;\n    }\n\n    .info-box-mono.centered {\n        text-align: center;\n    }\n\n    .info-box-mono.error {\n        background: #fef2f2;\n        border-color: #fecaca;\n        color: #991b1b;\n    }\n\n    .info-box-mono strong {\n        color: #111827;\n        font-weight: 600;\n    }\n\n    .warning-box {\n        background: #f0f9ff;\n        border: 1px solid #bae6fd;\n        border-radius: 0.5rem;\n        padding: 1rem;\n        margin-bottom: 1.5rem;\n        text-align: center;\n    }\n\n    .warning-box p {\n        margin-bottom: 0.5rem;\n        line-height: 1.5;\n        color: #6b7280;\n        font-size: 0.9375rem;\n    }\n\n    .warning-box p:last-child {\n        margin-bottom: 0;\n    }\n\n    .warning-box strong {\n        color: #0ea5e9;\n        font-weight: 600;\n    }\n\n    .warning-box a {\n        color: #0ea5e9;\n        text-decoration: underline;\n        font-weight: 600;\n    }\n\n    .warning-box a:hover {\n        color: #0284c7;\n        text-decoration: underline;\n    }\n"
```

**Inferred type** (`ty`, not declared in the source): `Literal["\n .info-box {\n background: #f0f9ff;\n border: 1px solid #bae6fd;\n border-radius: 0.5rem;\n padding: 1rem;\n margin-bottom: 1.5rem;\n text-align: left;\n font-size: 0.9375rem;\n line-height: 1.5;\n color: #374151;\n }\n\n .info-box p {\n margin-bottom: 0.5rem;\n }\n\n .info-box p:last-child {\n margin-bottom: 0;\n }\n\n .info-box.centered {\n text-align: center;\n }\n\n .info-box.error {\n background: #fef2f2;\n border-color: #fecaca;\n color: #991b1b;\n }\n\n .info-box strong {\n color: #0ea5e9;\n font-weight: 600;\n }\n\n .info-box .server-name-link {\n color: #0ea5e9;\n text-decoration: underline;\n font-weight: 600;\n cursor: pointer;\n transition: opacity 0.15s;\n }\n\n .info-box .server-name-link:hover {\n opacity: 0.8;\n }\n\n /* Monospace info box - gray styling with code font */\n .info-box-mono {\n background: #f9fafb;\n border: 1px solid #e5e7eb;\n border-radius: 0.5rem;\n padding: 0.875rem;\n margin: 1.25rem 0;\n font-size: 0.875rem;\n color: #6b7280;\n font-family: 'SF Mono', 'Monaco', 'Consolas', 'Courier New', monospace;\n text-align: left;\n }\n\n .info-box-mono.centered {\n text-align: center;\n }\n\n .info-box-mono.error {\n background: #fef2f2;\n border-color: #fecaca;\n color: #991b1b;\n }\n\n .info-box-mono strong {\n color: #111827;\n font-weight: 600;\n }\n\n .warning-box {\n background: #f0f9ff;\n border: 1px solid #bae6fd;\n border-radius: 0.5rem;\n padding: 1rem;\n margin-bottom: 1.5rem;\n text-align: center;\n }\n\n .warning-box p {\n margin-bottom: 0.5rem;\n line-height: 1.5;\n color: #6b7280;\n font-size: 0.9375rem;\n }\n\n .warning-box p:last-child {\n margin-bottom: 0;\n }\n\n .warning-box strong {\n color: #0ea5e9;\n font-weight: 600;\n }\n\n .warning-box a {\n color: #0ea5e9;\n text-decoration: underline;\n font-weight: 600;\n }\n\n .warning-box a:hover {\n color: #0284c7;\n text-decoration: underline;\n }\n"]`

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## REDIRECT_SECTION_STYLES

Import as `fastmcp.server.auth.oauth_proxy.ui.REDIRECT_SECTION_STYLES`  ·  defined at `fastmcp.utilities.ui.REDIRECT_SECTION_STYLES`

```python
REDIRECT_SECTION_STYLES = "\n    .redirect-section {\n        background: #fffbeb;\n        border: 1px solid #fcd34d;\n        border-radius: 0.5rem;\n        padding: 1rem;\n        margin-bottom: 1.5rem;\n        text-align: left;\n    }\n\n    .redirect-section .label {\n        font-size: 0.875rem;\n        color: #6b7280;\n        font-weight: 600;\n        margin-bottom: 0.5rem;\n        display: block;\n    }\n\n    .redirect-section .value {\n        font-family: 'SF Mono', 'Monaco', 'Consolas', 'Courier New', monospace;\n        font-size: 0.875rem;\n        color: #111827;\n        word-break: break-all;\n        margin-top: 0.25rem;\n    }\n"
```

**Inferred type** (`ty`, not declared in the source): `Literal["\n .redirect-section {\n background: #fffbeb;\n border: 1px solid #fcd34d;\n border-radius: 0.5rem;\n padding: 1rem;\n margin-bottom: 1.5rem;\n text-align: left;\n }\n\n .redirect-section .label {\n font-size: 0.875rem;\n color: #6b7280;\n font-weight: 600;\n margin-bottom: 0.5rem;\n display: block;\n }\n\n .redirect-section .value {\n font-family: 'SF Mono', 'Monaco', 'Consolas', 'Courier New', monospace;\n font-size: 0.875rem;\n color: #111827;\n word-break: break-all;\n margin-top: 0.25rem;\n }\n"]`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## STATUS_MESSAGE_STYLES

Import as `fastmcp.client.oauth_callback.STATUS_MESSAGE_STYLES`  ·  defined at `fastmcp.utilities.ui.STATUS_MESSAGE_STYLES`

```python
STATUS_MESSAGE_STYLES = '\n    .status-message {\n        display: flex;\n        align-items: center;\n        justify-content: center;\n        gap: 0.75rem;\n        margin-bottom: 1.5rem;\n    }\n\n    .status-icon {\n        font-size: 1.5rem;\n        line-height: 1;\n        display: inline-flex;\n        align-items: center;\n        justify-content: center;\n        width: 2rem;\n        height: 2rem;\n        border-radius: 0.5rem;\n        flex-shrink: 0;\n    }\n\n    .status-icon.success {\n        background: #10b98120;\n    }\n\n    .status-icon.error {\n        background: #ef444420;\n    }\n\n    .message {\n        font-size: 1.125rem;\n        line-height: 1.75;\n        color: #111827;\n        font-weight: 600;\n        text-align: left;\n    }\n'
```

**Inferred type** (`ty`, not declared in the source): `Literal["\n .status-message {\n display: flex;\n align-items: center;\n justify-content: center;\n gap: 0.75rem;\n margin-bottom: 1.5rem;\n }\n\n .status-icon {\n font-size: 1.5rem;\n line-height: 1;\n display: inline-flex;\n align-items: center;\n justify-content: center;\n width: 2rem;\n height: 2rem;\n border-radius: 0.5rem;\n flex-shrink: 0;\n }\n\n .status-icon.success {\n background: #10b98120;\n }\n\n .status-icon.error {\n background: #ef444420;\n }\n\n .message {\n font-size: 1.125rem;\n line-height: 1.75;\n color: #111827;\n font-weight: 600;\n text-align: left;\n }\n"]`

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## TOOLTIP_STYLES

Import as `fastmcp.server.auth.oauth_proxy.ui.TOOLTIP_STYLES`  ·  defined at `fastmcp.utilities.ui.TOOLTIP_STYLES`

```python
TOOLTIP_STYLES = "\n    .help-link-container {\n        position: fixed;\n        bottom: 1.5rem;\n        right: 1.5rem;\n        font-size: 0.875rem;\n    }\n\n    .help-link {\n        color: #6b7280;\n        text-decoration: none;\n        cursor: help;\n        position: relative;\n        display: inline-block;\n        border-bottom: 1px dotted #9ca3af;\n    }\n\n    @media (max-width: 640px) {\n        .help-link {\n            background: #ffffff;\n            padding: 0.25rem 0.5rem;\n            border-radius: 0.25rem;\n            box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);\n        }\n    }\n\n    .help-link:hover {\n        color: #111827;\n        border-bottom-color: #111827;\n    }\n\n    .help-link:hover .tooltip {\n        opacity: 1;\n        visibility: visible;\n    }\n\n    .tooltip {\n        position: absolute;\n        bottom: 100%;\n        right: 0;\n        left: auto;\n        margin-bottom: 0.5rem;\n        background: #1f2937;\n        color: #ffffff;\n        padding: 0.75rem 1rem;\n        border-radius: 0.5rem;\n        font-size: 0.8125rem;\n        line-height: 1.5;\n        width: 280px;\n        max-width: calc(100vw - 3rem);\n        opacity: 0;\n        visibility: hidden;\n        transition: opacity 0.2s, visibility 0.2s;\n        box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);\n        text-align: left;\n    }\n\n    .tooltip::after {\n        content: '';\n        position: absolute;\n        top: 100%;\n        right: 1rem;\n        border: 6px solid transparent;\n        border-top-color: #1f2937;\n    }\n\n    .tooltip-link {\n        color: #60a5fa;\n        text-decoration: underline;\n    }\n"
```

**Inferred type** (`ty`, not declared in the source): `Literal["\n .help-link-container {\n position: fixed;\n bottom: 1.5rem;\n right: 1.5rem;\n font-size: 0.875rem;\n }\n\n .help-link {\n color: #6b7280;\n text-decoration: none;\n cursor: help;\n position: relative;\n display: inline-block;\n border-bottom: 1px dotted #9ca3af;\n }\n\n @media (max-width: 640px) {\n .help-link {\n background: #ffffff;\n padding: 0.25rem 0.5rem;\n border-radius: 0.25rem;\n box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);\n }\n }\n\n .help-link:hover {\n color: #111827;\n border-bottom-color: #111827;\n }\n\n .help-link:hover .tooltip {\n opacity: 1;\n visibility: visible;\n }\n\n .tooltip {\n position: absolute;\n bottom: 100%;\n right: 0;\n left: auto;\n margin-bottom: 0.5rem;\n background: #1f2937;\n color: #ffffff;\n padding: 0.75rem 1rem;\n border-radius: 0.5rem;\n font-size: 0.8125rem;\n line-height: 1.5;\n width: 280px;\n max-width: calc(100vw - 3rem);\n opacity: 0;\n visibility: hidden;\n transition: opacity 0.2s, visibility 0.2s;\n box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);\n text-align: left;\n }\n\n .tooltip::after {\n content: '';\n position: absolute;\n top: 100%;\n right: 1rem;\n border: 6px solid transparent;\n border-top-color: #1f2937;\n }\n\n .tooltip-link {\n color: #60a5fa;\n text-decoration: underline;\n }\n"]`

_2 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

## create_button_group

`fastmcp.utilities.ui.create_button_group`

```python
def create_button_group(buttons: list[tuple[str, str, str]]) -> str
```

Create a group of buttons.

Args:
    buttons: List of (text, value, css_class) tuples

Returns:
    HTML for button group


## create_detail_box

`fastmcp.utilities.ui.create_detail_box`

```python
def create_detail_box(rows: list[tuple[str, str]]) -> str
```

Create a detail box with key-value pairs.

Args:
    rows: List of (label, value) tuples

Returns:
    HTML for detail box


## create_info_box

Import as `fastmcp.client.oauth_callback.create_info_box`  ·  defined at `fastmcp.utilities.ui.create_info_box`

```python
def create_info_box(content: str, is_error: bool = False, centered: bool = False, monospace: bool = False) -> str
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Create an info box.

Args:
    content: HTML content for the info box
    is_error: True for error styling, False for normal
    centered: True to center the text, False for left-aligned
    monospace: True to use gray monospace font styling instead of blue

Returns:
    HTML for info box


## create_logo

Import as `fastmcp.client.oauth_callback.create_logo`  ·  defined at `fastmcp.utilities.ui.create_logo`

```python
def create_logo(icon_url: str | None = None, alt_text: str = 'FastMCP') -> str
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Create logo HTML.

Args:
    icon_url: Optional custom icon URL. If not provided, uses the FastMCP logo.
    alt_text: Alt text for the logo image.

Returns:
    HTML for logo image tag.


## create_page

Import as `fastmcp.client.oauth_callback.create_page`  ·  defined at `fastmcp.utilities.ui.create_page`

```python
def create_page(content: str, title: str = 'FastMCP', additional_styles: str = '', csp_policy: str = "default-src 'none'; style-src 'unsafe-inline'; img-src https: data:; base-uri 'none'") -> str
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Create a complete HTML page with FastMCP styling.

Args:
    content: HTML content to place inside the page
    title: Page title
    additional_styles: Extra CSS to include
    csp_policy: Content Security Policy header value.
        If empty string "", the CSP meta tag is omitted entirely.

Returns:
    Complete HTML page as string


## create_secure_html_response

Import as `fastmcp.client.oauth_callback.create_secure_html_response`  ·  defined at `fastmcp.utilities.ui.create_secure_html_response`

```python
def create_secure_html_response(html: str, status_code: int = 200) -> HTMLResponse
```

_3 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Create an HTMLResponse with security headers.

Adds X-Frame-Options: DENY to prevent clickjacking attacks per MCP security best practices.

Args:
    html: HTML content to return
    status_code: HTTP status code

Returns:
    HTMLResponse with security headers


## create_status_message

Import as `fastmcp.client.oauth_callback.create_status_message`  ·  defined at `fastmcp.utilities.ui.create_status_message`

```python
def create_status_message(message: str, is_success: bool = True) -> str
```

_1 further import-site paths reach this item; they work but are not declared API. See `content/index/aliases.tsv`._

Create a status message with icon.

Args:
    message: Status message text
    is_success: True for success (✓), False for error (✕)

Returns:
    HTML for status message


