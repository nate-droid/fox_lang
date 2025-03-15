use crate::lang_ast::Ast;
use crate::lang_lexer::LangLexer;
use crate::lexer::TokenKind::{And, Comma};
use crate::lexer::{Token, TokenKind};
use crate::parser::{Node, Value};

pub struct LangParser<'a> {
    lexer: LangLexer<'a>,
    tokens: Vec<Token>,
    position: usize,
}

impl<'a> LangParser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = LangLexer::new(input);

        match lexer.tokenize() {
            Ok(_) => (),
            Err(e) => panic!("{:?}", e),
        }

        let tokens = lexer.tokens();

        Self {
            lexer,
            tokens,
            position: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Ast, String> {
        let mut globals = Vec::new();
        let mut ast = Ast::new();

        while self.position < self.tokens.len() {
            match self.current_token()?.kind {
                TokenKind::Word => {
                    let t = self.current_token()?;
                    
                    match t.value.as_str() {
                        "print" => {
                            self.consume(TokenKind::Word)?;
                            let node = self.parse_print()?;
                            ast.add_node(node);
                            continue;
                        }
                        "let" => {
                            self.advance();
                            let ident = self.parse_let()?;
                            
                            globals.push(ident.val());
                            ast.add_node(ident);
                            continue;
                        }
                        "type" => {
                            self.consume(TokenKind::Word)?;
                            let type_name = self.current_token()?;
                            self.consume(TokenKind::Word)?;

                            self.consume(TokenKind::Semicolon)?;

                            ast.add_node(Node::Type {
                                name: type_name.value,
                            });
                            continue;
                        }
                        "if" => {
                            self.consume(TokenKind::Word)?;
                            
                            let node = self.parse_if()?;
                            
                            ast.add_node(node);
                            continue;
                        }
                        "for" => {
                            self.consume(TokenKind::Word)?;
                            
                            let node = self.parse_for_loop()?;
                            
                            ast.add_node(node);
                            continue;
                        }
                        "fn" => {
                            let func = self.parse_function()?;
                            println!("{:?}", func);
                            ast.add_node(func);
                            continue;
                        }
                        _ => {
                            let ident = self.current_token()?;
                            self.consume(TokenKind::Word)?;
                            
                            match self.current_token()?.kind {
                                TokenKind::Equality => {
                                    self.consume(TokenKind::Equality)?;
                                    let value = self.parse_node()?;

                                    ast.add_node(Node::AssignStmt {
                                        left: Box::from(Node::Ident {
                                            name: ident.value,
                                            kind: "var".to_string(),
                                        }),
                                        right: Box::from(value),
                                        kind: "Nat".to_string(),
                                    });
                                    self.consume(TokenKind::Semicolon)?;
                                    continue;
                                }
                                TokenKind::LeftParenthesis => {
                                    // parsing a function call
                                    self.consume(TokenKind::LeftParenthesis)?;
                                    let mut arguments = Vec::new();
                                    while self.current_token()?.kind != TokenKind::RightParenthesis {
                                        let arg = self.parse_node()?;
                                        arguments.push(arg);
                                        if self.current_token()?.kind == TokenKind::RightParenthesis {
                                            break;
                                        }
                                        self.consume(Comma)?;
                                    }
                                    self.consume(TokenKind::RightParenthesis)?;
                                    self.consume(TokenKind::Semicolon)?;
                                    ast.add_node(Node::Call {
                                        name: ident.value,
                                        arguments,
                                        returns: vec![],
                                    })
                                }
                                TokenKind::LBracket => {
                                    self.consume(TokenKind::LBracket)?;
                                    let index = self.parse_node()?;

                                    self.consume(TokenKind::RBracket)?;
                                    self.consume(TokenKind::Equality)?;
                                    let value = self.parse_node()?;

                                    let index_update = Node::IndexExpression {
                                        left: Box::from(Node::Ident { name: ident.value, kind: "var".to_string() }),
                                        index: Box::from(index),
                                    };

                                    let n = Node::AssignStmt {
                                        left: Box::from(index_update),
                                        right: Box::from(value),
                                        kind: "".to_string(),
                                    };
                                    self.consume(TokenKind::Semicolon)?;
                                    ast.add_node(n);
                                    continue;
                                }
                                _ => {}
                            }
                        }
                    }
                }
                TokenKind::Comment => {
                    // skip comments
                    println!("Skipping comment");
                }
                TokenKind::HypothesisConjunction => {
                    self.consume(TokenKind::HypothesisConjunction)?;
                    if self.current_token()?.kind != TokenKind::HypothesisConjunction {
                        return Err("Expected another hypothesis conjunction".to_string());
                    }
                }
                _ => {
                    println!("current: {:?}", self.current_token()?);
                    // println!("peek: {:?}", self.tokens[self.position + 1].kind);
                    // println!("peek value: {:?}", self.tokens[self.position + 1].value);
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
                let val = Value::from_string(self.current_token()?.value);
                self.advance();
                Ok(Node::Atomic { value: val })
            }
            TokenKind::Word => {
                let name = self.current_token()?;
                self.advance();

                match name.value.as_str() {
                    "print" => return self.parse_print(),
                    "let" => {
                        let ident = self.parse_let()?;

                        return Ok(ident);
                    }
                    "if" => return self.parse_if(),
                    "break" => {
                        self.consume(TokenKind::Semicolon)?;
                        return Ok(Node::Break{});
                    }
                    _ => {}
                }

                match self.current_token()?.kind {
                    TokenKind::Equality => {
                        self.consume(TokenKind::Equality)?;
                        let value = self.parse_node()?;
                        self.consume(TokenKind::Semicolon)?;
                        Ok(Node::AssignStmt {
                            left: Box::from(Node::Ident { name: name.value, kind: "var".to_string() }),
                            kind: "Nat".to_string(),
                            right: Box::from(value),
                        })
                    }
                    TokenKind::Add => {
                        self.consume(TokenKind::Add)?;
                        let right = self.parse_node()?;
                        Ok(Node::BinaryExpression {
                            left: Box::from(Node::AssignStmt {
                                left: Box::from(Node::Ident { name: name.value, kind: "var".to_string() }),
                                right: Box::from(Node::Atomic {
                                    value: Value::Int(0),
                                }),
                                kind: "Nat".to_string(),
                            }),
                            operator: TokenKind::Add,
                            right: Box::from(right),
                        })
                    }
                    TokenKind::LBracket => {
                        self.consume(TokenKind::LBracket)?;
                        let index = self.parse_node()?;

                        self.consume(TokenKind::RBracket)?;
                        
                        // TODO: Check for nested arrays
                        if self.current_token()?.kind == TokenKind::LBracket {
                            self.consume(TokenKind::LBracket)?;
                            let index2 = self.parse_node()?;
                            self.consume(TokenKind::RBracket)?;
                            
                            if self.current_token()?.kind == TokenKind::Equality {
                                self.consume(TokenKind::Equality)?;
                                let right = self.parse_node()?;
                                panic!("not yet implemented");
                            }
                            
                            return Ok(Node::IndexExpression {
                                left: Box::from(Node::IndexExpression {
                                    left: Box::from(Node::Ident {name: name.value, kind: "var".to_string()}),
                                    index: Box::from(index),
                                }),
                                index: Box::from(index2),
                            });
                        }
                        
                        Ok(Node::IndexExpression {
                            left: Box::from(Node::AssignStmt {
                                left: Box::from(Node::Ident { name: name.value, kind: "var".to_string() }),
                                right: Box::from(Node::Atomic {
                                    value: Value::Int(0),
                                }),
                                kind: "Nat".to_string(),
                            }),
                            index: Box::from(index),
                        })
                    }
                    _ => {
                        Ok(Node::AssignStmt {
                            left: Box::from(Node::Ident { name: name.value, kind: "var".to_string() }),
                            right: Box::from(Node::Atomic {
                                value: Value::Int(0),
                            }),
                            kind: "Nat".to_string(),
                        })
                    }
                }
            }
            TokenKind::LeftParenthesis => {
                // at the moment, the language expects this to be an expression. This might need to be rethought as the language grows
                let mut expression: String = "(".to_string();
                self.consume(TokenKind::LeftParenthesis)?;
                while self.current_token()?.kind != TokenKind::Semicolon {
                    expression.push_str(&self.current_token()?.value);
                    self.advance();
                }

                Ok(Node::MMExpression { expression })
            }
            TokenKind::Identifier => {
                let name = self.current_token()?;
                self.advance();
                Ok(Node::AssignStmt {
                    left: Box::from(Node::Ident { name: name.value, kind: "var".to_string() }),
                    right: Box::from(Node::Atomic {
                        value: Value::Int(0),
                    }),
                    kind: "Nat".to_string(),
                })
            }
            TokenKind::LBracket => {
                // parse array
                self.consume(TokenKind::LBracket)?;
                let mut nodes = Vec::new();
                
                while self.current_token()?.kind != TokenKind::RBracket {
                    nodes.push(self.parse_node()?);
                    if self.current_token()?.kind == TokenKind::RBracket {
                        break;
                    }
                    self.consume(Comma)?;
                }
                self.consume(TokenKind::RBracket)?;
                
                Ok(Node::Array { elements: nodes })
            }
            TokenKind::String => {
                let val = Value::Str(self.current_token()?.value.clone());
                self.advance();
                Ok(Node::Atomic { value: val })
            }
            _ => {
                println!("kind: {:?}", self.current_token()?.kind);
                println!("value: {:?}", self.current_token()?.value);
                Err("Unexpected token".to_string())
            }
        }
    }
    
    fn parse_if(&mut self) -> Result<Node, String> {
        
        let condition = self.parse_condition_header()?;
        self.consume(TokenKind::LBracket)?;

        // parse consequence
        let consequence = self.parse_consequence()?;
        
        if self.current_token()?.value != "else" {
            return Ok(Node::Conditional {
                condition: Box::from(condition),
                consequence,
                alternative: vec![],
            });
        }
        
        self.consume(TokenKind::Word)?;
        self.consume(TokenKind::LBracket)?;
        
        let alternative = self.parse_consequence()?;

        Ok(Node::Conditional {
            condition: Box::from(condition),
            consequence,
            alternative,
        })
    }

    fn parse_let(&mut self) -> Result<Node, String> {
        let name = self.current_token()?;
        
        self.consume(TokenKind::Word)?;

        self.consume(TokenKind::Equality)?;

        let left = self.parse_node()?;
        
        // TODO: Will need a more robust way to handle expressions in the future
        if self.current_token()?.kind == TokenKind::Add
            || self.current_token()?.kind == TokenKind::Modulo
            || self.current_token()?.kind == TokenKind::Subtract
            || self.current_token()?.kind == TokenKind::Multiply
            || self.current_token()?.kind == TokenKind::Divide
        {
            
            let op = self.current_token()?;
            self.advance();

            let right = self.parse_node()?;

            let n = Node::BinaryExpression {
                left: Box::from(left),
                operator: op.kind,
                right: Box::from(right),
            };

            let ident = Node::AssignStmt {
                left: Box::from(Node::Ident { name: name.value, kind: "var".to_string() }),
                right: Box::from(n),
                kind: "str".to_string(),
            };
            
            
            self.consume(TokenKind::Semicolon)?;
            return Ok(ident)
        }

        let ident = Node::AssignStmt {
            left: Box::from(Node::Ident { name: name.value, kind: "var".to_string() }),
            right: Box::from(left),
            kind: "str".to_string(),
        };
        
        self.consume(TokenKind::Semicolon)?;
        
        Ok(ident)
    }
    
    fn parse_for_loop(&mut self) -> Result<Node, String> {
        let variable = self.current_token()?;
        self.consume(TokenKind::Word)?;

        self.consume(TokenKind::Word)?;
        let start = self.current_token()?;
        self.consume(TokenKind::Number)?;
        self.consume(TokenKind::Range)?;
        let end = self.current_token()?;
        self.consume(TokenKind::Number)?;
        self.consume(TokenKind::LBracket)?;


        let mut bracket_count = 1;

        let mut nodes = Vec::new();
        //while self.current_token()?.kind != TokenKind::RBracket {
        while bracket_count > 0 {
            
            let node = self.parse_node()?;
            nodes.push(node);
            if self.current_token()?.kind == TokenKind::LBracket {
                bracket_count += 1;
            } else if self.current_token()?.kind == TokenKind::RBracket {
                bracket_count -= 1;
            }
        }
        self.consume(TokenKind::RBracket)?;
        
        Ok(Node::ForLoop {
            variable: variable.value,
            range: (
                start.value.parse::<i32>().unwrap(),
                end.value.parse::<i32>().unwrap(),
            ),
            body: nodes,
        })
    }
    
    fn parse_condition_header(&mut self) -> Result<Node, String> {
        // conditions must be wrapped in parentheses
        self.consume(TokenKind::LeftParenthesis)?;
        match self.current_token()?.kind {
            TokenKind::Word => {
                let n = self.parse_condition()?;

                if self.current_token()?.kind == TokenKind::RightParenthesis {
                    self.advance();
                    return Ok(n);
                }
                let operator = self.current_token()?;
                self.advance();
                let right = self.parse_condition()?;
                
                Ok(Node::BinaryExpression {
                    left: Box::from(n),
                    operator: operator.kind,
                    right: Box::from(right),
                })
            }
            TokenKind::Number => {
                let left = self.current_token()?;
                self.advance();
                let operator = self.current_token()?;
                self.advance();
                let right = self.parse_condition()?;
                
                let node = Node::BinaryExpression {
                    left: Box::from(Node::Atomic {
                        value: Value::Int(left.value.parse::<i32>().unwrap()),
                    }),
                    operator: operator.kind,
                    right: Box::from(right),
                };
                Ok(node)
            }
            _ => {
                println!("{:?}", self.current_token()?.kind);
                Err("Invalid conditional".to_string())
            }
        }
    }
    
    fn parse_function(&mut self) -> Result<Node, String> {
        self.consume(TokenKind::Word)?; // consume "fn"
        let name = self.parse_function_name()?;

        self.consume(TokenKind::LeftParenthesis)?;
        let mut arguments = Vec::new();

        while self.current_token()?.kind != TokenKind::RightParenthesis {
            let arg = self.parse_function_input()?;
            
            if self.current_token()?.kind != TokenKind::RightParenthesis {
                self.consume(Comma)?;    
            }
            arguments.push(arg);
        }
        self.consume(TokenKind::RightParenthesis)?;
        
        // TODO: ignoring return types for now :(
        
        self.consume(TokenKind::LBracket)?;
        
        let body = self.parse_body()?;
        
        Ok(Node::FunctionDecl {
            name: Box::from(name),
            arguments,
            returns: vec![],
            body,
        })
    }

    fn parse_function_name(&mut self) -> Result<Node, String> {
        let name = self.current_token()?;
        self.consume(TokenKind::Word)?;
        
        Ok(Node::Ident {
            name: name.value,
            kind: "fn".to_string(),
        })
    }
    
    fn parse_function_input(&mut self) -> Result<Node, String> {
        let ident = self.current_token()?;
        self.consume(TokenKind::Word)?;
        
        // TODO this will need more advanced pattern matching when more types are introduced and supported
        
        Ok(Node::Ident {
            name: ident.value,
            kind: "var".to_string(),
        })
    }
    
    fn parse_condition(&mut self) -> Result<Node, String> {
        // conditions must be wrapped in parentheses
        let left = self.current_token()?;
        if left.value == "true" {
            self.advance();
            return Ok(Node::Atomic {
                value: Value::Bool(true),
            });
        } else if left.value == "false" {
            self.advance();
            return Ok(Node::Atomic {
                value: Value::Bool(false),
            });
        }
        
        if self.peek_token()?.kind == TokenKind::RightParenthesis {
            let val = self.current_token()?.value;
            self.advance();
            
            if self.current_token()?.kind != TokenKind::Number {
                self.consume(TokenKind::RightParenthesis)?;
                return Ok(Node::Atomic {
                    value: Value::Int(val.parse().unwrap()),
                });
            }

            self.advance();
            self.consume(TokenKind::RightParenthesis)?;
            
            return Ok(Node::AssignStmt {
                left: Box::from(Node::Ident { name: left.value, kind: "var".to_string() }),
                right: Box::from(Node::Atomic {
                    value: Value::Int(0),
                }),
                kind: "Nat".to_string(),
            });
        }
        
        self.advance();
        let operator = self.current_token()?;

        self.advance();

        let right = self.current_token()?;
        self.advance();
        
        // TODO: add support for the right side being a variable
        let node = Node::BinaryExpression {
            left: Box::from(Node::AssignStmt {
                left: Box::from(Node::Ident { name: left.value, kind: "var".to_string() }),
                right: Box::from(Node::Atomic {
                    value: Value::Int(0),
                }),
                kind: "Nat".to_string(),
            }),
            operator: operator.kind,
            right: Box::from(Node::Atomic {
                value: Value::Int(right.value.parse::<i32>().unwrap()),
            }),
        };
        
        if self.current_token()?.kind != And && self.current_token()?.kind != TokenKind::Or {
            return Ok(node);
        }
        
        let op = self.current_token()?;
        self.advance();

        let right2 = self.parse_condition()?;

        Ok(Node::BinaryExpression {
            left: Box::from(node),
            operator: op.kind,
            right: Box::from(right2),
        })
    }

    fn parse_consequence(&mut self) -> Result<Vec<Node>, String> {
        let mut nodes = Vec::new();
        while self.current_token()?.kind != TokenKind::RBracket {
            nodes.push(self.parse_node()?);
        }
        
        self.consume(TokenKind::RBracket)?;
        Ok(nodes)
    }
    
    fn parse_print(&mut self) -> Result<Node, String> {
        self.consume(TokenKind::LeftParenthesis)?;

        // let input = self.current_token()?;
        let input = self.parse_node()?;

        self.advance();
        
        self.consume(TokenKind::Semicolon)?;

        let node = Node::Call {
            name: "print".to_string(),
            arguments: vec![input],
            returns: vec![],
        };
        Ok(node)
    }

    fn parse_body(&mut self) -> Result<Vec<Node>, String> {
        let mut nodes = Vec::new();
        while self.current_token()?.kind != TokenKind::RBracket {
            nodes.push(self.parse_node()?);
        }
        
        self.consume(TokenKind::RBracket)?;
        Ok(nodes)
    }

    fn consume(&mut self, kind: TokenKind) -> Result<(), String> {
        // debug
        if self.tokens[self.position].kind == kind {
            self.advance();
            Ok(())
        } else {
            println!("{:?}", self.tokens[self.position].kind);
            println!("{:?}", self.tokens);
            Err(format!(
                "Expected {:?} but found {:?}",
                kind, self.tokens[self.position].kind
            ))
        }
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn current_token(&self) -> Result<Token, String> {
        if self.position < self.tokens.len() {
            Ok(self.tokens[self.position].clone())
        } else {
            Ok(Token {
                kind: TokenKind::EOF,
                value: "".to_string(),
            })
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
        let input = "let x = 10;";

        let mut parser = LangParser::new(input);

        let ast = parser.parse().expect("TODO: panic message");
        println!("{:?}", ast);
    }

    #[test]
    fn custom_types() {
        let input = "type nat;";
        let mut parser = LangParser::new(input);
        let ast = parser.parse().expect("TODO: panic message");
        println!("{:?}", ast);
    }

    #[test]
    fn multi_line_variables() {
        let input = "let x = 1;\
        let y = 2;";
        let mut parser = LangParser::new(input);
        println!("{:?}", parser.tokens);
        let ast = parser.parse().expect("unexpected failure");
        println!("{:?}", ast);
    }

    #[test]
    fn addition() {
        let input = "let x = 1 + 2;";
        let mut parser = LangParser::new(input);
        let ast = parser.parse().expect("unexpected failure");
        println!("{:?}", ast);

        assert_eq!(ast.nodes.len(), 1);
    }
    
    #[test]
    fn subtraction() {
        let input = "let x = 1 - 2;";
        let mut parser = LangParser::new(input);
        let ast = parser.parse().expect("unexpected failure");

        assert_eq!(ast.nodes.len(), 1);
    }
    
    #[test]
    fn multiplication() {
        let input = "let x = 1 * 2;";
        let mut parser = LangParser::new(input);
        let ast = parser.parse().expect("unexpected failure");

        assert_eq!(ast.nodes.len(), 1);
    }
    
    #[test]
    fn division() {
        let input = "let x = 1 / 2;";
        let mut parser = LangParser::new(input);
        let ast = parser.parse().expect("unexpected failure");

        assert_eq!(ast.nodes.len(), 1);
    }

    #[test]
    fn addition_with_variables() {
        let input = "let x = 1;\
        let y = 2;\
        let z = x + y;";
        let mut parser = LangParser::new(input);
        let ast = parser.parse().expect("unexpected failure");
        println!("{:?}", ast);

        assert_eq!(ast.nodes.len(), 3);
    }

    #[test]
    fn mm_expressions_in_fox() {
        let input = "let ax = (𝜓 → 𝜑);";
        let mut parser = LangParser::new(input);
        let ast = parser.parse().expect("unexpected failure");
        println!("{:?}", ast);

        // TODO: When parsing a "let" statement, check if the type is "Expression", and call the mm parser
    }

    #[test]
    fn error_handling() {
        // add checks that the parser returns an error when it should
        // for example, having an expression not terminate with a semicolon
    }
}
