use petram::lexer::Lexer;

fn main() {
    let lx = Lexer::new("../examples/hello_world.petra".to_string());
    println!("{}", lx.say_hello("Hello, Petram!"));
}
