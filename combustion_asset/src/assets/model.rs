//! Model asset

use std::io::prelude::*;
use std::ops::{Deref, DerefMut};
use std::ascii::AsciiExt;
use std::rc::Rc;
use std::cell::RefCell;

use capnp::serialize_packed;
use capnp::message::ReaderOptions;

use protocols::traits::Storage;
use protocols::model::{protocol, EXTENSION};
use protocols::model::data::{Model, Node};
use protocols::model::storage::ModelSaveArgs;

use assimp::{self, Scene};

use ::cache::AssetHashMapCache;
use ::error::{AssetResult, AssetError};
use ::asset::{Asset, AssetMedium, AssetQuery};

pub type AssimpSceneCache<'a> = AssetHashMapCache<'a, String, Scene<'a>>;

#[derive(Debug, Clone, Copy)]
pub enum ModelAssetQuery<'a> {
    SupportedExtension(&'a str),
}

impl<'a> AssetQuery for ModelAssetQuery<'a> {
    type Arguments = ModelAssetQuery<'a>;
    type Result = bool;
}

#[derive(Clone)]
pub struct ModelLoadArgs<'a> {
    pub scene_cache: Rc<RefCell<AssimpSceneCache<'a>>>,
}

/// Model Asset
pub struct ModelAsset(Model);

impl<'a> Asset<'a> for ModelAsset {
    type LoadArgs = ModelLoadArgs<'a>;
    type SaveArgs = ();

    type Query = ModelAssetQuery<'a>;

    fn query(query: ModelAssetQuery<'a>) -> AssetResult<bool> {
        match query {
            ModelAssetQuery::SupportedExtension(ext) => {
                Ok(ext == EXTENSION || assimp::formats::IMPORT_EXTENSIONS.contains_key(ext))
            },
            _ => unimplemented!()
        }
    }

    fn load<R: BufRead + Seek>(reader: R, medium: AssetMedium<'a>, mut args: ModelLoadArgs<'a>) -> AssetResult<ModelAsset> {
        if let AssetMedium::File(path) = medium {
            if let Some(ext) = path.extension() {
                let ext = try_throw!(ext.to_str().ok_or(AssetError::InvalidValue)).to_ascii_lowercase();

                if ext == EXTENSION {} else {}
            }
        }

        throw!(AssetError::UnsupportedMedium)
    }

    fn save<W: Write>(&self, _writer: W, _medium: AssetMedium<'a>, _: ()) -> AssetResult<()> {
        unimplemented!()
    }
}