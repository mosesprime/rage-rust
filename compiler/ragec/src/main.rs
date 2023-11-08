fn main() {
    let compiler = ragec_lib::Compiler::new("./examples/exit.ra".into(), "./examples/exit.asm".into());
    if let Err(errors) = compiler.run() {
        for e in errors {
            eprintln!("{e:?}");
        }
    }
}

#[test]
fn compile_example_exit_ra() {
    let compiler = ragec_lib::Compiler::new("./examples/exit.ra".into(), "./examples/exit.asm".into());
    assert!(compiler.run().is_ok())
}
