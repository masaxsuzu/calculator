extern crate calculator;

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

    let lexer = calculator::lexer::Lexer::new(&input);
    let mut parser = calculator::parser::Parser::new(lexer);

    match parser.parse() {
        Ok(o) => println!("{:?}", o),
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
