use crate::data::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Scope {
    Abs(usize),
}

impl Construct for Scope {
    fn logical_name(&self) -> String {
        match self {
            Self::Abs(id) =>
                format!("fn.{}", id),
        }
    }

    fn description(&self) -> String {
        match self {
            Self::Abs(id) =>
                format!("fn[{}]", id),
        }
    }
}
