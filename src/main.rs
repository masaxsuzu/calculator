extern crate calculator;

enum ExitCode {
    Ok,
    InputError,
    ParseError,
    RuntimeError,
}

fn try_read_from_stdin<T: std::str::FromStr>() -> Result<T, T::Err> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.parse()
}

fn main() {
    let mut input = match try_read_from_stdin::<String>() {
        Ok(x) => x,
        _ => return exit(ExitCode::InputError),
    };

    input.push('\0');

    let l = calculator::lexer::Lexer::new(&input);
    let mut p = calculator::parser::Parser::new(l);

    let program = match p.parse() {
        Ok(x) => x,
        Err(e) => {
            println!("{:?}", e);
            return exit(ExitCode::ParseError);
        }
    };

    for s in program {
        let code = calculator::compiler::Compiler::new().compile(s);
        println!("{}", code);

        exit(ExitCode::Ok)
    }
}

fn exit(code: ExitCode) {
    match code {
        ExitCode::Ok => std::process::exit(0),
        ExitCode::InputError => std::process::exit(-1),
        ExitCode::ParseError => std::process::exit(-2),
        ExitCode::RuntimeError => std::process::exit(-3),
    }
}
