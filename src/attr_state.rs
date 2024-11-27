use std::str::FromStr;

use tracing::{error, instrument, trace};
use tree_sitter::Node;

use crate::lsp::docs::ValueRequirment;

#[derive(Clone, Debug, Default)]
pub struct TrunkAttrState {
    // Wether a data-trunk attribute is already present.
    pub data_trunk: bool,
    /// If an asset type is currently selected.
    ///
    /// for example `rel=""` is `None` but `rel="css"` is `Some(AssetType::Css)`
    pub rel: Option<AssetType>,
    pub tag_name: TagName,
}

impl TrunkAttrState {
    pub fn with_tag_name(tag_name: TagName) -> Self {
        Self {
            tag_name,
            ..Default::default()
        }
    }

    pub fn is_rel_val(&self, s: &str, n: Node) -> bool {
        if self.rel.is_some() {
            return false;
        }

        n.utf8_text(s.as_bytes()).is_ok_and(|s| s == "rel")
    }

    #[instrument(level = "trace", skip(elem_nodes))]
    pub fn from_elem_items<'a, I: Iterator<Item = Node<'a>>>(
        s: &str,
        mut elem_nodes: I,
    ) -> Option<Self> {
        let tag_name = TagName::from(
            elem_nodes
                .next()
                .filter(|tag_name| tag_name.kind() == "tag_name")
                .and_then(|tag_name| tag_name.utf8_text(s.as_bytes()).ok())?,
        );
        let mut attr_state = Self::with_tag_name(tag_name);
        for ch in elem_nodes {
            let Some(attr_name) = ch.named_child(0).filter(|c| c.kind() == "attribute_name") else {
                continue;
            };

            let Ok(attr_name_str) = attr_name.utf8_text(s.as_bytes()) else {
                error!("unable to get UTF8 from attribute name node");
                continue;
            };

            if !attr_state.data_trunk && attr_name_str == "data-trunk" {
                attr_state.data_trunk = true;
            }

            let Some(attr_val) = ch.named_child(1).and_then(|c| {
                if c.kind() == "attribute_value" {
                    Some(c)
                } else if c.kind() == "quoted_attribute_value" {
                    c.named_child(0).filter(|c| c.kind() == "attribute_value")
                } else {
                    None
                }
            }) else {
                error!(
                    "cant get attr val child for = {:?}",
                    ch.utf8_text(s.as_bytes())
                );
                continue;
            };

            let Ok(attr_val_str) = attr_val.utf8_text(s.as_bytes()) else {
                error!("unable to get UTF8 from attribute value node");
                continue;
            };

            if attr_name_str == "rel" {
                attr_state.rel = AssetType::from_str(attr_val_str).ok();
            }
        }

        Some(attr_state)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum TagName {
    #[default]
    Unknown,
    Link,
    Script,
}

impl From<&str> for TagName {
    fn from(s: &str) -> Self {
        match s {
            "link" => Self::Link,
            "script" => Self::Script,
            _ => Self::Unknown,
        }
    }
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
