use crate::lang_parser::LangParser;
use crate::parser::Value;

#[test]
fn if_statement() {
    let input = "if true { print(\"hello, world\"); } else { print(\"goodbye, world\"); }";
    let mut parser = LangParser::new(input);
    let mut ast = parser.parse().expect("unexpected failure");
    println!("{:?}", ast);
    ast.eval().unwrap();
}

#[test]
fn longer_if_statement() {
    let input = "if true { print(\"hello, world\"); print(\"more hello!\"); } else { print(\"goodbye, world\"); }";
    let mut parser = LangParser::new(input);
    let mut ast = parser.parse().expect("unexpected failure");
    println!("{:?}", ast);
    ast.eval().unwrap();
}

#[test]
fn variables_in_conditionals() {
    let input = "let x : Nat = 5; if true { print(\"hello\"); x = x + 2;} print(x);";

    let mut parser = LangParser::new(input);
    let mut ast = parser.parse().expect("unexpected failure");
    
    ast.eval().unwrap();
    let res = ast.declarations.get("x").expect("unexpected failure");
    assert_eq!(res.val(), Value::Int(7));
}