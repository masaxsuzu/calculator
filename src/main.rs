pub mod ast;
pub mod compiler;
pub mod evaluator;
pub mod lexer;
pub mod object;
pub mod parser;
pub mod token;

fn try_read_from_stdin<T: std::str::FromStr>() -> Result<T, T::Err> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.parse()
}

fn main() {
    let mut input = match try_read_from_stdin::<String>() {
        Ok(x) => x,
        _ => std::process::exit(1),
    };

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
