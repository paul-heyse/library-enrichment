# Capability map

Each page maps one capability to the types that implement it, the upstream guide that
explains it, and the decisions worth making deliberately. Start here when the question
is *which construct do I want*, rather than *how is this spelled*.

| Topic | Covers |
|---|---|
| [Composing servers: providers, transforms, mounts and proxies](composition.md) | the four routes, which one you actually want, and the two that 4.0 removed |
| [Registering tools, resources and prompts](registering-components.md) | the decorator surface, what each decorator returns, and annotation requirements |
| [Intercepting requests with middleware](middleware-and-hooks.md) | the twelve hooks, their signatures, and the async requirement |
| [Authenticating callers](authentication.md) | the provider hierarchy, and which class to actually subclass |
| [Connecting a client](client-and-transports.md) | the client session, the eleven transport classes, and what each accepts |
| [What a tool body can do: context, progress, logging](context-and-progress.md) | the Context object, and which of its members are async |
| [Asking the caller: elicitation and sampling](elicitation-and-sampling.md) | the two round-trips from server back to client |
| [The wire model and protocol eras](protocol-eras-and-wire-types.md) | why the same type name exists more than once, and when that matters |
| [What fails, and what you catch](errors-and-failure.md) | the exception hierarchy, and the one class that changes shape |
| [Testing a server](testing.md) | in-process clients, and what the upstream suite actually does |
| [Running and deploying a server](deployment.md) | the server transports and the configuration surface |
| [What the optional installs unlock](optional-extras.md) | the seven extras, their distributions and their modules |
