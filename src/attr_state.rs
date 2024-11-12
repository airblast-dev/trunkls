use std::str::FromStr;

use crate::lsp::docs::ValueRequirment;

#[derive(Clone, Debug, Default)]
pub struct TrunkAttrState {
    // Wether a data-trunk attribute is already present.
    pub data_trunk: bool,
    // If an asset type is currently selected.
    //
    // for example `rel=""` is `None` but `rel="css"` is `Some(AssetType::Css)`
    pub rel: Option<AssetType>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AssetType {
    Rust,
    Css,
    Tailwind,
    Sass,
    Scss,
    Icon,
    Inline,
    CopyFile,
    CopyDir,
}

impl AssetType {
    pub fn to_info(self) -> &'static [(&'static str, &'static str, ValueRequirment)] {
        use crate::lsp::docs::*;
        match self {
            AssetType::Rust => RelRust::ASSET_ATTRS,
            AssetType::Css => RelCss::ASSET_ATTRS,
            AssetType::Sass => RelSass::ASSET_ATTRS,
            AssetType::Scss => RelScss::ASSET_ATTRS,
            AssetType::Icon => RelIcon::ASSET_ATTRS,
            AssetType::Tailwind => RelTailwind::ASSET_ATTRS,
            AssetType::CopyDir => RelCopyDir::ASSET_ATTRS,
            AssetType::CopyFile => RelCopyFile::ASSET_ATTRS,
            AssetType::Inline => RelInline::ASSET_ATTRS,
        }
    }
}

impl FromStr for AssetType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use AssetType::*;
        let asset = match s {
            "rust" => Rust,
            "css" => Css,
            "tailwind-css" => Tailwind,
            "sass" => Sass,
            "scss" => Scss,
            "icon" => Icon,
            "inline" => Inline,
            "copy-file" => CopyFile,
            "copy-dir" => CopyDir,
            _ => return Err(()),
        };

        Ok(asset)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::attr_state::AssetType;

    #[test]
    fn asset_type_from_str() {
        assert_eq!(AssetType::from_str("rust"), Ok(AssetType::Rust));
        assert_eq!(AssetType::from_str("css"), Ok(AssetType::Css));
        assert_eq!(AssetType::from_str("tailwind-css"), Ok(AssetType::Tailwind));
        assert_eq!(AssetType::from_str("sass"), Ok(AssetType::Sass));
        assert_eq!(AssetType::from_str("scss"), Ok(AssetType::Scss));
        assert_eq!(AssetType::from_str("icon"), Ok(AssetType::Icon));
        assert_eq!(AssetType::from_str("inline"), Ok(AssetType::Inline));
        assert_eq!(AssetType::from_str("copy-file"), Ok(AssetType::CopyFile));
        assert_eq!(AssetType::from_str("copy-dir"), Ok(AssetType::CopyDir));
        assert_eq!(AssetType::from_str("lol"), Err(()));
    }
}
