Specifies the value of the wasm-bindgen [flag --target](https://rustwasm.github.io/wasm-bindgen/reference/deployment.html) (see link for possible values).

Defaults to no-modules. The main use-case is to switch to web with data-type="worker" which reduces backwards [compatibility](https://caniuse.com/mdn-api_worker_worker_ecmascript_modules) but with some [advantages](https://rustwasm.github.io/wasm-bindgen/examples/without-a-bundler.html?highlight=no-modules#using-the-older---target-no-modules).
