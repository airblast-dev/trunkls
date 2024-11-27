use lsp_types::{Hover, HoverContents, MarkupContent, MarkupKind, Range};
use texter::{change::GridIndex, core::text::Text};
use tracing::{instrument, trace};
use tree_sitter::Node;

use crate::{
    attr_state::TrunkAttrState,
    utils::{find_attr, find_elem},
};

use super::docs::{DataTrunk, ValueRequirment};

#[instrument(level = "trace")]
pub fn hover(pos: GridIndex, n: Node, text: &Text) -> Option<Hover> {
    let in_pos = n.named_descendant_for_point_range(pos.into(), pos.into())?;

    let elem = find_elem(in_pos)?;

    let mut cursor = elem.walk();
    let attr_state =
        TrunkAttrState::from_elem_items(text.text.as_str(), elem.named_children(&mut cursor))?;

    match in_pos.kind() {
        "attribute_name" => attr_state.hover_attribute_name(text, in_pos),
        "attribute_value" => attr_state.hover_attribute_value(text, in_pos),
        _ => None,
    }
}
impl TrunkAttrState {
    #[instrument(skip(text), level = "trace")]
    fn hover_attribute_name(&self, text: &Text, in_pos: Node) -> Option<Hover> {
        assert_eq!(in_pos.kind(), "attribute_name");

        if in_pos.utf8_text(text.text.as_bytes()).ok()? == DataTrunk::DOC_OF {
            return Some(Hover {
                contents: DataTrunk::hover_contents(),
                range: None,
            });
        }

        let attr_name_str = in_pos.utf8_text(text.text.as_bytes()).ok()?;
        trace!("attr_name_str={:?}", attr_name_str);

        trace!("Geting attribute value for rel attribute");
        let rel = self.rel?;
        trace!("Found asset type = {:?}", rel);

        trace!("Finding asset specific hover");
        let hover = rel
            .to_info()
            .iter()
            .find(|(attr_name, _, _)| *attr_name == attr_name_str)
            .map(|(a, b, _)| (a, b))?;

        trace!("Found asset specific hover");
        let mut start_pos = GridIndex::from(in_pos.start_position());
        start_pos.denormalize(text);
        let mut end_pos = GridIndex::from(in_pos.end_position());
        end_pos.denormalize(text);
        Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: hover.1.to_string(),
            }),
            range: Some(Range {
                start: start_pos.into(),
                end: end_pos.into(),
            }),
        })
    }

    #[instrument(skip(text), level = "trace")]
    fn hover_attribute_value(&self, text: &Text, in_pos: Node) -> Option<Hover> {
        assert_eq!(in_pos.kind(), "attribute_value");
        let attr_node = find_attr(in_pos)?;
        let attr_name_node = attr_node
            .named_child(0)
            .filter(|n| n.kind() == "attribute_name")?;
        let attr_name_str = attr_name_node.utf8_text(text.text.as_bytes()).ok()?;
        let attr_val_str = in_pos.utf8_text(text.text.as_bytes()).ok()?;
        let rel = self.rel?;
        let (_, _, req) = rel
            .to_info()
            .iter()
            .find(|(attr_name, _, _)| *attr_name == attr_name_str)?;
        let (_, val_doc) = match req {
            ValueRequirment::Values(_, vals) => {
                *vals.iter().find(|(val, _)| *val == attr_val_str)?
            }
            _ => return None,
        };

        let mut start_pos = GridIndex::from(in_pos.start_position());
        start_pos.denormalize(text);
        let mut end_pos = GridIndex::from(in_pos.end_position());
        end_pos.denormalize(text);

        Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: val_doc.to_string(),
            }),
            range: Some(Range {
                start: start_pos.into(),
                end: end_pos.into(),
            }),
        })
    }
}
