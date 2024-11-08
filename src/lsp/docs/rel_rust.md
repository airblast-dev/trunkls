When used, Trunk will compile the Cargo project as WASM and load it.

A rust asset accepts the following attributes:
- href: (optional) The path to the `Cargo.toml`, if a directory is provided a `Cargo.toml` is searched in the directory. If no value is specified, the parent directory of the HMTL file is searched.
- `data-target-name`: (optional) the name of the target artifact to load. If the Cargo project has multiple targets (binaries and library), this value can be used to select which one should be used by trunk.
- `data-bin`: (optional) the name of the binary to compile and load. If the Cargo project has multiple binaries, this value can be used to specify that a specific binary should be compiled (using --bin) and used by trunk. This implicitly includes data-target-name.
- `data-type`: (optional) specifies how the binary should be loaded into the project. Can be set to main or worker. main is the default. There can only be one main link. For workers a wasm-bindgen javascript wrapper and the wasm file (with \_bg.wasm suffix) is created, named after the binary name (if provided) or project name. See one of the webworker examples on how to load them.
- `data-cargo-features`: (optional) Space or comma separated list of cargo features to activate.
- `data-cargo-no-default-features`: (optional) Disables the default Cargo features.
- `data-cargo-all-features`: (optional) Enables all Cargo features.
        Neither compatible with data-cargo-features nor data-cargo-no-default-features.
- `data-wasm-opt`: (optional) run wasm-opt with the set optimization level. The possible values are 0, 1, 2, 3, 4, s, z or an empty value for wasm-opt's default. 
- `data-keep-debug`: (optional) instruct wasm-bindgen to preserve debug info in the final WASM output, even for --release mode. This may conflict with the use of wasm-opt, so to be sure, it is recommended to set data-wasm-opt="0" when using this option.
- `data-no-demangle`: (optional) instruct wasm-bindgen to not demangle Rust symbol names.
- `data-reference-types`: (optional) instruct wasm-bindgen to enable reference types.
- `data-weak-refs`: (optional) instruct wasm-bindgen to enable weak references.
- `data-typescript`: (optional) instruct wasm-bindgen to output Typescript bindings. Defaults to false.
- `data-bindgen-target`: (optional) specifies the value of the wasm-bindgen flag [--target](https://rustwasm.github.io/wasm-bindgen/reference/deployment.html). Defaults to no-modules.
- `data-loader-shim`: (optional) instruct trunk to create a loader shim for web workers. Defaults to false.
- `data-cross-origin`: (optional) the crossorigin setting when loading the code & script resources. Defaults to plain anonymous.
- `data-integrity`: (optional) the integrity digest type for code & script resources. Defaults to plain sha384.
- `data-wasm-no-import`: (optional) by default, Trunk will generate an import of functions exported from Rust. Enabling this flag disables this feature. Defaults to false.
- `data-wasm-import-name`: (optional) the name of the global variable where the functions imported from WASM will be available (under the window object). Defaults to wasmBindings (which makes them available via window.wasmBindings.<functionName>).
- `data-target-path`: (optional) Path where the output is placed inside the dist dir. If not present, the directory is placed in the `dist` root. The path must be a relative path without `..`.

