use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::io;
use std::rc::Rc;
use std::borrow::Cow;
use std::collections::HashMap;

use regex::*;

lazy_static! {
    static ref INCLUDE_DIRECTIVE_RE: Regex = {
        /// Supports both the `#include "file"` and `#pragma include "file"` styles
        RegexBuilder::new("^\\s*#\\s*(?:pragma\\s+)?include\\s+\"(.*?)\"")
            .multi_line(true)
            .unicode(true)
            .compile()
            .unwrap()
    };
}

struct RcReplacer<F>(F) where F: FnMut(&Captures) -> Rc<String>;

impl<F> Replacer for RcReplacer<F> where F: FnMut(&Captures) -> Rc<String> {
    fn reg_replace<'a>(&'a mut self, caps: &Captures) -> Cow<'a, str> {
       (*(self.0)(caps)).clone().into()
    }
}

#[derive(Debug)]
pub struct IncludeResult {
    pub source: Rc<String>,
    //pub position_map: Vec<(usize, Rc<String>)>
}

//impl IncludeResult {
//    pub fn get_file_at(pos: usize) -> Option<Rc<String>> {
//        None
//    }
//}

//TODO: Build position map
fn real_include(path: &Path, mut cache: &mut HashMap<PathBuf, Rc<String>>) -> Result<IncludeResult, io::Error> {
    let mut file: File = try!(File::open(path));

    let mut source = String::new();

    try!(file.read_to_string(&mut source));

    let mut err: Option<io::Error> = None;

    source = INCLUDE_DIRECTIVE_RE.replace_all(&source, RcReplacer(|captures: &Captures| -> Rc<String> {
        if err.is_none() {
            //let pos = captures.pos(1).unwrap();
            let included_path = captures.at(1).unwrap().to_string();

            let parent = path.parent().unwrap_or(Path::new("./"));

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

                    return source;
                }
            }
        }

        Rc::new(String::new())
    }));

    if let Some(e) = err {
        Err(e)
    } else {
        Ok(IncludeResult {
            source: Rc::new(source),
            //position_map: vec![]
        })
    }
}

pub fn include<P: AsRef<Path>>(path: P) -> Result<IncludeResult, io::Error> {
    let mut cache = HashMap::new();

    real_include(path.as_ref(), &mut cache)
}