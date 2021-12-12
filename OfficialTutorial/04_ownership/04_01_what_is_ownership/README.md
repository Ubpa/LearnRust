# 04.01 什么是所有权？
  
本节讲一些跟 C++ 一样的概念。

所有权规则
- Rust 中的每一个值都有一个被称为其 **所有者**（owner）的变量。
- 值在任一时刻有且只有一个所有者。
- 当所有者（变量）离开作用域，这个值将被丢弃。

作用域，内存，堆栈

Rust 默认移动拷贝（C++ 默认拷贝），而需要拷贝则一般需要调用 `obj.clone()`。而且与 C++ 不同的是，在发生移动拷贝后，变量是语法上失效了，而不仅仅是语义上失效（不会再调用析构函数）。

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1); // 此处会产生编译错误，因为此时 s1 已经失效
```

调用函数时会发生参数的移动或复制
```rust
fn main() {
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
                                    // ... 所以到这里不再有效

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该移动函数里，
                                    // 但 i32 是 Copy 的，所以在后面可继续使用 x

} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作
```

