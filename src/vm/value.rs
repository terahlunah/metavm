use eq_float::F64;
use std::collections::HashMap;

pub type Table = HashMap<String, Value>;
pub type List = Vec<Value>;

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Int(i128),
    Float(f64),
    //Char(char),
    //List(List),
    //Table(Table)
}

impl PartialEq<Self> for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => {
                if a.is_nan() && b.is_nan() {
                    true
                } else {
                    a == b
                }
            }
            _ => false,
        }
    }
}

impl Eq for Value {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetaValue {
    pub value: Value,
    pub meta: Table,
}

impl MetaValue {
    pub fn new(value: Value) -> Self {
        Self {
            value,
            meta: Table::new(),
        }
    }

    pub fn bool(val: bool) -> Self {
        Self::new(Value::Bool(val))
    }

    pub fn int(val: i128) -> Self {
        Self::new(Value::Int(val))
    }

    pub fn float(val: f64) -> Self {
        Self::new(Value::Float(val))
    }

    pub fn is_bool(&self) -> bool {
        matches!(self.value, Value::Bool(_))
    }
    pub fn is_int(&self) -> bool {
        matches!(self.value, Value::Int(_))
    }
    pub fn is_float(&self) -> bool {
        matches!(self.value, Value::Float(_))
    }
}
