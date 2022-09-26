use crate::vm::{
    value::{List, MetaValue, Table, Value},
    RuntimeError,
};

#[derive(Debug)]
pub struct Stack {
    stack: Vec<MetaValue>,
}

impl Stack {
    pub fn new() -> Self {
        Self { stack: vec![] }
    }

    pub fn push(&mut self, val: MetaValue) {
        self.stack.push(val)
    }
    pub fn push_bool(&mut self, val: bool) {
        self.push(MetaValue::bool(val))
    }
    pub fn push_int(&mut self, val: i64) {
        self.push(MetaValue::int(val))
    }
    pub fn push_float(&mut self, val: f64) {
        self.push(MetaValue::float(val))
    }
    pub fn push_list(&mut self, val: List) {
        self.push(MetaValue::list(val))
    }
    pub fn push_table(&mut self, val: Table) {
        self.push(MetaValue::table(val))
    }

    pub fn pop(&mut self) -> Result<MetaValue, RuntimeError> {
        self.stack.pop().ok_or(RuntimeError::EmptyStack)
    }
    pub fn pop_bool(&mut self) -> Result<bool, RuntimeError> {
        self.pop().and_then(|v| match v {
            MetaValue {
                value: Value::Bool(v),
                ..
            } => Ok(v),
            _ => Err(RuntimeError::TypeError("Expected bool".to_string())),
        })
    }
    pub fn pop_int(&mut self) -> Result<i64, RuntimeError> {
        self.pop().and_then(|v| match v {
            MetaValue {
                value: Value::Int(v),
                ..
            } => Ok(v),
            _ => Err(RuntimeError::TypeError("Expected int".to_string())),
        })
    }
    pub fn pop_float(&mut self) -> Result<f64, RuntimeError> {
        self.pop().and_then(|v| match v {
            MetaValue {
                value: Value::Float(v),
                ..
            } => Ok(v),
            _ => Err(RuntimeError::TypeError("Expected float".to_string())),
        })
    }
    pub fn pop_list(&mut self) -> Result<List, RuntimeError> {
        self.pop().and_then(|v| match v {
            MetaValue {
                value: Value::List(v),
                ..
            } => Ok(v),
            _ => Err(RuntimeError::TypeError("Expected list".to_string())),
        })
    }
    pub fn pop_table(&mut self) -> Result<Table, RuntimeError> {
        self.pop().and_then(|v| match v {
            MetaValue {
                value: Value::Table(v),
                ..
            } => Ok(v),
            _ => Err(RuntimeError::TypeError("Expected table".to_string())),
        })
    }
}
