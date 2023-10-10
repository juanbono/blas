use cranelift::prelude::*;
use cranelift_module::{Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};

pub struct CodeGenerator {
    pub module: ObjectModule,
}

impl CodeGenerator {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        // create a settings builder to configure the opt level
        let mut settings_builder = settings::builder();
        // disable optimizations
        // TODO: take the opt level in the CLI.
        settings_builder.set("opt_level", "none").unwrap();

        let flags = settings::Flags::new(settings_builder);

        // create the ISA builder using the native configuration.
        let isa_builder = cranelift_native::builder().unwrap();
        let isa = isa_builder.finish(flags).unwrap();

        // To define a module, first we need an object builder
        let object_builder =
            ObjectBuilder::new(isa, "main", cranelift_module::default_libcall_names()).unwrap();
        let module = ObjectModule::new(object_builder);

        Self { module }
    }

    pub fn generate_main(&mut self, statements: &Vec<parser::Statement>) {
        // to create a function first we need to create its signature
        let mut signature = Signature::new(isa::CallConv::SystemV);

        // add the return type
        signature.returns.push(AbiParam::new(types::I64));

        // declare the function into the module
        let function_id = self
            .module
            .declare_function("main", Linkage::Export, &signature)
            .unwrap();

        // now we can create our function
        let mut function = codegen::ir::Function::with_name_signature(
            codegen::ir::UserFuncName::user(0, 0),
            signature,
        );
        // we also need a function builder context
        let mut func_ctx = FunctionBuilderContext::new();
        // and with both we can create our function builder, where we are going to store the generated instructions
        let mut function_builder = FunctionBuilder::new(&mut function, &mut func_ctx);

        // code generation
        // create an empty block and switch to it.
        let block = function_builder.create_block();
        function_builder.seal_block(block);
        function_builder.append_block_params_for_function_params(block);
        function_builder.switch_to_block(block);

        // generate code for each statement
        for stmt in statements {
            self.generate_stmt(&mut function_builder, stmt)
        }

        // finalize the code generation of the function
        function_builder.finalize();

        // create a codegen context for the function.
        // This context is the one that translate the cranelift IR to
        // assembly code for our architecture.
        let mut ctx = codegen::Context::for_function(function);

        self.module.define_function(function_id, &mut ctx).unwrap();
    }

    pub fn emit(self) -> Vec<u8> {
        // now that all is in place we can finish the module
        let object_product = self.module.finish();
        // and emit the code to a file
        object_product.emit().unwrap()
    }

    fn generate_stmt(&mut self, fn_builder: &mut FunctionBuilder, stmt: &parser::Statement) {
        match &stmt.data {
            parser::StatementData::Return(expr) => {
                self.generate_expr(fn_builder, expr);
            }
        }
    }

    fn generate_expr(&self, fn_builder: &mut FunctionBuilder, expr: &parser::Expression) {
        match expr.data {
            parser::ExpressionData::Number(n) => {
                // insert the "return n" instruction
                let return_value = fn_builder.ins().iconst(types::I64, n as i64);
                fn_builder.ins().return_(&[return_value]);
            }
        }
    }
}
