use std::{
    cell::RefCell,
    collections::HashMap,
    hash::Hash,
    rc::Rc,
};
use anyhow::{
    bail,
    Result,
};
use crate::data::*;

pub struct GenericStore<Key, Val>
where
    Key: Clone + Eq + Hash + Construct,
{
    map: HashMap<Key, Val>,
    vec: Vec<(Key, Val)>,
}

impl<Key, Val> GenericStore<Key, Val>
where
    Key: Clone + Eq + Hash + Construct,
    Val: Clone,
{
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            vec: Vec::new(),
        }
    }

    pub fn insert(&mut self, key: Key, val: Val) -> Result<()> {
        if self.map.contains_key(&key) {
            bail!(format!("Registration duplicated: `{}`", key.description()))
        }
        self.map.insert(key.clone(), val.clone());
        self.vec.push((key, val));
        Ok(())
    }

    pub fn insert_or_get(&mut self, key: Key, val: Val) -> Val {
        if !self.map.contains_key(&key) {
            self.map.insert(key.clone(), val.clone());
            self.vec.push((key.clone(), val));
        }
        self.map.get(&key).unwrap().clone()
    }

    pub fn get(&self, key: &Key) -> Result<Val> {
        match self.map.get(key) {
            Some(val) => Ok(val.clone()),
            None => bail!(format!("Key not found: `{}`", key.description())),
        }
    }

    pub fn keys_and_vals<'a>(&'a self) -> impl Iterator<Item = &(Key, Val)> + 'a {
        self.vec.iter()
    }
}

impl<Key, Val> GenericStore<Key, Rc<RefCell<Vec<Val>>>>
where
    Key: Clone + Eq + Hash + Construct,
    Val: Clone,
{
    pub fn insert_new_vec(&mut self, key: Key) -> Result<()> {
        if self.map.contains_key(&key) {
            bail!(format!("Registration duplicated: `{}`", key.description()));
        }
        let vec = Rc::new(RefCell::new(Vec::new()));
        self.map.insert(key.clone(), vec);
        Ok(())
    }

    pub fn push_into_vec(&mut self, key: Key, val: Val) {
        if !self.map.contains_key(&key) {
            let vec = Rc::new(RefCell::new(Vec::new()));
            self.map.insert(key.clone(), vec);
        }
        let vec = self.map.get(&key).unwrap();
        vec.borrow_mut().push(val);
    }
}

impl<OuterKey, InnerKey, Val> GenericStore<OuterKey, Rc<RefCell<HashMap<InnerKey, Val>>>>
where
    OuterKey: Clone + Eq + Hash + Construct,
    InnerKey: Clone + Eq + Hash + Construct,
    Val: Clone,
{
    pub fn insert_new_map(&mut self, outer_key: OuterKey) -> Result<()> {
        if self.map.contains_key(&outer_key) {
            bail!(format!("Registration duplicated: `{}`", outer_key.description()));
        }
        let map = Rc::new(RefCell::new(HashMap::new()));
        self.map.insert(outer_key.clone(), map);
        Ok(())
    }

    pub fn insert_into_map(&mut self, outer_key: OuterKey, inner_key: InnerKey, val: Val) -> Result<()> {
        if !self.map.contains_key(&outer_key) {
            let map = Rc::new(RefCell::new(HashMap::new()));
            self.map.insert(outer_key.clone(), map);
        }
        let mut map = self.map.get(&outer_key).unwrap().borrow_mut();
        if map.contains_key(&inner_key) {
            bail!(format!("Registration duplicated: `{}`", inner_key.description()));
        }
        else {
            map.insert(inner_key, val);
        }
        Ok(())
    }
}
