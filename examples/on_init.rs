use libphp::exec::Context;

fn main() {
    let mut context = Context::new();

    context.on_init(|ctx| {
        ctx.define("EXAMPLE_CONSTANT_FROM_RUST", "Hello, world!");
    });

    dbg!(context.result_of("EXAMPLE_CONSTANT_FROM_RUST"));
}