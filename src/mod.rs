// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(unstable)]
#![feature(box_syntax)]

extern crate reprint;
extern crate rustc;
extern crate rustc_driver;
extern crate rustc_resolve;
extern crate syntax;

use rustc::middle::ty::GlobMap;
use rustc::session::build_session;
use rustc::session::config::{self, Input};
use rustc_driver::driver;
use syntax::diagnostics::registry::Registry;
use syntax::ast_map;
use syntax::codemap::{CodeMap, Span};

// Use librustc to get glob information about a file
fn parse_for_globs(path: &Path, callback: &Fn(&Path, &GlobMap, &ast_map::Map, &CodeMap) -> ()) {
    let input = Input::File(path.clone());

    let mut opts = config::basic_options();
    // FIXME(#2) don't hardcode this
    opts.maybe_sysroot = Some(Path::new("/home/ncameron/rust3/x86_64-unknown-linux-gnu/stage2"));
    let reg = Registry::new(&rustc::DIAGNOSTICS);
    let sess = build_session(opts, Some(path.clone()), reg);
    let cfg = config::build_configuration(&sess);
    let mut control = driver::CompileController::basic();
    // FIXME: We can also stop after name resolution, rather than do the full analysis.
    control.after_analysis.stop = true;
    control.after_analysis.callback = box |state| {
        let analysis = state.analysis.unwrap();
        let glob_map = analysis.glob_map.as_ref().unwrap();
        let ast_map = &analysis.ty_cx.map;
        callback(path, glob_map, ast_map, state.session.codemap())
    };
    control.make_glob_map = rustc_resolve::MakeGlobMap::Yes;

    driver::compile_input(sess, cfg, &input, &None, &None, None, control);
}

// Print the expansion of globs.
fn show(path: &Path, glob_map: &GlobMap, ast_map: &ast_map::Map, codemap: &CodeMap) {
    for node_id in glob_map.keys() {
        let names: Vec<_> = glob_map[*node_id].iter().map(|n| n.as_str()).collect();
        let mut names_str = names.connect(", ");
        if names.len() > 1 {
            names_str = format!("{{{}}}", names_str);
        }
        let node = ast_map.expect_view_item(*node_id);
        let line = codemap.lookup_char_pos(node.span.hi).line;
        println!("{:?}:{} -> `{}`", path, line, names_str);
    }
}

// Replace globs with non-glob imports.
fn replace(path: &Path, glob_map: &GlobMap, ast_map: &ast_map::Map, codemap: &CodeMap) {
    let mut changes = vec![];
    for node_id in glob_map.keys() {
        let names: Vec<_> = glob_map[*node_id].iter().map(|n| n.as_str()).collect();
        let mut names_str = names.connect(", ");
        if names.len() > 1 {
            names_str = format!("{{{}}}", names_str);
        }
        let node = ast_map.expect_view_item(*node_id);
        let span = node.span;

        match find_glob_in_span(span, codemap) {
            Some(loc) => {
                let change = reprint::Change::new(loc, loc + 1, names_str);
                changes.push(change);
            }
            None => {
                // FIXME(#3): could handle errors better
                println!("Unexpected: couldn't find glob in import `{}`",
                         codemap.span_to_snippet(span).unwrap_or("<bad span>".to_string()));
            }
        }
    }

    reprint::reprint(path, changes);
}

// Sadly the span is for the whole view item, so we need to find the `*` within it.
fn find_glob_in_span(span: Span, codemap: &CodeMap) -> Option<u32> {
    let import = match codemap.span_to_snippet(span) {
        Some(s) => s,
        None => return None,
    };
    import.find_str("*").map(|x| x as u32 + span.lo.0)
}

fn main() {
    // TODO use args for this (see #1)
    let path = Path::new("/home/ncameron/deglobber/data/hello.rs");
    // FIXME(#5) Should be user specified whether to show or replace.
    parse_for_globs(&path, &replace);
}
