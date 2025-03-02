use crate::lang_parser::LangParser;

#[test]
fn test_arrays() {
    let input = "let x = [1, 2, 3, 4, 5]; print(x);";
    let mut ast = LangParser::new(input);
    let mut ast = ast.parse().expect("unexpected failure");

    match ast.eval() {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e),
    }

    println!("{:?}", ast.declarations);
}