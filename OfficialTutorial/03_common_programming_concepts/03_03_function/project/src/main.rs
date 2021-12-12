fn main() {
    println!("Hello, world!");

    another_function(five());
}

fn another_function(x : i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
//           ^^^ return type
    5 /* no ;*/
//  ^ the value of the last expression is the return value
}
