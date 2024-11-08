Specifies how the binary should be loaded into the project. 

Can be set to main or worker. main is the default. There can only be one main link. For workers a wasm-bindgen javascript wrapper and the wasm file (with \_bg.wasm suffix) is created, named after the binary name (if provided) or project name. See one of the webworker examples on how to load them.
