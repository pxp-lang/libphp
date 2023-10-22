use std::env::args;

use libphp::exec::Context;

fn main() {
    let file = args()
        .nth(1)
        .expect("Please provide the name of the script you would like to execute.");

    let mut context = Context::new();
    let return_value = context.execute_file(&file);

    println!("Return value of script: {:?}", return_value);
}
