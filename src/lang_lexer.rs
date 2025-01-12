use std::iter::Peekable;
use std::str::Chars;
use crate::lexer::{Token, TokenKind};

pub struct LangLexer<'a> {
    char: Option<char>,
    tokens: Vec<Token>,
    iterator: Peekable<Chars<'a>>,
}

impl <'a> LangLexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut iterator = input.chars().peekable();

        Self {
            tokens: Vec::new(),
            char: iterator.next(),
            iterator,
        }
    }
    
    pub fn tokenize(&mut self) -> Result<(), String> {
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
                        kind: TokenKind::Equality,
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
                // for MetaMath specific cases
                '⊢' => {
                    self.tokens.push(Token {
                        value: self.current_char().to_string(),
                        kind: TokenKind::Turnstile,
                    });
                }
                '→' => {
                    self.tokens.push(Token {
                        value: self.current_char().to_string(),
                        kind: TokenKind::Implies,
                    });
                }
                '¬' => {
                    self.tokens.push(Token {
                        value: self.current_char().to_string(),
                        kind: TokenKind::Negation,
                    });
                }
                '∀' => {
                    self.tokens.push(Token {
                        value: self.current_char().to_string(),
                        kind: TokenKind::ForAll,
                    });
                }
                '∃' => {
                    self.tokens.push(Token {
                        value: self.current_char().to_string(),
                        kind: TokenKind::Exists,
                    });
                }
                '𝜑' => {
                    self.tokens.push(Token {
                        value: self.current_char().to_string(),
                        // kind: TokenKind::Phi,
                        kind: TokenKind::WFF,
                    });
                }
                '𝜓' => {
                    self.tokens.push(Token {
                        value: self.current_char().to_string(),
                        // kind: TokenKind::Psi,
                        kind: TokenKind::WFF,
                    });
                }
                '𝜒' => {
                    self.tokens.push(Token {
                        value: self.current_char().to_string(),
                        // kind: TokenKind::Chi,
                        kind: TokenKind::WFF,
                    });
                }
                '&' => {
                    self.tokens.push(Token {
                        value: self.current_char().to_string(),
                        kind: TokenKind::HypothesisConjunction,
                    });
                }
                '⇒' => {
                    self.tokens.push(Token {
                        value: self.current_char().to_string(),
                        kind: TokenKind::HypothesisEnd,
                    });
                }
                'A' | 'B' | 'C' => {
                    self.tokens.push(Token {
                        value: self.current_char().to_string(),
                        kind: TokenKind::Identifier,
                    });
                }
                '𝑡' | '𝑢' | '𝑣' | '𝑥' | '𝑦' | '𝑧' | '𝑤' => {
                    self.tokens.push(Token {
                        value: self.current_char().to_string(),
                        kind: TokenKind::SetVar,
                    });
                }
                '∈' => {
                    self.tokens.push(Token {
                        value: self.current_char().to_string(),
                        kind: TokenKind::ElementOf,
                    });
                }
                '↔' => {
                    self.tokens.push(Token {
                        value: self.current_char().to_string(),
                        kind: TokenKind::Biconditional,
                    });
                }
                '∧' => {
                    self.tokens.push(Token {
                        value: self.current_char().to_string(),
                        kind: TokenKind::Conjunction,
                    });
                }
                
                // parse words
                _ if self.current_char().is_alphabetic() => {
                    let mut word = String::new();
                    word.push(self.current_char());
                    
                    loop {
                        if self.peek().is_alphanumeric() || self.peek() == '_' {
                            self.next_char();
                            word.push(self.current_char());
                        } else {
                            break;
                        }
                    }
                    
                    self.tokens.push(Token {
                        value: word,
                        kind: TokenKind::Word,
                    });
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

    #[test]
    fn mm_expressions_in_fox() {
        let input = "let ax1 : Expr = (𝜓 → 𝜑);";
        let mut lexer = LangLexer::new(input);
        lexer.tokenize().expect("TODO: panic message");
        let tokens = lexer.tokens();
        println!("{:?}", tokens);
    }
    
    // MM focused tests
    #[test]
    fn test_ax1() {
        let input = "⊢ (𝜑 → (𝜓 → 𝜑))";
        let mut lexer = LangLexer::new(input);
        lexer.tokenize().expect("TODO: panic message");

        let expected_tokens = vec![
            TokenKind::Turnstile,
            TokenKind::LeftParenthesis,
            TokenKind::WFF,
            TokenKind::Implies,
            TokenKind::LeftParenthesis,
            TokenKind::WFF,
            TokenKind::Implies,
            TokenKind::WFF,
            TokenKind::RightParenthesis,
            TokenKind::RightParenthesis,
        ];

        for (i, expected_kind) in expected_tokens.iter().enumerate() {
            assert_eq!(lexer.tokens[i].kind, *expected_kind);
        }

        println!("{:?}", lexer.tokens);
    }

    #[test]
    fn test_ax2() {
        let input = "⊢ ((𝜑 → (𝜓 → 𝜒)) → ((𝜑 → 𝜓) → (𝜑 → 𝜒)))";
        let mut lexer = LangLexer::new(input);
        lexer.tokenize().expect("TODO: panic message");

        let expected_tokens = vec![
            TokenKind::Turnstile,
            TokenKind::LeftParenthesis,
            TokenKind::LeftParenthesis,
            TokenKind::WFF,
            TokenKind::Implies,
            TokenKind::LeftParenthesis,
            TokenKind::WFF,
            TokenKind::Implies,
            TokenKind::WFF,
            TokenKind::RightParenthesis,
            TokenKind::RightParenthesis,
            TokenKind::Implies,
            TokenKind::LeftParenthesis,
            TokenKind::LeftParenthesis,
            TokenKind::WFF,
            TokenKind::Implies,
            TokenKind::WFF,
            TokenKind::RightParenthesis,
            TokenKind::Implies,
            TokenKind::LeftParenthesis,
            TokenKind::WFF,
            TokenKind::Implies,
            TokenKind::WFF,
            TokenKind::RightParenthesis,
            TokenKind::RightParenthesis,
        ];

        for (i, expected_kind) in expected_tokens.iter().enumerate() {
            assert_eq!(lexer.tokens[i].kind, *expected_kind);
        }
    }

    #[test]
    fn test_ax3() {
        let input = "⊢ ((¬ 𝜑 → ¬ 𝜓) → (𝜓 → 𝜑))";
        let mut lexer = LangLexer::new(input);
        lexer.tokenize().expect("TODO: panic message");

        let expected_tokens = vec![
            TokenKind::Turnstile,
            TokenKind::LeftParenthesis,
            TokenKind::LeftParenthesis,
            TokenKind::Negation,
            TokenKind::WFF,
            TokenKind::Implies,
            TokenKind::Negation,
            TokenKind::WFF,
            TokenKind::RightParenthesis,
            TokenKind::Implies,
            TokenKind::LeftParenthesis,
            TokenKind::WFF,
            TokenKind::Implies,
            TokenKind::WFF,
            TokenKind::RightParenthesis,
            TokenKind::RightParenthesis,
        ];

        for (i, expected_kind) in expected_tokens.iter().enumerate() {
            assert_eq!(lexer.tokens[i].kind, *expected_kind);
        }
    }

    #[test]
    fn test_ax_gen() {
        let input = "⊢ ∀𝑥𝜑";
        let mut lexer = LangLexer::new(input);
        lexer.tokenize().expect("TODO: panic message");

        let expected_tokens = vec![
            TokenKind::Turnstile,
            TokenKind::ForAll,
            TokenKind::SetVar,
            TokenKind::WFF,
        ];

        for (i, expected_kind) in expected_tokens.iter().enumerate() {
            assert_eq!(lexer.tokens[i].kind, *expected_kind);
        }
    }

    #[test]
    fn test_ax_4() {
        let input = "⊢ (∀𝑥(𝜑 → 𝜓) → (∀𝑥𝜑 → ∀𝑥𝜓))";
        let mut lexer = LangLexer::new(input);
        lexer.tokenize().expect("TODO: panic message");

        let expected_tokens = vec![
            TokenKind::Turnstile,
            TokenKind::LeftParenthesis,
            TokenKind::ForAll,
            TokenKind::SetVar,
            TokenKind::LeftParenthesis,
            TokenKind::WFF,
            TokenKind::Implies,
            TokenKind::WFF,
            TokenKind::RightParenthesis,
            TokenKind::Implies,
            TokenKind::LeftParenthesis,
            TokenKind::ForAll,
            TokenKind::SetVar,
            TokenKind::WFF,
            TokenKind::Implies,
            TokenKind::ForAll,
            TokenKind::SetVar,
            TokenKind::WFF,
            TokenKind::RightParenthesis,
            TokenKind::RightParenthesis,
        ];

        for (i, expected_kind) in expected_tokens.iter().enumerate() {
            assert_eq!(lexer.tokens[i].kind, *expected_kind);
        }
    }
}