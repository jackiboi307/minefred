// TODO
// make the index type (currently usize) generic,
// so this could be used for GameObjTypes

use std::collections::HashMap;

pub struct KeyedSliceBuilder<T> {
    vec: Vec<T>,
    key_index_map: HashMap<&'static str, usize>,
}

pub struct KeyedSlice<T> {
    pub arr: Box<[T]>,
    pub key_index_map: HashMap<&'static str, usize>,
}

impl<T> KeyedSliceBuilder<T> {
    pub fn new() -> Self {
        Self {
            vec: Vec::new(),
            key_index_map: HashMap::new(),
        }
    }

    pub fn add(&mut self, item: T) -> usize {
        self.vec.push(item);
        let index = self.vec.len() - 1;
        index
    }

    pub fn add_as(&mut self, item: T, key: &'static str) -> usize {
        let index = self.add(item);
        self.key_index_map.insert(key, index.clone());
        index
    }

    pub fn build(self) -> KeyedSlice<T> {
        KeyedSlice {
            arr: self.vec.into_boxed_slice(),
            key_index_map: self.key_index_map,
        }
    }
}

impl<T> KeyedSlice<T> {
    pub fn empty() -> Self {
        Self {
            arr: [].into(),
            key_index_map: HashMap::with_capacity(0),
        }
    }

    pub fn by_id(&self, id: usize) -> Option<&T> {
        self.arr.get(id)
    }

    pub fn by_id_mut(&mut self, id: usize) -> Option<&mut T> {
        self.arr.get_mut(id)
    }

    pub fn by_key(&self, key: &'static str) -> Option<&T> {
        if let Some(index) = self.key_index_map.get(key) {
            self.arr.get(*index)
        } else {
            None
        }
    }

    pub fn by_key_mut(&mut self, key: &'static str) -> Option<&mut T> {
        if let Some(index) = self.key_index_map.get(key) {
            self.arr.get_mut(*index)
        } else {
            None
        }
    }
}
