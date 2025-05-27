use crate::bytecode::{Chunk, OpCode, Value};
use ast::node::{Node, OperatorKind};
use ast::ast::Ast;
pub struct Compiler;

impl Compiler {
    fn emit_jump(op: OpCode, chunk: &mut Chunk) -> usize {
        chunk.write(op as u8);
        chunk.write(0xff);
        chunk.write(0xff);
        chunk.code.len() - 2
    }

    /// Overwrites the placeholder at the given offset with the calculated jump distance.
    fn patch_jump(offset: usize, chunk: &mut Chunk) {
        let jump = chunk.code.len() - offset - 2;

        if jump > u16::MAX as usize {
            panic!("Jump too large!");
        }

        chunk.code[offset] = ((jump >> 8) & 0xff) as u8;
        chunk.code[offset + 1] = (jump & 0xff) as u8;
    }
    pub fn compile(ast: &Ast) -> Result<Chunk, String> {
        let mut chunk = Chunk::new();

        if let Some((last_node, preceding_nodes)) = ast.nodes.split_last() {
            for node in preceding_nodes {
                Self::compile_node(node, &mut chunk)?;
                
                if is_expression_node(node) {
                    chunk.write(OpCode::Pop as u8);
                }
            }
            
            Self::compile_node(last_node, &mut chunk)?;

            // If the final node is a statement (not an expression), it produces
            // no value, so we push `nil` as the default result of the script.
            if !is_expression_node(last_node) {
                chunk.write(OpCode::Nil as u8);
            }
        } else {
            chunk.write(OpCode::Nil as u8);
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
                        chunk.write(OpCode::Constant as u8);
                        chunk.write(constant_index);
                    },
                    Value::Str(s) => {
                        let constant_index = chunk.add_constant(Value::Str(s.clone()));
                        chunk.write(OpCode::Constant as u8);
                        chunk.write(constant_index);
                    },
                    Value::Bool(b) => {
                        let constant_index = chunk.add_constant(Value::Bool(*b));
                        chunk.write(OpCode::Constant as u8);
                        chunk.write(constant_index);
                    },
                    _ => {
                        return Err(format!("Unsupported value type: {:?}", value));
                    },
                };
                
            }

            Node::BinaryExpression { left, operator, right } => {
                Self::compile_node(left, chunk)?;
                Self::compile_node(right, chunk)?;
                
                let op_code = OpCode::from(operator.clone());
                chunk.write(op_code as u8);
            }
            Node::UnaryExpression { operator, right } => {
                Self::compile_node(right, chunk)?;
                match operator {
                    OperatorKind::Subtract => chunk.write(OpCode::Negate as u8),
                    _ => return Err("Unsupported unary operator".to_string()),
                }
            }

            Node::AssignStmt { left, right, kind } => {
                Self::compile_node(right, chunk)?;
                
                if let Node::Identifier { value: name } = &**left {
                    let name_index = chunk.add_constant(Value::Str(name.clone()));

                    chunk.write(OpCode::DefineGlobal as u8);
                    chunk.write(name_index);
                } else {
                    return Err("Invalid assignment target".to_string());
                }
            }

            Node::Identifier { value: name } => {
                let name_index = chunk.add_constant(Value::Str(name.clone()));

                chunk.write(OpCode::GetGlobal as u8);
                chunk.write(name_index);
            }
            
            Node::Atomic { value } => {
                let const_index = chunk.add_constant(value.clone());
                chunk.write(OpCode::Constant as u8);
                chunk.write(const_index);
            }
            Node::Conditional { condition, consequence, alternative } => {
                Self::compile_node(condition, chunk)?;

                let else_jump = Self::emit_jump(OpCode::JumpIfFalse, chunk);
                
                Self::compile_block(consequence, chunk)?;
                
                let end_jump = Self::emit_jump(OpCode::Jump, chunk);
                
                Self::patch_jump(else_jump, chunk);
                
                Self::compile_block(alternative, chunk)?;
                
                Self::patch_jump(end_jump, chunk);
            }
            _ => return Err("Unsupported AST node".to_string()),
        }
        Ok(())
    }

    fn compile_block(nodes: &[Node], chunk: &mut Chunk) -> Result<(), String> {
        if let Some((last_node, preceding_nodes)) = nodes.split_last() {
            for node in preceding_nodes {
                Self::compile_node(node, chunk)?;
                if is_expression_node(node) {
                    chunk.write(OpCode::Pop as u8);
                }
            }
            
            Self::compile_node(last_node, chunk)?;
            
            if !is_expression_node(last_node) {
                chunk.write(OpCode::Nil as u8);
            }
        } else {
            chunk.write(OpCode::Nil as u8);
        }
        
        Ok(())
    }
}

fn is_expression_node(node: &Node) -> bool {
    matches!(
        node,
        Node::BinaryExpression { .. } |
        Node::UnaryExpression { .. } |
        Node::Identifier { .. } |
        Node::Atomic { .. } |
        Node::Call { .. } |
        Node::MethodCall { .. } |
        Node::IndexExpression { .. } |
        Node::Array { .. } |
        Node::HMap { .. } |
        Node::Conditional { .. }
    )
}

pub mod debug {
    use super::{Chunk, OpCode};

    /// Prints a human-readable representation of a bytecode chunk.
    pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
        println!("== {} ==", name);
        
        let mut offset = 0;
        while offset < chunk.code.len() {
            offset = disassemble_instruction(chunk, offset);
        }
    }

    /// Prints a single instruction and returns the offset of the next one.
    /// This is the core of the disassembler.
    pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
        // Print the byte offset of the instruction
        print!("{:04} ", offset);

        let instruction = chunk.code[offset];

        let opcode: OpCode = unsafe { std::mem::transmute(instruction) };

        match opcode {
            OpCode::Return | OpCode::Add | OpCode::Subtract | OpCode::Multiply | OpCode::Divide => {
                simple_instruction(opcode, offset)
            }
            OpCode::Constant => constant_instruction(opcode, chunk, offset),
            OpCode::Negate => simple_instruction(opcode, offset),
            OpCode::DefineGlobal => {
                let constant_index = chunk.code[offset + 1] as usize;
                let constant_value = &chunk.constants[constant_index];
                println!("{:_<-16} {:4} '{}'", format!("{:?}", opcode), constant_index, constant_value);
                offset + 2
            }
            OpCode::GetGlobal => {
                let constant_index = chunk.code[offset + 1] as usize;
                let constant_value = &chunk.constants[constant_index];
                println!("{:_<-16} {:4} '{}'", format!("{:?}", opcode), constant_index, constant_value);
                offset + 2
            }
            OpCode::SetGlobal => {
                let constant_index = chunk.code[offset + 1] as usize;
                let constant_value = &chunk.constants[constant_index];
                println!("{:_<-16} {:4} '{}'", format!("{:?}", opcode), constant_index, constant_value);
                offset + 2
            }
            OpCode::OpTrue | OpCode::OpFalse | OpCode::OpNot => {
                simple_instruction(opcode, offset)
            }
            OpCode::OpEqual | OpCode::OpGreater | OpCode::OpLess | OpCode::OpModulo => {
                simple_instruction(opcode, offset)
            }
            OpCode::JumpIfFalse | OpCode::Jump => {
                let jump_offset = ((chunk.code[offset + 1] as usize) << 8) | (chunk.code[offset + 2] as usize);
                println!("{:_<-16} {:4} -> {}", format!("{:?}", opcode), jump_offset, offset + jump_offset + 3);
                offset + 3
            }
            OpCode::Pop => {
                simple_instruction(opcode, offset)
            }
            OpCode::Nil => {
                simple_instruction(opcode, offset)
            }
        }
    }

    /// Helper for printing simple, one-byte instructions.
    fn simple_instruction(opcode: OpCode, offset: usize) -> usize {
        println!("{:?}", opcode);
        offset + 1
    }

    /// Helper for printing instructions that have a one-byte operand (the constant index).
    fn constant_instruction(opcode: OpCode, chunk: &Chunk, offset: usize) -> usize {
        let constant_index = chunk.code[offset + 1] as usize;
        let constant_value = &chunk.constants[constant_index];
        println!("{:_<-16} {:4} '{}'", format!("{:?}", opcode), constant_index, constant_value);
        offset + 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    #[test]
    fn test_compile_simple_addition() {
        // 5 + 10
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
        // (5 - 2) * 10
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
            OpCode::Constant as u8, 0,
            OpCode::Constant as u8, 1,
            OpCode::Subtract as u8,
            OpCode::Constant as u8, 2,
            OpCode::Multiply as u8,
            OpCode::Return as u8,
        ];

        assert_eq!(chunk.code, expected_code, "Bytecode for nested expression is incorrect");

        use debug::disassemble_chunk;
        disassemble_chunk(&chunk, "Test Chunk");
    }
}