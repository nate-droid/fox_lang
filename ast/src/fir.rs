// fir stands for "Foxlang Intermediate Representation"

#[derive(Debug, Clone, PartialEq)]
pub enum FirValue {
    Local(u32),
    /// Immediate constant values
    ConstInt(i32),
    ConstFloat(f64),
    ConstBool(bool),
    ConstString(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum FirInstr {
    /// Arithmetic and logic
    Add(FirValue, FirValue),
    Sub(FirValue, FirValue),
    Mul(FirValue, FirValue),
    Div(FirValue, FirValue),
    Mod(FirValue, FirValue),
    Eq(FirValue, FirValue),
    Lt(FirValue, FirValue),
    Gt(FirValue, FirValue),

    /// Variable access
    LoadLocal(String),
    StoreLocal(String, FirValue),

    /// Function calls
    Call {
        func: String,
        args: Vec<FirValue>,
    },

    /// Return statement
    Return(Option<FirValue>),

    /// Control flow (basic for now)
    Jump(String),
    JumpIf {
        cond: FirValue,
        then_label: String,
        else_label: Option<String>,
    },

    /// no-op
    Nop,
}

#[derive(Debug, Clone)]
pub struct FirBlock {
    pub label: String,
    pub instrs: Vec<FirInstr>,
}

#[derive(Debug, Clone)]
pub struct FirFunction {
    pub name: String,
    pub params: Vec<String>,
    pub locals: Vec<String>,
    pub blocks: Vec<FirBlock>,
}

impl FirFunction {
    pub fn new(name: impl Into<String>, params: Vec<String>) -> Self {
        Self {
            name: name.into(),
            params,
            locals: vec![],
            blocks: vec![FirBlock {
                label: "entry".into(),
                instrs: vec![],
            }],
        }
    }

    pub fn emit(&mut self, instr: FirInstr) {
        if let Some(block) = self.blocks.last_mut() {
            block.instrs.push(instr);
        }
    }
}

#[derive(Debug, Clone)]
pub struct FirModule {
    pub functions: Vec<FirFunction>,
}

impl FirModule {
    pub fn new() -> Self {
        Self { functions: vec![] }
    }

    pub fn add_function(&mut self, f: FirFunction) {
        self.functions.push(f);
    }
}