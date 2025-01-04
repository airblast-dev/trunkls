![Crates.io Version](https://img.shields.io/crates/v/trunkls)

Trunkls is an LSP server that provides hover, and completions for clients.

The provided hover and completions are based off of `trunk`'s [assets](https://trunkrs.dev/assets/) section with some slight modifications to make them more readable in editors.

# Usage

## Attribute Completion
`trunkls` provides completions for all attributes `trunk` supports. Only attributes that are compatible with the current HTML tag will be displayed, this includes asset types and the tag name.

![image](https://github.com/user-attachments/assets/c28002c9-77c8-4d6f-989b-f7d7fe65c807)

Attribute values are also supported!

![image](https://github.com/user-attachments/assets/854b365d-3293-447a-9811-5ec5c8b9c510)

## Hover Support

Hover information is also supported. 
In some cases other LSP servers may return doc information for an attribute without the context of them in `trunk` and cause issues if it takes precedence in the editor.
All `trunk` attributes unique to `trunk` work without issues.

![image](https://github.com/user-attachments/assets/c855c672-09ef-47b4-b0b5-31b282fa69a7)




## Installation

### Installing the binary

The crate is added to `crates.io`, installing is easy as running `cargo install trunkls` and adding the binary to your `$PATH`.

You can also clone the repository and compile it yourself by running `cargo install --path trunkls`. 

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
