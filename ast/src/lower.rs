use crate::fir::*;
use std::collections::HashMap;
use crate::node::{Node, OperatorKind};
use crate::value::Value;

pub struct LoweringContext {
    next_local: u32,
    locals: HashMap<String, u32>,
}

impl LoweringContext {
    pub fn new() -> Self {
        Self { next_local: 0, locals: HashMap::new() }
    }

    pub fn new_local(&mut self, name: &str) -> u32 {
        let id = self.next_local;
        self.next_local += 1;
        self.locals.insert(name.to_string(), id);
        id
    }

    pub fn get_local(&self, name: &str) -> Option<u32> {
        self.locals.get(name).cloned()
    }
}
fn ident_name(node: &Node) -> Option<String> {
    match node {
        Node::Identifier { value, .. } => Some(value.clone()),
        Node::Ident { name, .. } => Some(name.clone()),
        _ => None,
    }
}

fn expect_ident_name(node: &Node, what: &str) -> String {
    ident_name(node).unwrap_or_else(|| panic!("Expected identifier for {}, got {:?}", what, node))
}

pub fn lower_expr(expr: &Node, func: &mut FirFunction, ctx: &mut LoweringContext) -> FirValue {
    match expr {
        Node::Atomic { value } => match value {
            Value::Int(n) => FirValue::ConstInt(*n),
            Value::Bool(b) => FirValue::ConstBool(*b),
            Value::Str(s) => FirValue::ConstString(s.clone()),
            Value::Bin(u) => FirValue::ConstInt(*u as i32), // map bins to i32 for now
            // add other literals as needed
            _ => FirValue::ConstInt(0),
        },

        Node::Identifier { value } => {
            if let Some(idx) = ctx.get_local(value) {
                FirValue::Local(idx)
            } else {
                panic!("Unknown variable: {}", value)
            }
        }

        Node::Ident { name, .. } => {
            if let Some(idx) = ctx.get_local(name) {
                FirValue::Local(idx)
            } else {
                panic!("Unknown variable: {}", name)
            }
        }

        Node::BinaryExpression { left, operator, right } => {
            let lhs = lower_expr(left, func, ctx);
            let rhs = lower_expr(right, func, ctx);
            let tmp = ctx.new_local("_tmp");

            let instr = match operator {
                OperatorKind::Add => FirInstr::Add(lhs.clone(), rhs.clone()),
                OperatorKind::Subtract => FirInstr::Sub(lhs.clone(), rhs.clone()),
                OperatorKind::Multiply => FirInstr::Mul(lhs.clone(), rhs.clone()),
                OperatorKind::Divide => FirInstr::Div(lhs.clone(), rhs.clone()),
                OperatorKind::Modulo => FirInstr::Mod(lhs.clone(), rhs.clone()),
                OperatorKind::IsEqual => FirInstr::Eq(lhs.clone(), rhs.clone()),
                OperatorKind::LessThan => FirInstr::Lt(lhs.clone(), rhs.clone()),
                OperatorKind::GreaterThan => FirInstr::Gt(lhs.clone(), rhs.clone()),
                // you can add bitwise ops later as separate FIR opcodes
                _ => FirInstr::Nop,
            };

            func.emit(instr);
            FirValue::Local(tmp)
        }

        Node::Call { name, arguments, .. } => {
            let args: Vec<FirValue> = arguments.iter().map(|a| lower_expr(a, func, ctx)).collect();
            func.emit(FirInstr::Call { func: name.clone(), args: args.clone() });
            FirValue::Local(ctx.new_local("_call_tmp"))
        }

        _ => FirValue::ConstInt(0),
    }
}

pub fn lower_stmt(stmt: &Node, func: &mut FirFunction, ctx: &mut LoweringContext) {
    match stmt {
        Node::AssignStmt { left, right, .. } => {
            // left: Box<Node>
            if let Some(var) = ident_name(&**left) {
                let val = lower_expr(right, func, ctx);
                // ensure local exists
                if ctx.get_local(&var).is_none() {
                    ctx.new_local(&var);
                }
                func.emit(FirInstr::StoreLocal(var, val));
            } else {
                panic!("Assignment left side must be an identifier, got {:?}", left);
            }
        }

        // Return has value: Box<Node> (not Option)
        Node::Return { value } => {
            let ret_val = Some(lower_expr(&**value, func, ctx));
            func.emit(FirInstr::Return(ret_val));
        }

        Node::Call { .. } => {
            // side-effectful call as a statement
            let _ = lower_expr(stmt, func, ctx);
        }

        // Conditional uses Vec<Node> for branches
        Node::Conditional { condition, consequence, alternative, .. } => {
            let cond_val = lower_expr(condition, func, ctx);
            let then_label = "then".to_string();
            let else_label = if alternative.is_empty() { None } else { Some("else".to_string()) };
            func.emit(FirInstr::JumpIf { cond: cond_val, then_label, else_label });

            for s in consequence {
                lower_stmt(s, func, ctx);
            }
            if !alternative.is_empty() {
                for s in alternative {
                    lower_stmt(s, func, ctx);
                }
            }
        }

        _ => { todo!("not implemented yet") }
    }
}

pub fn lower_function(node: &Node) -> FirFunction {
    if let Node::FunctionDecl { name, arguments, body, .. } = node {
        // name: Box<Node> — extract string
        let func_name = expect_ident_name(&**name, "function name");

        let params: Vec<String> = arguments
            .iter()
            .map(|arg| expect_ident_name(arg, "function parameter"))
            .collect();

        let mut func = FirFunction::new(func_name, params.clone());
        let mut ctx = LoweringContext::new();
        for p in &params {
            ctx.new_local(p);
        }

        for stmt in body {
            lower_stmt(stmt, &mut func, &mut ctx);
        }

        func
    } else {
        panic!("Expected FunctionDecl node");
    }
}