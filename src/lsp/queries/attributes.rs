use std::sync::LazyLock;

use tree_sitter::Query;

/// Query for all elements containing a `data-trunk` attribute.
pub static TRUNK_ATTRS: LazyLock<Query> = LazyLock::new(|| {
    #[rustfmt::skip]
    const QS: &str =r#"
    (_
            (tag_name) @tag.name
            (#eq? @tag.name link)
            (attribute(attribute_name) @attr.name)+
            (#any-eq? @attr.name "data-trunk")
    ) @element
"#;
    Query::new(&tree_sitter_html::LANGUAGE.into(), QS).unwrap()
});
