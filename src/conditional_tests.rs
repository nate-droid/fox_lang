use crate::lang_parser::LangParser;

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
    let input = "\
    let x = 5; \
    if (true) { \
        print(\"hello\"); \
        x = x + 2;\
    } \
    print(x);";

    let mut parser = LangParser::new(input);
    let mut ast = parser.parse().expect("unexpected failure");

    match ast.eval() {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e),
    }

    let res = ast.declarations.get("x").expect("unexpected failure");
    assert_eq!(res.val(), ast::value::Value::Int(7));
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
    let input = "let x = 16 % 5; print(x);";
    let mut ast = LangParser::new(input);
    let mut ast = ast.parse().expect("unexpected failure");
    println!("{:?}", ast);
    match ast.eval() {
        Ok(_) => {
            println!("{:?}", ast.declarations);
        }
        Err(e) => panic!("{:?}", e),
    }
    assert_eq!(ast.declarations.get("x").unwrap().val(), ast::value::Value::Int(1));
}

#[test]
fn compare_expressions() {
    let input = "let x = 15 % 5; if (x == 0) { print(\"x is zero\"); } else { print(\"x is not zero\"); }";
    let mut ast = LangParser::new(input);
    let mut ast = ast.parse().expect("unexpected failure");

    match ast.eval() {
        Ok(_) => {
            println!("{:?}", ast.declarations);
        }
        Err(e) => panic!("{:?}", e),
    }
    println!("{:?}", ast.declarations);
}

#[test]
fn conditions_with_conjunctions() {
    let input = "let x = 15 % 5; let y = 15 % 3; if (x == 0 && y == 0) { print(\"x is zero\"); } else { print(\"x is not zero\"); }";
    let mut ast = LangParser::new(input);
    let mut ast = ast.parse().expect("unexpected failure");

    match ast.eval() {
        Ok(_) => {
            println!("{:?}", ast.declarations);
        }
        Err(e) => panic!("{:?}", e),
    }
    println!("{:?}", ast.declarations);
}

#[test]
fn sum_range() {
      let input = "for i in 0..16 {
        let sum = 0;
        let x = i % 3;
        let y = i % 5;
        
        if (x == 0 && y == 0) {
            print(\"hi\");
        } else {
            print(\"not equal\");
            print(x);
            print(y);
        }
        
        if (i == 15) {
            print(x);
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
fn sum_range_with_break() {
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
fn less_than() {
    let input = "let x = 2; if (x < 3) { print(\"x is less than 3\"); } else { print(\"x is not less than 3\"); }";
    let mut ast = LangParser::new(input);
    let mut ast = ast.parse().expect("unexpected failure");

    match ast.eval() {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e),
    }

    println!("{:?}", ast.declarations);
}

#[test]
fn greater_than() {
    let input = "let x = 5; if (x > 3) { print(\"x is greater than 3\"); } else { print(\"x is not greater than 3\"); }";
    let mut ast = LangParser::new(input);
    let mut ast = ast.parse().expect("unexpected failure");

    match ast.eval() {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e),
    }

    println!("{:?}", ast.declarations);
}

#[test]
fn test_break() {
    let input = "
    let a = 1;
    let b = 2;
    for i in 0..2 {
        if (a < 9) {
            print(\"breaking\");
            break;
        }
        b = 3;
    }";
    
    let mut ast = LangParser::new(input);
    let mut ast = ast.parse().expect("unexpected failure");
    
    match ast.eval() {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e),
    }
    
    let res = ast.declarations.get("b").expect("unexpected failure");
    assert_eq!(res.val(), ast::value::Value::Int(2));
}