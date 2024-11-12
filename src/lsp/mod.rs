mod completions;
pub mod docs;
mod document;
mod hover;
pub mod queries;

use anyhow::Context;
use completions::completions;
use document::DOCUMENTS;
use hover::hover;
use lsp_server::{Connection, Message, Response};
use lsp_types::{
    notification::{
        DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument, Notification,
    },
    request::{Completion, HoverRequest, Request},
    CompletionParams, DidChangeTextDocumentParams, DidCloseTextDocumentParams,
    DidOpenTextDocumentParams, HoverParams, TextDocumentPositionParams, Uri,
};
use texter::change::{Change, GridIndex};
use tracing::{error, warn};
use tree_sitter::Parser;

use crate::init::TextFn;

pub fn main_loop(text_fn: TextFn, con: Connection) -> anyhow::Result<()> {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_html::LANGUAGE.into())?;
    for msg in con.receiver {
        error!("Received message -> {:?}", msg);
        match msg {
            Message::Notification(noti) => handle_notification(&mut parser, text_fn, noti)?,
            Message::Request(req) => con
                .sender
                .send(Message::Response(handle_request(&mut parser, req)?))?,
            _ => continue,
        };
    }

    Ok(())
}

fn handle_notification(
    parser: &mut Parser,
    text_fn: TextFn,
    noti: lsp_server::Notification,
) -> anyhow::Result<()> {
    let mut docs = DOCUMENTS.lock().unwrap();
    match noti.method.as_str() {
        DidChangeTextDocument::METHOD => {
            let p: DidChangeTextDocumentParams = serde_json::from_value(noti.params)?;
            let (tree, text) = docs.get_mut(&p.text_document.uri).unwrap();
            for ch in p.content_changes.into_iter() {
                text.update(Change::from(ch), tree);
            }
        }
        DidOpenTextDocument::METHOD => {
            let p: DidOpenTextDocumentParams = serde_json::from_value(noti.params)?;
            let tree = parser
                .parse(p.text_document.text.as_bytes(), None)
                .context("Tree not returned during parsing")?;
            docs.insert(p.text_document.uri, (tree, text_fn(p.text_document.text)));
        }
        DidCloseTextDocument::METHOD => {
            let p: DidCloseTextDocumentParams = serde_json::from_value(noti.params)?;
            if docs.remove(&p.text_document.uri).is_none() {
                warn!("Closed unregistered document.")
            }
        }
        method => warn!("Unsupported notification recieved -> {}", method),
    };

    Ok(())
}

fn handle_request(parser: &mut Parser, req: lsp_server::Request) -> anyhow::Result<Response> {
    let mut docs = DOCUMENTS.lock().unwrap();
    match req.method.as_str() {
        Completion::METHOD => {
            let p: CompletionParams = serde_json::from_value(req.params)?;
            let (mut pos, uri): (GridIndex, Uri) = {
                let CompletionParams {
                    text_document_position,
                    ..
                } = p;

                (
                    text_document_position.position.into(),
                    text_document_position.text_document.uri,
                )
            };

            let (tree, text) = docs.get_mut(&uri).expect("unknown doc");
            *tree = parser.parse(text.text.as_str(), Some(tree)).unwrap();
            error!("pre_pos={:?}", pos);
            pos.normalize(text);
            error!("post_pos={:?}", pos);
            return Ok(Response::new_ok(
                req.id,
                completions(pos, tree.root_node(), text),
            ));
        }
        HoverRequest::METHOD => {
            let p: HoverParams = serde_json::from_value(req.params)?;
            let TextDocumentPositionParams {
                text_document: id,
                position: pos,
            } = p.text_document_position_params;
            let (tree, text) = docs.get_mut(&id.uri).unwrap();
            let mut pos = GridIndex::from(pos);
            pos.normalize(text);
            return Ok(Response::new_ok(req.id, hover(pos, tree.root_node(), text)));
        }
        _ => {}
    }

    Ok(Response::new_ok(req.id, None::<String>))
}
