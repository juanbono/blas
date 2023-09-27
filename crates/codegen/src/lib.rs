use cranelift::prelude::*;
use cranelift_object::{ObjectModule, ObjectBuilder};

pub struct CodeGenerator {
    pub module: ObjectModule,
}

impl CodeGenerator {
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

    pub fn generate_main(&mut self, function_name: &str, ) {
        todo!()
    }
}
