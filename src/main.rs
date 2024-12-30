mod lexer;
mod parser;

mod lang_lexer;
mod lang_parser;
mod lang_ast;

mod metamath_lexer;

mod cut;

mod combinator;

fn main() {
    println!("Welcome to the FoxLang REPL");
    println!("Type 'help' for a list of commands");

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
                let mut parser = lang_parser::LangParser::new(input.to_string());
                let ast = parser.parse().expect("unexpected failure");
                
                println!("Unknown command: {}", input);
                println!("Type 'help' for a list of commands");
            }
        }
    }
}
