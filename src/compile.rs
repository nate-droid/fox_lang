// use inkwell::context::Context;
// use inkwell::module::Module;
// use inkwell::builder::Builder;
// use crate::lang_ast::Ast;
// use crate::parser::{Node, Value};
// 
// fn generate_ir(ast: Ast) -> Result<Module, String> {
//     let context = Context::create();
//     let module = context.create_module("lang");
//     let builder = context.create_builder();
//     
//     Ok(module)
// }
// 
// fn compile_node(node: Node, builder: Builder, context: Context) {
//     match node {
//         Node::Atomic {
//             value
//         } => {
//             // match value {
//             //     Value::Int(i) => {
//             //         let i32_type = context.i32_type();
//             //         let i32_val = i32_type.const_int(i as u64, false);
//             //         builder.build_int_to_ptr(i32_val, i32_type.ptr_type(0), "int_to_ptr");
//             //     },
//             //     Value::Bool(b) => {
//             //         let bool_type = context.bool_type();
//             //         let bool_val = bool_type.const_int(b as u64, false);
//             //         builder.build_int_to_ptr(bool_val, bool_type.ptr_type(0), "bool_to_ptr");
//             //     },
//             //     Value::Str(s) => {
//             //         let string_type = context.i8_type().ptr_type(0);
//             //         let string_val = context.const_string(s.as_bytes(), false);
//             //         // builder.build_pointer_cast(string_val, string_type, "string_to_ptr");
//             //     },
//             //     _ => {
//             //         unimplemented!();
//             //     },
//             // }
//         }
//         _ => {
//             unimplemented!();
//         },
//     }
// }