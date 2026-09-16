# `fastmcp.contrib.component_manager.example`

Distribution: `fastmcp`

## auth

`fastmcp.contrib.component_manager.example.auth`

```python
auth = JWTVerifier(public_key=key_pair.public_key, issuer='https://dev.example.com', audience='my-dev-server', required_scopes=['mcp:read'])
```

**Inferred type** (`ty`, not declared in the source): `JWTVerifier`

## key_pair

`fastmcp.contrib.component_manager.example.key_pair`

```python
key_pair = RSAKeyPair.generate()
```

**Inferred type** (`ty`, not declared in the source): `RSAKeyPair`

## mcp

`fastmcp.contrib.component_manager.example.mcp`

```python
mcp = FastMCP(name='Component Manager', instructions='This is a test server with component manager.', auth=auth)
```

**Inferred type** (`ty`, not declared in the source): `FastMCP[Any]`

## mcp_token

`fastmcp.contrib.component_manager.example.mcp_token`

```python
mcp_token = key_pair.create_token(subject='dev-user', issuer='https://dev.example.com', audience='my-dev-server', scopes=['mcp:write', 'mcp:read'])
```

**Inferred type** (`ty`, not declared in the source): `str`

## mounted

`fastmcp.contrib.component_manager.example.mounted`

```python
mounted = FastMCP(name='Component Manager', instructions='This is a test server with component manager.', auth=auth)
```

**Inferred type** (`ty`, not declared in the source): `FastMCP[Any]`

## mounted_token

`fastmcp.contrib.component_manager.example.mounted_token`

```python
mounted_token = key_pair.create_token(subject='dev-user', issuer='https://dev.example.com', audience='my-dev-server', scopes=['mounted:write', 'mcp:read'])
```

**Inferred type** (`ty`, not declared in the source): `str`

## get_greeting

`fastmcp.contrib.component_manager.example.get_greeting`

```python
def get_greeting() -> str
```

Provides a simple greeting message.


## get_info

`fastmcp.contrib.component_manager.example.get_info`

```python
def get_info() -> str
```

Provides a simple info.


