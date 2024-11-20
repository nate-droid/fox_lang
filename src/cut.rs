use std::f32::consts::E;
use crate::parser::{Node, Parser};
use crate::lexer::TokenKind;

#[derive(Debug)]
pub enum ReduceError {
    EmptyNode,
}

impl std::fmt::Display for ReduceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ReduceError {}

// This function should return a box of nodes or an error
pub fn reduce(node: Node) -> Result<Vec<Node>, ReduceError>{
    match node.clone() {
        Node::UnaryExpression {operator, .. } => {
            println!("Found a unary expression");
        }
        Node::BinaryExpression { left, operator, right } => {
            match node.operator() {
                TokenKind::Implies => {
                    // time to do some reduction!
                    println!("Found an implication");

                    // |- A -> B becomes A |- B
                    // this would return a slice of (A, B)
                    let left = node.left();
                    let right = node.right();

                    // Check if left is a identity or an expression that needs to be recursively parsed

                    let x = *left.unwrap().clone();

                    // check if the first element of the left expression is a identity or an expression that needs to be recursively parsed

                    match x {
                        Node::Identifier {value} => {
                            println!("Found an identifier: {}", value);
                        }
                        _ => {}
                    }


                    // Check if right is a identity or an expression that needs to be recursively parsed

                    println!("Left: {:?}", left);
                    println!("Right: {:?}", right);
                }
                _ => {}
            }
        }
        Node::Identifier {value} => {}
        _ => {
            return Err(ReduceError::EmptyNode);
        }
    }

    Err(ReduceError::EmptyNode)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub_left() {
        let input = "((A -> C) -> B)";
        let mut parser = Parser::new(input.to_string());
        let res = parser.parse();
        println!("{:?}", res);
        // check if the result is Ok

        let x = res.unwrap();
        println!("{:?}", x);

        if let Node::BinaryExpression {left, operator, right} = x {
            assert_eq!(operator, TokenKind::Implies);

            if let Node::BinaryExpression {left: left_left, operator: left_operator, right: left_right} = *left {
                assert_eq!(left_operator, TokenKind::Implies);

                if let Node::Identifier {value} = *left_left {
                    assert_eq!(value, "A");
                } else {
                    panic!("Expected an identifier");
                }

                if let Node::Identifier {value} = *left_right {
                    assert_eq!(value, "C");
                } else {
                    panic!("Expected an identifier");
                }
            } else {
                panic!("Expected a binary expression");
            }

            if let Node::Identifier {value} = *right {
                assert_eq!(value, "B");
            } else {
                panic!("Expected an identifier");
            }
        }
    }

    #[test]
    fn test_sub_right() {
        let input = "(A -> (B -> C))";
        let mut parser = Parser::new(input.to_string());
        let res = parser.parse();
        println!("{:?}", res);

        let x = res.unwrap();

        if let Node::BinaryExpression {left, operator, right} = x {
            assert_eq!(operator, TokenKind::Implies);

            if let Node::Identifier {value} = *left {
                assert_eq!(value, "A");
            } else {
                panic!("Expected an identifier");
            }

            if let Node::BinaryExpression {left: right_left, operator: right_operator, right: right_right} = *right {
                assert_eq!(right_operator, TokenKind::Implies);

                if let Node::Identifier {value} = *right_left {
                    assert_eq!(value, "B");
                } else {
                    panic!("Expected an identifier");
                }

                if let Node::Identifier {value} = *right_right {
                    assert_eq!(value, "C");
                } else {
                    panic!("Expected an identifier");
                }
            } else {
                panic!("Expected a binary expression");
            }
        }
    }
}