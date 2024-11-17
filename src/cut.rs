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
    fn test_reduce() {
        let input = "((A -> C) -> B)";
        let mut parser = Parser::new(input.to_string());
        let res = parser.parse();
        println!("{:?}", res);
        // check if the result is Ok

        let x = res.unwrap_or(Node::EmptyNode);
        println!("{:?}", x);
        reduce(x);
    }
}