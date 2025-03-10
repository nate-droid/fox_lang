use std::env;

mod lexer;
pub mod parser;

mod lang_lexer;
mod lang_parser;
pub mod lang_ast;

pub mod cut;

fn main() {
    println!("Welcome to the FoxLang REPL");
    println!("Type 'help' for a list of commands");
    println!();
    let mut ast = lang_ast::Ast::new();
    
    loop {
        let args: Vec<String> = env::args().collect();
        println!("args: {:?}", args);
        if args.len() > 1 {
            // need to parse a file
        }
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        
        match command {
            "solve" => {
                println!("Solving...");
            }
            "help" => {
                println!();
                println!("help for the FoxLang REPL");
                println!();
                println!("You can assign variables with the following syntax:");
                println!("let x = 5;");
                println!();
                println!("You can print variables with the following syntax:");
                println!("print(\"Hello world!\");");
                println!();
                println!("Available commands:");
                println!("help - display this message");
                println!("exit - exit the hypervisor");
                println!("scope - display the current scope");
                println!("reset - reset the current scope");
            }
            "exit" => {
                println!("Exiting the Fox REPL");
                break;
            }
            "scope" => {
                println!("{:?}", ast.declarations);
            }
            "reset" => {
                ast = lang_ast::Ast::new();
            }
            _ => {
                // call the lang parser
                ast.parse(input.trim()).expect("unexpected failure");
                
                ast.eval().expect("unexpected failure");
            }
        }
    }
}
