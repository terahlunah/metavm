use crate::vm::instructions::Inst;
use std::collections::HashMap;

pub type Functions = HashMap<String, Function>;

#[derive(Debug, Clone)]
pub struct Function {
    pub instructions: Vec<Inst>,
    pub locals: usize,
}
