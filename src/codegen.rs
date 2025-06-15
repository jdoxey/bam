use cranelift_codegen::ir::{Function, UserFuncName, InstBuilder};
use cranelift_codegen::settings::{self, Configurable};
use cranelift_codegen::Context;
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Variable};
use cranelift_module::{Linkage, Module, FuncId, DataId};
use cranelift_object::{ObjectBuilder, ObjectModule};

use crate::{Stmt, Expr};
use std::collections::HashMap;

pub struct CodeGenerator {
    module: ObjectModule,
    ctx: Context,
    builder_ctx: FunctionBuilderContext,
    printf_func: Option<FuncId>,
    string_data: HashMap<String, DataId>,
}

impl CodeGenerator {
    pub fn new() -> Self {
        // Create the ISA (Instruction Set Architecture) for the target
        let mut flag_builder = settings::builder();
        flag_builder.set("use_colocated_libcalls", "false").unwrap();
        flag_builder.set("is_pic", "false").unwrap();
        
        // Try to force frame pointer preservation on Apple ARM64 for better stack alignment
        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        {
            flag_builder.set("preserve_frame_pointers", "true").unwrap();
        }
        
        // Detect the target triple for proper object format
        let target_triple = if cfg!(target_os = "macos") && cfg!(target_arch = "aarch64") {
            "aarch64-apple-darwin"
        } else if cfg!(target_os = "macos") && cfg!(target_arch = "x86_64") {
            "x86_64-apple-darwin"
        } else if cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") {
            "x86_64-unknown-linux-gnu"
        } else if cfg!(target_os = "linux") && cfg!(target_arch = "aarch64") {
            "aarch64-unknown-linux-gnu"
        } else if cfg!(target_os = "windows") && cfg!(target_arch = "x86_64") {
            "x86_64-pc-windows-msvc"
        } else {
            panic!("Unsupported target combination: {} {}", std::env::consts::OS, std::env::consts::ARCH);
        };
        
        let isa_builder = cranelift_codegen::isa::lookup(target_triple.parse().unwrap())
            .map_err(|e| format!("Failed to find ISA for {}: {}", target_triple, e))
            .unwrap();
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
            printf_func: None,
            string_data: HashMap::new(),
        }
    }

    pub fn compile_program(mut self, statements: &[Stmt]) -> Vec<u8> {
        // Declare puts function with explicit calling convention for macOS ARM64
        let mut puts_sig = self.module.make_signature();
        
        // Set the calling convention explicitly for the target platform
        // Try SystemV for all platforms to see if it works better for C function calls
        puts_sig.call_conv = cranelift_codegen::isa::CallConv::SystemV;
        
        puts_sig.params.push(cranelift_codegen::ir::AbiParam::new(self.module.target_config().pointer_type())); // char* string
        puts_sig.returns.push(cranelift_codegen::ir::AbiParam::new(cranelift_codegen::ir::types::I32));
        
        // On macOS, Cranelift automatically adds underscore, so just use the base name
        let puts_name = "puts";
        
        let puts_func_id = self.module
            .declare_function(puts_name, Linkage::Import, &puts_sig)
            .unwrap();
        self.printf_func = Some(puts_func_id); // Store but don't use

        // Add back string processing to test if global data causes issues
        let mut string_literals = Vec::new();
        self.collect_string_literals(statements, &mut string_literals);
        
        // Create string data before main compilation
        for string_literal in &string_literals {
            self.create_string_data(string_literal);
        }

        // Create a main function  
        let mut sig = self.module.make_signature();
        sig.returns.push(cranelift_codegen::ir::AbiParam::new(cranelift_codegen::ir::types::I32));
        sig.params.clear();
        
        // Set the calling convention explicitly for the target platform
        // Try SystemV for all platforms to see if it works better
        sig.call_conv = cranelift_codegen::isa::CallConv::SystemV;

        // Cranelift handles platform-specific symbol naming automatically
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
        
        // Ensure stack alignment for Apple ARM64 - create a dummy stack slot to force alignment
        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        {
            // Create a 16-byte aligned stack slot to force proper stack frame setup
            // align_shift is log2 of alignment, so 4 = log2(16) for 16-byte alignment
            let alignment_slot = builder.create_sized_stack_slot(cranelift_codegen::ir::StackSlotData::new(
                cranelift_codegen::ir::StackSlotKind::ExplicitSlot,
                16,
                4, // log2(16) = 4 for 16-byte alignment
            ));
            
            // Touch the stack slot to ensure it's allocated in the prologue
            let _addr = builder.ins().stack_addr(self.module.target_config().pointer_type(), alignment_slot, 0);
        }

        // Create a very simple main function
        let mut _variables: HashMap<String, Variable> = HashMap::new();

        // Add back statement compilation to test if function calls cause the issue
        CodeGenerator::compile_statements_static(
            statements,
            &mut builder,
            &mut _variables,
            &mut self.module,
            self.printf_func.unwrap(),
            &self.string_data,
        );

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

    fn compile_statements_static(
        statements: &[Stmt],
        builder: &mut FunctionBuilder,
        variables: &mut HashMap<String, Variable>,
        module: &mut ObjectModule,
        printf_func: FuncId,
        string_data: &HashMap<String, DataId>,
    ) {
        for stmt in statements {
            CodeGenerator::compile_statement_static(stmt, builder, variables, module, printf_func, string_data);
        }
    }

    fn compile_statement_static(
        stmt: &Stmt,
        builder: &mut FunctionBuilder,
        variables: &mut HashMap<String, Variable>,
        module: &mut ObjectModule,
        printf_func: FuncId,
        string_data: &HashMap<String, DataId>,
    ) {
        match stmt {
            Stmt::Assign(var_name, expr) => {
                let value = CodeGenerator::compile_expression_static(expr, builder, variables, module, printf_func, string_data);
                
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
                CodeGenerator::compile_expression_static(expr, builder, variables, module, printf_func, string_data);
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
        module: &mut ObjectModule,
        printf_func: FuncId,
        string_data: &HashMap<String, DataId>,
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
                let left_val = CodeGenerator::compile_expression_static(left, builder, variables, module, printf_func, string_data);
                let right_val = CodeGenerator::compile_expression_static(right, builder, variables, module, printf_func, string_data);
                builder.ins().iadd(left_val, right_val)
            }
            Expr::Str(s) => {
                CodeGenerator::get_string_ptr_static(s, builder, module, string_data)
            }
            Expr::Call(func_name, args) => {
                if func_name == "print" {
                    // Test if the issue is in call instruction generation vs stack frame setup
                    if let Some((param_name, expr)) = args.first() {
                        if param_name == "message" {
                            // Get the actual message value for later use
                            let message_val = CodeGenerator::compile_expression_static(expr, builder, variables, module, printf_func, string_data);
                            
                            let puts_func_ref = module.declare_func_in_func(
                                printf_func,
                                builder.func
                            );
                            
                            // Try a different approach: test if we can call without parameters first
                            // This might reveal if the issue is in parameter handling vs call instruction
                            
                            // First create a null pointer as before (unused but kept for reference)
                            let _null_ptr = builder.ins().iconst(module.target_config().pointer_type(), 0);
                            
                            // Simple direct call using the original puts function (now with SystemV convention)
                            let puts_func_ref = module.declare_func_in_func(
                                printf_func,
                                builder.func
                            );
                            
                            // Test with null pointer first to isolate parameter vs call issues
                            let null_ptr = builder.ins().iconst(module.target_config().pointer_type(), 0);
                            let call_inst = builder.ins().call(puts_func_ref, &[null_ptr]);
                            let results = builder.inst_results(call_inst);
                            results[0]
                        } else {
                            builder.ins().iconst(cranelift_codegen::ir::types::I32, 0)
                        }
                    } else {
                        builder.ins().iconst(cranelift_codegen::ir::types::I32, 0)
                    }
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

    fn collect_string_literals(&self, statements: &[Stmt], strings: &mut Vec<String>) {
        for stmt in statements {
            self.collect_strings_from_stmt(stmt, strings);
        }
    }

    fn collect_strings_from_stmt(&self, stmt: &Stmt, strings: &mut Vec<String>) {
        match stmt {
            Stmt::Assign(_, expr) => self.collect_strings_from_expr(expr, strings),
            Stmt::Expr(expr) => self.collect_strings_from_expr(expr, strings),
            Stmt::If(cond, body) => {
                self.collect_strings_from_expr(cond, strings);
                for stmt in body {
                    self.collect_strings_from_stmt(stmt, strings);
                }
            }
        }
    }

    fn collect_strings_from_expr(&self, expr: &Expr, strings: &mut Vec<String>) {
        match expr {
            Expr::Str(s) => {
                if !strings.contains(s) {
                    strings.push(s.clone());
                }
            }
            Expr::Add(left, right) => {
                self.collect_strings_from_expr(left, strings);
                self.collect_strings_from_expr(right, strings);
            }
            Expr::Call(_, args) => {
                for (_, expr) in args {
                    self.collect_strings_from_expr(expr, strings);
                }
            }
            Expr::Eq(left, right) => {
                self.collect_strings_from_expr(left, strings);
                self.collect_strings_from_expr(right, strings);
            }
            _ => {}
        }
    }

    fn create_string_data(&mut self, s: &str) {
        if self.string_data.contains_key(s) {
            return;
        }

        let mut data_desc = cranelift_module::DataDescription::new();
        let mut string_bytes = s.as_bytes().to_vec();
        string_bytes.push(0); // null terminator (puts handles newline automatically)
        
        data_desc.define(string_bytes.into_boxed_slice());

        let data_id = self.module
            .declare_anonymous_data(false, false)
            .unwrap();
        self.module.define_data(data_id, &data_desc).unwrap();
        self.string_data.insert(s.to_string(), data_id);
    }

    fn get_string_ptr_static(
        s: &str,
        builder: &mut FunctionBuilder,
        module: &mut ObjectModule,
        string_data: &HashMap<String, DataId>,
    ) -> cranelift_codegen::ir::Value {
        let data_id = string_data[s];
        let global_value = module.declare_data_in_func(data_id, builder.func);
        // Use the correct pointer type for the target platform
        let pointer_type = module.target_config().pointer_type();
        builder.ins().global_value(pointer_type, global_value)
    }
}