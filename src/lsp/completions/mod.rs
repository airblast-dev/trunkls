use lsp_types::{
    CompletionItem, CompletionItemKind, CompletionResponse, Documentation, InsertTextFormat,
    MarkupContent, MarkupKind,
};
use streaming_iterator::{IntoStreamingIterator, StreamingIterator};
use tree_sitter::{Node, QueryCursor};

use crate::{
    attr_state::{AssetType, TagName, TrunkAttrState},
    texter::{change::GridIndex, core::text::Text},
    utils::{find_attr, is_attr_name_completion, is_attr_value_completion},
};

use super::{
    docs::{self, Script, ValueRequirment},
    queries::attributes::TRUNK_ATTRS,
};

impl TrunkAttrState {
    fn to_completion(&self, s: &str, original: Node) -> Option<CompletionResponse> {
        match self.tag_name {
            TagName::Link => self.link_to_completion(s, original),
            TagName::Script => self.script_to_completion(s, original),
            TagName::Unknown => None,
        }
    }

    fn script_to_completion(&self, s: &str, original: Node) -> Option<CompletionResponse> {
        if self.is_data_trunk_attr(s, original) {
            return Some(CompletionResponse::Array(vec![
                docs::DataTrunk::completion(),
            ]));
        }

        let attr_node = find_attr(original)?;

        if is_attr_name_completion(original.kind()) {
            let tag = attr_node.parent()?;
            let mut cursor = tag.walk();
            let attr_names: Vec<&str> = tag
                .named_children(&mut cursor)
                .filter_map(|n| {
                    if n.kind() != "attribute" {
                        return None;
                    }

                    n.named_child(0)
                        .filter(|n| n.kind() == "attribute_name")?
                        .utf8_text(s.as_bytes())
                        .ok()
                })
                .collect();
            return self.complete_script_attr_name(s, attr_names, original);
        }

        if is_attr_value_completion(original.kind()) {
            return self.complete_script_attr_value(s, attr_node.named_child(0)?);
        }

        None
    }

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
                .named_children(&mut cursor)
                .filter_map(|n| {
                    if n.kind() != "attribute" {
                        return None;
                    }

                    n.named_child(0)
                        .filter(|n| n.kind() == "attribute_name")?
                        .utf8_text(s.as_bytes())
                        .ok()
                })
                .collect();
            return self.complete_link_attr_name(s, attr_names, attr_name_node, asset_type);
        };

        if is_attr_value_completion(original.kind()) {
            return self.complete_link_attr_value(s, attr_name_node, asset_type);
        }

        None
    }

    /// Accepts a node with a kind of "attribute_name".
    pub fn is_data_trunk_attr(&self, s: &str, n: Node) -> bool {
        if self.data_trunk {
            return false;
        }
        n.kind() == "attribute_name"
            && n.utf8_text(s.as_bytes())
                .is_ok_and(|s| s.starts_with("data-"))
    }

    /// Accepts a node with a kind of "attribute_name".
    pub fn complete_link_attr_name(
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
            .filter_map(|(attr, doc, req): &(&str, &str, ValueRequirment)| {
                if (!attr.starts_with(attr_name_str)) || attr_names.contains(attr) {
                    return None;
                }

                let insert_kind;
                let kind;
                let f_attr;
                if req.must_have_value() {
                    kind = Some(CompletionItemKind::SNIPPET);
                    insert_kind = InsertTextFormat::SNIPPET;
                    f_attr = String::from_iter([attr, "=\"$0\""]);
                } else {
                    kind = None;
                    insert_kind = InsertTextFormat::PLAIN_TEXT;
                    f_attr = attr.to_string();
                };

                Some(CompletionItem {
                    kind,
                    label: f_attr,
                    documentation: Some(Documentation::MarkupContent(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: doc.to_string(),
                    })),
                    insert_text_format: Some(insert_kind),
                    ..Default::default()
                })
            })
            .collect();

        Some(CompletionResponse::Array(comps))
    }

    /// Accepts a node with a kind of "attribute_value".
    fn complete_link_attr_value(
        &self,
        s: &str,
        attr_name_node: Node,
        asset_type: AssetType,
    ) -> Option<CompletionResponse> {
        let info = asset_type.to_info();
        let attr_name_str = attr_name_node.utf8_text(s.as_bytes()).ok()?;

        let comps = info
            .iter()
            .filter(|info| info.0 == attr_name_str)
            .filter_map(
                |(attr_name, _, req)| match (req, attr_name_str == *attr_name) {
                    (ValueRequirment::Values(_, accepts), true) => {
                        Some(accepts.iter().map(|(val, doc)| CompletionItem {
                            documentation: Some(Documentation::MarkupContent(MarkupContent {
                                kind: MarkupKind::Markdown,
                                value: doc.to_string(),
                            })),
                            label: val.to_string(),
                            ..Default::default()
                        }))
                    }
                    _ => None,
                },
            )
            .flatten();
        Some(CompletionResponse::Array(comps.collect()))
    }

    fn complete_script_attr_name(
        &self,
        s: &str,
        attr_names: Vec<&str>,
        attr_name_node: Node,
    ) -> Option<CompletionResponse> {
        let attr_name_str = attr_name_node.utf8_text(s.as_bytes()).ok()?;
        let comps = Script::ASSET_ATTRS
            .iter()
            .filter(|(attr_name, _, _)| {
                attr_name.starts_with(attr_name_str) && !attr_names.contains(attr_name)
            })
            .map(|(attr_name, doc, _)| CompletionItem {
                label: attr_name.to_string(),
                documentation: Some(Documentation::MarkupContent(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: doc.to_string(),
                })),
                ..Default::default()
            })
            .collect();

        Some(CompletionResponse::Array(comps))
    }

    fn complete_script_attr_value(
        &self,
        s: &str,
        attr_name_node: Node,
    ) -> Option<CompletionResponse> {
        let attr_name_str = attr_name_node.utf8_text(s.as_bytes()).ok()?;
        let comps = Script::ASSET_ATTRS
            .iter()
            .find(|(attr_name, _, _)| attr_name_str == *attr_name)
            .and_then(|(_, _, values)| match values {
                ValueRequirment::Values(_, vals) => Some(
                    vals.iter()
                        .map(|(val, doc)| CompletionItem {
                            label: val.to_string(),
                            documentation: Some(Documentation::MarkupContent(MarkupContent {
                                value: doc.to_string(),
                                kind: MarkupKind::Markdown,
                            })),
                            ..Default::default()
                        })
                        .collect(),
                ),
                _ => None,
            })?;
        Some(CompletionResponse::Array(comps))
    }
}

pub fn completions(pos: GridIndex, n: Node, text: &Text) -> Option<CompletionResponse> {
    let s = text.text.as_str();
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
    let attr_state = TrunkAttrState::from_elem_items(s, children)?;

    attr_state.to_completion(s, in_pos)
}
