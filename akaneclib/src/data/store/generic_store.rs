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

#[derive(Debug)]
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

    pub fn insert(&mut self, key: Key, val: Val) -> Result<Val> {
        if self.map.contains_key(&key) {
            bail!("Registration duplicated: `{}`", key.description());
        }
        self.map.insert(key.clone(), val.clone());
        self.vec.push((key, val.clone()));
        Ok(val)
    }

    pub fn insert_or_get(&mut self, key: Key, val: Val) -> Val {
        if let Some(val) = self.map.get(&key) {
            val.clone()
        }
        else {
            self.map.insert(key.clone(), val.clone());
            self.vec.push((key.clone(), val.clone()));
            val
        }
    }

    pub fn get(&self, key: &Key) -> Result<Val> {
        if let Some(val) = self.map.get(key) {
            Ok(val.clone())
        }
        else {
            bail!("Key not found: `{}`", key.description());
        }
    }

    pub fn keys_and_vals(&self) -> impl Iterator<Item = &(Key, Val)> + '_ {
        self.vec.iter()
    }
}

impl<Key, Val> GenericStore<Key, Rc<RefCell<Vec<Val>>>>
where
    Key: Clone + Eq + Hash + Construct,
    Val: Clone,
{
    pub fn insert_new_vec(&mut self, key: Key) -> Result<Rc<RefCell<Vec<Val>>>> {
        let vec = Rc::new(RefCell::new(Vec::new()));
        self.insert(key, vec.clone())?;
        Ok(vec)
    }

    pub fn push_into_vec(&mut self, key: Key, val: Val) {
        let vec = Rc::new(RefCell::new(Vec::new()));
        let vec = self.insert_or_get(key, vec);
        vec.borrow_mut().push(val);
    }
}

// impl<Key> GenericStore<Key, Rc<RefCell<TyEnvStore>>>
// where
//     Key: Clone + Eq + Hash + Construct,
// {
//     pub fn insert_new_ty_env_store(&mut self, key: Key, tvars: Vec<TVarKey>) -> Result<Rc<RefCell<TyEnvStore>>> {
//         let ty_env_store = TyEnvStore::new(tvars);
//         self.insert(key, ty_env_store.clone())?;
//         Ok(ty_env_store)
//     }
// }
