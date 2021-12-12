# 03.03 函数

基本格式
```rust
fn function_name(arg0: type0, ...) -> return_type {
    ...
}
```

语句（statement） 没有返回值，表达式（expression）有返回值。

语句：`let` 语句，函数定义等。

rust 有一个特殊的表达式——代码块表达式。如
```rust
let y = {
    let x = 3;
    x + 1
};
```

函数的返回值等同于函数体最后一个表达式的值。使用 `return some_value` 可以提前返回。