use crate::parser::{ParseError, Parser};

use ast::node::Node;
use ast::node::OperatorKind;

pub struct Axiom {
    index: usize,
    name: String,
    pub hypothesises: Vec<String>,
    pub steps: Vec<Step>,
    initial_assertion: String,
    parser: Parser,
}

impl Axiom {
    pub fn new(name: String, mut initial_assertion: String) -> Self {
        // check if initial assertion includes a ⇒ or a &, this means that there are hypothesis that need to be parsed and recorded
        
        let mut hypothesises = Vec::new();
        
        if initial_assertion.contains("⇒") {
            // split before and after the ⇒, the first part will be hypothesis, the second part will be the assertion
            let parts: Vec<&str> = initial_assertion.split("⇒").collect();
            let hypothesis = parts[0].trim();
            let assertion = parts[1].trim();
            
            // check if the hypothesis contains a &, this means that there are multiple hypothesis
            if hypothesis.contains("&") {
                let parts: Vec<&str> = hypothesis.split("&").collect();
                
                for part in parts {
                    hypothesises.push(part.trim().to_string());
                }
            } else {
                hypothesises.push(hypothesis.to_string());
            }
            
            // replace the initial assertion with the rest of the string
            initial_assertion = assertion.to_string();
        }
        
        let parser = Parser::new_mm(&initial_assertion);
        
        Self {
            index: 0,
            name,
            hypothesises,
            steps: Vec::new(),
            initial_assertion,
            parser,
        }
    }
    
    pub fn add_step(&mut self, node: Node) -> usize {
        let ref_index = self.index;
        let hypothesis = (0, 0);
        let reference = "".to_string();
        let expression = node.to_string();
        
        // TODO: Nate check if the entry already exists
        for (index, step) in self.steps.iter().enumerate() {
            if step.expression == expression {
                return index;
            }
        }
        
        self.steps.push(Step {
            index: self.index,
            hypothesis,
            reference,
            expression,
        });

        self.index += 1;
        
        ref_index
    }

    pub fn solve(&mut self) -> Result<(), ParseError> {
        
        let node = self.parser.parse()?;
        // add the initial node
        
        println!("string test: {}", node.to_string());

        let (reduce_left, reduce_right) = reduce(node.clone()).unwrap();
        
        self.add_step(node);
        self.add_step(reduce_left);
        self.add_step(reduce_right);
        
        println!("initial assertion: {}", self.initial_assertion);
        let mut i = 0;
        loop {
            if i >= self.steps.len() {
                break;
            }
            
            let step = &self.steps[i];
            // println!("Step: {:?}", step);
            
            let mut parser = Parser::new_mm(step.expression.as_str());
            
            let node = parser.parse()?;
            
            let (reduce_left, reduce_right) = reduce(node).unwrap();
            
            let _ = self.add_step(reduce_left.clone());
            let _ = self.add_step(reduce_right.clone());
            
            i += 1;
        }
        
        Ok(())
    }
    
    pub fn print_steps(&self) {
        for step in self.steps.iter() {
            println!("{:?}", step);
        }
        for step in self.steps.iter() {
            println!("{}", step.expression);
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
        Node::UnaryExpression { operator: _operator, .. } => {
            match node.operator() {
                OperatorKind::Negation => {
                    // |- ¬A becomes A |- ⊥
                    let right = node.right().unwrap();

                    Ok((node.clone(), right.clone()))
                }
                _ => {
                    Err(ReduceError::Unimplemented)
                }
            }
        }
        Node::BinaryExpression { left, operator: _operator, right } => {
            match node.operator() {
                OperatorKind::Implies | OperatorKind::Biconditional => {
                    // |- A -> B becomes A |- B

                    let node_left = *left;
                    let node_right = *right;

                    Ok((node_left, node_right))
                }
                OperatorKind::ForAll | OperatorKind::Equality | OperatorKind::ElementOf | OperatorKind::Conjunction | OperatorKind::Exists | OperatorKind::Disjunction => {
                    Ok((*left, *right))
                }
                _ => {
                    Err(ReduceError::Unimplemented)
                }
            }
        }
        Node::Identifier { value: _value } => {
            // found a single node, can't reduce any further
            Ok((node.clone(), node.clone()))
        }
        _ => {
            Err(ReduceError::Unimplemented)
        }
    }
}

#[cfg(test)]
mod tests {
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
        let input = "(𝜑 → (𝜓 → 𝜑))";
        
        let mut axiom = Axiom::new("ax-1".to_string(), input.to_string());
        axiom.solve().expect("TODO: panic message");

        assert_eq!(axiom.steps.len(), 4);
    }
}