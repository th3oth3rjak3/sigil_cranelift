use crate::ast::*;
use cranelift::prelude::*;
use cranelift_codegen::ir::{Function, UserFuncName};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{Linkage, Module};

fn compile_expr(expr: &Expr, builder: &mut FunctionBuilder) -> Value {
    match expr {
        Expr::Number(n) => builder.ins().iconst(types::I64, *n),
        Expr::Add(left, right) => {
            let left_val = compile_expr(left, builder);
            let right_val = compile_expr(right, builder);
            builder.ins().iadd(left_val, right_val)
        }
        Expr::Sub(left, right) => {
            let left_val = compile_expr(left, builder);
            let right_val = compile_expr(right, builder);
            builder.ins().isub(left_val, right_val)
        }
        Expr::Variable(_name) => {
            // For now, assume it's the first parameter
            builder.block_params(builder.current_block().unwrap())[0]
        }
        _ => builder.ins().iconst(types::I64, 999),
    }
}

pub fn compile_function(stmt: &Stmt) -> Option<i64> {
    if let Stmt::FunDecl(name, params, body) = stmt {
        println!("Compiling function: {} with {} params", name, params.len());

        let builder = JITBuilder::new(cranelift_module::default_libcall_names()).unwrap();
        let mut module = JITModule::new(builder);

        // Create function signature with parameters
        let mut sig = module.make_signature();
        for _param in params {
            sig.params.push(AbiParam::new(types::I64)); // Add parameter to signature
        }
        sig.returns.push(AbiParam::new(types::I64));

        let mut func = Function::with_name_signature(UserFuncName::user(0, 0), sig);
        let mut func_ctx = FunctionBuilderContext::new();
        let mut builder = FunctionBuilder::new(&mut func, &mut func_ctx);

        let block = builder.create_block();
        builder.append_block_params_for_function_params(block); // This creates the parameters
        builder.switch_to_block(block);
        builder.seal_block(block);

        // Compile the function body
        if let Stmt::Block(statements) = body.as_ref() {
            for stmt in statements {
                if let Stmt::Return(expr) = stmt {
                    let value = compile_expr(expr, &mut builder);
                    builder.ins().return_(&[value]);
                    break;
                }
            }
        }

        builder.finalize();

        // Compile and execute with a test value
        let id = module
            .declare_function(name, Linkage::Export, &func.signature)
            .unwrap();
        let mut ctx = codegen::Context::for_function(func);
        module.define_function(id, &mut ctx).unwrap();
        module.finalize_definitions().unwrap();

        let code = module.get_finalized_function(id);

        if params.len() == 1 {
            let test_fn: fn(i64) -> i64 = unsafe { std::mem::transmute(code) };
            Some(test_fn(21)) // Test with 21, should return 42 for double function
        } else {
            let test_fn: fn() -> i64 = unsafe { std::mem::transmute(code) };
            Some(test_fn())
        }
    } else {
        None
    }
}

// Add this simple function to compile a single expression
pub fn compile_simple_return(expr: &Expr) -> i64 {
    let builder = JITBuilder::new(cranelift_module::default_libcall_names()).unwrap();
    let mut module = JITModule::new(builder);

    let mut sig = module.make_signature();
    sig.returns.push(AbiParam::new(types::I64));

    let mut func = Function::with_name_signature(UserFuncName::user(0, 0), sig);
    let mut func_ctx = FunctionBuilderContext::new();
    let mut builder = FunctionBuilder::new(&mut func, &mut func_ctx);

    let block = builder.create_block();
    builder.append_block_params_for_function_params(block);
    builder.switch_to_block(block);
    builder.seal_block(block);

    // Compile the expression
    let value = compile_expr(expr, &mut builder);

    builder.ins().return_(&[value]);
    builder.finalize();

    // Compile and run
    let id = module
        .declare_function("test", Linkage::Export, &func.signature)
        .unwrap();
    let mut ctx = codegen::Context::for_function(func);
    module.define_function(id, &mut ctx).unwrap();
    module.finalize_definitions().unwrap();

    let code = module.get_finalized_function(id);
    let test_fn: fn() -> i64 = unsafe { std::mem::transmute(code) };

    test_fn()
}
