use std::collections::HashMap;
use crate::internal_types::{fetch_array, fetch_hash_map, fetch_integer, fetch_string};
use crate::node::{Node, OperatorKind};
use crate::value::Value;

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
        Node::AssignStmt { left, right, .. } => {
            // println!("{:indent$}{:?} = {:?};", "", left, right, indent=indent);
            // left
            print_with_indent(*left.clone(), indent +2);
            // right
            print_with_indent(*right.clone(), indent +2);
        }
        Node::Ident { name, .. } => {
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
                    Node::Ident { name, .. } => {
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
            Node::Atomic { .. } => {}
            Node::EmptyNode => {}
            Node::HMap { values } => {
                self.declarations.insert(String::from(""), Node::HMap { values });
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
                operator,
                right,
            } => {
                let res = self.eval_unary_expression(*right, operator)?;
                return Ok(res);
            }
            Node::Identifier { value: _value, .. } => {
                "Identifiers";
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

                let mut res = Node::EmptyNode;
                if let Node::Call { name, arguments, .. } = *right.clone() {
                    res = self.eval_call(name, arguments)?;
                } else {
                    let replaced = self.replace_var(*right.clone())?;
                    res = self.eval_node(replaced)?;
                }

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
            Node::Atomic { value } => {
                return Ok(Node::Atomic { value });
            }
            Node::MMExpression { expression } => {
                // todo!("will be migrated")
                // let mut axiom = Axiom::new("ax-1".to_string(), expression);
                // axiom.solve().expect("unexpected failure");
                // println!("{:?}", axiom.steps);
            }
            Node::Type { name: _name } => {
                return Ok(Node::EmptyNode);
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
            Node::EmptyNode => {
                return Ok(Node::EmptyNode);
            }
            Node::Break { .. } => {
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
                    Node::AssignStmt { left, .. } => {
                        let name = fetch_string(*left.clone())?;

                        // fetch the name from the declarations
                        let elements = self.declarations.get(&name).expect("unexpected failure");

                        // check if the value is an array
                        let result = fetch_array(elements.clone())?;
                        let i = extract_index(*index.clone())?;

                        // Using only Atomic values for now
                        Ok(Node::Atomic {
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

                return Ok(Node::EmptyNode);
            }
            Node::Return { value } => {
                // fetch the latest value of the variable
                let res = self.replace_var(*value.clone())?;
                return Ok(res);
            }
            Node::HMap {..} => {
                // TODO: only supports empty initialization
                return Ok(Node::HMap { values: Default::default() });
            }
            Node::MethodCall { name, target, arguments, .. } => {
                let res = self.eval_method_call(name, target, arguments)?;
                if let Node::EmptyNode{} = res {
                    return Ok(Node::EmptyNode);
                }
                return Ok(res);
            }
            _ => {
                println!("{:?}", ast);
                todo!("Unknown node");
            }
        }

        Ok(Node::EmptyNode)
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
            Node::Atomic { value } => {
                return Ok(Node::Atomic { value });
            }
            Node::IndexExpression { left, index } => {
                return self.replace_var_assign(*left, index);
            }
            Node::Ident { name, .. } => {
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
            Node::AssignStmt { left, .. } => {
                let name = fetch_string(*left.clone())?;

                // fetch the name from the declarations
                let elements = self.declarations.get(&name).expect("unexpected failure");

                let result = fetch_array(elements.clone())?;
                let i = extract_index(*index.clone())?;

                // Using only Atomic values for now
                Ok(Node::Atomic {
                    value: result[i].val(),
                })
            }
            Node::IndexExpression {index, .. } => {
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
        operator: OperatorKind,
        mut right: Node,
    ) -> Result<Node, String> {
        match operator {
            OperatorKind::Add => {
                // TODO: fix this. it is ugly
                if let Node::AssignStmt {
                    right: _left_val,
                    left: _name,
                    ..
                } = left.clone()
                {
                    left = self.replace_var(left)?;
                } else if let Node::Ident { .. } = left.clone() {
                    left = self.replace_var(left)?;
                }

                if let Node::AssignStmt {
                    right: _right_val,
                    left: _name,
                    ..
                } = right.clone()
                {
                    right = self.replace_var(right)?;
                } else if let Node::Ident { .. } = right.clone() {
                    right = self.replace_var(right)?;
                }

                if let Node::Atomic { value: left_val } = left {
                    if let Node::Atomic { value: right_val } = right {
                        return match (left_val, right_val) {
                            (Value::Int(left), Value::Int(right)) => Ok(Node::Atomic {
                                value: Value::Int(left + right),
                            }),
                            _ => Err("Invalid types".to_string()),
                        };
                    }
                }
                return Err("Invalid type".to_string())
            }
            OperatorKind::Subtract => {
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

                if let Node::Atomic { value: left_val } = left {
                    if let Node::Atomic { value: right_val } = right {
                        return match (left_val, right_val) {
                            (Value::Int(left), Value::Int(right)) => Ok(Node::Atomic {
                                value: Value::Int(left - right),
                            }),
                            _ => Err("Invalid types".to_string()),
                        };
                    }
                }
            }
            OperatorKind::Multiply => {
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

                if let Node::Atomic { value: left_val } = left {
                    if let Node::Atomic { value: right_val } = right {
                        return match (left_val, right_val) {
                            (Value::Int(left), Value::Int(right)) => Ok(Node::Atomic {
                                value: Value::Int(left * right),
                            }),
                            _ => Err("Invalid types".to_string()),
                        };
                    }
                }
            }
            OperatorKind::Divide => {
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

                if let Node::Atomic { value: left_val } = left {
                    if let Node::Atomic { value: right_val } = right {
                        return match (left_val, right_val) {
                            (Value::Int(left), Value::Int(right)) => Ok(Node::Atomic {
                                value: Value::Int(left / right),
                            }),
                            _ => Err("Invalid types".to_string()),
                        };
                    }
                }
            }
            OperatorKind::Modulo => {
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

                if let Node::Atomic { value: left_val } = left {
                    if let Node::Atomic { value: right_val } = right {
                        return match (left_val, right_val) {
                            (Value::Int(left), Value::Int(right)) => Ok(Node::Atomic {
                                value: Value::Int(left % right),
                            }),
                            _ => Err("Invalid types".to_string()),
                        };
                    }
                }
            }
            OperatorKind::BitwiseAnd => {
                if let Node::AssignStmt {
                    right: _left_val,
                    ..
                } = left.clone()
                {
                    left = self.replace_var(left)?;
                }
                if let Node::AssignStmt {
                    right: _right_val,
                    ..
                } = right.clone()
                {
                    right = self.replace_var(right)?;
                }

                if let Node::Atomic { value: left_val } = left {
                    if let Node::Atomic { value: right_val } = right {
                        return match (left_val.clone(), right_val.clone()) {
                            (Value::Bin(left), Value::Bin(right)) => Ok(Node::Atomic {
                                value: Value::Bin(left & right),
                            }),
                            (Value::Int(left), Value::Bin(right)) => Ok(Node::Atomic {
                                value: Value::Bin(left as u32 & right),
                            }),
                            (Value::Int(left), Value::Int(right)) => Ok(Node::Atomic {
                                value: Value::Bin(left as u32 & right as u32),
                            }),
                            _ => {
                                println!("left_val: {:?}", left_val);
                                println!("right_val: {:?}", right_val);
                                Err("Invalid types".to_string())
                            },
                        };
                    }
                }
            }
            OperatorKind::BitwiseOr => {
                if let Node::AssignStmt {
                    right: _left_val,
                    ..
                } = left.clone()
                {
                    left = self.replace_var(left)?;
                }
                if let Node::AssignStmt {
                    right: _right_val,
                    ..
                } = right.clone()
                {
                    right = self.replace_var(right)?;
                }

                if let Node::Atomic { value: left_val } = left {
                    if let Node::Atomic { value: right_val } = right {
                        return match (left_val, right_val) {
                            (Value::Bin(left), Value::Bin(right)) => Ok(Node::Atomic {
                                value: Value::Bin(left | right),
                            }),
                            (Value::Int(left), Value::Bin(right)) => Ok(Node::Atomic {
                                value: Value::Bin(left as u32 | right),
                            }),
                            (Value::Int(left), Value::Int(right)) => Ok(Node::Atomic {
                                value: Value::Bin(left as u32 | right as u32),
                            }),
                            _ => Err("Invalid types".to_string()),
                        };
                    }
                }
            }
            OperatorKind::BitwiseXor => {
                if let Node::AssignStmt {
                    right: _left_val,
                    ..
                } = left.clone()
                {
                    left = self.replace_var(left)?;
                }
                if let Node::AssignStmt {
                    right: _right_val,
                    ..
                } = right.clone()
                {
                    right = self.replace_var(right)?;
                }

                if let Node::Atomic { value: left_val } = left {
                    if let Node::Atomic { value: right_val } = right {
                        return match (left_val, right_val) {
                            (Value::Bin(left), Value::Bin(right)) => Ok(Node::Atomic {
                                value: Value::Bin(left ^ right),
                            }),
                            (Value::Int(left), Value::Bin(right)) => Ok(Node::Atomic {
                                value: Value::Bin(left as u32 ^ right),
                            }),
                            (Value::Int(left), Value::Int(right)) => Ok(Node::Atomic {
                                value: Value::Bin(left as u32 ^ right as u32),
                            }),
                            _ => Err("Invalid types".to_string()),
                        };
                    }
                }
            }
            OperatorKind::ShiftLeft => {
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

                if let Node::Atomic { value: left_val } = left {
                    if let Node::Atomic { value: right_val } = right {
                        return match (left_val.clone(), right_val.clone()) {
                            (Value::Bin(left), Value::Int(right)) => Ok(Node::Atomic {
                                value: Value::Bin(left << right),
                            }),
                            (Value::Int(left), Value::Bin(right)) => Ok(Node::Atomic {
                                value: Value::Bin(left as u32 & right),
                            }),
                            (Value::Int(left), Value::Int(right)) => Ok(Node::Atomic {
                                value: Value::Bin((left << right) as u32),
                            }),
                            _ => {
                                println!("left_val: {:?}", left_val);
                                println!("right_val: {:?}", right_val);
                                Err("Invalid types".to_string())
                            },
                        };
                    }
                }
            }
            OperatorKind::ShiftRight => {
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

                if let Node::Atomic { value: left_val } = left {
                    if let Node::Atomic { value: right_val } = right {
                        return match (left_val, right_val) {
                            (Value::Bin(left), Value::Int(right)) => Ok(Node::Atomic {
                                value: Value::Bin(left >> right),
                            }),
                            _ => Err("Invalid types".to_string()),
                        };
                    }
                }
            }
            _ => return Err("Unknown operator".to_string()),
        }

        Ok(Node::EmptyNode)
    }

    fn eval_unary_expression(
        &mut self,
        mut right: Node,
        operator: OperatorKind,
    ) -> Result<Node, String> {
        match operator {
            OperatorKind::Negation => {
                if let Node::AssignStmt {
                    right: _right_val,
                    left: _name,
                    ..
                } = right.clone()
                {
                    right = self.replace_var(right)?;
                }

                if let Node::Atomic { value: right_val } = right {
                    return match right_val {
                        Value::Bin(right) => Ok(Node::Atomic {
                            value: Value::Bin(!right),
                        }),
                        _ => Err("Invalid types".to_string()),
                    };
                }
                Ok(right)
            }
            _ => {
                Err("Unknown operator".to_string())
            }
        }
    }
    // pub fn parse(&mut self, input: &str) -> Result<(), String> {
    //     let mut parser = crate::lang_parser::LangParser::new(input);
    //     let ast = parser.parse().expect("unexpected failure");
    //
    //     for node in ast.nodes {
    //         self.add_node(node);
    //     }
    //
    //     Ok(())
    // }

    fn eval_call(&mut self, name: String, arguments: Vec<Node>) -> Result<Node, String> {
        // TODO: Have the call names be enums

        match name.as_str() {
            "print" => {
                // check if the first argument is an Object
                let temp2 = *arguments[0].left()?;

                match temp2 {
                    Node::Call { name, arguments, .. } => {
                        let x = self.eval_call(name, arguments)?;
                        println!("{:?}", x.val());
                        return Ok(Node::EmptyNode)
                    }
                    Node::IndexExpression { left, index } => {
                        let x = self.eval_array(Node::IndexExpression { left, index })?;
                        if let Node::Atomic { value } = x {
                            if let Value::Str(_0) = value {
                                println!("[{:?}]", _0);
                            } else {
                                println!("{:?}", value);
                            }
                        } else {
                            println!("{:?}", x);
                        }

                        return Ok(Node::EmptyNode)
                    }
                    _ => {}
                };

                println!("{:?}", self.replace_var(temp2).expect("unexpected failure"));
                return Ok(Node::EmptyNode)
            }
            "reduce" => {
                println!("{:?}", arguments[0].left());
            }
            "len" => {
                let name = fetch_string(arguments[0].clone())?;
                let n = self.declarations.get(&name).expect("missing declaration").clone();

                return match n.node_type() {
                    "Array" => {
                        let elements = fetch_array(n.clone())?;
                        Ok(Node::Atomic {
                            value: Value::Int(elements.len() as i32),
                        })
                    }
                    "Hashmap" => {
                        let elements = fetch_hash_map(n.clone())?;
                        Ok(Node::Atomic {
                            value: Value::Int(elements.len() as i32),
                        })
                    }
                    "Atomic" => {
                        let s = fetch_string(n.clone())?;
                        Ok(Node::Atomic {
                            value: Value::Int(s.len() as i32),
                        })
                    }
                    _ => {
                        return Err(format!("Unknown type {}", n.node_type()));
                    }
                }
            }
            _ => {
                println!("arguments: {:?}", arguments);
                // function call is not a builtin function
                let func = self.declarations.get(&name).expect("unexpected failure").clone();

                let node = self.eval_function(func, arguments)?;

                // upsert node
                self.upsert_declaration(node.clone())?;

                return Ok(node);
            }
        }

        Ok(Node::EmptyNode)
    }

    fn eval_method_call(&mut self, name: String, target: String, arguments: Vec<Node>) -> Result<Node, String> {
        match name.as_str() {
            "push" => {
                let array = self.declarations.get(&target).expect("missing declaration").clone();
                let mut elements = fetch_hash_map(array.clone())?;
                elements.insert(arguments[0].clone(), arguments[1].clone());

                let ident = Node::AssignStmt {
                    left: Box::new(Node::Ident { name: target, kind: "var".to_string() }),
                    right: Box::from(Node::HMap { values: elements }),
                    kind: "var".to_string(),
                };
                self.upsert_declaration(ident)?;
                Ok(Node::EmptyNode)
            }
            "get" => {
                let array = self.declarations.get(&target).expect("missing declaration").clone();
                let elements = fetch_hash_map(array.clone())?;
                let key = arguments[0].clone();
                let value = elements.get(&key).expect("missing key").clone();
                Ok(value)
            }
            _ => {
                Err(format!("Unknown method call {}", name))
            }
        }
    }

    fn eval_function(&mut self, node: Node, arguments: Vec<Node>) -> Result<Node, String> {
        // make sure that the node is a function
        if let Node::FunctionDecl {
            name: _name,
            arguments: args,
            returns: _returns,
            body} = node
        {
            let mut ret = Node::EmptyNode;
            for i in 0..args.len() {
                let name = fetch_string(args[i].clone())?;
                self.upsert_declaration(Node::AssignStmt {
                    left: Box::new(Node::Ident {
                        name: name.clone(),
                        kind: "var".to_string(),
                    }),
                    right: Box::new(arguments[i].clone()),
                    kind: "Nat".to_string(),
                })?;
            }

            for expr in body {
                // check if node is a return statement
                if let Node::Return { value } = expr {

                    // TODO: this assumes a variable, eventually this should also evaluate a node
                    ret = self.replace_var(*value)?;
                    break;
                }

                self.eval_node(expr)?;
            }

            for arg in args {
                self.remove_declaration(&fetch_string(arg)?)?;
            }

            return Ok(ret);
        }

        Ok(Node::EmptyNode)
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
                    OperatorKind::IsEqual => {
                        // TODO: This is using left.left and right.left which is an oopsie
                        let pre_left = left.left()?;
                        let replaced_left = self.replace_var(*pre_left.clone())?;

                        let pre_right = right.left()?;
                        let replaced_right = self.replace_var(*pre_right.clone())?;

                        let best = crate::value::compare_value(&replaced_left.val(), &replaced_right.val());
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
                    OperatorKind::Or => {
                        // TODO: Sub expressions will need to be evaluated recursively
                        let sub_left = left.left()?;
                        let sub_right = left.right()?;

                        let replaced_sub_left = self.replace_var(*sub_left.clone())?;
                        let replaced_sub_right = self.replace_var(sub_right.clone())?;

                        // check the truthiness of the left and right
                        let left_truth = crate::value::compare_value(&replaced_sub_left.val(), &replaced_sub_right.val());

                        let sub_right_left = right.left()?;
                        let sub_right_right = right.right()?;

                        let replaced_sub_right_left = self.replace_var(*sub_right_left.clone())?;
                        let replaced_sub_right_right = self.replace_var(sub_right_right.clone())?;

                        let right_truth = crate::value::compare_value(&replaced_sub_right_left.val(), &replaced_sub_right_right.val());

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
                    OperatorKind::LessThan | OperatorKind::GreaterThan => {
                        let replaced_left = self.replace_var(*left.clone())?;

                        let mut ordering = std::cmp::Ordering::Greater;
                        match operator {
                            OperatorKind::LessThan => {
                                ordering = std::cmp::Ordering::Less;
                            }
                            OperatorKind::GreaterThan => {
                                ordering = std::cmp::Ordering::Greater;
                            }
                            _ => {}
                        }

                        match replaced_left.val() {
                            Value::Int(i) => {
                                match right.val() {
                                    Value::Int(ii) => {
                                        let _ordering = i.cmp(&ii);
                                        for node in consequence.clone() {
                                            let res = self.eval_node(node)?;
                                            self.upsert_declaration(res)?
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
            Node::Atomic { value } => {
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

                if let Node::AssignStmt { left, .. } = *left {
                    if let Node::Ident { name, .. } = *left {
                        let string_node = self.declarations.get(&name).expect("missing x").clone();
                        return match string_node.node_type() {
                            "Array" => {
                                let array = fetch_array(string_node)?;
                                let index = fetch_integer(*sub_index)?;
                                let v = array[index as usize].clone();
                                Ok(v)
                            }
                            "Atomic" => {
                                let replaced = self.replace_var(Node::Ident { name, kind: "".to_string() })?;
                                let s = fetch_string(replaced)?;
                                let replaced_index = self.replace_var(*sub_index)?;
                                let index = fetch_integer(replaced_index)?;
                                let v = s.as_bytes()[index as usize] as char;

                                Ok(Node::Atomic { value: crate::value::Value::Str(v.to_string()) })
                            }
                            _ => {
                                println!("kind: {:?}", string_node.node_type());
                                Err("Invalid type".to_string())
                            }
                        }
                    }
                }

                Ok(Node::EmptyNode)
            }
            Node::Ident {name, kind} => {
                // fetch the ident by name from declarations and return
                Ok(Node::Ident { name, kind })
            }
            _=> {
                Err(format!("expected index expression but received {:?} while evaluating array", node))
            }
        }
    }

    fn parse_for(&mut self, variable: String, range: (Box<Node>, Box<Node>), body: Vec<Node>) -> Result<(), String> {
        let start_node = self.replace_var(*range.0)?;
        let end_node = self.replace_var(*range.1)?;

        let start = fetch_integer(start_node)?;
        let replaced_end = self.replace_var(end_node)?;
        println!("start node: {:?}", start);
        println!("end node: {:?}", replaced_end);
        let end = fetch_integer(replaced_end)?;


        self.upsert_declaration(Node::AssignStmt {
            left: Box::new(Node::Ident {
                name: variable.clone(),
                kind: "var".to_string(),
            }),
            right: Box::new(Node::Atomic {
                value: Value::Int(start),
            }),
            kind: "Nat".to_string(),
        })?;

        // let start = range.0;
        // let end = range.1;
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
                right: Box::new(Node::Atomic {
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
        Node::Atomic { value: Value::Int(i) } => Ok(i as usize),
        _ => {
            Err(format!("expected atomic but received {:?} while extracting index", index))
        }
    }
}
