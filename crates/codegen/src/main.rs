use cranelift::prelude::*;

fn main() {
    // create a settings builder to configure the opt level
    let mut settings_builder = settings::builder();
    // disable optimizations
    settings_builder.set("opt_level", "none").unwrap(); // TODO: take the opt level in the CLI.

    let flags = settings::Flags::new(settings_builder);

    // create the ISA builder using the native configuration.
    let isa_builder = cranelift_native::builder().unwrap();
    let isa = isa_builder.finish(flags).unwrap();

    // get the pointer_type from the ISA
    let pointer_type = isa.pointer_type();

    // to create a function first we need to create its signature
    let mut signature = Signature::new(isa::CallConv::SystemV);
    signature.params.push(AbiParam::new(pointer_type));
    // add the return type
    signature.returns.push(AbiParam::new(types::I64));

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
    function_builder.ins().return_(&[Value::from_u32(2)]);

    // finalize the code generation of the function
    function_builder.finalize();

    // verify that the generated function is well formed.
    codegen::verify_function(&function, &*isa).unwrap();

    // create a codegen context for the function.
    // This context is the one that translate the cranelift IR to
    // assembly code for our architecture.
    let mut ctx = codegen::Context::for_function(function);

    let mut ctrl_plane = codegen::control::ControlPlane::default();
    ctx.set_disasm(true);

    // compile to assembly code.
    let code = match ctx.compile(&*isa, &mut ctrl_plane) {
        Ok(x) => x,
        Err(err) => {
            eprintln!("error compiling: {:?}", err);
            std::process::exit(8);
        }
    };

    // println!("dissasembled code: \n{}", code.disasm.as_ref().unwrap());
    let code = code.code_buffer().to_vec();
    println!("Compiled function \n{}", ctx.func.display());
}
