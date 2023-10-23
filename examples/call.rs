use libphp::exec::Context;

fn main() {
    let mut context = Context::new();

    let version_result = context.call("phpversion");
    dbg!(version_result);

    let strlen_result = context.call_with("strlen", &["Hello, world!"]);
    dbg!(strlen_result);

    context.execute_file("./examples/scripts/functions.php");

    let fib_30_result = context.call_with("fib", &[35]);
    dbg!(fib_30_result);
}