use std::{
    collections::HashMap,
    hash::Hash,
};

pub struct Matrix<Col, Val>
where
    Col: Clone + Eq + Hash,
    Val: Clone,
{
    cols: Vec<Col>,
    rows: Vec<Val>,
}

impl<Col, Val> Matrix<Col, Val>
where
    Col: Clone + Eq + Hash,
    Val: Clone,
{
    pub fn new(cols: Vec<Col>) -> Self {
        Self {
            cols,
            rows: Vec::new(),
        }
    }

    pub fn push(&mut self, row: Vec<Val>) {
        if row.len() != self.cols.len() {
            panic!("Row length mismatch");
        }
        for val in row {
            self.rows.push(val);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = HashMap<Col, Val>> + '_ {
        self.rows.chunks(self.cols.len())
        .map(|row|
            self.cols.iter()
            .zip(row.iter())
            .map(|(col, val)| (col.clone(), val.clone()))
            .collect::<HashMap<Col, Val>>()
        )
    }
}
