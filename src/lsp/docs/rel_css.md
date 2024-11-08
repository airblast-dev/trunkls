Trunk will copy linked css files found in the source HTML without content modification. 

This content is hashed for cache control. The href attribute must be included in the link pointing to the css file to be processed.

A CSS asset accepts the following attributes:
- `data-integrity`: (optional) the integrity digest type for code & script resources. Defaults to plain sha384.
- `data-no-minify`: (optional) Opt-out of minification. Also see: [Minification](https://trunkrs.dev/assets/#minification).
- `data-target-path`: (optional) Path where the output is placed inside the dist dir. If not present, the directory is placed in the `dist` root. The path must be a relative path without `..`.

