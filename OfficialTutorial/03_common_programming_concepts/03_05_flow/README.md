# 03.05 控制流

## 1. if

与 C++ 的区别在于不需要括号
```rust
if condition_expression {
    //...
} else if {
    //...
} else if {
    //...
} else {
    //...
}
```

`if` 分支可以作为表达式使用，值的类型必须相同
```rust
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
```

> 实际上 `if` 分支就是表达式，如果“没有”值，那就是 `()`。

## 2. 循环

### 2.1 loop

`loop` 是无限循环，等价与 C++ 的 `while(true)`
```rust
fn main() {
    loop {
        println!("again!");
    }
}
```
为解决推出多层 loop 问题，rust 的方案是**循环标签**，作用于循环的关键字（如 `loop` 上），然后可以用 `[break|continue] <label>` 来选择需要处理的循环。
```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}
``` 
`loop` 也是一个表达式，值由 `break` 语句决定。
```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```

### 2.2 while

同于 C++ 的 `while`，但条件表达式不需要圆括号。

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}
```
### 2.3 for

rust 的 `for` 同于 C++ 的 range-`for`，而不是经典的 `for(...;...;...)...`。

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```