use crate::bytecode::{Chunk, OpCode, Value};
use ast::node::{Node, OperatorKind};
use ast::ast::Ast;
pub struct Compiler;

impl Compiler {
    pub fn compile(ast: &Ast) -> Result<Chunk, String> {
        let mut chunk = Chunk::new();

        for node in &ast.nodes {
            // We start by compiling each top-level node in the AST
            Self::compile_node(node, &mut chunk)?;
        }

        // Add a final instruction to stop the VM
        chunk.write(OpCode::Return as u8);

        Ok(chunk)
    }
    
    fn compile_node(node: &Node, chunk: &mut Chunk) -> Result<(), String> {
        match node {
            Node::Atomic { value } => {
                match value {
                    Value::Int(i) => {
                        let constant_index = chunk.add_constant(Value::Int(*i));
                        // ...and then we emit a Constant opcode followed by the index.
                        chunk.write(OpCode::Constant as u8);
                        chunk.write(constant_index);
                    },
                    _ => return Err("Unsupported value type".to_string()),
                };
                
            }

            Node::BinaryExpression { left, operator, right } => {
                // This is the magic! It's a post-order traversal.
                // 1. Compile the left-hand side. This will leave its result on the stack.
                Self::compile_node(left, chunk)?;

                // 2. Compile the right-hand side. This will leave its result on the stack.
                Self::compile_node(right, chunk)?;

                // 3. Emit the operator instruction. It will pop the two values
                //    we just compiled, operate on them, and push the result.
                let op_code = OpCode::from(operator.clone());
                chunk.write(op_code as u8);
            }
            _ => return Err("Unsupported AST node".to_string()),
        }
        Ok(())
    }
}

pub mod debug {
    use super::{Chunk, OpCode};

    /// Prints a human-readable representation of a bytecode chunk.
    pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
        println!("== {} ==", name);

        // We use a while loop instead of a for loop because instructions
        // can have different sizes. `offset` is advanced manually.
        let mut offset = 0;
        while offset < chunk.code.len() {
            offset = disassemble_instruction(chunk, offset);
        }
    }

    /// Prints a single instruction and returns the offset of the next one.
    /// This is the core of the disassembler.
    pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
        // Print the byte offset of the instruction - helps with debugging jumps later.
        print!("{:04} ", offset);

        let instruction = chunk.code[offset];

        let opcode: OpCode = unsafe { std::mem::transmute(instruction) };

        match opcode {
            OpCode::Return | OpCode::Add | OpCode::Subtract | OpCode::Multiply | OpCode::Divide => {
                simple_instruction(opcode, offset)
            }
            OpCode::Constant => constant_instruction(opcode, chunk, offset),
        }
    }

    /// Helper for printing simple, one-byte instructions.
    fn simple_instruction(opcode: OpCode, offset: usize) -> usize {
        println!("{:?}", opcode);
        offset + 1
    }

    /// Helper for printing instructions that have a one-byte operand (the constant index).
    fn constant_instruction(opcode: OpCode, chunk: &Chunk, offset: usize) -> usize {
        // The operand is the byte right after the opcode
        let constant_index = chunk.code[offset + 1] as usize;
        let constant_value = &chunk.constants[constant_index];
        // Print instruction name, constant index, and the value itself.
        println!("{:_<-16} {:4} '{}'", format!("{:?}", opcode), constant_index, constant_value);
        offset + 2
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the compiler module
    use std::collections::HashMap;
    
    #[test]
    fn test_compile_simple_addition() {
        // 5.0 + 10.0
        let ast = Ast {
            nodes: vec![
                Node::BinaryExpression {
                    left: Box::new(Node::Atomic { value: Value::Int(5) }),
                    operator: OperatorKind::Add,
                    right: Box::new(Node::Atomic { value: Value::Int(10) }),
                }
            ],
            declarations: HashMap::new(),
        };
        
        let chunk = Compiler::compile(&ast).expect("Compilation failed");
        
        assert_eq!(chunk.constants.len(), 2, "Should have two constants");
        match (&chunk.constants[0], &chunk.constants[1]) {
            (Value::Int(a), Value::Int(b)) => {
                assert_eq!(*a, 5);
                assert_eq!(*b, 10);
            },
            _ => panic!("Constants are not Number values"),
        }
        
        let expected_code = vec![
            OpCode::Constant as u8, 0,
            OpCode::Constant as u8, 1, 
            OpCode::Add as u8, 
            OpCode::Return as u8,      
        ];

        assert_eq!(chunk.code, expected_code, "Bytecode sequence is incorrect");
    }

    #[test]
    fn test_compile_nested_expression() {
        // (5.0 - 2.0) * 10.0
        let ast = Ast {
            nodes: vec![
                Node::BinaryExpression {
                    left: Box::new(Node::BinaryExpression {
                        left: Box::new(Node::Atomic { value: Value::Int(5) }),
                        operator: OperatorKind::Subtract,
                        right: Box::new(Node::Atomic { value: Value::Int(2) }),
                    }),
                    operator: OperatorKind::Multiply,
                    right: Box::new(Node::Atomic { value: Value::Int(10) }),
                }
            ],
            declarations: HashMap::new(),
        };
        
        let chunk = Compiler::compile(&ast).expect("Compilation failed");
        
        assert_eq!(chunk.constants.len(), 3, "Should have three constants");
        match (&chunk.constants[0], &chunk.constants[1], &chunk.constants[2]) {
            (Value::Int(a), Value::Int(b), Value::Int(c)) => {
                assert_eq!(*a, 5);
                assert_eq!(*b, 2);
                assert_eq!(*c, 10);
            },
            _ => panic!("Constants are not Number values"),
        }
        
        let expected_code = vec![
            OpCode::Constant as u8, 0, // Push 5.0
            OpCode::Constant as u8, 1, // Push 2.0
            OpCode::Subtract as u8,    // Evaluate 5.0 - 2.0, result is on stack
            OpCode::Constant as u8, 2, // Push 10.0
            OpCode::Multiply as u8,    // Evaluate (result) * 10.0
            OpCode::Return as u8,      // Stop
        ];

        assert_eq!(chunk.code, expected_code, "Bytecode for nested expression is incorrect");

        use debug::disassemble_chunk;
        disassemble_chunk(&chunk, "Test Chunk");
    }
}