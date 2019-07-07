extern crate calculator;

#[macro_use(crate_version, crate_authors)]
extern crate clap;
use clap::{App, Arg};

enum ExitCode {
    Ok,
    InputError,
    ParseError,
}

fn try_read_from_stdin<T: std::str::FromStr>() -> Result<T, T::Err> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.parse()
}

fn try_read_from_file(path: String) -> Option<String> {
    match std::fs::read_to_string(path) {
        Ok(f) => Some(f),
        Err(_) => None,
    }
}

fn main() {
    let arguments = App::new("Calculator")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Does Arithmetic Operations")
        .arg(
            Arg::with_name("f")
                .short("f")
                .long("file")
                .help("File path to compile")
                .takes_value(true),
        )
        .get_matches();

    let mut input: String = if let Some(f) = arguments.value_of("f") {
        match try_read_from_file(f.to_string()) {
            Some(s) => s,
            _ => return exit(ExitCode::InputError),
        }
    } else {
        match try_read_from_stdin::<String>() {
            Ok(x) => x,
            _ => return exit(ExitCode::InputError),
        }
    };

    input += "\0";

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
        print!("{}", code);

        exit(ExitCode::Ok)
    }
}

fn exit(code: ExitCode) {
    match code {
        ExitCode::Ok => std::process::exit(0),
        ExitCode::InputError => std::process::exit(-1),
        ExitCode::ParseError => std::process::exit(-2),
    }
}
