mod expr;
mod var;
mod cn;
mod abs;
mod app;
mod ty;
mod tvar;
mod base;
mod arrow;
mod qual;
mod scope;
mod macros;

pub use expr::*;
pub use var::*;
pub use cn::*;
pub use abs::*;
pub use app::*;
pub use ty::*;
pub use tvar::*;
pub use base::*;
pub use arrow::*;
pub use qual::*;
pub use scope::*;

use std::rc::Rc;
use anyhow::Result;
use crate::data::*;

pub trait Construct {
    fn logical_name(&self) -> String;
    fn description(&self) -> String;
}

pub trait ConstructVal: Construct {
    type Key: ConstructKey<Val=Self>;
    fn to_key(&self) -> Self::Key;
}

pub trait ConstructKey: Construct {
    type Val: Construct;
    fn get_val(&self, ctx: &SemantizerContext) -> Result<Rc<Self::Val>>;
}

impl Construct for usize {
    fn logical_name(&self) -> String {
        self.to_string()
    }

    fn description(&self) -> String {
        self.to_string()
    }
}

impl Construct for String {
    fn logical_name(&self) -> String {
        self.clone()
    }

    fn description(&self) -> String {
        self.clone()
    }
}
