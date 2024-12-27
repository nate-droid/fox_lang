use crate::lexer::{Token, TokenKind};
use crate::parser::Lexer;

pub struct LangLexer {
    input: String,
    position: usize,
    char: char,
    tokens: Vec<Token>,
}

impl LangLexer {
    pub fn new(input: String) -> Self {
        let first_char = input.chars().nth(0).unwrap_or(' ');
        
        Self {
            tokens: Vec::new(),
            input,
            position: 0,
            char: first_char,
        }
    }
    pub fn tokenize(&mut self) {
        while self.position < self.input.len() {
            match self.char {
                ' ' => {}
                '(' => {
                    self.tokens.push(Token {
                        value: self.char.to_string(),
                        kind: TokenKind::LeftParenthesis,
                    });
                }
                ')' => {
                    self.tokens.push(Token {
                        value: self.char.to_string(),
                        kind: TokenKind::RightParenthesis,
                    });
                }
                '\"' => {
                    let mut string = String::new();
                    self.next_char();
                    while self.char != '\"' {
                        string.push(self.char);
                        self.next_char();
                    }
                    self.tokens.push(Token {
                        value: string,
                        kind: TokenKind::String,
                    });
                }
                ';' => {
                    self.tokens.push(Token {
                        value: self.char.to_string(),
                        kind: TokenKind::Semicolon,
                    });
                }
                '=' => {
                    self.tokens.push(Token {
                        value: self.char.to_string(),
                        kind: TokenKind::Assign,
                    });
                }
                ':' => {
                    self.tokens.push(Token {
                        value: self.char.to_string(),
                        kind: TokenKind::Colon,
                    });
                }
                '+' => {
                    self.tokens.push(Token {
                        value: self.char.to_string(),
                        kind: TokenKind::Add,
                    })
                }
                // parse words
                _ if self.char.is_alphabetic() => {
                    let mut word = String::new();
                    word.push(self.char);

                    loop {
                        if self.peek().is_alphabetic() || self.peek() == '_' {
                            self.next_char();
                            word.push(self.char);
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
                _ if self.char.is_digit(10) => {
                    let mut number = String::new();
                    number.push(self.char);

                    loop {
                        if self.peek().is_numeric() {
                            self.next_char();
                            number.push(self.char);
                        } else {
                            break;
                        }
                    }

                    self.tokens.push(Token {
                        value: number,
                        kind: TokenKind::Number,
                    });
                }
                _ => {
                    panic!("Unknown character: {}", self.char);
                }
            }
            self.next_char();
        }
    }

    pub fn next_char(&mut self) {
        self.position += 1;
        self.char = self.input.chars().nth(self.position).unwrap_or(' ');
    }

    fn peek(&self) -> char {
        self.input.chars().nth(self.position + 1).unwrap_or(' ')
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
        let mut lexer = LangLexer::new(input.to_string());
        lexer.tokenize();
        let tokens = lexer.tokens();
        println!("{:?}", tokens);
    }
    
    #[test]
    fn basic_types() {
        let input = "type x;";
        let mut lexer = LangLexer::new(input.to_string());
        lexer.tokenize();
        let tokens = lexer.tokens();
        println!("{:?}", tokens);
    }
    
    #[test]
    fn basic_declarations() {
        let input = "let x : Nat = 10;";
        let mut lexer = LangLexer::new(input.to_string());
        lexer.tokenize();
        let tokens = lexer.tokens();
        println!("{:?}", tokens);
    }
}