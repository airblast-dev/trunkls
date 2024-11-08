use crate::{bulk_struct, load_md};

bulk_struct! {Html, JS, MJS, Module, Css, Svg}
load_md! {Html, "html", "html"}
load_md! {Css, "css", "css"}
load_md! {Svg, "svg", "svg"}
load_md! {JS, "js", "js"}
load_md! {MJS, "mjs", "mjs"}
load_md! {Module, "module", "module"}
