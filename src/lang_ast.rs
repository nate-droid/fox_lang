use crate::lexer::Token;
use crate::parser::Node;

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

fn eval(ast: Node) -> Result<(), String> {
    // traverse and evaluate the AST
    match ast {
        Node::BinaryExpression { left, operator, right } => {
            let left = eval(*left);
        }
        Node::UnaryExpression { operator, right } => {
            
        }
        Node::Identifier { value, .. } => {
            
        }
        Node::Identity { name, value, kind} => {
            println!("let {} : {} = {:?}", name, kind, value);
        }
        Node::Call { name, arguments, returns } => {
            eval_call(name, arguments)?;
        }
        Node::Atomic { value } => {
            
        }
        Node::EmptyNode => {}
    }
    
    Ok(())
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
        
        assert_eq!(eval(ast.nodes[0].clone()), Ok(()));
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
        
        assert_eq!(eval(ast.nodes[0].clone()), Ok(()));
    }
}