use crate::parser::Node;

fn eval(ast: Node) {
    // traverse and evaluate the AST
    match ast {
        Node::BinaryExpression { left, operator, right } => {
            let left = eval(*left);
        }
        Node::UnaryExpression { operator, right } => {
            
        }
        Node::Identifier { value, .. } => {
            
        }
        Node::EmptyNode => {}
    }
}