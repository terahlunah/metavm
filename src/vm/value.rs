use crate::vm::env::Env;
use eq_float::F64;
use std::{
    collections::BTreeMap,
    fmt::{write, Display, Formatter},
    hash::Hash,
};

pub type Table = BTreeMap<MetaValue, MetaValue>;
pub type List = Vec<MetaValue>;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct FunctionRef {
    pub name: String,
    pub env: Env,
}

impl<S> From<S> for FunctionRef
where
    S: Into<String>,
{
    fn from(s: S) -> Self {
        FunctionRef {
            name: s.into(),
            env: Env::default(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Value {
    Bool(bool),
    Int(i64),
    Float(F64),
    //Char(char),
    List(List),
    Table(Table),
    FunctionRef(FunctionRef), // Function Ref
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(v) => write!(f, "{}", v),
            Value::Int(v) => write!(f, "{}", v),
            Value::Float(v) => write!(f, "{}", v),
            Value::List(v) => {
                write!(
                    f,
                    "[{}]",
                    v.iter()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(",")
                )
            }
            Value::Table(v) => {
                write!(
                    f,
                    "[{}]",
                    v.iter()
                        .map(|(k, v)| format!("{}:{}", k, v.to_string()))
                        .collect::<Vec<String>>()
                        .join(",")
                )
            }
            Value::FunctionRef(v) => write!(f, "{}", v.name),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct MetaValue {
    pub value: Value,
    pub meta: Table,
}

impl Display for MetaValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl MetaValue {
    pub fn new(value: Value) -> Self {
        Self {
            value,
            meta: Table::new(),
        }
    }

    pub fn type_name(&self) -> String {
        match self.value {
            Value::Bool(_) => String::from("Bool"),
            Value::Int(_) => String::from("Int"),
            Value::Float(_) => String::from("Float"),
            Value::List(_) => String::from("List"),
            Value::Table(_) => String::from("Table"),
            Value::FunctionRef(_) => String::from("FunctionRef"),
        }
    }

    pub fn bool(val: bool) -> Self {
        Self::new(Value::Bool(val))
    }

    pub fn int(val: i64) -> Self {
        Self::new(Value::Int(val))
    }

    pub fn float(val: f64) -> Self {
        Self::new(Value::Float(F64(val)))
    }

    pub fn list(val: List) -> Self {
        Self::new(Value::List(val))
    }

    pub fn table(val: Table) -> Self {
        Self::new(Value::Table(val))
    }
    pub fn function_ref(val: FunctionRef) -> Self {
        Self::new(Value::FunctionRef(val))
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

impl From<i64> for MetaValue {
    fn from(v: i64) -> Self {
        MetaValue::int(v)
    }
}
