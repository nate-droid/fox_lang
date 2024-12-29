use crate::lexer::{Token, TokenKind};
use crate::parser::{Node, Value};
use crate::parser::Node::EmptyNode;

#[derive(Debug)]
pub struct Ast {
    pub nodes: Vec<Node>,
}


impl Ast {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
        }
    }
    
    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }
}

fn eval(ast: Node) -> Result<Node, String> {
    // traverse and evaluate the AST
    match ast {
        Node::BinaryExpression { left, operator, right } => {
            let res = eval_binary_expression(*left, operator, *right)?;
            return Ok(res);
        }
        Node::UnaryExpression { operator, right } => {
            
        }
        Node::Identifier { value, .. } => {
            
        }
        Node::Identity { name, value, kind} => {
            println!("let {} : {} = {:?}", name, kind, value);
            let new_val = eval(*value);
            println!("{:?}", new_val);
        }
        Node::Call { name, arguments, returns } => {
            eval_call(name, arguments)?;
        }
        Node::Atomic { value } => {
            
        }
        EmptyNode => {}
    }
    
    Ok(EmptyNode)
}

fn eval_binary_expression(left: Node, operator: TokenKind, right: Node) -> Result<Node, String> {
    match operator {
        TokenKind::Add => {
            if let Node::Atomic { value: left_val } = left {
                if let Node::Atomic { value: right_val } = right {
                    match (left_val, right_val) {
                        (Value::Int(left), Value::Int(right)) => {
                            return Ok(Node::Atomic {
                                value: Value::Int(left + right),
                            });
                        }
                        _ => return Err("Invalid types".to_string()),
                    }
                }
            }
        }
        _ => return Err("Unknown operator".to_string()),
    }
    
    Ok(EmptyNode)
}

fn eval_call(name: String, arguments: Vec<Token>) -> Result<(), String> {
    
    // TODO: Have the call names be enums
    
    match name.as_str() {
        "print" => {
            println!("{:?}", arguments[0].value);
        }
        "add" => {
            
        }
        _ => return Err("Unknown function".to_string()),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::{Token, TokenKind};
    use crate::parser::{Node, Value};
    
    #[test]
    fn test_var() {
        let mut ast = Ast::new();
        ast.add_node(Node::Identity {
            name: "x".to_string(),
            value: Box::from(Node::Atomic {
                value: Value::Int(10),
            }),
            kind: "Nat".to_string(),
        });
        let res = eval(ast.nodes[0].clone()).expect("unexpected failure");
    }
    
    #[test]
    fn test_eval() {
        let mut ast = Ast::new();
        ast.add_node(Node::Call {
            name: "print".to_string(),
            arguments: vec![Token {
                value: "Hello, World!".to_string(),
                kind: TokenKind::String,
            }],
            returns: vec![],
        });
        let res = eval(ast.nodes[0].clone()).expect("unexpected failure");
    }
    
    #[test]
    fn eval_addition() {
        let input = "let x : Nat = 1 + 2;";
        let mut parser = crate::lang_parser::LangParser::new(input.to_string());
        let ast = parser.parse().expect("unexpected failure");
        
        let res = eval(ast.nodes[0].clone()).expect("unexpected failure");
        println!("{:?}", res);
    }
}