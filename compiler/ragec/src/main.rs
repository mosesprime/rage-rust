use ragec_lib::Compiler;

fn main() {
    let compiler = Compiler::new();
    match compiler.run("./examples/demo.rg".into()) {
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
    Compiler::new().run("../../examples/demo.ra".into()).unwrap();
}

#[test]   
fn compile_test_rg() {
    Compiler::new().run("../../examples/demo.rg".into()).unwrap();
}
