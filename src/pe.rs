// these are the tests that attempt to solve problems posed in "Project Euler"

use crate::lang_parser::LangParser;

#[test]
fn problem1() {
    let input = "let sum = 0;
    for i in 0..1000 {
        let x = i % 3;
        let y = i % 5;

        if (x == 0 || y == 0) {
            sum = sum + i;
        }
    }";

    let mut ast = LangParser::new(input);
    let mut ast = ast.parse().expect("unexpected failure");

    match ast.eval() {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e),
    }

    println!("{:?}", ast.declarations);
}

#[test]
fn problem2() {
    let input = "let sum = 0;
    let a = 1;
    let b = 2;
    let c = 0;
    for i in 0..4000000 {
        c = a + b;
        a = b;
        b = c;
        if (c % 2 == 0) {
            sum = sum + c;
        }
    }";
    let input = "let sum = 0;
    let a = 1;
    let b = 2;
    let c = 0;
    for i in 0..100 {
        c = a + b;
        if (c < 4000000) {
            break;
        }
        a = b;
        let x = c % 2;
        if (x == 0) {
            sum = sum + c;
        }
    }";
    
    let mut ast = LangParser::new(input);
    let mut ast = ast.parse().expect("unexpected failure");

    match ast.eval() {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e),
    }

    println!("{:?}", ast.declarations);
}