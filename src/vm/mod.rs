use crate::vm::{
    env::Env,
    function::{Function, Functions},
    instructions::Inst,
    stack::Stack,
    value::{List, MetaValue, Table, Value},
};
use thiserror::Error;

pub mod emitter;
pub mod env;
pub mod function;
pub mod instructions;
pub mod stack;
pub mod value;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum RuntimeError {
    #[error("the stack is empty")]
    EmptyStack,
    #[error("the list is empty")]
    EmptyList,
    #[error("the table is empty")]
    EmptyTable,
    #[error("Expected {0}")]
    TypeError(String),
    #[error("Local not initialized")]
    LocalNotInitialized,
    #[error("Local not found")]
    LocalNotFound,
    #[error("Operation '{0}' not defined on {1}")]
    OperationNotDefined(String, String),
    #[error("Index out of range, got {0} but range is [{1}, {2}]")]
    RangeError(i64, i64, i64),
    #[error("Key {0} not found in table")]
    KeyNotFound(String),
    #[error("Function not found: {0}")]
    FunctionNotFound(String),
}

#[derive(Debug)]
pub struct VM {
    stack: Stack,
    functions: Functions,
    tracing: bool,
}

impl VM {
    pub fn new(functions: Functions) -> Self {
        Self {
            stack: Stack::new(),
            functions,
            tracing: false,
        }
    }

    pub fn tracing(&mut self, tracing: bool) {
        self.tracing = tracing;
    }

    pub fn push(&mut self, val: MetaValue) {
        self.stack.push(val)
    }

    pub fn pop(&mut self) -> Result<MetaValue, RuntimeError> {
        self.stack.pop()
    }

    pub fn run(&mut self, function: impl Into<String>) -> Result<(), RuntimeError> {
        let function = self.get_function(function.into())?;
        self.execute(function, Env::default())
    }

    fn get_function(&self, name: String) -> Result<Function, RuntimeError> {
        self.functions
            .get(&name)
            .ok_or(RuntimeError::FunctionNotFound(name))
            .cloned()
    }

    fn execute(&mut self, function: Function, mut env: Env) -> Result<(), RuntimeError> {
        let instructions = function.instructions;
        let mut pc = 0;
        env.reserve(function.locals);

        while pc < instructions.len() {
            let addr = pc;
            pc += 1;

            if self.tracing {
                println!("-------------");
                println!("PC: {:?}", instructions[addr]);
            }
            match instructions[addr].clone() {
                Inst::Nop => {}
                Inst::PushB(v) => self.stack.push_bool(v),
                Inst::PushI(v) => self.stack.push_int(v),
                Inst::PushF(v) => self.stack.push_float(v),
                Inst::IntoInt => {
                    let mv = self.stack.pop()?;
                    let v = match mv.value {
                        Value::Bool(v) => v as i64,
                        Value::Int(v) => v,
                        Value::Float(v) => v.0 as i64,
                        Value::List(v) => v.len() as i64,
                        Value::Table(v) => v.len() as i64,
                        Value::FunctionRef(_) => {
                            return operation_not_defined("into_int", mv.type_name())
                        }
                    };
                    self.stack.push_int(v);
                }
                Inst::IntoFloat => {
                    let mv = self.stack.pop()?;
                    let v = match mv.value {
                        Value::Bool(v) => v as u8 as f64,
                        Value::Int(v) => v as f64,
                        Value::Float(v) => v.0,
                        Value::List(v) => v.len() as f64,
                        Value::Table(v) => v.len() as f64,
                        Value::FunctionRef(_) => {
                            return operation_not_defined("into_float", mv.type_name())
                        }
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
                        Value::Bool(_)
                        | Value::List(_)
                        | Value::Table(_)
                        | Value::FunctionRef(_) => {
                            return operation_not_defined("+", b.type_name());
                        }
                        Value::Int(b) => {
                            let a = self.stack.pop_int()?;
                            self.stack.push_int(a + b)
                        }
                        Value::Float(b) => {
                            let a = self.stack.pop_float()?;
                            self.stack.push_float(a + b.0)
                        }
                    }
                }
                Inst::Sub => {
                    let b = self.stack.pop()?;
                    match b.value {
                        Value::Bool(_)
                        | Value::List(_)
                        | Value::Table(_)
                        | Value::FunctionRef(_) => {
                            return operation_not_defined("-", b.type_name());
                        }
                        Value::Int(b) => {
                            let a = self.stack.pop_int()?;
                            self.stack.push_int(a - b)
                        }
                        Value::Float(b) => {
                            let a = self.stack.pop_float()?;
                            self.stack.push_float(a - b.0)
                        }
                    }
                }
                Inst::Mul => {
                    let b = self.stack.pop()?;
                    match b.value {
                        Value::Bool(_)
                        | Value::List(_)
                        | Value::Table(_)
                        | Value::FunctionRef(_) => {
                            return operation_not_defined("*", b.type_name());
                        }
                        Value::Int(b) => {
                            let a = self.stack.pop_int()?;
                            self.stack.push_int(a * b)
                        }
                        Value::Float(b) => {
                            let a = self.stack.pop_float()?;
                            self.stack.push_float(a * b.0)
                        }
                    }
                }
                Inst::Div => {
                    let b = self.stack.pop()?;
                    match b.value {
                        Value::Bool(_)
                        | Value::List(_)
                        | Value::Table(_)
                        | Value::FunctionRef(_) => {
                            return operation_not_defined("/", b.type_name());
                        }
                        Value::Int(b) => {
                            let a = self.stack.pop_int()?;
                            self.stack.push_int(a / b)
                        }
                        Value::Float(b) => {
                            let a = self.stack.pop_float()?;
                            self.stack.push_float(a / b.0)
                        }
                    }
                }
                Inst::Mod => {
                    let b = self.stack.pop()?;
                    match b.value {
                        Value::Bool(_)
                        | Value::List(_)
                        | Value::Table(_)
                        | Value::FunctionRef(_) => {
                            return operation_not_defined("mod", b.type_name());
                        }
                        Value::Int(b) => {
                            let a = self.stack.pop_int()?;
                            self.stack.push_int(a % b)
                        }
                        Value::Float(b) => {
                            let a = self.stack.pop_float()?;
                            self.stack.push_float(a % b.0)
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
                Inst::BranchIfNot(idx) => {
                    let v = self.stack.pop_bool()?;
                    if !v {
                        pc = idx;
                    }
                }
                Inst::Call => {
                    let v = self.stack.pop_function_ref()?;
                    let function = self.get_function(v.name)?;
                    let env = v.env;
                    self.execute(function, env)?;
                    if self.tracing {
                        println!("-------------");
                    }
                }
                Inst::Bind => {
                    let env = self.stack.pop_list()?;
                    let mut f = self.stack.pop_function_ref()?;
                    f.env = Env::new(env.into_iter().map(Some).collect());
                    self.stack.push_function_ref(f);
                }
                Inst::PushFn(v) => self.stack.push_function_ref(v.into()),
                Inst::LocalLoad(idx) => {
                    let l = env.get_local(idx)?;
                    self.stack.push(l);
                }
                Inst::LocalStore(idx) => {
                    let l = self.stack.pop()?;
                    env.set_local(idx, l)?;
                }
                Inst::Dup => {
                    let v = self.stack.pop()?;
                    self.stack.push(v.clone());
                    self.stack.push(v);
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
                Inst::PushList => self.stack.push_list(List::new()),
                Inst::ListPush => {
                    let v = self.stack.pop()?;
                    let mut l = self.stack.pop_list()?;
                    l.push(v);
                    self.stack.push_list(l);
                }
                Inst::ListPop => {
                    let mut l = self.stack.pop_list()?;
                    let v = l.pop().ok_or(RuntimeError::EmptyList)?;
                    self.stack.push(v);
                }
                Inst::ListGet => {
                    let i = self.stack.pop_int()?;
                    let l = self.stack.pop_list()?;
                    if i >= 0 && i < l.len() as i64 {
                        let v = l.get(i as usize).ok_or(RuntimeError::RangeError(
                            i,
                            0,
                            l.len() as i64,
                        ))?;
                        self.stack.push(v.clone());
                    } else {
                        return Err(RuntimeError::RangeError(i, 0, l.len() as i64));
                    }
                }
                Inst::ListSet => {
                    let v = self.stack.pop()?;
                    let i = self.stack.pop_int()?;
                    let mut l = self.stack.pop_list()?;
                    let len = l.len() as i64;
                    if i >= 0 && i < len {
                        *(l.get_mut(i as usize)
                            .ok_or(RuntimeError::RangeError(i, 0, len))?) = v;
                        self.stack.push_list(l);
                    } else {
                        return Err(RuntimeError::RangeError(i, 0, l.len() as i64));
                    }
                }
                Inst::ListLen => {
                    let l = self.stack.pop_list()?;
                    self.stack.push_int(l.len() as i64);
                }
                Inst::PushTable => self.stack.push_table(Table::new()),
                Inst::TableGet => {
                    let k = self.stack.pop()?;
                    let t = self.stack.pop_table()?;
                    let v = t
                        .get(&k)
                        .ok_or(RuntimeError::KeyNotFound(k.to_string()))?
                        .clone();
                    self.stack.push(v);
                }
                Inst::TableSet => {
                    let v = self.stack.pop()?;
                    let k = self.stack.pop()?;
                    let mut t = self.stack.pop_table()?;
                    t.insert(k, v);
                    self.stack.push_table(t);
                }
                Inst::TableKeys => {
                    let t = self.stack.pop_table()?;
                    let keys: Vec<MetaValue> = t.keys().cloned().collect();
                    self.stack.push_list(keys);
                }
                Inst::TableLen => {
                    let t = self.stack.pop_table()?;
                    self.stack.push_int(t.len() as i64);
                }
                Inst::LoadMeta => {
                    let v = self.stack.pop()?;
                    self.stack.push_table(v.meta);
                }
                Inst::StoreMeta => {
                    let t = self.stack.pop_table()?;
                    let mut v = self.stack.pop()?;
                    v.meta = t;
                    self.stack.push(v);
                }
                Inst::Return => {}
            }
            if self.tracing {
                println!("Stack: {}", self.stack);
                println!("Env: {}", env);
            }
        }
        Ok(())
    }
}

fn operation_not_defined(
    op_name: impl Into<String>,
    type_name: impl Into<String>,
) -> Result<(), RuntimeError> {
    Err(RuntimeError::OperationNotDefined(
        op_name.into(),
        type_name.into(),
    ))
}
