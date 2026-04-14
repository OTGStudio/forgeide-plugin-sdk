# ForgeIDE Plugin SDK

Build plugins for [ForgeIDE](https://forgeide.com) — the local-first AI game development
workspace for Unity, Unreal Engine 5, and Godot.

Plugins compile to WebAssembly and run in ForgeIDE's Wasmtime sandbox. They can request
capabilities (read project files, search the knowledge base, send AI prompts) that users
explicitly grant in the Plugin Manager.

## Quick start (Rust)

```bash
# Install cargo-generate if you don't have it
cargo install cargo-generate

# Generate a new plugin from the template
cargo generate OTGStudio/forgeide-plugin-sdk --name my-plugin

# Build for WASM
cd my-plugin
cargo build --target wasm32-unknown-unknown --release

# Your plugin is at: target/wasm32-unknown-unknown/release/my_plugin.wasm
```

## Capabilities

Plugins must declare required capabilities in `forgeide-plugin.json`. Users grant
capabilities explicitly in the Plugin Manager before the plugin can use them.

| Capability | What it allows |
|---|---|
| `filesystem:read` | Read files within the current project |
| `kb:read` | Search the 300+ pattern knowledge base |
| `ai:routing` | Send prompts through ForgeIDE's local AI router |
| `filesystem:write` | Write files within the current project |
| `network:local` | TCP/HTTP to localhost |
| `network:internet` | Outbound internet requests |
| `process:spawn` | Spawn child processes |

## Plugin manifest schema

Your plugin must include a `forgeide-plugin.json` manifest. Validate it against the
[JSON Schema](schema/forgeide-plugin.schema.json).

## Distributing your plugin

Package your `.wasm` + `forgeide-plugin.json` (+ optional `README.md`) into a `.zip`:

```bash
zip my-plugin-1.0.0.zip plugin.wasm forgeide-plugin.json README.md
```

Users install via: ForgeIDE Plugin Manager > Install from file > select `.zip`.

Submit to the OTG Studio curated registry: open an issue at
[OTGStudio/forgeide-plugin-sdk](https://github.com/OTGStudio/forgeide-plugin-sdk/issues).

## Examples

- [hello-rust](examples/hello-rust/) — minimal Rust plugin
- unity-perf-advisor — Unity profiler analysis plugin (coming soon)
- godot-scene-linter — Godot scene validation plugin (coming soon)

## License

Apache 2.0 — see [LICENSE](LICENSE).
