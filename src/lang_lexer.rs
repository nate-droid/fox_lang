use std::iter::Peekable;
use std::str::Chars;
use crate::lexer::{Token, TokenKind};

pub struct LangLexer<'a> {
    input: &'a str,
    char: Option<char>,
    tokens: Vec<Token>,
    iterator: Peekable<Chars<'a>>,
}

impl <'a> LangLexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut iterator = input.chars().peekable();

        Self {
            tokens: Vec::new(),
            input,
            char: iterator.next(),
            iterator,
        }
    }
    
    pub fn tokenize(&mut self) -> Result<(), String> {
        // while self.position < self.input.len() {
        while self.char.is_some() {
            match self.current_char() {
                ' ' => {}
                '/' => {
                    if self.peek() == '/' {
                        while self.char.is_some() {
                            self.next_char();
                        }
                        self.tokens.push(Token {
                            value: self.current_char().to_string(),
                            kind: TokenKind::Comment,
                        });
                    } else {
                        return Err(format!("Expected '/' but found: {}", self.current_char()));
                    }
                }
                '(' => {
                    self.tokens.push(Token {
                        value: self.current_char().to_string(),
                        kind: TokenKind::LeftParenthesis,
                    });
                }
                ')' => {
                    self.tokens.push(Token {
                        value: self.current_char().to_string(),
                        kind: TokenKind::RightParenthesis,
                    });
                }
                '\"' => {
                    let mut string = String::new();
                    self.next_char();
                    while self.char != Some('\"') {
                        string.push(self.current_char());
                        self.next_char();
                    }
                    self.tokens.push(Token {
                        value: string,
                        kind: TokenKind::String,
                    });
                }
                ';' => {
                    self.tokens.push(Token {
                        value: self.current_char().to_string(),
                        kind: TokenKind::Semicolon,
                    });
                }
                '=' => {
                    self.tokens.push(Token {
                        value: self.current_char().to_string(),
                        kind: TokenKind::Assign,
                    });
                }
                ':' => {
                    self.tokens.push(Token {
                        value: self.current_char().to_string(),
                        kind: TokenKind::Colon,
                    });
                }
                '+' => {
                    self.tokens.push(Token {
                        value: self.current_char().to_string(),
                        kind: TokenKind::Add,
                    })
                }
                // parse words
                _ if self.current_char().is_alphabetic() => {
                    let mut word = String::new();
                    word.push(self.current_char());

                    loop {
                        if self.peek().is_alphabetic() || self.peek() == '_' {
                            self.next_char();
                            word.push(self.current_char());
                        } else {
                            break;
                        }
                    }

                    // if builtin:
                    if word == "print" {
                        self.tokens.push(Token {
                            value: word,
                            kind: TokenKind::Print,
                        });
                    } else if word == "type" {
                        self.tokens.push(Token {
                            value: word,
                            kind: TokenKind::Type,
                        });
                    } else if word == "let" {
                        self.tokens.push(Token {
                            value: word,
                            kind: TokenKind::Let,
                        });
                    } else if word == "Nat" {
                        self.tokens.push(Token {
                            value: word,
                            kind: TokenKind::Nat,
                        });
                    } else {
                        self.tokens.push(Token {
                            value: word,
                            kind: TokenKind::Word,
                        });
                    }

                }
                // check for numbers
                _ if self.current_char().is_digit(10) => {
                    let mut number = String::new();
                    number.push(self.current_char());

                    loop {
                        if self.peek().is_numeric() {
                            self.next_char();
                            number.push(self.current_char());
                        } else {
                            break;
                        }
                    }

                    self.tokens.push(Token {
                        value: number,
                        kind: TokenKind::Number,
                    });
                }
                '\n' => {}
                _ => {
                    return Err(format!("Unknown character: {}", self.current_char()));
                }
            }
            self.next_char();
        }

        Ok(())
    }
    
    pub fn current_char(&self) -> char {
        self.char.unwrap_or_else(|| '\0')
    }

    pub fn next_char(&mut self) {
        self.char = self.iterator.next();
    }

    fn peek(&mut self) -> char {
        self.iterator.peek().unwrap_or(&'\0').clone()
    }

    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::lang_lexer::LangLexer;
    use crate::lexer::DefaultLexer;
    use super::*;

    #[test]
    fn basic_lex() {
        let input = "print(\"hello world!\");";
        let mut lexer = LangLexer::new(input);
        lexer.tokenize().expect("TODO: panic message");
        let tokens = lexer.tokens();
        println!("{:?}", tokens);
    }
    
    #[test]
    fn basic_types() {
        let input = "type x;";
        let mut lexer = LangLexer::new(input);
        lexer.tokenize().expect("TODO: panic message");
        let tokens = lexer.tokens();
        println!("{:?}", tokens);
    }
    
    #[test]
    fn basic_declarations() {
        let input = "let x : Nat = 10;";
        let mut lexer = LangLexer::new(input);
        lexer.tokenize().expect("TODO: panic message");
        let tokens = lexer.tokens();
        println!("{:?}", tokens);
    }
}