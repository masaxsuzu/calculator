fn main() {
    loop {
        eprint!("> ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();

        let lexer = lexer::Lexer::new(&input);
        let mut parser = parser::Parser::new(lexer);
        let mut evaluator = evaluator::Evaluator::new();

        let program = parser.parse();

        match evaluator.eval(program) {
            Some(v) => println!("{}", v),
            _ => continue,
        }
    }
}

pub mod ast;
pub mod evaluator;
pub mod lexer;
pub mod object;
pub mod parser;
pub mod token;
