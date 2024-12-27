use crate::lexer::{Token, TokenKind};
use crate::parser::Lexer;

pub struct MetaMathLexer {
    input: String,
    position: usize,
    char: char,
    tokens: Vec<Token>,
}

impl MetaMathLexer {

    pub fn new(input: String) -> Self {
        let first_char = input.chars().nth(0).unwrap_or(' ');
        Self {
            input,
            position: 0,
            char: first_char,
            tokens: Vec::new(),
        }
    }
}

impl Lexer for MetaMathLexer {
    fn tokenize(&mut self) {
        while self.position < self.input.len() - 1 {
            match self.char {
                ' ' => {}
                '⊢' => {
                    self.tokens.push(Token {
                        value: self.char.to_string(),
                        kind: TokenKind::Turnstile,
                    });
                }
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
                '→' => {
                    self.tokens.push(Token {
                        value: self.char.to_string(),
                        kind: TokenKind::Implies,
                    });
                }
                '¬' => {
                    self.tokens.push(Token {
                        value: self.char.to_string(),
                        kind: TokenKind::Negation,
                    });
                }
                '∀' => {
                    self.tokens.push(Token {
                        value: self.char.to_string(),
                        kind: TokenKind::ForAll,
                    });
                }
                '∃' => {
                    self.tokens.push(Token {
                        value: self.char.to_string(),
                        kind: TokenKind::Exists,
                    });
                }
                '𝑥' => {
                    self.tokens.push(Token {
                        value: self.char.to_string(),
                        // kind: TokenKind::BoundX,
                        kind: TokenKind::Identifier, // changing this to an identifier in order to simplify the early stages
                    });
                }
                '𝜑' => {
                    self.tokens.push(Token {
                        value: self.char.to_string(),
                        // kind: TokenKind::Phi,
                        kind: TokenKind::Identifier,
                    });
                }
                '𝜓' => {
                    self.tokens.push(Token {
                        value: self.char.to_string(),
                        // kind: TokenKind::Psi,
                        kind: TokenKind::Identifier,
                    });
                }
                '𝜒' => {
                    self.tokens.push(Token {
                        value: self.char.to_string(),
                        // kind: TokenKind::Chi,
                        kind: TokenKind::Identifier,
                    });
                }
                '&' => {
                    self.tokens.push(Token {
                        value: self.char.to_string(),
                        kind: TokenKind::HypothesisConjunction,
                    });
                }
                '⇒' => {
                    self.tokens.push(Token {
                        value: self.char.to_string(),
                        kind: TokenKind::HypothesisEnd,
                    });
                }
                '=' => {
                    self.tokens.push(Token {
                        value: self.char.to_string(),
                        kind: TokenKind::Equality,
                    });
                }
                'A' | 'B' | 'C' => {
                    self.tokens.push(Token {
                        value: self.char.to_string(),
                        kind: TokenKind::Identifier,
                    });
                }
                '𝑦' | '𝑧' => {
                    self.tokens.push(Token {
                        value: self.char.to_string(),
                        kind: TokenKind::Identifier,
                    });
                }
                '∈' => {
                    self.tokens.push(Token {
                        value: self.char.to_string(),
                        kind: TokenKind::ElementOf,
                    });
                }
                '↔' => {
                    self.tokens.push(Token {
                        value: self.char.to_string(),
                        kind: TokenKind::Biconditional,
                    });
                }
                _ => {
                    panic!("Invalid token: {}", self.char);
                }
            }
            self.next_char();
        }
    }

    fn current_token(&self) -> Token {
        self.tokens[self.position].clone()
    }


    fn next_char(&mut self) {
        self.position += 1;
        self.char = self.input.chars().nth(self.position).unwrap_or(' ');
    }

    fn peek(&self) -> char {
        self.input.chars().nth(self.position + 1).unwrap_or(' ')
    }

    fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ax1() {
        let input = "⊢ (𝜑 → (𝜓 → 𝜑))";
        let mut lexer = MetaMathLexer::new(input.to_string());
        lexer.tokenize();

        let expected_tokens = vec![
            TokenKind::Turnstile,
            TokenKind::LeftParenthesis,
            TokenKind::Identifier,
            TokenKind::Implies,
            TokenKind::LeftParenthesis,
            TokenKind::Identifier,
            TokenKind::Implies,
            TokenKind::Identifier,
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
        let mut lexer = MetaMathLexer::new(input.to_string());
        lexer.tokenize();

        let expected_tokens = vec![
            TokenKind::Turnstile,
            TokenKind::LeftParenthesis,
            TokenKind::LeftParenthesis,
            TokenKind::Identifier,
            TokenKind::Implies,
            TokenKind::LeftParenthesis,
            TokenKind::Identifier,
            TokenKind::Implies,
            TokenKind::Identifier,
            TokenKind::RightParenthesis,
            TokenKind::RightParenthesis,
            TokenKind::Implies,
            TokenKind::LeftParenthesis,
            TokenKind::LeftParenthesis,
            TokenKind::Identifier,
            TokenKind::Implies,
            TokenKind::Identifier,
            TokenKind::RightParenthesis,
            TokenKind::Implies,
            TokenKind::LeftParenthesis,
            TokenKind::Identifier,
            TokenKind::Implies,
            TokenKind::Identifier,
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
        let mut lexer = MetaMathLexer::new(input.to_string());
        lexer.tokenize();

        let expected_tokens = vec![
            TokenKind::Turnstile,
            TokenKind::LeftParenthesis,
            TokenKind::LeftParenthesis,
            TokenKind::Negation,
            TokenKind::Identifier,
            TokenKind::Implies,
            TokenKind::Negation,
            TokenKind::Identifier,
            TokenKind::RightParenthesis,
            TokenKind::Implies,
            TokenKind::LeftParenthesis,
            TokenKind::Identifier,
            TokenKind::Implies,
            TokenKind::Identifier,
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
        let mut lexer = MetaMathLexer::new(input.to_string());
        lexer.tokenize();

        let expected_tokens = vec![
            TokenKind::Turnstile,
            TokenKind::ForAll,
            TokenKind::Identifier,
            TokenKind::Identifier,
        ];

        for (i, expected_kind) in expected_tokens.iter().enumerate() {
            assert_eq!(lexer.tokens[i].kind, *expected_kind);
        }
    }

    #[test]
    fn test_ax_4() {
        let input = "⊢ (∀𝑥(𝜑 → 𝜓) → (∀𝑥𝜑 → ∀𝑥𝜓))";
        let mut lexer = MetaMathLexer::new(input.to_string());
        lexer.tokenize();

        let expected_tokens = vec![
            TokenKind::Turnstile,
            TokenKind::LeftParenthesis,
            TokenKind::ForAll,
            TokenKind::Identifier,
            TokenKind::LeftParenthesis,
            TokenKind::Identifier,
            TokenKind::Implies,
            TokenKind::Identifier,
            TokenKind::RightParenthesis,
            TokenKind::Implies,
            TokenKind::LeftParenthesis,
            TokenKind::ForAll,
            TokenKind::Identifier,
            TokenKind::Identifier,
            TokenKind::Implies,
            TokenKind::ForAll,
            TokenKind::Identifier,
            TokenKind::Identifier,
            TokenKind::RightParenthesis,
            TokenKind::RightParenthesis,
        ];

        for (i, expected_kind) in expected_tokens.iter().enumerate() {
            assert_eq!(lexer.tokens[i].kind, *expected_kind);
        }
    }
}