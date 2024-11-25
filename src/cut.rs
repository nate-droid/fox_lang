use std::collections::HashMap;
use crate::parser::{Node, Parser};
use crate::lexer::TokenKind;

pub struct Axiom {
    index: usize,
    name: String,
    hypothesises: Vec<(String, String)>,
    pub(crate) steps: HashMap<usize, Step>,
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
            steps: HashMap::new(),
            best_steps: Vec::new(),
            initial_assertion,
            parser,
        }
    }

    pub fn add_step(&mut self, node: Node) {
        let hypothesis = (0, 0);
        let reference = "".to_string();
        let expression = node.to_string();

        self.steps.insert(self.index, Step {
            index: self.index,
            hypothesis,
            reference,
            expression,
        });

        self.index += 1;
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

    pub fn solve(&mut self) {
        // let mut parser = Parser::new_mm(self.initial_assertion.to_string());
        // let node = parser.parse().unwrap();
        let node = self.parser.parse().unwrap();
        // add the initial node
        
        // self.add_step(node.clone());
        println!("string test: {}", node.to_string());
        println!("left: {:?}", node.left());
        println!("right: {:?}", node.right());
        // TODO: node to string doesn't work correctly
        self.add_best_step(node.clone());

        // if let Node::BinaryExpression { left, operator, right } = node.clone() {
        //     // reduce the node
        //     let (reduce_left, reduce_right) = reduce(node.clone()).unwrap();
        // 
        //     // TODO: Need to add the correct Hypothesis and Reference
        //     self.add_step(reduce_left.clone());
        //     self.add_step(reduce_right.clone());
        //     
        //     // self.add_best_step(reduce_left.clone());
        //     // self.add_best_step(reduce_right.clone());
        // }

        // self.index = 0;
        // loop {
        //     let mut step = &Step{
        //         index: 0,
        //         hypothesis: (0, 0),
        //         reference: "".to_string(),
        //         expression: "".to_string(),
        //     };
        // 
        //     if let Some(found_step) = self.steps.get(&self.index) {
        //         step = found_step;
        //     } else {
        //         break;
        //     }
        //     self.index += 1;
        // 
        //     // TODO: Check if is not an Identifier and doesn't have a hypothesis
        //     
        //     let mut parser = Parser::new(step.expression.clone());
        //     let node = parser.parse().unwrap();
        // 
        //     if let Node::Identifier { value } = node.clone() {
        //         self.index += 1;
        //         // self.add_step(node.clone());
        //         continue;
        //     }
        // 
        //     let (reduce_left, reduce_right) = reduce(node).unwrap();
        // 
        //     self.add_step(reduce_left.clone());
        //     self.add_step(reduce_right.clone());
        // }
        
        println!("{:?}", self.best_steps);
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
        
        // let best_steps: Vec<Node> = self.best_steps.iter()
        //     .map(|step| { 
        //         let mut parser = Parser::new(step.expression.clone());
        //         parser.parse().unwrap() 
        //     })
        //     .collect();
        // 
        // for node in best_steps {
        //     let (reduce_left, reduce_right) = reduce(node).unwrap();
        //     self.add_best_step(reduce_left.clone());
        //     self.add_best_step(reduce_right.clone());
        // }
    }
    
    pub fn print_steps(&self) {
        for (index, step) in self.steps.iter() {
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

        let lexer = DefaultLexer::new(input.to_string());

        let mut parser = Parser::new(input.to_string());
        let node = parser.parse().unwrap();
        let mut axiom = Axiom::new("ax-1".to_string(), vec![], node.to_string(), parser);
        axiom.solve();
        println!("{:?}", axiom.steps);

        assert_eq!(axiom.steps.len(), 4);
    }
}