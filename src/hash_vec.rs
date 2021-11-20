use std::hash::Hash;
use core::slice::Iter;
use std::collections::HashMap;

/// 元素以线性存储，但是又可以通过Key来快速访问
pub struct HashVec<K, V> {
    vec: Vec<V>,
    map: HashMap<K, usize>,
}

impl<K, V> HashVec<K, V> where K: Eq + Hash {
    pub fn new() -> Self {
        Self {
            vec: Vec::new(),
            map: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn vec(&self) -> Iter<'_, V> {
        self.vec.iter()
    }

    pub fn contains(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    pub fn key_get(&self, key: &K) -> Option<&V> {
        let index = *self.map.get(key)?;
        if index > self.vec.len() {
            return None;
        }
        Some(&self.vec[index])
    }

    pub fn key_get_mut(&mut self, key: &K) -> Option<&mut V> {
        let index = *self.map.get(key)?;
        if index > self.vec.len() {
            return None;
        }
        Some(&mut self.vec[index])
    }

    /// 通过key获取index，没找到返回None
    pub fn key_get_index(&self, key: &K) -> Option<usize> {
        match self.map.get(key) {
            Some(index) => Some(*index),
            None => None,
        }
    }

    pub fn index_get(&self, index: usize) -> Option<&V> {
        if index > self.vec.len() {
            return None;
        }
        Some(&self.vec[index])
    }

    pub fn index_get_mut(&mut self, index: usize) -> Option<&mut V> {
        if index > self.vec.len() {
            return None;
        }
        Some(&mut self.vec[index])
    }

    pub fn insert(&mut self, key: K, value: V) {
        match self.map.get(&key) {
            Some(i) => {
                self.vec[*i] = value;
            }
            None => {
                let i = self.vec.len();
                self.vec.push(value);
                self.map.insert(key, i);
            }
        }
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        match self.map.remove(key) {
            Some(i) => {
                let old = self.vec.remove(i);
                for (_, v) in self.map.iter_mut() {
                    if *v > i {
                        *v -= 1;
                    }
                }
                Some(old)
            }
            None => None,
        }
    }
}