use crate::lang_ast::Ast;
use crate::lang_lexer::LangLexer;
use crate::lexer::{Token, TokenKind};
use crate::parser::{Node, Value};

struct LangParser {
    lexer: LangLexer,
    tokens: Vec<Token>,
    position: usize,
}

impl LangParser {
    fn new(input: String) -> Self {
        let mut lexer = LangLexer::new(input);
        lexer.tokenize();

        let tokens = lexer.tokens().clone();
        
        Self {
            lexer,
            tokens,
            position: 0,
        }
    }
    
    fn parse(&mut self) -> Result<Ast, String> {
        let mut globals = Vec::new();
        let mut ast = Ast::new();
        
        while self.position < self.tokens.len() {
            match self.tokens[self.position].kind {
                TokenKind::Print => {
                    self.consume(TokenKind::Print)?;
                    self.consume(TokenKind::LeftParenthesis)?;
                    println!("{:?}", self.tokens[self.position].value);
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
                    
                    // TODO: Add a "fetch value and consume" function
                    let left = self.parse_node()?;
                    
                    // TODO: Will need a more robust way to handle expressions in the future
                    if self.current_token()?.kind == TokenKind::Add {
                        self.consume(TokenKind::Add)?;
                        
                        // ensure that left is a number
                        let val = match left {
                            Node::Atomic { value } => value,
                            _ => return Err("Unexpected token".to_string()),
                        };
                        
                        // fetch the right side and ensure it is a number
                        let right = self.parse_node()?;
                        let val2 = match right {
                            Node::Atomic { value } => value,
                            _ => return Err("Unexpected token".to_string()),
                        };
                        
                        let n = Node::BinaryExpression {
                            left: Box::from(Node::Atomic { value: val }),
                            operator: TokenKind::Add,
                            right: Box::from(Node::Atomic { value: val2 }),
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
                _ => {
                    println!("{:?}", self.tokens[self.position].kind);
                    println!("{:?}", self.tokens[self.position].value);
                    return Err("Unexpected token".to_string());
                }
            }
            self.advance();
        }

        dbg!(globals);

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
            _ => {
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
        
        let mut parser = LangParser::new(input.to_string());
        
        let ast = parser.parse().expect("TODO: panic message");
        println!("{:?}", ast);
    }
    
    #[test]
    fn variables() {
        let input = "let x : Nat = 10;";
        
        let mut parser = LangParser::new(input.to_string());
        
        let ast = parser.parse().expect("TODO: panic message");
        println!("{:?}", ast);
    }
    
    #[test]
    fn multi_line_variables() {
        let input = "let x : Nat = 1;\
        let y : Nat = 2;";
        let mut parser = LangParser::new(input.to_string());
        println!("{:?}", parser.tokens);
        let ast = parser.parse().expect("unexpected failure");
        println!("{:?}", ast);
    }
    
    #[test]
    fn addition() {
        let input = "let x : Nat = 1 + 2;";
        let mut parser = LangParser::new(input.to_string());
        let ast = parser.parse().expect("unexpected failure");
        println!("{:?}", ast);
        // todo!("Implement addition");
    }
    
    #[test]
    fn error_handling() {
        // add checks that the parser returns an error when it should
        // for example, having an expression not terminate with a semicolon
    }
}