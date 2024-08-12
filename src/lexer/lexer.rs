// src/lexer/lexer.rs

use std::fmt;
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
    Variable(String),

    // Types
    Type(String),
    GenericTypeIdentifiers(Vec<String>),

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
    DoubleColon,

    // Simplified bracket tokens
    OpenBracket(char),  // Can be '(', '[', or '{'
    CloseBracket(char), // Can be ')', ']', or '}'
    ListBraceOpen,      // For '{|'
    ListBraceClose,     // For '|}'

    // Special
    ExprStart,
    ExprEnd, // For #[ and ]#
    Comment(String),
    Error(String),
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
        let ch = self.input.next();
        if let Some(c) = ch {
            self.column += 1;
            if c == '\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
        }
        ch
    }

    fn advance_by(&mut self, amount: usize) {
        for _ in 0..amount {
            if let Some(c) = self.input.next() {
                self.column += 1;
                if c == '\n' {
                    self.line += 1;
                    self.column = 0;
                }
            } else {
                break;
            }
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn peek_next(&mut self) -> Option<char> {
        self.input.clone().nth(1)
    }

    fn peek_by(&mut self, amount: usize) -> Option<String> {
        let mut peeked = String::new();
        for _ in 0..amount {
            peeked.push(self.input.next().unwrap());
        }
        Some(peeked)
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.peek() {
            if ch.is_whitespace() && ch != '\n' {
                self.advance();
            } else {
                break;
            }
        }
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
                self.advance_by(4);
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
                    column: 0,
                });
            }
            if spaces != *self.indentation_stack.last().unwrap() {
                // Mismatched indentation
                panic!("Indentation error at line {}", self.line);
            }
        }

        tokens
    }

    fn parse_type_identifier(&mut self) -> Option<String> {
        let mut identifier = String::new();

        if let Some(&ch) = self.peek() {
            if ch.is_ascii_uppercase() {
                identifier.push(ch);
                self.advance();

                while let Some(&ch) = self.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        identifier.push(ch);
                        self.advance();
                    } else {
                        break;
                    }
                }

                Some(identifier)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn parse_identifier(&mut self) -> Option<(String, usize)> {
        let mut identifier = String::new();
        let start_column = self.column;
        let mut expect_lowercase_or_underscore = true;

        while let Some(&ch) = self.peek() {
            match ch {
                'a'..='z' => {
                    identifier.push(ch);
                    expect_lowercase_or_underscore = true;
                    self.advance();
                }
                'A'..='Z' => {
                    return None; // Uppercase letters are not allowed
                }
                '0'..='9' => {
                    if identifier.is_empty() {
                        return None; // Identifiers can't start with a number
                    }
                    identifier.push(ch);
                    expect_lowercase_or_underscore = true;
                    self.advance();
                }
                '_' => {
                    if !expect_lowercase_or_underscore || identifier.ends_with('_') {
                        return None; // Consecutive underscores or underscore after a number are not allowed
                    }
                    identifier.push(ch);
                    expect_lowercase_or_underscore = false;
                    self.advance();
                }
                _ => break,
            }
        }

        if identifier.is_empty() || identifier.ends_with('_') {
            None
        } else {
            Some((identifier, start_column))
        }
    }

    fn error_token(&self, message: &str, line: usize, column: usize) -> Token {
        Token {
            token_type: TokenType::Error(message.to_string()),
            line,
            column,
        }
    }

    pub fn next_token(&mut self) -> Token {
        if self.column == 0 {
            // We're at the start of a new line
            let mut indent_tokens = self.handle_indentation();
            if !indent_tokens.is_empty() {
                return indent_tokens.remove(0);
            }
        }

        let start_column = self.column;
        let start_line = self.line;

        self.skip_whitespace();

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
                        line: self.line - 2, 
                        column: self.column,
                    };
                }

                // Comments or single arrow ->
                '-' => {
                    self.advance(); // Consume the first '-'
                    if self.peek() == Some(&'>') {
                        // Single arrow ->
                        self.advance();
                        return Token {
                            token_type: TokenType::Arrow,
                            line: self.line,
                            column: self.column - 2,
                        };
                    } else if self.peek() == Some(&'-') {
                        self.advance(); // Consume second '-'
                        let mut comment_content = String::new();
                        while let Some(&ch) = self.peek() {
                            if ch == '\n' || ch == '\r' {
                                break;
                            }
                            comment_content.push(ch);
                            self.advance();
                        }
                        return Token {
                            token_type: TokenType::Comment(comment_content.trim().to_string()),
                            line: start_line,
                            column: start_column + comment_content.len(),
                        };
                    } else {
                        return Token {
                            token_type: TokenType::Minus,
                            line: start_line,
                            column: start_column,
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
                            line: start_line,
                            column: start_column,
                        };
                    } else {
                        return Token {
                            token_type: TokenType::OpenBracket(bracket),
                            line: start_line,
                            column: start_column,
                        };
                    }
                }
                ')' | ']' | '}' => {
                    let bracket = self.advance().unwrap();
                    if self.peek() == Some(&'#') {
                        self.advance();
                        return Token {
                            token_type: TokenType::ExprEnd,
                            line: start_line,
                            column: start_column,
                        };
                    }
                    return Token {
                        token_type: TokenType::CloseBracket(bracket),
                        line: start_line,
                        column: start_column,
                    };
                }

                // Bitwise operators, pipe, or closing list brace
                '|' => {
                    self.advance();
                    if self.peek() == Some(&'}') {
                        self.advance();
                        return Token {
                            token_type: TokenType::ListBraceClose,
                            line: start_line,
                            column: start_column,
                        };
                    } else if self.peek() == Some(&'>') {
                        self.advance();
                        return Token {
                            token_type: TokenType::Pipe,
                            line: start_line,
                            column: start_column,
                        };
                    } else if self.peek() == Some(&'|') {
                        self.advance();
                        return Token {
                            token_type: TokenType::Or,
                            line: start_line,
                            column: start_column,
                        };
                    } else {
                        return Token {
                            token_type: TokenType::BitwiseOr,
                            line: start_line,
                            column: start_column,
                        };
                    }
                }
                '&' => {
                    self.advance();
                    if self.peek() == Some(&'&') {
                        self.advance();
                        return Token {
                            token_type: TokenType::And,
                            line: start_line,
                            column: start_column,
                        };
                    } else {
                        return Token {
                            token_type: TokenType::BitwiseAnd,
                            line: start_line,
                            column: start_column,
                        };
                    }
                }
                '^' => {
                    self.advance();
                    return Token {
                        token_type: TokenType::BitwiseXor,
                        line: start_line,
                        column: start_column,
                    };
                }
                '~' => {
                    self.advance();
                    return Token {
                        token_type: TokenType::BitwiseNot,
                        line: start_line,
                        column: start_column,
                    };
                }
                '<' => {
                    self.advance();
                    match self.peek() {
                        Some(&'<') => {
                            self.advance();
                            return Token {
                                token_type: TokenType::LeftShift,
                                line: start_line,
                                column: start_column,
                            };
                        }
                        Some(&'=') => {
                            self.advance();
                            return Token {
                                token_type: TokenType::LessEqual,
                                line: start_line,
                                column: start_column,
                            };
                        }
                        Some(&ch) if ch.is_uppercase() => {
                            let mut type_identifiers = vec![];
                            let start_column = self.column;

                            loop {
                                if let Some(identifier) = self.parse_type_identifier() {
                                    type_identifiers.push(identifier);
                                }

                                match self.peek() {
                                    Some(&',') => {
                                        self.advance();
                                        self.skip_whitespace();
                                    }
                                    Some(&'>') => {
                                        self.advance();
                                        break;
                                    }
                                    _ => {
                                        return self.error_token("Expected ',' or '>' after type identifier", self.line, self.column);
                                    }
                                }
                            }
                            return Token {
                                token_type: TokenType::GenericTypeIdentifiers(type_identifiers),
                                line: start_line,
                                column: start_column,
                            };
                        }
                        _ => {
                            return Token {
                                token_type: TokenType::LessThan,
                                line: start_line,
                                column: start_column,
                            };
                        }
                    }
                }
                '>' => {
                    self.advance();
                    if self.peek() == Some(&'>') {
                        self.advance();
                        return Token {
                            token_type: TokenType::RightShift,
                            line: start_line,
                            column: start_column,
                        };
                    } else if self.peek() == Some(&'=') {
                        self.advance();
                        return Token {
                            token_type: TokenType::GreaterEqual,
                            line: start_line,
                            column: start_column,
                        };
                    } else {
                        return Token {
                            token_type: TokenType::GreaterThan,
                            line: start_line,
                            column: start_column,
                        };
                    }
                }

                // Variables
                '$' => {
                    self.advance();
                    if let Some((identifier, _id_start_column)) = self.parse_identifier() {
                        return Token {
                            token_type: TokenType::Variable(identifier),
                            line: start_line,
                            column: start_column,
                        }
                    } else {
                        return self.error_token("Invalid variable name. Must be in snake_case.", start_line, start_column)
                    }
                }
                // Other operators
                '+' => {
                    self.advance();
                    return Token {
                        token_type: TokenType::Plus,
                        line: start_line,
                        column: start_column,
                    };
                }
                '*' => {
                    self.advance();
                    return Token {
                        token_type: TokenType::Multiply,
                        line: start_line,
                        column: start_column,
                    };
                }
                '/' => {
                    self.advance();
                    return Token {
                        token_type: TokenType::Divide,
                        line: start_line,
                        column: start_column,
                    };
                }
                '%' => {
                    self.advance();
                    return Token {
                        token_type: TokenType::Modulo,
                        line: start_line,
                        column: start_column,
                    };
                }
                '=' => {
                    self.advance();
                    if self.peek() == Some(&'=') {
                        self.advance();
                        return Token {
                            token_type: TokenType::Equal,
                            line: start_line,
                            column: start_column,
                        };
                    } else if self.peek() == Some(&'>') {
                        self.advance();
                        return Token {
                            token_type: TokenType::FatArrow,
                            line: start_line,
                            column: start_column,
                        };
                    } else {
                        return Token {
                            token_type: TokenType::Assign,
                            line: start_line,
                            column: start_column,
                        };
                    }
                }
                '!' => {
                    self.advance();
                    if self.peek() == Some(&'=') {
                        self.advance();
                        return Token {
                            token_type: TokenType::NotEqual,
                            line: start_line,
                            column: start_column,
                        };
                    } else {
                        return Token {
                            token_type: TokenType::Not,
                            line: start_line,
                            column: start_column,
                        };
                    }
                }
                ':' => {
                    self.advance();
                    match self.peek() {
                        Some(&'=') => {
                            self.advance();
                            return Token {
                                token_type: TokenType::DeclareAssign,
                                line: start_line,
                                column: start_column,
                            };
                        }
                        Some(&':') => {
                            self.advance();
                            return Token {
                                token_type: TokenType::DoubleColon,
                                line: start_line,
                                column: start_column,
                            };
                        }
                        Some(&ch) if ch.is_whitespace() || ch.is_uppercase() => {
                            let mut type_name = String::new();

                            self.skip_whitespace();

                            while let Some(&ch) = self.peek() {
                                if ch.is_alphanumeric() {
                                    type_name.push(ch);
                                    self.advance();
                                } else {
                                    break;
                                }
                            }

                            return Token {
                                token_type: TokenType::Type(type_name.trim().to_string()),
                                line: start_line,
                                column: start_column,
                            };
                        }
                        _ => {
                            return Token {
                                token_type: TokenType::Colon,
                                line: start_line,
                                column: start_column,
                            };
                        }
                    }
                }
                // Other tokens
                ',' => {
                    self.advance();
                    return Token {
                        token_type: TokenType::Comma,
                        line: start_line,
                        column: start_column,
                    };
                }
                '.' => {
                    self.advance();
                    return Token {
                        token_type: TokenType::Dot,
                        line: start_line,
                        column: start_column,
                    };
                }
                '#' => {
                    self.advance();
                    if self.peek() == Some(&'[') {
                        self.advance();
                        return Token {
                            token_type: TokenType::ExprStart,
                            line: start_line,
                            column: start_column,
                        };
                    }
                }
                // Identifiers and keywords
                'a'..='z' | '_' => {
                    if let Some((identifier, id_start_column)) = self.parse_identifier() {
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
                            line: start_line,
                            column: id_start_column,
                        }
                    } else {
                        return self.error_token("Invalid identifier. Must be in snake_case.", start_line, start_column)
                    }
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
                        line: start_line,
                        column: start_column,
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
                        line: start_line,
                        column: start_column,
                    };
                }

                _ => {
                    // Unrecognized character
                    self.advance();
                    return Token {
                        token_type: TokenType::EOF,
                        line: start_line,
                        column: start_column,
                    };
                }
            }
        }
        Token {
            token_type: TokenType::EOF,
            line: start_line,
            column: start_column,
        }
    }
}
