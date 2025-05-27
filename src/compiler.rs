use crate::bytecode::{Chunk, OpCode, Value};
use ast::node::{Node, OperatorKind};
use ast::ast::Ast;
pub struct Compiler;

impl Compiler {
    fn emit_jump(op: OpCode, chunk: &mut Chunk) -> usize {
        chunk.write(op as u8);
        chunk.write(0xff); // Placeholder byte 1
        chunk.write(0xff); // Placeholder byte 2
        chunk.code.len() - 2 // Return the address of the placeholder
    }

    /// Overwrites the placeholder at the given offset with the calculated jump distance.
    fn patch_jump(offset: usize, chunk: &mut Chunk) {
        // -3 to account for the size of the jump instruction's operand itself (2 bytes)
        // and the jump opcode (1 byte)
        let jump = chunk.code.len() - offset - 2;

        if jump > u16::MAX as usize {
            // Handle error: jump is too large
            panic!("Jump too large!");
        }

        chunk.code[offset] = ((jump >> 8) & 0xff) as u8;
        chunk.code[offset + 1] = (jump & 0xff) as u8;
    }
    pub fn compile(ast: &Ast) -> Result<Chunk, String> {
        let mut chunk = Chunk::new();

        // for node in &ast.nodes {
        //     // We start by compiling each top-level node in the AST
        //     Self::compile_node(node, &mut chunk)?;
        // 
        //     if is_expression_node(node) {
        //         chunk.write(OpCode::Pop as u8);
        //     }
        // }

        //     if !last_node_is_expr {
        //         chunk.write(OpCode::Nil as u8);
        //     }
        
        // chunk.write(OpCode::Nil as u8);

        // Handle all statements except for the very last one.
        if let Some((last_node, preceding_nodes)) = ast.nodes.split_last() {
            for node in preceding_nodes {
                Self::compile_node(node, &mut chunk)?;
                // If the node was an expression, its result is unused, so pop it.
                if is_expression_node(node) {
                    chunk.write(OpCode::Pop as u8);
                }
            }

            // Now, compile the final node in the program.
            Self::compile_node(last_node, &mut chunk)?;

            // If the final node is a statement (not an expression), it produces
            // no value, so we push `nil` as the default result of the script.
            if !is_expression_node(last_node) {
                chunk.write(OpCode::Nil as u8);
            }
        } else {
            // The script is empty, so its result is `nil`.
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
                        // ...and then we emit a Constant opcode followed by the index.
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
                        // display an error for unsupported value types
                        //
                        // return Err("Unsupported value type".to_string())
                        return Err(format!("Unsupported value type: {:?}", value));
                    },
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
            Node::UnaryExpression { operator, right } => {
                // 1. Compile the expression on the right. Its value will be on the stack.
                Self::compile_node(right, chunk)?;
                // 2. Emit the operator instruction.
                match operator {
                    OperatorKind::Subtract => chunk.write(OpCode::Negate as u8),
                    // You would add OperatorKind::Not for logical not, etc.
                    _ => return Err("Unsupported unary operator".to_string()),
                }
            }

            Node::AssignStmt { left, right, kind } => {
                // For now, we'll treat `let my_var = ...` the same as `my_var = ...`
                // and assume they are all global.

                // 1. Compile the right-hand side. The value to be assigned is now on the stack.
                Self::compile_node(right, chunk)?;

                // 2. Get the variable name from the left-hand side.
                if let Node::Identifier { value: name } = &**left {
                    // 3. Add the name to the constant pool.
                    let name_index = chunk.add_constant(Value::Str(name.clone())); // You'll need to add String to your `Value` enum!

                    // 4. Emit the correct instruction.
                    // A simple implementation could always use DefineGlobal.
                    // A more advanced one would track declared variables.
                    chunk.write(OpCode::DefineGlobal as u8);
                    chunk.write(name_index);
                } else {
                    return Err("Invalid assignment target".to_string());
                }
            }

            Node::Identifier { value: name } => {
                // This is for when a variable is *used*, not assigned to.
                // 1. Add the name to the constant pool.
                let name_index = chunk.add_constant(Value::Str(name.clone())); // Again, needs `Value::String`
                // 2. Emit an instruction to get the variable's value.
                chunk.write(OpCode::GetGlobal as u8);
                chunk.write(name_index);
            }

            // You would also need to update your `Node::Atomic` to handle your custom Value type
            Node::Atomic { value } => {
                let const_index = chunk.add_constant(value.clone());
                chunk.write(OpCode::Constant as u8);
                chunk.write(const_index);
            }
            Node::Conditional { condition, consequence, alternative } => {
                // 1. Compile the condition. Its boolean result is now on the stack.
                Self::compile_node(condition, chunk)?;

                // 2. Emit a jump. It will execute and POP the condition value.
                //    If the condition was false, it jumps to the `else` branch.
                let else_jump = Self::emit_jump(OpCode::JumpIfFalse, chunk);

                // 3. Compile the `then` branch. Our helper ensures it leaves one value.
                Self::compile_block(consequence, chunk)?;

                // 4. Emit an unconditional jump to skip over the `else` branch.
                let end_jump = Self::emit_jump(OpCode::Jump, chunk);

                // 5. Backpatch the `JumpIfFalse` to point to the start of the else logic.
                Self::patch_jump(else_jump, chunk);

                // 6. Compile the `else` branch. Our helper ensures it also leaves one value.
                Self::compile_block(alternative, chunk)?;

                // 7. Backpatch the final `Jump` to point to the end of the whole expression.
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
                // If the node was an expression, its result is unused, so pop it.
                if is_expression_node(node) {
                    chunk.write(OpCode::Pop as u8);
                }
            }

            // Compile the final node in the block.
            Self::compile_node(last_node, chunk)?;

            // If the final node is a statement, it produces no value,
            // so we push `nil` as the result of the block.
            if !is_expression_node(last_node) {
                chunk.write(OpCode::Nil as u8);
            }
        } else {
            // The block is empty, so its result is `nil`.
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
            OpCode::Negate => simple_instruction(opcode, offset),
            OpCode::DefineGlobal => {
                // The next byte is the index of the constant (the variable name).
                let constant_index = chunk.code[offset + 1] as usize;
                let constant_value = &chunk.constants[constant_index];
                println!("{:_<-16} {:4} '{}'", format!("{:?}", opcode), constant_index, constant_value);
                offset + 2
            }
            OpCode::GetGlobal => {
                // The next byte is the index of the constant (the variable name).
                let constant_index = chunk.code[offset + 1] as usize;
                let constant_value = &chunk.constants[constant_index];
                println!("{:_<-16} {:4} '{}'", format!("{:?}", opcode), constant_index, constant_value);
                offset + 2
            }
            OpCode::SetGlobal => {
                // The next byte is the index of the constant (the variable name).
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
                // The next two bytes are the jump offset.
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