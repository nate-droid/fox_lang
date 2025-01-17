use crate::lang_parser::LangParser;

#[test]
fn if_statement() {
    let input = "if true { print(\"hello, world\"); } else { print(\"goodbye, world\"); }";
    let mut parser = LangParser::new(input);
    let mut ast = parser.parse().expect("unexpected failure");
    println!("{:?}", ast);
    ast.eval().unwrap();
    // TODO: When parsing a "let" statement, check if the type is "Expression", and call the mm parser
}