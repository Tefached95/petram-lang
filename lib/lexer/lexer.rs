pub enum Token {
    // Types
    Int8,
    Int16,
    Int32,
    Int64,
    Int,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Uint,
    Float32,
    Float64,
    Float,
    Bool,
    String,
    Char,

    // Keywords
    Func,
    Struct,
    Method,
    Enum,
    If,
    Else,
    Match,
    Field,
    Constrained,
    New,
    Return,
    Protocol,

    // Math operators
    Plus,
    Minus,
    Mul,
    Div,
    Mod,

    // Logic operators
    Not,
    And,
    Or,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,

    // Assignment
    Assign,      // =
    InferAssign, // :=
    TypeDecl,    // :

    // Punctuation
    OpenHashBrace,
    CloseHashBrace,
    OpenList,
    CloseList,
    OpenGenericBracket,
    CloseGenericBracker,
    Comma,
    Whitespace,
    DoubleColon,

    // Identifiers
    Variable(String),
    FunctionName(String),
    MethodName(String),
    ProtocolName(String),

    // Literals
    // TODO(tefached95): Remove?
    IntLiteral(i64),
    UintLiteral(u64),
    FloatLiteral(f64),
    StringLiteral(String),
    CharLiteral(char),

    // Misc
    LineComment(String),  // -- This is a comment
    BlockComment(String), // {- This is a comment -}
    EOF,
    EOL,
}

#[derive(Debug)]
pub struct Lexer {
    source: String,
    line: usize,
    column: usize,
    indentation_stack: Vec<usize>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source,
            line: 1,
            column: 1,
            indentation_stack: vec![0],
        }
    }
}
