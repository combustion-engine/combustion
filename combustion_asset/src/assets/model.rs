//! Model asset

use std::io::prelude::*;
use std::ops::{Deref, DerefMut};
use std::ascii::AsciiExt;
use std::sync::{Arc, RwLock};

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
    pub scene_cache: Arc<RwLock<AssimpSceneCache<'a>>>,
}

unsafe impl<'a> Send for ModelLoadArgs<'a> {}

unsafe impl<'a> Sync for ModelLoadArgs<'a> {}

/// Model Asset
pub struct ModelAsset(Model);

impl<'a> Asset<'a> for ModelAsset {
    type LoadArgs = ModelLoadArgs<'a>;
    type SaveArgs = ();

    type Query = ModelAssetQuery<'a>;

    fn query(query: ModelAssetQuery<'a>) -> AssetResult<bool> {
        match query {
            ModelAssetQuery::SupportedExtension(ext) => {
                Ok(ext == EXTENSION || assimp::formats::is_extension_supported(ext))
            },
            _ => unimplemented!()
        }
    }

    fn load<R: BufRead + Seek, T: AsMut<R>>(mut reader: T, medium: AssetMedium<'a>, mut args: ModelLoadArgs<'a>) -> AssetResult<ModelAsset> {
        if let AssetMedium::File(path) = medium {
            if let Some(ext) = path.extension() {
                let ext = try_throw!(ext.to_str().ok_or(AssetError::InvalidValue)).to_ascii_lowercase();

                if ext == EXTENSION {
                    let message_reader = try_throw!(serialize_packed::read_message(reader.as_mut(), ReaderOptions {
                        traversal_limit_in_words: u64::max_value(),
                        nesting_limit: 1024,
                    }));

                    let model_reader = try_throw!(message_reader.get_root::<protocol::model::Reader>());

                    let model = try_rethrow!(Model::load_from_reader(model_reader));

                    return Ok(ModelAsset(model));
                } else {
                    // TODO: Load up an assimp scene and convert it into a Combustion scene
                    unimplemented!()
                }
            }
        }

        throw!(AssetError::UnsupportedMedium)
    }

    fn save<W: Write, T: AsMut<W>>(&self, _writer: T, _medium: AssetMedium<'a>, _: ()) -> AssetResult<()> {
        unimplemented!()
    }
}