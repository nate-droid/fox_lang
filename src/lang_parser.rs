use crate::lang_ast::Ast;
use crate::lang_lexer::LangLexer;
use crate::lexer::{Token, TokenKind};
use crate::parser::{Node, Value};

pub(crate) struct LangParser<'a> {
    lexer: LangLexer<'a>,
    tokens: Vec<Token>,
    position: usize,
}

impl<'a> LangParser<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        let mut lexer = LangLexer::new(input);
        lexer.tokenize();

        let tokens = lexer.tokens().clone();
        
        Self {
            lexer,
            tokens,
            position: 0,
        }
    }
    
    pub(crate) fn parse(&mut self) -> Result<Ast, String> {
        let mut globals = Vec::new();
        let mut ast = Ast::new();
        
        while self.position < self.tokens.len() {
            match self.tokens[self.position].kind {
                TokenKind::Print => {
                    self.consume(TokenKind::Print)?;
                    self.consume(TokenKind::LeftParenthesis)?;

                    let input = self.current_token()?;
                    
                    self.advance();
                    self.consume(TokenKind::RightParenthesis)?;
                    self.consume(TokenKind::Semicolon)?;
                    
                    let node = Node::Call {
                        name: "print".to_string(),
                        arguments: vec![input],
                        returns: vec![],
                    };
                    ast.add_node(node);
                }
                TokenKind::Let => {
                    self.consume(TokenKind::Let)?;
                    globals.push(self.current_token()?);

                    let name = self.current_token()?;

                    self.consume(TokenKind::Word)?;
                    self.consume(TokenKind::Colon)?;

                    // TODO: write a function to grab "kind" from the tokens
                    let kind = self.current_token()?;

                    self.consume(TokenKind::Nat)?;

                    self.consume(TokenKind::Assign)?;
                    
                    let left = self.parse_node()?;

                    // TODO: Will need a more robust way to handle expressions in the future
                    if self.current_token()?.kind == TokenKind::Add {
                        self.consume(TokenKind::Add)?;
                        
                        let right = self.parse_node()?;

                        let n = Node::BinaryExpression {
                            left: Box::from(left),
                            operator: TokenKind::Add,
                            right: Box::from(right),
                        };

                        let ident = Node::Identity {
                            name: name.value.to_string(),
                            value: Box::from(n),
                            kind: kind.value,
                        };

                        ast.add_node(ident);
                        self.consume(TokenKind::Semicolon)?;
                        continue;
                    }

                    let ident = Node::Identity {
                        name: name.value.to_string(),
                        value: Box::from(left),
                        kind: kind.value,
                    };

                    ast.add_node(ident);
                    self.consume(TokenKind::Semicolon)?;
                    continue;
                }
                TokenKind::Comment => {
                    // skip comments
                    println!("Skipping comment");
                }
                _ => {
                    println!("{:?}", self.tokens[self.position].kind);
                    println!("{:?}", self.tokens[self.position].value);
                    return Err("Unexpected token".to_string());
                }
            }
            self.advance();
        }

        // dbg!(globals);

        Ok(ast)
    }

    fn parse_node(&mut self) -> Result<Node, String> {
        match self.current_token()?.kind {
            TokenKind::Number => {
                let val = Value::from_string(self.current_token()?.value.clone());
                self.advance();
                Ok(Node::Atomic {
                    value: val,
                })
            }
            TokenKind::Word => {
                let name = self.current_token()?;
                self.advance();
                Ok(Node::Identity {
                    name: name.value.clone(),
                    value: Box::from(Node::Atomic { value: Value::Int(0) }),
                    kind: "Nat".to_string(),
                })
            }
            _ => {
                println!("{:?}", self.current_token()?.kind);
                Err("Unexpected token".to_string())
            }
        }
    }

    fn consume(&mut self, kind: TokenKind) -> Result<(), String> {
        if self.tokens[self.position].kind == kind {
            self.advance();
            Ok(())
        } else {
            println!("{:?}", self.tokens[self.position].kind);
            println!("{:?}", self.tokens);
            Err(format!("Expected {:?} but found {:?}", kind, self.tokens[self.position].kind))
        }
    }
    
    fn advance(&mut self) {
        self.position += 1;
    }
    
    fn current_token(&self) -> Result<Token, String> {
        if self.position < self.tokens.len() {
            Ok(self.tokens[self.position].clone())
        } else {
            Err("No more tokens".to_string())
        }
    }
    
    fn peek_token(&self) -> Result<Token, String> {
        if self.position + 1 < self.tokens.len() {
            Ok(self.tokens[self.position + 1].clone())
        } else {
            Err("No more tokens".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print() {
        let input = "print(\"hello world!\");";
        
        let mut parser = LangParser::new(input);
        
        let ast = parser.parse().expect("TODO: panic message");
        println!("{:?}", ast);
    }
    
    #[test]
    fn ignore_comments() {
        let input = "print(\"hello world!\"); // this is a comment";
        
        let mut parser = LangParser::new(input);
        
        let ast = parser.parse().expect("TODO: panic message");
        println!("{:?}", ast);
    }
    
    #[test]
    fn variables() {
        let input = "let x : Nat = 10;";
        
        let mut parser = LangParser::new(input);
        
        let ast = parser.parse().expect("TODO: panic message");
        println!("{:?}", ast);
    }
    
    #[test]
    fn multi_line_variables() {
        let input = "let x : Nat = 1;\
        let y : Nat = 2;";
        let mut parser = LangParser::new(input);
        println!("{:?}", parser.tokens);
        let ast = parser.parse().expect("unexpected failure");
        println!("{:?}", ast);
    }
    
    #[test]
    fn addition() {
        let input = "let x : Nat = 1 + 2;";
        let mut parser = LangParser::new(input);
        let ast = parser.parse().expect("unexpected failure");
        println!("{:?}", ast);
        
        assert_eq!(ast.nodes.len(), 1);
    }
    
    #[test]
    fn addition_with_variables() {
        let input = "let x : Nat = 1;\
        let y : Nat = 2;\
        let z : Nat = x + y;";
        let mut parser = LangParser::new(input);
        let ast = parser.parse().expect("unexpected failure");
        println!("{:?}", ast);
        
        assert_eq!(ast.nodes.len(), 3);
    }
    
    #[test]
    fn error_handling() {
        // add checks that the parser returns an error when it should
        // for example, having an expression not terminate with a semicolon
    }
}