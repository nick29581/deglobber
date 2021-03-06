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
extern crate csv;

use std::collections::HashMap;
use std::str::FromStr;

type GlobMap = Vec<HashMap<String, String>>;

// Get glob information from saved analysis info.
fn parse_for_globs(analysis_path: &Path,
                   callback: &Fn(&GlobMap, &Path) -> ()) {
    let mut analysis = csv::Reader::from_file(analysis_path);
    analysis = analysis.has_headers(false);
    analysis = analysis.flexible(true);
    let mut globs = vec![];
    for record in analysis.records() {
        if let Ok(record) = record {
            if record[0] == "use_glob" {
                globs.push(parse_record(record));
            }
        }
    }

    callback(&globs, &analysis_path.dir_path())
}

// Parse a CSV record of key,value pairs into a HashMap
fn parse_record(record: Vec<String>) -> HashMap<String, String> {
    let mut iter = record.into_iter();
    let kind = iter.next().unwrap();
    assert!(&kind[] == "use_glob");
    let mut result = HashMap::new();
    while let Some(r) = iter.next() {
        result.insert(r, iter.next().unwrap());
    }
    result
}

// Print the expansion of globs.
fn show(glob_map: &GlobMap, _: &Path) {
    for glob in glob_map.iter() {
        let mut names = glob["value".to_string()].clone();
        if names.contains(",") {
            names = format!("{{{}}}", names);
        }
        println!("{}:{} -> `{}`", glob["file_name".to_string()], glob["file_line".to_string()], names);
    }
}

// Replace globs with non-glob imports.
fn replace(glob_map: &GlobMap, root_path: &Path) {
    let mut changes = vec![];
    for glob in glob_map.iter() {
        let mut names = glob["value".to_string()].clone();
        if names.contains(",") {
            names = format!("{{{}}}", names);
        }

        let mut path = Path::new(&glob["file_name".to_string()][]);
        if !path.is_absolute() {
            path = root_path.join(path);
        }

        let change = reprint::Change::new(
            path,
            FromStr::from_str(&glob["extent_start_bytes".to_string()][]).unwrap(),
            FromStr::from_str(&glob["extent_end_bytes".to_string()][]).unwrap(),
            names);
        changes.push(change);
    }

    reprint::reprint(changes);
}

fn main() {
    // TODO use args for this (see #1)
    let analysis_path = Path::new("/home/ncameron/deglobber/data/hello.csv");
    // FIXME(#5) Should be user specified whether to show or replace.
    parse_for_globs(&analysis_path, &replace);
}
