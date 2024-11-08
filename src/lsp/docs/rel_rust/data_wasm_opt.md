Run wasm-opt with the set optimization level. 

The possible values are 0, 1, 2, 3, 4, s, z or an empty value for wasm-opt's default. Set this option to 0 to disable wasm-opt explicitly. The values 1-4 are increasingly stronger optimization levels for speed. s and z (z means more optimization) optimize for binary size instead. Only used in --release mode.
