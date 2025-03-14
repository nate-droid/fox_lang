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
}
