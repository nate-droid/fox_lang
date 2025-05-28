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

    /// Pops a value, negates it, and pushes the result.
    Negate,

    /// Defines a new global variable.
    /// Operand: 1 byte (index of the variable name in the constant pool).
    DefineGlobal,

    /// Pushes the value of a global variable onto the stack.
    /// Operand: 1 byte (index of the variable name).
    GetGlobal,

    /// Pops a value and assigns it to an existing global variable.
    /// Operand: 1 byte (index of the variable name).
    SetGlobal,

    OpTrue,
    OpFalse,
    OpNot,
    OpEqual,
    OpGreater,
    OpLess,
    OpModulo,

    JumpIfFalse,
    Jump,
    Pop,
    Nil,
    Call,
}

impl From<OperatorKind> for OpCode {
    fn from(op: OperatorKind) -> Self {
        match op {
            OperatorKind::Add => OpCode::Add,
            OperatorKind::Subtract => OpCode::Subtract,
            OperatorKind::Multiply => OpCode::Multiply,
            OperatorKind::Divide => OpCode::Divide,
            _ => {
                panic!("Unsupported operator: {:?}", op);
            }
        }
    }
}

pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
        }
    }
    
    pub fn write(&mut self, byte: u8) {
        self.code.push(byte);
    }

    /// Adds a constant to the pool and returns its index.
    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.push(value);
        (self.constants.len() - 1) as u8
    }
}
