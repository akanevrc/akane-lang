use crate::data::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Scope {
    Abs(String),
}

impl Construct for Scope {
    fn logical_name(&self) -> String {
        match self {
            Self::Abs(x) =>
                x.clone(),
        }
    }

    fn description(&self) -> String {
        match self {
            Self::Abs(x) =>
                format!("fn {}", x),
        }
    }
}
