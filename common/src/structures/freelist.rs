use vec_map::{VecMap, Keys, Iter, IterMut};

pub struct FreelistVecMap<T> {
    freelist: Vec<usize>,
    map: VecMap<T>,
}

impl<T> Default for FreelistVecMap<T> {
    #[inline(always)]
    fn default() -> FreelistVecMap<T> { FreelistVecMap::new() }
}

impl<T> FreelistVecMap<T> {
    pub fn new() -> FreelistVecMap<T> {
        FreelistVecMap {
            freelist: Vec::new(),
            map: VecMap::new()
        }
    }

    pub fn with_capacity(cap: usize) -> FreelistVecMap<T> {
        FreelistVecMap {
            freelist: Vec::with_capacity(cap),
            map: VecMap::with_capacity(cap)
        }
    }

    pub fn add(&mut self, value: T) -> usize {
        let index = if let Some(index) = self.freelist.pop() { index } else {
            self.map.len()
        };

        self.map.insert(index, value);

        index
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        let res = self.map.remove(index);

        if res.is_some() || !self.freelist.contains(&index) {
            self.freelist.push(index);
        }

        res
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    #[inline(always)]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.map.get(index)
    }

    #[inline(always)]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.map.get_mut(index)
    }

    pub fn clear(&mut self) {
        self.freelist.clear();
        self.map.clear();
    }

    #[inline(always)]
    pub fn keys(&self) -> Keys<T> {
        self.map.keys()
    }

    #[inline(always)]
    pub fn iter(&self) -> Iter<T> {
        self.map.iter()
    }

    #[inline(always)]
    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.map.iter_mut()
    }
}