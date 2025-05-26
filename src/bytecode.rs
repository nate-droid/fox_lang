use ast::node::OperatorKind;
pub(crate) use ast::value::Value;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum OpCode {
    /// Pushes a constant from the constant pool onto the stack.
    /// Operand: 1 byte, the index of the constant.
    Constant,

    /// Pops two values, adds them, pushes the result.
    Add,
    /// Pops two values, subtracts them, pushes the result.
    Subtract,
    /// Pops two values, multiplies them, pushes the result.
    Multiply,
    /// Pops two values, divides them, pushes the result.
    Divide,
    /// A temporary instruction to end execution.
    Return,
}

// Helper to convert from your AST Operator to an OpCode
// You'll need to fill this in with your actual OperatorKind enum
impl From<OperatorKind> for OpCode {
    fn from(op: OperatorKind) -> Self {
        match op {
            OperatorKind::Add => OpCode::Add,
            OperatorKind::Subtract => OpCode::Subtract,
            OperatorKind::Multiply => OpCode::Multiply,
            OperatorKind::Divide => OpCode::Divide,
            // ... handle other operators or panic
            _ => {
                panic!("Unsupported operator: {:?}", op);
            }
        }
    }
}

pub struct Chunk {
    /// The actual bytecode instructions.
    pub code: Vec<u8>,
    /// The pool of constant values.
    pub constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
        }
    }

    /// Writes a single byte of code (like an OpCode).
    pub fn write(&mut self, byte: u8) {
        self.code.push(byte);
    }

    /// Adds a constant to the pool and returns its index.
    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.push(value);
        // We'll use a single byte for the index for now, so we're limited to 256 constants.
        (self.constants.len() - 1) as u8
    }
}
