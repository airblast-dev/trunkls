use std::sync::{LazyLock, Mutex};

use fxhash::FxHashMap;
use lsp_types::Uri;
use tree_sitter::Tree;

use crate::texter::core::text::Text;

type Documents = FxHashMap<Uri, (Tree, Text)>;
pub static DOCUMENTS: LazyLock<Mutex<Documents>> = LazyLock::new(Mutex::default);
