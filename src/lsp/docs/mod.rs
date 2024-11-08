mod inline;

use constcat::concat_slices;
use lsp_types::{CompletionItem, Documentation, HoverContents, MarkupContent, MarkupKind};

#[derive(Clone, Copy)]
pub enum RequiresValue {
    Bool(bool),
    Values(&'static [(&'static str, &'static str)]),
    AcceptsValue(bool),
}

impl RequiresValue {
    pub fn should_have_value(&self) -> bool {
        matches!(self, Self::Bool(true) | Self::Values(_))
    }
}

const DATA_INTEGRITY: (&str, Option<&str>, RequiresValue) = (
    "data-integrity",
    Some("The hashing algorithm that Trunk will use for integrity checking."),
    RequiresValue::Values(&[
        ("none", "Trunk will not perform any hashing to the asset."),
        (
            "sha256",
            "Trunk will hash the content for integrity checking using `sha256`.",
        ),
        (
            "sha384",
            "Trunk will hash the content for integrity checking using `sha384`.",
        ),
        (
            "sha512",
            "Trunk will hash the content for integrity checking using `sha512`.",
        ),
    ]),
);

#[macro_export]
macro_rules! load_md {
    ($struct:ident, $path:literal, $doc_of:literal) => {
        impl $struct {
            const DOC_OF: &'static str = $doc_of;
            pub const fn as_str() -> &'static str {
                include_str!(concat!($path, ".md"))
            }
        }
    };
}

#[macro_export]
macro_rules! bulk_struct {
    ($($ident:ident),+) => {
        $(
            pub struct $ident;
        )+
    };
}

#[macro_export]
macro_rules! completions {
    ($($ident:ident),+) => {
        $(
            impl $ident {
                pub fn completion() -> CompletionItem {
                    CompletionItem {
                        label: Self::DOC_OF.to_string(),
                        documentation: Some(Documentation::MarkupContent(MarkupContent {
                            kind: MarkupKind::Markdown,
                            value: Self::as_str().to_string(),
                        })),
                        ..Default::default()
                    }
                }
            }
        )+
    };
}

#[macro_export]
macro_rules! hover {
    ($($ident:ident),+) => {
        $(
            impl $ident {
                pub fn hover_contents() -> HoverContents {
                    HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: Self::as_str().to_string(),
                    })
                }
            }
        )+
    };
}

#[macro_export]
macro_rules! asset_attrs {
    ($($ident:ident),+) => {
        $(
            impl $ident {
                pub const ASSET_ATTRS: &'static [(&str, Option<&str>, RequiresValue)] = concat_slices!(
                    [(&str, Option<&str>, RequiresValue)]: $ident::REQUIRED_ASSET_ATTRS, $ident::OPTIONAL_ASSET_ATTRS
                ).as_slice();
            }
        )+
    };
}

#[macro_export]
macro_rules! required_asset_attrs {
    ($ident:ident, $($arr:expr),*) => {
        impl $ident {
            pub const REQUIRED_ASSET_ATTRS: &'static [(&str, Option<&str>, RequiresValue)] = [$($arr),*].as_slice();
        }
    };
}

#[macro_export]
macro_rules! optional_asset_attrs {
    ($ident:ident, $($arr:expr),*) => {
        impl $ident {
            pub const OPTIONAL_ASSET_ATTRS: &'static [(&str, Option<&str>, RequiresValue)] = [$($arr),*].as_slice();
        }
    };
}

bulk_struct! {DataTrunk, RelCopyDir, RelCopyFile, RelCss, RelIcon, RelInline, RelRust, RelSass, RelScss, RelTailwind}

load_md!(DataTrunk, "data_trunk", "data-trunk");
load_md!(RelCopyDir, "rel_copy_dir", "copy-dir");
load_md!(RelCopyFile, "rel_copy_file", "copy-file");
load_md!(RelCss, "rel_css", "css");
load_md!(RelIcon, "rel_icon", "icon");
load_md!(RelInline, "rel_inline", "inline");
load_md!(RelRust, "rel_rust", "rust");
load_md!(RelSass, "rel_sass", "sass");
load_md!(RelScss, "rel_sass", "scss");
load_md!(RelTailwind, "rel_tailwind", "tailwind-css");

completions! {DataTrunk, RelCopyDir, RelCopyFile, RelCss, RelIcon, RelInline, RelRust, RelSass, RelScss, RelTailwind}
required_asset_attrs! {RelCopyFile, ("href", Some("Trunk will copy the file specified in the href attribute to the dist dir."), RequiresValue::Bool(true))}
optional_asset_attrs! {RelCopyFile, ("data-target-path", None, RequiresValue::Bool(true))}

required_asset_attrs! {RelCopyDir, ("href", Some("Trunk will recursively copy the directory specified in the href attribute to the dist dir."), RequiresValue::Bool(true))}
optional_asset_attrs! {RelCopyDir, ("data-target-path", None, RequiresValue::Bool(true))}

required_asset_attrs! {RelInline, ("href", Some("Trunk will inline the content of the file specified in the href attribute into index.html."), RequiresValue::Bool(true))}
optional_asset_attrs! {RelInline, ("type", Some("If not present, the type is inferred by the file extension.
The accepted values are:
- html, svg
- css: CSS wrapped in style tags
- js: JavaScript wrapped in script tags
- mjs, module: JavaScript wrapped in script tags with type=\"module\"
"), RequiresValue::Values(
        &[
            ("html", inline::Html::as_str()),
            ("svg", inline::Svg::as_str()),
            ("js", inline::JS::as_str()),
            ("mjs", inline::MJS::as_str()),
            ("module", inline::Module::as_str()),
            ("css", inline::Css::as_str())
        ]
))}

// TODO: add data-integrity docs
required_asset_attrs! {RelCss, ("href", Some("Trunk will copy linked css files found in the source HTML without content modification."), RequiresValue::Bool(true))}
optional_asset_attrs! {RelCss,
    ("data-no-minify", Some("Opt-out of minification."), RequiresValue::AcceptsValue(false)),
    ("data-target-path", None, RequiresValue::Bool(true)),
    DATA_INTEGRITY
}

required_asset_attrs! {RelIcon,
    ("href", Some("Trunk will copy the icon image specified in the href attribute to the dist dir. "), RequiresValue::Bool(true))
}
optional_asset_attrs! {RelIcon,
    ("data-no-minify", Some("Opt-out of minification."), RequiresValue::AcceptsValue(false)),
    ("data-target-path", None, RequiresValue::Bool(true)),
    DATA_INTEGRITY
}

required_asset_attrs! {RelTailwind,
    ("href", Some("The href attribute must be included in the link pointing to the sass/scss file to be processed."), RequiresValue::Bool(true))
}
optional_asset_attrs! {RelTailwind,
    ("data-inline", Some("Trunk will inline the compiled CSS from the tailwind compilation\
                  into a <style> tag instead of using a <link rel=\"stylesheet\"> tag."), RequiresValue::AcceptsValue(false)),
    ("data-no-minify", Some("Opt-out of minification."), RequiresValue::AcceptsValue(false)),
    ("data-target-path", None, RequiresValue::Bool(true)),
    DATA_INTEGRITY
}

required_asset_attrs! {RelSass, ("href", Some("The href attribute must be included in the link pointing to the sass/scss file to be processed."), RequiresValue::Bool(true))}
optional_asset_attrs! {RelSass,
    ("data-inline", Some("Trunk will inline the compiled CSS from the SASS/SCSS file into a <style> tag instead of using a <link rel=\"stylesheet\"> tag."), RequiresValue::AcceptsValue(false)),
    ("data-target-path", None, RequiresValue::Bool(true)),
    DATA_INTEGRITY
}

required_asset_attrs! {RelScss, ("href", Some("The href attribute must be included in the link pointing to the sass/scss file to be processed."), RequiresValue::Bool(true))}
optional_asset_attrs! {RelScss,
    ("data-inline", Some("Trunk will inline the compiled CSS from the SASS/SCSS file into a <style> tag instead of using a <link rel=\"stylesheet\"> tag."), RequiresValue::AcceptsValue(false)),
    ("data-target-path", None, RequiresValue::Bool(true)),
    DATA_INTEGRITY
}

required_asset_attrs! {RelRust, }

optional_asset_attrs! {RelRust,
    ("href", Some("The value should be the path to the Cargo.toml of the Rust project.

If a directory is specified, then Trunk will look for the Cargo.toml in the given directory.\
If no value is specified, then Trunk will look for a Cargo.toml in the parent directory of the source HTML file."), RequiresValue::Bool(true)),
    ("data-target-name", Some("The the name of the target artifact to load. If the Cargo project has multiple targets (binaries and library), this value can be used to select which one should be used by trunk."), RequiresValue::Bool(true)),
    ("data-bin", None, RequiresValue::Bool(true)),
    ("data-type", None, RequiresValue::Bool(true)),
    ("data-cargo-features", None, RequiresValue::Bool(true)),
    ("data-cargo-no-default-features", None, RequiresValue::AcceptsValue(false)),
    ("data-cargo-all-features", None, RequiresValue::AcceptsValue(false)),
    ("data-wasm-opt", None, RequiresValue::Bool(true)),
    ("data-wasm-opt-params", None, RequiresValue::Bool(true)),
    ("data-keep-debug", None, RequiresValue::AcceptsValue(false)),
    ("data-no-demangle", None, RequiresValue::AcceptsValue(false)),
    ("data-reference-types", None, RequiresValue::AcceptsValue(false)),
    ("data-weak-refs", None, RequiresValue::AcceptsValue(false)),
    ("data-typescript", None, RequiresValue::AcceptsValue(true)),
    ("data-bindgen-target", None, RequiresValue::Bool(true)),
    ("data-loader-shim", None, RequiresValue::AcceptsValue(false)),
    ("data-cross-origin", None, RequiresValue::Bool(true))
    // add the rest
}

asset_attrs! {RelCopyDir, RelCopyFile, RelCss, RelIcon, RelInline, RelRust, RelSass, RelScss, RelTailwind}
hover! {DataTrunk, RelCopyDir, RelCopyFile, RelCss, RelIcon, RelInline, RelRust, RelSass, RelScss, RelTailwind}
