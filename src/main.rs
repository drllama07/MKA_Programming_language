use std::env;
use std::fs;


use crate::tokens::Token;
use crate::parser::parse;
use crate::parser::Expr;
use crate::scanner::lexer;
use crate::interpreter::Interpreter;
use crate::environments::Environment;


pub mod interpreter;
pub mod environments;
pub mod parser;
pub mod scanner;
pub mod tokens;

fn read_file(file_path: & String) {
    let mut file_content: String = fs::read_to_string(file_path).unwrap();
    println!("File: {}", file_path);
    let tokens: Vec<Token> = run(&mut file_content);
    for tok in tokens.iter() {
        println!("Token -> {} Value -> {}", tok.kind, tok.value);
    }
    let parsed = parse(tokens);
    for pars in parsed.iter(){
        println!("{:?}", pars);
    }
    interpreter_expr(parsed);
}
fn interpreter_expr(mut parsed: Vec<Expr>) {
      let mut global_env = Environment::new( "global");
      parsed.reverse();
      while parsed.len() > 0 {
        let exp = parsed.pop().unwrap();
        match exp {
            Expr::Useless => continue,
            _ => {
                Interpreter::new(exp, "_".to_string()).run(&mut global_env);
            },
        }
      }
}

fn run(code: &mut String) -> Vec<Token>{
    lexer(code)
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2{
       println!("Only one file path can be entered!!!{}", &args[0]);
    } else if args.len() == 2{
        read_file(&args[1]);
    } else {
        println!("Please provide a file path");
    }
}
