#[derive(Debug)]
pub enum Inst {
    Nop,
    // Stack
    Dup,
    Drop,
    Swap,
    // Primitives
    PushB(bool),
    PushI(i64),
    PushF(f64),
    IntoInt,
    IntoFloat,
    PushList,
    PushTable,
    // List
    ListPush,
    ListPop,
    ListGet,
    ListSet,
    ListLen,
    // List
    TablePush,
    TablePop,
    TableGet,
    TableSet,
    TableLen,
    // Meta
    LoadMeta,
    StoreMeta,
    // Locals
    LocalReserve(usize),
    LocalLoad(usize),
    LocalStore(usize),
    // Boolean Operations
    And,
    Or,
    Xor,
    Not,
    // Arithmetic Operations
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    // Logic and Control Flow
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,
    // TODO, make them relative
    Branch(usize),
    BranchIf(usize),
    Call(String),
    Return,
    // TableCall - Indirect
}
