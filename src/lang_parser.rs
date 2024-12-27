use crate::lang_lexer::LangLexer;
use crate::lexer::{Token, TokenKind};

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
    
    fn parse(&mut self) -> Result<(), String> {
        let mut globals = Vec::new();
        while self.position < self.tokens.len() {
            match self.tokens[self.position].kind {
                TokenKind::Print => {
                    self.consume(TokenKind::Print)?;
                    self.consume(TokenKind::LeftParenthesis)?;
                    println!("{:?}", self.tokens[self.position].value);
                    self.advance();
                    self.consume(TokenKind::RightParenthesis)?;
                    self.consume(TokenKind::Semicolon)?;
                }
                TokenKind::Let => {
                    self.consume(TokenKind::Let)?;
                    globals.push(self.current_token()?);
                    self.consume(TokenKind::Word)?;
                    self.consume(TokenKind::Colon)?;
                    self.consume(TokenKind::Nat)?;
                    self.consume(TokenKind::Assign)?;
                    println!("{:?}", self.tokens[self.position].value);
                    self.advance();
                    self.consume(TokenKind::Semicolon)?;
                }
                _ => {
                    println!("{:?}", self.tokens[self.position].kind);
                    return Err("Unexpected token".to_string());
                }
            }
            self.advance();
        }

        dbg!(globals);

        // test by printing a variable
        // test by implementing addition and subtraction

        // TODO: Have this return a Node or vec of Nodes

        Ok(())
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
}

#[cfg(test)]
mod tests {
    use crate::lang_lexer::LangLexer;
    use crate::lexer::TokenKind;
    use super::*;

    #[test]
    fn print() {
        let input = "print(\"hello world!\");";
        
        let mut parser = LangParser::new(input.to_string());
        
        parser.parse().expect("TODO: panic message");
    }
    
    #[test]
    fn variables() {
        let input = "let x : Nat = 10;";
        
        let mut parser = LangParser::new(input.to_string());
        
        parser.parse().expect("TODO: panic message");
        
    }
}