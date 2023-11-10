fn main() {
    let compiler = ragec_lib::Compiler::new("./examples/demo.rg".into(), "./examples/exit.asm".into());
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
    let compiler = ragec_lib::Compiler::new("./examples/demo.ra".into(), "./examples/exit.asm".into());
    assert!(compiler.run().is_err());
}

#[test]   
fn compile_test_rg() {
    let compiler = ragec_lib::Compiler::new("./examples/demo.rg".into(), "./examples/exit.asm".into());
    assert!(compiler.run().is_err());
}
