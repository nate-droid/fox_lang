// this will serve as the REPL entry point for the theorem prover
fn repl() {
    println!("Welcome to the FoxLang REPL");
    println!("Type 'help' for a list of commands");

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
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
                println!("Unknown command: {}", input);
                println!("Type 'help' for a list of commands");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    // fn test_repl() {
    //     repl();
    // }
}