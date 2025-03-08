use crate::cut::Axiom;
use crate::lexer::{TokenKind};
use crate::parser::Node::{Atomic, Break, EmptyNode};
use crate::parser::{compare_value, Node, Value};
use std::collections::HashMap;
use std::fmt;

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

fn print_with_indent(node: Node, indent: usize) {
    match node {
        Node::AssignStmt { left, right, kind } => {
            // println!("{:indent$}{:?} = {:?};", "", left, right, indent=indent);
            // left
            print_with_indent(*left.clone(), indent +2);
            // right
            print_with_indent(*right.clone(), indent +2);
        }
        Node::Ident { name, kind } => {
            print_with_indent(Node::Identifier { value: name }, indent +2);
        }
        _ => {
            println!("{:indent$}{:?}", "", node, indent=indent);
        }
    }
}

impl Ast {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            declarations: HashMap::new(),
        }
    }

    pub fn print(&self) {
        for node in self.nodes.clone() {
            print_with_indent(node, 0);
        }
    }

    pub fn upsert_declaration(&mut self, node: Node) -> Result<(), String> {
        match node {
            // Node::AssignStmt { name, value, ..} => {
            Node::AssignStmt { left, right, .. } => {
                match *left {
                    Node::Identifier { value: _name } => {
                        self.declarations.insert(_name, *right);
                    }
                    Node::Ident { name, kind } => {
                        self.declarations.insert(name, *right);
                    }
                    Node::IndexExpression { left: left2, index } => {
                        // fetch name from left

                        let name = match *left2 {
                            Node::Identifier { value } => value,
                            Node::Ident { name, kind: _kind } => name,
                            Node::IndexExpression {left: left3, ..} => {
                                let name = match *left3 {
                                    Node::Identifier { value } => value,
                                    Node::Ident { name, kind: _kind } => name,
                                    _ => {
                                        println!("{:?}", left3);
                                        return Err("Invalid type".to_string());
                                    }
                                };
                                name
                            }
                            _ => {
                                println!("{:?}", left2);
                                return Err("Invalid type".to_string());
                            }
                        };
                        let array = self.declarations.get(&name).expect("var not found").clone();
                        let mut elements = match array {
                            Node::Array { elements } => elements,
                            _ => {
                                println!("{:?}", array);
                                return Err("Invalid type".to_string());
                            }
                        };
                        let i = match *index {
                            Atomic { value: Value::Int(i) } => i as usize,
                            _ => {
                                println!("{:?}", index);
                                return Err("Invalid type".to_string());
                            }
                        };
                        elements[i] = *right;

                        self.declarations.insert(name, Node::Array { elements });
                    }
                    _ => {
                        println!("{:?}", left);
                        return Err("Invalid type".to_string());
                    }
                }

            }
            EmptyNode => {}
            Atomic { value } => {
            }
            _ => {
                // println!("{:?}", node);
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
            Node::AssignStmt {
                left,
                right,
                kind: _kind,
            } => {
                if let Node::IndexExpression { left, index } = *left.clone() {
                    println!("found a : {:?}", left);
                    println!("found an index : {:?}", index);
                    println!("right: {:?}", right);
                    let replaced = self.replace_var(*index.clone()).expect("unexpected failure");
                    let res = self.eval_node(replaced)?;
                    println!("res: {:?}", res);
                    self.upsert_declaration(Node::AssignStmt {
                        left: Box::from(Node::IndexExpression {
                            left,
                            index: Box::from(res.clone()),
                        }),
                        right: right.clone(),
                        kind: _kind,
                    })?;
                    return Ok(res);
                }
                
                let replaced = self.replace_var(*right.clone()).expect("unexpected failure");

                let res = self.eval_node(replaced)?;

                self.upsert_declaration(Node::AssignStmt {
                    left,
                    right: Box::from(res.clone()),
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
            Break { .. } => {
                // return Ok(Break {});
                return Err("Break".to_string());
            }
            Node::Array {elements: _elements } => {
                let x = Node::Array {
                    elements: _elements,
                };
                return Ok(x);
            }
            Node::IndexExpression { left, index } => {
                let y = *left;

                // ensure that "y" is an array
                return match y.clone() {
                    Node::AssignStmt { left, right, kind } => {
                        // let i = index;
                        // let i = i as usize;

                        let name = match *left {
                            Node::Identifier { value } => value,
                            Node::Ident { name, kind: _kind } => name,
                            _ => {
                                return Err("Invalid type".to_string());
                            }
                        };

                        // fetch the name from the declarations
                        let elements = self.declarations.get(&name).expect("unexpected failure");

                        // check if the value is an array
                        let result = match elements {
                            Node::Array { elements } => {
                                elements
                            }
                            _ => {
                                println!("{:?}", elements);
                                return Err("Invalid type".to_string());
                            }
                        };
                        let i = match *index {
                            Atomic { value: Value::Int(i) } => i as usize,
                            _ => {
                                println!("{:?}", index);
                                return Err("Invalid type".to_string());
                            }
                        };

                        // Using only Atomic values for now
                        let x = Atomic {
                            value: result[i].val(),
                        };
                        Ok(x)
                    }
                    _ => {
                        println!("{:?}", y);
                        Err("Invalid type".to_string())
                    }
                }
            }
            _ => {
                println!("{:?}", ast);
                todo!("Unknown node");
            }
        }

        Ok(EmptyNode)
    }

    fn replace_var(&mut self, mut node: Node) -> Result<Node, String> {
        match node.clone() {
            Node::AssignStmt {
                right: _left_val,
                left,
                ..
            } => {
                let name = match *left {
                    Node::Identifier { value } => value,
                    Node::Ident { name, kind: _kind } => name,
                    _ => {
                        println!("{:?}", left);
                        return Err("Invalid type".to_string());
                    }
                };

                let res = self
                    .declarations
                    .get(&name)
                    .expect("unexpected failure")
                    .clone();
                return Ok(res);
            }
            Node::Object {name, ..} => {
                let res = self
                    .declarations
                    .get(&name)
                    .expect("unexpected failure").clone();
                return Ok(res);
            }
            Atomic { value } => {
                return Ok(Atomic { value });
            }
            _ => {
            }
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
                if let Node::AssignStmt {
                    right: _left_val,
                    left: _name,
                    ..
                } = left.clone()
                {
                    left = self.replace_var(left)?;
                }
                if let Node::AssignStmt {
                    right: _right_val,
                    left: _name,
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
                if let Node::AssignStmt {
                    right: _left_val,
                    left: _name,
                    ..
                } = left.clone()
                {
                    left = self.replace_var(left)?;
                }
                if let Node::AssignStmt {
                    right: _right_val,
                    left: _name,
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
                    TokenKind::LessThan => {
                        
                        let replaced_left = self.replace_var(*left.clone()).expect("unexpected failure");
                        match replaced_left.val() {
                            Value::Int(i) => {
                                match right.val() {
                                    Value::Int(ii) => {
                                        match i.cmp(&ii) {
                                            std::cmp::Ordering::Less => {
                                                for node in consequence.clone() {
                                                    let res = self.eval_node(node)?;
                                                    self.upsert_declaration(res)?
                                                }
                                            }
                                            _ => {
                                                for node in alternative.clone() {
                                                    let res = self.eval_node(node)?;
                                                    self.upsert_declaration(res)?
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        println!("right: {:?}", right);
                                        return Err("Invalid types".to_string());
                                    }
                                }
                            }
                            _ => {
                                println!("left: {:?}", left);
                                return Err("Invalid types".to_string());
                            }
                        }
                        
                    }
                    TokenKind::GreaterThan => {
                        let replaced_left = self.replace_var(*left.clone()).expect("unexpected failure");
                        
                        match replaced_left.val() {
                            Value::Int(i) => {
                                match right.val() {
                                    Value::Int(ii) => {
                                        match i.cmp(&ii) {
                                            std::cmp::Ordering::Greater => {
                                                for node in consequence.clone() {
                                                    let res = self.eval_node(node)?;
                                                    self.upsert_declaration(res)?
                                                }
                                            }
                                            _ => {
                                                for node in alternative.clone() {
                                                    let res = self.eval_node(node)?;
                                                    self.upsert_declaration(res)?
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        println!("right: {:?}", right);
                                        return Err("Invalid types".to_string());
                                    }
                                }
                            }
                            _ => {
                                println!("left: {:?}", left);
                                return Err("Invalid types".to_string());
                            }
                        }
                    }
                    _ => {
                        println!("operator: {:?}", operator);
                        // todo!("Unknown operator");
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

        self.upsert_declaration(Node::AssignStmt {
            left: Box::new(Node::Ident {
                name: variable.clone(),
                kind: "var".to_string(),
            }),
            right: Box::new(Atomic {
                value: Value::Int(range.0),
            }),
            kind: "Nat".to_string(),
        })?;

        let start = range.0;
        let end = range.1;
        let mut i = start;

        while i < end {
            for node in body.clone() {
                let x = self.eval_node(node);
                match x {
                    Ok(_) => (),
                    Err(e) => {
                        if e == "Break" {
                            i = end;
                            break;
                        }
                    },
                }
            }
            i += 1;

            self.upsert_declaration(Node::AssignStmt {
                left: Box::new(Node::Ident {
                    name: variable.clone(),
                    kind: "var".to_string(),
                }),
                right: Box::new(Atomic {
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
        ast.add_node(Node::AssignStmt {
            left: Box::new(Node::Ident {
                name: "x".to_string(),
                kind: "var".to_string(),
            }),
            right: Box::from(Atomic {
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
