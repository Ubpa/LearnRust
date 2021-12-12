# 03.02 数据类型

Rust 是静态类型语言，与 C++ 类似，但自动类型推断比 C++ 智能非常多。

```rust
fn main() {
//  let guess = "42".parse().expect("Not a number");
//      ^^^^^ consider giving `guess` a type
    let guess: u32 = "42".parse().expect("Not a number");
//             ^^^ 帮助 parse() 推断类型
    println!("guess: {}", guess);
}

```

## 1. 标量类型

### 1.1 整型

整型类型
|长度|有符号|无符号|
|:--:|:--:|:--:|
|8-bit|i8|u8|
|16-bit|i16|u16|
|32-bit|i32|u32|
|64-bit|i64|u64|
|128-bit|i128|u128|
|arch|isize|usize|

其中 `arch` 由机器架构而定，32 位架构上是 32 位，64 位架构上是 64 位。

规律：`i/u + bits`

整型字面值
|数字字面值|例子|规律|
|:--|:--|:--|
|十进制|98222||
|十六进制|0xFF|`0x` 开头|
|八进制|0o77|`0o` 开头|
|二进制|0b11011|`0b` 开头|
|字节|b\`A\`|`b` + 字符|

### 1.2 浮点

rust 的浮点数类型是 `f32` 和 `f64`，默认是 `f64`。

### 1.3 字符类型

`char` 是 4 bytes，代表 Unicode 标量值（注意不是 ASCII）。
> 范围：`U+0000 - U+D7FF` 和 `U+E000 - U+10FFFF`

## 2. 复合类型

元组 `tuple` 和 数组 `array`

### 2.1 元组

类似 C++ 的 `std::tuple`，只不过 rust 做进了语法里
> 啥都往语法里刻

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

同样支持解构（C++ 里叫绑定）
```rust
let tup = (500, 6.4, 1);
let (x, y, z) = tup;
```

也可以访问单个元素（C++ 就是靠 `std::get<Index/Type>(...)`），语法是 `t.N`：
```rust
let x: (i32, f64, u8) = (500, 6.4, 1);

let five_hundred = x.0;
let six_point_four = x.1;
let one = x.2;
```

空元组 `()` 称为单元类型 `unit type`，该类型只有一个值，也写成 `()`。如果表达式不返回其他值，则隐式返回单元值。

### 2.2 数组
类似 C++ 的 `std::array<Type, N>`，含有成员方法，比较特殊的是其值的表示方式，包括：
```rust

```