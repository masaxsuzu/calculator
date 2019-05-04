pub mod ast;
pub mod compiler;
pub mod evaluator;
pub mod lexer;
pub mod object;
pub mod parser;
pub mod token;

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let mut input = read::<String>();
    input.push('\0');

    let lexer = lexer::Lexer::new(&input);
    let mut parser = parser::Parser::new(lexer);

    let program = parser.parse();
    let compiler = compiler::RPNCompiler::new();

    match compiler.compile(program) {
        Some(o) => println!("{}", o),
        _ => {
            println!("parser error");
        }
    }
}
