use crate::parser::Lexer;

#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    pub kind: TokenKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Number,
    Operator,
    BinaryOperator,
    UnaryOperator,
    LeftParenthesis,
    RightParenthesis,
    Whitespace,
    Identifier,
    End,
    Pipe,

    // Logic
    Implies,
    Negation,
    Turnstile,
    And,
    ForAll,
    Exists,
    BoundX,

}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Number => write!(f, "Number"),
            TokenKind::Operator => write!(f, "Operator"),
            TokenKind::BinaryOperator => write!(f, "BinaryOperator"),
            TokenKind::UnaryOperator => write!(f, "UnaryOperator"),
            TokenKind::LeftParenthesis => write!(f, "LeftParenthesis"),
            TokenKind::RightParenthesis => write!(f, "RightParenthesis"),
            TokenKind::Whitespace => write!(f, "Whitespace"),
            TokenKind::Identifier => write!(f, "Identifier"),
            TokenKind::End => write!(f, "End"),
            TokenKind::Pipe => write!(f, "Pipe"),
            TokenKind::Implies => write!(f, "→"),
            TokenKind::Negation => write!(f, "¬"),
            TokenKind::Turnstile => write!(f, "Turnstile"),
            TokenKind::And => write!(f, "And"),
            TokenKind::ForAll => write!(f, "ForAll"),
            TokenKind::Exists => write!(f, "Exists"),
            TokenKind::BoundX => write!(f, "BoundX"),
        }
    }
}

impl TokenKind {
    pub fn is_binary_operator(&self) -> bool {
        match self {
            TokenKind::Implies => true,
            _ => false,
        }
    }

    pub fn is_unary_operator(&self) -> bool {
        match self {
            TokenKind::UnaryOperator => true,
            TokenKind::Negation => true,
            _ => false,
        }
    }

}

pub struct DefaultLexer {
    pub(crate) tokens: Vec<Token>,
    input: String,
    pub(crate) position: usize,
    char: char,
}

impl DefaultLexer {
    pub fn new(input: String) -> Self {
        let first_char = input.chars().nth(0).unwrap_or(' ');
        Self {
            tokens: Vec::new(),
            input,
            position: 0,
            char: first_char,
        }
    }
}

impl Lexer for DefaultLexer {
    fn tokenize(&mut self) {
        while self.position < self.input.len() {
            match self.char {
                ' ' => {
                    // TODO: Add this as a flag to include or exclude whitespace
                    // self.tokens.push(Token {
                    //     value: self.char.to_string(),
                    //     kind: TokenKind::Whitespace,
                    // });
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
                '|' => {
                    if self.peek() == '-' {
                        self.tokens.push(Token {
                            value: "|-".to_string(),
                            kind: TokenKind::Turnstile,
                        });
                        self.next_char();
                    } else {
                        self.tokens.push(Token {
                            value: self.char.to_string(),
                            kind: TokenKind::Pipe,
                        });
                    }
                }
                '-' => {
                    if self.peek() == '>' {
                        self.tokens.push(Token {
                            value: "->".to_string(),
                            kind: TokenKind::Implies,
                        });
                        self.next_char();
                    } else {
                        self.tokens.push(Token {
                            value: self.char.to_string(),
                            kind: TokenKind::Operator,
                        });
                    }
                }
                '~' | '¬' => {
                    self.tokens.push(Token {
                        value: self.char.to_string(),
                        kind: TokenKind::Negation,
                    });
                }
                '→' => {
                    self.tokens.push(Token {
                        value: self.char.to_string(),
                        kind: TokenKind::Implies,
                    });
                }
                // test if alphabetic
                ch if ch.is_alphanumeric() => {
                    // peek next char
                    if self.peek().is_alphabetic() {
                        let mut identifier = self.char.to_string();
                        while self.peek().is_alphabetic() {
                            self.next_char();
                            identifier.push(self.char);
                        }
                        self.tokens.push(Token {
                            value: identifier,
                            kind: TokenKind::Identifier,
                        });
                    } else {
                        self.tokens.push(Token {
                            value: self.char.to_string(),
                            kind: TokenKind::Identifier,
                        });
                    }
                }
                '\0' => {
                    self.tokens.push(Token {
                        value: self.char.to_string(),
                        kind: TokenKind::End,
                    });
                }
                _ => {
                    self.tokens.push(Token {
                        value: self.char.to_string(),
                        kind: TokenKind::Number,
                    });
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
    fn split_strings() {
        let input = "Hello, world!";
        let mut lexer = DefaultLexer::new(input.to_string());
        lexer.tokenize();

        assert_eq!(lexer.tokens.len(), 4);
    }

    #[test]
    fn parse_logical_expression() {
        let input = "(A -> B)";
        let mut lexer = DefaultLexer::new(input.to_string());
        lexer.tokenize();
        println!("{:?}", lexer.tokens);
        assert_eq!(lexer.tokens.len(), 5);
    }

    #[test]
    fn lex_negation() {
        let input = "~A";
        let mut lexer = DefaultLexer::new(input.to_string());
        lexer.tokenize();
        println!("{:?}", lexer.tokens);
        assert_eq!(lexer.tokens.len(), 2);
    }

    #[test]
    fn lex_turnstile() {
        let input = "|-";
        let mut lexer = DefaultLexer::new(input.to_string());
        lexer.tokenize();
        println!("{:?}", lexer.tokens);
        assert_eq!(lexer.tokens.len(), 1);
    }
}
