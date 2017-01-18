//! Source preprocessing

//TODO: Rewrite this whole thing with flexibility in mind, such as code reuse, defines, include directories, and so forth.

use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::io;
//use std::rc::Rc;
//use std::borrow::Cow;
use std::collections::HashMap;

use regex::*;

lazy_static! {
    static ref INCLUDE_DIRECTIVE_RE: Regex = {
        /// Supports both the `#include "file"` and `#pragma include "file"` styles
        RegexBuilder::new("^\\s*#\\s*(?:pragma\\s+)?include\\s+\"(.*?)\"")
            .multi_line(true)
            .unicode(true)
            .build()
            .unwrap()
    };
}

/// Processed source code
#[derive(Debug)]
pub struct IncludeResult {
    /// Final source
    pub source: String,
}

//TODO: Build position map
fn real_include(path: &Path, mut cache: &mut HashMap<PathBuf, String>) -> Result<IncludeResult, io::Error> {
    let mut file: File = try!(File::open(path));

    let mut source = String::new();

    try!(file.read_to_string(&mut source));

    let mut err: Option<io::Error> = None;

    source = INCLUDE_DIRECTIVE_RE.replace_all(&source, |captures: &Captures| -> String {
        if err.is_none() {
            //let pos = captures.pos(1).unwrap();
            let included_path = captures.get(1).unwrap().as_str();

            let parent = path.parent().unwrap_or_else(|| Path::new("./"));

            let full_path = parent.join(included_path);

            if let Some(cached) = cache.get(&full_path) {
                return cached.clone();
            }

            match real_include(full_path.as_path(), &mut cache) {
                Err(e) => {
                    err = Some(e);
                }
                Ok(IncludeResult { source, .. }) => {
                    cache.insert(full_path, source.clone());

                    return source.into();
                }
            }
        }

        String::new()
    }).into();

    if let Some(e) = err {
        Err(e)
    } else {
        Ok(IncludeResult {
            source: source,
        })
    }
}

/// TODO
pub fn include<P: AsRef<Path>>(path: P) -> Result<IncludeResult, io::Error> {
    let mut cache = HashMap::new();

    real_include(path.as_ref(), &mut cache)
}