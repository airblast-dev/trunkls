mod rel_copy_dir;
mod rel_copy_file;
mod rel_css;
mod rel_icon;
mod rel_inline;
mod rel_rust;
mod rel_sass_scss;
mod rel_tailwind;

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

const DATA_INTEGRITY: (&str, &str, RequiresValue) = (
    "data-integrity",
    "The hashing algorithm that Trunk will use for integrity checking.",
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

const DATA_TARGET_PATH: (&str, &str, RequiresValue) = (
    "data-target-path",
    "Path where the output is placed inside the `dist` dir. If not present, the directory is placed in the dist root. The path must be a relative path without `..`.",
    RequiresValue::Bool(true)
);

const DATA_NO_MINIFY: (&str, &str, RequiresValue) = (
    "data-no-minify",
    "Opt-out of minification.",
    RequiresValue::AcceptsValue(false),
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
                pub const ASSET_ATTRS: &'static [(&str, &str, RequiresValue)] = concat_slices!(
                    [(&str, &str, RequiresValue)]: $ident::REQUIRED_ASSET_ATTRS, $ident::OPTIONAL_ASSET_ATTRS
                ).as_slice();
            }
        )+
    };
}

#[macro_export]
macro_rules! required_asset_attrs {
    ($ident:ident, $($arr:expr),*) => {
        impl $ident {
            pub const REQUIRED_ASSET_ATTRS: &'static [(&str, &str, RequiresValue)] = [$($arr),*].as_slice();
        }
    };
}

#[macro_export]
macro_rules! optional_asset_attrs {
    ($ident:ident, $($arr:expr),*) => {
        impl $ident {
            pub const OPTIONAL_ASSET_ATTRS: &'static [(&str, &str, RequiresValue)] = [$($arr),*].as_slice();
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
required_asset_attrs! {RelCopyFile, ("href", rel_copy_file::Href::as_str(), RequiresValue::Bool(true))}
optional_asset_attrs! {RelCopyFile, DATA_TARGET_PATH}

required_asset_attrs! {RelCopyDir, ("href", rel_copy_dir::Href::as_str(), RequiresValue::Bool(true))}
optional_asset_attrs! {RelCopyDir, DATA_TARGET_PATH}

required_asset_attrs! {RelInline, ("href", rel_inline::Href::as_str(), RequiresValue::Bool(true))}
optional_asset_attrs! {RelInline, ("type", rel_inline::Type::as_str(), RequiresValue::Values(
        &[
            ("html", rel_inline::Html::as_str()),
            ("svg", rel_inline::Svg::as_str()),
            ("js", rel_inline::JS::as_str()),
            ("mjs", rel_inline::MJS::as_str()),
            ("module", rel_inline::Module::as_str()),
            ("css", rel_inline::Css::as_str())
        ]
))}

required_asset_attrs! {RelCss, ("href", rel_css::Href::as_str(), RequiresValue::Bool(true))}
optional_asset_attrs! {RelCss,
    DATA_NO_MINIFY,
    DATA_TARGET_PATH,
    DATA_INTEGRITY
}

required_asset_attrs! {RelIcon,
    ("href", rel_icon::Href::as_str(), RequiresValue::Bool(true))
}
optional_asset_attrs! {RelIcon,
DATA_NO_MINIFY,
DATA_TARGET_PATH,
    DATA_INTEGRITY
}

required_asset_attrs! {RelTailwind,
    ("href", rel_tailwind::Href::as_str(), RequiresValue::Bool(true))
}
optional_asset_attrs! {RelTailwind,
    ("data-inline", rel_tailwind::DataInline::as_str(), RequiresValue::AcceptsValue(false)),
DATA_NO_MINIFY,
    DATA_TARGET_PATH,
    DATA_INTEGRITY
}

required_asset_attrs! {RelSass, ("href", rel_sass_scss::Href::as_str(), RequiresValue::Bool(true))}
optional_asset_attrs! {RelSass,
    ("data-inline", rel_sass_scss::DataInline::as_str(), RequiresValue::AcceptsValue(false)),
    DATA_TARGET_PATH,
    DATA_INTEGRITY
}

required_asset_attrs! {RelScss, ("href", rel_sass_scss::Href::as_str(), RequiresValue::Bool(true))}
optional_asset_attrs! {RelScss,
    ("data-inline", rel_sass_scss::DataInline::as_str(), RequiresValue::AcceptsValue(false)),
    DATA_TARGET_PATH,
    DATA_INTEGRITY
}

required_asset_attrs! {RelRust, }
optional_asset_attrs! {RelRust,
    ("href", rel_rust::Href::as_str(), RequiresValue::Bool(true)),
    ("data-target-name", rel_rust::DataTargetName::as_str(), RequiresValue::Bool(true)),
    ("data-bin", rel_rust::DataBin::as_str(), RequiresValue::Bool(true)),
    ("data-type", rel_rust::DataType::as_str(), RequiresValue::Bool(true)),
    ("data-cargo-features", rel_rust::DataCargoFeatures::as_str(), RequiresValue::Bool(true)),
    ("data-cargo-no-default-features", rel_rust::DataCargoNoDefaultFeatures::as_str(), RequiresValue::AcceptsValue(false)),
    ("data-cargo-all-features", rel_rust::DataCargoAllFeatures::as_str(), RequiresValue::AcceptsValue(false)),
    ("data-wasm-opt", rel_rust::DataWasmOpt::as_str(), RequiresValue::AcceptsValue(true)),
    ("data-wasm-opt-params", rel_rust::DataWasmOptParams::as_str(), RequiresValue::Bool(true)),
    ("data-keep-debug", rel_rust::DataKeepDebug::as_str(), RequiresValue::AcceptsValue(false)),
    ("data-no-demangle", rel_rust::DataNoDemangle::as_str(), RequiresValue::AcceptsValue(false)),
    ("data-reference-types", rel_rust::DataReferenceTypes::as_str(), RequiresValue::AcceptsValue(false)),
    ("data-weak-refs", rel_rust::DataWeakRefs::as_str(), RequiresValue::AcceptsValue(false)),
    ("data-typescript", rel_rust::DataTypeScript::as_str(), RequiresValue::AcceptsValue(true)),
    ("data-bindgen-target", rel_rust::DataBindgenTarget::as_str(), RequiresValue::Bool(true)),
    ("data-loader-shim", rel_rust::DataLoaderShim::as_str(), RequiresValue::AcceptsValue(false)),
    ("data-cross-origin", rel_rust::DataCrossOrigin::as_str(), RequiresValue::Bool(true))
    // add the rest
}

asset_attrs! {RelCopyDir, RelCopyFile, RelCss, RelIcon, RelInline, RelRust, RelSass, RelScss, RelTailwind}
hover! {DataTrunk, RelCopyDir, RelCopyFile, RelCss, RelIcon, RelInline, RelRust, RelSass, RelScss, RelTailwind}
