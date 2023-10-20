use crate::data::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ScopeSem {
    Fn(String),
}

impl Sem for ScopeSem {
    fn logical_name(&self) -> String {
        match self {
            Self::Fn(x) =>
                x.clone(),
        }
    }

    fn description(&self) -> String {
        match self {
            Self::Fn(x) =>
                format!("fn {}", x),
        }
    }
}
