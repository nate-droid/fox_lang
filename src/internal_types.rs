use crate::parser::Node;

pub(crate) fn fetch_array(node: Node) -> Result<Vec<Node>, String> {
    if let Node::Array { elements } = node {
        Ok(elements)
    } else {
        Err("expected array".to_string())
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
        _ => Err("unexpected type".to_string()),
    }
}

pub(crate) fn fetch_integer(node: Node) -> Result<i32, String> {
    match node {
        Node::Atomic { value } => {
            if let crate::parser::Value::Int(i) = value {
                Ok(i)
            } else {
                Err("expected integer".to_string())
            }
        }
        _ => Err("expected integer".to_string()),
    }
}

pub(crate) fn fetch_boolean(node: Node) -> Result<bool, String> {
    match node {
        Node::Atomic { value } => {
            if let crate::parser::Value::Bool(b) = value {
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
            if let crate::parser::Value::Bin(b) = value {
                Ok(b)
            } else {
                Err(format!("expected binary operator but got {:?}", value))
            }
        },
        _ => Err(format!("expected binary operator but got {:?}", node)),
    }
}