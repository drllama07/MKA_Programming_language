use std::env;
use std::fs;
use std::io;

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
    /*for tok in tokens.iter() {
        println!("Token -> {} Value -> {}", tok.kind, tok.value);
    }*/
    let parsed = parse(tokens);
    /*for pars in parsed.iter(){
        println!("{:?}", pars);
    }*/
    let mut file_name = file_path.clone();
    file_name.truncate(file_path.clone().rfind(".").unwrap());
    interpreter_expr(parsed, file_name);
}
fn interpreter_expr(mut parsed: Vec<Expr>, file_name: String) {
      let mut global_env = Environment::new( "global");
      global_env.check_import(&file_name);
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
fn helpoptions() {
    println!("                 --<| HELP OPTIONS |>-- ");
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  ╭─────── REACH OUT ──────────────────────────────────────╮  ║");
    println!("║  │  Feel free to reach out                                │  ║");
    println!("║  │  to me on GitHub for                                   │  ║");
    println!("║  │  collaboration,                                        │  ║");
    println!("║  │  questions, or just to                                 │  ║");
    println!("║  │  say hi!                                               │  ║");
    println!("║  │                                                        │  ║");
    println!("║  │  GitHub:                                               │  ║");
    println!("║  │  https://github.com/drllama07/MKA_Programming_language │  ║");
    println!("╚══╯────────────────────────────────────────────────────────╯══╝");
}

fn menu() {
    println!("           --<| MENU OPTIONS |>-- ");
    println!("╔═════════════════════════════════════════╗");
    println!("║   ╭─────────────────────────────────╮   ║");
    println!("║   │  Help Options         ---> help │   ║");
    println!("║   │  Run                  ---> run  │   ║");
    println!("║   │  Exit                 ---> exit │   ║");
    println!("║   │  Other Options ---> Not Implemented ║");
    println!("╚════╯────────────────────────────────╯═══╝");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let trimmed_input = input.trim().to_lowercase();
    match trimmed_input.as_str() {
        "help" => helpoptions(),
        "run" => {
                let mut input = String::new();
                println!(" --> Type the file name");
                io::stdin().read_line(&mut input).expect("Failed to read line");

                let trimmed_input = input.trim().to_lowercase();
                
                if trimmed_input.ends_with(".mka") {
                    read_file(&trimmed_input);
                } else if  trimmed_input == ""{
                    read_file(&String::from("main.mka"));
                } else {
                    println!("You can either enter YOURFILE.mka or just calc to run the default main.mka");
                }
        }
        "exit" => panic!("Exiting the file not an error"),
        &_ =>  println!("You need to choose and type a option")
    }

}



fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2{
       println!("Only 1 argument from the options -> ( calc or menu )!!!{}", &args[0]);
    } else if args.len() == 2{
        

        match args[1].as_str() {
            "calc" => {
                let mut input = String::new();
                println!(" --> Type the file name");
                io::stdin().read_line(&mut input).expect("Failed to read line");

                let trimmed_input = input.trim().to_lowercase();
                if trimmed_input.ends_with(".mka") {
                   
                   read_file(&trimmed_input);
                } else if  trimmed_input == ""{
                    read_file(&String::from("main.mka"));
                } else {
                    println!("You can either enter YOURFILE.mka or just calc to run the default main.mka");
                }
            }
            "menu" => {
                loop { 
                    menu();
                }
            }
            &_ =>  println!("You need to choose and type a option -> ( calc or menu )")
        }
    } else {
        println!("You need to choose and type a option -> ( calc or menu )");
    }
}
