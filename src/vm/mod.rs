use crate::vm::instructions::Inst;
use crate::vm::stack::Stack;
use crate::vm::value::{MetaValue, Value};
use thiserror::Error;

pub mod emitter;
pub mod instructions;
pub mod stack;
pub mod value;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("the stack is empty")]
    EmptyStack,
    #[error("{0}")]
    TypeError(String),
    #[error("Local not initialized")]
    LocalNotInitialized,
    #[error("Local not found")]
    LocalNotFound,
    #[error("Operation '{0}' not defined on {1}")]
    OperationNotDefined(String, String),
}

#[derive(Debug)]
pub struct VM {
    stack: Stack,
    locals: Vec<Option<MetaValue>>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            stack: Stack::new(),
            locals: Vec::new(),
        }
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

    pub fn push(&mut self, val: MetaValue) {
        self.stack.push(val)
    }

    pub fn pop(&mut self) -> Result<MetaValue, RuntimeError> {
        self.stack.pop()
    }

    pub fn execute(&mut self, instructions: Vec<Inst>) -> Result<(), RuntimeError> {
        let mut pc = 0;

        while pc < instructions.len() {
            println!("VM: {:?}", self);
            let addr = pc;
            pc += 1;

            println!("PC: {:?}", instructions[addr]);
            match instructions[addr] {
                Inst::Nop => {}
                Inst::PushB(v) => self.stack.push_bool(v),
                Inst::PushI(v) => self.stack.push_int(v),
                Inst::PushF(v) => self.stack.push_float(v),
                Inst::IntoInt => {
                    let v = match self.stack.pop()?.value {
                        Value::Bool(v) => v as i64,
                        Value::Int(v) => v,
                        Value::Float(v) => v as i64,
                    };
                    self.stack.push_int(v);
                }
                Inst::IntoFloat => {
                    let v = match self.stack.pop()?.value {
                        Value::Bool(v) => v as u8 as f64,
                        Value::Int(v) => v as f64,
                        Value::Float(v) => v,
                    };
                    self.stack.push_float(v);
                }
                Inst::And => {
                    let b = self.stack.pop_bool()?;
                    let a = self.stack.pop_bool()?;
                    self.stack.push_bool(a && b)
                }
                Inst::Or => {
                    let b = self.stack.pop_bool()?;
                    let a = self.stack.pop_bool()?;
                    self.stack.push_bool(a || b)
                }
                Inst::Xor => {
                    let b = self.stack.pop_bool()?;
                    let a = self.stack.pop_bool()?;
                    self.stack.push_bool(a ^ b)
                }
                Inst::Not => {
                    let a = self.stack.pop_bool()?;
                    self.stack.push_bool(!a)
                }
                Inst::Add => {
                    let b = self.stack.pop()?;
                    match b.value {
                        Value::Bool(b) => {
                            return Err(RuntimeError::OperationNotDefined(
                                "+".to_string(),
                                "bool".to_string(),
                            ))
                        }
                        Value::Int(b) => {
                            let a = self.stack.pop_int()?;
                            self.stack.push_int(a + b)
                        }
                        Value::Float(b) => {
                            let a = self.stack.pop_float()?;
                            self.stack.push_float(a + b)
                        }
                    }
                }
                Inst::Sub => {
                    let b = self.stack.pop()?;
                    match b.value {
                        Value::Bool(b) => {
                            return Err(RuntimeError::OperationNotDefined(
                                "-".to_string(),
                                "bool".to_string(),
                            ))
                        }
                        Value::Int(b) => {
                            let a = self.stack.pop_int()?;
                            self.stack.push_int(a - b)
                        }
                        Value::Float(b) => {
                            let a = self.stack.pop_float()?;
                            self.stack.push_float(a - b)
                        }
                    }
                }
                Inst::Mul => {
                    let b = self.stack.pop()?;
                    match b.value {
                        Value::Bool(b) => {
                            return Err(RuntimeError::OperationNotDefined(
                                "*".to_string(),
                                "bool".to_string(),
                            ))
                        }
                        Value::Int(b) => {
                            let a = self.stack.pop_int()?;
                            self.stack.push_int(a * b)
                        }
                        Value::Float(b) => {
                            let a = self.stack.pop_float()?;
                            self.stack.push_float(a * b)
                        }
                    }
                }
                Inst::Div => {
                    let b = self.stack.pop()?;
                    match b.value {
                        Value::Bool(b) => {
                            return Err(RuntimeError::OperationNotDefined(
                                "/".to_string(),
                                "bool".to_string(),
                            ))
                        }
                        Value::Int(b) => {
                            let a = self.stack.pop_int()?;
                            self.stack.push_int(a / b)
                        }
                        Value::Float(b) => {
                            let a = self.stack.pop_float()?;
                            self.stack.push_float(a / b)
                        }
                    }
                }
                Inst::Mod => {
                    let b = self.stack.pop()?;
                    match b.value {
                        Value::Bool(b) => {
                            return Err(RuntimeError::OperationNotDefined(
                                "+".to_string(),
                                "bool".to_string(),
                            ))
                        }
                        Value::Int(b) => {
                            let a = self.stack.pop_int()?;
                            self.stack.push_int(a % b)
                        }
                        Value::Float(b) => {
                            let a = self.stack.pop_float()?;
                            self.stack.push_float(a % b)
                        }
                    }
                }
                Inst::Equal => {
                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;
                    self.stack.push_bool(a == b)
                }
                Inst::NotEqual => {
                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;
                    self.stack.push_bool(a != b)
                }
                Inst::LessThan => {
                    let b = self.stack.pop_int()?;
                    let a = self.stack.pop_int()?;
                    self.stack.push_bool(a < b)
                }
                Inst::GreaterThan => {
                    let b = self.stack.pop_int()?;
                    let a = self.stack.pop_int()?;
                    self.stack.push_bool(a > b)
                }
                Inst::LessEqual => {
                    let b = self.stack.pop_int()?;
                    let a = self.stack.pop_int()?;
                    self.stack.push_bool(a <= b)
                }
                Inst::GreaterEqual => {
                    let b = self.stack.pop_int()?;
                    let a = self.stack.pop_int()?;
                    self.stack.push_bool(a >= b)
                }
                Inst::Branch(idx) => pc = idx,
                Inst::BranchIf(idx) => {
                    let v = self.stack.pop_bool()?;
                    if v {
                        pc = idx;
                    }
                }
                Inst::Call(_) => {}
                Inst::LocalReserve(count) => self.locals = vec![None; count],
                Inst::LocalLoad(idx) => {
                    let l = self.get_local(idx)?;
                    self.stack.push(l);
                }
                Inst::LocalStore(idx) => {
                    let l = self.stack.pop()?;
                    self.set_local(idx, l);
                }
                Inst::Dup => {
                    let l = self.stack.pop()?;
                    self.stack.push(l.clone());
                    self.stack.push(l);
                }
                Inst::Drop => {
                    self.stack.pop()?;
                }
                Inst::Swap => {
                    let a = self.stack.pop()?;
                    let b = self.stack.pop()?;
                    self.stack.push(a);
                    self.stack.push(b);
                }
                Inst::PushList => {}
                Inst::PushTable => {}
                Inst::ListPush => {}
                Inst::ListPop => {}
                Inst::ListGet => {}
                Inst::ListSet => {}
                Inst::TablePush => {}
                Inst::TablePop => {}
                Inst::TableGet => {}
                Inst::TableSet => {}
                Inst::LoadMeta => {}
                Inst::StoreMeta => {}
                Inst::Return => {}
            }
        }
        Ok(())
    }
}
