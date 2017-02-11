//! Model asset

use std::ops::{Deref, DerefMut};
use std::ascii::AsciiExt;
use std::sync::{Arc, RwLock};
use std::io::BufReader;

use capnp::serialize_packed;
use capnp::message::ReaderOptions;

use protocols::traits::Storage;
use protocols::model::{protocol, EXTENSION};
use protocols::model::data::Model;
use protocols::model::storage;

use assimp::{self, Scene};

use ::cache::AssetHashMapCache;
use ::error::{AssetResult, AssetError};
use ::asset::{Asset, AssetMedium, AssetQuery};

/// Cache object for Assimp scenes
pub type AssimpSceneCache<'a> = AssetHashMapCache<'a, String, Scene<'a>>;

/// Model Asset queries
#[derive(Debug, Clone, Copy)]
pub enum ModelAssetQuery<'a> {
    /// Check if a file extension for a model file is supported
    SupportedExtension(&'a str),
}

impl<'a> AssetQuery for ModelAssetQuery<'a> {
    type Arguments = ModelAssetQuery<'a>;
    type Result = bool;
}

/// Arguments for loading models
#[derive(Default, Clone)]
pub struct ModelLoadArgs<'a> {
    /// Assimp scene cache
    pub scene_cache: Arc<RwLock<AssimpSceneCache<'a>>>,
}

unsafe impl<'a> Send for ModelLoadArgs<'a> {}

unsafe impl<'a> Sync for ModelLoadArgs<'a> {}

/// Arguments for model save routines
#[derive(Debug, Default, Clone)]
pub struct ModelSaveArgs {
    /// Arguments for the storage routines
    pub storage_args: storage::ModelSaveArgs,
}

/// Model Asset
pub struct ModelAsset(Model);

impl<'a> Asset<'a> for ModelAsset {
    type LoadArgs = ModelLoadArgs<'a>;
    type SaveArgs = ModelSaveArgs;

    type Query = ModelAssetQuery<'a>;

    fn query(query: ModelAssetQuery<'a>) -> AssetResult<bool> {
        match query {
            ModelAssetQuery::SupportedExtension(ext) => {
                Ok(ext == EXTENSION || assimp::formats::is_extension_supported(ext))
            },
        }
    }

    fn load(medium: AssetMedium<'a>, _ /*TODO*/: ModelLoadArgs<'a>) -> AssetResult<ModelAsset> {
        if let AssetMedium::File(path, vfs) = medium {
            if let Some(ext) = path.extension() {
                let ext = try_throw!(ext.to_str().ok_or(AssetError::InvalidValue)).to_ascii_lowercase();

                if ext == EXTENSION {
                    let mut reader = BufReader::new(try_throw!(vfs.open(path)));

                    let message_reader = try_throw!(serialize_packed::read_message(&mut reader, ReaderOptions {
                        traversal_limit_in_words: u64::max_value(),
                        nesting_limit: 1024,
                    }));

                    let model_reader = try_throw!(message_reader.get_root::<protocol::model::Reader>());

                    let model = try_rethrow!(Model::load_from_reader(model_reader));

                    return Ok(ModelAsset(model));
                } else {
                    // Use custom IO for Assimp so it can use the virtual filesystem to interact with data
                    let mut io = assimp::io::CustomIO::callback(move |path| vfs.open(path));

                    let scene = try_rethrow!(assimp::Scene::import_from(path, None, &mut io));

                    let model = try_rethrow!(::external::assimp::scene_to_model(scene));

                    return Ok(ModelAsset(model));
                }
            }
        }

        throw!(AssetError::UnsupportedMedium)
    }

    fn save(&self, medium: AssetMedium<'a>, args: ModelSaveArgs) -> AssetResult<()> {
        if let AssetMedium::File(path, vfs) = medium {
            if let Some(ext) = path.extension() {
                let ext = try_throw!(ext.to_str().ok_or(AssetError::InvalidValue)).to_ascii_lowercase();

                let mut writer = try_throw!(vfs.open_or_create(path));

                if ext == EXTENSION {
                    let mut message = ::capnp::message::Builder::new_default();

                    {
                        let model_builder = message.init_root::<protocol::model::Builder>();

                        try_rethrow!(self.0.save_to_builder_args(model_builder, args.storage_args));
                    }

                    try_throw!(serialize_packed::write_message(&mut writer, &message));

                    return Ok(());
                } else {
                    throw!(AssetError::Unimplemented("Non-combustion model exporting"));
                }
            }
        }

        throw!(AssetError::UnsupportedMedium)
    }
}

impl Deref for ModelAsset {
    type Target = Model;

    fn deref(&self) -> &Model {
        &self.0
    }
}

impl DerefMut for ModelAsset {
    fn deref_mut(&mut self) -> &mut Model {
        &mut self.0
    }
}