use std::fmt::Error;
use crate::lexer::{Token, TokenKind};
use crate::lexer::Lexer;

pub struct Parser {
    lexer: Lexer,
    position: usize,
    tokens: Vec<Token>,
}

use std::fmt;
use crate::parser::ParseError::EmptyNode;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken,
    EmptyNode, // this is when there is no node to parse
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken => write!(f, "Unexpected token"),
            ParseError::EmptyNode => write!(f, "Empty node"),
        }
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug, Clone)]
pub enum Node {
    BinaryExpression {
        left: Box<Node>,
        operator: TokenKind,
        right: Box<Node>,
    },
    UnaryExpression {
        operator: TokenKind,
        right: Box<Node>,
    },
    Identifier {
        value: String,
    },
    EmptyNode,
}

impl Node {
    fn evaluate(&self) -> bool {
        match self {
            Node::BinaryExpression { left, operator, right } => {
                match operator {
                    _ => false
                }
            }
            Node::UnaryExpression { operator, right } => {
                match operator {
                    _ => false
                }
            }
            Node::Identifier { value } => { false }
            _ => {
                false
            }
        }

    }

    pub fn operator(&self) -> TokenKind {
        match self {
            Node::BinaryExpression { left, operator, right } => {
                operator.clone()
            }
            Node::UnaryExpression { operator, right } => {
                operator.clone()
            }
            Node::Identifier { value } => {
                TokenKind::Identifier
            }
            _ => {
                TokenKind::End
            }
        }
    }

    pub fn left(&self) -> Option<&Box<Node>> {

        match self {
            Node::BinaryExpression { left, .. } => Some(left),
            _ => None,
        }
    }

    pub fn right(&self) -> Option<&Box<Node>> {
        match self {
            Node::BinaryExpression { right, .. } => Some(right),
            _ => None,
        }
    }
}

impl Parser {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer::new(input);
        lexer.tokenize();

        let tokens = lexer.tokens.clone();

        Self {
            lexer,
            position: 0,
            tokens,
        }
    }

    pub fn parse(&mut self) -> Result<Node, ParseError> {
        while self.position < self.lexer.tokens.len() {

            match self.tokens[self.position].kind {
                TokenKind::LeftParenthesis => {

                    self.advance();

                    let expr = self.parse_expression()?;

                    println!("Best Expression: {:?}", expr);
                    println!("current: {:?}", self.tokens[self.position]);
                    // match self.tokens[self.position].kind {
                    //     TokenKind::Identifier => {
                    //
                    //         let left_node = Node::Identifier {
                    //             value: self.tokens[self.position].value.clone(),
                    //         };
                    //
                    //         self.advance();
                    //
                    //         let operator = self.tokens[self.position].kind.clone();
                    //
                    //         self.advance();
                    //
                    //         let right_node = Node::Identifier {
                    //             value: self.tokens[self.position].value.clone(),
                    //         };
                    //
                    //         let binary_expression = Node::BinaryExpression {
                    //             left: Box::new(left_node),
                    //             operator,
                    //             right: Box::new(right_node),
                    //         };
                    //         println!("Binary Expression: {:?}", binary_expression);
                    //
                    //         // TODO: add an "expect" function to check if the next token is a RightParenthesis
                    //         return Ok(binary_expression);
                    //     }
                    //     TokenKind::UnaryOperator => {
                    //         println!("Unary Operator");
                    //     }
                    //     TokenKind::LeftParenthesis => {
                    //         // need to parse another sub-expression recursively
                    //         let x = self.parse();
                    //         println!("sub results!");
                    //         println!("{:?}", x);
                    //         // TODO: Nate, come back here and unpack the results and store in the parent node
                    //
                    //     }
                    //     _ => {
                    //         println!("Unexpected token {:?}", self.tokens[self.position]);
                    //         Err(ParseError::UnexpectedToken)?;
                    //     }
                    // }
                }
                TokenKind::RightParenthesis => {
                    // End "parse Expression"
                    println!("End parsing Expression");
                }
                TokenKind::Implies => {
                    println!("Implies");
                }
                TokenKind::Identifier => {
                    println!("Identifier");
                }
                _ => {
                    println!("Other");
                }
            }
            self.advance();

        }

        Err(EmptyNode)
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn parse_expression(&mut self) -> Result<Node, ParseError> {
        let mut node = Node::EmptyNode;
        match self.tokens[self.position].kind {
            TokenKind::LeftParenthesis => {
                // this is a sub expression
                self.advance();
                let sub_expression = self.parse_expression()?;
                node = sub_expression;
                // TODO: What is the best way to handle the results of the sub expression?
            }
            TokenKind::Identifier => {
                let binary_expression = self.parse_binary_expression()?;
                // TODO: add an "expect" function to check if the next token is a RightParenthesis
                return Ok(binary_expression);
            }
            TokenKind::UnaryOperator => {
                // parse unary expression
                return Ok(Node::EmptyNode);
            }
            _ => {
                return Err(ParseError::UnexpectedToken);
            }
        }

        // parse expression
        Ok(node)
    }

    fn parse_binary_expression(&mut self) -> Result<Node, ParseError> {
        let left_node = Node::Identifier {
            value: self.tokens[self.position].value.clone(),
        };

        self.advance();

        let operator = self.tokens[self.position].kind.clone();

        self.advance();

        let right_node = Node::Identifier {
            value: self.tokens[self.position].value.clone(),
        };

        let binary_expression = Node::BinaryExpression {
            left: Box::new(left_node),
            operator,
            right: Box::new(right_node),
        };
        println!("Binary Expression: {:?}", binary_expression);

        // TODO: add an "expect" function to check if the next token is a RightParenthesis
        Ok(binary_expression)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let input = "(A -> B)";
        let mut parser = Parser::new(input.to_string());
        let res = parser.parse();
        println!("{:?}", res);
        // check if the result is Ok

        let x = res.unwrap();
        println!("{:?}", x);
    }

    #[test]
    fn parse_unexpected_token() {
        let input = "(A -> B) (C -> D)";
        let mut parser = Parser::new(input.to_string());
        let res = parser.parse();
        println!("{:?}", res);
    }
}