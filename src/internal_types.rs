use std::collections::{BTreeMap, HashMap};
use parser::Value;
use crate::parser;
use crate::parser::Node;

pub(crate) fn fetch_array(node: Node) -> Result<Vec<Node>, String> {
    if let Node::Array { elements } = node {
        Ok(elements)
    } else {
        Err("expected array".to_string())
    }
}

pub(crate) fn fetch_hash_map(node: Node) -> Result<BTreeMap<Node, Node>, String> {
    match node {
        Node::HMap { values } => {
            Ok(values)
        }
        _ => {
            Err(format!("expected hash map, got {:?}", node))
        }
    }
}

pub(crate) fn fetch_string(node: Node) -> Result<String, String> {
    match node {
        Node::Identifier { value } => Ok(value),
        Node::Ident{name, ..} => Ok(name),
        Node::IndexExpression { left, .. } => {
            let left = fetch_string(*left)?;
            Ok(left)
        }
        Node::Atomic { value, .. } => {
            if let Value::Str(s) = value {
                Ok(s)
            } else {
                Err("expected string".to_string())
            }
        }
        Node::AssignStmt { left, right, kind } => {
            let left = fetch_string(*left)?;
            Ok(left)
        }
        _ => Err(format!("unexpected node {:?}", node)),
    }
}

pub(crate) fn fetch_integer(node: Node) -> Result<i32, String> {
    match node {
        Node::Atomic { value } => {
            if let Value::Int(i) = value {
                Ok(i)
            } else {
                Err(format!("expected integer, got {:?}", value))
            }
        }
        _ => {
            println!("expected integer but got: {:?}", node);
            Err(format!("expected integer, got {:?}", node)) 
        },
    }
}

pub(crate) fn fetch_boolean(node: Node) -> Result<bool, String> {
    match node {
        Node::Atomic { value } => {
            if let Value::Bool(b) = value {
                Ok(b)
            } else {
                Err("expected boolean".to_string())
            }
        }
        _ => Err("expected boolean".to_string()),
    }
}

pub(crate) fn fetch_binary(node: Node) -> Result<u32, String> {
    match node {
        Node::Atomic { value } => {
            if let Value::Bin(b) = value {
                Ok(b)
            } else {
                Err(format!("expected binary operator but got {:?}", value))
            }
        },
        _ => Err(format!("expected binary operator but got {:?}", node)),
    }
}