pub struct Ast {
    pub imports: Vec<String>,
    pub definitions: Vec<Definition>,
}

pub struct Definition {
    pub name: String,
    pub body: Vec<Expr>,
}

pub enum Expr {
    Int(i64),
    Float(f64),
    Char(char),
    Str(String),
    Term(String),
    Closure(Vec<Expr>),
}
