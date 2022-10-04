use crate::vm::{value::MetaValue, RuntimeError};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Default)]
pub struct Env {
    locals: Vec<Option<MetaValue>>,
}

impl Display for Env {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            self.locals
                .iter()
                .map(|it| it
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or("_".to_string()))
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl Env {
    pub fn reserve(&mut self, count: usize) {
        self.locals.append(&mut vec![None; count])
    }

    pub fn get_local(&mut self, idx: usize) -> Result<MetaValue, RuntimeError> {
        match self.locals.get(idx) {
            Some(Some(v)) => Ok(v.clone()),
            Some(None) => Err(RuntimeError::LocalNotInitialized),
            None => Err(RuntimeError::LocalNotFound),
        }
    }

    pub fn set_local(&mut self, idx: usize, val: MetaValue) -> Result<(), RuntimeError> {
        match self.locals.get_mut(idx) {
            Some(v) => Ok(*v = Some(val)),
            None => Err(RuntimeError::LocalNotFound),
        }
    }
}
