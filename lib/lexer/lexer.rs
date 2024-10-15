use lazy_static::lazy_static;
use std::{collections::HashMap, iter::Peekable, str::Chars};
use strum_macros::{Display, EnumString};

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = HashMap::from([
        ("func", TokenType::Func),
        ("struct", TokenType::Struct),
        ("method", TokenType::Method),
        ("enum", TokenType::Enum),
        ("if", TokenType::If),
        ("else", TokenType::Else),
        ("match", TokenType::Match),
        ("field", TokenType::Field),
        ("constrained", TokenType::Constrained),
        ("new", TokenType::New),
        ("return", TokenType::Return),
        ("protocol", TokenType::Protocol),
    ]);
}

#[derive(Debug, Clone, PartialEq, Display, EnumString)]
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
    Colon,       // :

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
    #[strum(serialize = "Variable({0})")]
    Variable(String),
    #[strum(serialize = "Identifier({0})")]
    Identifier(String),

    // Literals
    // TODO(tefached95): Remove?
    #[strum(serialize = "IntLiteral({0})")]
    IntLiteral(i64),
    #[strum(serialize = "UintLiteral({0})")]
    UintLiteral(u64),
    FloatLiteral(f64),
    #[strum(serialize = "StringLiteral(\"{0}\")")]
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

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({}:{})", self.token_type, self.line, self.column)
    }
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
        Self {
            source: source.chars().peekable(),
            line: 1,
            column: 1,
            tokens: vec![],
            indentation_stack: vec![],
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        while let Some(c) = self.peek_next() {
            match c {
                ' ' | '\t' => {
                    self.advance_by(1);
                    continue;
                }
                '\n' | '\r' => {
                    self.advance_by(1);
                    let ret = Some(Token {
                        token_type: TokenType::EOL,
                        line: self.line - 1,
                        column: self.column,
                    });
                    self.line += 1;
                    self.column = 1;
                    return ret;
                }
                '~' => {
                    self.advance_by(1);
                    if let Some(ch) = self.peek_next() {
                        if ch == '>' {
                            self.advance_by(1);
                            return Some(Token {
                                token_type: TokenType::WavyArrow,
                                line: self.line,
                                column: self.column,
                            });
                        }
                    }
                }
                '-' => {
                    self.advance_by(1);
                    if let Some(ch) = self.peek_next() {
                        if ch == '-' {
                            self.advance_up_to(|c| c == '\n');
                            return Some(Token {
                                token_type: TokenType::LineComment,
                                line: self.line,
                                column: self.column,
                            });
                        } else if ch == '>' {
                            self.advance_by(1);

                            return Some(Token {
                                token_type: TokenType::ThinArrow,
                                line: self.line,
                                column: self.column,
                            });
                        } else {
                            return Some(Token {
                                token_type: TokenType::Minus,
                                line: self.line,
                                column: self.column,
                            });
                        }
                    }
                }
                ':' => {
                    self.advance_by(1);
                    if let Some(ch) = self.peek_next() {
                        if ch == ':' {
                            self.advance_by(1);

                            return Some(Token {
                                token_type: TokenType::DoubleColon,
                                line: self.line,
                                column: self.column,
                            });
                        } else if ch == '=' {
                            self.advance_by(1);

                            return Some(Token {
                                token_type: TokenType::InferAssign,
                                line: self.line,
                                column: self.column,
                            });
                        } else {
                            return Some(Token {
                                token_type: TokenType::Colon,
                                line: self.line,
                                column: self.column,
                            });
                        }
                    }
                }
                '$' => {
                    self.advance_by(1);

                    return Some(Token {
                        token_type: TokenType::Colon,
                        line: self.line,
                        column: self.column,
                    });
                }
                '0'..='9' => {
                    let number = self.handle_number();
                    return Some(Token {
                        token_type: TokenType::IntLiteral(number),
                        line: self.line,
                        column: self.column,
                    });
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let identifier = self.handle_identifier();
                    if let Some(kw) = KEYWORDS.get(&identifier.as_str()) {
                        return Some(Token {
                            token_type: kw.clone(),
                            line: self.line,
                            column: self.column,
                        });
                    }
                    return Some(Token {
                        token_type: TokenType::Identifier(identifier),
                        line: self.line,
                        column: self.column,
                    });
                }
                '#' => {
                    self.advance_by(1);
                    if let Some(ch) = self.peek_next() {
                        if ch == '{' {
                            self.advance_by(1);
                            return Some(Token {
                                token_type: TokenType::OpenHashBrace,
                                line: self.line,
                                column: self.column,
                            });
                        }
                    }
                }
                '}' => {
                    self.advance_by(1);
                    if let Some(ch) = self.peek_next() {
                        if ch == '#' {
                            self.advance_by(1);
                            return Some(Token {
                                token_type: TokenType::CloseHashBrace,
                                line: self.line,
                                column: self.column,
                            });
                        }
                    }
                }
                '"' => {
                    let string = self.handle_string();
                    return Some(Token {
                        token_type: TokenType::StringLiteral(string),
                        line: self.line,
                        column: self.column,
                    });
                }
                _ => todo!("Handle character: {}", c),
            }

            self.advance_by(1);
        }

        None
    }

    pub fn peek_next(&mut self) -> Option<char> {
        self.source.peek().copied()
    }

    pub fn advance_by(&mut self, amount: usize) {
        for _ in 0..amount {
            self.source.next();
        }
        self.column += amount;
    }

    /// Advances the lexer up to, but not including, the char that the predicate matches.
    ///
    /// Example:
    /// ```
    /// use petram::lexer::Lexer;
    /// let source = String::from("hello world");
    /// let mut lexer = Lexer::new(&source);
    /// lexer.advance_until(|s| s == 'w');
    /// assert_eq!(lexer.peek_next(), Some('w'));
    /// ```
    pub fn advance_up_to(&mut self, predicate: impl Fn(char) -> bool) {
        while let Some(c) = self.peek_next() {
            if predicate(c) {
                break;
            }
            self.advance_by(1);
        }
    }

    pub fn handle_string(&mut self) -> String {
        self.advance_by(1);
        let mut string = String::new();
        while let Some(c) = self.peek_next() {
            if c == '"' {
                self.advance_by(1);
                break;
            }
            string.push(c);
            self.advance_by(1);
        }
        return string;
    }

    pub fn handle_number(&mut self) -> i64 {
        let mut number = String::new();

        while let Some(c) = self.peek_next() {
            if c.is_digit(10) {
                number.push(c);
                self.advance_by(1);
            } else {
                break;
            }
        }

        let value = number
            .parse::<i64>()
            .expect("Could not parse integer value");
        value
    }

    pub fn handle_identifier(&mut self) -> String {
        let mut identifier = String::new();

        while let Some(c) = self.peek_next() {
            if c.is_whitespace() {
                break;
            } else if c.is_alphanumeric() || c == '_' {
                identifier.push(c);
                self.advance_by(1);
            } else {
                break;
            }
        }

        return identifier;
    }

    pub fn handle_indentation(&mut self) {
        let mut indent_level = 0;

        while let Some(c) = self.peek_next() {
            match c {
                ' ' | '\t' => {
                    self.advance_by(1);
                    indent_level += 1;
                }
                _ => break,
            }
        }

        let current_indent = *self.indentation_stack.last().unwrap_or(&0);

        if indent_level > current_indent {
            self.indentation_stack.push(indent_level);
            self.tokens.push(Token {
                token_type: TokenType::Indent,
                line: self.line,
                column: self.column,
            });
        } else if indent_level < current_indent {
            while !self.indentation_stack.is_empty()
                && indent_level < *self.indentation_stack.last().unwrap()
            {
                self.indentation_stack.pop();
                self.tokens.push(Token {
                    token_type: TokenType::Dedent,
                    line: self.line,
                    column: self.column,
                });
            }
            if indent_level != *self.indentation_stack.last().unwrap_or(&0) {
                // Indentation error: mismatched indent level
                panic!("Indentation error at line {}", self.line);
            }
        }

        self.column += indent_level;
    }
}
