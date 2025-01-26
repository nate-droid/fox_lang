use crate::lang_parser::LangParser;
use crate::parser::Value;

#[test]
fn if_statement() {
    let input = "if (true) { print(\"hello, world\"); } else { print(\"goodbye, world\"); }";
    let mut parser = LangParser::new(input);
    let mut ast = parser.parse().expect("unexpected failure");
    
    match ast.eval() {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e),
    }
}

#[test]
fn longer_if_statement() {
    let input = "if (true) { print(\"hello, world\"); print(\"more hello!\"); } else { print(\"goodbye, world\"); }";
    let mut parser = LangParser::new(input);
    let mut ast = parser.parse().expect("unexpected failure");
    
    match ast.eval() {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e),
    }
}

#[test]
fn variables_in_conditionals() {
    let input = "let x : Nat = 5; if (true) { print(\"hello\"); x = x + 2;} print(x);";

    let mut parser = LangParser::new(input);
    let mut ast = parser.parse().expect("unexpected failure");
    
    match ast.eval() {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e),
    }
    
    let res = ast.declarations.get("x").expect("unexpected failure");
    assert_eq!(res.val(), Value::Int(7));
}

#[test]
fn simple_for_loop() {
    let input = "for i in 0..5 { print(i); }";
    let mut ast = LangParser::new(input);
    let mut ast = ast.parse().expect("unexpected failure");
    
    match ast.eval() {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e),
    }
}

#[test]
fn simple_modulo() {
    let input = "let x : Nat = 16 % 5; print(x);";
    let mut ast = LangParser::new(input);
    let mut ast = ast.parse().expect("unexpected failure");

    match ast.eval() {
        Ok(_) => {
            println!("{:?}", ast.declarations);
        },
        Err(e) => panic!("{:?}", e),
    }
    assert_eq!(ast.declarations.get("x").unwrap().val(), Value::Int(1));
}

#[test]
fn compare_expressions() {
    let input = "let x : Nat = 15 % 5; if (x == 0) { print(\"x is zero\"); } else { print(\"x is not zero\"); }";
    let mut ast = LangParser::new(input);
    let mut ast = ast.parse().expect("unexpected failure");

    match ast.eval() {
        Ok(_) => {
            println!("{:?}", ast.declarations);
        },
        Err(e) => panic!("{:?}", e),
    }
    println!("{:?}", ast.declarations);
}