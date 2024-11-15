Classic script assets processed by Trunk must follow these three rules:

    Must be declared as a valid HTML script tag.
    Must have the attribute data-trunk.
    Must have the attribute src, pointing to a script file

This will typically look like: `<script data-trunk src="{path}" ..other options here..></script>. All <script data-trunk ...></script>` HTML elements will be replaced with the output HTML of the associated pipeline.

Trunk will copy script files found in the source HTML without content modification. This content is hashed for cache control. The `src` attribute must be included in the script pointing to the script file to be processed.

    `data-no-minify`: (optional) Opt-out of minification. Also see: [Minification](https://trunkrs.dev/assets/#minification).
    `data-target-path`: (optional) Path where the output is placed inside the dist dir. If not present, the directory is placed in the dist root. The path must be a relative path without `..`.

