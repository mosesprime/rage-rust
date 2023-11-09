fn main() {
    let compiler = ragec_lib::Compiler::new("./examples/test.ra".into(), "./examples/exit.asm".into());
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
