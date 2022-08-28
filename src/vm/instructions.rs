#[derive(Debug)]
pub enum Inst {
    Nop,
    // Bool
    PushB(bool),
    And,
    Or,
    Xor,
    Not,
    // Int
    PushI(i128),
    IntoInt,
    AddI,
    SubI,
    MulI,
    DivI,
    ModI,
    // Float
    PushF(f64),
    IntoFloat,
    AddF,
    SubF,
    MulF,
    DivF,
    ModF,
    // Logic and Control Flow
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,
    Jump(usize),
    Branch(usize),
    Call(String),
    // Locals
    Reserve(usize),
    Load(usize),
    Store(usize),
    // Stack
    Dup,
    Drop,
    Swap,
}
