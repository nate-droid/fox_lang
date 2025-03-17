use crate::lang_parser::LangParser;


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_function() {
        let input = "fn add(x, y) {
            print(\"Adding\");
        }";
        let mut ast = LangParser::new(input);
        let mut ast = ast.parse().expect("unexpected failure");
        
        println!("{:?}", ast);
        // evaluate the ast
        match ast.eval() {
            Ok(_) => (),
            Err(e) => panic!("{:?}", e),
        }
    }
    
    #[test]
    fn call_function() {
        let input = "fn add(x, y) {
            print(\"Adding\");
            let z = x + y;
            print(z);
        }
        add(5, 10);";
        let mut ast = LangParser::new(input);
        let mut ast = ast.parse().expect("unexpected failure");
        
        // evaluate the ast
        match ast.eval() {
            Ok(_) => (),
            Err(e) => panic!("{:?}", e),
        }
    }
    
    #[test]
    fn prep_for_binary_conversion() {
        let input = "
        let x = 5;
        for i in 0..10 {
            let y = x % 2;
            x = x / 2;
            print(y);
            if (x == 0) {
                break;
            }
        }";
        let mut ast = LangParser::new(input);
        let mut ast = ast.parse().expect("unexpected failure");

        // evaluate the ast
        match ast.eval() {
            Ok(_) => (),
            Err(e) => panic!("{:?}", e),
        }
    }
    
    #[test]
    fn binary_type() {
        let input = "\
        let b = bin(5);\
        print(b);";
        let mut ast = LangParser::new(input);
        let mut ast = ast.parse().expect("unexpected failure");
        match ast.eval() {
            Ok(_) => (),
            Err(e) => panic!("{:?}", e),
        }
    }
}
