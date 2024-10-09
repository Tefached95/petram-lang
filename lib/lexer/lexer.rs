#[derive(Debug)]
pub enum TokenType {
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
    ThinArrow,
    FatArrow,
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
    LineComment,  // -- This is a comment
    BlockComment, // {- This is a comment -}
    EOF,
    EOL,
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    line: usize,
    column: usize,
}

#[derive(Debug)]
pub struct Lexer {
    source: String,
    line: usize,
    column: usize,
    tokens: Vec<Token>,
    indentation_stack: Vec<usize>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source,
            line: 1,
            column: 1,
            tokens: vec![],
            indentation_stack: vec![0],
        }
    }

    //TODO(tefached95): Figure out a better way to handle line and column numbering instead of having to manually call `advance_by()` in every match arm
    pub fn next_token(&mut self) {
        while let Some(c) = self.peek_next() {
            match c {
                '#' => match self.peek_next() {
                    Some('{') => {
                        self.tokens.push(Token {
                            token_type: TokenType::OpenHashBrace,
                            line: self.line,
                            column: self.column,
                        });
                        self.advance_by(2);
                    }
                    _ => {
                    }
                },
                '-' => {
                    match self.peek_next() {
                        Some('-') => {
                            // Single line comment, parse until end of line
                            self.tokens.push(Token {
                                token_type: TokenType::LineComment,
                                line: self.line,
                                column: self.column,
                            });
                            self.advance_until(|str: String| {
                                str != "\n" || str != "\r\n"
                            });
                        },
                        Some('>') => {
                            // Thin arrow ->
                            self.tokens.push(Token {
                                token_type: TokenType::ThinArrow,
                                line: self.line,
                                column: self.column,
                            });
                            self.advance_by(2);
                        }
                        _ => {
                            // Minus operator
                        }
                    }
                }
                _ => {
                    self.tokens.push(Token {
                        token_type: TokenType::EOF,
                        line: 0,
                        column: 0,
                    })
                }
            }
        }
    }

    pub fn peek_next(&self) -> Option<char> {
        self.source.chars().peekable().peek().copied()
    }

    pub fn advance_by(&mut self, amount: usize) {
        self.source.chars().nth(amount);
        self.column += amount;
    }

    pub fn advance_until(&mut self, predicate: impl Fn(String) -> bool) {
        let mut amt: usize = 0;
        while !predicate(
            self.source
                .chars()
                .nth(0)
                .expect("Could not get the next char")
                .to_string(),
        ) {
            amt += 1;
        }
        self.advance_by(amt);
    }
}
