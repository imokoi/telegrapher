use macros::{make_answer, my_attribute};

#[my_attribute]
fn my_function() {
    println!("Inside the function");
}

make_answer!();

fn main() {
    my_function();

    println!("The answer is {}", answer()); // 输出: The answer is 42
}
