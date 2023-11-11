use std::thread;

use ragec_lib::Compiler;

fn main() {
    let tasks = vec!["./examples/demo.rg", "./examples/demo.ra"];
    let mut handles = vec![];
    for task in tasks {
        let h = thread::spawn(move || {
            match Compiler::new().run(task.into()) {
                Err(errors) => {
                    for e in errors {
                        eprintln!("{e:?}");
                    }
                },
                Ok(()) => {
                    println!("Okay");
                },
            }    
        });
        handles.push(h);
    }
    for handle in handles {
        handle.join().unwrap();
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
