use std::marker::PhantomData;
use std::hash::Hash;

use fnv::FnvHashMap;

pub trait AssetCache<'a> {
    type Asset: 'a;
}

#[derive(Debug, Clone)]
pub struct AssetHashMapCache<'a, K, T: 'a> where K: Hash + PartialEq + Eq {
    map: FnvHashMap<K, T>,
    _marker: PhantomData<&'a ()>,
}

impl<'a, K, T: 'a> AssetCache<'a> for AssetHashMapCache<'a, K, T> where K: Hash + PartialEq + Eq {
    type Asset = T;
}