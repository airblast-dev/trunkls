use crate::{bulk_struct, load_md};

bulk_struct! {Html, JS, MJS, Module, Css, Svg, Type, Href}
load_md! {Type, "type", "type"}
load_md! {Html, "types/html", "html"}
load_md! {Css, "types/css", "css"}
load_md! {Svg, "types/svg", "svg"}
load_md! {JS, "types/js", "js"}
load_md! {MJS, "types/mjs", "mjs"}
load_md! {Module, "types/module", "module"}
load_md! { Href, "href", "href"}
