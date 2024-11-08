Instruct wasm-bindgen to preserve debug info in the final WASM output, even for --release mode. 

This may conflict with the use of wasm-opt, so to be sure, it is recommended to set data-wasm-opt="0" when using this option.
