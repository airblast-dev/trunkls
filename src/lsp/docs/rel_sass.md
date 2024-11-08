Trunk uses the official dart-sass for compilation. Just link to your sass files from your source HTML, and Trunk will handle the rest. This content is hashed for cache control. The href attribute must be included in the link pointing to the sass/scss file to be processed.

A sass/scss asset accepts the following attributes.
- `data-inline`: (optional) this attribute will inline the compiled CSS from the SASS/SCSS file into a `<style>` tag instead of using a `<link rel="stylesheet">` tag.
- `data-integrity`: (optional) the integrity digest type for code & script resources. Defaults to plain sha384.
- `data-target-path`: (optional) Path where the output is placed inside the dist dir. If not present, the directory is placed in the `dist` root. The path must be a relative path without `..`.

