# 42 Header for Zed

_An extension for the Zed editor (https://zed.dev/) that automatically inserts and updates the standard 42 school header.
Features_
 * Auto-insert: Inserts the header when creating a new file.
 * Auto-update: Updates the Updated timestamp on every save.
 * Feedback: Displays a subtle notification (echo) in the status bar to confirm the operation success.
Installation
From Source
 1. Clone this repository into your Zed extensions directory:
 ```
   mkdir -p ~/.config/zed/extensions
   cd ~/.config/zed/extensions
   git clone <YOUR_REPO_URL> 42header
   ```
 2. In Zed, open the command palette``` (Cmd+Shift+P or Ctrl+Shift+P) and type extensions: dev mode.```
 3. Select the 42header folder you just cloned.
Configuration
To personalize the header with your information, add the following to your Zed settings (Cmd+, then open settings.json):
 ```
{
"42header.username": "your_login",
"42header.email": "your_login@student.42.fr"
}
```
Usage
 * The extension triggers automatically upon saving a file (Cmd+S).
 * Feedback: You will see a "42 Header inserted!" or "42 Header updated!" message in the status bar to confirm the action.
Development
This project is written in Rust and compiled to WebAssembly (WASM) to integrate natively with Zed.
 * To build: cargo build --release --target wasm32-wasi
License
This project is distributed under the MIT License. See the LICENSE file for details.

 <https://github.com/Louka2b/Zed_42Header>
