use crate::cut::Axiom;
use crate::lexer::{TokenKind};
use crate::parser::Node::{Atomic, Break, EmptyNode, Ident};
use crate::parser::{compare_value, Node, Value};
use std::collections::HashMap;
use std::fmt::Arguments;
use crate::internal_types::{fetch_array, fetch_integer, fetch_string};

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
            Node::AssignStmt { left, right, .. } => {
                match *left {
                    Node::Identifier { value: _name } => {
                        self.declarations.insert(_name, *right);
                    }
                    Ident { name, kind } => {
                        self.declarations.insert(name, *right);
                    }
                    Node::IndexExpression { left: left2, index } => {
                        // fetch name from left

                        let name = fetch_string(*left2.clone())?;
                        
                        let array = self.declarations.get(&name).expect("var not found").clone();

                        let mut elements = fetch_array(array)?;
                        
                        let i = extract_index(*index.clone())?;
                        elements[i] = *right;

                        self.declarations.insert(name, Node::Array { elements });
                    }
                    _ => {
                        return Err(format!("invalid type {:?} for left branch", left));
                    }
                }

            }
            Node::FunctionDecl { name, arguments, returns, body } => {
                let n = fetch_string(*name.clone())?;
                self.declarations.insert(n, Node::FunctionDecl {
                    name,
                    arguments,
                    returns,
                    body,
                });
            }
            EmptyNode => {}
            Atomic { value } => {
            }
            _ => {
                return Err(format!("Invalid node type: {:?}", node));
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

                    let replaced = self.replace_var(*index.clone())?;
                    let res = self.eval_node(replaced)?;

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

                let replaced = self.replace_var(*right.clone())?;

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
                        let name = fetch_string(*left.clone())?;
                        
                        // fetch the name from the declarations
                        let elements = self.declarations.get(&name).expect("unexpected failure");

                        // check if the value is an array
                        let result = fetch_array(elements.clone())?;
                        let i = extract_index(*index.clone())?;

                        // Using only Atomic values for now
                        Ok(Atomic {
                            value: result[i].val(),
                        })
                    }
                    _ => {
                        println!("{:?}", y);
                        Err("Invalid type".to_string())
                    }
                }
            }
            Node::FunctionDecl {
                name,
                arguments,
                returns, 
                body,
            } => {
                let res = Node::FunctionDecl {
                    name,
                    arguments,
                    returns,
                    body,
                    
                };
                
                self.upsert_declaration(res)?;
                
                return Ok(EmptyNode);
            }
            _ => {
                println!("{:?}", ast);
                todo!("Unknown node");
            }
        }

        Ok(EmptyNode)
    }

    fn replace_var(&mut self, node: Node) -> Result<Node, String> {
        match node.clone() {
            Node::AssignStmt {
                right: _left_val,
                left,
                ..
            } => {
                let name = fetch_string(*left.clone())?;
                
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
            Node::IndexExpression { left, index } => {
                return self.replace_var_assign(*left, index);
            }
            Ident { name, kind } => {
                let res = self
                    .declarations
                    .get(&name)
                    .expect("unexpected failure")
                    .clone();
                return Ok(res);
            }
            _ => {
            }
        }

        Ok(node)
    }

    fn replace_var_assign(&mut self, node: Node, index: Box<Node>) -> Result<Node, String> {
        match node.clone() {
            Node::AssignStmt { left, right, kind } => {
                let name = fetch_string(*left.clone())?;

                // fetch the name from the declarations
                let elements = self.declarations.get(&name).expect("unexpected failure");

                let result = fetch_array(elements.clone())?;
                let i = extract_index(*index.clone())?;

                // Using only Atomic values for now
                Ok(Atomic {
                    value: result[i].val(),
                })
            }
            Node::IndexExpression {left, index} => {
                let name = fetch_string(node)?;
                
                let array = self.declarations.get(&name).expect("var not found").clone();

                let outer_index = fetch_integer(*index.clone())?;
                
                let elements = fetch_array(array)?;

                let child_node = elements[outer_index as usize].clone();
                println!("child node {:?}", child_node);
                let child_array = fetch_array(child_node)?;
                let res = child_array[0].clone();
                
                Ok(res)
            }
            _ => {
                println!("{:?}", node);
                Err("Invalid type".to_string())
            }
        }
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
            TokenKind::Subtract => {
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
                                value: Value::Int(left - right),
                            }),
                            _ => Err("Invalid types".to_string()),
                        };
                    }
                }
            }
            TokenKind::Multiply => {
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
                                value: Value::Int(left * right),
                            }),
                            _ => Err("Invalid types".to_string()),
                        };
                    }
                }
            }
            TokenKind::Divide => {
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
                                value: Value::Int(left / right),
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
                let temp2 = *arguments[0].left()?;
                
                if let  Node::IndexExpression { left, index } = temp2 {
                    let x = self.eval_array(Node::IndexExpression { left, index })?;
                    println!("{:?}", x);
                    return Ok(())
                };

                println!("{:?}", self.replace_var(temp2).expect("unexpected failure").to_string());
            }
            "reduce" => {
                println!("{:?}", arguments[0].left());
            }
            _ => { 
                println!("arguments: {:?}", arguments);
                // function call is not a builtin function
                let func = self.declarations.get(&name).expect("unexpected failure").clone();

                self.eval_function(func, arguments)?;
            }
        }
        
        Ok(())
    }
    
    fn eval_function(&mut self, node: Node, arguments: Vec<Node>) -> Result<(), String> {
        // make sure that the node is a function
        if let Node::FunctionDecl {
                name,
                arguments: args,
                returns,
                body} = node 
        {
            for i in 0..args.len() {
                let name = fetch_string(args[i].clone())?;
                self.upsert_declaration(Node::AssignStmt {
                    left: Box::new(Ident {
                        name: name.clone(),
                        kind: "var".to_string(),
                    }),
                    right: Box::new(arguments[i].clone()),
                    kind: "Nat".to_string(),
                })?;
            }
            
            for expr in body {
                self.eval_node(expr)?;
            }
            
            for arg in args {
                self.remove_declaration(&fetch_string(arg)?)?;
            }
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
                        let pre_left = left.left()?;
                        let replaced_left = self.replace_var(*pre_left.clone())?;

                        let pre_right = right.left()?;
                        let replaced_right = self.replace_var(*pre_right.clone())?;
                        
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
                        let sub_left = left.left()?;
                        let sub_right = left.right()?;
                        
                        let replaced_sub_left = self.replace_var(*sub_left.clone())?;
                        let replaced_sub_right = self.replace_var(sub_right.clone())?;
                        
                        // check the truthiness of the left and right
                        let left_truth = compare_value(&replaced_sub_left.val(), &replaced_sub_right.val());
                        
                        let sub_right_left = right.left()?;
                        let sub_right_right = right.right()?;
                        
                        let replaced_sub_right_left = self.replace_var(*sub_right_left.clone())?;
                        let replaced_sub_right_right = self.replace_var(sub_right_right.clone())?;
                        
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
                        let replaced_left = self.replace_var(*left.clone())?;
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
                        let replaced_left = self.replace_var(*left.clone())?;
                        
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
    
    fn eval_array(&mut self, node: Node) -> Result<Node, String> {
        match node {
            Node::IndexExpression { left, index: sub_index } => {
                if let Node::IndexExpression {left, index} = *left {
                    let res = self.eval_array(*left)?;

                    let name = fetch_string(res.clone())?;
                    let blah = self.declarations.get(&name).expect("missing declaration").clone();
                    let array = fetch_array(blah)?;
                    let index = fetch_integer(*index)?;
                    let sub_index = fetch_integer(*sub_index)?;
                    let outer = array[index as usize].clone();

                    let inner = fetch_array(outer)?;
                    
                    let last = inner[sub_index as usize].clone();
                    
                    return Ok(last);
                }
                Ok(EmptyNode)
            }
            Ident {name, kind} => {
                // fetch the ident by name from declarations and return
                Ok(Ident { name, kind })
            }
            _=> {
                Err(format!("expected index expression but received {:?} while evaluating array", node))
            }
        }
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

fn extract_index(index: Node) -> Result<usize, String> {
    match index {
        Atomic { value: Value::Int(i) } => Ok(i as usize),
        _ => {
            Err(format!("expected atomic but received {:?} while extracting index", index))
        }
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
    fn eval_subtraction() {
        let input = "let x = 1 - 2;";
        let mut parser = LangParser::new(input);
        let mut ast = parser.parse().expect("unexpected failure");

        ast.eval().expect("unexpected failure");

        let res = ast.declarations.get("x").expect("unexpected failure");

        assert_eq!(res.val(), Value::Int(-1));
    }

    #[test]
    fn eval_multiplication() {
        let input = "let x = 2 * 3;";
        let mut parser = LangParser::new(input);
        let mut ast = parser.parse().expect("unexpected failure");

        ast.eval().expect("unexpected failure");

        let res = ast.declarations.get("x").expect("unexpected failure");

        assert_eq!(res.val(), Value::Int(6));
    }

    #[test]
    fn eval_division() {
        let input = "let x = 6 / 3;";
        let mut parser = LangParser::new(input);
        let mut ast = parser.parse().expect("unexpected failure");

        ast.eval().expect("unexpected failure");

        let res = ast.declarations.get("x").expect("unexpected failure");

        assert_eq!(res.val(), Value::Int(2));
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
