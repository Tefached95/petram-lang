use std::{iter::Peekable, str::Chars};

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
    WavyArrow,
    Comma,
    Whitespace,
    DoubleColon,

    // Indentation
    Indent,
    Dedent,

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
pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>,
    line: usize,
    column: usize,
    tokens: Vec<Token>,
    indentation_stack: Vec<usize>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a String) -> Self {
        let lexer = Self {
            source: source.chars().peekable(),
            line: 1,
            column: 1,
            tokens: vec![],
            indentation_stack: vec![0],
        };
        lexer
    }

    pub fn next_token(&mut self) -> Option<Token> {
        None
    }

    pub fn peek_next(&mut self) -> Option<char> {
        self.source.peek().copied()
    }

    pub fn advance_by(&mut self, amount: usize) {
        self.source.nth(amount);
        self.column += amount;
    }

    pub fn advance_until(&mut self, predicate: impl Fn(String) -> bool) {
        let mut amt: usize = 0;
        while !predicate(
            self.source
                .next()
                .expect("Could not get the next char")
                .to_string(),
        ) {
            amt += 1;
        }
        self.advance_by(amt);
    }

    pub fn handle_indentation(&mut self) {
        let mut indent_level = 0;
        
        while let Some(c) = self.peek_next() {
            match c {
                ' ' => {
                    self.advance_by(1);
                    indent_level += 1;
                }
                '\t' => {
                    self.advance_by(4);
                    indent_level += 4;
                }
                _ => break,
            }
        }

        let mut current_indent = self.indentation_stack.last().unwrap();
        
        if indent_level > *current_indent {
            self.indentation_stack.push(indent_level);
            self.tokens.push(Token {
                token_type: TokenType::Indent,
                line: self.line,
                column: self.column,
            });
        } else if indent_level < *current_indent {
            while indent_level < *current_indent {
                self.indentation_stack.pop();
                self.tokens.push(Token {
                    token_type: TokenType::Dedent,
                    line: self.line,
                    column: self.column,
                });
                current_indent = self.indentation_stack.last().unwrap();
            }
        } else {
            eprintln!("Error: Invalid indentation level on line {}", self.line);
            std::process::exit(1);
        }
    }
}
