use std::rc::Rc;
use anyhow::{
    bail,
    Result,
};
use crate::data::*;

pub struct QualStack {
    stack: Vec<QualKey>,
}

impl QualStack {
    pub fn new() -> Self {
        Self {
            stack: vec![QualKey::top()],
        }
    }

    pub fn push(&mut self, qual: &Rc<QualSem>) -> QualKey {
        let key = qual.to_key();
        self.stack.push(key.clone());
        key
    }

    pub fn peek(&self) -> QualKey {
        self.stack.last().unwrap().clone()
    }

    pub fn pop(&mut self) -> Result<QualKey> {
        let qual = self.stack.pop();
        if qual.is_none() || qual.clone().unwrap() == QualKey::top() {
            bail!("Top scope has been popped");
        }
        Ok(qual.unwrap())
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = QualKey> + 'a {
        self.stack.iter().rev().cloned()
    }
}
