use std::sync::{LazyLock, Mutex};

use fxhash::FxHashMap;
use lsp_types::Uri;
use texter::core::text::Text;
use tree_sitter::Tree;

type Documents = FxHashMap<Uri, (Tree, Text)>;
pub static DOCUMENTS: LazyLock<Mutex<Documents>> = LazyLock::new(Mutex::default);
