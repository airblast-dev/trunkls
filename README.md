Trunkls is an LSP server that provides hover, and completions for clients.

The provided hover and completions are based off of `trunk`'s [assets](https://trunkrs.dev/assets/) section with some slight modifications to make them more readable in editors.

## Installation
After cloning the repository, you can run `cargo install --path trunkls` to compile and install the binary. 

### VsCode
Running `vsce package` inside `trunkls/vscode` will build the extension. From there you can select the extension file and install it through VsCode.

Depending on your setup you may have to run `npm install` in the `trunkls/vscode` directory.

### Neovim
The exact steps will differ depending on your config, but using `lspconfig` it can be setup as so.
```rust
local configs = require("lspconfig.configs")
configs.trunkls = {
	default_config = {
		cmd = { "trunkls" },
		root_dir = vim.uv.cwd(),
		filetypes = { 'html' }
	},
}
```

## Configuration
Trunkls accepts a log file via `-o` for debugging purposes. 

The logs will be filtered via the environment variable `RUST_LOG=...`.
