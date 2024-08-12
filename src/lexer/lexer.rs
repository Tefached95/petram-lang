// src/lexer/lexer.rs

use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Keywords
    Func,
    Struct,
    Trait,
    New,
    Return,
    Constrain,
    Field,
    Method,
    If,
    Else,
    Foreach,
    In,
    Match,

    // Literals
    Identifier(String),
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),

    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    Not,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    LeftShift,
    RightShift,
    Assign,
    DeclareAssign,
    Colon,
    Arrow,
    FatArrow,
    Pipe,

    // Simplified bracket tokens
    OpenBracket(char),  // Can be '(', '[', or '{'
    CloseBracket(char), // Can be ')', ']', or '}'
    ListBraceOpen,      // For '{|'
    ListBraceClose,     // For '|}'

    // Special
    ExprStart,
    ExprEnd, // For #[ and ]#
    Comment,
    Indent,
    Dedent,
    Newline,
    EOF,

    // Other delimiters
    Comma,
    Dot,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    line: usize,
    column: usize,
    indentation_stack: Vec<usize>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
            line: 1,
            column: 0,
            indentation_stack: vec![0], // Start with 0 indentation
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            let is_eof = token.token_type == TokenType::EOF;
            tokens.push(token);
            if is_eof {
                break;
            }
        }
        tokens
    }

    fn advance(&mut self) -> Option<char> {
        self.column += 1;
        self.input.next()
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn peek_next(&mut self) -> Option<char> {
        self.input.clone().nth(1)
    }

    fn handle_indentation(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut spaces = 0;

        // Count the spaces at the start of the line
        while let Some(&ch) = self.peek() {
            if ch == ' ' {
                spaces += 1;
                self.advance();
            } else if ch == '\t' {
                // Treat tabs as 4 spaces
                spaces += 4;
                self.advance();
            } else {
                break;
            }
        }

        let current_indent = *self.indentation_stack.last().unwrap();

        if spaces > current_indent {
            // Indentation increased
            self.indentation_stack.push(spaces);
            tokens.push(Token {
                token_type: TokenType::Indent,
                line: self.line,
                column: self.column,
            });
        } else if spaces < current_indent {
            // Indentation decreased
            while spaces < *self.indentation_stack.last().unwrap() {
                self.indentation_stack.pop();
                tokens.push(Token {
                    token_type: TokenType::Dedent,
                    line: self.line,
                    column: self.column,
                });
            }
            if spaces != *self.indentation_stack.last().unwrap() {
                // Mismatched indentation
                panic!("Indentation error at line {}", self.line);
            }
        }

        tokens
    }

    pub fn next_token(&mut self) -> Token {
        if self.column == 0 {
            // We're at the start of a new line
            let mut indent_tokens = self.handle_indentation();
            if !indent_tokens.is_empty() {
                return indent_tokens.remove(0);
            }
        }

        while let Some(ch) = self.peek() {
            match ch {
                // Whitespace
                ' ' | '\t' => {
                    self.advance();
                    continue;
                }
                '\n' | '\r' => {
                    self.advance();
                    self.line += 1;
                    self.column = 0;
                    return Token {
                        token_type: TokenType::Newline,
                        line: self.line - 1,
                        column: self.column,
                    };
                }

                // Comments or single arrow ->
                '-' => {
                    if self.peek() == Some(&'>') {
                        // Single arrow ->
                        self.advance();
                        return Token {
                            token_type: TokenType::Arrow,
                            line: self.line,
                            column: self.column - 1,
                        };
                    } else if self.peek_next() == Some('-') {
                        self.advance(); // Consume first '-'
                        self.advance(); // Consume second '-'
                        while let Some(ch) = self.peek() {
                            if *ch == '\n' || *ch == '\r' {
                                break;
                            }
                            self.advance();
                        }
                        return Token {
                            token_type: TokenType::Comment,
                            line: self.line,
                            column: self.column,
                        };
                    } else {
                        self.advance();
                        return Token {
                            token_type: TokenType::Minus,
                            line: self.line,
                            column: self.column,
                        };
                    }
                }

                // Brackets and braces
                '(' | '[' | '{' => {
                    let bracket = self.advance().unwrap();
                    if bracket == '{' && self.peek() == Some(&'|') {
                        self.advance();
                        return Token {
                            token_type: TokenType::ListBraceOpen,
                            line: self.line,
                            column: self.column - 1,
                        };
                    } else {
                        return Token {
                            token_type: TokenType::OpenBracket(bracket),
                            line: self.line,
                            column: self.column,
                        };
                    }
                }
                ')' | ']' | '}' => {
                    let bracket = self.advance().unwrap();
                    if self.peek() == Some(&'#') {
                        self.advance();
                        return Token {
                            token_type: TokenType::ExprEnd,
                            line: self.line,
                            column: self.column - 1,
                        };
                    }
                    return Token {
                        token_type: TokenType::CloseBracket(bracket),
                        line: self.line,
                        column: self.column,
                    };
                }

                // Bitwise operators and pipe
                '|' => {
                    self.advance();
                    if self.peek() == Some(&'}') {
                        self.advance();
                        return Token {
                            token_type: TokenType::ListBraceClose,
                            line: self.line,
                            column: self.column - 1,
                        };
                    } else if self.peek() == Some(&'>') {
                        self.advance();
                        return Token {
                            token_type: TokenType::Pipe,
                            line: self.line,
                            column: self.column - 1,
                        };
                    } else if self.peek() == Some(&'|') {
                        self.advance();
                        return Token {
                            token_type: TokenType::Or,
                            line: self.line,
                            column: self.column - 1,
                        };
                    } else {
                        return Token {
                            token_type: TokenType::BitwiseOr,
                            line: self.line,
                            column: self.column,
                        };
                    }
                }
                '&' => {
                    self.advance();
                    if self.peek() == Some(&'&') {
                        self.advance();
                        return Token {
                            token_type: TokenType::And,
                            line: self.line,
                            column: self.column - 1,
                        };
                    } else {
                        return Token {
                            token_type: TokenType::BitwiseAnd,
                            line: self.line,
                            column: self.column,
                        };
                    }
                }
                '^' => {
                    self.advance();
                    return Token {
                        token_type: TokenType::BitwiseXor,
                        line: self.line,
                        column: self.column,
                    };
                }
                '~' => {
                    self.advance();
                    return Token {
                        token_type: TokenType::BitwiseNot,
                        line: self.line,
                        column: self.column,
                    };
                }
                '<' => {
                    self.advance();
                    if self.peek() == Some(&'<') {
                        self.advance();
                        return Token {
                            token_type: TokenType::LeftShift,
                            line: self.line,
                            column: self.column - 1,
                        };
                    } else if self.peek() == Some(&'=') {
                        self.advance();
                        return Token {
                            token_type: TokenType::LessEqual,
                            line: self.line,
                            column: self.column - 1,
                        };
                    } else {
                        return Token {
                            token_type: TokenType::LessThan,
                            line: self.line,
                            column: self.column,
                        };
                    }
                }
                '>' => {
                    self.advance();
                    if self.peek() == Some(&'>') {
                        self.advance();
                        return Token {
                            token_type: TokenType::RightShift,
                            line: self.line,
                            column: self.column - 1,
                        };
                    } else if self.peek() == Some(&'=') {
                        self.advance();
                        return Token {
                            token_type: TokenType::GreaterEqual,
                            line: self.line,
                            column: self.column - 1,
                        };
                    } else {
                        return Token {
                            token_type: TokenType::GreaterThan,
                            line: self.line,
                            column: self.column,
                        };
                    }
                }

                // Other operators
                '+' => {
                    self.advance();
                    return Token {
                        token_type: TokenType::Plus,
                        line: self.line,
                        column: self.column,
                    };
                }
                '*' => {
                    self.advance();
                    return Token {
                        token_type: TokenType::Multiply,
                        line: self.line,
                        column: self.column,
                    };
                }
                '/' => {
                    self.advance();
                    return Token {
                        token_type: TokenType::Divide,
                        line: self.line,
                        column: self.column,
                    };
                }
                '%' => {
                    self.advance();
                    return Token {
                        token_type: TokenType::Modulo,
                        line: self.line,
                        column: self.column,
                    };
                }
                '=' => {
                    self.advance();
                    if self.peek() == Some(&'=') {
                        self.advance();
                        return Token {
                            token_type: TokenType::Equal,
                            line: self.line,
                            column: self.column - 1,
                        };
                    } else if self.peek() == Some(&'>') {
                        self.advance();
                        return Token {
                            token_type: TokenType::FatArrow,
                            line: self.line,
                            column: self.column - 1,
                        };
                    } else {
                        return Token {
                            token_type: TokenType::Assign,
                            line: self.line,
                            column: self.column,
                        };
                    }
                }
                '!' => {
                    self.advance();
                    if self.peek() == Some(&'=') {
                        self.advance();
                        return Token {
                            token_type: TokenType::NotEqual,
                            line: self.line,
                            column: self.column - 1,
                        };
                    } else {
                        return Token {
                            token_type: TokenType::Not,
                            line: self.line,
                            column: self.column,
                        };
                    }
                }
                ':' => {
                    self.advance();
                    if self.peek() == Some(&'=') {
                        self.advance();
                        return Token {
                            token_type: TokenType::DeclareAssign,
                            line: self.line,
                            column: self.column - 1,
                        };
                    } else {
                        return Token {
                            token_type: TokenType::Colon,
                            line: self.line,
                            column: self.column,
                        };
                    }
                }
                // Other tokens
                ',' => {
                    self.advance();
                    return Token {
                        token_type: TokenType::Comma,
                        line: self.line,
                        column: self.column,
                    };
                }
                '.' => {
                    self.advance();
                    return Token {
                        token_type: TokenType::Dot,
                        line: self.line,
                        column: self.column,
                    };
                }
                '#' => {
                    self.advance();
                    if self.peek() == Some(&'[') {
                        self.advance();
                        return Token {
                            token_type: TokenType::ExprStart,
                            line: self.line,
                            column: self.column - 1,
                        };
                    }
                }
                // Identifiers and keywords
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut identifier = String::new();
                    let start_column = self.column;

                    while let Some(&ch) = self.peek() {
                        if ch.is_alphanumeric() || ch == '_' {
                            identifier.push(ch);
                            self.advance();
                        } else {
                            break;
                        }
                    }

                    let token_type = match identifier.as_str() {
                        "func" => TokenType::Func,
                        "struct" => TokenType::Struct,
                        "trait" => TokenType::Trait,
                        "new" => TokenType::New,
                        "return" => TokenType::Return,
                        "constrain" => TokenType::Constrain,
                        "field" => TokenType::Field,
                        "method" => TokenType::Method,
                        "if" => TokenType::If,
                        "else" => TokenType::Else,
                        "foreach" => TokenType::Foreach,
                        "in" => TokenType::In,
                        "match" => TokenType::Match,
                        "true" => TokenType::Bool(true),
                        "false" => TokenType::Bool(false),
                        _ => TokenType::Identifier(identifier),
                    };
                    
                    return Token {
                        token_type,
                        line: self.line,
                        column: start_column,
                    };
                }

                // Numbers
                '0'..='9' => {
                    let mut number = String::new();
                    let mut is_float = false;
                    while let Some(&ch) = self.peek() {
                        if ch.is_digit(10) {
                            number.push(ch);
                            self.advance();
                        } else if ch == '.' && !is_float {
                            is_float = true;
                            number.push(ch);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    let token_type = if is_float {
                        TokenType::Float(number.parse().unwrap())
                    } else {
                        TokenType::Integer(number.parse().unwrap())
                    };
                    return Token {
                        token_type,
                        line: self.line,
                        column: self.column - number.len(),
                    };
                }

                // Strings
                '"' => {
                    self.advance();
                    let mut string = String::new();
                    while let Some(&ch) = self.peek() {
                        if ch == '"' {
                            self.advance();
                            break;
                        } else if ch == '\\' {
                            self.advance();
                            if let Some(&next_ch) = self.peek() {
                                string.push(match next_ch {
                                    'n' => '\n',
                                    't' => '\t',
                                    'r' => '\r',
                                    '\\' => '\\',
                                    '"' => '"',
                                    _ => next_ch,
                                });
                                self.advance();
                            }
                        } else {
                            string.push(ch);
                            self.advance();
                        }
                    }
                    return Token {
                        token_type: TokenType::String(string.clone()),
                        line: self.line,
                        column: self.column - string.len() - 2,
                    };
                }

                _ => {
                    // Unrecognized character
                    self.advance();
                    return Token {
                        token_type: TokenType::EOF,
                        line: self.line,
                        column: self.column,
                    };
                }
            }
        }
        Token {
            token_type: TokenType::EOF,
            line: self.line,
            column: self.column,
        }
    }
}
