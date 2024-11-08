use std::str::FromStr;

use lsp_types::{CompletionItem, CompletionResponse, Documentation, MarkupContent, MarkupKind};
use streaming_iterator::{IntoStreamingIterator, StreamingIterator};
use texter::{change::GridIndex, core::text::Text};
use tracing::error;
use tree_sitter::{Node, QueryCursor};

use super::{
    docs::{self, RequiresValue},
    queries::attributes::TRUNK_ATTRS,
};

#[derive(Clone, Debug, Default)]
struct TrunkAttrState {
    // Wether a data-trunk attribute is already present.
    data_trunk: bool,
    // If an asset type is currently selected.
    //
    // for example `rel=""` is `None` but `rel="css"` is `Some(AssetType::Css)`
    rel: Option<AssetType>,
}

fn is_attr_name_completion(kind: &str) -> bool {
    matches!(
        kind,
        "self_closing_tag" | "start_tag" | "attribute_name" | "attribute"
    )
}

fn is_attr_value_completion(kind: &str) -> bool {
    matches!(kind, "quoted_attribute_value" | "attribute_value")
}

fn find_attr(n: Node) -> Option<Node> {
    let attr_node = match n.kind() {
        "attribute_value" => {
            let pn = n.parent()?;
            match pn.kind() {
                "quoted_attribute_value" => pn.parent().filter(|ppn| ppn.kind() == "attribute")?,
                "attribute" => pn,
                _ => return None,
            }
        }
        "quoted_attribute_value" | "attribute_name" => {
            n.parent().filter(|pn| pn.kind() == "attribute")?
        }
        "attribute" => n,
        _ => return None,
    };

    assert_eq!(attr_node.kind(), "attribute");

    Some(attr_node)
}

fn attr_to_attr_val(n: Node) -> Option<Node> {
    let attr_val = n.named_child(1)?;
    match attr_val.kind() {
        "attribute_value" => Some(attr_val),
        "quoted_attribute_value" => attr_val
            .named_child(0)
            .filter(|attr_val| attr_val.kind() == "attribute_value"),
        _ => None,
    }
}

impl TrunkAttrState {
    fn link_to_completion(&self, s: &str, original: Node) -> Option<CompletionResponse> {
        if self.is_data_trunk_attr(s, original) {
            return Some(CompletionResponse::Array(vec![
                docs::DataTrunk::completion(),
            ]));
        }

        let attr_node = find_attr(original)?;
        let attr_name_node = attr_node
            .named_child(0)
            .filter(|ann| ann.kind() == "attribute_name")?;

        if self.is_rel_val(s, attr_name_node) {
            use docs::*;
            return Some(CompletionResponse::Array(vec![
                RelRust::completion(),
                RelSass::completion(),
                RelScss::completion(),
                RelCss::completion(),
                RelTailwind::completion(),
                RelIcon::completion(),
                RelInline::completion(),
                RelCopyFile::completion(),
                RelCopyDir::completion(),
            ]));
        }

        let asset_type = self.rel?;

        if is_attr_name_completion(original.kind()) {
            let tag = attr_node.parent()?;
            let mut cursor = tag.walk();
            let attr_names: Vec<&str> = tag
                .children(&mut cursor)
                .filter_map(|n| {
                    if n.kind() != "attribute" {
                        return None;
                    }

                    n.named_child(0)?.utf8_text(s.as_bytes()).ok()
                })
                .collect();
            return self.complete_attr_name(s, attr_names, attr_name_node, asset_type);
        };

        if is_attr_value_completion(original.kind()) {
            let attr_val_str = attr_to_attr_val(attr_node)
                .and_then(|n| n.utf8_text(s.as_bytes()).ok())
                .unwrap_or_default();
            return self.complete_attr_value(s, attr_name_node, attr_val_str, asset_type);
        }

        None
    }

    /// Accepts a node with a kind of "attribute_name".
    fn is_data_trunk_attr(&self, s: &str, n: Node) -> bool {
        if self.data_trunk {
            return false;
        }
        n.kind() == "attribute_name"
            && n.utf8_text(s.as_bytes())
                .is_ok_and(|s| s.starts_with("data-"))
    }

    fn is_rel_val(&self, s: &str, n: Node) -> bool {
        if self.rel.is_some() {
            return false;
        }

        n.utf8_text(s.as_bytes()).is_ok_and(|s| s == "rel")
    }

    /// Accepts a node with a kind of "attribute_name".
    fn complete_attr_name(
        &self,
        s: &str,
        attr_names: Vec<&str>,
        attr_name_node: Node,
        asset_type: AssetType,
    ) -> Option<CompletionResponse> {
        let attr_name_str = attr_name_node.utf8_text(s.as_bytes()).ok()?;
        let comps = asset_type
            .to_info()
            .iter()
            .filter_map(|(attr, doc, req): &(&str, &str, RequiresValue)| {
                if (!attr.starts_with(attr_name_str)) || attr_names.contains(attr) {
                    return None;
                }

                let attr = if req.should_have_value() {
                    String::from_iter([attr, "=\"\""])
                } else {
                    attr.to_string()
                };

                Some(CompletionItem {
                    label: attr,
                    documentation: Some(Documentation::MarkupContent(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: doc.to_string(),
                    })),
                    ..Default::default()
                })
            })
            .collect();

        Some(CompletionResponse::Array(comps))
    }

    /// Accepts a node with a kind of "attribute_value".
    fn complete_attr_value(
        &self,
        s: &str,
        attr_name_node: Node,
        attr_val_node: &str,
        asset_type: AssetType,
    ) -> Option<CompletionResponse> {
        let info = asset_type.to_info();
        let attr_name_str = attr_name_node.utf8_text(s.as_bytes()).ok()?;

        let comps = info
            .iter()
            .filter(|info| info.0 == attr_name_str)
            .filter_map(
                |(attr_name, _, req)| match (req, attr_name_str == *attr_name) {
                    (RequiresValue::Values(accepts), true) => Some(
                        accepts
                            .iter()
                            .filter(|accepted_val| accepted_val.0.starts_with(attr_val_node))
                            .map(|(val, doc)| CompletionItem {
                                documentation: Some(Documentation::MarkupContent(MarkupContent {
                                    kind: MarkupKind::Markdown,
                                    value: doc.to_string(),
                                })),
                                label: val.to_string(),
                                ..Default::default()
                            }),
                    ),
                    _ => None,
                },
            )
            .flatten();
        Some(CompletionResponse::Array(comps.collect()))
    }
}

#[derive(Clone, Copy, Debug)]
enum AssetType {
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
    fn to_info(self) -> &'static [(&'static str, &'static str, RequiresValue)] {
        use docs::*;
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

pub fn completions(pos: GridIndex, n: Node, text: &Text) -> Option<CompletionResponse> {
    let s = text.text.as_str();
    let lang = n.language();
    let attr_id = lang.id_for_node_kind("attribute", true);
    let attr_name_id = lang.id_for_node_kind("attribute_name", true);
    let quoted_attr_value_id = lang.id_for_node_kind("quoted_attribute_value", true);
    let attr_value_id = lang.id_for_node_kind("attribute_value", true);
    let mut cursor = QueryCursor::new();
    let element_id = TRUNK_ATTRS
        .capture_names()
        .iter()
        .position(|e| *e == "element")
        .unwrap() as u32;
    let mut matches = cursor
        .matches(&TRUNK_ATTRS, n, s.as_bytes())
        .flat_map(|qm| {
            qm.captures
                .iter()
                .filter(|cap| cap.index == element_id && cap.node.end_position() > pos.into())
                .into_streaming_iter()
        });
    let current = matches.next()?;

    let prev_pos = {
        let mut pos = pos;
        pos.col = pos.col.saturating_sub(1);
        pos
    };
    let in_pos = current
        .node
        .named_descendant_for_point_range(prev_pos.into(), pos.into())?;

    // If the current position is not preceeded by a whitespace, we cannot give any attribute
    // completions so we return early.
    if s.as_bytes()[in_pos.start_byte()] != b' '
        && matches!(in_pos.kind(), "self_closing_tag" | "start_tag")
    {
        return None;
    }

    // If the end of the found node is " we shouldn't return a completion as the cursor is after a
    // quote.
    let byte_pos = text.br_indexes.row_start(pos.row) + pos.col;
    let prev_byte = s.as_bytes()[byte_pos.saturating_sub(1)];
    if matches!(prev_byte, b'\'' | b'"')
        && in_pos.kind() == "quoted_attribute_value"
        && byte_pos == in_pos.end_byte()
    {
        return None;
    }

    let mut cursor = current.node.walk();
    let children = current.node.named_children(&mut cursor);
    let mut attr_state = TrunkAttrState::default();
    for ch in children.filter(|ch| ch.kind_id() == attr_id) {
        let Some(attr_name) = ch.named_child(0).filter(|c| c.kind_id() == attr_name_id) else {
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
            if c.kind_id() == attr_value_id {
                Some(c)
            } else if c.kind_id() == quoted_attr_value_id {
                c.named_child(0).filter(|c| c.kind_id() == attr_value_id)
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

    attr_state.link_to_completion(s, in_pos)
}
