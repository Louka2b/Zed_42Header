# 42 Header for Zed

_An extension for the Zed editor (https://zed.dev/) that aims to automatically insert and update the standard 42 school header.

> [!IMPORTANT]
> **Current Status:** As of `zed_extension_api` 0.7.0, Zed's WASM extension API does not yet natively support `on_save` hooks. This extension includes the logic but it is currently disabled in the source code until the API is stabilized.

## Features
 * **Auto-insert**: (Planned) Inserts the header when creating a new file.
 * **Auto-update**: (Planned) Updates the Updated timestamp on every save.
 * **Metadata Ready**: Configured for publication on the Zed extension store.

## Installation

### From Source
 1. Clone this repository into your Zed extensions directory:
 ```bash
   mkdir -p ~/.config/zed/extensions
   cd ~/.config/zed/extensions
   git clone https://github.com/Louka2b/Zed_42Header 42header
 ```
 2. In Zed, open the command palette (`Cmd+Shift+P`) and type `extensions: dev mode`.
 3. Select the `42header` folder you just cloned.

## Configuration
To personalize the header, add the following to your Zed `settings.json` (`Cmd+,`):
 ```json
{
  "42header.username": "your_login",
  "42header.email": "your_login@student.42.fr"
}
```

## Development
This project is written in Rust and compiled to WebAssembly (WASM).
 * To build: `cargo build --release --target wasm32-wasip2`

## License
MIT License. See the [LICENSE](LICENSE) file for details.
