pub struct Lexer {
    source: String,
    position: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self { source, position: 0 }
    }

    pub fn say_hello<'a>(&self, what_to_say: &'a str) -> &'a str {
        what_to_say
    }
}