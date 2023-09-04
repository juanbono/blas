use cranelift::prelude::settings;

fn main() {
    let isa_builder = cranelift_native::builder().unwrap();
    let flag_builder = settings::builder();
    let isa = isa_builder
        .finish(settings::Flags::new(flag_builder))
        .unwrap();

    println!("{}", isa.triple());
}
