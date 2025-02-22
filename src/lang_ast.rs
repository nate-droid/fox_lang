use crate::cut::Axiom;
use crate::lexer::{TokenKind};
use crate::parser::Node::{Atomic, EmptyNode};
use crate::parser::{compare_value, Node, Value};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Ast {
    pub nodes: Vec<Node>,
    pub declarations: HashMap<String, Node>,
}

impl Default for Ast {
    fn default() -> Self {
        Self::new()
    }
}

impl Ast {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            declarations: HashMap::new(),
        }
    }

    pub fn upsert_declaration(&mut self, node: Node) -> Result<(), String> {
        match node {
            Node::Identity { name, value, ..} => {
                self.declarations.insert(name, *value);
            }
            EmptyNode => {}
            Atomic { value } => {
            }
            _ => {
                println!("{:?}", node);
                return Err("Invalid node type".to_string());
            }
        }
        Ok(())
    }

    pub fn remove_declaration(&mut self, name: &str) -> Result<(), String> {
        self.declarations.remove(name);
        Ok(())
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn eval(&mut self) -> Result<(), String> {
        for node in self.nodes.clone() {
            self.eval_node(node)?;
        }
        Ok(())
    }

    fn eval_node(&mut self, ast: Node) -> Result<Node, String> {
        // traverse and evaluate the AST
        match ast {
            Node::BinaryExpression {
                left,
                operator,
                right,
            } => {
                let res = self.eval_binary_expression(*left.clone(), operator, *right)?;

                return Ok(res);
            }
            Node::UnaryExpression {
                operator: _operator,
                right: _right,
            } => {
                todo!("Unary expressions");
            }
            Node::Identifier { value: _value, .. } => {
                todo!("Identifiers");
            }
            Node::Identity {
                name: _name,
                value,
                kind: _kind,
            } => {
                let res = self.eval_node(*value)?;
                self.upsert_declaration(Node::Identity {
                    name: _name,
                    value: Box::from(res.clone()),
                    kind: _kind,
                })?;
                return Ok(res);
            }
            Node::Call {
                name,
                arguments,
                returns: _returns,
            } => {
                self.eval_call(name, arguments)?;
            }
            Atomic { value } => {
                return Ok(Atomic { value });
            }
            Node::MMExpression { expression } => {
                let mut axiom = Axiom::new("ax-1".to_string(), expression);
                axiom.solve().expect("unexpected failure");
                println!("{:?}", axiom.steps);
            }
            Node::Type { name: _name } => {
                return Ok(EmptyNode);
            }
            Node::Conditional {
                condition,
                consequence,
                alternative,
            } => {
                self.eval_conditional(condition, consequence, alternative)?;
            }
            Node::ForLoop {
                variable,
                range,
                body,
            } => {
                self.parse_for(variable, range, body)?;
            }
            EmptyNode => {
                return Ok(EmptyNode);
            }
            _ => {
                todo!("Unknown node");
            }
        }

        Ok(EmptyNode)
    }

    fn replace_var(&mut self, mut node: Node) -> Result<Node, String> {
        if let Node::Identity {
            value: _left_val,
            name,
            ..
        } = node.clone()
        {
            let res = self
                .declarations
                .get(&name)
                .expect("unexpected failure")
                .clone();
            node = res;
        }
        if let Node::Object { name, kind } = node.clone() {
            let res = self
                .declarations
                .get(&name)
                .expect("unexpected failure").clone();
            node = res;
            
        }
        Ok(node)
    }

    fn eval_binary_expression(
        &mut self,
        mut left: Node,
        operator: TokenKind,
        mut right: Node,
    ) -> Result<Node, String> {
        match operator {
            TokenKind::Add => {
                if let Node::Identity {
                    value: _left_val,
                    name: _name,
                    ..
                } = left.clone()
                {
                    left = self.replace_var(left)?;
                }
                if let Node::Identity {
                    value: _right_val,
                    name: _name,
                    ..
                } = right.clone()
                {
                    right = self.replace_var(right)?;
                }

                if let Atomic { value: left_val } = left {
                    if let Atomic { value: right_val } = right {
                        return match (left_val, right_val) {
                            (Value::Int(left), Value::Int(right)) => Ok(Atomic {
                                value: Value::Int(left + right),
                            }),
                            _ => Err("Invalid types".to_string()),
                        };
                    }
                }
            }
            TokenKind::Modulo => {
                if let Node::Identity {
                    value: _left_val,
                    name: _name,
                    ..
                } = left.clone()
                {
                    left = self.replace_var(left)?;
                }
                if let Node::Identity {
                    value: _right_val,
                    name: _name,
                    ..
                } = right.clone()
                {
                    right = self.replace_var(right)?;
                }

                if let Atomic { value: left_val } = left {
                    if let Atomic { value: right_val } = right {
                        return match (left_val, right_val) {
                            (Value::Int(left), Value::Int(right)) => Ok(Atomic {
                                value: Value::Int(left % right),
                            }),
                            _ => Err("Invalid types".to_string()),
                        };
                    }
                }
            }
            _ => return Err("Unknown operator".to_string()),
        }

        Ok(EmptyNode)
    }

    pub fn parse(&mut self, input: &str) -> Result<(), String> {
        let mut parser = crate::lang_parser::LangParser::new(input);
        let ast = parser.parse().expect("unexpected failure");

        for node in ast.nodes {
            self.add_node(node);
        }

        Ok(())
    }

    fn eval_call(&mut self, name: String, arguments: Vec<Node>) -> Result<(), String> {
        // TODO: Have the call names be enums

        match name.as_str() {
            "print" => {
                // check if the first argument is an Object
                if let Node::Object { name, kind: _kind } = arguments[0].clone() {
                    let found = self.declarations.get(name.as_str());

                    if let Some(node) = found {
                        let res = self.replace_var(arguments[0].clone())?;
                        println!("{:?}", res.clone().to_string());
                        return Ok(())
                    }
                }
                
                let temp = arguments[0].left().expect("unexpected failure");
                let temp2 = *temp;
                println!("{:?}", temp2.to_string());
                
            }
            "reduce" => {
                println!("{:?}", arguments[0].left());
            }
            _ => return Err("Unknown function".to_string()),
        }
        Ok(())
    }
    
    fn eval_conditional(&mut self, condition: Box<Node>, consequence: Vec<Node>, alternative: Vec<Node>) -> Result<(), String> {
        // assert that node is of type Conditional
        match *condition.clone() {
            Node::BinaryExpression {
                left,
                operator,
                right,
            } => {

                match operator {
                    TokenKind::IsEqual => {
                        // TODO: This is using left.left and right.left which is an oopsie
                        let pre_left = left.left().expect("unexpected failure");
                        let replaced_left = self.replace_var(*pre_left.clone()).expect("unexpected failure");

                        let pre_right = right.left().expect("unexpected failure");
                        let replaced_right = self.replace_var(*pre_right.clone()).expect("unexpected failure");

                        // println!("condition: {:?}", condition);

                        // println!("left: {:?}", pre_left);
                        // println!("right: {:?}", pre_right);
                        // println!("replaced right: {:?}", replaced_right);
                        let best = compare_value(&replaced_left.val(), &replaced_right.val());
                        if best {
                            for node in consequence.clone() {
                                let res = self.eval_node(node)?;
                                self.upsert_declaration(res)?
                            }
                        } else {
                            for node in alternative.clone() {
                                self.eval_node(node)?;
                            }
                        }
                    }
                    TokenKind::Or => {
                        // TODO: Sub expressions will need to be evaluated recursively
                        let sub_left = left.left().expect("unexpected failure");
                        let sub_right = left.right().expect("unexpected failure");
                        
                        let replaced_sub_left = self.replace_var(*sub_left.clone()).expect("unexpected failure");
                        let replaced_sub_right = self.replace_var(*sub_right.clone()).expect("unexpected failure");
                        
                        // check the truthiness of the left and right
                        let left_truth = compare_value(&replaced_sub_left.val(), &replaced_sub_right.val());
                        
                        let sub_right_left = right.left().expect("unexpected failure");
                        let sub_right_right = right.right().expect("unexpected failure");
                        
                        let replaced_sub_right_left = self.replace_var(*sub_right_left.clone()).expect("unexpected failure");
                        let replaced_sub_right_right = self.replace_var(*sub_right_right.clone()).expect("unexpected failure");
                        
                        let right_truth = compare_value(&replaced_sub_right_left.val(), &replaced_sub_right_right.val());
                        
                        if left_truth || right_truth {
                            for node in consequence.clone() {
                                    let res = self.eval_node(node)?;
                                    self.upsert_declaration(res)?
                                }
                            } else {
                                for node in alternative.clone() {
                                    self.eval_node(node)?;
                                }
                            }
                    }
                    _ => {
                        println!("operator: {:?}", operator);
                    }
                }
            }
            Atomic { value } => {
                if value == Value::Bool(true) {
                    for node in consequence.clone() {
                        let res = self.eval_node(node)?;
                        self.upsert_declaration(res)?
                    }
                }
            }
            _ => {
                println!("condition: {:?}", condition);
                // TODO: Check if boolean
            }
        }
        Ok(())
    }
    
    fn parse_for(&mut self, variable: String, range: (i32, i32), body: Vec<Node>) -> Result<(), String> {
        self.upsert_declaration(Node::Identity {
            name: variable.clone(),
            value: Box::new(Atomic {
                value: Value::Int(range.0),
            }),
            kind: "Nat".to_string(),
        })?;

        let start = range.0;
        let end = range.1;
        let mut i = start;

        while i < end {
            for node in body.clone() {
                self.eval_node(node)?;
            }
            i += 1;

            self.upsert_declaration(Node::Identity {
                name: variable.clone(),
                value: Box::new(Atomic {
                    value: Value::Int(i),
                }),
                kind: "Nat".to_string(),
            })?;
        }

        self.remove_declaration(&variable)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lang_parser::LangParser;
    use crate::parser::{Node, Value};

    #[test]
    fn test_var() {
        let mut ast = Ast::new();
        ast.add_node(Node::Identity {
            name: "x".to_string(),
            value: Box::from(Atomic {
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
            arguments: vec![Atomic {
                value: Value::Str("hello, world".to_string()),
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

        assert_eq!(res.val(), Value::Int(3));
    }

    #[test]
    fn eval_variable_addition() {
        let input = "let x = 1; let y = 2; let z = x + y;";
        let mut parser = LangParser::new(input);
        let mut ast = parser.parse().expect("unexpected failure");

        ast.eval().expect("unexpected failure");
        println!("{:?}", ast);
        let res = ast.declarations.get("z").expect("unexpected failure");
        assert_eq!(res.val(), Value::Int(3));
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
        let mut ast = Ast::new();
        ast.parse(input).expect("unexpected failure");
        let input2 = "let y = 2;";
        ast.parse(input2).expect("unexpected failure");
        println!("{:?}", ast.nodes);
    }

    #[test]
    fn custom_types() {
        let input = "type nat;";
        let mut ast = Ast::new();
        ast.parse(input).expect("unexpected failure");
        ast.eval().expect("unexpected failure");
        println!("{:?}", ast);
    }
}
