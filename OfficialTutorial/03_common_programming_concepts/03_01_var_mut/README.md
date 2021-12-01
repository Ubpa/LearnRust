# 03.01 变量与可变性

```rust
let x = 5;     // x 相当于 const
let mut y = 3; // y 相当于 non-const
```

Rust 的设计理念就是 `explicit`，C++ 的问题就在于写 `const` 很麻烦，Rust 就通过默认 `const` 来规避这个问题。

Rust 可以在不同作用域里隐藏变量，甚至可以在相同作用域里隐藏变量。
```rust
let x = 5;
let x = x + 1; // <-------------------------
//  ^ redefinition / shadowing            //
{                                         //
    let x = x * 2; // <----------------   //
    //  ^ shadowing again             |   //
    println!("The value of x is: {}", x); //
}                                         //
                                          //
println!("The value of x is: {}", x); //----
```

甚至可以改变类型
``` rust
let spaces = "   ";
let spaces = spaces.len();
//  ^^^^^^ different types: string -> integer
println!("The length of spaces is: {}", spaces);
```

这不同于修改变量的值，这里本质上是新的变量。
``` rust

```