Trunk will inline the content of the file specified in the href attribute into index.html. 

This content is copied exactly, no hashing is performed.

An asset that is inline accepts the following tags:
- type: (optional) – If not present, the type is inferred by the file extension.
    - `html`, `svg`
    - `css`: CSS wrapped in `style` tags
    - `js`: JavaScript wrapped in `script` tags
    - `mjs`, module: JavaScript wrapped in `script` tags with `type="module"`
