fn main() {
    let compiler = ragec_lib::Compiler::new("./examples/demo.rg".into()).expect("Failed to load source into compiler.");
    match compiler.run() {
        Err(errors) => {
            for e in errors {
                eprintln!("{e:?}");
            }
        },
        Ok(()) => {
            println!("Okay");
        },
    }
}

#[test]   
fn compile_test_ra() {
    let compiler = ragec_lib::Compiler::new("../../examples/demo.ra".into()).unwrap();
    compiler.run().unwrap();
}

#[test]   
fn compile_test_rg() {
    let compiler = ragec_lib::Compiler::new("../../examples/demo.rg".into()).unwrap();
    compiler.run().unwrap();
}
