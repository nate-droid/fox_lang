use std::cmp::PartialEq;
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
    AccessOutOfBoundsToken,
    UnhandledBehaviour,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken => write!(f, "Unexpected token"),
            ParseError::EmptyNode => write!(f, "Empty node"),
            ParseError::AccessOutOfBoundsToken => write!(f, "Access out of bounds token"),
            ParseError::UnhandledBehaviour => write!(f, "Unhandled behaviour"),
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
                    println!("expr!! {:?}", expr);
                    println!("current token: {:?}", self.current());

                    // TODO: Make sure that all expressions consume the corresponding right and left parenthesis

                    if self.current().kind == TokenKind::LeftParenthesis {
                        self.advance();

                        let right = self.parse_expression()?;

                        println!("expr {:?}", expr);

                        let binary_expression = Node::BinaryExpression {
                            left: Box::new(expr),
                            operator: TokenKind::Implies,
                            right: Box::new(right),
                        };
                        return Ok(binary_expression);
                    }

                    self.advance();

                    if self.current().kind != TokenKind::RightParenthesis {
                        println!("Unexpected token: {:?}", self.current());
                        return Err(ParseError::UnexpectedToken);
                    }

                    self.advance();

                    if self.position >= self.tokens.len() {
                        // Reached the end of input
                        return Ok(expr);
                    }

                    // TODO: if the current token is an Operator, parse the right hand side of the expression
                    if self.current().kind == TokenKind::Implies {
                        // TODO: Write a function that checks a list of "operator" tokens
                        let left = expr.clone();
                        let operator = self.tokens[self.position].kind.clone();
                        self.advance();

                        let right = self.parse_expression()?;

                        let binary_expression = Node::BinaryExpression {
                            left: Box::new(left),
                            operator,
                            right: Box::new(right),
                        };

                        return Ok(binary_expression);
                    }
                }
                TokenKind::RightParenthesis => {
                    // End "parse Expression"
                    println!("End parsing Expression");
                }
                TokenKind::Implies => {
                    println!("Implies");
                    // TODO Parse the right hand side of the expression
                }
                TokenKind::Identifier => {
                    println!("Identifier");
                }
                _ => {
                    return Err(ParseError::UnexpectedToken);
                }
            }
            self.advance();

        }

        Err(EmptyNode)
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn peek(&self) -> Token {
        if self.position + 1 >= self.tokens.len() {
            return Token {
                value: "".to_string(),
                kind: TokenKind::End,
            };
        }
        self.tokens[self.position+1].clone()
    }

    fn current(&self) -> Token {
        self.tokens[self.position].clone()
    }

    fn parse_expression(&mut self) -> Result<Node, ParseError> {
        let mut node = Node::EmptyNode;
        match self.tokens[self.position].kind {
            TokenKind::LeftParenthesis => {
                // this is a sub expression
                self.advance();
                let sub_expression = self.parse_expression()?;
                node = sub_expression;

                self.advance();

                return Ok(node);
            }
            TokenKind::Identifier => {
                if self.peek().kind == TokenKind::RightParenthesis {
                    let identifier = Node::Identifier {
                        value: self.tokens[self.position].value.clone(),
                    };
                    return Ok(identifier);
                } else if self.peek().kind == TokenKind::Implies { // TODO add more operators
                    let left = Node::Identifier {
                        value: self.tokens[self.position].value.clone(),
                    };

                    self.advance();
                    let operator = self.tokens[self.position].kind.clone();
                    self.advance();

                    if self.tokens[self.position].kind == TokenKind::LeftParenthesis {
                        let right = self.parse_expression()?;

                        return Ok(Node::BinaryExpression {
                            left: Box::new(left),
                            operator,
                            right: Box::new(right),
                        });
                    }

                    let right = Node::Identifier {
                        value: self.tokens[self.position].value.clone(),
                    };

                    let binary_expression = Node::BinaryExpression {
                        left: Box::new(left),
                        operator,
                        right: Box::new(right),
                    };

                    return Ok(binary_expression);
                } else {
                    println!("Identifier: {:?}", self.tokens[self.position]);
                    println!("Peek: {:?}", self.peek());
                    return Err(ParseError::UnhandledBehaviour);
                }
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

        if self.position >= self.tokens.len() {
            return Err(ParseError::AccessOutOfBoundsToken);
        }

        let right_node = Node::Identifier {
            value: self.tokens[self.position].value.clone(),
        };

        let binary_expression = Node::BinaryExpression {
            left: Box::new(left_node),
            operator,
            right: Box::new(right_node),
        };

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

        // Assert that the result is Ok
        assert!(res.is_ok());

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