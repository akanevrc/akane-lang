mod qual_sem;
mod scope_sem;
mod ty_sem;
mod ty2_sem;
mod ty1_sem;
mod fn_sem;
mod macros;

pub use qual_sem::*;
pub use scope_sem::*;
pub use ty_sem::*;
pub use ty2_sem::*;
pub use ty1_sem::*;
pub use fn_sem::*;

use std::rc::Rc;
use anyhow::Result;
use crate::data::*;

pub trait Sem {
    fn logical_name(&self) -> String;
    fn description(&self) -> String;
}

pub trait SemVal<Key> {
    fn to_key(&self) -> Key;
}

pub trait SemKey<Val> {
    fn get_val(&self, ctx: &SemContext) -> Result<Rc<Val>>;
}

impl Sem for String {
    fn logical_name(&self) -> String {
        self.clone()
    }

    fn description(&self) -> String {
        self.clone()
    }
}
