use tree_sitter::Node;

pub fn is_attr_name_completion(kind: &str) -> bool {
    matches!(
        kind,
        "self_closing_tag" | "start_tag" | "attribute_name" | "attribute"
    )
}

pub fn is_attr_value_completion(kind: &str) -> bool {
    matches!(kind, "quoted_attribute_value" | "attribute_value")
}

pub fn find_attr(n: Node) -> Option<Node> {
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

pub fn attr_to_attr_val(n: Node) -> Option<Node> {
    let attr_val = n.named_child(1)?;
    match attr_val.kind() {
        "attribute_value" => Some(attr_val),
        "quoted_attribute_value" => attr_val
            .named_child(0)
            .filter(|attr_val| attr_val.kind() == "attribute_value"),
        _ => None,
    }
}

pub fn find_elem(mut n: Node) -> Option<Node> {
    while !matches!(n.kind(), "start_tag" | "self_closing_tag") {
        n = n.parent()?;
    }

    Some(n)
}
