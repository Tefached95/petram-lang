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
    ThinArrow,
    FatArrow,
    WavyArrow,
    Comma,
    DoubleColon,

    // Indentation with level
    #[strum(serialize = "Indent({0})")]
    Indent(usize),

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
    #[strum(serialize = "FloatLiteral({0})")]
    FloatLiteral(f64),
    #[strum(serialize = "StringLiteral(\"{0}\")")]
    StringLiteral(String),
    #[strum(serialize = "CharLiteral({0})")]
    CharLiteral(char),

    // Misc
    LineComment,  // -- This is a comment
    BlockComment, // {- This is a comment -}
    EOL,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
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
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a String) -> Self {
        Self {
            source: source.chars().peekable(),
            line: 1,
            column: 1,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.column == 1 {
            if let Some(indent) = self.handle_indentation() {
                return Some(indent);
            }
        }

        let start_line = self.line;
        let start_column = self.column + self.skip_whitespace();

        while let Some(c) = self.peek_next() {
            match c {
                '\n' | '\r' => {
                    self.advance_by(1);
                    let ret = Some(Token {
                        token_type: TokenType::EOL,
                        line: start_line,
                        column: start_column,
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
                                line: start_line,
                                column: start_column,
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
                                line: start_line,
                                column: start_column,
                            });
                        } else if ch == '>' {
                            self.advance_by(1);

                            return Some(Token {
                                token_type: TokenType::ThinArrow,
                                line: start_line,
                                column: start_column,
                            });
                        } else {
                            return Some(Token {
                                token_type: TokenType::Minus,
                                line: start_line,
                                column: start_column,
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
                                line: start_line,
                                column: start_column,
                            });
                        } else if ch == '=' {
                            self.advance_by(1);

                            return Some(Token {
                                token_type: TokenType::InferAssign,
                                line: start_line,
                                column: start_column,
                            });
                        } else {
                            return Some(Token {
                                token_type: TokenType::Colon,
                                line: start_line,
                                column: start_column,
                            });
                        }
                    }
                }
                '$' => {
                    self.advance_by(1);

                    return Some(Token {
                        token_type: TokenType::Colon,
                        line: start_line,
                        column: start_column,
                    });
                }
                '0'..='9' => {
                    let number = self.handle_number();
                    return Some(Token {
                        token_type: TokenType::IntLiteral(number),
                        line: start_line,
                        column: start_column,
                    });
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let identifier = self.handle_identifier();
                    if let Some(kw) = KEYWORDS.get(&identifier.as_str()) {
                        return Some(Token {
                            token_type: kw.clone(),
                            line: start_line,
                            column: start_column,
                        });
                    }
                    return Some(Token {
                        token_type: TokenType::Identifier(identifier),
                        line: start_line,
                        column: start_column,
                    });
                }
                '#' => {
                    self.advance_by(1);
                    if let Some(ch) = self.peek_next() {
                        if ch == '{' {
                            self.advance_by(1);
                            return Some(Token {
                                token_type: TokenType::OpenHashBrace,
                                line: start_line,
                                column: start_column,
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
                                line: start_line,
                                column: start_column,
                            });
                        }
                    }
                }
                '"' => {
                    let string = self.handle_string();
                    return Some(Token {
                        token_type: TokenType::StringLiteral(string),
                        line: start_line,
                        column: start_column,
                    });
                }
                _ => todo!("Handle character: {}", c),
            }

            // self.advance_by(1);
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

    pub fn skip_whitespace(&mut self) -> usize {
        let mut whitespace_count = 0;
        // Skip whitespace and update column
        while let Some(c) = self.peek_next() {
            match c {
                ' ' => {
                    self.advance_by(1);
                    whitespace_count += 1;
                }
                '\t' => {
                    self.advance_by(1);
                    whitespace_count += 4;
                }
                _ => break,
            }
        }
        whitespace_count
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
    pub fn advance_up_to<F>(&mut self, predicate: F)
    where
        F: Fn(char) -> bool,
    {
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
        let number: String = self
            .source
            .by_ref()
            .take_while(|c| c.is_digit(10))
            .collect();

        self.column += number.len();

        number
            .parse::<i64>()
            .expect("Could not parse integer value")
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

    pub fn handle_indentation(&mut self) -> Option<Token> {
        let mut indentation_level = 0;

        while let Some(c) = self.peek_next() {
            match c {
                ' ' => indentation_level += 1,
                '\t' => indentation_level += 4,
                _ => break,
            }
            self.advance_by(1);
        }

        if indentation_level == 0 {
            return None;
        }

        // TODO(tefached95): determine a way to detect tab width and use that instead of hardcoding 4
        let indentation_width = (indentation_level + 3) / 4; // Round up division by 4

        let t = Some(Token {
            token_type: TokenType::Indent(indentation_width),
            line: self.line,
            column: 1, // indentation is always on column 1
        });

        self.column += indentation_level - 1; // hack?

        t
    }
}
