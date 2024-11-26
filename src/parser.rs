use std::cmp::PartialEq;
use std::fmt::Error;
use crate::lexer::{Token, TokenKind};
use crate::lexer::DefaultLexer;

pub struct Parser {
    // lexer: L,
    position: usize,
    tokens: Vec<Token>,
}

pub trait Lexer {
    fn tokenize(&mut self);
    fn current_token(&self) -> Token;
    // fn new(input: String) -> Self;
    fn next_char(&mut self);
    fn peek(&self) -> char;

    fn tokens(&self) -> Vec<Token>;
}

use std::fmt;
use crate::lexer::TokenKind::{Identifier, RightParenthesis};
use crate::metamath_lexer::MetaMathLexer;
// import MetaMathLexer

use crate::parser::ParseError::{EmptyNode, UnhandledBehaviour};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken,
    EmptyNode, // this is when there is no node to parse
    AccessOutOfBoundsToken,
    UnhandledBehaviour,
    UnclosedParenthesis,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken => write!(f, "Unexpected token"),
            ParseError::EmptyNode => write!(f, "Empty node"),
            ParseError::AccessOutOfBoundsToken => write!(f, "Access out of bounds token"),
            ParseError::UnhandledBehaviour => write!(f, "Unhandled behaviour"),
            ParseError::UnclosedParenthesis => write!(f, "Unclosed parenthesis"),
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

    pub fn to_string(&self) -> String {
        // TODO: Add nested expressions
        match self {
            Node::BinaryExpression { left, operator, right } => {
                format!("({} {} {})", left.to_string(), operator, right.to_string())
                // let left_str = match **left {
                //     Node::BinaryExpression { .. } => format!("({})", left.to_string()),
                //     _ => left.to_string(),
                // };
                // let right_str = match **right {
                //     Node::BinaryExpression { .. } => format!("({})", right.to_string()),
                //     _ => right.to_string(),
                // };
                // format!("{} {} {}", left_str, operator, right_str)
            }
            Node::UnaryExpression { operator, right } => {
                // format!("({} {})", operator, right.to_string())
                let right_str = match **right {
                    Node::BinaryExpression { .. } | Node::UnaryExpression { .. } => format!("({})", right.to_string()),
                    _ => right.to_string(),
                };
                format!("{} {}", operator, right_str)
            }
            Node::Identifier { value } => {
                value.clone()
            }
            Node::EmptyNode => {
                "".to_string()
            }
        }
    }
}

impl Parser {
    pub fn new(input: String) -> Self {

        let mut lexer = DefaultLexer::new(input);
        lexer.tokenize();

        let tokens = lexer.tokens().clone();

        Self {
            position: 0,
            tokens,
        }
    }

    pub fn new_mm(input: String) -> Self {
        let mut l = MetaMathLexer::new(input.clone());
        l.tokenize();

        let tokens = l.tokens().clone();

        Self {
            position: 0,
            tokens,
        }
    }

    pub fn parse(&mut self) -> Result<Node, ParseError> {
        while self.position < self.tokens.len() {

            match self.tokens[self.position].kind {
                TokenKind::LeftParenthesis => {

                    let expr = self.parse_expression()?;

                    return Ok(expr);

                    if self.current().kind.is_binary_operator() {
                        panic!("hi");
                        let operator = self.current().kind.clone();
                        println!("4current: {:?}", self.current());
                        self.advance();
                        println!("4current: {:?}", self.current());
                        if self.current().kind == Identifier {

                            return Ok(Node::BinaryExpression {
                                left: Box::new(expr),
                                operator,
                                right: Box::new(self.parse_identifier()?),
                            });
                        }

                        let right = self.parse_expression()?;
                        return Ok(Node::BinaryExpression {
                            left: Box::new(expr),
                            operator,
                            right: Box::new(right),
                        });
                    }

                    return Ok(expr);
                }
                RightParenthesis => {
                    // End "parse Expression"
                    println!("End parsing Expression");
                }
                TokenKind::Implies => {
                    println!("Implies");
                }
                Identifier => {

                    return Ok(Node::Identifier {
                        value: self.current().value.clone(),
                    });
                }
                TokenKind::Turnstile => {
                    // For now, not doing anything with the turnstile, so just consume and continue
                    println!("skipping turnstile");
                }
                _ => {
                    println!("Unexpected token: {:?}", self.current());
                    return Err(ParseError::UnexpectedToken);
                }
            }
            self.advance();

        }

        println!("initial position: {:?}", self.tokens);
        Err(EmptyNode)
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn consume(&mut self, expect: TokenKind) -> Result<(), ParseError> {
        if self.current().kind != expect {
            return Err(ParseError::UnexpectedToken);
        }
        self.advance();
        Ok(())
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
        // TODO: I want to have this expression consume the entire expression

        match self.current().kind {
            TokenKind::LeftParenthesis => {
                self.consume(TokenKind::LeftParenthesis)?;

                let mut left = Node::EmptyNode;
                if self.is_binary_expression() {
                    left = self.parse_expression()?;
                    // if !self.peek().kind.is_binary_operator() {
                    if self.position >= self.tokens.len() || !self.current().kind.is_binary_operator() {
                        // println!("current: {:?}", self.current());
                        // println!("peek: {:?}", self.peek());
                        return Ok(left);
                    }

                    let operator = self.current().kind.clone();
                    self.advance();
                    println!("5current: {:?}", self.current());
                    let right = self.parse_expression()?;
                    return Ok(Node::BinaryExpression {
                        left: Box::new(left),
                        operator,
                        right: Box::new(right),
                    });
                    // TODO: this needs a check to see if there is anything else in the expression: ((A -> B) -> C) for example
                } else if self.is_identifier() {

                    let identifier = self.parse_identifier()?;
                    if self.current().kind.is_binary_operator() {
                        panic!("this should be parsed as a binary expression");
                    }
                    if !self.peek().kind.is_binary_operator() {
                        return Ok(self.parse_identifier()?);
                    }

                    self.advance();
                    let operator = self.current().kind.clone();
                    let right = self.parse_expression()?;
                    return Ok(Node::BinaryExpression {
                        left: Box::new(self.parse_identifier()?),
                        operator,
                        right: Box::new(right),
                    });
                    // return self.parse_identifier();
                }

                // this is a sub expression
                let sub_expression = self.parse_expression()?;
                
                if self.position >= self.tokens.len() {
                    return Ok(sub_expression);
                }
                
                if self.current().kind == RightParenthesis {
                    self.advance();
                    return Ok(sub_expression);
                }

                // self.advance();

                if self.current().kind.is_binary_operator() {
                    return Ok(sub_expression);
                } else if self.current().kind == TokenKind::RightParenthesis && self.peek().kind.is_binary_operator() {

                    self.consume(TokenKind::RightParenthesis)?;

                    if !self.current().kind.is_binary_operator() {
                        return Err(ParseError::UnexpectedToken)
                    }

                    let operator = self.current().kind.clone();
                    self.advance();

                    if self.current().kind == TokenKind::LeftParenthesis {
                        // TODO: temp, clean this up once all works
                        self.consume(TokenKind::LeftParenthesis)?;
                        let temp = self.parse_expression()?;
                        println!("temp: {:?}", temp);
                        println!("current: {:?}", self.current());
                        return Ok(Node::BinaryExpression {
                            left: Box::new(sub_expression),
                            operator,
                            right: Box::new(temp),
                        });
                    }

                    let right = self.parse_expression()?;

                    self.consume(TokenKind::RightParenthesis)?;

                    return Ok(Node::BinaryExpression {
                        left: Box::new(sub_expression),
                        operator,
                        right: Box::new(right),
                    });
                }
                // TODO: Add a check if the current token is a ) AND if it needs more parsing like a binary expression

                Ok(sub_expression)
            }
            Identifier => {
                if self.peek().kind == RightParenthesis {
                    let ident = self.parse_identifier()?;

                    self.consume(RightParenthesis)?;

                    Ok(ident)
                } else if self.peek().kind.is_binary_operator() {
                    let expression = self.parse_binary_expression()?;

                    if self.current().kind == TokenKind::RightParenthesis {
                        // close off expression
                        self.consume(RightParenthesis)?;
                    }

                    return Ok(expression);
                } else {
                    println!("Identifier: {:?}", self.current());
                    println!("Peek: {:?}", self.peek());
                    return Err(ParseError::UnhandledBehaviour);
                }
            }
            TokenKind::UnaryOperator => {
                // parse unary expression
                Ok(Node::EmptyNode)
            }
            _ => {
                println!("Unexpected token: {:?}", self.current());
                Err(ParseError::UnexpectedToken)
            }
        }
    }

    fn is_binary_expression(&self) -> bool {
        self.current().kind == Identifier && self.peek().kind.is_binary_operator()
    }

    fn is_identifier(&self) -> bool {
        self.current().kind == Identifier
    }

    fn parse_identifier(&mut self) -> Result<Node, ParseError> {
        if self.current().kind != Identifier {
            return Err(ParseError::UnexpectedToken);
        }

        let identifier = Node::Identifier {
            value: self.current().value.clone(),
        };

        self.consume(Identifier)?;

        Ok(identifier)
    }

    fn parse_binary_expression(&mut self) -> Result<Node, ParseError> {
        if self.current().kind != TokenKind::Identifier {
            panic!("this needs to be parsed as an expression");
            return Err(ParseError::UnexpectedToken);
        }
        let left_node = Node::Identifier {
            value: self.current().value.clone(),
        };
        self.consume(Identifier)?;
        
        let operator = self.current().kind.clone();
        self.advance();

        if self.position >= self.tokens.len() {
            return Err(ParseError::AccessOutOfBoundsToken);
        }
        
        if self.current().kind == TokenKind::LeftParenthesis {
            return Ok(Node::BinaryExpression {
                left: Box::new(left_node),
                operator,
                right: Box::new(self.parse_expression()?),
            });
        }
        
        let right_node = Node::Identifier {
            value: self.current().value.clone(),
        };
        self.consume(Identifier)?;

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
        assert!(res.is_err());
    }

    #[test]
    fn test_nested_balanced() {
        let input = "((A -> B) -> (C -> D))";

        let mut parser = Parser::new(input.to_string());
        let res = parser.parse();
        println!("{:?}", res);
        assert!(res.is_ok());

        let x = res.unwrap();

        if let Node::BinaryExpression {left, operator, right} = x {
            assert_eq!(operator, TokenKind::Implies);

            if let Node::BinaryExpression {left: left_left, operator: left_operator, right: left_right} = *left {
                assert_eq!(left_operator, TokenKind::Implies);

                if let Node::Identifier {value} = *left_left {
                    assert_eq!(value, "A");
                } else {
                    panic!("Expected an identifier");
                }

                if let Node::Identifier {value} = *left_right {
                    assert_eq!(value, "B");
                } else {
                    panic!("Expected an identifier");
                }
            } else {
                panic!("Expected a binary expression");
            }

            if let Node::BinaryExpression {left: right_left, operator: right_operator, right: right_right} = *right {
                assert_eq!(right_operator, TokenKind::Implies);

                if let Node::Identifier {value} = *right_left {
                    assert_eq!(value, "C");
                } else {
                    panic!("Expected an identifier");
                }

                if let Node::Identifier {value} = *right_right {
                    assert_eq!(value, "D");
                } else {
                    panic!("Expected an identifier");
                }
            } else {
                panic!("Expected a binary expression");
            }
        }

    }

    #[test]
    fn test_sub_left() {
        let input = "((A -> C) -> B)";

        let mut parser = Parser::new(input.to_string());
        let res = parser.parse();

        let x = res.unwrap();
        println!("{:?}", x);

        if let Node::BinaryExpression {left, operator, right} = x {
            assert_eq!(operator, TokenKind::Implies);

            if let Node::BinaryExpression {left: left_left, operator: left_operator, right: left_right} = *left {
                assert_eq!(left_operator, TokenKind::Implies);

                if let Node::Identifier {value} = *left_left {
                    assert_eq!(value, "A");
                } else {
                    panic!("Expected an identifier");
                }

                if let Node::Identifier {value} = *left_right {
                    assert_eq!(value, "C");
                } else {
                    panic!("Expected an identifier");
                }
            } else {
                panic!("Expected a binary expression");
            }

            if let Node::Identifier {value} = *right {
                assert_eq!(value, "B");
            } else {
                panic!("Expected an identifier");
            }
        }
    }

    #[test]
    fn test_sub_right() {
        let input = "(A -> (B -> C))";

        let mut parser = Parser::new(input.to_string());
        let res = parser.parse();

        let x = res.unwrap();

        if let Node::BinaryExpression {left, operator, right} = x {
            assert_eq!(operator, TokenKind::Implies);

            if let Node::Identifier {value} = *left {
                assert_eq!(value, "A");
            } else {
                panic!("Expected an identifier");
            }

            if let Node::BinaryExpression {left: right_left, operator: right_operator, right: right_right} = *right {
                assert_eq!(right_operator, TokenKind::Implies);

                if let Node::Identifier {value} = *right_left {
                    assert_eq!(value, "B");
                } else {
                    panic!("Expected an identifier");
                }

                if let Node::Identifier {value} = *right_right {
                    assert_eq!(value, "C");
                } else {
                    panic!("Expected an identifier");
                }
            } else {
                panic!("Expected a binary expression");
            }
        }
    }
}