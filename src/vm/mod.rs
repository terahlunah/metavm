use crate::vm::instructions::Inst;
use crate::vm::stack::Stack;
use crate::vm::value::MetaValue;
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
                Inst::PushI(v) => self.stack.push_int(v),
                Inst::IntoInt => {}
                Inst::AddI => {
                    let b = self.stack.pop_int()?;
                    let a = self.stack.pop_int()?;
                    self.stack.push_int(a + b)
                }
                Inst::SubI => {
                    let b = self.stack.pop_int()?;
                    let a = self.stack.pop_int()?;
                    self.stack.push_int(a - b)
                }
                Inst::MulI => {
                    let b = self.stack.pop_int()?;
                    let a = self.stack.pop_int()?;
                    self.stack.push_int(a * b)
                }
                Inst::DivI => {
                    let b = self.stack.pop_int()?;
                    let a = self.stack.pop_int()?;
                    self.stack.push_int(a / b)
                }
                Inst::ModI => {
                    let b = self.stack.pop_int()?;
                    let a = self.stack.pop_int()?;
                    self.stack.push_int(a % b)
                }
                Inst::PushF(v) => self.stack.push_float(v),
                Inst::IntoFloat => {}
                Inst::AddF => {
                    let b = self.stack.pop_float()?;
                    let a = self.stack.pop_float()?;
                    self.stack.push_float(a + b)
                }
                Inst::SubF => {
                    let b = self.stack.pop_float()?;
                    let a = self.stack.pop_float()?;
                    self.stack.push_float(a - b)
                }
                Inst::MulF => {
                    let b = self.stack.pop_float()?;
                    let a = self.stack.pop_float()?;
                    self.stack.push_float(a * b)
                }
                Inst::DivF => {
                    let b = self.stack.pop_float()?;
                    let a = self.stack.pop_float()?;
                    self.stack.push_float(a / b)
                }
                Inst::ModF => {
                    let b = self.stack.pop_float()?;
                    let a = self.stack.pop_float()?;
                    self.stack.push_float(a % b)
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
                Inst::Jump(idx) => pc = idx,
                Inst::Branch(idx) => {
                    let v = self.stack.pop_bool()?;
                    if v {
                        pc = idx;
                    }
                }
                Inst::Call(_) => {}
                Inst::Reserve(count) => self.locals = vec![None; count],
                Inst::Load(idx) => {
                    let l = self.get_local(idx)?;
                    self.stack.push(l);
                }
                Inst::Store(idx) => {
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
            }
        }
        Ok(())
    }
}
