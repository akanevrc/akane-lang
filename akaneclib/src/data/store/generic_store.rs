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

    pub fn keys_and_vals(&self) -> impl Iterator<Item = &(Key, Val)> + '_ {
        self.vec.iter()
    }
}

impl<Key, Val> GenericStore<Key, Rc<RefCell<Vec<Val>>>>
where
    Key: Clone + Eq + Hash + Construct,
    Val: Clone,
{
    pub fn insert_new_vec(&mut self, key: Key) -> Result<()> {
        let vec = Rc::new(RefCell::new(Vec::new()));
        self.insert(key, vec)
    }

    pub fn push_into_vec(&mut self, key: Key, val: Val) {
        let vec = Rc::new(RefCell::new(Vec::new()));
        let vec = self.insert_or_get(key, vec);
        vec.borrow_mut().push(val);
    }
}

impl<Key, Col, Val> GenericStore<Key, Rc<RefCell<Matrix<Col, Val>>>>
where
    Key: Clone + Eq + Hash + Construct,
    Col: Clone + Eq + Hash + Construct,
    Val: Clone,
{
    pub fn insert_new_matrix(&mut self, key: Key, cols: Vec<Col>) -> Result<()> {
        let matrix = Rc::new(RefCell::new(Matrix::new(cols)));
        self.insert(key, matrix)
    }

    pub fn insert_row_into_matrix(&mut self, key: Key, row: Vec<Val>) -> Result<()> {
        if !self.map.contains_key(&key) {
            bail!(format!("Registration not found: `{}`", key.description()));
        }
        let mut matrix = self.map.get(&key).unwrap().borrow_mut();
        matrix.push(row);
        Ok(())
    }
}
