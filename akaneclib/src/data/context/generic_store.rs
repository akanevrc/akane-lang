use std::{
    collections::HashMap,
    hash::Hash,
};
use anyhow::{
    bail,
    Result,
};
use crate::data::*;

pub struct GenericStore<Key, Val>
where
    Key: Clone + Eq + Hash + Sem,
{
    map: HashMap<Key, Val>,
}

impl<Key, Val> GenericStore<Key, Val>
where
    Key: Clone + Eq + Hash + Sem,
{
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: Key, val: Val) -> Result<()> {
        if self.map.contains_key(&key) {
            bail!(format!("Registration duplicated: `{}`", key.description()))
        }
        self.map.insert(key, val);
        Ok(())
    }

    pub fn insert_or_get(&mut self, key: Key, val: Val) -> &Val {
        if !self.map.contains_key(&key) {
            self.map.insert(key.clone(), val);
        }
        self.map.get(&key).unwrap()
    }

    pub fn get(&self, key: &Key) -> Result<&Val> {
        match self.map.get(key) {
            Some(val) => Ok(val),
            None => bail!(format!("Key not found: `{}`", key.description())),
        }
    }

    pub fn keys_and_vals<'a>(&'a self) -> impl Iterator<Item = (&Key, &Val)> + 'a {
        self.map.iter()
    }
}

impl<Key, Val> GenericStore<Key, Vec<Val>>
where
    Key: Clone + Eq + Hash + Sem,
{
    pub fn push_into_vec(&mut self, key: &Key, val: Val) {
        if !self.map.contains_key(key) {
            let vec = Vec::new();
            self.map.insert(key.clone(), vec);
        }
        let vec = self.map.get_mut(key).unwrap();
        vec.push(val);
    }
}
