use cranelift_codegen::ir::{Function, UserFuncName, InstBuilder};
use cranelift_codegen::settings::{self, Configurable};
use cranelift_codegen::Context;
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Variable};
use cranelift_module::{Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};

use crate::{Stmt, Expr};
use std::collections::HashMap;

pub struct CodeGenerator {
    module: ObjectModule,
    ctx: Context,
    builder_ctx: FunctionBuilderContext,
}

impl CodeGenerator {
    pub fn new() -> Self {
        // Create the ISA (Instruction Set Architecture) for the target
        let mut flag_builder = settings::builder();
        flag_builder.set("use_colocated_libcalls", "false").unwrap();
        flag_builder.set("is_pic", "false").unwrap();
        let isa_builder = cranelift_codegen::isa::lookup_by_name("x86_64").unwrap();
        let isa = isa_builder.finish(settings::Flags::new(flag_builder)).unwrap();

        // Create the object module
        let object_builder = ObjectBuilder::new(
            isa,
            "bam_program",
            cranelift_module::default_libcall_names(),
        ).unwrap();
        let module = ObjectModule::new(object_builder);

        Self {
            module,
            ctx: Context::new(),
            builder_ctx: FunctionBuilderContext::new(),
        }
    }

    pub fn compile_program(mut self, statements: &[Stmt]) -> Vec<u8> {
        // Create a main function  
        let mut sig = self.module.make_signature();
        sig.returns.push(cranelift_codegen::ir::AbiParam::new(cranelift_codegen::ir::types::I32));
        sig.params.clear();

        let main_func_id = self.module
            .declare_function("main", Linkage::Export, &sig)
            .unwrap();

        // Build the function
        let mut func = Function::with_name_signature(UserFuncName::user(0, main_func_id.as_u32()), sig);
        let mut builder = FunctionBuilder::new(&mut func, &mut self.builder_ctx);

        // Create the entry block
        let entry_block = builder.create_block();
        builder.append_block_params_for_function_params(entry_block);
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);

        // For our tracer bullet, we'll implement very basic functionality
        let mut variables = HashMap::new();

        for stmt in statements {
            CodeGenerator::compile_statement_static(stmt, &mut builder, &mut variables);
        }

        // Return 0 (success)
        let zero = builder.ins().iconst(cranelift_codegen::ir::types::I32, 0);
        builder.ins().return_(&[zero]);

        // Finalize the function
        builder.finalize();

        // Define the function in the module
        self.ctx.func = func;
        self.module.define_function(main_func_id, &mut self.ctx).unwrap();

        // Finalize the module and get the object bytes
        let object_product = self.module.finish();
        object_product.emit().unwrap()
    }

    fn compile_statement_static(
        stmt: &Stmt,
        builder: &mut FunctionBuilder,
        variables: &mut HashMap<String, Variable>,
    ) {
        match stmt {
            Stmt::Assign(var_name, expr) => {
                let value = CodeGenerator::compile_expression_static(expr, builder, variables);
                
                // Create a new variable if it doesn't exist
                if !variables.contains_key(var_name) {
                    let var = Variable::from_u32(variables.len() as u32);
                    builder.declare_var(var, cranelift_codegen::ir::types::I32);
                    variables.insert(var_name.clone(), var);
                }
                
                let var = variables[var_name];
                builder.def_var(var, value);
            }
            Stmt::Expr(expr) => {
                // For now, just compile the expression (useful for function calls)
                CodeGenerator::compile_expression_static(expr, builder, variables);
            }
            Stmt::If(_, _) => {
                // TODO: Implement if statements
                println!("If statements not yet implemented in codegen");
            }
        }
    }

    fn compile_expression_static(
        expr: &Expr,
        builder: &mut FunctionBuilder,
        variables: &HashMap<String, Variable>,
    ) -> cranelift_codegen::ir::Value {
        match expr {
            Expr::Num(n) => {
                builder.ins().iconst(cranelift_codegen::ir::types::I32, *n as i64)
            }
            Expr::Var(name) => {
                if let Some(&var) = variables.get(name) {
                    builder.use_var(var)
                } else {
                    // Return 0 if variable doesn't exist
                    builder.ins().iconst(cranelift_codegen::ir::types::I32, 0)
                }
            }
            Expr::Add(left, right) => {
                let left_val = CodeGenerator::compile_expression_static(left, builder, variables);
                let right_val = CodeGenerator::compile_expression_static(right, builder, variables);
                builder.ins().iadd(left_val, right_val)
            }
            Expr::Str(_) => {
                // For now, return a placeholder
                // TODO: Implement string data section
                builder.ins().iconst(cranelift_codegen::ir::types::I64, 0)
            }
            Expr::Call(func_name, _args) => {
                if func_name == "print" {
                    // For our tracer bullet, we'll implement a simple print
                    // TODO: Implement actual printf call
                    builder.ins().iconst(cranelift_codegen::ir::types::I32, 0)
                } else {
                    builder.ins().iconst(cranelift_codegen::ir::types::I32, 0)
                }
            }
            Expr::Eq(_, _) => {
                // TODO: Implement comparison
                builder.ins().iconst(cranelift_codegen::ir::types::I32, 0)
            }
        }
    }
}