use macros::{make_answer, my_attribute, MyTrait};

#[derive(MyTrait)]
struct MyStruct;

pub trait MyTrait {
    fn hello(&self);
}

#[my_attribute]
fn my_function() {
    println!("Inside the function");
}

make_answer!();

fn main() {
    let my_struct = MyStruct;
    my_struct.hello();

    my_function();

    println!("The answer is {}", answer()); // 输出: The answer is 42
}
