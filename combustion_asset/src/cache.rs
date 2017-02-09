//! Types and traits for asset caching

use std::marker::PhantomData;
use std::hash::Hash;

use fnv::FnvHashMap;

/// Some cache type
pub trait AssetCache<'a> {
    /// The Asset type being cached
    type Asset: 'a;
}

/// Hashmap cache structure
#[derive(Debug, Clone)]
pub struct AssetHashMapCache<'a, K, T: 'a> where K: Hash + PartialEq + Eq {
    map: FnvHashMap<K, T>,
    _marker: PhantomData<&'a ()>,
}

impl<'a, K, T: 'a> AssetCache<'a> for AssetHashMapCache<'a, K, T> where K: Hash + PartialEq + Eq {
    type Asset = T;
}