use std::cmp::PartialEq;
use std::collections::HashMap;
use crate::compiler::debug;
use crate::bytecode::{Chunk, OpCode, Value};

// A helper macro to handle binary operations.
// It pops two numbers, performs an operation, and pushes the result.
// This avoids a lot of repetitive code in the main loop.
// TODO: use an AST type for the return value so it can be more flexible.
macro_rules! binary_op {
    ($self:expr, $op:tt) => {
        {
            // Pop the two operands from the stack.
            // Note: The right-hand side is popped first!
            let b = $self.stack.pop().expect("Stack underflow");
            let a = $self.stack.pop().expect("Stack underflow");

            // Ensure both values are numbers.
            if let (Value::Number(a_val), Value::Number(b_val)) = (a, b) {
                // Perform the operation and push the result.
                $self.stack.push(Value::Number(a_val $op b_val));
            } else {
                // If they're not numbers, we have a runtime error.
                return Err("Operands must be numbers.".to_string());
            }
        }
    };
}


pub struct VM {
    // The VM has its own stack. `Vec` is perfect for this.
    stack: Vec<Value>,
    globals: HashMap<String, Value>,
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for OpCode {
    fn eq(&self, other: &Self) -> bool {
        matches!((self, other), (OpCode::Return, OpCode::Return) | (OpCode::Constant, OpCode::Constant) | (OpCode::Nil, OpCode::Nil) | (OpCode::OpTrue, OpCode::OpTrue) | (OpCode::OpFalse, OpCode::OpFalse) | (OpCode::Pop, OpCode::Pop) | (OpCode::Add, OpCode::Add) | (OpCode::Subtract, OpCode::Subtract) | (OpCode::Multiply, OpCode::Multiply) | (OpCode::Divide, OpCode::Divide) | (OpCode::Jump, OpCode::Jump) | (OpCode::JumpIfFalse, OpCode::JumpIfFalse))
    }
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack: Vec::new(),
            globals: HashMap::new(),
        }
    }
    
    pub fn interpret(&mut self, chunk: &Chunk) -> Result<Value, String> {
        // instruction pointer
        let mut ip = 0;
        
        loop {
            // Print the current state of the stack before each instruction.
            print!("          ");
            for value in &self.stack {
                print!("[ {} ]", value);
            }
            println!();
            
            debug::disassemble_instruction(chunk, ip);
            
            let instruction = chunk.code[ip];
            let opcode: OpCode = unsafe { std::mem::transmute(instruction) };
            match opcode {
                OpCode::Return => {
                    return Ok(self.stack.pop().unwrap_or(Value::Null));
                }
                OpCode::Constant => {
                    let const_index = chunk.code[ip + 1] as usize;
                    self.stack.push(chunk.constants[const_index].clone());
                    ip += 2;
                }
                OpCode::Nil => {
                    self.stack.push(Value::Null);
                    ip += 1;
                }
                OpCode::OpTrue => {
                    self.stack.push(Value::Bool(true));
                    ip += 1;
                }
                OpCode::OpFalse => {
                    self.stack.push(Value::Bool(false));
                    ip += 1;
                }
                OpCode::Pop => {
                    self.stack.pop();
                    ip += 1;
                }
                OpCode::OpEqual | OpCode::OpGreater | OpCode::OpLess | OpCode::Add | OpCode::Subtract | OpCode::Multiply | OpCode::Divide => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
 
                    if let (Value::Int(a_val), Value::Int(b_val)) = (a.clone(), b.clone()) {
                        let result = match opcode {
                            OpCode::OpEqual => Value::Bool(a_val == b_val),
                            OpCode::OpGreater => Value::Bool(a_val > b_val),
                            OpCode::OpLess => Value::Bool(a_val < b_val),
                            OpCode::Add => Value::Int(a_val + b_val),
                            OpCode::Subtract => Value::Int(a_val - b_val),
                            OpCode::Multiply => Value::Int(a_val * b_val),
                            OpCode::Divide => Value::Int(a_val / b_val),
                            _ => unreachable!(),
                        };
                        self.stack.push(result);
                    } else if opcode == OpCode::OpEqual {
                        self.stack.push(Value::Bool(a == b));
                    } else {
                        return Err("Mismatched types for operation".to_string());
                    }
                    ip += 1;
                }

                OpCode::Jump => {
                    let offset = ((chunk.code[ip + 1] as u16) << 8) | chunk.code[ip + 2] as u16;
                    ip += 3 + offset as usize;
                }
                OpCode::JumpIfFalse => {
                    let offset = ((chunk.code[ip + 1] as u16) << 8) | chunk.code[ip + 2] as u16;
                    let condition = self.stack.pop().expect("Stack underflow");

                    if value_is_falsey(&condition) {
                        ip += 3 + offset as usize;
                    } else {
                        ip += 3;
                    }
                }
                OpCode::Negate   => {
                    let value = self.stack.pop().expect("Stack underflow");
                    match value {
                        Value::Int(val) => self.stack.push(Value::Int(-val)),
                        _ => return Err("Operand must be a number.".to_string()),
                    }
                    ip += 1;
                },
                OpCode::DefineGlobal => {
                    let name_index = chunk.code[ip + 1] as usize;
                    let name = chunk.constants[name_index].clone();
                    if let Value::Str(name_str) = name {
                        let value = self.stack.pop().expect("Stack underflow");
                        self.globals.insert(name_str.clone(), value);
                        println!("Defined global variable: {}", name_str);
                    } else {
                        return Err("Global variable name must be a string.".to_string());
                    }
                    ip += 2;
                },
                OpCode::GetGlobal => {
                    let name_index = chunk.code[ip + 1] as usize;
                    let name = chunk.constants[name_index].clone();
                    if let Value::Str(name_str) = name {
                        match self.globals.get(&name_str) {
                            Some(value) => {
                                self.stack.push(value.clone());
                            }
                            None => {
                                return Err(format!("Undefined variable '{}'.", name_str));
                            }
                        }
                    } else {
                        return Err("Global variable name must be a string.".to_string());
                    }
                    ip += 2;
                },
                OpCode::SetGlobal => {
                    let name_index = chunk.code[ip + 1] as usize;
                    let name = chunk.constants[name_index].clone();
                    if let Value::Str(name_str) = name {
                        let value = self.stack.last().expect("Stack underflow").clone();
                        if self.globals.contains_key(&name_str) {
                            self.globals.insert(name_str.clone(), value);
                        } else {
                            return Err(format!("Undefined variable '{}'.", name_str));
                        }
                    } else {
                        return Err("Global variable name must be a string.".to_string());
                    }
                    ip += 2;
                },
                OpCode::OpNot => {
                    let value = self.stack.pop().expect("Stack underflow");
                    match value {
                        Value::Bool(b) => self.stack.push(Value::Bool(!b)),
                        _ => return Err("Operand must be a boolean.".to_string()),
                    }
                },
                OpCode::OpModulo => {
                    let b = self.stack.pop().expect("Stack underflow");
                    let a = self.stack.pop().expect("Stack underflow");
                    match (a, b) {
                        (Value::Int(a_val), Value::Int(b_val)) => {
                            if b_val == 0 {
                                return Err("Division by zero.".to_string());
                            }
                            self.stack.push(Value::Int(a_val % b_val));
                        },
                        _ => return Err("Operands must be numbers.".to_string()),
                    }
                },
            }
        }
    }
}

fn value_is_falsey(value: &Value) -> bool {
    matches!(value, Value::Bool(false))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::Compiler;
    use std::collections::HashMap;
    use ast::ast::Ast;
    use ast::node::{Node, OperatorKind};
    use ast::value::Value::Int;

    /// A helper function to run a test case.
    /// It takes an AST node, compiles it, runs the VM, and returns the result.
    fn run_vm_test(root_node: Node) -> Value {
        let ast = Ast {
            nodes: vec![root_node],
            declarations: HashMap::new(),
        };
        let chunk = Compiler::compile(&ast).expect("Test compilation failed");
        let mut vm = VM::new();
        vm.interpret(&chunk).expect("Test VM execution failed")
    }

    #[test]
    fn test_simple_arithmetic() {
        let node = Node::BinaryExpression {
            left: Box::new(Node::Atomic { value: Int(5) }),
            operator: OperatorKind::Add,
            right: Box::new(Node::Atomic { value: Int(10) }),
        };
        let result = run_vm_test(node);
        assert_eq!(result.to_string(), "15");
        
        let node = Node::BinaryExpression {
            left: Box::new(Node::Atomic { value: Int(20) }),
            operator: OperatorKind::Subtract,
            right: Box::new(Node::Atomic { value: Int(2) }),
        };
        let result = run_vm_test(node);
        assert_eq!(result.to_string(), "18");
        
        let node = Node::BinaryExpression {
            left: Box::new(Node::Atomic { value: Int(7) }),
            operator: OperatorKind::Multiply,
            right: Box::new(Node::Atomic { value: Int(7) }),
        };
        let result = run_vm_test(node);
        assert_eq!(result.to_string(), "49");
        
        let node = Node::BinaryExpression {
            left: Box::new(Node::Atomic { value: Int(100) }),
            operator: OperatorKind::Divide,
            right: Box::new(Node::Atomic { value: Int(20) }),
        };
        let result = run_vm_test(node);
        assert_eq!(result.to_string(), "5");
    }

    #[test]
    fn test_complex_nested_expression() {
        let node = Node::BinaryExpression {
            left: Box::new(Node::BinaryExpression {
                left: Box::new(Node::Atomic { value: Int(100) }),
                operator: OperatorKind::Subtract,
                right: Box::new(Node::Atomic { value: Int(20) }),
            }),
            operator: OperatorKind::Divide,
            right: Box::new(Node::BinaryExpression {
                left: Box::new(Node::Atomic { value: Int(2) }),
                operator: OperatorKind::Multiply,
                right: Box::new(Node::Atomic { value: Int(2) }),
            }),
        };

        let result = run_vm_test(node);
        assert_eq!(result.to_string(), "20");
    }

    #[test]
    fn test_precedence_is_handled_by_ast_structure() {
        let node = Node::BinaryExpression {
            left: Box::new(Node::Atomic { value: Int(5) }),
            operator: OperatorKind::Add,
            right: Box::new(Node::BinaryExpression {
                left: Box::new(Node::Atomic { value: Int(2) }),
                operator: OperatorKind::Multiply,
                right: Box::new(Node::Atomic { value: Int(10) }),
            }),
        };
        
        let result = run_vm_test(node);
        assert_eq!(result.to_string(), "25");
    }

    #[test]
    fn test_unary_negation() {
        // Simulates compiling `-10`
        let mut chunk = Chunk::new();
        let const_idx = chunk.add_constant(Int(10));
        chunk.write(OpCode::Constant as u8);
        chunk.write(const_idx);
        chunk.write(OpCode::Negate as u8);
        chunk.write(OpCode::Return as u8);

        let mut vm = VM::new();
        let result = vm.interpret(&chunk).unwrap();
        assert_eq!(result.to_string(), "-10");
    }

    #[test]
    fn test_global_variable_definition_and_use() {
        // Simulates compiling and running:
        // let a = 20;
        // a / 4

        let mut chunk = Chunk::new();

        // Statement 1: `let a = 20;`
        let val_idx = chunk.add_constant(Int(20));
        chunk.write(OpCode::Constant as u8);
        chunk.write(val_idx);

        let name_idx = chunk.add_constant(Value::Str("a".to_string()));
        chunk.write(OpCode::DefineGlobal as u8);
        chunk.write(name_idx);

        // Statement 2: `a / 4`
        chunk.write(OpCode::GetGlobal as u8);
        chunk.write(name_idx);

        let four_idx = chunk.add_constant(Int(4));
        chunk.write(OpCode::Constant as u8);
        chunk.write(four_idx);

        chunk.write(OpCode::Divide as u8); // a / 4
        chunk.write(OpCode::Return as u8);

        let mut vm = VM::new();
        let result = vm.interpret(&chunk).unwrap();
        assert_eq!(result.to_string(), "5");
    }

    #[test]
    fn test_multiple_global_variables() {
        // let a = 3;
        // let b = 4;
        // a * b

        let mut chunk = Chunk::new();

        // `let a = 3;`
        let three_idx = chunk.add_constant(Int(3));
        let a_idx = chunk.add_constant(Value::Str("a".to_string()));
        chunk.write(OpCode::Constant as u8); chunk.write(three_idx);
        chunk.write(OpCode::DefineGlobal as u8); chunk.write(a_idx);

        // `let b = 4;`
        let four_idx = chunk.add_constant(Int(4));
        let b_idx = chunk.add_constant(Value::Str("b".to_string()));
        chunk.write(OpCode::Constant as u8); chunk.write(four_idx);
        chunk.write(OpCode::DefineGlobal as u8); chunk.write(b_idx);

        // `a * b`
        chunk.write(OpCode::GetGlobal as u8); chunk.write(a_idx);
        chunk.write(OpCode::GetGlobal as u8); chunk.write(b_idx);
        chunk.write(OpCode::Multiply as u8);
        chunk.write(OpCode::Return as u8);

        let mut vm = VM::new();
        let result = vm.interpret(&chunk).unwrap();
        assert_eq!(result.to_string(), "12");
    }

    #[test]
    fn test_runtime_error_for_undefined_variable() {
        // Simulates running code that uses an undefined variable `x`.
        let mut chunk = Chunk::new();
        let name_idx = chunk.add_constant(Value::Str("x".to_string()));
        chunk.write(OpCode::GetGlobal as u8);
        chunk.write(name_idx);

        let mut vm = VM::new();
        let result = vm.interpret(&chunk);

        assert!(result.is_err(), "Expected an error, but got Ok");
        assert_eq!(result.unwrap_err(), "Undefined variable 'x'.");
    }
    #[test]
    fn test_if_else_statement() {
        let ast_true = Ast {
            nodes: vec![Node::Conditional {
                condition: Box::new(Node::Atomic { value: Value::Bool(true) }),
                consequence: vec![Node::Atomic { value: Int(10) }],
                alternative: vec![Node::Atomic { value: Int(20) }],
            }],
            declarations: Default::default(),
        };
        let chunk_true = Compiler::compile(&ast_true).expect("Compilation failed for true branch");
        let mut vm_true = VM::new();
        let result_true = vm_true.interpret(&chunk_true).expect("VM execution failed for true branch");
        assert_eq!(result_true.to_string(), "10");
    }

    #[test]
    fn test_expression_statement_cleans_stack() {
        let mut chunk = Chunk::new();
        
        let idx10 = chunk.add_constant(Value::Int(10));
        chunk.write(OpCode::Constant as u8); chunk.write(idx10);
        chunk.write(OpCode::Pop as u8); // Pop the unused value

        let idx20 = chunk.add_constant(Value::Int(20));
        chunk.write(OpCode::Constant as u8); chunk.write(idx20);
        chunk.write(OpCode::Pop as u8);

        chunk.write(OpCode::Nil as u8);
        chunk.write(OpCode::Return as u8);

        let mut vm = VM::new();
        let result = vm.interpret(&chunk).unwrap();

        assert_eq!(result, Value::Null);
    }
}