use libphp::exec::Context;

fn main() {
    let mut context = Context::new();
    let array = context.result_of("[1, 2, 3, 4, 5, 'key' => 6]");

    println!("The array returned is: {array:?}");
    println!("is_array(): {}", array.is_array());

    let array = array.to_array();

    println!("array.len(): {}", array.len());
    println!("array.is_empty(): {}", array.is_empty());

    for (idx, key, value) in array.iter() {
        println!("array[{}] (pos: {}) = {:?}", key, idx, value);
    }
}
