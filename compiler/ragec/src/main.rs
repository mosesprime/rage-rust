use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let compiler = ragec_lib::Compiler::new("./examples/exit.rg".into(), "./examples/exit.asm".into());
    Ok(compiler.run()?)
}
