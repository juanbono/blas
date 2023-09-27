use std::{path::Path, process::Command};

use ::codegen::CodeGenerator;
use cranelift::prelude::*;
use cranelift_module::{Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};

fn main() {
    let mut code_generator = CodeGenerator::new();

    // to create a function first we need to create its signature
    let mut signature = Signature::new(isa::CallConv::SystemV);

    // add the return type
    signature.returns.push(AbiParam::new(types::I64));

    // declare the function into the module
    let function_id = code_generator.module
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

    // insert the "return 2" instruction
    let return_value = function_builder.ins().iconst(types::I64, 42);
    function_builder.ins().return_(&[return_value]);

    // finalize the code generation of the function
    function_builder.finalize();

    // create a codegen context for the function.
    // This context is the one that translate the cranelift IR to
    // assembly code for our architecture.
    let mut ctx = codegen::Context::for_function(function);

    code_generator.module.define_function(function_id, &mut ctx).unwrap();

    // now that all is in place we can finish the module
    let object_product = code_generator.module.finish();
    // and emit the code to a file
    let bytes = object_product.emit().unwrap();

    std::fs::write("main.o", bytes).unwrap();

    link(Path::new("main.o"), Path::new("main"));
}

fn link(obj_file: &Path, output: &Path) {
    Command::new("cc")
        .args(&[&obj_file, Path::new("-o"), output])
        .status()
        .unwrap();
}
