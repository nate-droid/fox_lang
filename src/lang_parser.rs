use crate::internal_types::{fetch_binary, fetch_integer, fetch_string};
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
        
        println!("tokens: {:?}", tokens);
        
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
                            ast.add_node(func);
                            continue;
                        }
                        "bin" => {
                            // parsing binary number type
                            self.consume(TokenKind::Word)?;
                            self.consume(TokenKind::LeftParenthesis)?;
                            let value = self.parse_node()?;
                            let bin = fetch_binary(value)?;

                            self.consume(TokenKind::RightParenthesis)?;

                            ast.add_node(Node::Atomic { value: Value::Bin(bin) });
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
                                TokenKind::Period => {
                                    // this is a method call
                                    let res = self.parse_method_call(ident)?;
                                    self.consume(TokenKind::Semicolon)?;
                                    ast.add_node(res);
                                    continue;
                                }
                                _ => {
                                }
                            }
                        }
                    }
                }
                TokenKind::Comment => {
                    // skip comments
                    self.advance();
                    continue;
                }
                TokenKind::BitwiseAnd => {
                    self.consume(TokenKind::BitwiseAnd)?;
                    if self.current_token()?.kind != TokenKind::BitwiseAnd {
                        return Err("Expected another hypothesis conjunction".to_string());
                    }
                }
                _ => {
                    println!("peek: {:?}", self.peek_token()?);
                    return Err(format!("unexpected token: {:?} while parsing", self.current_token()?));
                }
            }
            self.advance();
        }

        // dbg!(globals);

        Ok(ast)
    }

    fn parse_node(&mut self) -> Result<Node, String> {
        match self.current_token()?.kind {
            TokenKind::Number => { self.parse_number() }
            TokenKind::Word => { self.parse_word() }
            TokenKind::LeftParenthesis => { self.parse_left_parenthesis() }
            TokenKind::Identifier => { self.parse_identifier() }
            TokenKind::LBracket => { self.parse_left_bracket() }
            TokenKind::String => { self.parse_string() }
            TokenKind::Negation => { self.parse_negation() }
            TokenKind::LessThan => { self.parse_less_than() }
            _ => {
                println!("current: {:?}", self.current_token()?);
                println!("peek: {:?}", self.peek_token()?);
                Err(format!("unexpected token: {:?}", self.current_token()?))
            }
        }
    }
    
    fn parse_number(&mut self) -> Result<Node, String> {
        let val = Value::from_string(self.current_token()?.value);
        self.advance();
        Ok(Node::Atomic { value: val })
    }
    
    fn parse_word(&mut self) -> Result<Node, String> {
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
            "return" => {
                let value = self.parse_node()?;
                self.consume(TokenKind::Semicolon)?;
                return Ok(Node::Return { value: Box::from(value) });
            }
            "true" => {
                return Ok(Node::Atomic { value: Value::Bool(true) });
            }
            "false" => {
                return Ok(Node::Atomic { value: Value::Bool(false) });
            }
            "bin" => {
                self.consume(TokenKind::LeftParenthesis)?;
                let value = self.parse_node()?;
                
                self.consume(TokenKind::RightParenthesis)?;

                if self.current_token()?.kind.is_binary_operator()  {
                    let op = self.current_token()?;
                    self.advance();
                    let right = self.parse_node()?;

                    return Ok(Node::BinaryExpression {
                        left: Box::from(value),
                        operator: op.kind,
                        right: Box::from(right),
                    });
                }
                let integer = fetch_integer(value)?;
                return Ok(Node::Atomic { value: Value::Bin(integer as u32) });
            }
            _ => {}
        }

        if self.current_token()?.kind == TokenKind::Period {
            let res = self.parse_method_call(name)?;
            if self.current_token()?.kind == TokenKind::Semicolon {
                self.advance();
            }
            return Ok(res);
        }

        // parse potential function
        if self.current_token()?.kind == TokenKind::LeftParenthesis {
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
            return Ok(Node::Call {
                name: name.value,
                arguments,
                returns: vec![],
            });
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
            TokenKind::Add | TokenKind::Subtract | TokenKind::Multiply | TokenKind::Divide => {
                let operator = self.current_token()?;
                self.advance();
                let right = self.parse_node()?;
                Ok(Node::BinaryExpression {
                    left: Box::from(Node::AssignStmt {
                        left: Box::from(Node::Ident { name: name.value, kind: "var".to_string() }),
                        right: Box::from(Node::Atomic {
                            value: Value::Int(0),
                        }),
                        kind: "Nat".to_string(),
                    }),
                    operator: operator.kind,
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
            TokenKind::Period => {
                Ok(Node::Ident { name: name.value, kind: "var".to_string() })
            }
            _ => {
                Ok(Node::Ident { name: name.value, kind: "var".to_string() })
            }
        }
    }
    
    fn parse_left_parenthesis(&mut self) -> Result<Node, String> {
        // at the moment, the language expects this to be an expression. This might need to be rethought as the language grows
        let mut expression: String = "(".to_string();
        self.consume(TokenKind::LeftParenthesis)?;
        while self.current_token()?.kind != TokenKind::Semicolon {
            expression.push_str(&self.current_token()?.value);
            self.advance();
        }

        Ok(Node::MMExpression { expression })
    }
    
    fn parse_identifier(&mut self) -> Result<Node, String> {
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
    
    fn parse_left_bracket(&mut self) -> Result<Node, String> {
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
    
    fn parse_string(&mut self) -> Result<Node, String> {
        let val = Value::Str(self.current_token()?.value.clone());
        self.advance();
        Ok(Node::Atomic { value: val })
    }
    
    fn parse_negation(&mut self) -> Result<Node, String> {
        self.consume(TokenKind::Negation)?;
        let node = self.parse_node()?;
        Ok(Node::UnaryExpression { operator: TokenKind::Negation, right: Box::from(node) })
    }
    
    fn parse_less_than(&mut self) -> Result<Node, String> {
        // parsing a new Hashmap
        self.consume(TokenKind::LessThan)?;
        self.consume(TokenKind::GreaterThan)?;
        // TODO: there is currently only support for empty initialization
        Ok(Node::HMap { values: Default::default() })
    }
    
    fn parse_if(&mut self) -> Result<Node, String> {
        
        let condition = self.parse_condition_header()?;
        println!("condition: {:?}", condition);
        self.consume(TokenKind::LCurlyBracket)?;

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
        self.consume(TokenKind::LCurlyBracket)?;
        
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
            || self.current_token()?.kind == TokenKind::BitwiseAnd
            || self.current_token()?.kind == TokenKind::BitwiseOr
            || self.current_token()?.kind == TokenKind::BitwiseXor
            || self.current_token()?.kind == TokenKind::ShiftLeft
            || self.current_token()?.kind == TokenKind::ShiftRight
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
        
        // TODO: this is so ugly. Will need to find a way to unify semicolon consumption
        if self.current_token()?.kind == TokenKind::Semicolon {
            self.consume(TokenKind::Semicolon)?;    
        }
        
        
        Ok(ident)
    }
    
    fn parse_method_call(&mut self, target: Token) -> Result<Node, String> {
        self.consume(TokenKind::Period)?;
        let function_name = self.current_token()?;
        self.consume(TokenKind::Word)?;
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

        let right = Node::MethodCall {
            name: function_name.value,
            target: target.value,
            arguments,
            returns: vec![],
        };

        Ok(right)
    }
    
    fn parse_for_loop(&mut self) -> Result<Node, String> {
        let variable = self.current_token()?;
        self.consume(TokenKind::Word)?;

        self.consume(TokenKind::Word)?; // consume "in"
        
        let start = self.current_token()?;
        
        self.consume(TokenKind::Number)?;
        self.consume(TokenKind::Range)?;
        let end = self.current_token()?;
        match end.kind {
            TokenKind::Number => {
                // self.consume(TokenKind::Number)?;
                self.advance();
                self.consume(TokenKind::LCurlyBracket)?;


                let mut bracket_count = 1;

                let mut nodes = Vec::new();
                //while self.current_token()?.kind != TokenKind::RBracket {
                while bracket_count > 0 {

                    let node = self.parse_node()?;
                    nodes.push(node);
                    if self.current_token()?.kind == TokenKind::LCurlyBracket {
                        bracket_count += 1;
                    } else if self.current_token()?.kind == TokenKind::RCurlyBracket {
                        bracket_count -= 1;
                    }
                }
                self.consume(TokenKind::RCurlyBracket)?;

                Ok(Node::ForLoop {
                    variable: variable.value,
                    range: (
                        Box::from(Node::Atomic { value: Value::Int(start.value.parse::<i32>().unwrap()) }),
                        Box::from(Node::Atomic { value: Value::Int(end.value.parse::<i32>().unwrap()) }),
                    ),
                    body: nodes,
                })
            }
            TokenKind::Word => {
                self.consume(TokenKind::Word)?;
                self.consume(TokenKind::LCurlyBracket)?;

                let mut bracket_count = 1;

                let mut nodes = Vec::new();
                //while self.current_token()?.kind != TokenKind::RBracket {
                while bracket_count > 0 {

                    let node = self.parse_node()?;
                    nodes.push(node);
                    if self.current_token()?.kind == TokenKind::LCurlyBracket {
                        bracket_count += 1;
                    } else if self.current_token()?.kind == TokenKind::RCurlyBracket {
                        bracket_count -= 1;
                    }
                }
                self.consume(TokenKind::RCurlyBracket)?;

                Ok(Node::ForLoop {
                    variable: variable.value,
                    range: (
                        Box::from(Node::Atomic { value: Value::Int(start.value.parse::<i32>().unwrap()) }),
                        Box::from(Node::Ident { name: end.value, kind: "var".to_string() }),
                    ),
                    body: nodes,
                })
            }
            _ => {
                Err(format!("expected number, got {:?}", end))
            }
        }
        
    }
    
    fn parse_condition_header(&mut self) -> Result<Node, String> {
        // conditions must be wrapped in parentheses
        self.consume(TokenKind::LeftParenthesis)?;
        match self.current_token()?.kind {
            TokenKind::Word => {
                let n = self.parse_node()?;
                
                if self.current_token()?.kind == TokenKind::RightParenthesis {
                    self.advance();
                    return Ok(n);
                }
                
                let operator = self.current_token()?;
                self.advance();
                
                let right = self.parse_node()?;
                
                // todo: check if the current token is a binary operator
                if self.current_token()?.kind.is_binary_operator() {
                    let op = self.current_token()?;
                    self.advance();
                    let right_left = self.parse_node()?;

                    if self.current_token()?.kind != TokenKind::RightParenthesis {
                        let op_right = self.current_token()?;
                        self.advance();
                        // more expressions to parse
                        let right_right = self.parse_node()?;
                        self.consume(TokenKind::RightParenthesis)?;
                        return Ok(Node::BinaryExpression {
                            left: Box::from(Node::BinaryExpression {
                                left: Box::from(n),
                                operator: operator.kind,
                                right: Box::from(right),
                            }),
                            operator: op.kind,
                            right: Box::from(Node::BinaryExpression {
                                left: Box::from(right_left),
                                operator: op_right.kind,
                                right: Box::from(right_right),
                            }),
                        });
                    }
                    
                    return Ok(Node::BinaryExpression {
                        left: Box::from(Node::BinaryExpression {
                            left: Box::from(n),
                            operator: operator.kind,
                            right: Box::from(right),
                        }),
                        operator: op.kind,
                        right: Box::from(right_left),
                    });
                }
                self.consume(TokenKind::RightParenthesis)?;
                
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
                Err(format!("invalid conditional: {:?}", self.current_token()?))
            }
        }
    }
    
    // parse_expression will parse any potential expressions
    // example x will return a unary expression node
    // example x + 1 will return a BinaryExpression node
    // example: x == 1 will return a BinaryExpression node
    fn parse_expression(&mut self) -> Result<Node, String> {
        match self.current_token()?.kind {
            _ => {
                Err("not yet implemented".to_string())
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

        self.consume(TokenKind::LCurlyBracket)?;

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
        println!("operator: {:?}", operator);
        
        let right = self.parse_node()?;
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
            right: Box::from(right),
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
        while self.current_token()?.kind != TokenKind::RCurlyBracket {
            nodes.push(self.parse_node()?);
        }
        
        self.consume(TokenKind::RCurlyBracket)?;
        Ok(nodes)
    }
    
    fn parse_print(&mut self) -> Result<Node, String> {
        self.consume(TokenKind::LeftParenthesis)?;

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
    
    fn parse_len(&mut self) -> Result<Node, String> {
        self.consume(TokenKind::LeftParenthesis)?;
        let input = self.parse_node()?;
        self.consume(TokenKind::RightParenthesis)?;
        self.consume(TokenKind::Semicolon)?;
        
        let node = Node::Call {
            name: "len".to_string(),
            arguments: vec![input],
            returns: vec![],
        };
        Ok(node)
    }

    fn parse_body(&mut self) -> Result<Vec<Node>, String> {
        let mut nodes = Vec::new();
        while self.current_token()?.kind != TokenKind::RCurlyBracket {
            nodes.push(self.parse_node()?);
        }
        
        self.consume(TokenKind::RCurlyBracket)?;
        Ok(nodes)
    }

    fn consume(&mut self, kind: TokenKind) -> Result<(), String> {
        // debug
        if self.tokens[self.position].kind == kind {
            self.advance();
            Ok(())
        } else {
            println!("position: {:?}", self.position);
            println!("Val: {:?}", self.tokens[self.position].value);
            println!("Kind: {:?}", self.tokens[self.position].kind);
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
        let input = "
        print(\"hello world!\");
        // this is a comment
        print(\"there was a comment before this\");";

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
