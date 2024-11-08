The name of the global variable where the functions imported from WASM will be available (under the `window` object). 

Defaults to `wasmBindings` (which makes them available via `window.wasmBindings.<functionName>`).
