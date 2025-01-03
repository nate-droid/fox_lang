mod lexer;
pub mod parser;

mod lang_lexer;
mod lang_parser;
pub mod lang_ast;

pub mod cut;

fn main() {
    println!("Welcome to the FoxLang REPL");
    println!("Type 'help' for a list of commands");
    let mut ast = lang_ast::Ast::new();
    
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        
        match command {
            "solve" => {
                println!("Solving...");
            }
            "help" => {
                println!("Available commands:");
                println!("help - display this message");
                println!("exit - exit the hypervisor");
            }
            "exit" => {
                println!("Exiting the hypervisor");
                break;
            }
            _ => {
                // call the lang parser
                ast.parse(input.trim()).expect("unexpected failure");
                
                // ast.eval().expect("unexpected failure");
                println!("{:?}", ast.nodes);
                println!("{:?}", ast.nodes.len());
                // TODO: refactor this after working on multi parsing
                
                println!("Unknown command: {}", input);
                println!("Type 'help' for a list of commands");
            }
        }
    }
}
