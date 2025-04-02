use crate::parser::Lexer;

#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    pub kind: TokenKind,
}

#[derive(Debug, Clone, PartialEq, Hash)]
#[derive(Eq)]
#[derive(PartialOrd)]
#[derive(Ord)]
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
    Comment,

    WFF, // Well-formed formula
    SetVar, // Set variable
    
    // Logic
    Implies,
    Negation,
    Turnstile,
    And,
    Or,
    ForAll,
    Exists,
    BoundX,
    LessThan,
    GreaterThan,
    
    // Bitwise operators
    BitwiseOr,  // |
    BitwiseAnd, // &, (formerly HypothesisConjunction this links two or more hypotheses
    BitwiseXor, // ^
    ShiftLeft, // <<
    ShiftRight, // >>
    
    // MetaMath specific
    HypothesisEnd, // ⇒, this ends a list of hypotheses
    Equality, // =
    ElementOf, // ∈
    Biconditional, // ↔
    Conjunction, // ∧
    Disjunction, // ∨
    Subset, // ⊆
    Equinumerosity, // ≈

    // builtins
    Print,
    Word, // once the format settles down, lets rename this to "identity"
    String,
    Semicolon,
    Type,
    Let,
    Colon,
    Nat,
    MMExpression,
    Break,
    Period,
    
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    
    LBracket,
    RBracket,
    LCurlyBracket,
    RCurlyBracket,
    Range,
    Comma,
    Index,
    Function,
    
    IsEqual, // ==
    EOF,
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
            TokenKind::BitwiseOr => write!(f, "Pipe"),
            TokenKind::Implies => write!(f, "→"),
            TokenKind::Negation => write!(f, "¬"),
            TokenKind::Turnstile => write!(f, "Turnstile"),
            TokenKind::And => write!(f, "And"),
            TokenKind::Or => write!(f, "Or"),
            TokenKind::ForAll => write!(f, "ForAll"),
            TokenKind::Exists => write!(f, "∃"),
            TokenKind::BoundX => write!(f, "BoundX"),
            TokenKind::BitwiseAnd => write!(f, "&"),
            TokenKind::BitwiseXor => write!(f, "^"),
            TokenKind::ShiftLeft => write!(f, "<<"),
            TokenKind::ShiftRight => write!(f, ">>"),
            TokenKind::HypothesisEnd => write!(f, "⇒"),
            TokenKind::Equality => write!(f, "="),
            TokenKind::ElementOf => write!(f, "∈"),
            TokenKind::Biconditional => write!(f, "↔"),
            TokenKind::Print => write!(f, "Print"),
            TokenKind::Word => write!(f, "Word"),
            TokenKind::String => write!(f, "String"),
            TokenKind::Semicolon => write!(f, "Semicolon"),
            TokenKind::Type => write!(f, "Type"),
            TokenKind::Let => write!(f, "Let"),
            TokenKind::Colon => write!(f, "Colon"),
            TokenKind::Nat => write!(f, "Nat"),
            TokenKind::Add => write!(f, "+"),
            TokenKind::Subtract => write!(f, "-"),
            TokenKind::Multiply => write!(f, "*"),
            TokenKind::Divide => write!(f, "/"),
            TokenKind::Modulo => write!(f, "%"),
            TokenKind::Comment => write!(f, "//"),
            TokenKind::MMExpression => write!(f, "MMExpression"),
            TokenKind::WFF => write!(f, "WFF"),
            TokenKind::SetVar => write!(f, "SetVar"),
            TokenKind::Conjunction => write!(f, "∧"),
            TokenKind::Disjunction => write!(f, "∨"),
            TokenKind::Subset => write!(f, "⊆"),
            TokenKind::Equinumerosity => write!(f, "≈"),
            TokenKind::LBracket => write!(f, "LBracket"),
            TokenKind::RBracket => write!(f, "RBracket"),
            TokenKind::LCurlyBracket => write!(f, "LCurlyBracket"),
            TokenKind::RCurlyBracket => write!(f, "RCurlyBracket"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Range => write!(f, "Range"),
            TokenKind::IsEqual => write!(f, "=="),
            TokenKind::EOF => write!(f, "EOF"),
            TokenKind::LessThan => write!(f, "<"),
            TokenKind::GreaterThan => write!(f, ">"),
            TokenKind::Break => write!(f, "Break"),
            TokenKind::Index => write!(f, "Index"),
            TokenKind::Function => write!(f, "Function"),
            TokenKind::Period => write!(f, "."),
        }
    }
}

impl TokenKind {
    pub fn is_binary_operator(&self) -> bool {
        matches!(self, 
            TokenKind::Implies 
            | TokenKind::ForAll 
            | TokenKind::Equality 
            | TokenKind::ElementOf 
            | TokenKind::Biconditional 
            | TokenKind::Conjunction 
            | TokenKind::Disjunction 
            | TokenKind::Subtract 
            | TokenKind::Add 
            | TokenKind::Multiply 
            | TokenKind::Divide 
            | TokenKind::Modulo 
            | TokenKind::IsEqual 
            | TokenKind::LessThan 
            | TokenKind::GreaterThan 
            | TokenKind::BitwiseOr 
            | TokenKind::BitwiseAnd 
            | TokenKind::BitwiseXor 
            | TokenKind::ShiftLeft 
            | TokenKind::ShiftRight
            | TokenKind::Or
            | TokenKind::And)
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
                '[' => {
                    self.tokens.push(Token {
                        value: self.char.to_string(),
                        kind: TokenKind::LBracket,
                    });
                }
                ']' => {
                    self.tokens.push(Token {
                        value: self.char.to_string(),
                        kind: TokenKind::RBracket,
                    });
                }
                '|' => {
                    if self.peek() == '-' {
                        self.tokens.push(Token {
                            value: "|-".to_string(),
                            kind: TokenKind::Turnstile,
                        });
                        self.next_char();
                    } else if self.peek() == '|' {
                        self.tokens.push(Token {
                            value: "||".to_string(),
                            kind: TokenKind::Or,
                        });
                        self.next_char();
                    }
                    else {
                        self.tokens.push(Token {
                            value: self.char.to_string(),
                            kind: TokenKind::BitwiseOr,
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
                '&' => {
                    if self.peek() == '&' {
                        self.tokens.push(Token {
                            value: "&&".to_string(),
                            kind: TokenKind::Conjunction,
                        });
                        self.next_char();
                    } else {
                        self.tokens.push(Token {
                            value: self.char.to_string(),
                            kind: TokenKind::BitwiseAnd,
                        });
                    }
                }
                '⇒' => {
                    self.tokens.push(Token {
                        value: self.char.to_string(),
                        kind: TokenKind::HypothesisEnd,
                    });
                }
                '∀' => {
                    self.tokens.push(Token {
                        value: self.char.to_string(),
                        kind: TokenKind::ForAll,
                    });
                }
                '𝜑' | '𝜓' | '𝜒' => {
                   self.tokens.push(Token {
                       value: self.char.to_string(),
                       kind: TokenKind::WFF,
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
