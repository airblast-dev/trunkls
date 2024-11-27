use lsp_types::{
    CompletionOptions, HoverProviderCapability, InitializeParams, InitializeResult,
    PositionEncodingKind, ServerCapabilities, ServerInfo, TextDocumentSyncCapability,
    TextDocumentSyncKind,
};

use crate::texter::core::text::Text;

pub type TextFn = fn(String) -> Text;

pub fn initialize_result(p: &InitializeParams) -> (TextFn, InitializeResult) {
    let pos_encoding = p
        .capabilities
        .general
        .as_ref()
        .and_then(|g| g.position_encodings.as_deref());

    let (t_fn, enc) = decide_encoding(pos_encoding);

    let res = InitializeResult {
        capabilities: ServerCapabilities {
            position_encoding: Some(enc),
            text_document_sync: Some(TextDocumentSyncCapability::Kind(
                TextDocumentSyncKind::INCREMENTAL,
            )),
            hover_provider: Some(HoverProviderCapability::Simple(true)),
            completion_provider: Some(CompletionOptions {
                trigger_characters: Some(vec!["-".to_string(), "\"".to_string(), " ".to_string()]),
                ..Default::default()
            }),
            ..Default::default()
        },
        server_info: Some(ServerInfo {
            name: String::from("trunkls"),
            version: env!("CARGO_PKG_VERSION").to_string().into(),
        }),
    };
    (t_fn, res)
}

fn decide_encoding(encs: Option<&[PositionEncodingKind]>) -> (TextFn, PositionEncodingKind) {
    const DEFAULT: (TextFn, PositionEncodingKind) = (Text::new_utf16, PositionEncodingKind::UTF16);
    let Some(encs) = encs else {
        return DEFAULT;
    };

    for enc in encs {
        if *enc == PositionEncodingKind::UTF16 {
            return (Text::new_utf16, enc.clone());
        } else if *enc == PositionEncodingKind::UTF8 {
            return (Text::new, enc.clone());
        }
    }

    DEFAULT
}
