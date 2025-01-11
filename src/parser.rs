use std::cmp::PartialEq;
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
use crate::lang_lexer::LangLexer;
use crate::lexer::TokenKind::{ForAll, Identifier, RightParenthesis};

use crate::parser::ParseError::{EmptyNode};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken,
    EmptyNode, // this is when there is no node to parse
    AccessOutOfBoundsToken { position: usize, total_tokens: usize },
    UnhandledBehaviour,
    UnclosedParenthesis,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken => write!(f, "Unexpected token"),
            EmptyNode => write!(f, "Empty node"),
            ParseError::AccessOutOfBoundsToken { position, total_tokens } => write!(f, "Access out of bounds token at position {} with total tokens {}", position, total_tokens),
            ParseError::UnhandledBehaviour => write!(f, "Unhandled behaviour"),
            ParseError::UnclosedParenthesis => write!(f, "Unclosed parenthesis"),
        }
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug, Clone)]
#[derive(PartialEq)]
pub enum Value {
    Int(i32),
    Float(f64),
    Str(String),
    // Add other types as needed
}

impl Value {
    pub fn from_string(s: String) -> Self {
        if let Ok(i) = s.parse::<i32>() {
            return Value::Int(i);
        } else if let Ok(f) = s.parse::<f64>() {
            return Value::Float(f);
        }
        Value::Str(s)
    }
}

impl fmt::Display for Value {
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Int (i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::Str(s) => write!(f, "{}", s),
        }
    }
}

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
    Identity {
        name: String,
        value: Box<Node>,
        kind: String,
    },
    Atomic {
        value: Value,
    },
    Call {
        name: String,
        arguments: Vec<Token>,
        returns: Vec<Node>,
    },
    MMExpression {
      expression: String,  
    },
    Type {
        name: String,
    },
    EmptyNode,
}

impl Node {

    pub fn operator(&self) -> TokenKind {
        match self {
            Node::BinaryExpression { left: _left, operator, right: _right } => {
                operator.clone()
            }
            Node::UnaryExpression { operator, right: _right } => {
                operator.clone()
            }
            Node::Identifier { value: _value } => {
                Identifier
            }
            Node::MMExpression { expression: _expression } => {
                TokenKind::MMExpression
            }
            Node::Identity { name: _name, value: _value, kind: _kind } => {
                TokenKind::Word
            }
            _ => {
                println!("{:?}", self);
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
            Node::UnaryExpression { right, .. } => Some(right),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Node::BinaryExpression { left, operator, right } => {
                if *operator == ForAll {
                    format!("∀{}{}", left.to_string(), right.to_string())
                } else {
                    format!("({} {} {})", left.to_string(), operator, right.to_string())
                }
            }
            Node::UnaryExpression { operator, right } => {
                format!("({} {})", operator, right.to_string())
            }
            Node::Identifier { value } => {
                value.clone()
            }
            Node::Identity { name, value, kind } => {
                format!("{} : {} = {:?}", name, kind, value)
            }
            Node::Atomic { value } => {
                value.to_string()
            }
            Node::Call { name, arguments, returns: _returns } => {
                format!("{}({:?})", name, arguments)
            }
            Node::MMExpression { expression } => {
                expression.clone()
            }
            Node::Type { name } => {
                name.clone()
            }
            Node::EmptyNode => {
                "".to_string()
            }
        }
    }

    pub fn val(&self) -> Value {
        match self {
            Node::Atomic { value } => value.clone(),
            _ => Value::Str("".to_string()),
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
        let mut l = LangLexer::new(input.as_str());
        l.tokenize().expect("Failed to tokenize");

        let tokens = l.tokens().clone();
        
        Self {
            position: 0,
            tokens,
        }
    }

    pub fn parse(&mut self) -> Result<Node, ParseError> {
        while self.position < self.tokens.len() {

            match self.tokens[self.position].kind {
                TokenKind::LeftParenthesis | ForAll => {
                    return self.parse_expression()
                }
                RightParenthesis => {
                    // End "parse Expression"
                    println!("End parsing Expression");
                }
                TokenKind::Implies => {
                    println!("Implies");
                }
                Identifier => {
                    // need a bit more robust check to see if there are any "next" operators in the expression
                    // TODO: circle back here, after fixing how the for all block gets parsed

                    if self.peek().kind.is_binary_operator() {
                        let left = self.parse_identifier()?;
                        let operator = self.get_operator()?;
                        self.consume(TokenKind::BinaryOperator)?;

                        let right = self.parse_expression()?;
                        return Ok(Node::BinaryExpression {
                            left: Box::new(left),
                            operator,
                            right: Box::new(right),
                        });
                    }
                    return Ok(Node::Identifier {
                        value: self.current()?.value.clone(),
                    });
                }
                TokenKind::Turnstile => {
                    // For now, not doing anything with the turnstile, so just consume and continue
                    println!("skipping turnstile");
                }
                _ => {
                    if self.current()?.kind.is_unary_operator() {
                        let node = self.parse_unary_expression()?;
                        return Ok(node);
                    } 
                    println!("Unexpected token: {:?}", self.current());
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

    fn consume(&mut self, expect: TokenKind) -> Result<(), ParseError> {
        if expect == TokenKind::BinaryOperator {
            if !self.current()?.kind.is_binary_operator() {
                println!("Expected: {:?}, got: {:?}", expect, self.current());
                return Err(ParseError::UnexpectedToken);
            }
            self.advance();
            return Ok(());
        } else if expect == TokenKind::UnaryOperator {
            if !self.current()?.kind.is_unary_operator() {
                println!("Expected: {:?}, got: {:?}", expect, self.current());
                return Err(ParseError::UnexpectedToken);
            }
            self.advance();
            return Ok(());
        }
        
        if self.current()?.kind != expect {
            println!("Expected: {:?}, got: {:?}", expect, self.current());
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

    fn current(&self) -> Result<Token, ParseError> {
        if self.position < self.tokens.len() {
            Ok(self.tokens[self.position].clone())
        } else {
            Err(ParseError::AccessOutOfBoundsToken {
                position: self.position,
                total_tokens: self.tokens.len(),
            })
        }
    }

    fn parse_expression(&mut self) -> Result<Node, ParseError> {
        // This needs some new logic to properly parse any potential unary operators that may be included as a sub expression of a binary expression

        match self.current()?.kind {
            TokenKind::LeftParenthesis => {
                self.consume(TokenKind::LeftParenthesis)?;

                // this could happen by parsing for a "left" expression
                // that expression could be either unary, binary, or simply an identifier

                let left = self.parse_expression()?;

                // println!("left: {:?}", left);

                if self.position >= self.tokens.len() {
                    return Ok(left);
                }

                if self.current()?.kind.is_binary_operator() {
                    let operator = self.get_operator()?;
                    self.consume(TokenKind::BinaryOperator)?;

                    let right = self.parse_expression()?;
                    // self.consume(RightParenthesis)?;

                    Ok(Node::BinaryExpression {
                        left: Box::new(left),
                        operator,
                        right: Box::new(right),
                    })
                } else if self.peek().kind == RightParenthesis {
                    self.consume(RightParenthesis)?;
                    return Ok(left);
                } else {
                    return Ok(left);
                }
            }
            Identifier => {
                let ident = self.parse_identifier()?;
                if self.position >= self.tokens.len() {
                    return Ok(ident);
                }

                if self.current()?.kind == RightParenthesis {
                    self.consume(RightParenthesis)?;
                    return Ok(ident)
                }

                // peek and check if the next token is a binary operator, if yes, parse as binary node
                if self.current()?.kind.is_binary_operator() {
                    let operator = self.get_operator()?;
                    self.consume(TokenKind::BinaryOperator)?;

                    if operator == TokenKind::Equality {
                        let right = self.parse_identifier()?;
                        return Ok(Node::BinaryExpression {
                            left: Box::new(ident),
                            operator,
                            right: Box::new(right),
                        });
                    }

                    let right = self.parse_expression()?;
                    return Ok(Node::BinaryExpression {
                        left: Box::new(ident),
                        operator,
                        right: Box::new(right),
                    });
                }

                Ok(ident)
            }
            TokenKind::UnaryOperator => {
                let node = self.parse_unary_expression()?;

                if self.peek().kind.is_binary_operator() {
                    let operator = self.get_operator()?;
                    self.consume(TokenKind::BinaryOperator)?;
                    let right = self.parse_expression()?;
                    return Ok(Node::BinaryExpression {
                        left: Box::new(node),
                        operator,
                        right: Box::new(right),
                    });
                }
                Ok(node)
            }
            ForAll => {

                let operator = self.get_operator()?;
                self.consume(ForAll)?;

                let first = self.parse_identifier()?;

                if self.current()?.kind == TokenKind::LeftParenthesis || self.current()?.kind.is_unary_operator() || self.current()?.kind == ForAll{
                    let second = self.parse_expression()?;
                    return Ok(Node::BinaryExpression {
                        left: Box::new(first),
                        operator,
                        right: Box::new(second),
                    });
                }

                let second = self.parse_identifier()?;

                Ok(Node::BinaryExpression {
                    left: Box::new(first),
                    operator,
                    right: Box::new(second),
                })
            }
            _ => {
                if self.current()?.kind.is_unary_operator() {
                    let node = self.parse_unary_expression()?;
                    return Ok(node);
                }
                println!("Unexpected token: {:?}", self.current());
                Err(ParseError::UnexpectedToken)
            }
        }
}
    
    fn is_unary_expression(&self) -> bool {
        if let Ok(current) = self.current() {
            return current.kind.is_unary_operator()
        }
        false
    }

    fn is_binary_expression(&self) -> bool {
        if let Ok(current) = self.current() {
            current.kind == Identifier && self.peek().kind.is_binary_operator()
        } else {
            false
        }
    }

    fn parse_identifier(&mut self) -> Result<Node, ParseError> {
        if self.current()?.kind != Identifier {
            println!("Unexpected token: {:?}", self.current());
            return Err(ParseError::UnexpectedToken);
        }

        let identifier = Node::Identifier {
            value: self.current()?.value.clone(),
        };

        self.consume(Identifier)?;

        Ok(identifier)
    }
    
    fn parse_unary_expression(&mut self) -> Result<Node, ParseError> {
        if !self.current()?.kind.is_unary_operator() {
            println!("Unexpected token: {:?}", self.current());
            return Err(ParseError::UnexpectedToken);
        }

        let operator = self.get_operator()?;
        
        self.consume(TokenKind::UnaryOperator)?;
        
        let right = self.parse_expression()?;
        
        Ok(Node::UnaryExpression {
            operator,
            right: Box::new(right),
        })
    }

    fn parse_binary_expression(&mut self) -> Result<Node, ParseError> {
        if self.current()?.kind != Identifier {
            panic!("this needs to be parsed as an expression");
            return Err(ParseError::UnexpectedToken);
        }
        let left_node = Node::Identifier {
            value: self.current()?.value.clone(),
        };
        self.consume(Identifier)?;

        let operator = self.get_operator()?;
        
        self.consume(TokenKind::BinaryOperator)?;

        if self.position >= self.tokens.len() {
            return         Err(ParseError::AccessOutOfBoundsToken {
                position: self.position,
                total_tokens: self.tokens.len(),
            });
        }
        
        if self.current()?.kind == TokenKind::LeftParenthesis {
            return Ok(Node::BinaryExpression {
                left: Box::new(left_node),
                operator,
                right: Box::new(self.parse_expression()?),
            });
        }
        
        let right_node = Node::Identifier {
            value: self.current()?.value.clone(),
        };
        
        if self.current()?.kind.is_unary_operator() {
            return Ok(Node::BinaryExpression {
                left: Box::new(left_node),
                operator,
                right: Box::new(self.parse_unary_expression()?),
            });    
        }
        
        self.consume(Identifier)?;

        let binary_expression = Node::BinaryExpression {
            left: Box::new(left_node),
            operator,
            right: Box::new(right_node),
        };

        Ok(binary_expression)
    }

    fn get_operator(&self) -> Result<TokenKind, ParseError> {
        match self.current() {
            Ok(token) => Ok(token.kind.clone()),
            Err(e) => {
                println!("Error retrieving current token: {:?}", e);
                Err(e)
            }
        }
    }

    pub fn dump_state(&mut self) {
        println!("Position: {}", self.position);
        println!("Tokens: {:?}", self.tokens);
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