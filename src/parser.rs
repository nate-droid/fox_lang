use std::fmt::Error;
use crate::lexer::{Token, TokenKind};
use crate::lexer::Lexer;

struct Parser {
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

#[derive(Debug)]
enum Node {
    BinaryExpression {
        left: Box<Node>,
        operator: TokenKind,
        right: Box<Node>,
    },
    UnaryExpression {
        operator: String,
        right: Box<Node>,
    },
    Identifier {
        value: String,
    },
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
        }
    }
}

impl Parser {
    fn new(input: String) -> Self {
        let mut lexer = Lexer::new(input);
        lexer.tokenize();

        let tokens = lexer.tokens.clone();

        Self {
            lexer,
            position: 0,
            tokens,
        }
    }

    fn parse(&mut self) -> Result<Node, ParseError> {
        while self.position < self.lexer.tokens.len() {

            match self.tokens[self.position].kind {
                TokenKind::LeftParenthesis => {

                    self.advance();

                    match self.tokens[self.position].kind {
                        TokenKind::Identifier => {

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
                            return Ok(binary_expression);
                        }
                        TokenKind::UnaryOperator => {
                            println!("Unary Operator");
                        }
                        _ => {
                            Err(ParseError::UnexpectedToken)?;
                        }
                    }
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