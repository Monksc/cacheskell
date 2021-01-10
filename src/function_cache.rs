use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use super::*;

#[derive(Debug, Clone)]
pub struct FunctionCache {
    cache : Vec<Option<(FunctionCall, Expression)>>,
}

impl FunctionCache {

    pub fn new(size: u16) -> FunctionCache {

        let mut v = Vec::new();
        for _ in 0..size {
            v.push(None);
        }

        FunctionCache { cache: v, }
    }

    fn get_index(&self, key : &FunctionCall) -> usize {
        let mut s = DefaultHasher::new();
        key.hash(&mut s);
        return s.finish() as usize % self.cache.len();
    }

    pub fn get(&self, key: &FunctionCall) -> Option<Expression> {

        let index = self.get_index(key);

        if let Some((_, value)) = &self.cache[index] {
            return Some(value.clone());
        }

        None
    }

    pub fn put(&mut self, key: FunctionCall, value: Expression) {

        let index = self.get_index(&key);

        self.cache[index] = Some((key, value));
    }
}
