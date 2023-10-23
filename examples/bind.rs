use libphp::exec::Context;

fn main() {
    let mut context = Context::new();

    context.on_init(|ctx| {
        ctx.bind("myVar", "Hello, this variable is defined in Rust!");
    });

    let my_var = context.result_of("$myVar");
    println!("my_var = {:?}", my_var);
}
