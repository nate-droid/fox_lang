use std::collections::HashMap;
use crate::parser::{Node, Parser};
use crate::lexer::TokenKind;

pub struct Axiom {
    index: usize,
    name: String,
    hypothesises: Vec<(String, String)>,
    pub(crate) best_steps: Vec<Step>,
    initial_assertion: String,
    parser: Parser,
}

impl Axiom {
    pub fn new(name: String, hypothesises: Vec<(String, String)>, initial_assertion: String, parser: Parser) -> Self {
        Self {
            index: 0,
            name,
            hypothesises,
            best_steps: Vec::new(),
            initial_assertion,
            parser,
        }
    }
    
    pub fn add_best_step(&mut self, node: Node) -> usize {
        let ref_index = self.index.clone();
        let hypothesis = (0, 0);
        let reference = "".to_string();
        let expression = node.to_string();
        
        // TODO: Nate check if the entry already exists
        for (index, step) in self.best_steps.iter().enumerate() {
            if step.expression == expression {
                return index;
            }
        }
        
        self.best_steps.push(Step {
            index: self.index,
            hypothesis,
            reference,
            expression,
        });

        self.index += 1;
        
        ref_index
    }

    // TODO: add a return type to handle errors
    pub fn solve(&mut self) {
        let node = self.parser.parse().unwrap();
        // add the initial node
        
        // self.add_step(node.clone());
        println!("string test: {}", node.to_string());
        
        self.add_best_step(node.clone());
        
        println!("initial assertion: {}", self.initial_assertion);
        let mut i = 0;
        loop {
            if i >= self.best_steps.len() {
                break;
            }
            
            let step = &self.best_steps[i];
            println!("{:?}", step);
            
            let mut parser = Parser::new(step.expression.clone());
            
            let node = parser.parse().unwrap();
            let (reduce_left, reduce_right) = reduce(node).unwrap();
            let left_index = self.add_best_step(reduce_left.clone());
            let right_index = self.add_best_step(reduce_right.clone());
            
            i += 1;
        }
    }
    
    pub fn print_steps(&self) {
        for step in self.best_steps.iter() {
            println!("{:?}", step);
        }
    }
}

#[derive(Debug)]
pub(crate) struct Step {
    index: usize,
    hypothesis: (usize, usize),
    reference: String,
    expression: String,
}

impl Step {
    pub(crate) fn new(node: Node) -> Self {
        let index = 0;
        let hypothesis = (0, 0);
        let reference = "".to_string();
        let expression = node.to_string();

        Self {
            index,
            hypothesis,
            reference,
            expression,
        }
    }
}


#[derive(Debug)]
pub enum ReduceError {
    EmptyNode,
    Unimplemented,
}

impl std::fmt::Display for ReduceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ReduceError {}

// This function should return a box of nodes or an error
pub fn reduce(node: Node) -> Result<(Node, Node), ReduceError>{
    match node.clone() {
        Node::UnaryExpression {operator, .. } => {
            println!("Found a unary expression");
        }
        Node::BinaryExpression { left, operator, right } => {
            match node.operator() {
                TokenKind::Implies => {
                    // |- A -> B becomes A |- B

                    let node_left = *left;
                    let node_right = *right;

                    return Ok((node_left, node_right));
                }
                _ => {
                    return Err(ReduceError::Unimplemented);
                }
            }
        }
        Node::Identifier {value} => {
            // found a single node, can't reduce any further
            return Ok((node.clone(), node.clone()));
        }
        _ => {
            return Err(ReduceError::Unimplemented);
        }
    }

    Err(ReduceError::EmptyNode)
}

#[cfg(test)]
mod tests {
    use crate::lexer::DefaultLexer;
    use super::*;

    #[test]
    fn test_simple_reduce() {
        let input = "(A -> B)";
        let mut parser = Parser::new(input.to_string());

        let node = parser.parse();

        let result = reduce(node.unwrap());

        // assert that the result is A, B
        let (left, right) = result.unwrap();

        if let Node::Identifier { value } = left {
            assert_eq!(value, "A");
        } else {
            panic!("Expected Node::Identifier");
        }
        if let Node::Identifier { value } = right {
            assert_eq!(value, "B");
        } else {
            panic!("Expected Node::Identifier");
        }
    }

    #[test]
    fn test_recursive_reduce() {
        let input = "(A -> (B -> C))";
        
        let parser = Parser::new(input.to_string());
        let mut axiom = Axiom::new("ax-1".to_string(), vec![], input.to_string(), parser);
        axiom.solve();

        assert_eq!(axiom.best_steps.len(), 5);
    }
}