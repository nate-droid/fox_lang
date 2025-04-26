use std::collections::BTreeMap;
use std::fmt;
use std::fmt::Display;

impl Display for OperatorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperatorKind::ForAll => write!(f, "∀"),
            OperatorKind::Exists => write!(f, "∃"),
            OperatorKind::Implies => write!(f, "→"),
            OperatorKind::Disjunction => write!(f, "∨"),
            OperatorKind::Conjunction => write!(f, "∧"),
            OperatorKind::Equality => write!(f, "="),
            OperatorKind::ElementOf => write!(f, "∈"), 
            OperatorKind::Identifier => write!(f, "Identifier"),
            OperatorKind::Add => write!(f, "+"),
            OperatorKind::Subtract => write!(f, "-"),
            OperatorKind::Multiply => write!(f, "*"),
            OperatorKind::Divide => write!(f, "/"),
            OperatorKind::Modulo => write!(f, "%"),
            OperatorKind::BitwiseAnd => write!(f, "&"),
            OperatorKind::BitwiseOr => write!(f, "|"),
            OperatorKind::BitwiseXor => write!(f, "^"),
            OperatorKind::ShiftLeft => write!(f, "<<"),
            OperatorKind::ShiftRight => write!(f, ">>"),
            OperatorKind::Negation => write!(f, "¬"),
            OperatorKind::IsEqual => write!(f, "=="),
            OperatorKind::Or => write!(f, "||"),
            OperatorKind::LessThan => write!(f, "<"),
            OperatorKind::GreaterThan => write!(f, ">"),
            OperatorKind::Biconditional => write!(f, "↔"),
            OperatorKind::And => write!(f, "&&"),
        } 
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum OperatorKind {
    ForAll,
    Exists,
    Implies,
    Disjunction,
    Conjunction,
    Equality,
    ElementOf,
    Identifier,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    ShiftLeft,
    ShiftRight,
    Negation,
    IsEqual,
    Or,
    And,
    LessThan,
    GreaterThan,
    Biconditional,
    
}

#[derive(Debug, Clone, PartialOrd)]
#[derive(Eq, PartialEq, Ord)]
pub enum Node {
    BinaryExpression {
        left: Box<Node>,
        operator: OperatorKind,
        right: Box<Node>,
    },
    UnaryExpression {
        operator: OperatorKind,
        right: Box<Node>,
    },
    Identifier {
        value: String,
    },
    AssignStmt {
        left: Box<Node>,
        right: Box<Node>,
        kind: String,
    },
    Ident {
        name: String,
        kind: String,
    },
    Atomic {
        value: crate::value::Value,
    },
    Call {
        name: String,
        arguments: Vec<Node>,
        returns: Vec<Node>,
    },
    MethodCall {
        name: String,
        target: String, // what the method is called on
        arguments: Vec<Node>,
        returns: Vec<Node>,
    },
    MMExpression {
        expression: String,
    },
    Type {
        name: String,
    },
    Conditional {
        condition: Box<Node>,
        consequence: Vec<Node>,
        alternative: Vec<Node>,
    },
    ForLoop {
        variable: String,
        // range: (i32, i32),
        range: (Box<Node>, Box<Node>),
        body: Vec<Node>,
    },
    Array {
        elements: Vec<Node>,
    },
    EmptyNode,
    Object {
        name: String,
        kind: String,
    },
    Break {

    },
    IndexExpression {
        left: Box<Node>,
        index: Box<Node>,
    },
    FunctionDecl {
        name: Box<Node>,
        arguments: Vec<Node>,
        returns: Vec<Node>,
        body: Vec<Node>,
    },
    Return {
        value: Box<Node>,
    },
    HMap {
        values: BTreeMap<Node, Node>,
    },
}

impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::BinaryExpression { left, operator, right } => {
                match operator {
                    OperatorKind::ForAll => {
                        write!(f, "∀{}{}", left, right)
                    }
                    OperatorKind::Equality => {
                        write!(f, "{} = {}", left, right)
                    }
                    OperatorKind::ElementOf => {
                        write!(f, "{} ∈ {}", left, right)
                    }
                    OperatorKind::Exists => {
                        write!(f, "∃{}{}", left, right)
                    }
                    OperatorKind::Implies => {
                        write!(f, "({} → {})", left, right)
                    }
                    OperatorKind::Disjunction => {
                        write!(f, "{} ∨ {}", left, right)
                    }
                    _ => {
                        write!(f, "({} {} {})", left, operator, right)
                    }
                }
            }
            Node::UnaryExpression { operator, right } => {
                write!(f, "({} {})", operator, right)
            }
            Node::Identifier { value } => {
                write!(f, "{}", value.clone())
            }
            Node::AssignStmt { left, right, kind } => {
                // write!(f, "{} : {} = {:?}", name, kind, value)
                write!(f, "{} = {}", left, right)
            }
            Node::Atomic { value } => {
                write!(f, "{}", value)
            }
            Node::Call { name, arguments, returns: _returns } => {
                write!(f, "{}({:?})", name, arguments)
            }
            Node::MMExpression { expression } => {
                write!(f, "{}", expression.clone())
            }
            Node::Type { name } => {
                write!(f, "{}", name.clone())
            }
            Node::Conditional { condition, consequence, alternative } => {
                todo!()
            }
            Node::ForLoop { variable, range, body } => {
                write!(f, "for {} in {}..{} {{ {:?} }}", variable, range.0, range.1, body)
            }
            Node::EmptyNode => {
                write!(f, "")
            }
            Node::Object { name, .. } => {
                write!(f, "{}", name)
            }
            Node::Array { elements } => {
                write!(f, "[{:?}]", elements)
            }
            Node::Break {} => {
                write!(f, "break")
            }
            Node::IndexExpression { left, index } => {
                write!(f, "{}[{}]", left, index)
            }
            Node::Ident { name, kind } => {
                write!(f, "{} : {}", name, kind)
            }
            Node::FunctionDecl { name, arguments, returns, body } => {
                write!(f, "fn {}({:?}) -> {:?} {{ {:?} }}", name, arguments, returns, body)
            }
            Node::Return { value } => {
                write!(f, "return {:?}", value)
            }
            Node::HMap { values } => {
                write!(f, "{:?}", values)
            }
            Node::MethodCall { name, target, arguments, returns } => {
                write!(f, "{}.{}({:?}) -> {:?}", target, name, arguments, returns)
            }
        }
    }
}
impl Node {

    pub fn operator(&self) -> OperatorKind {
        match self {
            Node::BinaryExpression { operator, .. } => {
                operator.clone()
            }
            Node::UnaryExpression { operator, .. } => {
                operator.clone()
            }
            Node::Identifier { .. } => {
                OperatorKind::Identifier
            }
            _ => {
                println!("{:?}", self);
                todo!()
            }
        }
    }

    pub fn left(&self) -> Result<Box<Node>, String> {

        match self {
            Node::Atomic { value } => {
                Ok(Box::from(Node::Atomic { value: value.clone() }))
            },
            Node::AssignStmt { left, right, kind } => {
                Ok(left.clone())
            },
            Node::BinaryExpression { left, .. } => Ok(Box::from(*left.clone())),
            Node::Object { name, kind } => Ok(Box::from(Node::Object { name: name.clone(), kind: kind.clone() })),
            Node::IndexExpression { left, index } => Ok(Box::from(Node::IndexExpression { left: left.clone(), index: index.clone() })),
            Node::Call { name, arguments, .. } => {
                Ok(Box::from(Node::Call { name: name.clone(), arguments: arguments.clone(), returns: vec![] }))
            }
            Node::Ident { name, kind } => Ok(Box::from(Node::Ident { name: name.clone(), kind: kind.clone() })),
            _ => {
                Err(format!("unexpected token {:?}", self))
            },
        }
    }

    pub fn right(&self) -> Result<&Node, String> {
        match self {
            Node::BinaryExpression { right, .. } => Ok(right),
            Node::UnaryExpression { right, .. } => Ok(right),
            _ => Err(format!("unexpected token {:?}", self)),
        }
    }

    pub fn val(&self) -> crate::value::Value {
        match self {
            Node::Atomic { value } => value.clone(),
            _ => crate::value::Value::Str("".to_string()),
        }
    }

    pub fn node_type(&self) -> &str {
        match self {
            Node::BinaryExpression { .. } => "BinaryExpression",
            Node::UnaryExpression { .. } => "UnaryExpression",
            Node::Array { .. } => "Array",
            Node::AssignStmt { .. } => "AssignStmt",
            Node::Ident { .. } => "Ident",
            Node::Atomic { .. } => "Atomic",
            Node::Call { .. } => "Call",
            Node::Conditional { .. } => "Conditional",
            Node::EmptyNode => "EmptyNode",
            _ => "MissingType",
        }
    }
}


