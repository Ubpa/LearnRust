# 02. 猜猜看

rust 默认用 const，用 `mut` 显式表明为可变量

& 被看成是取引用，而不是取地址（规避了指针？），并且默认是 const，需要用 &mut 明确表明是可变的

之前说了 rust 自带包管理器 cargo（太棒了！），使用依赖包很简单，比如用 rand 包

```toml
[dependencies]

rand = "0.8.3"
```

运行 `cargo build` 会拉取依赖包

注意 rust 首先需要拉去 crates.io index，这是从 github 拉取的，网不好的话会出错，可以改镜像源

新建文件 `~/.cargo/config`（`~` 表示用户文件夹，`config` 一开始是不存在的，需要自己新建，并且注意没有后缀），写入

```toml
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
```

match-pattern 语法比较神奇，类似 switch-case，但实际上它是一个表达式（有值），貌似必须把所有 pattern 写上（可以写 break 来忽略其他情况？）

