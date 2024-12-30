use std::collections::HashMap;
use crate::lexer::{Token, TokenKind};
use crate::parser::{Node, Value};
use crate::parser::Node::EmptyNode;

#[derive(Debug)]
pub struct Ast {
    pub nodes: Vec<Node>,
    declarations: HashMap<String, Node>,
}


impl Ast {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            declarations: HashMap::new(),
        }
    }
    
    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }
    
    pub fn eval(&mut self) -> Result<(), String> {
        for node in self.nodes.clone() {

            match node {
                Node::BinaryExpression { left, operator, right } => {
                    let res = self.eval_binary_expression(*left, operator, *right)?;
                    return Ok(())
                }
                Node::UnaryExpression { operator, right } => {

                }
                Node::Identifier { value, .. } => {

                }
                Node::Identity { name, value, kind} => {
                    let res = self.eval_node(*value)?;
                    
                    self.declarations.insert(name, res);
                }
                Node::Call { name, arguments, returns } => {
                    eval_call(name, arguments)?;
                }
                Node::Atomic { value } => {

                }
                EmptyNode => {}
            }
        }
        Ok(())
    }

    fn eval_node(&mut self, ast: Node) -> Result<Node, String> {
        // traverse and evaluate the AST
        match ast {
            Node::BinaryExpression { left, operator, right } => {
                let res = self.eval_binary_expression(*left, operator, *right)?;
                return Ok(res);
            }
            Node::UnaryExpression { operator, right } => {
                todo!("Unary expressions");
            }
            Node::Identifier { value, .. } => {
                todo!("Identifiers");
            }
            Node::Identity { name, value, kind} => {
                return self.eval_node(*value);
            }
            Node::Call { name, arguments, returns } => {
                eval_call(name, arguments)?;
            }
            Node::Atomic { value } => {
                return Ok(Node::Atomic { value });
            }
            EmptyNode => {
                todo!("Empty node");
            }
        }

        Ok(EmptyNode)
    }

    fn replace_var(&mut self, mut node: Node) -> Result<Node, String> {
        if let Node::Identity { value: left_val, name, .. } = node.clone() {
            let res = self.declarations.get(&name).expect("unexpected failure").clone();
            node = res;
        }
        Ok(node)
    }
    
    fn eval_binary_expression(&mut self, mut left: Node, operator: TokenKind, mut right: Node) -> Result<Node, String> {
        match operator {
            TokenKind::Add => {
                if let Node::Identity { value: left_val, name, .. } = left.clone() {
                    left = self.replace_var(left)?;
                }
                if let Node::Identity { value: right_val, name, .. } = right.clone() {
                    right = self.replace_var(right)?;
                }

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
        ast.eval().expect("unexpected failure");
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
        ast.eval().expect("unexpected failure");
    }
    
    #[test]
    fn eval_addition() {
        let input = "let x : Nat = 1 + 2;";
        let mut parser = crate::lang_parser::LangParser::new(input.to_string());
        let mut ast = parser.parse().expect("unexpected failure");
        
        ast.eval().expect("unexpected failure");
        
        let res = ast.declarations.get("x").expect("unexpected failure").clone();
        
        assert_eq!(res.val(), Value::Int(3));
    }
    
    #[test]
    fn eval_variable_addition() {
        let input = "let x : Nat = 1; let y : Nat = 2; let z : Nat = x + y;";
        let mut parser = crate::lang_parser::LangParser::new(input.to_string());
        let mut ast = parser.parse().expect("unexpected failure");
        
        ast.eval().expect("unexpected failure");

        let res = ast.declarations.get("z").expect("unexpected failure").clone();
        assert_eq!(res.val(), Value::Int(3));
    }
}