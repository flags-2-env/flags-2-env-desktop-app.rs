# flags-2-env-desktop-app.rs

Native Rust desktop app. No webviews, no React. UI rendering is isolated in `src/ui.rs`.

Connection lifecycle is a closed `ConnectionPhase` / `ConnectionEvent` state
machine. `DesktopState::transition` is pure and exhaustive, invalid internal
events fail closed, and `failed` is absorbing. The finite implementation graph
test evaluates all 12 phase/event edges; it does not prove network reachability
or remote service behavior.

`net::probe_with` adds a typed native-transport injection point: it invokes a
single caller-provided probe, preserves the configured endpoint, and maps its
`Reachable`/`Unavailable` result to the existing lifecycle. Tests cover
single invocation, success and failure. The default `net::probe` still fails
closed: this change does **not** invent an HTTP client or declare production
connectivity before a verified native transport adapter exists.
