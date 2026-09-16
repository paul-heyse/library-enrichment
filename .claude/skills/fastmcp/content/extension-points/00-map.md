# Extension points

What you subclass, how much you must write, and how many already exist.
`Required` counts obligations: `@abstractmethod` members, or every member when the base is a Protocol.

| Extension point | Required | Implementors | Role |
|---|---:|---:|---|
| [`Provider`](Provider.md) | 0 | 26 | Where components come from. A server is itself a Provider, and `mount()` and `create_proxy()` are thin wrappers over one. |
| [`Transform`](Transform.md) | 0 | 11 | Rewriting the component catalog, observably. The system can see what you changed, which is what separates this from a Provider that lies. |
| [`Middleware`](Middleware.md) | 0 | 17 | Intercepting requests in flight. Twelve hooks, all async, all pass-through unless you override them. |
| [`AuthProvider`](AuthProvider.md) | 11 | 38 | Authenticating callers. Upstream never subclasses this directly -- read `TokenVerifier` or `OAuthProxy` first. |
| [`TokenVerifier`](TokenVerifier.md) | 0 | 14 | Verifying a bearer token. The smallest useful auth surface, and the one the corpus actually extends. |
| [`OAuthProvider`](OAuthProvider.md) | 0 | 13 | A full OAuth authorization server, rather than verification alone. |
| [`ClientTransport`](ClientTransport.md) | 1 | 11 | How a client reaches a server. Eleven ship; write one for a transport that does not. |
| [`FastMCPComponent`](FastMCPComponent.md) | 0 | 28 | The shared base of tools, resources and prompts -- what every registered thing is. |
| [`AggregateProvider`](AggregateProvider.md) | 0 | 11 | Composing several Providers into one. `FastMCP` reaches `Provider` through this. |
