use crate::lang_parser::LangParser;
use crate::parser::Node;

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

#[test]
fn array_index() {
    let input = "let x = [1, 2, 3, 4, 5];
    let y = x[2];
    print(y);";
    let mut ast = LangParser::new(input);
    let mut ast = ast.parse().expect("unexpected failure");

    match ast.eval() {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e),
    }

    println!("{:?}", ast.declarations);
    assert_eq!(ast.declarations.get("y").unwrap().to_string(), "3");
}

#[test]
fn update_array() {
    let input = "let x = [1, 2, 3, 4, 5];
    x[2] = 10;
    print(x);";
    let mut ast = LangParser::new(input);
    let mut ast = ast.parse().expect("unexpected failure");

    match ast.eval() {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e),
    }

    let x = ast.declarations.get("x").unwrap();
    
    match x {
        Node::Array{elements} => {
            assert_eq!(elements[2].to_string(), "10");
        },
        _ => panic!("x is not an array"),
    }
}

#[test]
fn access_array_with_var() {
    let input = "let x = [1, 2, 3, 4, 5];
    let y = 2;
    x[y] = 10;";
    let mut ast = LangParser::new(input);
    let mut ast = ast.parse().expect("unexpected failure");

    match ast.eval() {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e),
    }

    let x = ast.declarations.get("x").unwrap();

    match x {
        Node::Array{elements} => {
            assert_eq!(elements[2].to_string(), "10");
        },
        _ => panic!("x is not an array"),
    }
}