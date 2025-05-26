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
    // We don't store the chunk directly, but we will have references to it during execution.
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack: Vec::new(),
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
            // Disassemble the instruction
            debug::disassemble_instruction(chunk, ip);

            // Read the byte at the instruction pointer.
            let instruction = chunk.code[ip];
            ip += 1; // Advance the pointer

            // Decode and execute the instruction.
            let opcode: OpCode = unsafe { std::mem::transmute(instruction) };
            match opcode {
                OpCode::Return => {
                    return Ok(self.stack.pop().expect("Stack empty upon return."));
                }
                OpCode::Constant => {
                    let const_index = chunk.code[ip] as usize;
                    ip += 1;
                    self.stack.push(chunk.constants[const_index].clone());
                }
                OpCode::Add      => {
                    let b = self.stack.pop().expect("Stack underflow");
                    let a = self.stack.pop().expect("Stack underflow");
                    match (a, b) {
                        (Value::Int(a_val), Value::Int(b_val)) => {
                            self.stack.push(Value::Int(a_val + b_val));
                        },
                        _ => return Err("Operands must be numbers.".to_string()),
                    }
                },
                OpCode::Subtract => {
                    let b = self.stack.pop().expect("Stack underflow");
                    let a = self.stack.pop().expect("Stack underflow");
                    match (a, b) {
                        (Value::Int(a_val), Value::Int(b_val)) => {
                            self.stack.push(Value::Int(a_val - b_val));
                        },
                        _ => return Err("Operands must be numbers.".to_string()),
                    }
                },
                OpCode::Multiply => {
                    let b = self.stack.pop().expect("Stack underflow");
                    let a = self.stack.pop().expect("Stack underflow");
                    match (a, b) {
                        (Value::Int(a_val), Value::Int(b_val)) => {
                            self.stack.push(Value::Int(a_val * b_val));
                        },
                        _ => return Err("Operands must be numbers.".to_string()),
                    }
                },
                OpCode::Divide   => {
                    let b = self.stack.pop().expect("Stack underflow");
                    let a = self.stack.pop().expect("Stack underflow");
                    match (a, b) {
                        (Value::Int(a_val), Value::Int(b_val)) => {
                            self.stack.push(Value::Int(a_val / b_val));
                        },
                        _ => return Err("Operands must be numbers.".to_string()),
                    }
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import VM
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
}