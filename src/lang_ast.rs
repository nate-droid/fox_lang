use crate::cut::Axiom;
use crate::lexer::{TokenKind};
use std::collections::HashMap;
use ast::ast::Ast;
use ast::node::Node;

#[cfg(test)]
mod tests {
    use crate::lang_parser;
    use super::*;
    use crate::lang_parser::LangParser;

    #[test]
    fn test_var() {
        let mut ast = Ast::new();
        ast.add_node(Node::AssignStmt {
            left: Box::new(Node::Ident {
                name: "x".to_string(),
                kind: "var".to_string(),
            }),
            right: Box::from(Node::Atomic {
                value: ast::value::Value::Int(10),
            }),
            kind: "Nat".to_string(),
        });
        ast.eval().expect("unexpected failure");
    }

    #[test]
    fn test_eval() {
        let mut ast = Ast::new();
        ast.add_node(ast::node::Node::Call {
            name: "print".to_string(),
            arguments: vec![Node::Atomic {
                value: ast::value::Value::Str("hello, world".to_string()),
            }],
            returns: vec![],
        });
        ast.eval().expect("unexpected failure");
    }

    #[test]
    fn eval_addition() {
        let input = "let x = 1 + 2;";
        let mut parser = LangParser::new(input);
        let mut ast = parser.parse().expect("unexpected failure");

        ast.eval().expect("unexpected failure");

        let res = ast.declarations.get("x").expect("unexpected failure");

        assert_eq!(res.val(), ast::value::Value::Int(3));
    }

    #[test]
    fn eval_subtraction() {
        let input = "let x = 1 - 2;";
        let mut parser = LangParser::new(input);
        let mut ast = parser.parse().expect("unexpected failure");

        ast.eval().expect("unexpected failure");

        let res = ast.declarations.get("x").expect("unexpected failure");

        assert_eq!(res.val(), ast::value::Value::Int(-1));
    }

    #[test]
    fn eval_multiplication() {
        let input = "let x = 2 * 3;";
        let mut parser = LangParser::new(input);
        let mut ast = parser.parse().expect("unexpected failure");

        ast.eval().expect("unexpected failure");

        let res = ast.declarations.get("x").expect("unexpected failure");

        assert_eq!(res.val(), ast::value::Value::Int(6));
    }

    #[test]
    fn eval_division() {
        let input = "let x = 6 / 3;";
        let mut parser = LangParser::new(input);
        let mut ast = parser.parse().expect("unexpected failure");

        ast.eval().expect("unexpected failure");

        let res = ast.declarations.get("x").expect("unexpected failure");

        assert_eq!(res.val(), ast::value::Value::Int(2));
    }

    #[test]
    fn eval_variable_addition() {
        let input = "let x = 1; let y = 2; let z = x + y;";
        let mut parser = LangParser::new(input);
        let mut ast = parser.parse().expect("unexpected failure");

        ast.eval().expect("unexpected failure");
        println!("{:?}", ast);
        let res = ast.declarations.get("z").expect("unexpected failure");
        assert_eq!(res.val(), ast::value::Value::Int(3));
    }

    #[test]
    fn reduce_expression() {
        let input = "let x = (𝜑 → (𝜓 → 𝜑));";
        let mut parser = LangParser::new(input);
        let mut ast = parser.parse().expect("unexpected failure");
        println!("{:?}", ast);
        ast.eval().expect("unexpected failure");
    }

    #[test]
    fn parse_several() {
        let input = "let x = 1;";
        let mut parser = LangParser::new(input);
        parser.parse_input(input).expect("unexpected failure");
        let input2 = "let y = 2;";
        parser.parse_input(input2).expect("unexpected failure");
        println!("{:?}", parser.ast.nodes);
    }

    #[test]
    fn ignore_comment() {
        let input = "let x = 1;
        // this is a comment
        print(x);
        ";
        let mut parser = LangParser::new(input);
        parser.parse_input(input).expect("unexpected failure");
        parser.ast.eval().expect("unexpected failure");
        println!("{:?}", parser.ast);
    }
}
